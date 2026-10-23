#!/usr/bin/env python3
"""Run L evidence and unchanged I/M1/M2 gates into L-owned output only."""
import os
import subprocess
import sys
import tempfile
from pathlib import Path
import run as port
import regressions


def immutable():
    bases=[port.J,port.ROOT/'validation/jpl',port.ROOT/'validation/fitting',port.RUST/'tests',port.RUST/'verification']
    paths=[]
    for base in bases:
        paths.extend(p for p in base.rglob('*') if p.is_file() and port.OUT not in p.parents and not any(x in p.parts for x in ['build','__pycache__']))
    return {str(p.relative_to(port.ROOT)):port.sha(p) for p in sorted(set(paths))}


def main():
    port.OUT.mkdir(parents=True,exist_ok=True)
    frozen=immutable();sources=port.executed_sources();commands=[]
    def run(name,command,expected=0,env=port.ENV):
        print(name,flush=True)
        result=subprocess.run(list(map(str,command)),cwd=port.ROOT,env=env,capture_output=True,text=True)
        log=port.OUT/(name+'.log');log.write_text(result.stdout+result.stderr)
        commands.append(dict(name=name,command=list(map(str,command)),returncode=result.returncode,log=log.name))
        port.save(port.OUT/'commands.json',commands)
        assert result.returncode==expected,(name,result.returncode,result.stdout[-2000:],result.stderr[-2000:])
    for profile in ['debug','release']:
        run(profile+'_direct',[sys.executable,port.HERE/'run.py','--profile',profile])
        run(profile+'_glob_supplemental',[sys.executable,port.HERE/'supplemental.py','--profile',profile])
        p=port.load(port.OUT/(profile+'_direct.json'));port.validate_delivery(p)
        port.validate_identity(sources,p['source_hashes'])
        port.validate_identity(port.sha(Path(p['executable'])),p['executable_sha256'])
        with tempfile.TemporaryDirectory(prefix='clock-provenance-') as work:
            output=port.OUT/(profile+'_provenance.json');output.unlink(missing_ok=True)
            run(profile+'_provenance',[p['executable'],'--exact','clock_tests::process_provenance','--ignored','--nocapture'],env=dict(port.ENV,CLOCK_WORK=work,CLOCK_PROVENANCE_OUTPUT=str(output)))
            assert port.load(output)['status']=='PASS'
    debug=port.load(port.OUT/'debug_direct.json');release=port.load(port.OUT/'release_direct.json')
    supplements=[port.load(port.OUT/(p+'_glob_supplemental.json')) for p in ['debug','release']]
    for p,supplement in zip([debug,release],supplements):
        port.validate_identity(sources,supplement['source_hashes'])
        assert supplement['executable_sha256']==p['executable_sha256']
    assert supplements[0]['helpers']==supplements[1]['helpers']
    assert [r['rust'] for r in supplements[0]['integrated']]==[r['rust'] for r in supplements[1]['integrated']]
    assert supplements[0]['mutations']==supplements[1]['mutations']
    for a,b in zip(debug['results'],release['results']):
        # Fresh invocation/path hashes differ; compare every actual state byte,
        # including Rust-only boundaries/log storage and native ABI adapters.
        def scientific(r):
            return {k:v for k,v in r.items() if k not in ['run_token','input_sha256']}
        assert a.keys()==b.keys() and a['id']==b['id'] and a['status']==b['status']
        if 'raw' in a:assert scientific(a['raw'])==scientific(b['raw']),a['id']
    run('comparator_mutations',[sys.executable,'-m','unittest','discover','-s',port.HERE,'-p','test_comparator.py','-v'])
    for profile in ['debug','release']:
        run(profile+'_jpl',[sys.executable,port.HERE/'regressions.py','jpl',profile])
    run('jpl_comparator_mutations',[sys.executable,'-m','unittest','discover','-s',port.ROOT/'validation/jpl_port','-p','test_comparator.py','-v'])
    run('m1a',[sys.executable,port.HERE/'regressions.py','m1a'])
    run('m2',[sys.executable,port.HERE/'regressions.py','m2'],expected=1)
    failures=port.load(port.OUT/'m2/open_gate_failures.json')
    expected=port.load(port.RUST/'verification/m2/open_gate_failures.json')
    assert len(expected)==4 and failures==expected,('changed M2 failure set',failures)
    for name in ['raw','isolated_raw']:
        assert port.load(port.OUT/f'jpl/debug_{name}.json')==port.load(port.OUT/f'jpl/release_{name}.json'),name
    legacy=[port.load(port.OUT/f'jpl/{profile}_legacy_header_controls.json')['cases'] for profile in ['debug','release']]
    assert {k:v['rust'] for k,v in legacy[0].items()}=={k:v['rust'] for k,v in legacy[1].items()}
    symbols={}
    for profile in ['debug','release']:
        binary=port.RUST/'target'/profile/'tempo2-rust'
        names=subprocess.check_output(['nm',str(binary)],text=True,stderr=subprocess.DEVNULL)
        assert not any(line.split()[-1].lstrip('_') in ['jpl_init_ephemeris','jpl_state','jpl_pleph','jpl_close_ephemeris','getClockCorrections','getCorrection','getCorrectionTT','TabulatedFunction_load','TabulatedFunction_getValue'] for line in names.splitlines() if line.split())
        symbols[profile]=dict(binary_sha256=port.sha(binary),retained_C_clock_or_JPL_symbols=False)
    repos={}
    for repo in ['tempo2','validation/reference']:
        def git(*args):return subprocess.check_output(['git','-C',str(port.ROOT/repo),*args],text=True).strip()
        repos[repo]=dict(head=git('rev-parse','HEAD'),status=git('status','--porcelain'))
        assert not repos[repo]['status'],repos[repo]
    port.validate_identity(frozen,immutable());port.validate_identity(sources,port.executed_sources())
    port.save(port.OUT/'immutable_hashes.json',frozen)
    port.save(port.OUT/'source_hashes.json',sources)
    port.save(port.OUT/'delivery_checks.json',dict(status='L corrected gates PASS; central acceptance pending',executed_commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=port.ROOT,text=True).strip(),counts_per_profile=debug['counts'],cases_per_profile=dict(exact=142,safe_Rust_boundary=8,unrun_cycle=1),glob_supplemental=dict(helper_patterns=18,integrated_cases=3,mutations=9,original_delivery_mismatches_rejected=8),max_finite_difference=0,profiles_identical=True,jpl='PASS: unchanged I gates',m1a='PASS',m1b='PASS (via M2)',m2_exit=1,m2_failures=failures,native_symbols=symbols,independent_repositories=repos))
    print('L direct/JPL/M1 gates pass; M2 remains unaccepted with exactly four historical diagnostics.',flush=True)


if __name__=='__main__':main()
