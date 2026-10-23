import contextlib,copy,hashlib,importlib,json,subprocess,sys,tempfile
from collections import Counter
from pathlib import Path
sys.dont_write_bytecode=True
root=Path('/Users/Eric.Dong/Downloads/tempo2rust');revision='68de89a'
result={'reviewed_commit':subprocess.check_output(['git','rev-parse',revision],cwd=root,text=True).strip()}
with tempfile.TemporaryDirectory(prefix='n-',dir='/tmp') as td:
 base=Path(td).resolve();padding=len(str(root))-len(str(base))-1;assert padding>0
 tmp=base/('x'*padding);tmp.mkdir();assert len(str(tmp))==len(str(root))
 here=tmp/'validation/tim_io';here.mkdir(parents=True)
 for name in subprocess.check_output(['git','ls-tree','-r','--name-only',revision,'--','validation/tim_io'],cwd=root,text=True).splitlines():
  f=tmp/name;f.parent.mkdir(parents=True,exist_ok=True);f.write_bytes(subprocess.check_output(['git','show',revision+':'+name],cwd=root))
 (tmp/'tempo2').symlink_to(root/'tempo2',target_is_directory=True)
 (tmp/'validation/reference').symlink_to(root/'validation/reference',target_is_directory=True)
 sys.path.insert(0,str(here));p=importlib.import_module('profile');run=importlib.import_module('run');verify=importlib.import_module('verify')
 print('Building final N source in temporary storage',flush=True)
 exe,profile=p.compile_probe();profile['stable']['runtime_fixtures']=run.runtime();expected=json.loads((here/'profile.json').read_text())
 expected['stable']['header_dependencies']={k.replace(str(root),str(tmp)):v for k,v in expected['stable']['header_dependencies'].items()}
 original_macros=(root/'validation/tim_io/build/probe-macros.txt').read_bytes();fresh_macros=(p.BUILD/'probe-macros.txt').read_bytes()
 assert p.sha(original_macros)==expected['stable']['macros_sha256'];assert sorted(original_macros.splitlines())==sorted(fresh_macros.splitlines()),'macro definitions changed'
 result['macro_definition_equivalence']=dict(definitions=len(original_macros.splitlines()),original_sha256=p.sha(original_macros),temporary_sha256=p.sha(fresh_macros),sorted_definitions_sha256=p.sha(b'\n'.join(sorted(fresh_macros.splitlines()))),exact_multiset=True)
 expected['stable']['macros_sha256']=profile['stable']['macros_sha256']
 run.check_identity(expected,profile,exe)
 # Only the disposable expected build metadata is adapted. Input/result goldens stay byte-identical.
 p.save(here/'profile.json',expected)
 before={n:p.sha((here/n).read_bytes()) for n in ['cases.json','inventory.json','observations.json','reference.raw.json.gz']}
 logfile=Path('/tmp/tempo2-tim-central-review.log')
 with logfile.open('w') as log,contextlib.redirect_stdout(log),contextlib.redirect_stderr(log):
  cs=run.cases();gold=json.loads((here/'observations.json').read_text());run.compare(json.loads((here/'cases.json').read_text()),cs,'cases');run.compare(json.loads((here/'inventory.json').read_text()),run.inventory(),'inventory')
  relocations=[];rows=[]
  def relocate(x,path='$'):
   if isinstance(x,dict):
    out={k:relocate(v,path+'.'+k) for k,v in x.items()}
    if path.endswith('.p0.deleteFileName') or path.endswith('.p1.deleteFileName'):
     def one(v):
      if not isinstance(v,dict) or set(v)!=set(['size','nonzero']):return v
      out=copy.deepcopy(v)
      for runbytes in out['nonzero']:
       raw=bytes.fromhex(runbytes[1]);prefix=str(tmp).encode()
       if runbytes[0]==0 and raw.startswith(prefix):
        changed=str(root).encode()+raw[len(prefix):];assert len(changed)==len(raw)
        relocations.append(dict(case=c['id'],field=path,original=raw.decode(),projected=changed.decode()))
        runbytes[1]=changed.hex()
      return out
     if set(out)==set(['before','after']):out={k:one(v) for k,v in out.items()}
     else:out=one(out)
    return out
   if isinstance(x,list):return [relocate(v,path+f'[{i}]') for i,v in enumerate(x)]
   return x
  for i,(c,g) in enumerate(zip(cs,gold)):
   r=run.execute(c,exe,profile);run.validate(c,r);projected=relocate(r);run.compare(g,projected,c['id']);rows.append(projected)
   if i%20==0:print('PASS',i+1,len(cs),c['id'],flush=True)
  run.compare(gold,rows,'complete_ordered_results');run.execution_identity(exe,profile,True)
  replay=dict(cases=len(rows),events=sum(len(r['events']) for r in rows),complete_ordered_comparison=True,execution_identity=run.execution_identity(exe,profile,True),executed_profile=profile,caller_path_relocations=relocations,observations_sha256=p.sha((here/'observations.json').read_bytes()))
  # Run all normal identity/comparator/launch controls; retain compiled mutation evidence for separate identity review.
  verify.MUTATIONS=[];verify.main();controls=json.loads((here/'verification.json').read_text())
 for n,h in before.items():assert p.sha((here/n).read_bytes())==h,n
 run.execution_identity(exe,controls['profile'],True)
 cs=run.cases();assert replay['complete_ordered_comparison']
 result.update(status='PASS',cases=replay['cases'],events=replay['events'],actions=sum(len(c['actions']) for c in cs),classifications=dict(Counter(c['classification'] for c in cs)),replay=replay,identity_controls=controls['identity_controls'],comparison_controls=controls['comparison_controls'],final_current_execution=controls['execution_identity'],frozen_inputs=before,log_sha256=p.sha(logfile.read_bytes()),adaptations=['Same byte-length temporary source root preserves path-dependent truncation.','Absolute probe-header dependency key and raw macro-dump order digest are adapted only in disposable expected metadata after exact definition-multiset verification.','The initial temporary replay rejected literal caller-path bytes in deleteFileName. Final comparison relocates only the declared p0/p1.deleteFileName raw prefix, at byte offset zero with identical length; all such projections are recorded. Input, inventory and output goldens remain byte-identical. Normal verifier controls run with its compiled-mutation list empty; those mutations and gcov are reviewed from pinned evidence, not rerun.'])
Path('/tmp/tempo2-tim-central-review.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
print('PASS',result['cases'],'cases',result['events'],'events;',len(result['identity_controls'])+len(result['comparison_controls']),'controls')
