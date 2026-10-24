"""Central W corrected-contract audit and fresh debug/release execution."""
from pathlib import Path
import gzip, hashlib, json, shutil, subprocess, sys, tempfile
sys.dont_write_bytecode=True
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
HERE=ROOT/'validation/terrestrial_resources_port'
ACCEPTED=ROOT/'rust/verification/terrestrial_resources'
U=ROOT/'validation/terrestrial_resources'
FULL=ACCEPTED/'before_guard_review'
sys.path.insert(0,str(HERE))
import freeze, run, controls
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
load=lambda p:json.loads(p.read_text())
readgz=lambda p:json.loads(gzip.decompress(p.read_bytes()))
assert subprocess.check_output(['git','diff','HEAD','--','rust/src','rust/native','rust/examples','rust/tests','rust/Cargo.toml','rust/Cargo.lock','rust/build.rs','validation/terrestrial_resources_port'],cwd=ROOT)==b'', 'Wait for clean committed W code and harness'
OUT=Path(tempfile.mkdtemp(prefix='w-central-',dir='/private/tmp'))
print('OUT',OUT,flush=True)
original_files=[ACCEPTED/n for n in ['corrected_native/contract.json','corrected_native/reference.json.gz','corrected_native/changes.json','wet_original_mutation/receipt.json','routing/staging.json','routing/unsafe_audit.json']]
original_files += [p for p in FULL.rglob('*') if p.is_file()]
original_files += [ACCEPTED/n for n in ['final_scope.json','final_commands.json','debug/direct.json','release/direct.json','debug/controls.json','release/controls.json','fixture_repro/receipt.json']]
original_files += [p for p in HERE.iterdir() if p.is_file()]
before={str(p):sha(p) for p in original_files}
protected=freeze.check();frozen=load(HERE/'build/freeze.json');catalog=frozen['cases']
delivery=load(FULL/'delivery.json');current_identity=run.identity()
final=load(ACCEPTED/'final_scope.json');run.compare(final['identity'],current_identity,'final scoped identity')
assert sha(ACCEPTED/'final_commands.json')==final['commands_sha256']
for c in load(ACCEPTED/'final_commands.json'):
 assert c['returncode']==0 and sha(ACCEPTED/c['log'])==c['sha256'],c['name']
assert sha(FULL/'delivery.json')==final['prior_full_receipt_sha256']
old_identity=delivery['identity']
run.compare(list(old_identity['sources']),[n for n in current_identity['sources'] if n in old_identity['sources']],'retained source membership')
source_additions=[n for n in current_identity['sources'] if n not in old_identity['sources']]
assert source_additions==['validation/terrestrial_resources_port/final_review.py']
source_changes=[n for n,h in old_identity['sources'].items() if current_identity['sources'][n]!=h]
assert source_changes==['rust/src/tropo.rs','validation/terrestrial_resources_port/CONTRACT.md'],source_changes
# The added final-review runner and appended contract were inspected separately;
# neither changes a production calculation. Bind them to their actual commit.
for name in ['validation/terrestrial_resources_port/final_review.py','validation/terrestrial_resources_port/CONTRACT.md']:
 assert (ROOT/name).read_bytes()==subprocess.check_output(['git','show','eb2a589:'+name],cwd=ROOT)
old_contract=subprocess.check_output(['git','show','7946aee:validation/terrestrial_resources_port/CONTRACT.md'],cwd=ROOT)
assert hashlib.sha256(old_contract).hexdigest()==old_identity['sources']['validation/terrestrial_resources_port/CONTRACT.md']
assert (HERE/'CONTRACT.md').read_bytes().startswith(old_contract)
for name,h in load(ACCEPTED/'R/O/regressions/source_hashes.json').items():
 if name!='rust/src/tropo.rs':assert sha(ROOT/name)==h,name
for key in old_identity:
 if key!='sources':run.compare(old_identity[key],current_identity[key],key)
old_tropo=(FULL/'tropo.rs').read_bytes()
assert hashlib.sha256(old_tropo).hexdigest()==old_identity['sources']['rust/src/tropo.rs']
assert old_tropo==subprocess.check_output(['git','show','7df6234:rust/src/tropo.rs'],cwd=ROOT)
old_guard=b'        if site.len() >= 256 {\n            return Err(boundary("meteorology_site_storage"));\n        }\n'
new_guard=b"        // load() guarantees a '#' header of at most 255 bytes; this token\n        // therefore fits siteName[256], including its NUL terminator.\n"
assert old_tropo.count(old_guard)==1
assert (ROOT/'rust/src/tropo.rs').read_bytes()==old_tropo.replace(old_guard,new_guard)
# read_bytes validates <=255 header bytes and a leading # before load succeeds;
# header[1..]'s site token is therefore <=254 bytes. No fabricated invalid
# internal state is used to justify a second production check.
loader=(ROOT/'rust/src/tabulated_function.rs').read_text()
assert 'if header.len() > 255 {' in loader and "if self.header.first() != Some(&b'#')" in loader
assert sha(FULL/'artifacts.json')==delivery['artifacts_sha256'] and sha(FULL/'modules.json')==delivery['modules_sha256']
artifacts=load(FULL/'artifacts.json')
for name,h in artifacts.items():
 path=FULL/name if (FULL/name).is_file() else ACCEPTED/name
 assert sha(path)==h,name
for c in load(FULL/'commands.json'):
 path=FULL/c['log'] if (FULL/c['log']).is_file() else ACCEPTED/c['log']
 assert c['returncode']==0 and sha(path)==c['sha256'],c['name']
assert len(catalog)==delivery['cases_per_profile']==130
assert sum(len(c['actions']) for c in catalog)==delivery['actions_per_profile']==1128
original=readgz(U/'outputs/reference.json.gz')
corrected=readgz(ACCEPTED/'corrected_native/reference.json.gz')
contract=load(ACCEPTED/'corrected_native/contract.json')
assert sha(U/'outputs/reference.json.gz')==contract['original_sha256']
assert sha(ACCEPTED/'corrected_native/reference.json.gz')==contract['corrected_sha256']
assert contract['patch']==['tropo.C','c = frac*bs[ilat2] + (1.0-frac)*cs[ilat1];','c = frac*cs[ilat2] + (1.0-frac)*cs[ilat1];']
assert sha(ACCEPTED/'fixed00/probe')==contract['profile']['executable_sha256']
for p,h in contract['profile']['dependencies'].items():assert sha(Path(p))==h,p
for p in (ACCEPTED/'fixed00/source').rglob('*'):
 if not p.is_file():continue
 q=U/'build/ordinary/source'/p.relative_to(ACCEPTED/'fixed00/source')
 expected=q.read_bytes()
 if p.name=='tropo.C':
  old,new=[s.encode() for s in contract['patch'][1:]];assert expected.count(old)==1;expected=expected.replace(old,new)
 assert p.read_bytes()==expected,p
# Recompute every changed leaf from the two separately retained native oracles.
def differences(a,b,path):
 assert type(a) is type(b)
 if isinstance(a,dict):
  assert list(a)==list(b)
  return [d for k in a for d in differences(a[k],b[k],path+'.'+k)]
 if isinstance(a,list):
  assert len(a)==len(b)
  return [d for i,(x,y) in enumerate(zip(a,b)) for d in differences(x,y,path+f'[{i}]')]
 return [] if a==b else [dict(path=path,original=a,corrected=b)]
run.compare([c['id'] for c in catalog],[r['id'] for r in original],'original membership')
run.compare([c['id'] for c in catalog],[r['id'] for r in corrected],'corrected membership')
changes=[]
for a,b in zip(original,corrected):
 ds=differences(a,b,a['id'])
 if ds:changes.append(dict(id=a['id'],fields=ds))
run.compare(contract['changes'],changes,'corrected native contract')
run.compare(load(ACCEPTED/'corrected_native/changes.json'),changes,'changed-field receipt')
assert len(changes)==10 and sum(len(c['fields']) for c in changes)==31
assert all(d['path'].endswith('.result.wet.bits') for c in changes for d in c['fields'])
mutation=load(ACCEPTED/'wet_original_mutation/receipt.json')
assert mutation['status']=='PASS' and mutation['original_expression_exit']==101 and mutation['corrected_exit']==0
assert hashlib.sha256(old_tropo).hexdigest()==mutation['source_sha256']
# The historical mutation ran this exact old source; the sole later guard delta
# above is outside nmf_wet and changes no calculation. Do not relabel its run.
for n,h in mutation['logs'].items():assert sha(ACCEPTED/'wet_original_mutation'/n)==h
assert 'test result: FAILED' in (ACCEPTED/'wet_original_mutation/original-expression.log').read_text()
assert 'test result: ok' in (ACCEPTED/'wet_original_mutation/corrected-expression.log').read_text()
# The original preservation runner and source-unsafe bodies must stay untouched.
assert sha(ROOT/'validation/observation_storage_port/run.py')=='7d6424654a666d3852255bd9264ba0fbb88ba3c6e57f7bf717da4087a6b10529'
import stage_regressions
for name,audit in load(ACCEPTED/'routing/unsafe_audit.json').items():
 current=ROOT/'rust/src'/name
 raw=subprocess.check_output(['git','show','a8ef85c:rust/src/'+name],cwd=ROOT,text=True)
 assert stage_regressions.unsafe_blocks(raw)==stage_regressions.unsafe_blocks(current.read_text())
 assert sha(current)==audit['current_sha256']
for stage,details in load(ACCEPTED/'routing/staging.json').items():
 folder={'T':'pulsar_lifecycle_port','R':'observation_storage_port'}[stage]
 for n,h in details['files'].items():
  assert sha(ROOT/'validation'/folder/n)==h['original']
  assert sha(ACCEPTED/'routing'/folder/n)==h['staged']
 assert sha(ACCEPTED/'routing'/(stage+'.diff'))==details['diff_sha256']
regressions=load(ACCEPTED/'R/O/regressions/delivery_checks.json')
assert regressions['m1a']=='PASS' and regressions['m1b']=='PASS (via M2)' and regressions['jpl']=='PASS: unchanged I gates'
assert regressions['m2_exit']==1
run.compare(regressions['m2_failures'],delivery['M2_failures'],'M2 delivery')
run.compare(delivery['M2_failures'],load(ROOT/'rust/verification/m2/open_gate_failures.json'),'historic M2 failures')
assert len(delivery['M2_failures'])==4
# Resolve/build final test binaries with Cargo, then run the entire corpus.
# Broader regression receipts retain their earlier, exactly proved source identity.
run.OUT=OUT
shutil.copytree(ACCEPTED/'corrected_native',OUT/'corrected_native')
profiles={}
for kind in ('debug','release'):
 delivered=load(FULL/kind/'direct.json')
 assert delivered['status']=='PASS' and len(delivered['cases'])==130 and not delivered['failures']
 assert sha(FULL/'executables'/kind)==delivered['identity']['executable_sha256']
 scoped=load(ACCEPTED/kind/'direct.json')
 assert sha(ACCEPTED/kind/'direct.json')==final['profiles'][kind]['receipt_sha256']
 assert sha(ACCEPTED/kind/'controls.json')==final['profiles'][kind]['controls_sha256']
 assert scoped['status']=='PASS' and scoped['failures']==[]
 run.compare(delivered['cases'],scoped['cases'],kind+'.historical versus final membership')
 exe=run.build(kind);ident=run.identity(exe)
 run.compare(scoped['identity'],ident,kind+'.final delivered binary')
 assert ident['executable_sha256']==final['profiles'][kind]['executable_sha256']
 run.compare({k:v for k,v in ident.items() if k not in ('executable','executable_sha256')},current_identity,kind+'.current_identity')
 results=[]
 for case,expected in zip(catalog,corrected):
  actual=run.execute(case,exe,ident,OUT/kind/case['id'])
  run.compare(run.project(expected),run.project(actual),case['id'])
  results.append(dict(id=case['id'],events=len(actual['events']),status=actual['status']))
  if len(results)%25==0:print('PASS',kind,len(results),'cases',flush=True)
 run.compare(delivered['cases'],results,kind+'.delivered membership')
 controls.checks(kind,exe,ident,catalog)
 cs=load(OUT/kind/'controls.json');assert len(cs['controls'])==28 and cs['status']=='PASS'
 profiles[kind]=dict(identity=ident,cases=results,controls_sha256=sha(OUT/kind/'controls.json'))
 assert run.identity(exe)==ident
assert freeze.check()==protected
assert {str(p):sha(p) for p in original_files}==before
result=dict(status='PASS',scope='Corrected native contract audit, historical full-regression identities and sole validated-guard delta proved; Cargo builds and fresh challenged replay of all final cases; broader regressions not rerun',output=str(OUT),profiles=profiles,changed_native_cases=10,changed_native_fields=31,preserved_files=before,protected_frozen_entries=protected,M2_failures=delivery['M2_failures'],final_scope_sha256=sha(ACCEPTED/'final_scope.json'),historical_full_delivery_sha256=sha(FULL/'delivery.json'),historical_tested_revision=delivery['executed_head'],source_changes=source_changes,source_additions=source_additions,script_sha256=sha(Path(__file__)),source_revision=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip())
run.save(OUT/'central-review.json',result)
print('PASS central W: 130 cases and 28 controls per profile; selected bug correction audited',flush=True)
print('RECEIPT',OUT/'central-review.json',sha(OUT/'central-review.json'),flush=True)
