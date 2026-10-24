"""Inspect T's delivered identities, full regression chain and preserved prior run."""
from pathlib import Path
import hashlib,json,re,subprocess,sys
sys.dont_write_bytecode=True
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust');HERE=ROOT/'validation/pulsar_lifecycle_port';O=ROOT/'rust/verification/pulsar_lifecycle'
OUT=Path(Path('/tmp/tempo2-lifecycle-central-review.log').read_text().splitlines()[0][4:])
sys.path.insert(0,str(HERE));import run,supplement
sha=run.sha;load=lambda p:json.loads(p.read_text())
d=load(O/'delivery.json');identity=run.identity();assert d['identity']==identity
assert run.protected()==d['protected_files']==8141
artifacts=load(O/'artifacts.json');assert sha(O/'artifacts.json')==d['artifacts_sha256']
actual={str(p.relative_to(O)):sha(p) for p in sorted(O.rglob('*')) if p.is_file() and not any(k in p.relative_to(O).parts for k in ['build','native','prior_0048']) and p.name not in ['delivery.json','artifacts.json']}
assert actual==artifacts
assert len(artifacts)==531
for name in ['commands','modules']:assert sha(O/(name+'.json'))==d[name+'_sha256']
# Independent static field membership comparison against the source-checked map.
inv=load(O/'freeze/field-and-operation-map.json');text=(ROOT/'rust/src/pulsar.rs').read_text()
for native,rust in [('pulsar','SourcePulsar'),('FitInfo','FitInfo'),('FitOutput','FitOutput'),('storePrecision','StorePrecision')]:
 body=text.split('pub struct '+rust+' {\n',1)[1].split('\n}',1)[0]
 fields=re.findall(r'^    pub (\w+):',body,re.M);assert fields==[x['name'] for x in inv['types'][native]],native
assert len(inv['types']['pulsar'])==349
baseline=load(O/'freeze/baseline.json')
for name,expected in baseline['nested'].items():
 assert subprocess.check_output(['git','-C',str(ROOT/name),'rev-parse','HEAD'],text=True).strip()==expected['head']
 assert subprocess.check_output(['git','-C',str(ROOT/name),'status','--porcelain'],text=True)==expected['status']==''
# No native build input/dependency or unsafe-containing Rust file changed in T.
assert not subprocess.check_output(['git','diff','d350727','4225f48','--','rust/native','rust/Cargo.toml','rust/Cargo.lock','rust/build.rs']).strip()
modules=load(O/'modules.json')
for p,v in modules.items():assert sha(ROOT/p)==v['sha256']
for p,same in d['unchanged_unsafe_files'].items():assert same and sha(ROOT/p)==baseline['rust'][p]
for p in (ROOT/'rust/src').glob('*.rs'):
 if sha(p)!=baseline['rust'].get(str(p.relative_to(ROOT))):
  assert not re.search(r'\bunsafe\s*(?:\{|extern\b|fn\b|impl\b|trait\b)',p.read_text()),p
membership=load(O/'freeze/membership.json')['S'];keys=[c['id'] for c in membership]
for mode in ['debug','release']:
 direct=load(O/mode/'direct.json');assert direct['status']=='PASS' and direct['protected']==8141
 assert [c['id'] for c in direct['cases']]==keys and sum(c['events'] for c in direct['cases'])==133
 assert sum(c['records'] for c in direct['cases'])==5329542 and len(direct['controls'])==31
 assert direct['identity']==run.identity(Path(direct['identity']['executable']))
 for c in direct['cases']:
  folder=O/mode/c['id'];assert sha(folder/'events.jsonl.gz')==c['output_sha256']
  assert sha(ROOT/'validation/pulsar_lifecycle/expected'/(c['id']+'.json.gz'))==c['expected_sha256']
  launch=load(folder/'launch.json');assert launch['identity']==direct['identity'] and launch['returncode']==0
  assert sha(folder/'input.json')==launch['input_sha256']
 helper=load(O/mode/'helpers/receipt.json');assert helper['status']=='PASS';supplement.validate(helper['native'])
 assert helper['identity']==direct['identity']
 assert (helper['Q_helpers'],helper['helpers'],helper['shifts'],helper['clamps'])==(9,18,320,8)
 for m in helper['mutations']:
  supplement.validate(m['identity']);assert sha(ROOT/'validation/pulsar_lifecycle/profile.py')==m['identity']['source_helpers']
# Verify the original/staged R membership and the only two path substitutions.
routing=load(O/'routing/identity.json');rr=load(O/'routing/receipt.json')
assert rr['status']=='PASS' and rr['decision']==routing['decision']=='0050' and rr['returncode']==0
assert sha(O/'routing/identity.json')==rr['identity_sha256'] and sha(O/'routing/run.diff')==rr['diff_sha256']==routing['diff_sha256']
for group in ['original','staged']:
 for p,h in routing[group].items():assert sha(ROOT/p)==h
original=ROOT/'validation/observation_storage_port';stage=O/'routing/observation_storage_port'
assert {p.name for p in original.iterdir() if p.is_file()}=={p.name for p in stage.iterdir() if p.is_file()}
assert len(routing['original'])==len(routing['staged'])==13
raw=(original/'run.py').read_text();routed=(stage/'run.py').read_text()
expected=raw.replace('ROOT=HERE.parents[1]',f'ROOT=Path({str(ROOT)!r})').replace("OUT=RUST/'verification/observation_storage'",f"OUT=Path({str(O/'R')!r})")
assert expected==routed
for p in original.iterdir():
 if p.is_file() and p.name!='run.py':assert p.read_bytes()==(stage/p.name).read_bytes()
assert sha(original/'run.py')==rr['original_runner_sha256']=='7d6424654a666d3852255bd9264ba0fbb88ba3c6e57f7bf717da4087a6b10529'
# Check every command exit/log through the R/O/clock/JPL/M1/M2 chain.
command_count=0
for rel in ['', 'R','R/O','R/O/regressions']:
 folder=O/rel
 for c in load(folder/'commands.json'):
  expected=1 if rel=='R/O/regressions' and c['name']=='m2' else 0
  assert c['returncode']==expected,(rel,c['name'])
  h=c.get('sha256',c.get('log_sha256'))
  if h is not None:assert sha(folder/c['log'])==h
  command_count+=1
r=load(O/'R/delivery_checks.json');o=load(O/'R/O/delivery_checks.json');reg=load(O/'R/O/regressions/delivery_checks.json')
assert sha(O/'R/delivery_checks.json')==d['regression_receipt_sha256']==rr['regression_receipt_sha256']
assert sha(O/'R/O/delivery_checks.json')==r['O_sha256']
assert sha(O/'R/O/regressions/delivery_checks.json')==r['regressions_sha256']==o['regressions_sha256']
failures=load(ROOT/'rust/verification/m2/open_gate_failures.json')
assert len(failures)==4 and d['M2_failures']==r['M2_failures']==o['M2_failures']==reg['m2_failures']==failures
assert load(O/'R/O/regressions/m2/open_gate_failures.json')==failures
assert reg['m2_exit']==1 and reg['m1a']=='PASS' and reg['m1b']=='PASS (via M2)' and reg['jpl']=='PASS: unchanged I gates'
assert reg['profiles_identical'] and reg['max_finite_difference']==0
assert r['fields']==95 and r['controls_per_profile']==39 and o['controls_per_profile']==30
for mode in ['debug','release']:
 for v in [r['executables'][mode],o['executables'][mode]]:assert sha(Path(v['path']))==v['sha256']
# The superseded complete run is historical, with all indexed files preserved.
prior=O/'prior_0048';old=load(prior/'artifacts.json');assert len(old)==518
assert all(sha(prior/p)==h for p,h in old.items())
assert sha(prior/'delivery.json')=='a08aecb21ff6470d2523fddd33bbd2c1e62c02fbf7df017c75d238e9e5d85c98'
assert identity==run.identity() and run.protected()==8141
result=dict(status='PASS',scope='Delivered T identities, declared fields, two-path R staging, required regression receipts and preserved prior artifacts; full wider regression chain not rerun centrally',delivered_revision='4225f48',implementation_revision='8daa4aa',source_identity=identity,protected_files=8141,current_indexed_artifacts=len(artifacts),prior_indexed_artifacts=len(old),commands_checked=command_count,original_R_sha256=sha(original/'run.py'),delivery_sha256=sha(O/'delivery.json'),report_sha256=sha(ROOT/'rust/PULSAR_LIFECYCLE_REPORT.md'),M2_failures=failures)
run.save(OUT/'central-delivery-audit.json',result)
print('PASS T delivered identity/state/routing/regression/history audit; commands',command_count)
print('RECEIPT',OUT/'central-delivery-audit.json',sha(OUT/'central-delivery-audit.json'))
