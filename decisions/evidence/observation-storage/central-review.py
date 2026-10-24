#!/usr/bin/env python3
"""Run delivered R's exact gates with only output/native-build roots relocated."""
import gzip,importlib.util,json,os,subprocess,sys,tempfile
from pathlib import Path
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust');HERE=ROOT/'validation/observation_storage_port';DELIVERY=ROOT/'rust/verification/observation_storage'
sys.path.insert(0,str(HERE));sys.dont_write_bytecode=True
import run as port
OUT=Path(tempfile.mkdtemp(prefix='tempo2-observation-central-',dir='/private/tmp'));port.OUT=OUT
import native
# native imported after setting port.OUT; its caller remains byte-identical R source.
assert native.OUT==OUT
receipt=Path('/tmp/tempo2-observation-central-review.json');receipt.unlink(missing_ok=True)
load=lambda p:json.loads(gzip.decompress(p.read_bytes()) if p.suffix=='.gz' else p.read_bytes())
def artifact_checks():
 seal=load(DELIVERY/'final_artifacts.json')
 for group in ['artifacts','source_and_projection_hashes']:
  for name,h in seal[group].items():
   p=ROOT/name
   assert port.sha(p)==h,(group,name)
 return seal
seal=artifact_checks();frozen=port.protected();profiles={}
print('PASS delivered artifacts and source/projection hashes; protected',len(frozen),flush=True)
for profile in ['debug','release']:
 expected=load(DELIVERY/(profile+'_direct.json'));port.validate_identity(expected['identity'],port.identity(Path(expected['executable'])));native.validate(expected['native'])
 sys.argv=[str(HERE/'run.py'),'--profile',profile]
 port.main()
 fresh=load(OUT/(profile+'_direct.json'))
 assert fresh['identity']==expected['identity'],'Rust executable/source identity changed'
 for suffix in ['_raw.json.gz','_native.json.gz']:
  a=load(DELIVERY/(profile+suffix));b=load(OUT/(profile+suffix))
  for key in ['token','input_sha256']:a.pop(key,None);b.pop(key,None)
  assert a==b,(profile,suffix,'delivered/fresh data differs')
 assert fresh['counts']==expected['counts']
 assert fresh['controls']==expected['controls'] and len(fresh['controls'])==39
 exe=Path(fresh['executable']);command=[str(exe),'observation_storage_tests::','--nocapture']
 p=subprocess.run(command,cwd=port.RUST,env=port.ENV,capture_output=True,text=True,timeout=60)
 assert p.returncode==0 and '5 passed; 0 failed; 1 ignored' in p.stdout,(p.returncode,p.stdout,p.stderr)
 (OUT/(profile+'_focused.log')).write_text(p.stdout+p.stderr)
 profiles[profile]=dict(direct=fresh,focused=dict(command=command,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr),raw_sha256={n:port.sha(OUT/(profile+n)) for n in ['_raw.json.gz','_native.json.gz']})
 print('PASS fresh',profile,'data equals delivery; focused tests 5 passed',flush=True)
assert artifact_checks()==seal and port.protected()==frozen
reg=load(DELIVERY/'O/regressions/delivery_checks.json')
assert reg['m2_exit']==1 and len(reg['m2_failures'])==4
assert load(DELIVERY/'O/regressions/m2/open_gate_failures.json')==load(port.RUST/'verification/m2/open_gate_failures.json')
result=dict(delivery='04918d5',production='b7a4d17',output=str(OUT),profiles=profiles,protected_files=len(frozen),artifact_count=len(seal['artifacts']),source_projection_count=len(seal['source_and_projection_hashes']),all_delivery_artifacts_preserved=True,exact_Rust_identities_match_delivery=True,all_fresh_state_equals_delivered=True,normalization_removed_only=['token','input_sha256'],M2_failure_list=reg['m2_failures'],full_regressions_independently_rerun=False,scope='Fresh debug/release exact direct gates and all 39 controls, plus five focused tests each. Only output/native-build roots relocated; no oracle, fixture, comparator, production source, precision or tolerance change. Full regression evidence inspected separately.')
port.save(receipt,result)
print('PASS complete central R direct review',receipt,flush=True)
