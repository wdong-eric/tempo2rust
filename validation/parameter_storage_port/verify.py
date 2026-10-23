#!/usr/bin/env python3
"""Full O verification with accepted regressions redirected, never rewritten."""
import gzip,json,os,subprocess,sys
from pathlib import Path
import run as port

def main():
    port.OUT.mkdir(parents=True,exist_ok=True)
    receipt=port.OUT/'delivery_checks.json';receipt.unlink(missing_ok=True)
    commands=[];frozen=port.protected()
    def run(name,cmd,env=port.ENV):
        print(name,flush=True)
        p=subprocess.run(list(map(str,cmd)),cwd=port.ROOT,env=env,capture_output=True,text=True)
        log=port.OUT/(name+'.log');log.write_text(p.stdout+p.stderr)
        commands.append(dict(name=name,command=list(map(str,cmd)),returncode=p.returncode,log=log.name,log_sha256=port.sha(log)))
        port.save(port.OUT/'commands.json',commands);assert p.returncode==0,(name,p.returncode,p.stdout[-1500:],p.stderr[-1500:])
    run('fmt',['cargo','fmt','--manifest-path','rust/Cargo.toml','--check'])
    run('clippy',['cargo','clippy','--manifest-path','rust/Cargo.toml','--offline','--locked','--all-targets','--','-D','warnings'])
    for profile in ['debug','release']:
        run(profile+'_tests',['cargo','test','--manifest-path','rust/Cargo.toml','--offline','--locked']+(['--release'] if profile=='release' else []))
        run(profile+'_direct',[sys.executable,port.HERE/'run.py','--profile',profile])
    direct=[port.load(port.OUT/(p+'_direct.json')) for p in ['debug','release']]
    raw=[json.loads(gzip.decompress((port.OUT/(p+'_raw.json.gz')).read_bytes())) for p in ['debug','release']]
    for r in raw:
        for key in ['token','input_sha256']:r.pop(key)
    assert raw[0]==raw[1], 'debug/release state difference'
    for receipt_direct in direct:
        port.validate_identity(receipt_direct['identity'],port.identity(Path(receipt_direct['executable'])))
    # The unchanged L runner dispatches subprocesses; only its output-root plumbing differs.
    regression_root=port.OUT/'regressions'
    regression_root.mkdir(exist_ok=True)
    env=dict(port.ENV,TEMPO2_CLOCK_VERIFICATION_ROOT=str(regression_root))
    print('redirected_clock_JPL_M1_M2',flush=True)
    cmd=[sys.executable,str(port.ROOT/'validation/clocks_port/verify.py')]
    p=subprocess.run(cmd,cwd=port.ROOT,env=env,capture_output=True,text=True)
    log=regression_root/'dispatch.log';log.write_text(p.stdout+p.stderr)
    commands.append(dict(name='redirected_clock_JPL_M1_M2',command=cmd,returncode=p.returncode,log=str(log.relative_to(port.OUT)),log_sha256=port.sha(log),environment={'TEMPO2_CLOCK_VERIFICATION_ROOT':str(regression_root)}));port.save(port.OUT/'commands.json',commands)
    assert p.returncode==0,(p.stdout[-1500:],p.stderr[-1500:])
    regression=port.load(regression_root/'delivery_checks.json')
    assert regression['m2_exit']==1 and len(regression['m2_failures'])==4
    assert port.load(regression_root/'m2/open_gate_failures.json')==port.load(port.RUST/'verification/m2/open_gate_failures.json')
    assert frozen==port.protected()
    for r in direct:port.validate_identity(r['identity'],port.identity(Path(r['executable'])))
    modules={str(p.relative_to(port.ROOT)):{'sha256':port.sha(p),'unsafe_lines':[i for i,line in enumerate(p.read_text().splitlines(),1) if 'unsafe ' in line]} for p in sorted((port.RUST/'src').glob('*.rs'))}
    port.save(port.OUT/'module_inventory.json',modules)
    port.save(receipt,dict(status='O checks pass; central acceptance pending; M2 unresolved',direct_profiles_identical=True,counts_per_profile=direct[0]['counts'],direct_cases_per_profile=10,events_per_profile=direct[0]['events'],controls_per_profile=len(direct[0]['controls']),protected_files=len(frozen),regressions_sha256=port.sha(regression_root/'delivery_checks.json'),M2_failures=regression['m2_failures'],executed_head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=port.ROOT,text=True).strip(),executables={r['profile']:dict(path=r['executable'],sha256=r['identity']['executable_sha256'],native_sha256=r['native']['executable_sha256']) for r in direct},runner_sha256=port.sha(Path(__file__)),command_receipts_sha256=port.sha(port.OUT/'commands.json')))
    print('O exact checks and unchanged regressions finished; four M2 failures retained.',flush=True)
if __name__=='__main__':main()
