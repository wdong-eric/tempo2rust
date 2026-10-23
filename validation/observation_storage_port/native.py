"""R native header/assignment probe, staged exclusively beneath R output."""
import importlib.util,shlex,shutil,subprocess
from pathlib import Path
from run import ROOT,HERE,OUT,ENV,sha,save

def build(profile):
 spec=importlib.util.spec_from_file_location('k_profile',ROOT/'validation/parameters/profile.py');k=importlib.util.module_from_spec(spec);spec.loader.exec_module(k)
 k.BUILD=OUT/'build'/profile;src=k.prepare()
 exe=k.BUILD/'probe';exe.unlink(missing_ok=True);cc=Path(shutil.which('g++-16')).resolve()
 flags=['-O0' if profile=='debug' else '-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(src)]
 cmd=[str(cc),*flags,str(HERE/'probe.cc'),'-lquadmath','-o',str(exe)]
 p=subprocess.run(cmd,capture_output=True,text=True,env=ENV);save(OUT/(profile+'_native_compile.json'),dict(command=cmd,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr));assert p.returncode==0,p.stderr
 dep=subprocess.check_output([str(cc),*flags,'-M',str(HERE/'probe.cc')],env=ENV,text=True)
 dependencies={p:sha(Path(p)) for p in shlex.split(dep.split(':',1)[1].replace('\\\n','')) if Path(p).is_file()}
 libs={}
 for name in ['libquadmath.0.dylib','libstdc++.6.dylib','libgcc_s.1.1.dylib']:
  p=Path(subprocess.check_output([str(cc),'-print-file-name='+name],text=True).strip()).resolve();libs[str(p)]=sha(p)
 backend=Path(subprocess.check_output([str(cc),'-print-prog-name=cc1plus'],text=True).strip()).resolve()
 ident=dict(pin=k.PIN,compiler=str(cc),compiler_sha256=sha(cc),backend=str(backend),backend_sha256=sha(backend),compiler_version=subprocess.check_output([str(cc),'--version'],text=True),headers=dependencies,runtime=libs,executable=str(exe),executable_sha256=sha(exe),runner_sha256=sha(Path(__file__)),harness_sha256=sha(HERE/'probe.cc'),profile_helper_sha256=sha(ROOT/'validation/parameters/profile.py'))
 return exe,ident

def validate(identity):
 for key in ['compiler','backend','executable']:assert sha(Path(identity[key]))==identity[key+'_sha256'],'native '+key+' changed'
 for key in ['headers','runtime']:assert {p:sha(Path(p)) for p in identity[key]}==identity[key],'native '+key+' changed'
 assert sha(Path(__file__))==identity['runner_sha256'];assert sha(HERE/'probe.cc')==identity['harness_sha256'];assert sha(ROOT/'validation/parameters/profile.py')==identity['profile_helper_sha256']
