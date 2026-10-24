"""Actual raw-source build and launch identity, without writing accepted inputs."""
import os
import platform
import shutil
import subprocess
import sys
from pathlib import Path
from inventory import HERE, ROOT, PIN, PRIMARY, blob, sha, save

BUILD = HERE/'build'
ENV = {**os.environ, 'LC_ALL':'C', 'LANG':'C'}
ENV.pop('TEMPO2_ALIAS', None)
CC = '/opt/homebrew/bin/g++-16'
FC = '/opt/homebrew/bin/gfortran-16'
FILES = [*PRIMARY, 'observatory.C','eop.C', 'tempo2.h', 'tabulatedfunction.C', 'tabulatedfunction.h',
         'clkcorr.C','TKlog.C','TKlog.h','TKlongdouble.C','TKlongdouble.float128.h',
         'tempo2Util.C','global.C']
RUNTIMES = ('libgfortran.5.dylib','libquadmath.0.dylib','libstdc++.6.dylib','libgcc_s.1.1.dylib')

OBSERVERS = [
 ('tai2ut1.C', '    if (read==0) /* Read data file */',
  '    extern void x_ut1_observer(const int*,const double*,const double*,const int*,const int*,const int*,const int*);\n    x_ut1_observer(&read,&mjd1,&mjd2,mjdVal,entry,&count,&count2);\n    if (read==0) /* Read data file */'),

]

def generate_checks(path):
    import json
    types = json.loads((HERE/'outputs/frozen/source_map.json').read_text())['types']
    text = ['// Mechanical field membership from the frozen S/R type map.']
    for kind in types:
        text += [f'template<> struct Equal<{kind}> {{ static bool check(const {kind}& a,const {kind}& b); }};']
    for kind, fields in types.items():
        text += [f'bool Equal<{kind}>::check(const {kind}& a,const {kind}& b) {{']
        for field in fields:
            name = field['name']
            if kind == 'observation' and name in ('observatory_earth','siteVel','zenith','correctionUT1'): continue
            if kind == 'pulsar' and name in ('posPulsar','posPulsarEquatorial','velPulsar','accPulsar','correctTroposphere'): continue
            text += [f' if(!same(a.{name},b.{name})) {{ std::cerr << "changed {kind}.{name}\\n"; return false; }}']
        text += [' return true;','}']
    path.write_text('\n'.join(text)+'\n')

def compile_probe(name='ordinary', mutation=None, coverage=False, caller=None, observer=False):
    folder = BUILD/name; src = folder/'source'; src.mkdir(parents=True, exist_ok=True)
    for file in FILES: (src/file).write_bytes(blob(file))
    (src/'TKlongdouble.h').write_bytes(blob('TKlongdouble.float128.h'))
    config = (ROOT/'validation/reference/build/source/config.h').read_bytes()
    assert sha(config) == '470c61b68a61504558e76ef772639e46b565a1e4500115d6e3fb6a71286dd69e'
    (src/'config.h').write_bytes(config)
    archive = ROOT/'validation/reference/build/libtempo2-reference.a'
    assert sha(archive.read_bytes()) == '313507c5063cb6a8156a0e5fd0f752e2baa59a77c78b5b32822d8efeddd2dbf5'
    g = blob('global.C')
    (src/'global_data.C').write_bytes(g[:g.index(b'void extra_delays')]+g[g.index(b'void CVSdisplayVersion'):])
    from inventory import sofa_closure
    sofa = sofa_closure()
    (src/'sofa').mkdir(exist_ok=True)
    for n in sofa['closure']: (src/'sofa'/(n+'.for')).write_bytes(blob('sofa/'+n+'.for'))
    if observer:
        for file,old,new in OBSERVERS:
            p=src/file; s=p.read_text(); assert s.count(old)==1
            p.write_text(s.replace(old,new))
    if mutation:
        file, old, new = mutation
        p=src/file; s=p.read_text(); assert s.count(old)==1,(file,old)
        p.write_text(s.replace(old,new))
    generate_checks(src/'field_checks.h')
    flags = ['-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(src)]
    if observer: flags += ['-DX_OBSERVER']
    if coverage: flags += ['--coverage','-fno-inline']
    units = [caller or HERE/'probe.cc', *[src/n for n in [*PRIMARY,'eop.C','TKlog.C','TKlongdouble.C','tempo2Util.C','global_data.C']]]
    commands=[]; dependencies=set(); objects=[]
    for unit in units:
        obj=folder/(unit.stem+'.o'); dep=obj.with_suffix('.d')
        cmd=[CC,*flags,'-MD','-MF',str(dep),'-c',str(unit),'-o',str(obj)]
        r=subprocess.run(cmd,capture_output=True,text=True,env=ENV)
        (folder/(unit.stem+'.compile.log')).write_text(r.stdout+r.stderr)
        assert r.returncode==0, r.stderr
        commands.append(cmd); objects.append(obj)
        dependencies.update(Path(p) for p in dep.read_text().replace('\\\n','').split(':',1)[1].split())
    fflags=['-O2','-fno-second-underscore','-fPIC']
    if coverage: fflags += ['--coverage','-fno-inline']
    for name in sofa['closure']:
        obj=folder/('sofa_'+name+'.o')
        cmd=[FC,*fflags,'-c',str(src/'sofa'/(name+'.for')),'-o',str(obj)]
        r=subprocess.run(cmd,capture_output=True,text=True,env=ENV)
        assert r.returncode==0,r.stderr
        commands.append(cmd);objects.append(obj)
    exe=folder/'probe'; cmd=[CC,*map(str,objects),'-lquadmath','-lgfortran','-Wl,-map,'+str(folder/'probe.map'),'-o',str(exe)]
    if coverage: cmd += ['--coverage']
    r=subprocess.run(cmd,capture_output=True,text=True,env=ENV); assert r.returncode==0,r.stderr
    commands.append(cmd)
    runtime={str(Path(subprocess.check_output([CC,'-print-file-name='+n],text=True).strip()).resolve()):None for n in RUNTIMES}
    toolchain={}
    for program in ('cc1plus','collect2','as','ld'):
        selected=subprocess.check_output([CC,'-print-prog-name='+program],text=True).strip()
        toolchain[program]=str(Path(selected if '/' in selected else shutil.which(selected)).resolve())
    toolchain['f951']=subprocess.check_output([FC,'-print-prog-name=f951'],text=True).strip()
    toolchain['gfortran']=str(Path(FC).resolve())
    for program in ('as','ld'):
        toolchain['xcrun_'+program]=subprocess.check_output(['xcrun','-f',program],text=True).strip()
    toolchain['python']=str(Path(sys.executable).resolve())
    tracked = {*dependencies, Path(CC).resolve(), *map(Path,runtime), *map(Path,toolchain.values()),
               *[p for p in src.rglob('*') if p.is_file()], *HERE.glob('*.py'), HERE/'CONTRACT.md', *(HERE/'outputs/frozen').iterdir()}
    hashes={str(p):sha(p.read_bytes()) for p in sorted(tracked) if p.is_file()}
    macros=subprocess.check_output([CC,*flags,'-dM','-E','-x','c++','-include',str(src/'tempo2.h'),'/dev/null'],env=ENV)
    (folder/'macros.txt').write_bytes(macros)
    profile=dict(pin=PIN, release='64cab7fc47f72d054a5d3af61441f20f069d9962', commands=commands, flags=flags,
        compiler_version=subprocess.check_output([CC,'--version'],text=True),
        toolchain=toolchain,python_version=sys.version,
        host=subprocess.check_output(['sw_vers'],text=True), machine=platform.machine(), locale='C',
        archive_context_sha256=sha(archive.read_bytes()), archive_linked=False,
        global_slices=['global.C:1-64','global.C:176-200'], macros_sha256=sha(macros),
        dependencies=hashes, executable_sha256=sha(exe.read_bytes()),
        link_map_sha256=sha((folder/'probe.map').read_bytes()),
        dynamic_libraries=subprocess.check_output(['otool','-L',str(exe)],text=True).splitlines()[1:],
        runtime_files=list(runtime), coverage=coverage, mutation=mutation, observer=observer,
        observer_patch=OBSERVERS if observer else [], sofa=sofa, fortran_flags=fflags,
        fortran_version=subprocess.check_output([FC,'--version'],text=True),
        primary_symbols=subprocess.check_output(['nm','-u',str(folder/'get_obsCoord.o')],text=True),
        sofa_symbols=subprocess.check_output(['nm',*[str(o) for o in objects if o.name.startswith('sofa_')]],text=True))
    assert '\n_fortran_mod\n' in '\n'+profile['primary_symbols'], 'public binary128 overload not referenced'
    assert '_Z11fortran_moddd' not in profile['primary_symbols'], 'binary64 overload leaked into primary TU'
    save(folder/'profile.json',profile)
    return exe, profile

def actual_identity(exe, profile):
    assert sha(exe.read_bytes()) == profile['executable_sha256'], 'executed binary changed'
    for p,h in profile['dependencies'].items():
        assert sha(Path(p).read_bytes()) == h, 'executed dependency changed: '+p
    return dict(executable_sha256=sha(exe.read_bytes()), dependency_count=len(profile['dependencies']))

def stable(profile):
    # macOS linker UUIDs/map bytes can differ; each binary is checked at launch.
    return {k:v for k,v in profile.items() if k not in ('executable_sha256','link_map_sha256')}
