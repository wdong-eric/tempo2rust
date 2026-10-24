"""W delivery checks; generated evidence stays beneath the ignored W output root."""
import json,subprocess,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
from run import ROOT,RUST,HERE,OUT,ENV,sha,save,identity
from freeze import check
from stage_regressions import stage

def main():
 OUT.mkdir(parents=True,exist_ok=True);receipt=OUT/'delivery.json';receipt.unlink(missing_ok=True)
 initial=identity();protected=check();commands=[]
 def command(name,args):
  print(name,flush=True);started=time.monotonic()
  p=subprocess.run(list(map(str,args)),cwd=ROOT,env=ENV,capture_output=True,text=True)
  log=OUT/(name+'.log');log.write_text(p.stdout+p.stderr)
  commands.append(dict(name=name,command=list(map(str,args)),returncode=p.returncode,elapsed_s=time.monotonic()-started,log=log.name,sha256=sha(log)))
  save(OUT/'commands.json',commands);assert p.returncode==0,(name,p.stdout[-2500:],p.stderr[-2500:])
 routing=stage()
 for profile in ['debug','release']:
  command(profile+'_direct',[sys.executable,HERE/'run.py','--profile',profile])
  command(profile+'_T',[sys.executable,routing/'pulsar_lifecycle_port/run.py','--profile',profile])
  command(profile+'_T_helpers',[sys.executable,routing/'pulsar_lifecycle_port/supplement.py','--profile',profile])
  for name in ['pulsar_lifecycle_tests::lifecycle_logging_after_clock_fault','terrestrial_tests::resources_after_terminal_clock_fault']:
   command(profile+'_'+name.split('::')[-1],['cargo','test','--manifest-path',RUST/'Cargo.toml','--locked','--offline','--lib',*(['--release'] if profile=='release' else []),name,'--','--exact','--ignored'])
 command('R_O_clock_JPL_M1_M2',[sys.executable,routing/'observation_storage_port/verify.py'])
 # Current public resource semantics are also exercised by M1b above. Keep a
 # separately named receipt at W so the integration result is easy to inspect.
 for profile in ['debug','release']:
  scratch=OUT/(profile+'_resources')
  if scratch.exists():
   import shutil
   shutil.rmtree(scratch)
  command(profile+'_resources',[RUST/'target'/profile/'examples/check_terrestrial_resources',ROOT/'tempo2/T2runtime',scratch,RUST/'target'/profile/'tempo2-rust'])
 assert identity()==initial and check()==protected
 profiles=[json.loads((OUT/p/'direct.json').read_text()) for p in ['debug','release']]
 for r in profiles:
  assert r['status']=='PASS' and len(r['cases'])==130
  assert sum(c['events'] for c in r['cases'])==2256
  assert sum(c['status']==1 for c in r['cases'])==24
  assert identity(Path(r['identity']['executable']))==r['identity']
  assert r['cases']==profiles[0]['cases']
 t=[json.loads((OUT/'T'/p/'direct.json').read_text()) for p in ['debug','release']]
 assert all(r['status']=='PASS' and len(r['cases'])==30 and sum(c['events'] for c in r['cases'])==133 for r in t)
 m2=json.loads((OUT/'R/O/regressions/m2/open_gate_failures.json').read_text())
 assert len(m2)==4 and m2==json.loads((RUST/'verification/m2/open_gate_failures.json').read_text())
 modules={str(p.relative_to(ROOT)):dict(sha256=sha(p),unsafe_lines=[i for i,l in enumerate(p.read_text().splitlines(),1) if 'unsafe ' in l]) for p in sorted((RUST/'src').glob('*.rs'))}
 save(OUT/'modules.json',modules)
 artifacts={str(p.relative_to(OUT)):sha(p) for p in [OUT/'commands.json',OUT/'routing/staging.json',OUT/'routing/unsafe_audit.json',OUT/'R/delivery_checks.json',OUT/'R/O/delivery_checks.json',OUT/'R/O/regressions/delivery_checks.json',OUT/'corrected_native/contract.json',OUT/'wet_original_mutation/receipt.json',*[OUT/p/n for p in ['debug','release'] for n in ['direct.json','controls.json']]]}
 save(OUT/'artifacts.json',artifacts)
 save(receipt,dict(status='W checks pass; central acceptance pending; M2 remains unaccepted',executed_head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),identity=initial,routines=22,cases_per_profile=130,actions_per_profile=1128,events_per_profile=2256,source_exits_per_profile=24,protected_frozen_entries=protected,controls_per_profile=len(json.loads((OUT/'debug/controls.json').read_text())['controls']),T_cases_per_profile=30,T_events_per_profile=133,M2_failures=m2,artifacts_sha256=sha(OUT/'artifacts.json'),modules_sha256=sha(OUT/'modules.json')))
 print('W complete: exact corrected U, T/O/R, clocks, JPL and M1 checks pass; four historical M2 diagnostics retained.',flush=True)
if __name__=='__main__':main()
