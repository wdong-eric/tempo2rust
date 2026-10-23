import copy,hashlib,importlib,json,shlex,subprocess,sys,tempfile
from pathlib import Path
sys.dont_write_bytecode=True
root=Path(__file__).resolve().parents[3];revision='ccdf55c8143d0986c5af478d74db329286f57354'
result={'reviewed_revision':subprocess.check_output(['git','rev-parse',revision],cwd=root,text=True).strip(),'checks':[]}
with tempfile.TemporaryDirectory(prefix='tempo2-parameter-identity-',dir='/tmp') as td:
 tmp=Path(td).resolve();here=tmp/'validation/parameters';here.mkdir(parents=True)
 for name in subprocess.check_output(['git','ls-tree','-r','--name-only',revision,'--','validation/parameters'],cwd=root,text=True).splitlines():
  f=tmp/name;f.parent.mkdir(parents=True,exist_ok=True);f.write_bytes(subprocess.check_output(['git','show',revision+':'+name],cwd=root))
 for name in ['tempo2','decisions','handoffs']:(tmp/name).symlink_to(root/name,target_is_directory=True)
 for name in ['reference','binary','coverage']:(tmp/'validation'/name).symlink_to(root/'validation'/name,target_is_directory=True)
 sys.path.insert(0,str(here));p=importlib.import_module('profile');run=importlib.import_module('run')
 print('Building corrected K probe in temporary storage',flush=True)
 exe,profile=p.compile_probe(p.prepare());expected=json.loads((here/'profile.json').read_text())
 expected['command']=[s.replace(str(root),str(tmp)) for s in expected['command']]
 identity=run.check_identity(expected,profile,exe)
 result['checks'].append({'name':'fresh build','accepted':True});result['executed_profile']=profile;result['execution_identity']=identity
 cases=json.loads((here/'cases.json').read_text());gold=json.loads((here/'observations.json').read_text())
 case=next(c for c in cases if c['id']=='numeric');observed=run.execute(case,exe,profile);run.validate(case,observed);run.compare(next(c for c in gold if c['id']=='numeric'),observed)
 result['numeric_positive_control']='exact'
 for name,keys in [('compiler',['compiler_sha256']),('source',['sources','readParfile.C']),('runtime',['runtime_hashes','libquadmath.0.dylib']),('runner',['runner_sources','run.py']),('harness',['runner_sources','probe.cc'])]:
  changed=copy.deepcopy(profile);parent=changed
  for key in keys[:-1]:parent=parent[key]
  parent[keys[-1]]='0'*64
  try:run.check_identity(expected,changed,exe)
  except AssertionError as error:result['checks'].append({'name':'wrong '+name,'rejected':True,'diagnostic':str(error)})
  else:raise AssertionError('accepted wrong '+name)
 native=exe.with_name('reviewed_native');exe.rename(native)
 try:
  exe.write_text('#!/bin/sh\nexec '+shlex.quote(str(native))+' "$@"\n');exe.chmod(0o700)
  try:run.check_identity(expected,profile,exe)
  except AssertionError as error:
   assert 'execution.executable_sha256' in str(error)
   result['checks'].append({'name':'original forwarding wrapper defect','rejected':True,'diagnostic':str(error),'wrapper_sha256':p.sha(exe.read_bytes())})
  else:raise AssertionError('accepted wrapper')
 finally:exe.unlink();native.rename(exe)
 run.check_identity(expected,profile,exe)
 receipt=here/'regeneration.json';before=(p.sha(receipt.read_bytes()),receipt.stat().st_mtime_ns)
 proc=subprocess.run([sys.executable,str(here/'run.py'),'--no-build'],capture_output=True,text=True,timeout=30)
 after=(p.sha(receipt.read_bytes()),receipt.stat().st_mtime_ns)
 assert proc.returncode==2 and 'unrecognized arguments: --no-build' in proc.stderr and before==after
 result['checks'].append({'name':'removed shortcut','rejected':True,'returncode':proc.returncode,'receipt_unchanged':True,'stderr':proc.stderr})
 result['status']='PASS';result['scope']='Independent focused verification of the corrected normal identity gate; numerical/source inputs were already fully replayed in central-review.json and remain unchanged.'
Path('/tmp/tempo2-parameters-identity-review.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
print('PASS',len(result['checks']),'identity controls and exact numeric positive control',flush=True)
