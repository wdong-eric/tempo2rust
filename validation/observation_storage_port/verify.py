#!/usr/bin/env python3
"""R full checks; O's unchanged gate logic dispatches all historical regressions."""
import gzip,json,os,subprocess,sys
from pathlib import Path
import run as port

def main():
 port.OUT.mkdir(parents=True,exist_ok=True);receipt=port.OUT/'delivery_checks.json';receipt.unlink(missing_ok=True)
 frozen=port.protected();commands=[]
 def run(name,cmd,env=port.ENV):
  print(name,flush=True)
  p=subprocess.run(list(map(str,cmd)),cwd=port.ROOT,env=env,capture_output=True,text=True)
  log=port.OUT/(name+'.log');log.write_text(p.stdout+p.stderr)
  commands.append(dict(name=name,command=list(map(str,cmd)),returncode=p.returncode,log=log.name,log_sha256=port.sha(log)))
  port.save(port.OUT/'commands.json',commands);assert p.returncode==0,(name,p.returncode,p.stdout[-2500:],p.stderr[-2500:])
 for profile in ['debug','release']:run(profile+'_direct',[sys.executable,port.HERE/'run.py','--profile',profile])
 raw=[json.loads(gzip.decompress((port.OUT/(p+'_raw.json.gz')).read_bytes())) for p in ['debug','release']]
 for r in raw:
  for key in ['token','input_sha256']:r.pop(key)
 assert raw[0]==raw[1],'profile state mismatch'
 run('O_and_regressions',[sys.executable,port.ROOT/'validation/parameter_storage_port/verify.py'],env=dict(port.ENV,TEMPO2_PARAMETER_VERIFICATION_ROOT=str(port.OUT/'O')))
 o=port.load(port.OUT/'O/delivery_checks.json');reg=port.load(port.OUT/'O/regressions/delivery_checks.json')
 assert reg['m2_exit']==1 and len(reg['m2_failures'])==4
 assert port.load(port.OUT/'O/regressions/m2/open_gate_failures.json')==port.load(port.RUST/'verification/m2/open_gate_failures.json')
 direct=[port.load(port.OUT/(p+'_direct.json')) for p in ['debug','release']]
 import native
 for d in direct:port.validate_identity(d['identity'],port.identity(Path(d['executable'])));native.validate(d['native'])
 assert frozen==port.protected()
 modules={str(p.relative_to(port.ROOT)):{'sha256':port.sha(p),'unsafe_lines':[i for i,line in enumerate(p.read_text().splitlines(),1) if 'unsafe ' in line]} for p in sorted((port.RUST/'src').glob('*.rs'))}
 port.save(port.OUT/'module_inventory.json',modules)
 # Only safe string helper removal is allowed in an existing unsafe-containing file.
 baseline=port.load(port.HERE/'baseline.json')['sources']
 unchanged_unsafe={p:info['sha256']==baseline[p] for p,info in modules.items() if info['unsafe_lines'] and p!='rust/src/clock_text.rs'}
 assert all(unchanged_unsafe.values())
 port.save(receipt,dict(status='R checks pass; central acceptance pending; M2 unresolved',direct_profiles_identical=True,fields=95,direct_cases=3,m_histories=4,boundaries=direct[0]['boundaries'],counts_per_profile=direct[0]['counts'],controls_per_profile=len(direct[0]['controls']),protected_files=len(frozen),O_sha256=port.sha(port.OUT/'O/delivery_checks.json'),regressions_sha256=port.sha(port.OUT/'O/regressions/delivery_checks.json'),M2_failures=reg['m2_failures'],executed_head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=port.ROOT,text=True).strip(),executables={d['profile']:dict(path=d['executable'],sha256=d['identity']['executable_sha256'],native_sha256=d['native']['executable_sha256']) for d in direct},runner_sha256=port.sha(Path(__file__)),command_receipts_sha256=port.sha(port.OUT/'commands.json'),unchanged_unsafe_files=unchanged_unsafe))
 print('R exact checks, O and unchanged regressions finished; exactly four historical M2 failures retained.',flush=True)
if __name__=='__main__':main()
