"""Central replay of T's corrected comparator using independently built, unchanged Rust executables."""
from pathlib import Path
import json,shutil,subprocess,sys,tempfile
sys.dont_write_bytecode=True
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust');HERE=ROOT/'validation/pulsar_lifecycle_port'
PRE=ROOT/'decisions/evidence/pulsar-lifecycle-port/build/pre0052'
OUT=Path(tempfile.mkdtemp(prefix='tempo2-t-post0052-'));print('OUT',OUT,flush=True)
sys.path.insert(0,str(HERE));import run
accepted=run.OUT;before=run.identity();n=run.protected()
old=json.loads((PRE/'central-review.json').read_text())
assert run.sha(PRE/'central-review.json')=='3c3d20129aee66cc803d5204dec5e974b440de0afc7ea0a1b0389c669975e2b8'
assert {p:h for p,h in before['sources'].items() if p.startswith('rust/')}=={p:h for p,h in old['source_identity']['sources'].items() if p.startswith('rust/')}
for k in ['compiler','compiler_sha256','cargo','cargo_sha256']:assert before[k]==old['source_identity'][k],k
changes={p for p in before['sources'] if before['sources'][p]!=old['source_identity']['sources'].get(p)}
assert changes=={'validation/pulsar_lifecycle_port/'+p for p in ['run.py','controls.py','verify.py','PROJECTION.md']},changes
shutil.copytree(accepted/'freeze',OUT/'freeze');run.OUT=OUT
import controls
cases=json.loads((OUT/'freeze/membership.json').read_text())['S'];assert len(cases)==30
profiles=[]
for profile in ['debug','release']:
 exe=PRE/'executables'/profile
 prior=next(p for p in old['profiles'] if p['profile']==profile)
 assert run.sha(exe)==prior['identity']['executable_sha256']
 ident=run.identity(exe);rows=[]
 for case in cases:
  rows.append(run.execute(case,exe,ident,OUT/profile/case['id']))
  print('PASS',profile,case['id'],flush=True)
 control=controls.run(profile,exe,ident)
 assert len(control)==35 and sum(x['status']=='accepted' for x in control)==2
 assert sum(x['events'] for x in rows)==133 and sum(x['records'] for x in rows)==5329542
 data=dict(profile=profile,identity=ident,cases=rows,controls=control)
 run.save(OUT/profile/'central-profile.json',data);profiles.append(data)
p=subprocess.run([sys.executable,ROOT/'decisions/evidence/pulsar-lifecycle-port/type-equality-reproducer.py'],capture_output=True,text=True,check=True)
witness=json.loads(p.stdout);assert len(witness['witnesses'])==3 and all(x['outcome']=='REJECTED' for x in witness['witnesses'])
assert witness['case_sha256']=='461e77b0d9032cf999641abca44c5903fd31346dff97c2c7bc8fa4b5b0b9256b'
run.save(OUT/'type-witnesses.json',witness)
assert run.identity()==before and run.protected()==n==8141
result=dict(status='PASS',scope='Fresh challenged debug/release S replay and 35 controls with corrected comparator; original independent Rust build bytes reused after source/compiler identity verification; no new native-helper or wider regression run in this script',output=str(OUT),prior_central_receipt_sha256=run.sha(PRE/'central-review.json'),source_identity=before,changed_harness_files=sorted(changes),profiles=profiles,protected_files=n,type_witnesses=witness)
run.save(OUT/'central-review.json',result)
print('PASS corrected T comparator: 30 cases / 133 events / 5329542 records and 35 controls per profile; 3 independent type mutations rejected',flush=True)
print('RECEIPT',OUT/'central-review.json',run.sha(OUT/'central-review.json'),flush=True)
