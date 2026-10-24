"""Separate unchanged native translation units, reproducible build and launch identities."""
import json
import os
import platform
import shutil
import subprocess
import sys
from pathlib import Path
from inventory import HERE, ROOT, PIN, RELEASE, PRIMARY, blob, sha, save
BUILD=HERE/'build'
ENV={**os.environ,'LC_ALL':'C','LANG':'C'}
ENV.pop('TEMPO2_ALIAS',None)
CC='/opt/homebrew/bin/g++-16'
DEPENDENCIES=['clkcorr.C','tropo.C','observatory.C','tabulatedfunction.C','tempo2Util.C','TKlog.C','TKlongdouble.C']
FILES=[*PRIMARY,*DEPENDENCIES,'tempo2.h','TKlog.h','TKlongdouble.float128.h','tabulatedfunction.h','ifunc.h','shapelet.h','t2fit_solarwind.h','global.C']
OBSERVERS=[
 ('dm_delays.C', 'const char *CVS_verNum = "$Id: 5f0645a3c1b456c7bbbbec108e8f82a50be66ef1 $";', 'extern void ab_call(const char*,int,int,double,double); ab_call("dm_delays",p,i,delt,dt_SSB); const char *CVS_verNum = "$Id: 5f0645a3c1b456c7bbbbec108e8f82a50be66ef1 $";'),
 ('shapiro_delay.C', 'const char *CVS_verNum = "$Id: 46a5b64137ce96c73eb4241953b044b069fb8aed $";', 'extern void ab_call(const char*,int,int,double,double); ab_call("shapiro_delay",p,i,delt,dt_SSB); const char *CVS_verNum = "$Id: 46a5b64137ce96c73eb4241953b044b069fb8aed $";'),
 ('dm_delays.C','static int t=0xFFFF;','static int t=0xFFFF; extern void ab_mask(const int*); ab_mask(&t);'),
 ('calculate_bclt.C','if (veryFast==1) break;','extern void ab_iteration(int,int,int,double,double,double); ab_iteration(p,i,loop,dt_SSB_old,dt_SSB,delt); if (veryFast==1) break;'),
 ('clkcorr.C','double getCorrectionTT(observation *obs){','double getCorrectionTT(observation *obs){ extern void ab_tt(observation*); ab_tt(obs);'),
 ('sw_delay.C','nCurrent = readCurrentSheet(fname,currentLon,currentLat);','nCurrent = readCurrentSheet(fname,currentLon,currentLat); extern void ab_sheet(const char*,int,double,double); ab_sheet(fname,nCurrent,crlon[i],helat[i]);')]
def generate(path):
 types=json.loads((HERE/'outputs/frozen/source_map.json').read_text())['types']
 out=[]
 for kind in types:out.append(f'void dump(const std::string&,const {kind}&);')
 for kind,fields in types.items():
  out.append(f'void dump(const std::string& p,const {kind}& x){{')
  for f in fields:out.append(f' dump(p+".{f["name"]}",x.{f["name"]});')
  out.append('}')
 for kind in ['clock_correction','observation']:
  out.append(f'void seed({kind}& x,int value){{')
  for f in types[kind]:out.append(f' seed(x.{f["name"]},value);')
  out.append('}')
 for kind in ['pulsar','observation']:
  out.append(f'void setfield({kind}&x,const std::string&f,int i,const std::string&v){{')
  for f in types[kind]:
   n=f['name'];t=f['type']
   if f['pointers'] or t not in ['int','double','longdouble','float','char','series_type','enum series_type']:continue
   if len(f['extents'])>1:continue
   if not f['extents']:
    val='parse_longdouble(v.c_str())' if t=='longdouble' else 'strtod(v.c_str(),nullptr)' if t in ['double','float'] else 'std::stoi(v)'
    out.append(f' if(f=="{n}"){{ x.{n}=static_cast<decltype(x.{n})>({val});return;}}')
   elif t=='char':out.append(f' if(f=="{n}"){{assert(v.size()<sizeof x.{n});strcpy(x.{n},v.c_str());return;}}')
   else:
    val='parse_longdouble(v.c_str())' if t=='longdouble' else 'strtod(v.c_str(),nullptr)' if t in ['double','float'] else 'std::stoi(v)'
    out.append(f' if(f=="{n}"){{assert(i>=0&&size_t(i)<sizeof x.{n}/sizeof x.{n}[0]);x.{n}[i]={val};return;}}')
  out.append(' throw std::runtime_error("unknown caller field: "+f); }')
 path.write_text('\n'.join(out)+'\n')
def compile_probe(name='ordinary',mutation=None,coverage=False,observer=False):
 folder=BUILD/name;src=folder/'source';src.mkdir(parents=True,exist_ok=True)
 for f in FILES:(src/f).write_bytes(blob(f))
 (src/'TKlongdouble.h').write_bytes(blob('TKlongdouble.float128.h'))
 config=(ROOT/'validation/reference/build/source/config.h').read_bytes()
 assert sha(config)=='470c61b68a61504558e76ef772639e46b565a1e4500115d6e3fb6a71286dd69e'
 (src/'config.h').write_bytes(config)
 g=blob('global.C');(src/'global_data.C').write_bytes(g[:g.index(b'void extra_delays')]+g[g.index(b'void CVSdisplayVersion'):])
 for f,old,new in (OBSERVERS if observer else [])+([mutation] if mutation else []):
  p=src/f;s=p.read_text();assert s.count(old)==1,(f,old);p.write_text(s.replace(old,new))
 generate(src/'generated.h')
 flags=['-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(src)]
 if observer:flags+=['-DAB_OBSERVER']
 if coverage:flags+=['--coverage','-fno-inline']
 commands=[];deps=set();objects=[]
 for unit in [HERE/'probe.cc',*[src/f for f in [*PRIMARY,*DEPENDENCIES,'global_data.C']]]:
  obj=folder/(unit.stem+'.o');dep=obj.with_suffix('.d')
  cmd=[CC,*flags,'-MD','-MF',str(dep),'-c',str(unit),'-o',str(obj)]
  r=subprocess.run(cmd,capture_output=True,text=True,env=ENV)
  (folder/(unit.stem+'.compile.log')).write_text(r.stdout+r.stderr)
  assert r.returncode==0,r.stderr
  commands.append(cmd);objects.append(obj)
  deps.update(Path(p) for p in dep.read_text().replace('\\\n','').split(':',1)[1].split())
 exe=folder/'probe';cmd=[CC,*map(str,objects),'-lquadmath','-Wl,-map,'+str(folder/'probe.map'),'-o',str(exe)]
 if coverage:cmd+=['--coverage']
 r=subprocess.run(cmd,capture_output=True,text=True,env=ENV);assert r.returncode==0,r.stderr;commands.append(cmd)
 tools={}
 for prog in ['cc1plus','collect2','as','ld']:
  s=subprocess.check_output([CC,'-print-prog-name='+prog],text=True).strip();tools[prog]=str(Path(s if '/' in s else shutil.which(s)).resolve())
 for prog in ['as','ld']:tools['xcrun_'+prog]=subprocess.check_output(['xcrun','-f',prog],text=True).strip()
 runtime=[Path(subprocess.check_output([CC,'-print-file-name='+n],text=True).strip()).resolve() for n in ['libquadmath.0.dylib','libstdc++.6.dylib','libgcc_s.1.1.dylib']]
 tracked={*deps,Path(CC).resolve(),Path(sys.executable).resolve(),*runtime,*map(Path,tools.values()),*src.iterdir(),*[HERE/f for f in ('inventory.py','profile.py','fixtures.py','run.py')],HERE/'CONTRACT.md',*(HERE/'outputs/frozen').iterdir()}
 macros=subprocess.check_output([CC,*flags,'-dM','-E','-x','c++','-include',str(src/'tempo2.h'),'/dev/null'],env=ENV)
 (folder/'macros.txt').write_bytes(macros)
 profile=dict(pin=PIN,release=RELEASE,commands=commands,flags=flags,compiler=subprocess.check_output([CC,'--version'],text=True),toolchain=tools,
  machine=platform.machine(),host=subprocess.check_output(['sw_vers'],text=True),python=sys.version,macros_sha256=sha(macros),
  dependencies={str(p):sha(p.read_bytes()) for p in sorted(tracked) if p.is_file()},archive_linked=False,
  global_slices=['global.C:1-64','global.C:176-200'],executable_sha256=sha(exe.read_bytes()),link_map_sha256=sha((folder/'probe.map').read_bytes()),
  dynamic_libraries=subprocess.check_output(['otool','-L',str(exe)],text=True).splitlines()[1:],
  coverage=coverage,mutation=mutation,observer=observer,observer_patch=OBSERVERS if observer else [])
 save(folder/'profile.json',profile);return exe,profile

def actual_identity(exe,profile):
 assert sha(exe.read_bytes())==profile['executable_sha256'],'executed binary changed'
 for p,h in profile['dependencies'].items():assert sha(Path(p).read_bytes())==h,'executed dependency changed: '+p
 return dict(executable_sha256=sha(exe.read_bytes()),dependency_count=len(profile['dependencies']))
def stable(p):return {k:v for k,v in p.items() if k not in ['executable_sha256','link_map_sha256']}
