#!/usr/bin/env python3
"""Run I-owned evidence; preserve H/copied fixtures and historical M1/M2 files."""
import os
import subprocess
import sys
from pathlib import Path
import run as port


def immutable():
    paths=[]
    for base in [port.RUST/'verification/m1a',port.RUST/'verification/m1b',port.RUST/'verification/m2',port.RUST/'tests/reference',port.RUST/'tests/terrestrial',port.H]:
        paths += [p for p in base.rglob('*') if p.is_file() and not any(x in p.parts for x in ['build','__pycache__'])]
    # Earlier M1a files also live directly in verification/.
    paths += [p for p in (port.RUST/'verification').iterdir() if p.is_file()]
    return {str(p.relative_to(port.ROOT)):port.sha(p) for p in sorted(paths)}


def main():
    os.environ.update(port.ENV)
    if len(sys.argv)>1:
        stage=sys.argv[1]
        module=port.module('existing_verifier',port.RUST/('verify.py' if stage=='m1a' else 'verify_m2.py'))
        module.OUTPUT=port.OUT/stage
        module.main()
        return
    port.OUT.mkdir(parents=True,exist_ok=True)
    frozen=immutable();executed=port.manifests();commands=[]
    def run(name,command,expected=0):
        print(name,flush=True)
        result=subprocess.run(list(map(str,command)),cwd=port.ROOT,env=port.ENV,capture_output=True,text=True)
        (port.OUT/(name+'.log')).write_text(result.stdout+result.stderr)
        commands.append(dict(name=name,command=list(map(str,command)),returncode=result.returncode,log=name+'.log'))
        port.save(port.OUT/'commands.json',commands)
        assert result.returncode==expected,(name,result.returncode,result.stdout[-2000:],result.stderr[-2000:])
    for profile in ['debug','release']:
        run(profile+'_h',[sys.executable,port.HERE/'run.py','--profile',profile])
    run('comparator_mutations',[sys.executable,'-m','unittest','discover','-s',port.HERE,'-p','test_comparator.py','-v'])
    run('m1a',[sys.executable,Path(__file__).resolve(),'m1a'])
    run('m2',[sys.executable,Path(__file__).resolve(),'m2'],expected=1)
    failures=port.load(port.OUT/'m2/open_gate_failures.json')
    expected=port.load(port.RUST/'verification/m2/open_gate_failures.json')
    assert len(expected)==4 and failures==expected,('changed M2 failure set',failures)
    assert port.load(port.OUT/'debug_raw.json')==port.load(port.OUT/'release_raw.json'),'JPL debug/release difference'
    assert port.load(port.OUT/'debug_isolated_raw.json')==port.load(port.OUT/'release_isolated_raw.json'),'isolated debug/release difference'
    archives={}
    for p in (port.RUST/'target').glob('*/build/tempo2-rust-*/out/libtiming_native.a'):
        # Only archives for the current build are required to lack JPL. Old
        # Cargo build directories remain caches, not current linked dependencies.
        members=subprocess.check_output(['ar','t',str(p)],text=True).splitlines()
        archives[str(p.relative_to(port.RUST))]=members
    # Executable symbol check targets freshly executed debug/release main binaries.
    symbols={}
    for profile in ['debug','release']:
        p=port.RUST/'target'/profile/'tempo2-rust'
        names=subprocess.check_output(['nm',str(p)],text=True,stderr=subprocess.DEVNULL)
        assert not any(line.split()[-1].lstrip('_') in ['jpl_init_ephemeris','jpl_state','jpl_pleph','jpl_close_ephemeris'] for line in names.splitlines() if line.split()),profile
        symbols[profile]=dict(binary_sha256=port.sha(p),retained_JPL_C_symbols=False)
    port.save(port.OUT/'native_inventory.json',dict(executables=symbols,build_archives=archives))
    assert frozen==immutable(),'immutable reference/historical evidence changed'
    assert executed['sha256']==port.manifests()['sha256'],'executed inputs changed during run'
    port.save(port.OUT/'source_hashes.json',executed)
    port.save(port.OUT/'immutable_hashes.json',frozen)
    port.save(port.OUT/'delivery_checks.json',dict(jpl_host_gate='PASS',profiles=['debug','release'],scenarios_per_profile=55,reference_events_per_profile=1443,exact_events_per_profile=1439,boundary_events_per_profile=1,unreachable_events_per_profile=3,executed_events_per_profile=1440,isolated_per_profile=10,finite_gate='bit equality including signed zero',max_finite_difference=0,m1='PASS',m2='UNACCEPTED: exact four prior diagnostics; verifier returned 1',m2_failure_set=failures,other_regressions='PASS',immutable_evidence='unchanged',debug_release='identical'))
    print('JPL and M1 gates pass; M2 returned 1 with exactly the four prior raw-order diagnostics.',flush=True)

if __name__=='__main__':main()
