"""Separate ordinary/observer builds with actual resolved build and launch identity."""
import importlib.util
import json
import os
import platform
import shutil
import subprocess
import sys
from pathlib import Path
from inventory import HERE,ROOT,PIN,RELEASE,FILES,PROFILE,blob,sha,save
from fixtures import VECTORS,PLANETS,RESOURCES
BUILD=HERE/'build'; ENV={**os.environ,'LC_ALL':'C','LANG':'C'}
CC=PROFILE['cxx'];C=PROFILE['cc']
OBSERVERS=[
 ('            rsq=v1*v1+v2*v2;', '            rsq=v1*v1+v2*v2;\n            extern void z_gaussian_attempt(double,double,double);\n            z_gaussian_attempt(v1,v2,rsq);'),
 ('    rm1=1./m1;', '    extern void z_random_observer(const longdouble*,const longdouble*,const int*,const int*,const int*,const int*);\n    z_random_observer(r,&result,&iff,&ix1,&ix2,&ix3);\n    rm1=1./m1;'),
 ('    if  (iset == 0) {','    extern void z_gas_observer(const int*,const double*);\n    z_gas_observer(&iset,&gset);\n    if  (iset == 0) {'),
 ('#define MAX_SHOTS 10000','#define MAX_SHOTS 10000\nextern void *z_init(const char*,char (*)[6],double*);\nextern int z_pleph(void*,double,int,int,double*,int);\nextern void z_close(void*);'),
]
def resources():
    # Executing the accepted generator definitions only; never its recording runner.
    src=ROOT/'validation/jpl/run.py';ns={'__name__':'z_readonly_jpl_generator','__file__':str(src)}
    exec(compile(src.read_bytes(),str(src),'exec'),ns)
    root=BUILD/'resources';root.mkdir(parents=True,exist_ok=True);result={}
    for name,kwargs in RESOURCES.items():
        p=root/(name+'.bin')
        if kwargs.get('missing'):
            assert not p.exists();result[name]=dict(exists=False);continue
        if kwargs.get('real'):
            real=ns['REAL'];data=real.read_bytes();assert sha(data)=='86786f07aeddc1c5b6ddb24d8c65930803458938c6a2f920e8ea72a86da7479c'
            if not p.exists():p.symlink_to(real)
            assert p.resolve()==real.resolve();spec={'real_path':str(real)}
        elif kwargs.get('short'):data=b'short';spec={'defined_init_error':'short title'}
        else:data,spec=ns['synthetic'](**kwargs)
        if not kwargs.get('real'):p.write_bytes(data)
        assert p.read_bytes()==data
        result[name]=dict(path=str(p),sha256=sha(data),size=len(data),spec=spec)
    save(BUILD/'resources.json',result);return result

def generate_checks(path):
    types=json.loads((HERE/'outputs/frozen/source_map.json').read_text())['types'];lines=[]
    for kind in types:lines.append(f'template<> struct Equal<{kind}> {{static bool check(const {kind}&a,const {kind}&b);}};')
    for kind,fields in types.items():
        lines.append(f'bool Equal<{kind}>::check(const {kind}&a,const {kind}&b) {{')
        for f in fields:
            n=f['name']
            if kind=='observation' and n in VECTORS+PLANETS:continue
            lines.append(f' if(!same(a.{n},b.{n})){{std::cerr<<"changed {kind}.{n}\\n";return false;}}')
        lines.extend([' return true;','}'])
    path.write_text('\n'.join(lines)+'\n')

def compile_probe(name='ordinary',mutation=None,coverage=False,observer=False):
    folder=BUILD/name;src=folder/'source';src.mkdir(parents=True,exist_ok=True)
    for f in FILES:(src/f).write_bytes(blob(f))
    (src/'TKlongdouble.h').write_bytes(blob('TKlongdouble.float128.h'))
    config=(ROOT/'validation/reference/build/source/config.h').read_bytes();assert sha(config)==PROFILE['config_sha256'];(src/'config.h').write_bytes(config)
    g=blob('global.C');(src/'global_data.C').write_bytes(g[:g.index(b'void extra_delays')]+g[g.index(b'void CVSdisplayVersion'):])
    source=src/'readEphemeris.C'
    if mutation:
        old,new=mutation;s=source.read_text();assert s.count(old)==1,(name,old,s.count(old));source.write_text(s.replace(old,new))
    if observer:
        s=source.read_text()
        for old,new in OBSERVERS:assert s.count(old)==1;s=s.replace(old,new)
        for a,b in [('jpl_init_ephemeris(','z_init('),('jpl_pleph(','z_pleph('),('jpl_close_ephemeris(','z_close(')]:s=s.replace(a,b)
        source.write_text(s)
    generate_checks(src/'field_checks.h')
    flags=PROFILE['cxx_flags']+['-I'+str(src)];cflags=PROFILE['c_flags']+['-I'+str(src)]
    if observer:flags+=['-DZ_OBSERVER']
    if coverage:flags+=['--coverage','-fno-inline'];cflags+=['--coverage','-fno-inline']
    units=[HERE/'probe.cc',*[src/f for f in ['readEphemeris.C','clkcorr.C','tabulatedfunction.C','observatory.C','TKlog.C','TKlongdouble.C','tempo2Util.C','global_data.C','jpleph.c']]]
    commands=[];dependencies=set();objects=[]
    for unit in units:
        obj=folder/(unit.stem+'.o');dep=obj.with_suffix('.d');cmd=[C if unit.suffix=='.c' else CC,*(cflags if unit.suffix=='.c' else flags),'-MD','-MF',str(dep),'-c',str(unit),'-o',str(obj)]
        r=subprocess.run(cmd,capture_output=True,text=True,env=ENV);(folder/(unit.stem+'.compile.log')).write_text(r.stdout+r.stderr);assert r.returncode==0,r.stderr
        commands.append(cmd);objects.append(obj);dependencies.update(Path(p) for p in dep.read_text().replace('\\\n','').split(':',1)[1].split())
    exe=folder/'probe';cmd=[CC,*map(str,objects),*PROFILE['link'],'-Wl,-map,'+str(folder/'probe.map'),'-o',str(exe)]
    if coverage:cmd+=['--coverage']
    r=subprocess.run(cmd,capture_output=True,text=True,env=ENV);assert r.returncode==0,r.stderr;commands.append(cmd)
    toolchain={}
    for program in ('cc1plus','cc1','collect2','as','ld'):
        n=subprocess.check_output([CC,'-print-prog-name='+program],text=True).strip();toolchain[program]=str(Path(n if '/' in n else shutil.which(n)).resolve())
    for program in ('as','ld'):toolchain['xcrun_'+program]=subprocess.check_output(['xcrun','-f',program],text=True).strip()
    toolchain.update(cxx=str(Path(CC).resolve()),cc=str(Path(C).resolve()),python=str(Path(sys.executable).resolve()))
    runtimes=[str(Path(subprocess.check_output([CC,'-print-file-name='+n],text=True).strip()).resolve()) for n in ('libquadmath.0.dylib','libstdc++.6.dylib','libgcc_s.1.1.dylib')]
    macros=subprocess.check_output([CC,*flags,'-dM','-E','-x','c++','-include',str(src/'tempo2.h'),'/dev/null'],env=ENV);(folder/'macros.txt').write_bytes(macros)
    symbols=subprocess.check_output(['nm','-u',str(folder/'readEphemeris.o')],text=True)
    assert '\n_fortran_mod\n' in '\n'+symbols and '_Z11fortran_moddd' not in symbols
    tracked=dependencies|set(map(Path,[*toolchain.values(),*runtimes]))|set(src.glob('*'))|set(HERE.glob('*.py'))|{HERE/'probe.cc',HERE/'CONTRACT.md',ROOT/'validation/jpl/run.py'}
    profile=dict(pin=PIN,release=RELEASE,commands=commands,flags=flags,c_flags=cflags,compiler_version=subprocess.check_output([CC,'--version'],text=True),host=subprocess.check_output(['sw_vers'],text=True),machine=platform.machine(),python_version=sys.version,toolchain=toolchain,runtimes=runtimes,
       macros_sha256=sha(macros),dependencies={str(p):sha(p.read_bytes()) for p in sorted(tracked) if p.is_file()},objects={p.name:sha(p.read_bytes()) for p in objects},
       executable_sha256=sha(exe.read_bytes()),link_map_sha256=sha((folder/'probe.map').read_bytes()),dynamic_libraries=subprocess.check_output(['otool','-L',str(exe)],text=True).splitlines()[1:],
       primary_symbols=symbols,utility_symbols=subprocess.check_output(['nm',str(folder/'tempo2Util.o')],text=True),coverage=coverage,mutation=mutation,observer=observer,observer_patch=OBSERVERS if observer else [])
    save(folder/'profile.json',profile);return exe,profile

def actual_identity(exe,profile):
    assert sha(exe.read_bytes())==profile['executable_sha256'],'executed binary changed'
    for p,h in profile['dependencies'].items():assert sha(Path(p).read_bytes())==h,'executed dependency changed: '+p
    for p,h in profile['objects'].items():assert sha((exe.parent/p).read_bytes())==h,'executed object changed'
    return dict(executable_sha256=sha(exe.read_bytes()),dependency_count=len(profile['dependencies']))
def stable(p):return {k:v for k,v in p.items() if k not in ('executable_sha256','link_map_sha256','objects')}
