import contextlib,hashlib,importlib,io,json,subprocess,sys,tempfile,unittest
from pathlib import Path
sys.dont_write_bytecode=True
root=Path('/Users/Eric.Dong/Downloads/tempo2rust');here=root/'validation/clocks_port';revision='84b6b86'
sys.path.insert(0,str(here));port=importlib.import_module('run');supp=importlib.import_module('supplemental')
record={'reviewed_commit':subprocess.check_output(['git','rev-parse',revision],cwd=root,text=True).strip(),'profiles':{},'scope':'Fresh Rust direct/supplemental rebuilds and central execution; no M1/M2/JPL regression suite rerun by central.'}
expected=port.load(root/'rust/verification/clocks/glob_correction/source_hashes.json');assert port.executed_sources()==expected
with tempfile.TemporaryDirectory(prefix='L-review-',dir='/tmp') as td:
 port.OUT=Path(td);log=Path('/tmp/tempo2-clock-central-review.log')
 with log.open('w') as f,contextlib.redirect_stdout(f),contextlib.redirect_stderr(f):
  for profile in ['debug','release']:
   sys.argv=['run.py','--profile',profile];port.main()
   direct=port.load(port.OUT/(profile+'_direct.json'));port.validate_delivery(direct)
   sys.argv=['supplemental.py','--profile',profile];supp.main()
   additional=port.load(port.OUT/(profile+'_glob_supplemental.json'))
   record['profiles'][profile]={'direct':{k:direct[k] for k in ['source_hashes','frozen_hashes','counts','failures','identities','executable_sha256','runner_sha256','host']},'supplemental':additional,'direct_cases':len(direct['results']),'direct_results_sha256':hashlib.sha256(json.dumps(direct['results'],sort_keys=True).encode()).hexdigest()}
  debug=port.load(port.OUT/'debug_direct.json');release=port.load(port.OUT/'release_direct.json')
  for a,b in zip(debug['results'],release['results']):
   assert a.keys()==b.keys() and a['id']==b['id'] and a['status']==b['status']
   if 'raw' in a:
    clean=lambda r:{k:v for k,v in r.items() if k not in ['run_token','input_sha256']}
    assert clean(a['raw'])==clean(b['raw']),a['id']
  controls=unittest.defaultTestLoader.discover(str(here),pattern='test_comparator.py')
  result=unittest.TextTestRunner(stream=f,verbosity=2).run(controls);assert result.wasSuccessful()
  record['comparator_tests']=result.testsRun
 assert port.executed_sources()==expected
 record.update(status='PASS',profiles_identical=True,log_sha256=hashlib.sha256(log.read_bytes()).hexdigest())
Path('/tmp/tempo2-clock-central-review.json').write_text(json.dumps(record,indent=2,sort_keys=True)+'\n')
print('PASS: 142 exact J cases and 8 safe Rust boundaries per profile, supplemental comparisons, profile equality and',record['comparator_tests'],'comparator tests')
