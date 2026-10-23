"""O supplemental source build: unchanged pinned units and controlled storage."""
import importlib.util, os, shlex, shutil, subprocess
from pathlib import Path
from run import ROOT,HERE,OUT,ENV,sha,save
PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f'

def build():
    spec=importlib.util.spec_from_file_location('k_profile',ROOT/'validation/parameters/profile.py');k=importlib.util.module_from_spec(spec);spec.loader.exec_module(k)
    k.BUILD=HERE/'build/native';src=k.prepare()
    exe=k.BUILD/'probe';exe.unlink(missing_ok=True);cc=Path(shutil.which('g++-16')).resolve()
    flags=['-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(src)]
    units=[HERE/'probe.cc',*[src/n for n in k.SOURCES],src/'global_data.C']
    command=[str(cc),*flags,*map(str,units),'-lquadmath','-o',str(exe)]
    proc=subprocess.run(command,capture_output=True,text=True,env=ENV);save(OUT/'native_compile.json',dict(command=command,returncode=proc.returncode,stdout=proc.stdout,stderr=proc.stderr));assert proc.returncode==0,proc.stderr
    dependencies={}
    for u in units:
        dep=subprocess.check_output([str(cc),*flags,'-M',str(u)],env=ENV,text=True)
        for p in shlex.split(dep.split(':',1)[1].replace('\\\n','')):
            if Path(p).is_file():dependencies[p]=sha(Path(p))
    libraries={}
    for n in ['libquadmath.0.dylib','libstdc++.6.dylib','libgcc_s.1.1.dylib']:
        p=Path(subprocess.check_output([str(cc),'-print-file-name='+n],text=True).strip()).resolve();libraries[str(p)]=sha(p)
    backend=Path(subprocess.check_output([str(cc),'-print-prog-name=cc1plus'],text=True).strip()).resolve()
    identity=dict(pin=PIN,compiler=str(cc),compiler_sha256=sha(cc),backend=str(backend),backend_sha256=sha(backend),compiler_version=subprocess.check_output([str(cc),'--version'],text=True),sources={str(p):sha(p) for p in src.iterdir() if p.is_file()},headers=dependencies,runtime=libraries,executable=str(exe),executable_sha256=sha(exe),runner_sha256=sha(Path(__file__)),harness_sha256=sha(HERE/'probe.cc'))
    return exe,identity

def validate(identity):
    for key in ['compiler','backend','executable']:
        assert sha(Path(identity[key]))==identity[key+'_sha256'], 'native '+key+' changed'
    for key in ['sources','headers','runtime']:
        assert {p:sha(Path(p)) for p in identity[key]}==identity[key], 'native '+key+' changed'
    assert sha(Path(__file__))==identity['runner_sha256']
    assert sha(HERE/'probe.cc')==identity['harness_sha256']
