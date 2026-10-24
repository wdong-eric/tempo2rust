"""Canonical fresh verification; no stale success receipt survives a failed attempt."""
import json
import subprocess
import sys
from inventory import HERE,sha,save,check
from run import OUTPUT

STEPS=[
 ('ordinary','run.py','last_execution.json'),
 ('supplemental','supplemental.py','supplemental/last_execution.json'),
 ('independent','independent.py','independent.json'),
 ('observer','observer_audit.py','observer_audit.json'),
 ('controls','controls.py','controls.json'),
 ('branch_controls','branch_controls.py','branch_controls.json'),
 ('coverage','coverage.py','coverage.json')]
def source_hashes():
 return {p.name:sha(p.read_bytes()) for p in sorted([*HERE.glob('*.py'),HERE/'probe.cc',HERE/'CONTRACT.md'])}
def main():
 success=OUTPUT/'verification.json';success.unlink(missing_ok=True)
 for _,_,receipt in STEPS:(OUTPUT/receipt).unlink(missing_ok=True)
 before=source_hashes();oracles={str(p.relative_to(OUTPUT)):sha(p.read_bytes()) for p in [OUTPUT/'reference.json.gz',OUTPUT/'supplemental/reference.json.gz']}
 check();logs=OUTPUT/'pipeline';logs.mkdir(exist_ok=True);receipts={}
 for name,script,receipt in STEPS:
  print('RUN',name,flush=True)
  with (logs/(name+'.log')).open('wb') as log:
   proc=subprocess.run([sys.executable,str(HERE/script)],stdout=log,stderr=subprocess.STDOUT)
  assert proc.returncode==0,f'{name} failed; see {logs/(name+".log")}'
  assert source_hashes()==before,'verifier source changed during execution'
  data=json.loads((OUTPUT/receipt).read_text());assert data['status']=='PASS',(name,data['status'])
  receipts[name]=dict(path=receipt,sha256=sha((OUTPUT/receipt).read_bytes()))
  print('PASS',name,flush=True)
 subprocess.run([sys.executable,str(HERE/'field_map.py')],check=True)
 check();assert source_hashes()==before
 assert all(sha((OUTPUT/p).read_bytes())==h for p,h in oracles.items()),'an original oracle changed'
 save(success,dict(status='PASS',commands=[[sys.executable,str(HERE/s)] for _,s,_ in STEPS],sources=before,oracles=oracles,receipts=receipts,
  scope='25 original native routines; 311 primary plus 13 supplemental cases; no Rust changes or acceptance'))
 print('PASS complete fresh verification',flush=True)
if __name__=='__main__':main()
