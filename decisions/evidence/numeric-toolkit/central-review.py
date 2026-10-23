import concurrent.futures,copy,hashlib,importlib,json,subprocess,sys,tempfile
from collections import Counter
from pathlib import Path
sys.dont_write_bytecode=True
root=Path(__file__).resolve().parents[3];revision='a01d4281fe5452111b5067d8885c2df793ac44f4'
result={'reviewed_commit':subprocess.check_output(['git','rev-parse',revision],cwd=root,text=True).strip()}
with tempfile.TemporaryDirectory(prefix='m-',dir='/tmp') as td:
 base=Path(td).resolve();padding=len(str(root))-len(str(base))-1;assert padding>0
 tmp=base/('x'*padding);tmp.mkdir();assert len(str(tmp))==len(str(root))
 here=tmp/'validation/numeric_toolkit';here.mkdir(parents=True)
 for name in subprocess.check_output(['git','ls-tree','-r','--name-only',revision,'--','validation/numeric_toolkit'],cwd=root,text=True).splitlines():
  f=tmp/name;f.parent.mkdir(parents=True,exist_ok=True);f.write_bytes(subprocess.check_output(['git','show',revision+':'+name],cwd=root))
 (tmp/'tempo2').symlink_to(root/'tempo2',target_is_directory=True)
 (tmp/'validation/reference').symlink_to(root/'validation/reference',target_is_directory=True)
 sys.path.insert(0,str(here));p=importlib.import_module('profile');run=importlib.import_module('run');verify=importlib.import_module('verify')
 print('Building corrected final M source in temporary storage',flush=True)
 exe,profile=p.compile_probe();expected=json.loads((here/'profile.json').read_text())
 old=expected['stable']['header_dependencies'];expected['stable']['header_dependencies']={k.replace(str(root),str(tmp)):v for k,v in old.items()}
 original_macros=(root/'validation/numeric_toolkit/build/probe-macros.txt').read_bytes();fresh_macros=(p.BUILD/'probe-macros.txt').read_bytes()
 assert p.sha(original_macros)==expected['stable']['macros_sha256']
 assert sorted(original_macros.splitlines())==sorted(fresh_macros.splitlines()),'macro definitions changed'
 result['macro_definition_equivalence']=dict(definitions=len(original_macros.splitlines()),original_sha256=p.sha(original_macros),temporary_sha256=p.sha(fresh_macros),sorted_definitions_sha256=p.sha(b'\n'.join(sorted(fresh_macros.splitlines()))),exact_multiset=True)
 expected['stable']['macros_sha256']=profile['stable']['macros_sha256']
 identity=run.identity_gate(expected,profile,exe)
 cases=json.loads((here/'cases.json').read_text());gold=json.loads((here/'observations.json').read_text());run.compare(cases,run.cases(),'cases')
 def one(pair):
  c,g=pair;r=run.execute(c,exe,profile);run.validate(c,r);run.compare(g,r,c['id']);return r
 with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:rows=list(pool.map(one,zip(cases,gold)))
 run.compare(gold,rows,'complete_ordered_results')
 print('PASS',len(rows),'cases and',sum(len(r['events']) for r in rows),'events exactly',flush=True)
 controls=verify.controls(cases,rows,exe,profile)
 print('PASS',len(controls),'normal identity/comparator/freshness controls',flush=True)
 run.identity_gate(expected,profile,exe)
 result.update(status='PASS',cases=len(rows),events=sum(len(r['events']) for r in rows),actions=sum(len(c['actions']) for c in cases),classifications=dict(Counter(c['classification'] for c in cases)),controls=controls,profile=profile,execution=identity,reference_sha256=p.sha((here/'observations.json').read_bytes()),adaptations=['Temporary source root has exactly the original byte length to retain raw stdout positions/truncation.','Only the absolute probe-header dependency key is relocated in the in-memory expected build identity. Scientific/action/output goldens unchanged.','Raw -dM definition order differs after relocation; all 3731 complete definitions, including multiplicity, must match exactly before adapting only that raw-byte digest in memory. The frozen macro file must match its frozen digest.','Fresh executable bytes are recorded and checked, not required to equal unrelated build paths.'],scope='Fresh independent complete same-profile replay and normal gate controls; no source mutations or gcov build rerun by central.')
Path('/tmp/tempo2-numeric-central-review.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
