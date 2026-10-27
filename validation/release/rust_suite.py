#!/usr/bin/env python3
"""Default debug/release suite and explicitly enabled runtime-dependent tests."""
import argparse
import os
from pathlib import Path
import re
from common import Run, sha

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','candidate','target']:p.add_argument('--'+name,type=Path,required=True)
    a=p.parse_args();run=Run(a.out);manifest=a.candidate.resolve()/'rust/Cargo.toml'
    env=dict(os.environ,CARGO_TARGET_DIR=str(a.target.resolve()),VECLIB_MAXIMUM_THREADS='1',OPENBLAS_NUM_THREADS='1')
    results=[]
    for profile,flags in [('debug',[]),('release',['--release'])]:
        command=['cargo','test','--offline','--locked','--manifest-path',manifest,*flags]
        for name,selection in [('all',['--all-targets']),('timing',['--lib','timing::tests::','--','--ignored','--test-threads=1']),
            ('constraints',['--lib','canonical_constraint_callbacks_survive_context_snapshot','--','--ignored','--exact','--test-threads=1']),
            ('jpl',['--lib','jpleph::tests::title_scan_boundaries_close_stream_and_allow_reinitialization','--','--ignored','--exact']),
            ('observer',['--lib','observer_coordinates_tests::existing_frontend_observes_imag_alias_mutation','--','--ignored','--exact']),
            ('clock-recovery',['--lib','terrestrial_tests::resources_after_terminal_clock_fault','--','--ignored','--exact'])]:
            if name=='constraints':selection[1]='general_fitting_tests::canonical_constraint_callbacks_survive_context_snapshot'
            label=profile+'-'+name
            stdout=run.run([*command,*selection],label,env=env,expected=None).decode(errors='replace')
            counts=[int(x) for x in re.findall(r'test result: ok\. (\d+) passed',stdout)]
            executed=sum(counts)>0
            results.append({'label':label,'exit':run.commands[-1]['status'],'executed':executed,'passing_tests':sum(counts)})
            print(results[-1],flush=True)
    for label,command in [('fmt',['cargo','fmt','--manifest-path',manifest,'--','--check']),
        ('clippy',['cargo','clippy','--offline','--locked','--manifest-path',manifest,'--all-targets','--','-D','warnings'])]:
        run.run(command,label,env=env,expected=None);results.append({'label':label,'exit':run.commands[-1]['status'],'executed':True})
    failed=[r for r in results if r['exit']!=0 or not r['executed']]
    run.save({'status':'failure' if failed else 'pass','results':results,'failed':failed,'manifest':sha(manifest),'lockfile':sha(manifest.with_name('Cargo.lock'))})
    raise SystemExit(bool(failed))
