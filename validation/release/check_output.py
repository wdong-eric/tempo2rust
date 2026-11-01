#!/usr/bin/env python3
"""Run unchanged AH gates with the recorded native compilation directory mapped."""
import argparse
import importlib.util
import json
from pathlib import Path
import sys
from common import ROOT, sha
from diagnostic_attribution import Attribution

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','native','core','candidate','rust']:p.add_argument('--'+name,type=Path,required=True)
    a=p.parse_args();native=a.native.resolve();core=a.core.resolve()
    attribution=Attribution(a.candidate)
    identity=json.loads((native/'receipt.json').read_text())
    assert identity['core_receipt']==sha(core/'receipt.json')
    for name in json.loads((core/'receipt.json').read_text())['source']:
        assert sha(core/'source'/name)==sha(native/'source'/name)
    path=a.candidate.resolve()/'validation/core_output/check.py'
    spec=importlib.util.spec_from_file_location('al_ah',path);m=importlib.util.module_from_spec(spec)
    # AH's source-tree target is absent in a clean private installation. Bind
    # the actual installed CLI explicitly; leave every workflow/check unchanged.
    source=path.read_text()
    original_cli="    rust = ROOT / 'rust/target' / ('release' if args.release else 'debug') / 'tempo2-rust'"
    assert source.count(original_cli)==1
    installed_cli="    rust = Path("+repr(str(a.rust.resolve()))+")"
    exec(compile(source.replace(original_cli,installed_cli),str(path),'exec'),m.__dict__)
    original=m.compare
    def compare(left,right,normalize=lambda data,kind:data,diff_dir=None):
        def normalize_source(data,kind):
            if kind=='native':
                data=data.replace(b'['+str(core/'source').encode()+b'/',b'['+str(native/'source').encode()+b'/')
            return attribution.stream(normalize(data,kind),kind)
        return original(left,right,normalize_source,diff_dir)
    m.compare=compare
    # The historical 75-second budget is for the API execution, not a cold build.
    # Build the exact selected libtest binary once, recording its own budget.
    original_run=m.run
    build_receipt=None
    def run(command,directory,env,expected=0,timeout=75):
        global build_receipt
        if command[0]=='cargo' and 'core_output_tests::write_oracle_evidence' in command:
            build=[*command[:command.index('core_output_tests::write_oracle_evidence')],
                   '--offline','--locked','--lib','--no-run','--message-format=json']
            build_receipt=original_run(build,directory.with_name('probe-build'),env,timeout=900)
            if build_receipt['status']!=0:
                return {**build_receipt,'expected':expected,'execution_unavailable':'libtest build did not complete'}
            artifacts=[json.loads(line) for line in (directory.with_name('probe-build')/'stdout').read_text().splitlines() if line.startswith('{')]
            executables=[Path(row['executable']) for row in artifacts if row.get('reason')=='compiler-artifact' and row.get('profile',{}).get('test') and row.get('executable')]
            assert len(executables)==1,executables
            exe=executables[0];build_receipt['test_executable']={'path':str(exe),'sha256':sha(exe)}
            command=[exe,'core_output_tests::write_oracle_evidence','--ignored','--exact','--test-threads=1']
        return original_run(command,directory,env,expected=expected,timeout=timeout)
    m.run=run
    sys.argv=[str(path),'--native',str(native),'--out',str(a.out),'--release']
    status=m.main()
    receipt=a.out/'receipt.json';record=json.loads(receipt.read_text())
    record['al_source_directory_binding']={'wrapper':sha(__file__),'compiled':str(core/'source'),
        'observer_copy':str(native/'source'),'core_receipt':sha(core/'receipt.json'),
        'meaning':'Exact known __FILE__ directory relocation before the unchanged pinned-line mapping; no numerical/text-value normalization'}
    record['rust_diagnostic_attribution']=attribution.manifest()
    record['direct_api_build']=build_receipt
    record['installed_cli_binding']={'path':str(a.rust.resolve()),'sha256':sha(a.rust),
        'original':original_cli,'adapted':installed_cli,'historical_checker':sha(path)}
    receipt.write_text(json.dumps(record,indent=2)+'\n')
    raise SystemExit(status)
