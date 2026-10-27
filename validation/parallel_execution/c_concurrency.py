"""Bounded actual C callers with caller exclusivity; C-only sanitizer scope."""
from pathlib import Path
import hashlib,json,os,re,shlex,subprocess,sys
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
out=Path(sys.argv[1]).resolve();out.mkdir(parents=True,exist_ok=False)
# AJ's standalone client expects its test-local callback count to start at zero.
# Retain every assertion while making repeated main calls a valid test harness.
consumer=ROOT/'validation/library_interfaces/consumer.c'
raw=consumer.read_text();entry='int main(int argc,char **argv){'
assert raw.count(entry)==1
(out/'consumer.c').write_text(raw.replace(entry,entry+'\n    calls=0;',1))
text=(HERE/'build/native-static-libs.log').read_text();libs=shlex.split(re.findall('native-static-libs: (.*)',text)[-1])
archive=ROOT/'rust/target/release/libtempo2_rust.a';records=[]
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
def run(label,cmd,cwd):
 cmd=list(map(str,cmd));r=subprocess.run(cmd,cwd=cwd,capture_output=True,timeout=120,env=dict(os.environ,UBSAN_OPTIONS='halt_on_error=1:print_stacktrace=1',TSAN_OPTIONS='halt_on_error=1'))
 (out/(label+'.stdout')).write_bytes(r.stdout);(out/(label+'.stderr')).write_bytes(r.stderr);records.append(dict(label=label,command=cmd,exit=r.returncode));return r
for mode,extra in [('plain',[]),('asan',['-fsanitize=address,undefined']),('tsan',['-fsanitize=thread'])]:
 wd=out/mode;wd.mkdir();flags=['-std=c11','-O1','-g','-fno-omit-frame-pointer','-I'+str(ROOT/'rust/include'),*extra];objects=[]
 for name in ['consumer','storage','low_count','boundary']:
  source=out/'consumer.c' if name=='consumer' else ROOT/'validation/library_interfaces'/(name+'.c')
  obj=wd/(name+'.o');r=run(mode+'-'+name,['clang',*flags,'-Dmain=ak_'+name+'_main','-c',source,'-o',obj],wd);assert r.returncode==0,r.stderr;objects.append(obj)
 obj=wd/'predictor.o';r=run(mode+'-boundary-shim',['clang',*flags,'-c',ROOT/'rust/native/predictor.c','-o',obj],wd);assert r.returncode==0,r.stderr;objects.append(obj)
 exe=wd/'callers';r=run(mode+'-link',['clang',*flags,HERE/'predictor_threads.c',*objects,archive,*libs,'-pthread','-o',exe],wd);assert r.returncode==0,r.stderr
 r=run(mode+'-execute',[exe],wd)
 if r.returncode!=0:
  print(mode,'failed',r.returncode,r.stderr[-1500:].decode(errors='replace'),flush=True)
 else:
  assert r.stdout.count(b'AK_BEGIN')==12 and r.stdout.count(b'AK_END')==12
  assert b'ERROR: AddressSanitizer' not in r.stderr and b'runtime error:' not in r.stderr and b'WARNING: ThreadSanitizer' not in r.stderr
  print(mode,'twelve complete caller-exclusive workflows passed',flush=True)
(out/'receipt.json').write_text(json.dumps(dict(scope='C clients and C shim instrumented; Rust/MPFR/Fortran/providers are not sanitizer-instrumented',records=records,inputs={str(p):sha(p) for p in [archive,ROOT/'rust/native/predictor.c',HERE/'predictor_threads.c',*[ROOT/'validation/library_interfaces'/(n+'.c') for n in ['consumer','storage','low_count','boundary']]]},files={str(p.relative_to(out)):sha(p) for p in out.rglob('*') if p.is_file()}),indent=2)+'\n')
assert all(r['exit']==0 for r in records)
