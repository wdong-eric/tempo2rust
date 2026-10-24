"""Raw pinned source build and current execution identities, owned by Assignment S."""
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import shutil
import shlex
import subprocess

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
BUILD = HERE/'build'
PIN = 'fda6ad4a9753ad7dc5d291406f2f310a9161423f'
FILES = ['TKlongdouble.C','tempo2Util.C','TKlog.C','TKlog.h','tempo2.h',
         'initialise.C','TKmatrix.C','TKmatrix.h','T2accel.h','t2fit_nestlike.h','TKfit.h','tempo2Util.h','TKlongdouble.float128.h','TKlongdouble.ld.h','global.C']
UNITS = ['TKlongdouble.C','tempo2Util.C','TKlog.C','initialise.C','TKmatrix.C','global_data.C']
ENV = {**os.environ, 'LC_ALL':'C', 'LANG':'C'}
RUNNERS = ['profile.py','fixtures.py','run.py','verify.py','coverage.py','inventory.py','generate.py','evidence.py','correction.py','probe.cc','inventory.json','cases.json','generated.h']


def sha(b):return hashlib.sha256(b).hexdigest()


def save(p,x):
    p.parent.mkdir(parents=True,exist_ok=True)
    p.write_text(json.dumps(x,indent=2,sort_keys=True,allow_nan=False)+'\n')


def blob(n):return subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':'+n])


def runners():return {n:sha((HERE/n).read_bytes()) for n in RUNNERS}


def fields(kind):
    s=blob('tempo2.h').decode()
    s=re.sub(r'/\*.*?\*/|//[^\n]*','',s,flags=re.S)
    s=s.split('typedef struct '+kind+' {',1)[1].split('} '+kind+';',1)[0]
    result=[]
    for declaration in s.split(';'):
        if not declaration.strip():continue
        # All declarations in these two source structs are simple fields, no methods.
        m=re.fullmatch(r'\s*(?:enum\s+)?\w+(?:\s+long)?\s+(.+?)\s*',declaration,re.S)
        assert m,declaration
        for item in m[1].split(','):
            m2=re.fullmatch(r'\s*\**\s*(\w+)(?:\s*\[[^\]]+\])*\s*',item)
            assert m2,item
            result.append(m2[1])
    assert len(set(result))==len(result)
    return result


def prepare(name='source',mutation=None):
    src=BUILD/name
    if src.exists():shutil.rmtree(src)
    src.mkdir(parents=True)
    for n in FILES:(src/n).write_bytes(blob(n))
    (src/'TKlongdouble.h').write_bytes(blob('TKlongdouble.float128.h'))
    (src/'config.h').write_bytes((ROOT/'validation/reference/build/source/config.h').read_bytes())
    g=blob('global.C');end=g.index(b'void extra_delays')
    (src/'global_data.C').write_bytes(g[:end]+g[g.index(b'void CVSdisplayVersion'):])
    if mutation:
        file,old,new=mutation;p=src/file;s=p.read_text()
        assert s.count(old)==1,(file,old,s.count(old))
        p.write_text(s.replace(old,new))
    return src


def compile_probe(name='probe',mutation=None,coverage=False):
    src=prepare(name+'_source',mutation);cc=shutil.which('g++-16');assert cc
    identity=runners();exe=BUILD/name;exe.unlink(missing_ok=True)
    flags=['-O0' if coverage else '-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(src)]
    if coverage:flags+=['--coverage']
    cmd=[cc,*flags,str(HERE/'probe.cc'),*[str(src/n) for n in UNITS],'-lquadmath','-Wl,-map,'+str(BUILD/(name+'.map')),'-o',str(exe)]
    p=subprocess.run(cmd,capture_output=True,text=True,env=ENV)
    (BUILD/(name+'.compile.log')).write_text(p.stdout+p.stderr)
    if p.returncode:raise RuntimeError(p.stderr)
    pre={};pre_normalized={}
    for unit in UNITS:
        data=subprocess.check_output([cc,*flags,'-E','-P',str(src/unit)],env=ENV)
        (BUILD/(name+'-'+unit+'.ii')).write_bytes(data)
        pre[unit]=sha(data)
        pre_normalized[unit]=sha(data.replace(str(src).encode(),b'{source}'))
    macros=subprocess.check_output([cc,*flags,'-dM','-E',str(src/'tempo2Util.C')],env=ENV)
    (BUILD/(name+'-macros.txt')).write_bytes(macros)
    libraries={n:str(Path(subprocess.check_output([cc,'-print-file-name='+n],text=True).strip()).resolve()) for n in ['libquadmath.0.dylib','libstdc++.6.dylib','libgcc_s.1.1.dylib']}
    header_dependencies={}
    for unit in [str(HERE/'probe.cc'),*[str(src/n) for n in UNITS]]:
        deps=subprocess.check_output([cc,*flags,'-M',unit],text=True,env=ENV)
        for dep in shlex.split(deps.split(':',1)[1].replace(chr(92)+chr(10),'')):
            path=Path(dep)
            if path.exists():header_dependencies[str(path).replace(str(src),'{source}')]=sha(path.read_bytes())
    cc1plus=Path(subprocess.check_output([cc,'-print-prog-name=cc1plus'],text=True).strip()).resolve()
    dependencies=subprocess.check_output(['otool','-L',str(exe)],text=True).splitlines()[1:]
    stable=dict(pin=PIN,compiler=subprocess.check_output([cc,'--version'],text=True).splitlines()[0],compiler_sha256=sha(Path(cc).read_bytes()),
                flags=[f.replace(str(src),'{source}') for f in flags],platform=platform.platform(),machine=platform.machine(),locale='C',
                preprocessed_normalized_sha256=pre_normalized,compiler_backend=dict(path=str(cc1plus),sha256=sha(cc1plus.read_bytes())),header_dependencies=header_dependencies,
                sources={n:sha((src/n).read_bytes()) for n in FILES},generated={n:sha((src/n).read_bytes()) for n in ['config.h','global_data.C','TKlongdouble.h','global_data.C']},
                runtime={n:dict(path=p,sha256=sha(Path(p).read_bytes())) for n,p in libraries.items()},dependencies=dependencies,
                runner_sources=identity,macros_sha256=sha(macros),
                global_slice=dict(byte_start=0,byte_end=blob('global.C').index(b'void extra_delays'),lines='1-64 plus CVSdisplayVersion suffix'),coverage=coverage)
    profile=dict(stable=stable,command=cmd,executable_path=str(exe),executable_sha256=sha(exe.read_bytes()),link_map_sha256=sha((BUILD/(name+'.map')).read_bytes()),
                 preprocessed_sha256=pre,compile_log_sha256=sha((BUILD/(name+'.compile.log')).read_bytes()),mutation=mutation)
    assert identity==runners(),'runner changed during compilation'
    return exe,profile
