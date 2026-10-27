#!/usr/bin/env python3
"""Clean AL execution. Scientific failures remain failures; independent stages continue."""
import argparse
import json
import os
from pathlib import Path
import platform
import sys
from common import ROOT, Run, sha


def qualify(args):
    run=Run(args.out);out=run.out;records={};python=sys.executable
    env=dict(os.environ,LC_ALL='C',VECLIB_MAXIMUM_THREADS='1',OPENBLAS_NUM_THREADS='1',
             OMP_NUM_THREADS='1',OMP_DYNAMIC='FALSE',CARGO_TARGET_DIR=str(out/'test-target'))
    provider=out/'providers';a2=out/'a2';candidate=out/'candidate/source'
    env['CALCEPH_DIR']=str(provider/'prefix')
    env['TEMPO2_ACCELERATE' if platform.system()=='Darwin' else 'TEMPO2_OPENBLAS']='1'
    def step(name,command,requires=(),extra_env=None):
        blocked=[key for key in requires if records[key]['exit']!=0]
        if blocked:
            records[name]={'exit':None,'blocked_by':blocked}
        else:
            run.run(command,name,cwd=ROOT,env=env if extra_env is None else dict(env,**extra_env),expected=None)
            records[name]={'exit':run.commands[-1]['status']}
        run.save({'status':'running','stages':records,'candidate':args.revision})
        print(name,records[name],flush=True)
        return records[name]['exit']==0
    def script(name,*values):return [python,ROOT/'validation/release'/name,*values]
    for label,command in [('host',['uname','-a']),('rust',['rustc','-Vv']),('cargo',['cargo','-V']),
                           ('linker',[os.environ.get('CC','clang'),'--version'])]:step(label,command)
    step('harness-controls',[python,'-m','unittest','discover','-s',ROOT/'validation/release','-p','test_*.py'])
    bootstrap=script('bootstrap.py','--out',a2)
    if args.native_repository:bootstrap+=['--native-repository',args.native_repository.resolve()]
    step('a2',bootstrap)
    repo=args.native_repository.resolve() if args.native_repository else a2/'upstream.git'
    provision=script('providers.py','--out',provider,'--cmake',args.cmake)
    for key in ['provider_source','ephemeris']:
        if getattr(args,key):provision+=['--source' if key=='provider_source' else '--data',getattr(args,key).resolve()]
    step('providers',provision)
    step('provider-abi',script('provider_abi.py','--out',out/'provider-abi','--provider',provider/'prefix',
                             '--ephemeris',provider/'inpop17a.dat'),['providers'])
    step('candidate',script('export_candidate.py','--out',out/'candidate','--revision',args.revision,'--a2',a2),['a2'])
    manifest=candidate/'rust/Cargo.toml'
    step('dependencies',['cargo','fetch','--locked','--manifest-path',manifest],['candidate'])
    step('m2-inputs',script('m2.py','--out',out/'m2-inputs','--a2',a2,'--candidate',candidate),['candidate'])
    cargo=['cargo','test','--offline','--locked','--manifest-path',manifest]
    step('rust-suite',script('rust_suite.py','--out',out/'rust-suite','--candidate',candidate,
         '--target',out/'test-target'),['dependencies','m2-inputs'])
    step('source-audit',script('source_audit.py','--out',out/'source-audit','--native-repository',repo),['a2'])
    step('package-list',['cargo','package','--list','--manifest-path',manifest],['candidate'])
    for label,features in [('default',''),('combined','calceph,lapack')]:
        prefix=out/('install-'+label)
        step('install-'+label,[python,candidate/'rust/install.py','--prefix',prefix,'--features',features],['dependencies','providers'])
        step('c-'+label,script('qualify_predictor.py','--prefix',prefix,'--out',out/('c-'+label),
             '--native-repository',repo),['install-'+label])
        step('c-concurrency-'+label,script('c_concurrency.py','--out',out/('c-concurrency-'+label),
             '--prefix',prefix),['install-'+label])
        step('external-'+label,script('qualify_rust.py','--prefix',prefix,'--out',out/('external-'+label),
             '--source-root',candidate,'--runtime',a2/'runtime','--ephemeris',provider/'inpop17a.dat',
             '--release','--features',features),['install-'+label])
    step('package-audit',script('audit.py','--candidate',candidate,
         '--prefix',out/'install-default','--prefix',out/'install-combined','--out',out/'package-audit'),
         ['install-default','install-combined'])
    for profile,features in [('disabled',''),('blas','blas'),('lapack','lapack'),('calceph','calceph'),
                             ('combined','calceph,lapack'),('qr-default','calceph,lapack,qr-default')]:
        core=out/('core-'+profile)
        step('native-'+profile,script('native_core.py','--out',core,'--native-repository',repo,'--a2',a2,
             '--provider',provider/'prefix','--profile',profile),['a2','providers'])
        feature_flags=['--features',features] if features else []
        step('backend-tests-'+profile,[*cargo,'--release','--test','native_backends',*feature_flags],['dependencies','providers'])
        step('backend-build-'+profile,['cargo','build','--offline','--locked','--manifest-path',manifest,
             '--release','--example','native_backends','--example','release_contraction',*feature_flags],['dependencies','providers'])
        native=core/'probe';rust=out/'test-target/release/examples/native_backends'
        step('paired-'+profile,script('check_backends.py','--out',out/('paired-'+profile),'--source-root',candidate,
             '--native',native,'--rust',rust,'--runtime',a2/'runtime','--ephemeris',provider/'inpop17a.dat',
             '--profile','combined' if profile=='qr-default' else profile),['native-'+profile,'backend-build-'+profile])
        if profile=='combined':
            step('contraction',script('contraction.py','--out',out/'contraction','--a2',a2,'--core',core,
                 '--rust-lomb',out/'test-target/release/examples/release_contraction','--rust-backend',rust),
                 ['native-combined','backend-build-combined'])
    step('qr-default-independent',[*cargo,'--release','--test','native_backends','--features','qr-default'],
         ['dependencies'])
    step('performance',script('performance.py','--out',out/'performance','--prefix',out/'install-default',
         '--a2',a2,'--core',out/'core-disabled','--predictor',out/'c-default/native-run/cheby.dat',
         '--runtime',a2/'runtime'),['native-disabled','c-default'])
    step('m2-example',['cargo','build','--offline','--locked','--release','--manifest-path',manifest,
         '--example','compare_fitting'],['dependencies'])
    step('m2-comparison',script('m2_compare.py','--out',out/'m2-comparison','--candidate',candidate,
         '--example',out/'test-target/release/examples/compare_fitting'),['m2-example','rust-suite'])
    step('native-fitting',script('native_fitting.py','--out',out/'native-fitting','--native-repository',repo,'--a2',a2),['a2'])
    step('fits',script('fit_campaign.py','--out',out/'fits','--candidate',candidate,'--native',out/'native-fitting/fit',
         '--rust',out/'install-default/bin/tempo2-rust'),['native-fitting','install-default'])
    step('boundaries',script('boundaries.py','--out',out/'boundaries','--a2',a2,
         '--rust',out/'external-default/client/target/release/release_boundaries'),['external-default'])
    step('ddh',script('ddh.py','--out',out/'ddh','--a2',a2,'--native',out/'native-fitting/fit',
         '--rust',out/'external-default/client/target/release/fitting_ideal','--source-root',candidate),
         ['native-fitting','external-default'])
    step('native-science',script('native_science.py','--out',out/'science','--core',out/'core-disabled','--a2',a2),['native-disabled'])
    step('timing',script('check_timing.py','--out',out/'timing','--native',out/'science',
         '--rust',out/'install-default/bin/tempo2-rust','--lifecycle',out/'external-default/client/target/release/timing_lifecycle',
         '--runtime',a2/'runtime'),['native-science','external-default'])
    step('predictors',script('check_predictor_science.py','--out',out/'predictors','--candidate',candidate,
         '--native',out/'science/predictor','--rust',out/'install-default/bin/tempo2-rust',
         '--abi-receipt',out/'c-default/receipt.json'),['native-science','c-default'])
    step('output',script('check_output.py','--out',out/'output','--native',out/'science',
         '--core',out/'core-disabled','--candidate',candidate),['native-science','install-default'])
    failed=[name for name,row in records.items() if row['exit'] not in (0,None)]
    blocked=[name for name,row in records.items() if row['exit'] is None]
    # The retained predecessor inventories/corpora are audited separately.
    # This automated matrix never claims their execution by omission.
    pending=['P and early input-corpus native reconstruction on Linux',
             'AK native OpenMP measured-worker and matched performance campaign on Linux',
             'Complete early-corpus and exhaustive body/branch execution closure'] if platform.system()=='Linux' else []
    run.save({'status':'failure' if failed or blocked else 'pass', 'qualification':'incomplete' if pending else 'matrix executed; central acceptance separate',
        'candidate':args.revision,'stages':records,'failed':failed,'blocked':blocked,'pending':pending,
        'runner':sha(__file__),'environment':{k:env[k] for k in ['FC','NATIVE_CC','NATIVE_CXX','CALCEPH_DIR',
             'CARGO_TARGET_DIR','TEMPO2_ACCELERATE','TEMPO2_OPENBLAS','VECLIB_MAXIMUM_THREADS','OPENBLAS_NUM_THREADS'] if k in env}})
    return bool(failed or blocked or pending)


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--out',type=Path,required=True);p.add_argument('--revision',required=True)
    p.add_argument('--native-repository',type=Path);p.add_argument('--cmake',default='cmake')
    p.add_argument('--provider-source',type=Path);p.add_argument('--ephemeris',type=Path)
    raise SystemExit(qualify(p.parse_args()))
