"""AK native builds: verified AI/AH dependencies plus pinned observation loop.

Only disposable calculate_bclt/dm_delays objects change. The observer writes each
iteration's own slot. Shared diagnostic race runs are never comparison goldens.
"""
from pathlib import Path
import difflib, hashlib, json, subprocess, sys
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f';REF=ROOT/'validation/reference/build'
CXX='/opt/homebrew/bin/g++-16'
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def raw(name):return subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':'+name],text=True)
def build(out):
 out=Path(out).resolve();out.mkdir(parents=True,exist_ok=False)
 receipt=ROOT/'validation/native_backends/build/profiles-final-v3/disabled-release/native/identity.json'
 identity=json.loads(receipt.read_text());deps=[]
 for name,digest in identity['objects'].items():
  assert sha(name)==digest,name
  if Path(name).name not in ['calculate_bclt.C.o','dm_delays.C.o']:deps.append(name)
 archive=REF/'libtempo2-reference.a';assert sha(archive)==identity['archive']
 commands=[]
 def run(label,cmd):
  cmd=list(map(str,cmd));r=subprocess.run(cmd,cwd=out,capture_output=True,timeout=180)
  (out/(label+'.log')).write_bytes(r.stdout+r.stderr);commands.append(dict(label=label,command=cmd,exit=r.returncode))
  (out/'commands.json').write_text(json.dumps(commands,indent=2)+'\n');assert r.returncode==0,(label,r.stderr[-2000:])
 flags=['-O2','-std=gnu++17','-DHAVE_CONFIG_H','-I'+str(REF/'source')]
 raw_calc=raw('calculate_bclt.C');(out/'original-calculate_bclt.C').write_text(raw_calc)
 text=raw_calc.replace('void calculate_bclt(pulsar *psr,int npsr)', 'extern void ak_begin(pulsar*,int);\nextern void ak_visit(int,int);\nextern void ak_end();\nvoid calculate_bclt(pulsar *psr,int npsr)',1)
 text=text.replace('    const char *CVS_verNum', '    ak_begin(psr,npsr);\n    const char *CVS_verNum',1)
 text=text.replace('for (int i=0;i<psr[p].nobs;i++){','for (int i=0;i<psr[p].nobs;i++){\n            ak_visit(p,i);',1)
 text=text.replace('    logdbg("Leaving calculate_bclt");','    ak_end();\n    logdbg("Leaving calculate_bclt");',1)
 text=text.replace('if (psr[p].correctTroposphere==0) psr[0].obsn[i].troposphericDelay=0;', 'if (psr[p].correctTroposphere==0) psr[p].obsn[i].troposphericDelay=0; // AC accepted selected-pulsar correction.')
 (out/'calculate_bclt.C').write_text(text)
 dm=raw('dm_delays.C');(out/'original-dm_delays.C').write_text(dm)
 a=dm.index('            if (psr[p].param[param_dm].paramSet[0] == 1 &&');b=dm.index('            //dmoffs is basically an ifunc',a)
 corrected=dm[:a]+'''            // AK: dm_delays.C:160-195 static t races in the source OpenMP loop.
            // Serialize only mask checks, log bookkeeping and once-only messages.
#ifdef HAVE_OPENMP
#pragma omp critical(ak_dmmodel_warning)
#endif
            {
'''+dm[a:b]+'''            }
'''+dm[b:]
 (out/'dm_delays.C').write_text(corrected)
 sw=raw('sw_delay.C');(out/'original-sw_delay.C').write_text(sw)
 sw='extern void ak_sheet(const char*);\nextern void ak_solar(int,int,double,double,int,int,double);\n'+sw
 assert sw.count('    return(DM_sun);')==1
 sw=sw.replace('    return(DM_sun);', '    ak_solar(irot1,irot2,bcrlon,DM_sun,nSlow,nFast,elong);\n    return(DM_sun);',1)
 sw=sw.replace('    while (!feof(fin))', '    ak_sheet(fname);\n    while (!feof(fin))',1)
 (out/'sw_delay.C').write_text(sw)
 run('solar',[CXX,*flags,'-c',out/'sw_delay.C','-o',out/'solar.o'])
 (out/'corrections.patch').write_text(''.join(difflib.unified_diff(raw_calc.splitlines(True),text.splitlines(True),fromfile='pinned/calculate_bclt.C',tofile='instrumented/calculate_bclt.C'))+''.join(difflib.unified_diff(dm.splitlines(True),corrected.splitlines(True),fromfile='pinned/dm_delays.C',tofile='corrected/dm_delays.C')))
 hook=HERE/'native_workers.cc';run('hooks',[CXX,*flags,'-fopenmp','-c',hook,'-o',out/'workers.o'])
 harness=ROOT/'validation/input_to_bat/native_pipeline.cc';body=harness.read_text().replace('MAX_PSR=n;MAX_OBSN=64;', 'MAX_PSR=n;MAX_OBSN=20000;')
 body='#include <chrono>\n'+body
 body=body.replace(' if(argc<5)return 2;', ' auto begin=std::chrono::steady_clock::now();\n if(argc<5)return 2;')
 needle='preProcess(ps,n,options.size(),options.data());formBatsAll(ps,n);'
 assert body.count(needle)==1
 body=body.replace(needle, '''preProcess(ps,n,options.size(),options.data());
 fprintf(stderr,"AK_LOAD %.9f\\n",std::chrono::duration<double>(std::chrono::steady_clock::now()-begin).count());
 int repeats=getenv("AK_REPEATS")?atoi(getenv("AK_REPEATS")):1;
 for(int pass=0;pass<repeats;pass++){
  auto start=std::chrono::steady_clock::now();formBatsAll(ps,n);
  fprintf(stderr,"AK_EVALUATE %d %.9f\\n",pass,std::chrono::duration<double>(std::chrono::steady_clock::now()-start).count());
 }
''')
 (out/'pipeline.cc').write_text(body);run('pipeline',[CXX,*flags,'-c',out/'pipeline.cc','-o',out/'pipeline.o'])
 # Reuse the actual AI standalone command's main object for matched fitting and prediction.
 main=next(Path(c[1]) for c in identity['commands'] if len(c)>2 and c[-1].endswith('/tempo2') and c[1].endswith('tempo2.C.o'))
 for mode,extra,dm_source in [('serial',[],out/'dm_delays.C'),('parallel',['-fopenmp','-DHAVE_OPENMP'],out/'dm_delays.C'),('racy-original',['-fopenmp','-DHAVE_OPENMP'],out/'original-dm_delays.C')]:
  objects=[]
  for name,source in [('calculate',out/'calculate_bclt.C'),('dm',dm_source)]:
   obj=out/f'{mode}-{name}.o';run(mode+'-'+name,[CXX,*flags,*extra,'-c',source,'-o',obj]);objects.append(obj)
  for target,entry in [('pipeline',out/'pipeline.o'),('tempo2',main)]:
   exe=out/(mode+'-'+target);run(mode+'-'+target,[CXX,'-fopenmp',entry,*objects,out/'workers.o',out/'solar.o',*deps,archive,'-lquadmath','-lgfortran','-Wl,-map,'+str(exe)+'.map','-o',exe]);run(mode+'-'+target+'-linkage',['otool','-L',exe])
 (out/'identity.json').write_text(json.dumps(dict(pin=PIN,predecessor_receipt={str(receipt):sha(receipt)},commands=commands,dependencies={str(p):sha(p) for p in [*deps,main,archive]},sources={str(p):sha(p) for p in [hook,harness]},artifacts={p.name:sha(p) for p in out.iterdir() if p.is_file()}),indent=2)+'\n')
 return out
if __name__=='__main__':print(build(sys.argv[1]))
