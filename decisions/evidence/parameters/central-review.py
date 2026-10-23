import concurrent.futures,hashlib,importlib,json,os,shlex,shutil,subprocess,sys,tempfile
from pathlib import Path
sys.dont_write_bytecode=True
root=Path(__file__).resolve().parents[3];revision='6d135816a4f57aa8019b011882a7e0d58b51bad6'
out=Path('/tmp/tempo2-parameters-central-review.json')
result={'reviewed_revision':subprocess.check_output(['git','rev-parse',revision],cwd=root,text=True).strip(),'scope':'Independent fresh replay of K ordinary/normal-exit cases; no undefined diagnostic executed.'}
with tempfile.TemporaryDirectory(prefix='tempo2-parameters-review-',dir='/tmp') as td:
 tmp=Path(td).resolve();here=tmp/'validation/parameters';here.mkdir(parents=True)
 names=subprocess.check_output(['git','ls-tree','-r','--name-only',revision,'--','validation/parameters'],cwd=root,text=True).splitlines()
 for name in names:
  p=tmp/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(subprocess.check_output(['git','show',revision+':'+name],cwd=root))
 for name in ['tempo2','decisions','handoffs']:(tmp/name).symlink_to(root/name,target_is_directory=True)
 for name in ['reference','binary','coverage']:(tmp/'validation'/name).symlink_to(root/'validation'/name,target_is_directory=True)
 sys.path.insert(0,str(here));p=importlib.import_module('profile');run=importlib.import_module('run')
 cases=json.loads((here/'cases.json').read_text());expected=json.loads((here/'observations.json').read_text())
 print('Building copied pinned K probe',flush=True)
 exe,actual=p.compile_probe(p.prepare())
 baseline=json.loads((here/'profile.json').read_text())
 stable=['source_revision','compiler','compiler_sha256','command','platform','machine','locale','sources','generated','numeric_header_sha256','globals_data_sha256','probe_sha256','runtime_hashes','support_policy','mutation','coverage_instrumentation']
 for key in stable:
  old=baseline[key]
  if key=='command':old=[x.replace(str(root),str(tmp)) for x in old]
  run.compare(old,actual[key],'profile.'+key)
 def one(pair):
  c,g=pair;r=run.execute(c,exe);run.validate(c,r);run.compare(g,r,c['id']);return len(r['events'])
 with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
  event_counts=list(pool.map(one,zip(cases,expected)))
 result.update(cases=len(cases),events=sum(event_counts),fresh_replay='PASS',executed_profile=actual,stable_fields_checked=stable)
 print('Fresh replay PASS',len(cases),sum(event_counts),flush=True)
 # A harmless wrapper forwards to the already reviewed probe. --no-build must
 # not attribute its execution to a stale stored executable identity.
 native=exe.with_name('probe_forwarded_native');exe.rename(native)
 exe.write_text('#!/bin/sh\nexec '+shlex.quote(str(native))+' "$@"\n');exe.chmod(0o700)
 wrapper_hash=hashlib.sha256(exe.read_bytes()).hexdigest()
 invocation=[sys.executable,str(here/'run.py'),'--no-build']
 check=subprocess.run(invocation,cwd=tmp,capture_output=True,text=True,env={**os.environ,'PYTHONDONTWRITEBYTECODE':'1'},timeout=180)
 receipt=json.loads((here/'regeneration.json').read_text()) if check.returncode==0 else None
 result['no_build_identity_probe']={'command':invocation,'returncode':check.returncode,'stdout_tail':check.stdout.splitlines()[-2:],'stderr_tail':check.stderr[-1000:],'actual_launched_executable_sha256':wrapper_hash,'forwarded_native_sha256':hashlib.sha256(native.read_bytes()).hexdigest(),'receipt_claimed_executable_sha256':receipt['executed_profile']['executable_sha256'] if receipt else None,'receipt_cases':receipt['cases'] if receipt else None,'invalid_success_detected':check.returncode==0 and receipt['executed_profile']['executable_sha256']!=wrapper_hash,'scope':'Temporary harmless forwarding wrapper only; no source/numerical output modified. The reproduced success receipt is erroneous provenance, not accepted evidence.'}
 assert result['no_build_identity_probe']['invalid_success_detected'],result['no_build_identity_probe']
 print('Confirmed --no-build accepts a false executed identity',flush=True)
out.write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
print(out,flush=True)
