#!/usr/bin/env python3
"""Partial 0117 qualification; the complete 69-stage matrix remains required."""
import argparse
import json
import os
from pathlib import Path
import sys
from common import ROOT, Run, sha


def qualify(args):
    run=Run(args.out);out=run.out;records={};python=sys.executable
    env=dict(os.environ,LC_ALL='C',LANG='C',OPENBLAS_NUM_THREADS='1',OMP_NUM_THREADS='1',
             CARGO_TARGET_DIR=str(out/'test-target'))
    a2=out/'a2';candidate=out/'candidate/source';corpora=out/'native-corpora'
    scope='Partial 0117 harness, spin, measured contraction and event qualification; not the full 69-stage release matrix.'
    def script(name,*values):return [python,ROOT/'validation/release'/name,*values]
    def step(name,command,requires=()):
        blocked=[key for key in requires if records[key]['exit']!=0]
        if blocked:records[name]={'exit':None,'blocked_by':blocked}
        else:
            run.run(command,name,cwd=ROOT,env=env,expected=None)
            records[name]={'exit':run.commands[-1]['status']}
        run.save({'status':'running','scope':scope,'candidate':args.revision,'stages':records})
        print(name,records[name],flush=True)
    for name,command in [('host',['uname','-a']),('rust',['rustc','-Vv']),('cargo',['cargo','-V'])]:step(name,command)
    step('harness-controls',[python,'-m','unittest','discover','-s',ROOT/'validation/release','-p','test_*.py'])
    bootstrap=script('bootstrap.py','--out',a2)
    if args.native_repository:bootstrap+=['--native-repository',args.native_repository.resolve()]
    step('a2',bootstrap)
    repo=args.native_repository.resolve() if args.native_repository else a2/'upstream.git'
    step('candidate',script('export_candidate.py','--out',out/'candidate','--revision',args.revision,'--a2',a2),['a2'])
    manifest=candidate/'rust/Cargo.toml'
    step('dependencies',['cargo','fetch','--locked','--manifest-path',manifest],['candidate'])
    cargo=['cargo','test','--offline','--locked','--release','--manifest-path',manifest]
    # Obtain the causal source results early; scientific failures do not block
    # the independent public comparisons or the later complete corpus replay.
    for name,recipe in [('spin-source-profile','spin_profile.py'),('event-source-profile','event_profile.py')]:
        step(name,script(recipe,'--out',out/name,'--candidate',candidate,'--a2',a2,'--target',out/'test-target'),['dependencies','a2'])
    step('m2-inputs',script('m2.py','--out',out/'m2-inputs','--a2',a2,'--candidate',candidate),['candidate'])
    step('m2-shared',[*cargo,'--lib','fitting_tests::independent_source_solver_inputs_and_callbacks','--','--exact'],['dependencies','m2-inputs'])
    step('public-build',['cargo','build','--offline','--locked','--release','--manifest-path',manifest,
        '--bin','tempo2-rust','--example','compare_fitting','--example','timing_lifecycle'],['dependencies'])
    binary=out/'test-target/release/tempo2-rust'
    step('m2-comparison',script('m2_compare.py','--out',out/'m2-comparison','--candidate',candidate,
        '--signed-reference',out/'m2-inputs/signed-reference','--example',out/'test-target/release/examples/compare_fitting'),['public-build','m2-shared'])
    step('native-fitting',script('native_fitting.py','--out',out/'native-fitting','--native-repository',repo,'--a2',a2),['a2'])
    step('public-spin',script('fit_campaign.py','--out',out/'fits','--candidate',candidate,
        '--native',out/'native-fitting/fit','--rust',binary,'--public-case','spin'),['native-fitting','public-build'])
    # The disabled core has no optional provider dependency. Its existing
    # builder accepts a provider include path but never reads a provider here.
    step('native-disabled',script('native_core.py','--out',out/'core-disabled','--native-repository',repo,
        '--a2',a2,'--provider',out/'unused-provider','--profile','disabled'),['a2'])
    step('native-science',script('native_science.py','--out',out/'science','--core',out/'core-disabled','--a2',a2),['native-disabled'])
    step('timing-events',script('check_timing.py','--out',out/'timing','--native',out/'science',
        '--rust',binary,'--lifecycle',out/'test-target/release/examples/timing_lifecycle',
        '--runtime',a2/'runtime','--case','events'),['native-science','public-build'])
    # A failed numerical timing gate still supplies the original native products.
    step('event-diagnostic',script('event_diagnostic.py','--out',out/'event-diagnostic','--core',out/'core-disabled',
        '--science',out/'science','--timing',out/'timing','--runtime',a2/'runtime',
        '--rust',binary,'--candidate',candidate),['native-science','public-build'])
    selected=['parameters','par_specialized','terrestrial_resources','observer_coordinates']
    step('native-corpora',script('native_corpora.py','--out',corpora,'--native-repository',repo,
        '--a2',a2,'--stages',*selected),['a2'])
    # Run X's controls in its new native staging, where the retained shallow
    # history is available. The later comparison copy intentionally omits .git.
    step('observer-native-controls',[python,corpora/'staging/validation/observer_coordinates/controls.py'],['a2'])
    # corpus_compare records per-corpus availability and retains other failures.
    step('corpora-comparison',script('corpus_compare.py','--out',out/'corpus','--candidate',candidate,
        '--reference-root',corpora/'staging','--target',out/'corpus-target','--stages',*selected),['dependencies'])
    # The retained original-producer failure must not hide corrected controls.
    step('observer-controls',script('observer_controls.py','--out',out/'observer-controls','--corpus',out/'corpus'),['dependencies'])
    step('contraction-source-profile',script('contraction_profile.py','--out',out/'contraction-source-profile',
        '--observer',corpora/'staging/validation/observer_coordinates',
        '--terrestrial',corpora/'staging/validation/terrestrial_resources'),['a2'])
    step('format',['cargo','fmt','--manifest-path',manifest,'--','--check'],['candidate'])
    step('clippy',['cargo','clippy','--offline','--locked','--manifest-path',manifest,'--all-targets','--','-D','warnings'],['dependencies'])
    failed=[name for name,row in records.items() if row['exit'] is not None and row['exit']!=0]
    blocked=[name for name,row in records.items() if row['exit'] is None]
    result={'status':'failure' if failed or blocked else 'pass','scope':scope,'candidate':args.revision,
        'stages':records,'failed':failed,'blocked':blocked,'producer':sha(__file__),
        'unrun':'All release stages outside this explicit partial recipe, including the complete timing/fit/output/provider/installed-client matrix, remain required.'}
    (out/'matrix.json').write_text(json.dumps(result,indent=2)+'\n');run.save(result)
    return bool(failed or blocked)


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--out',type=Path,required=True);p.add_argument('--revision',required=True)
    p.add_argument('--native-repository',type=Path)
    raise SystemExit(qualify(p.parse_args()))
