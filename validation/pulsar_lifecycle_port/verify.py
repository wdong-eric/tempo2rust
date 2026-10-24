"""T complete checks with all generated evidence confined beneath T."""
import json,os,subprocess,sys,time
from pathlib import Path
from run import ROOT,RUST,HERE,OUT,ENV,sha,save,protected,identity

def main():
 OUT.mkdir(parents=True,exist_ok=True);commands=[];started=identity();n=protected();receipt=OUT/'delivery.json';receipt.unlink(missing_ok=True)
 def command(name,args,env=ENV):
  print(name,flush=True);start=time.monotonic();p=subprocess.run(list(map(str,args)),cwd=ROOT,env=env,capture_output=True,text=True)
  log=OUT/(name+'.log');log.write_text(p.stdout+p.stderr);commands.append(dict(name=name,command=list(map(str,args)),returncode=p.returncode,elapsed_s=time.monotonic()-start,log=str(log.relative_to(OUT)),sha256=sha(log)));save(OUT/'commands.json',commands)
  assert p.returncode==0,(name,p.stdout[-2500:],p.stderr[-2500:])
 command('fmt',['cargo','fmt','--manifest-path','rust/Cargo.toml','--check'])
 command('clippy',['cargo','clippy','--manifest-path','rust/Cargo.toml','--locked','--offline','--all-targets','--','-D','warnings'])
 for profile in ['debug','release']:
  command(profile+'_direct',[sys.executable,HERE/'run.py','--profile',profile])
  command(profile+'_supplement',[sys.executable,HERE/'supplement.py','--profile',profile])
  command(profile+'_independent_diagnostics',['cargo','test','--manifest-path','rust/Cargo.toml','--locked','--offline','--lib',*(['--release'] if profile=='release' else []),'pulsar_lifecycle_tests::lifecycle_logging_after_clock_fault','--','--exact','--ignored'])
 command('R_O_and_regressions',[sys.executable,ROOT/'validation/observation_storage_port/verify.py'],dict(ENV,TEMPO2_OBSERVATION_VERIFICATION_ROOT=str(OUT/'R')))
 assert identity()==started and protected()==n
 profiles=[json.loads((OUT/p/'direct.json').read_text()) for p in ['debug','release']]
 for p in profiles:
  assert identity(Path(p['identity']['executable']))==p['identity']
  assert len(p['cases'])==30 and sum(c['events'] for c in p['cases'])==133
  assert all(c['events']==d['events'] and c['records']==d['records'] for c,d in zip(p['cases'],profiles[0]['cases']))
 r=json.loads((OUT/'R/delivery_checks.json').read_text());m2=json.loads((OUT/'R/O/regressions/m2/open_gate_failures.json').read_text())
 assert m2==json.loads((RUST/'verification/m2/open_gate_failures.json').read_text())
 modules={str(p.relative_to(ROOT)):dict(sha256=sha(p),unsafe_lines=[i for i,l in enumerate(p.read_text().splitlines(),1) if 'unsafe {' in l or 'unsafe extern' in l]) for p in sorted((RUST/'src').glob('*.rs'))}
 baseline=json.loads((OUT/'freeze/baseline.json').read_text())
 unchanged={p:meta['sha256']==baseline['rust'][p] for p,meta in modules.items() if meta['unsafe_lines']};assert all(unchanged.values())
 save(OUT/'modules.json',modules)
 artifacts={str(p.relative_to(OUT)):sha(p) for p in sorted(OUT.rglob('*')) if p.is_file() and not any(k in p.relative_to(OUT).parts for k in ['build','native']) and p.name not in ['delivery.json','artifacts.json']}
 save(OUT/'artifacts.json',artifacts)
 save(receipt,dict(status='T checks pass; central acceptance pending; M2 unaccepted',executed_head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),identity=started,cases_per_profile=30,events_per_profile=133,records_per_profile=sum(c['records'] for c in profiles[0]['cases']),controls_per_profile=len(profiles[0]['controls']),protected_files=n,unchanged_unsafe_files=unchanged,regression_receipt_sha256=sha(OUT/'R/delivery_checks.json'),M2_failures=m2,commands_sha256=sha(OUT/'commands.json'),modules_sha256=sha(OUT/'modules.json'),artifacts_sha256=sha(OUT/'artifacts.json')))
 print('T complete; exact S/Q, native witnesses, controls and regressions pass; historical M2 failures retained.',flush=True)
if __name__=='__main__':main()
