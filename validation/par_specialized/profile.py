"""Raw pinned source build and current execution identities, owned by Assignment Q."""
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
K_COMMIT = '38d62af9c183a0bb3294638999ca63cb431fc046'
RELEASE = 'ad6fbf6'
FILES = ['initialise.C','readParfile.C','TKlongdouble.C','tempo2Util.C','TKmatrix.C','TKlog.C','clkcorr.C','tabulatedfunction.C','observatory.C','tempo2.h','TKlongdouble.float128.h','TKlongdouble.ld.h','TKlog.h','TKmatrix.h','T2accel.h','tempo2Util.h','t2fit_nestlike.h','tabulatedfunction.h','global.C','TKfit.h']
PRIMARY = ['readParfile.C','t2fit_nestlike.h']
UNITS = [n for n in FILES if n.endswith('.C') and n!='global.C']+['global_data.C']
ENV = {**os.environ, 'LC_ALL':'C', 'LANG':'C'}
ENV.pop('TEMPO2_ALIAS',None)
RUNNERS = ['profile.py','fixtures.py','run.py','verify.py','coverage.py','inventory.py','probe.cc','protected.py','analyze.py']


def sha(b):return hashlib.sha256(b).hexdigest()


def save(p,x):
    p.parent.mkdir(parents=True,exist_ok=True)
    p.write_text(json.dumps(x,indent=2,sort_keys=True,allow_nan=False)+'\n')


def blob(n):return subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':'+n])


def project_blob(name,revision=K_COMMIT):return subprocess.check_output(['git','-C',str(ROOT),'show',revision+':'+name])

def runners():return {n:sha((HERE/n).read_bytes()) for n in RUNNERS}


def prepare(name='source',mutation=None):
    src=BUILD/name
    if src.exists():shutil.rmtree(src)
    src.mkdir(parents=True)
    for n in FILES:(src/n).write_bytes(blob(n))
    (src/'TKlongdouble.h').write_bytes(blob('TKlongdouble.float128.h'))
    config=json.loads(project_blob('validation/parameters/profile.json'))['generated']['config.h']
    assert sha(config['text'].encode())==config['sha256']
    (src/'config.h').write_bytes(config['text'].encode())
    g=blob('global.C');end=g.index(b'void extra_delays')
    (src/'global_data.C').write_bytes(g[:end]+g[g.index(b'void CVSdisplayVersion'):])
    from inventory import projection
    (src/'fields.h').write_text(''.join('FIELD('+n+');\n' for n in projection()))
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
    caller_flags=[('-O2' if f=='-O0' else f) for f in flags]
    commands=[]
    if coverage:
        objects=[]
        for unit in [HERE/'probe.cc',*[src/n for n in UNITS]]:
            obj=BUILD/(name+'-'+unit.stem+'.o');objects.append(str(obj))
            commands.append([cc,*(caller_flags if unit.name=='probe.cc' else flags),'-c',str(unit),'-o',str(obj)])
        cmd=[cc,*flags,*objects,'-lquadmath','-Wl,-map,'+str(BUILD/(name+'.map')),'-o',str(exe)]
    else:
        cmd=[cc,*flags,str(HERE/'probe.cc'),*[str(src/n) for n in UNITS],'-lquadmath','-Wl,-map,'+str(BUILD/(name+'.map')),'-o',str(exe)]
    commands.append(cmd);logs=[]
    for command in commands:
        p=subprocess.run(command,capture_output=True,text=True,env=ENV);logs.append(p.stdout+p.stderr)
        (BUILD/(name+'.compile.log')).write_text(''.join(logs))
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
        deps=subprocess.check_output([cc,*(caller_flags if unit==str(HERE/'probe.cc') else flags),'-M',unit],text=True,env=ENV)
        for dep in shlex.split(deps.split(':',1)[1].replace(chr(92)+chr(10),'')):
            path=Path(dep)
            if path.exists():header_dependencies[str(path).replace(str(src),'{source}')]=sha(path.read_bytes())
    cc1plus=Path(subprocess.check_output([cc,'-print-prog-name=cc1plus'],text=True).strip()).resolve()
    dependencies=subprocess.check_output(['otool','-L',str(exe)],text=True).splitlines()[1:]
    stable=dict(pin=PIN,compiler=subprocess.check_output([cc,'--version'],text=True).splitlines()[0],compiler_sha256=sha(Path(cc).read_bytes()),
                flags=[f.replace(str(src),'{source}') for f in flags],caller_flags=[f.replace(str(src),'{source}') for f in caller_flags],platform=platform.platform(),machine=platform.machine(),locale='C',
                preprocessed_normalized_sha256=pre_normalized,compiler_backend=dict(path=str(cc1plus),sha256=sha(cc1plus.read_bytes())),header_dependencies=header_dependencies,
                sources={n:sha((src/n).read_bytes()) for n in FILES},generated={n:sha((src/n).read_bytes()) for n in ['config.h','global_data.C','TKlongdouble.h','fields.h']},
                runtime={n:dict(path=p,sha256=sha(Path(p).read_bytes())) for n,p in libraries.items()},dependencies=dependencies,
                runner_sources=identity,macros_sorted_sha256=sha(b'\n'.join(sorted(macros.splitlines()))),
                global_slice=dict(byte_start=0,byte_end=blob('global.C').index(b'void extra_delays'),lines='1-64 and CVSdisplayVersion to EOF'),coverage=coverage)
    profile=dict(stable=stable,command=cmd,compile_commands=commands,executable_path=str(exe),executable_sha256=sha(exe.read_bytes()),link_map_sha256=sha((BUILD/(name+'.map')).read_bytes()),
                 preprocessed_sha256=pre,macros_raw_sha256=sha(macros),compile_log_sha256=sha((BUILD/(name+'.compile.log')).read_bytes()),mutation=mutation)
    assert identity==runners(),'runner changed during compilation'
    return exe,profile
