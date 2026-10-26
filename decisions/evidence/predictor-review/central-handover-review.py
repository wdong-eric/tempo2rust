"""Verify AF delivery identities and summarize bounded central committed-source checks."""
from pathlib import Path
from decimal import Decimal as D
import hashlib,json,subprocess
root=Path('/Users/Eric.Dong/Downloads/tempo2rust');build=root/'validation/predictors/build'
out=Path(Path('/tmp/tempo2-af-handover-current').read_text().strip())
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
index=build/'delivery-index.json';assert sha(index)=='324e4ab1b8874a8098bd5a3be36b2a640ae9820783dfb52c37bc92da6571efb4'
entries=json.loads(index.read_text())
for name,h in entries.items():assert sha(root/name)==h,name
public=[];source_differences={};products=0
for profile in ['debug','release']:
 folder=build/('public-'+profile+'-corrected');d=json.loads((folder/'receipt.json').read_text())
 assert len(d['cases'])==14 and sum(c['checks'] for c in d['cases'])==37296
 assert sha(folder/'native.snapshot')==d['native'] and sha(folder/'rust.snapshot')==d['rust']
 failures=[]
 for c in d['cases']:
  assert all(x['returncode']==0 for x in c['commands']),c['name']
  for name,h in c['files'].items():assert sha(folder/(c['name']+'-'+c['kind'])/name)==h,(c['name'],name);products+=1
  failures += [dict(f,case=c['name'],kind=c['kind']) for f in c['failures']]
 assert len(failures)==7 and all(x['field'].endswith('.log10rms') for x in failures)
 for name,h in d['sources'].items():
  if sha(root/name)!=h:source_differences[name]={'recorded':h,'current':sha(root/name)}
 public.append({'profile':profile,'checks':sum(c['checks'] for c in d['cases']),'failures':failures})
 audits=json.loads((build/('audit-'+profile+'-final')/'receipt.json').read_text());assert not any(r['failures'] for r in audits)
 assert sum(r['plan_checks']+r.get('serialization_checks',0) for r in audits)==3906
 q=json.loads((build/('quality-'+profile+'-corrected')/'receipt.json').read_text());assert len(q['cases'])==24
 assert sum(c['fidelity']['checks'] for c in q['cases'])==4224
 assert not any(c['fidelity']['failures'] for c in q['cases'])
 assert all(all(x['returncode']==0 for x in c['commands']) for c in q['cases'])
 assert all(c['approximation'][owner]['passes'] for c in q['cases'] for owner in ['native','rust'])
 public[-1]['quality_max_seconds']=str(max(D(c['approximation'][owner]['max_time_seconds']) for c in q['cases'] for owner in ['native','rust']))
 public[-1]['quality_max_frequency_hz']=str(max(D(c['approximation'][owner]['max_centered_frequency_error_hz']) for c in q['cases'] for owner in ['native','rust']))
# Reconcile every source-hash difference against the recorded public runner.
expected_differences={'validation/predictors/run_quality.py','validation/predictors/audit_records.py','validation/predictors/run_public.py','rust/src/t1polyco.rs'}
assert set(source_differences)==expected_differences
hash_text=lambda v:hashlib.sha256(v.encode()).hexdigest()
t=(root/'rust/src/t1polyco.rs').read_text().replace('polyco.C:419','polyco.C:405')
assert hash_text(t)==source_differences['rust/src/t1polyco.rs']['recorded']
t=(root/'validation/predictors/run_quality.py').read_text();t=t[:t.index('raise SystemExit(')].rstrip()+'\n'
assert hash_text(t)==source_differences['validation/predictors/run_quality.py']['recorded']
t=(root/'validation/predictors/audit_records.py').read_text().replace('from compare import plan,native_models,read_tsv,serialization,Checks,phase_bound,cheby_scale','from compare import plan,native_models,read_tsv,serialization')
a=t.index(" if kind=='cheby':\n");b=t.index(' for file in',a);t=t[:a]+t[b:]
assert hash_text(t)==source_differences['validation/predictors/audit_records.py']['recorded']
old=(root/'decisions/evidence/predictor-review/build/plan-corrections-20261026/run_public.py').read_text()
assert hash_text(old)==source_differences['validation/predictors/run_public.py']['recorded']
current=(root/'validation/predictors/run_public.py').read_text().splitlines(True)
reconstructed=[];i=0
while i<len(current):
 line=current[i]
 if line.startswith('    for owner,mav '):
  reconstructed.append(line.rstrip() + current[i+1].lstrip());i+=3
 elif line.startswith("    for key,field in [('rms','rms_turn')"):i+=1
 else:reconstructed.append(line);i+=1
assert ''.join(reconstructed)==old
for profile in ['debug','release']:
 d=json.loads((build/('public-'+profile+'-corrected')/'receipt.json').read_text())
 assert all(c['approximation']['native_passes'] and c['approximation']['rust_passes'] for c in d['cases'] if c['kind']=='cheby')
 assert sha(out/('audit-'+profile)/'receipt.json')=='bcea259d7c3791794f85e291d049e0700b446e3a0a9d950170c46b4a178919c3'
source_reconciliation={'t1polyco':'Only adjacent source line reference 405 -> 419.', 'run_public':'Adds failure propagation for already-recorded approximation flags and two source RMS/MAV checks per Cheby case; every recorded flag passes, and the additional checks were independently replayed.', 'audit_records':'Adds the same two source RMS/MAV checks; both current-script replay receipts exactly reproduce the final AF audit.', 'run_quality':'Only adds failure exit-status propagation; all recorded fidelity and approximation outcomes pass.'}
projection=Path(Path('/tmp/tempo2-af-projection-review-current').read_text().strip())
projection_receipt=json.loads((projection/'receipt.json').read_text())
assert [r['runner_failed'] for r in projection_receipt['records']]==[False,True,True]
assert [r['case_failures'] for r in projection_receipt['records']]==[[['cheby',0],['polyco',0]],[['cheby',1],['polyco',0]],[['cheby',1],['polyco',0]]]
ni=json.loads((build/'native-corrected/identity.json').read_text());objects=0
for name,h in ni['objects'].items():assert sha(Path(name))==h,name;objects+=1
for name,h in ni['sources'].items():assert sha(build/'native-corrected/source'/name)==h['corrected'],name
assert sha(root/'validation/reference/build/libtempo2-reference.a')==ni['archive']
assert sha(build/'native-corrected/predictor')==ni['executable']
assert sha(build/'native-corrected/corrections.patch')==ni['patch']
assert sha(root/'validation/predictors/native.cc')==ni['harness']
assert sha(root/'validation/predictors/native.py')==ni['builder']
local=json.loads((build/'rms-localization-delivery/receipt.json').read_text());assert len(local['records'])==5
assert all(r['shared_native_rounding_cells_match'] for r in local['records'])
assert sum(len(r['shared_phase_log10rms']) for r in local['records'])==10
for r in local['records']:
 assert all(float(x)==float(y) for x,y in zip(r['shared_phase_log10rms'],r['native_log10rms']))
fresh=[]
for name in ['isolated-bary','binary-pks']:
 p=out/('public-'+name)/'receipt.json';d=json.loads(p.read_text())
 assert sum(c['checks'] for c in d['cases'])==5330
 failures=[dict(f,kind=c['kind']) for c in d['cases'] for f in c['failures']]
 assert len(failures)==(0 if name=='isolated-bary' else 2)
 assert all(f['kind']=='polyco' and f['field'].endswith('.log10rms') for f in failures)
 fresh.append({'case':name,'checks':5330,'failures':failures,'receipt_sha256':sha(p)})
assert json.loads((out/'direct-receipt.json').read_text())['returncode']==0
rejections=json.loads((build/'rejections-final/receipt.json').read_text());assert len(rejections)==15 and all(r['rejected'] for r in rejections)
receipt={'scope':'Central handover review, not full scientific acceptance. Fresh committed-source release CLI and direct consumer; two fresh public cases in both predictor forms. Reviewed broader AF receipts without rerunning unchanged campaigns.',
 'production_revision':'742062270f0cd29af57c0a43299f23865d05d834','validation_revision':'543760f288dd7dffbcefb2d76efbf50892c84de1',
 'delivery_index_sha256':sha(index),'indexed_artifacts_verified':len(entries),'public_product_hashes_verified':products,'native_objects_verified':objects,
 'reported_public':public,'recorded_to_current_source_differences':source_differences,'source_reconciliation':source_reconciliation,'projection_mutation_receipt_sha256':sha(projection/'receipt.json'),
 'fresh_public':fresh,'fresh_checks':10660,'fresh_open_failures':2,'direct_consumer':'passed',
 'rms_localization':{'shared_phases_match_count':10,'maximum_original_phase_difference_turn':str(max(D(r['maximum_original_phase_difference_turn']) for r in local['records']))},
 'build_identity':json.loads((out/'identity.json').read_text()),'validation_identity':json.loads((out/'validation-identity.json').read_text())}
(out/'central-review-receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps({'indexed_artifacts_verified':len(entries),'products_verified':products,'native_objects_verified':objects,'source_differences':source_differences,'fresh_checks':10660,'fresh_open_failures':2,'receipt_sha256':sha(out/'central-review-receipt.json')},indent=2))
