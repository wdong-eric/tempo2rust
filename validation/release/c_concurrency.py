"""AL installed C caller exclusivity; unchanged AK assertions and C-only sanitizer scope."""
from pathlib import Path
from cache_lifetime import prepare
import hashlib,json,os,re,shlex,subprocess,sys
ROOT=Path(__file__).resolve().parents[2];HERE=ROOT/'validation/parallel_execution'
import argparse
p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True);p.add_argument('--prefix',type=Path,required=True);a=p.parse_args()
out=a.out.resolve();out.mkdir(parents=True,exist_ok=False);prefix=a.prefix.resolve()
# AJ's standalone client expects its test-local callback count to start at zero.
# Retain every assertion while making repeated main calls a valid test harness.
consumer=ROOT/'validation/library_interfaces/consumer.c'
raw=consumer.read_text();entry='int main(int argc,char **argv){'
assert raw.count(entry)==1
(out/'consumer.c').write_text(raw.replace(entry,entry+'\n    calls=0;',1))
install_receipt=prefix.parent/(prefix.name+'-build')/'install.json';libs=json.loads(install_receipt.read_text())['native_static_libs']
archive=prefix/'lib/libtempo2pred_rust.a';records=[]
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
# A separate diagnostic caller releases MPFR's documented thread-local caches
# before thread exit. Original callers and their leak result remain the gate.
# Exact MPFR 4.2.2 mpfr.h enum/prototype; the installed Rust archive owns MPFR.
caller=HERE/'predictor_threads.c';original=caller.read_text()
# LSan can terminate without libc's normal final stdout flush. Preserve the
# completed evidence in both callers before shutdown, including the last line.
end='    puts("AK four caller threads completed twelve exclusive full workflows");'
assert original.count(end)==1
flushed=original.replace(end,end+'\n    assert(fflush(stdout)==0);')
(out/'flushed_callers.c').write_text(flushed)
cleanup='''typedef enum { MPFR_FREE_LOCAL_CACHE=1, MPFR_FREE_GLOBAL_CACHE=2 } mpfr_free_cache_t;
extern void mpfr_free_cache2(mpfr_free_cache_t);
'''
assert original.count('    return NULL;')==1
(out/'cache_cleanup.c').write_text(cleanup+flushed.replace('    return NULL;',
    '    mpfr_free_cache2(MPFR_FREE_LOCAL_CACHE);\n    return NULL;'))
cache_control={};ownership_control={}
def run(label,cmd,cwd,timeout=120):
 cmd=list(map(str,cmd));r=subprocess.run(cmd,cwd=cwd,capture_output=True,timeout=timeout,env=dict(os.environ,UBSAN_OPTIONS='halt_on_error=1:print_stacktrace=1',TSAN_OPTIONS='halt_on_error=1'))
 (out/(label+'.stdout')).write_bytes(r.stdout);(out/(label+'.stderr')).write_bytes(r.stderr);records.append(dict(label=label,command=cmd,exit=r.returncode));return r
for mode,extra in [('plain',[]),('asan',['-fsanitize=address,undefined']),('tsan',['-fsanitize=thread'])]:
 wd=out/mode;wd.mkdir();flags=['-D_GNU_SOURCE','-std=c11','-O1','-g','-fno-omit-frame-pointer','-I'+str(prefix/'include'),*extra];objects=[]
 for name in ['consumer','storage','low_count','boundary']:
  source=out/'consumer.c' if name=='consumer' else ROOT/'validation/library_interfaces'/(name+'.c')
  obj=wd/(name+'.o');r=run(mode+'-'+name,['clang',*flags,'-Dmain=ak_'+name+'_main','-c',source,'-o',obj],wd);assert r.returncode==0,r.stderr;objects.append(obj)
 obj=wd/'predictor.o';r=run(mode+'-boundary-shim',['clang',*flags,'-c',prefix/'share/tempo2-rust/rust/native/predictor.c','-o',obj],wd);assert r.returncode==0,r.stderr;objects.append(obj)
 exe=wd/'callers';r=run(mode+'-link',['clang',*flags,out/'flushed_callers.c',*objects,archive,*libs,'-pthread','-o',exe],wd);assert r.returncode==0,r.stderr
 r=run(mode+'-execute',[exe],wd)
 if r.returncode!=0:
  print(mode,'failed',r.returncode,r.stderr[-1500:].decode(errors='replace'),flush=True)
 else:
  assert r.stdout.count(b'AK_BEGIN')==12 and r.stdout.count(b'AK_END')==12
  assert b'ERROR: AddressSanitizer' not in r.stderr and b'runtime error:' not in r.stderr and b'WARNING: ThreadSanitizer' not in r.stderr
  print(mode,'twelve complete caller-exclusive workflows passed',flush=True)
 if mode=='asan':
  original_result=r
  original_files={p.name:sha(p) for p in wd.glob('*.tsv')}
  clean=out/'asan-cache-cleanup';clean.mkdir();exe=clean/'callers'
  r=run('asan-cache-cleanup-link',['clang',*flags,out/'cache_cleanup.c',*objects,archive,*libs,'-pthread','-o',exe],clean);assert r.returncode==0,r.stderr
  corrected=run('asan-cache-cleanup-execute',[exe],clean)
  # Caller ordering is scheduled; each complete id/pass transcript and all
  # storage products must agree. Do not remove LSan reports from saved stderr.
  blocks=lambda data:sorted(re.findall(rb'AK_BEGIN (\d+) (\d+)\n(.*?)AK_END \1 \2\n',data,re.S))
  before,after=blocks(original_result.stdout),blocks(corrected.stdout)
  cache_control=dict(original_exit=original_result.returncode,cleanup_exit=corrected.returncode,
      complete_workflows=len(before)==len(after)==12,workflow_output_equal=before==after,
      storage_equal=original_files=={p.name:sha(p) for p in clean.glob('*.tsv')},
      interpretation='Diagnostic cache-lifetime intervention only; original sanitizer failures remain failures. No production leak fix or full-dependency sanitizer coverage is established.')
  if corrected.returncode==0 and all(cache_control[k] for k in ['complete_workflows','workflow_output_equal','storage_equal']):
   guarded,identity=prepare(prefix,out,run)
   owned=out/'asan-library-owned';owned.mkdir();exe=owned/'callers'
   r=run('asan-library-owned-link',['clang',*flags,out/'flushed_callers.c',*objects,guarded,*libs,'-pthread','-o',exe],owned);assert r.returncode==0,r.stderr
   result=run('asan-library-owned-execute',[exe],owned)
   ownership_control=dict(identity=identity,exit=result.returncode,complete_workflows=len(blocks(result.stdout))==12,
      workflow_output_equal=blocks(result.stdout)==before,
      storage_equal=original_files=={p.name:sha(p) for p in owned.glob('*.tsv')})
  else:ownership_control={'unavailable':'Incomplete flushed baseline/explicit-cleanup proof'}
(out/'receipt.json').write_text(json.dumps(dict(scope='C clients and C shim instrumented; Rust/MPFR/Fortran/providers are not sanitizer-instrumented',cache_control=cache_control,ownership_control=ownership_control,records=records,inputs={str(p):sha(p) for p in [archive,prefix/'share/tempo2-rust/rust/native/predictor.c',caller,*[ROOT/'validation/library_interfaces'/(n+'.c') for n in ['consumer','storage','low_count','boundary']]]},files={str(p.relative_to(out)):sha(p) for p in out.rglob('*') if p.is_file() and not any(part in p.relative_to(out).parts for part in ['library-owned-cache-source','cache-target'])}),indent=2)+'\n')
assert all(r['exit']==0 for r in records)
assert cache_control['complete_workflows'] and cache_control['workflow_output_equal'] and cache_control['storage_equal']
assert ownership_control['complete_workflows'] and ownership_control['workflow_output_equal'] and ownership_control['storage_equal']
