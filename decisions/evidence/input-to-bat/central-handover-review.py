"""Central read-only AC handover review; uses existing declared evidence and binaries."""
from pathlib import Path
from decimal import Decimal
import hashlib, importlib.util, json, os, subprocess, sys, tempfile
sys.dont_write_bytecode=True
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust');HERE=ROOT/'validation/input_to_bat';BUILD=HERE/'build'
OUT=Path(tempfile.mkdtemp(prefix='tempo2-ac-handover-'))
def sha(p):
    h=hashlib.sha256()
    with p.open('rb') as f:
        for b in iter(lambda:f.read(1024*1024),b''):h.update(b)
    return h.hexdigest()
manifest=BUILD/'handover-receipt.json';m=json.loads(manifest.read_text())
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()
assert head=='d47ae5135d64fe61e52dee56e0eb2eaa68355cdc' and m['observed_git_head']==head
assert subprocess.check_output(['git','status','--porcelain'],cwd=ROOT)==b''
verified={}
for collection in ['authored_tree_at_receipt','evidence','public_workflows','components']:
    for item in m[collection]:
        p=ROOT/item['path'];assert sha(p)==item['sha256'],str(p)
        verified[item['path']]=item['sha256']
assert sha(next((ROOT/'decisions').glob('0008*')))=='2c5f81b82c43105fcd48b62139f1d78f2dc7c4089055ea162e9feef82ff4e12c'
for repository in [ROOT/'tempo2',ROOT/'validation/reference']:
    assert subprocess.check_output(['git','status','--porcelain'],cwd=repository)==b'',str(repository)
sys.path.insert(0,str(HERE));spec=importlib.util.spec_from_file_location('ac_compare',HERE/'check_pipeline.py')
compare=importlib.util.module_from_spec(spec);spec.loader.exec_module(compare)
comparator_sha=sha(HERE/'check_pipeline.py')
public=[];maxima={};native_ids=set();rust_ids=set()
for item in m['public_workflows']:
    p=ROOT/item['path'];r=json.loads(p.read_text());work=p.parent
    native=work/'native.tsv';assert sha(native)==r['native_execution']['output_sha256']
    assert r['native_execution']['executable_sha256']==r['native_sha256']
    for name,digest in r['native_execution']['inputs'].items():assert sha(Path(name))==digest,name
    assert r['comparator_sha256']==comparator_sha
    result=json.loads((work/('rust-'+item['profile']+'.json')).read_text())
    errors,measurements=compare.compare(native.read_text(),result)
    assert not errors,(item['case'],errors)
    assert measurements==r['measurements']
    for v in measurements:
        key=v['unit'];maxima[key]=max(maxima.get(key,Decimal(0)),abs(Decimal(v['difference'])))
    public.append({'case':item['case'],'profile':item['profile'],'measurements':len(measurements)})
    native_ids.add(r['native_sha256']);rust_ids.add((item['profile'],r['rust_sha256']))
assert len(public)==36
proof_path=BUILD/'propagation-corrected/native-proof.json';proof=json.loads(proof_path.read_text())
assert sha(BUILD/'propagation-corrected/corrections.patch')==proof['patch_sha256']
assert sha(ROOT/'validation/propagation/outputs/reference.json.gz')==proof['original_reference_sha256']
for name,digest in proof['linked_objects_sha256'].items():assert sha(Path(name))==digest,name
# Fresh public calls use only the final existing release binary, without a build.
rust=ROOT/'rust/target/release/tempo2-rust';rust_sha=sha(rust);fresh=[]
for case in ['ordinary','mixed_mean_tzr','mixed_tzr_mean','mixed_tropo_disabled']:
    work=BUILD/'pipeline'/case;r=json.loads((work/'comparison-release.json').read_text())
    nc=r['native_execution']['command'];n=int(nc[2]);pairs=nc[3:3+2*n];extra=nc[3+2*n:]
    cmd=[str(rust),'bats']
    for i in range(n):cmd += ['--par',pairs[2*i],'--tim',pairs[2*i+1]]
    cmd += ['--runtime',str(ROOT/'validation/reference/build/source/T2runtime'),'--',*extra]
    run=subprocess.run(cmd,cwd=ROOT,capture_output=True,timeout=60)
    (OUT/(case+'.json')).write_bytes(run.stdout);(OUT/(case+'.stderr')).write_bytes(run.stderr)
    assert run.returncode==0,(case,run.stderr.decode())
    result=json.loads(run.stdout);errors,measurements=compare.compare((work/'native.tsv').read_text(),result)
    assert not errors,(case,errors)
    fresh.append({'case':case,'command':cmd,'returncode':run.returncode,'output_sha256':hashlib.sha256(run.stdout).hexdigest(),'measurements':len(measurements)})
assert sha(rust)==rust_sha and sha(HERE/'check_pipeline.py')==comparator_sha
receipt={'scope':'implementation handover review, not full AC scientific acceptance','head':head,
         'manifest_sha256':sha(manifest),'review_source_sha256':sha(Path(__file__)),
         'verified_identity_count':len(verified),'authored_identity_count':len(m['authored_tree_at_receipt']),
         'public':public,'stored_max_abs_by_unit':{k:str(v) for k,v in maxima.items()},
         'native_executable_identities':sorted(native_ids),'recorded_rust_identities':sorted(rust_ids),
         'fresh_rust_executable_sha256':rust_sha,'fresh_public':fresh,'open_P_record_counts':m['open_P_record_counts'],
         'original_AB_record_counts':m['original_AB_record_counts'],'corrected_native_proof_sha256':sha(proof_path),
         'acceptance_pending':True}
(OUT/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps({'out':str(OUT),'receipt_sha256':sha(OUT/'receipt.json'),'verified_identity_count':len(verified),
                  'stored_public_comparisons':len(public),'maxima':receipt['stored_max_abs_by_unit'],
                  'fresh_public_cases':[v['case'] for v in fresh]},indent=2))
