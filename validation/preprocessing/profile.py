"""Raw pinned source build and current execution identities, owned by Assignment P."""
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import shutil
import shlex
import subprocess
import sys

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
BUILD = HERE/'build'
PIN = 'fda6ad4a9753ad7dc5d291406f2f310a9161423f'
PRIMARY = ['preProcess.C','preProcessSimple.C','units.C','updateEpoch.C']
SUPPORT = ['initialise.C','readParfile.C','TKlongdouble.C','tempo2Util.C','TKmatrix.C','TKlog.C','clkcorr.C','tabulatedfunction.C','observatory.C']
FILES = PRIMARY + SUPPORT + ['tempo2.h','tempo2Util.h','TKlongdouble.float128.h','TKlongdouble.ld.h','TKlog.h','TKmatrix.h','T2accel.h','t2fit_nestlike.h','tabulatedfunction.h','TKfit.h','ifteph.h','global.C']
UNITS = PRIMARY + SUPPORT + ['global_data.C']
ENV = {**os.environ, 'LC_ALL':'C', 'LANG':'C'}
ENV.pop('TEMPO2_ALIAS',None)
RUNNERS = ['profile.py','fixtures.py','run.py','verify.py','coverage.py','inventory.py','probe.cc','callback.cc','inspect.py']


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
    for kind in ['pulsar','observation']:
        (src/(kind+'_fields.h')).write_text(''.join('FIELD('+n+');\n' for n in fields(kind)))
    from inventory import enum_names, projected_fields
    (src/'names.h').write_text('static const char *names[] = {'+', '.join(json.dumps(n) for n in enum_names())+'};\n')
    (src/'projected_fields.h').write_text(''.join('FIELD('+n+');\n' for n in projected_fields()))
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
    caller_command=None;caller=str(HERE/'probe.cc')
    if coverage:
        caller=str(BUILD/(name+'-caller.o'))
        caller_command=[cc,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(src),'-c',str(HERE/'probe.cc'),'-o',caller]
        subprocess.run(caller_command,capture_output=True,text=True,env=ENV,check=True)
    cmd=[cc,*flags,caller,*[str(src/n) for n in UNITS],'-lquadmath','-Wl,-map,'+str(BUILD/(name+'.map')),'-o',str(exe)]
    p=subprocess.run(cmd,capture_output=True,text=True,env=ENV)
    (BUILD/(name+'.compile.log')).write_text(p.stdout+p.stderr)
    if p.returncode:raise RuntimeError(p.stderr)
    plugins={}
    (BUILD/'plugins').mkdir(exist_ok=True)
    for kind,defines in [('ok',[]),('noversion',['-DOMIT_VERSION']),('wrong',['-DWRONG_VERSION'])]:
        target=BUILD/'plugins'/(kind+'_fixture_splug.t2')
        pcmd=[cc,'-O2','-std=gnu++11','-dynamiclib','-DHAVE_CONFIG_H','-I'+str(src),*defines,str(HERE/'callback.cc'),'-o',str(target)]
        subprocess.run(pcmd,capture_output=True,text=True,env=ENV,check=True)
        plugins[kind]=dict(path=str(target),sha256=sha(target.read_bytes()),defines=defines,command=[a.replace(str(src),'{source}') for a in pcmd])
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
    stable=dict(python=dict(path=str(Path(sys.executable).resolve()),sha256=sha(Path(sys.executable).read_bytes()),version=sys.version),caller_optimization='-O2',plugins=plugins,pin=PIN,compiler=subprocess.check_output([cc,'--version'],text=True).splitlines()[0],compiler_sha256=sha(Path(cc).read_bytes()),
                flags=[f.replace(str(src),'{source}') for f in flags],platform=platform.platform(),machine=platform.machine(),locale='C',
                preprocessed_normalized_sha256=pre_normalized,compiler_backend=dict(path=str(cc1plus),sha256=sha(cc1plus.read_bytes())),header_dependencies=header_dependencies,
                sources={n:sha((src/n).read_bytes()) for n in FILES},generated={n:sha((src/n).read_bytes()) for n in ['config.h','global_data.C','TKlongdouble.h','pulsar_fields.h','observation_fields.h','names.h','projected_fields.h']},
                runtime={n:dict(path=p,sha256=sha(Path(p).read_bytes())) for n,p in libraries.items()},dependencies=dependencies,
                runner_sources=identity,macros_sorted_sha256=sha(b'\n'.join(sorted(macros.splitlines()))),
                global_slice=dict(byte_start=0,byte_end=blob('global.C').index(b'void extra_delays'),lines='1-64 and CVSdisplayVersion to EOF',cvs_byte_start=blob('global.C').index(b'void CVSdisplayVersion'),cvs_byte_end=len(blob('global.C'))),coverage=coverage)
    profile=dict(stable=stable,caller_command=caller_command,command=cmd,executable_path=str(exe),executable_sha256=sha(exe.read_bytes()),link_map_sha256=sha((BUILD/(name+'.map')).read_bytes()),
                 preprocessed_sha256=pre,macros_raw_sha256=sha(macros),compile_log_sha256=sha((BUILD/(name+'.compile.log')).read_bytes()),mutation=mutation)
    assert identity==runners(),'runner changed during compilation'
    return exe,profile
