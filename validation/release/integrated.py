#!/usr/bin/env python3
"""Clean AL execution. Scientific failures remain failures; independent stages continue."""
import argparse
import json
import os
from pathlib import Path
import platform
import sys
from common import ROOT, Run, sha

ALLOCATION_REVISIONS={
    "before":"dc435f53a64e4dd5c25cae6319841a28bd981fcb",
    "after":"68973d669af0e1180ad8532bb8b364452ca203d7",
}
PREDICTOR_REVISIONS={
    "before":"68973d669af0e1180ad8532bb8b364452ca203d7",
    "after":"0bd8e2cf2ef54e9da6feace122acd8aa96aa3fbe",
}
EVALUATOR_REVISIONS={
    "before":"0bd8e2cf2ef54e9da6feace122acd8aa96aa3fbe",
    "after":"8716d1dc026641f3a51c2ba88850c5599293cbdd",
}
RECURRENCE_REVISIONS={
    "before":"8716d1dc026641f3a51c2ba88850c5599293cbdd",
    "after":"e24622d5210fdfc8abe10f9f58a21b7e4a2a1aca",
}
COEFFICIENT_REVISIONS={
    "before":"e24622d5210fdfc8abe10f9f58a21b7e4a2a1aca",
    "after":"a1851c55fcc0b1ac733b064e631bfbbd2e0118d5",
}

TEMPORARY_REVISIONS={
    "before":"a1851c55fcc0b1ac733b064e631bfbbd2e0118d5",
    "after":"e1d34d835377451dcc7822161afce96a87948005",
}


def qualify(args):
    run=Run(args.out);out=run.out;records={};python=sys.executable
    env=dict(os.environ,LC_ALL='C',VECLIB_MAXIMUM_THREADS='1',OPENBLAS_NUM_THREADS='1',
             OMP_NUM_THREADS='1',OMP_DYNAMIC='FALSE',CARGO_TARGET_DIR=str(out/'test-target'),
             GMP_MPFR_SYS_CACHE=os.environ.get('GMP_MPFR_SYS_CACHE',str(out/'gmp-cache')))
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
    corpora=out/'native-corpora'
    step('native-corpora',script('native_corpora.py','--out',corpora,'--native-repository',repo,
         '--a2',a2,'--coverage'),['a2'])
    # A coverage/observer failure must not suppress unrelated ordinary corpora.
    # The comparator checks and records each stage's native prerequisites.
    step('corpora-comparison',script('corpus_compare.py','--out',out/'corpus','--candidate',candidate,
         '--reference-root',corpora/'staging','--target',out/'test-target'),['dependencies'])
    step('contraction-source-profile',script('contraction_profile.py','--out',out/'contraction-source-profile',
         '--observer',corpora/'staging/validation/observer_coordinates',
         '--terrestrial',corpora/'staging/validation/terrestrial_resources',
         '--planetary',corpora/'staging/validation/planetary_ephemeris',
         '--propagation',corpora/'staging/validation/propagation'),['a2'])
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
            step('lomb-diagnostic',script('lomb_diagnostic.py','--out',out/'lomb-diagnostic',
                 '--prefix',out/'install-default','--contraction',out/'contraction',
                 '--target',out/'external-default/client/target'),
                 ['native-combined','backend-build-combined','external-default'])
    step('qr-default-independent',[*cargo,'--release','--test','native_backends','--features','qr-default'],
         ['dependencies'])
    step('performance',script('performance.py','--out',out/'performance','--prefix',out/'install-default',
         '--a2',a2,'--core',out/'core-disabled','--predictor',out/'c-default/native-run/cheby.dat',
         '--runtime',a2/'runtime'),['native-disabled','c-default'])
    step('workers',script('workers.py','--out',out/'workers','--prefix',out/'install-default',
         '--a2',a2,'--core',out/'core-disabled','--native-repository',repo),['native-disabled','install-default'])
    step('m2-example',['cargo','build','--offline','--locked','--release','--manifest-path',manifest,
         '--example','compare_fitting'],['dependencies'])
    step('m2-comparison',script('m2_compare.py','--out',out/'m2-comparison','--candidate',candidate,
         '--signed-reference',out/'m2-inputs/signed-reference',
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
    step('event-diagnostic',script('event_diagnostic.py','--out',out/'event-diagnostic',
         '--core',out/'core-disabled','--science',out/'science','--timing',out/'timing',
         '--runtime',a2/'runtime','--rust',out/'install-default/bin/tempo2-rust','--candidate',candidate),['native-science','external-default'])
    step('event-source-profile',script('event_profile.py','--out',out/'event-source-profile',
         '--candidate',candidate,'--a2',a2,'--target',out/'test-target','--nan-trace'),['dependencies','a2'])
    step('spin-source-profile',script('spin_profile.py','--out',out/'spin-source-profile',
         '--candidate',candidate,'--a2',a2,'--target',out/'test-target'),['dependencies','a2'])
    step('predictors',script('check_predictor_science.py','--out',out/'predictors','--candidate',candidate,
         '--native',out/'science/predictor','--rust',out/'install-default/bin/tempo2-rust',
         '--abi-receipt',out/'c-default/receipt.json'),['native-science','c-default'])
    step('output',script('check_output.py','--out',out/'output','--native',out/'science',
         '--core',out/'core-disabled','--candidate',candidate,'--rust',out/'install-default/bin/tempo2-rust'),['native-science','install-default'])
    # 0123 freezes an allocator-only pair. Its Cargo.lock matches the populated
    # candidate cache; AN overrides our shared target separately for each export.
    allocation=out/'allocation';allocation.mkdir()
    an=candidate/'validation/allocation_profile'
    step('allocation-path-controls',[python,an/'check_build_paths.py',allocation/'path-controls'],
         ['candidate','dependencies'])
    for label,revision in ALLOCATION_REVISIONS.items():
        step('allocation-source-'+label,['git','-C',ROOT,'cat-file','-e',revision+'^{commit}'])
        step('allocation-build-'+label,[python,ROOT/'validation/allocation_profile/build.py',
             '--revision',revision,'--out',allocation/label,'--a2',a2],
             ['a2','dependencies','allocation-source-'+label,'allocation-path-controls'])
        # Reuse the candidate's canonical header/runtime binding outside each
        # frozen rust/ manifest. Debug controls do not depend on S gate success.
        step('allocation-resources-'+label,['ln','-s',candidate/'tempo2',allocation/label/'source/tempo2'],
             ['allocation-build-'+label,'candidate'])
    step('allocation-libc',['ldd','--version'] if platform.system()=='Linux' else ['sw_vers'])
    step('allocation-measure',[python,an/'measure.py','--before',allocation/'before',
         '--after',allocation/'after','--native-cli',out/'core-disabled/tempo2','--a2',a2,
         '--out',allocation/'measure','--samples','5'],
         ['allocation-build-before','allocation-build-after','native-disabled','candidate'])
    # Check S's ordinary record and its separate corrected-native proof, not the
    # aggregate corpus exit: unrelated failures must not suppress valid S inputs.
    step('allocation-verify',script('allocation_verify.py','--out',allocation/'verify',
         '--before',allocation/'before','--after',allocation/'after','--a2',a2,
         '--candidate',candidate,'--native-corpora',corpora,'--native-fit',out/'native-fitting/fit',
         '--lifecycle-corrected',out/'corpus/lifecycle-corrected-native'),
         ['allocation-resources-before','allocation-resources-after','native-fitting','candidate'])
    step('allocation-predictor-after',[python,an/'predictor_gate.py','--out',allocation/'predictor-after',
         '--a2',a2,'--native',out/'science/predictor','--rust',allocation/'after/bin/tempo2-rust',
         '--abi-receipt',out/'c-default/receipt.json'],['allocation-build-after','native-science','c-default','candidate'])
    if records['allocation-predictor-after']['exit'] not in (0,None):
        step('allocation-predictor-before',[python,an/'predictor_gate.py','--out',allocation/'predictor-before',
             '--a2',a2,'--native',out/'science/predictor','--rust',allocation/'before/bin/tempo2-rust',
             '--abi-receipt',out/'c-default/receipt.json'],['allocation-build-before','native-science','c-default','candidate'])
    after_manifest=allocation/'after/source/rust/Cargo.toml'
    step('allocation-providers',['cargo','test','--offline','--locked','--release','--manifest-path',after_manifest,
         '--features','calceph,lapack','--test','native_backends','--','--test-threads=1'],
         ['allocation-build-after','providers'],extra_env={'CARGO_TARGET_DIR':str(allocation/'provider-target')})
    step('allocation-debug-lifecycle',['cargo','test','--offline','--locked','--manifest-path',after_manifest,
         '--lib','pulsar_lifecycle_tests::','--','--test-threads=1'],['allocation-resources-after'],
         extra_env={'CARGO_TARGET_DIR':str(allocation/'debug-target')})
    # AO's installed predictor pair is distinct from the combined candidate.
    predictor=out/'predictor-performance';predictor.mkdir()
    ao=candidate/'validation/predictor_performance'
    cargo_home=Path(env.get('CARGO_HOME',Path.home()/'.cargo')).resolve()
    for label,revision in PREDICTOR_REVISIONS.items():
        step('predictor-source-'+label,['git','-C',ROOT,'cat-file','-e',revision+'^{commit}'])
        step('predictor-build-'+label,[python,ROOT/'validation/predictor_performance/build.py',
             '--revision',revision,'--out',predictor/label,'--cargo-home',cargo_home],
             ['dependencies','predictor-source-'+label])
        step('predictor-qualify-'+label,script('qualify_predictor.py',
             '--prefix',predictor/label/'prefix','--out',predictor/('qualify-'+label),
             '--native-repository',repo),['predictor-build-'+label])
    prediction=out/'workers/performance/cheby-native-1-2/t2pred.dat'
    # An unrelated worker failure cannot hide this case's successful bound
    # product; missing/failed generation never substitutes the 4x2 ABI fixture.
    step('predictor-workload',script('predictor_workload.py','--performance',out/'workers/performance',
         '--candidate',candidate,'--out',predictor/'workload'),['candidate'])
    step('predictor-pair-verify',[python,ao/'verify.py','--before',predictor/'before/prefix',
         '--after',predictor/'after/prefix','--before-qualification',predictor/'qualify-before',
         '--after-qualification',predictor/'qualify-after','--predictor',prediction,
         '--out',predictor/'verify'],['predictor-qualify-before','predictor-qualify-after','predictor-workload','candidate'])
    step('predictor-pair-native-gate',script('predictor_performance_gate.py',
         '--verification',predictor/'verify/receipt.json','--out',predictor/'native-gate'),['predictor-pair-verify'])
    step('predictor-pair-concurrency',script('c_concurrency.py','--prefix',predictor/'after/prefix',
         '--out',predictor/'concurrency'),['predictor-build-after'])
    step('predictor-pair-consumers',[python,ao/'consumers.py','--before',predictor/'before/prefix',
         '--after',predictor/'after/prefix','--out',predictor/'consumers','--cargo-home',cargo_home],
         ['predictor-build-before','predictor-build-after','candidate'])
    for profile in ['debug','release']:
        step('predictor-pair-tests-'+profile,['cargo','test','--offline','--locked',
             '--manifest-path',predictor/'after/source/rust/Cargo.toml',
             '--target-dir',predictor/'predictor-tests',*(['--release'] if profile=='release' else []),
             '--lib','predictor','--','--test-threads=1'],['predictor-build-after'])
    # 0128 reuses the transport pair's frozen after install as its before.
    # Direct and complete C evaluations share the same bound native 8x4 file.
    evaluator=predictor/'evaluator';evaluator.mkdir()
    before=predictor/'after/prefix';after=evaluator/'after/prefix'
    step('evaluator-source-after',['git','-C',ROOT,'cat-file','-e',EVALUATOR_REVISIONS['after']+'^{commit}'])
    step('evaluator-build-after',[python,ROOT/'validation/predictor_performance/build.py',
         '--revision',EVALUATOR_REVISIONS['after'],'--out',evaluator/'after','--cargo-home',cargo_home],
         ['dependencies','evaluator-source-after'])
    step('evaluator-qualify-after',script('qualify_predictor.py','--prefix',after,
         '--out',evaluator/'qualify-after','--native-repository',repo),['evaluator-build-after'])
    step('evaluator-pair-verify',[python,ao/'verify.py','--before',before,'--after',after,
         '--before-qualification',predictor/'qualify-after','--after-qualification',evaluator/'qualify-after',
         '--predictor',prediction,'--out',evaluator/'verify'],
         ['predictor-qualify-after','evaluator-qualify-after','predictor-workload','candidate'])
    step('evaluator-pair-native-gate',script('predictor_performance_gate.py',
         '--verification',evaluator/'verify/receipt.json','--out',evaluator/'native-gate'),['evaluator-pair-verify'])
    for label,prefix,dependency in [('before',before,'predictor-build-after'),('after',after,'evaluator-build-after')]:
        step('evaluator-direct-build-'+label,[python,ao/'evaluator_build.py','--prefix',prefix,
             '--out',evaluator/(label+'-direct'),'--cargo-home',cargo_home],[dependency,'candidate'])
    step('evaluator-direct-verify',[python,ao/'evaluator_verify.py','--before',evaluator/'before-direct',
         '--after',evaluator/'after-direct','--predictor',prediction,'--out',evaluator/'direct-verify'],
         ['evaluator-direct-build-before','evaluator-direct-build-after','predictor-workload','candidate'])
    step('evaluator-pair-concurrency',script('c_concurrency.py','--prefix',after,
         '--out',evaluator/'concurrency'),['evaluator-build-after'])
    step('evaluator-pair-consumers',[python,ao/'consumers.py','--before',before,'--after',after,
         '--out',evaluator/'consumers','--cargo-home',cargo_home],['predictor-build-after','evaluator-build-after','candidate'])
    for profile in ['debug','release']:
        step('evaluator-pair-tests-'+profile,['cargo','test','--offline','--locked',
             '--manifest-path',evaluator/'after/source/rust/Cargo.toml',
             '--target-dir',evaluator/'predictor-tests',*(['--release'] if profile=='release' else []),
             '--lib','predictor','--','--test-threads=1'],['evaluator-build-after'])
    # 0137 reuses the owner-reuse install, qualifier and direct clients as the
    # recurrence baseline; the combined candidate remains a separate contract.
    recurrence=predictor/'recurrence';recurrence.mkdir()
    recurrence_before=after;recurrence_after=recurrence/'after/prefix'
    step('recurrence-source-after',['git','-C',ROOT,'cat-file','-e',RECURRENCE_REVISIONS['after']+'^{commit}'])
    step('recurrence-build-after',[python,ROOT/'validation/predictor_performance/build.py',
         '--revision',RECURRENCE_REVISIONS['after'],'--out',recurrence/'after','--cargo-home',cargo_home],
         ['dependencies','recurrence-source-after'])
    step('recurrence-qualify-after',script('qualify_predictor.py','--prefix',recurrence_after,
         '--out',recurrence/'qualify-after','--native-repository',repo),['recurrence-build-after'])
    step('recurrence-pair-verify',[python,ao/'verify.py','--before',recurrence_before,'--after',recurrence_after,
         '--before-qualification',evaluator/'qualify-after','--after-qualification',recurrence/'qualify-after',
         '--predictor',prediction,'--out',recurrence/'verify'],
         ['evaluator-qualify-after','recurrence-qualify-after','predictor-workload','candidate'])
    step('recurrence-pair-native-gate',script('predictor_performance_gate.py',
         '--verification',recurrence/'verify/receipt.json','--out',recurrence/'native-gate'),['recurrence-pair-verify'])
    step('recurrence-direct-build-after',[python,ao/'evaluator_build.py','--prefix',recurrence_after,
         '--out',recurrence/'after-direct','--cargo-home',cargo_home],['recurrence-build-after','candidate'])
    step('recurrence-direct-verify',[python,ao/'evaluator_verify.py','--before',evaluator/'after-direct',
         '--after',recurrence/'after-direct','--predictor',prediction,'--out',recurrence/'direct-verify'],
         ['evaluator-direct-build-after','recurrence-direct-build-after','predictor-workload','candidate'])
    step('recurrence-pair-concurrency',script('c_concurrency.py','--prefix',recurrence_after,
         '--out',recurrence/'concurrency'),['recurrence-build-after'])
    step('recurrence-pair-consumers',[python,ao/'consumers.py','--before',recurrence_before,
         '--after',recurrence_after,'--out',recurrence/'consumers','--cargo-home',cargo_home],
         ['evaluator-build-after','recurrence-build-after','candidate'])
    for profile in ['debug','release']:
        step('recurrence-pair-tests-'+profile,['cargo','test','--offline','--locked',
             '--manifest-path',recurrence/'after/source/rust/Cargo.toml',
             '--target-dir',recurrence/'predictor-tests',*(['--release'] if profile=='release' else []),
             '--lib','predictor','--','--test-threads=1'],['recurrence-build-after'])
        if platform.system()=='Linux':
            # A successful Cargo invocation with zero matching tests is not a
            # Linux significand/exponent-range witness.
            step('recurrence-wide-range-'+profile,[python,'-c',
                 "from pathlib import Path; import sys; assert 'test cheby2d::evaluation::platform::tests::predictor_recurrence_linux_keeps_wide_inputs ... ok' in Path(sys.argv[1]).read_text().splitlines(), 'Linux wide-range test absent or failed'",
                 out/('recurrence-pair-tests-'+profile+'.stdout')],['recurrence-pair-tests-'+profile])
    step('recurrence-object',(['otool','-tvV',recurrence_after/'lib/libtempo2pred_rust.dylib']
         if platform.system()=='Darwin' else ['objdump','-Cd',recurrence_after/'lib/libtempo2pred_rust.so']),
         ['recurrence-build-after'])
    # 0142 reuses the accepted recurrence endpoint for the C coefficient view.
    coefficient=predictor/'coefficient-view';coefficient.mkdir()
    coefficient_before=recurrence_after;coefficient_after=coefficient/'after/prefix'
    step('coefficient-source-after',['git','-C',ROOT,'cat-file','-e',COEFFICIENT_REVISIONS['after']+'^{commit}'])
    step('coefficient-build-after',[python,ROOT/'validation/predictor_performance/build.py',
         '--revision',COEFFICIENT_REVISIONS['after'],'--out',coefficient/'after','--cargo-home',cargo_home],
         ['dependencies','coefficient-source-after'])
    step('coefficient-qualify-after',script('qualify_predictor.py','--prefix',coefficient_after,
         '--out',coefficient/'qualify-after','--native-repository',repo),['coefficient-build-after'])
    step('coefficient-pair-verify',[python,ao/'verify.py','--before',coefficient_before,'--after',coefficient_after,
         '--before-qualification',recurrence/'qualify-after','--after-qualification',coefficient/'qualify-after',
         '--predictor',prediction,'--out',coefficient/'verify'],
         ['recurrence-qualify-after','coefficient-qualify-after','predictor-workload','candidate'])
    step('coefficient-pair-native-gate',script('predictor_performance_gate.py',
         '--verification',coefficient/'verify/receipt.json','--out',coefficient/'native-gate'),['coefficient-pair-verify'])
    step('coefficient-witness',[python,ao/'coefficient_verify.py','--before',coefficient_before,
         '--after',coefficient_after,'--out',coefficient/'coefficient-verify'],
         ['recurrence-build-after','coefficient-build-after','candidate'])
    step('coefficient-direct-build-after',[python,ao/'evaluator_build.py','--prefix',coefficient_after,
         '--out',coefficient/'after-direct','--cargo-home',cargo_home],['coefficient-build-after','candidate'])
    step('coefficient-direct-verify',[python,ao/'evaluator_verify.py','--before',recurrence/'after-direct',
         '--after',coefficient/'after-direct','--predictor',prediction,'--out',coefficient/'direct-verify'],
         ['recurrence-direct-build-after','coefficient-direct-build-after','predictor-workload','candidate'])
    step('coefficient-pair-concurrency',script('c_concurrency.py','--prefix',coefficient_after,
         '--out',coefficient/'concurrency'),['coefficient-build-after'])
    step('coefficient-pair-consumers',[python,ao/'consumers.py','--before',coefficient_before,
         '--after',coefficient_after,'--out',coefficient/'consumers','--cargo-home',cargo_home],
         ['recurrence-build-after','coefficient-build-after','candidate'])
    for profile in ['debug','release']:
        step('coefficient-pair-tests-'+profile,['cargo','test','--offline','--locked',
             '--manifest-path',coefficient/'after/source/rust/Cargo.toml',
             '--target-dir',coefficient/'predictor-tests',*(['--release'] if profile=='release' else []),
             '--lib','predictor','--','--test-threads=1'],['coefficient-build-after'])
        if platform.system()=='Linux':
            # A successful Cargo invocation with zero matching tests is not a
            # Linux significand/exponent-range witness.
            step('coefficient-wide-range-'+profile,[python,'-c',
                 "from pathlib import Path; import sys; lines=Path(sys.argv[1]).read_text().splitlines(); required=['cheby2d::evaluation::platform::tests::predictor_recurrence_linux_keeps_wide_inputs','predictor_tests::predictor_owned_operations_preserve_rounding_and_classes','predictor_tests::predictor_native_subnormal_single_rounding']; assert all('test '+name+' ... ok' in lines for name in required), 'Linux range test absent or failed'",
                 out/('coefficient-pair-tests-'+profile+'.stdout')],['coefficient-pair-tests-'+profile])
    step('coefficient-object',(['otool','-tvV',coefficient_after/'lib/libtempo2pred_rust.dylib']
         if platform.system()=='Darwin' else ['objdump','-Cd',coefficient_after/'lib/libtempo2pred_rust.so']),
         ['coefficient-build-after'])
    # 0145 reuses the coefficient-view endpoint; midpoint/range execution is
    # required on actual Linux in both profiles, with live public borrows.
    temporary=predictor/'temporary-reuse';temporary.mkdir()
    temporary_before=coefficient_after;temporary_after=temporary/'after/prefix'
    step('temporary-source-after',['git','-C',ROOT,'cat-file','-e',TEMPORARY_REVISIONS['after']+'^{commit}'])
    step('temporary-build-after',[python,ROOT/'validation/predictor_performance/build.py',
         '--revision',TEMPORARY_REVISIONS['after'],'--out',temporary/'after','--cargo-home',cargo_home],
         ['dependencies','temporary-source-after'])
    step('temporary-qualify-after',script('qualify_predictor.py','--prefix',temporary_after,
         '--out',temporary/'qualify-after','--native-repository',repo),['temporary-build-after'])
    step('temporary-pair-verify',[python,ao/'verify.py','--before',temporary_before,'--after',temporary_after,
         '--before-qualification',coefficient/'qualify-after','--after-qualification',temporary/'qualify-after',
         '--predictor',prediction,'--out',temporary/'verify'],
         ['coefficient-qualify-after','temporary-qualify-after','predictor-workload','candidate'])
    step('temporary-pair-native-gate',script('predictor_performance_gate.py',
         '--verification',temporary/'verify/receipt.json','--out',temporary/'native-gate'),['temporary-pair-verify'])
    step('temporary-witness',[python,ao/'coefficient_verify.py','--before',temporary_before,
         '--after',temporary_after,'--out',temporary/'coefficient-verify'],
         ['coefficient-build-after','temporary-build-after','candidate'])
    step('temporary-direct-build-after',[python,ao/'evaluator_build.py','--prefix',temporary_after,
         '--out',temporary/'after-direct','--cargo-home',cargo_home],['temporary-build-after','candidate'])
    step('temporary-direct-verify',[python,ao/'evaluator_verify.py','--before',coefficient/'after-direct',
         '--after',temporary/'after-direct','--predictor',prediction,'--out',temporary/'direct-verify'],
         ['coefficient-direct-build-after','temporary-direct-build-after','predictor-workload','candidate'])
    step('temporary-pair-concurrency',script('c_concurrency.py','--prefix',temporary_after,
         '--out',temporary/'concurrency'),['temporary-build-after'])
    step('temporary-pair-consumers',[python,ao/'consumers.py','--before',temporary_before,
         '--after',temporary_after,'--out',temporary/'consumers','--cargo-home',cargo_home],
         ['coefficient-build-after','temporary-build-after','candidate'])
    for profile in ['debug','release']:
        step('temporary-pair-tests-'+profile,['cargo','test','--offline','--locked',
             '--manifest-path',temporary/'after/source/rust/Cargo.toml',
             '--target-dir',temporary/'predictor-tests',*(['--release'] if profile=='release' else []),
             '--lib','predictor','--','--test-threads=1'],['temporary-build-after'])
        if platform.system()=='Linux':
            # A successful Cargo invocation with zero matching tests is not a
            # Linux significand/exponent-range witness.
            step('temporary-wide-range-'+profile,[python,'-c',
                 "from pathlib import Path; import sys; lines=Path(sys.argv[1]).read_text().splitlines(); required=['cheby2d::evaluation::platform::tests::predictor_recurrence_linux_keeps_wide_inputs','predictor_tests::predictor_owned_operations_preserve_rounding_and_classes','predictor_tests::predictor_native_subnormal_single_rounding','predictor_tests::predictor_owned_subnormal_midpoint_keeps_first_rounding_direction']; assert all('test '+name+' ... ok' in lines for name in required), 'Linux range/midpoint test absent or failed'",
                 out/('temporary-pair-tests-'+profile+'.stdout')],['temporary-pair-tests-'+profile])
    step('temporary-object',(['otool','-tvV',temporary_after/'lib/libtempo2pred_rust.dylib']
         if platform.system()=='Darwin' else ['objdump','-Cd',temporary_after/'lib/libtempo2pred_rust.so']),
         ['temporary-build-after'])
    if platform.system()=='Linux':
        # Root owns the GUI; use its existing control, parity and native tests
        # against this export. All graphics work precedes predictor measurement.
        gui=out/'gui';gui.mkdir();(gui/'xdg-runtime').mkdir(mode=0o700)
        gui_manifest=candidate/'gui/Cargo.toml'
        gui_env={'CARGO_TARGET_DIR':str(gui/'target'),'TEMPO2':str(a2/'runtime'),
                 'XDG_RUNTIME_DIR':str(gui/'xdg-runtime')}
        gui_cargo=['cargo','test','--offline','--locked','--manifest-path',gui_manifest]
        step('gui-dependencies',['cargo','fetch','--locked','--manifest-path',gui_manifest],['candidate'])
        step('gui-install',['cargo','install','--offline','--locked','--path',candidate/'gui',
             '--root',gui/'prefix','--bin','tempo2-gui'],['gui-dependencies'],extra_env=gui_env)
        step('gui-libraries',['ldd',gui/'prefix/bin/tempo2-gui'],['gui-install'])
        step('gui-tests-build',[*gui_cargo,'--lib','--bin','tempo2-gui','--no-run','--message-format=json'],
             ['gui-dependencies'],extra_env=gui_env)
        step('gui-library-tests',[*gui_cargo,'--lib','--','--include-ignored','--test-threads=1'],
             ['gui-tests-build'],extra_env=gui_env)
        step('gui-examples-build',['cargo','build','--offline','--locked','--manifest-path',gui_manifest,
             '--examples'],['gui-dependencies'],extra_env=gui_env)
        step('gui-control-render',['timeout','180s',*gui_cargo,'--bin','tempo2-gui',
             'tests::load_inspect_and_render','--','--ignored','--exact','--test-threads=1'],
             ['gui-tests-build'],extra_env={**gui_env,'TEMPO2_GUI_TEST_OUTPUT':str(gui/'render')})
        step('gui-parity',[python,candidate/'gui/verify.py','--cli',out/'install-default/bin/tempo2-rust',
             '--viewer-export',gui/'target/debug/examples/export_timing',
             '--session-workflow',gui/'target/debug/examples/session_workflow',
             '--runtime',a2/'runtime','--out',gui/'parity'],['gui-examples-build','install-default'])
        step('gui-native',['timeout','60s','xvfb-run','-a','-s','-screen 0 1280x900x24',
             *gui_cargo,'--bin','tempo2-gui','native_test::native_x11_load_render_close',
             '--','--ignored','--exact','--test-threads=1'],['gui-tests-build'],
             extra_env={**gui_env,'TEMPO2_GUI_TEST_OUTPUT':str(gui/'native'),'LIBGL_ALWAYS_SOFTWARE':'1'})
        step('gui-evidence',script('gui_evidence.py','--out',gui/'evidence','--gui',gui,
             '--candidate',candidate),['gui-libraries','gui-library-tests','gui-control-render','gui-parity','gui-native'])
    # All comparison compilation precedes these shuffled fresh processes.
    # A supplemental native failure remains a gate failure, not a timing veto.
    step('predictor-pair-measure',[python,ao/'measure.py',
         '--before',predictor/'verify/before-evaluation','--after',predictor/'verify/after-evaluation',
         '--native',predictor/'verify/native-evaluation','--before-period',predictor/'before/prefix/bin/getPeriod-rust',
         '--after-period',predictor/'after/prefix/bin/getPeriod-rust',
         '--native-period',predictor/'qualify-before/native-getPeriod','--predictor',prediction,
         '--out',predictor/'measure','--iterations','100000','--samples','7'],['predictor-pair-verify','candidate'])
    step('evaluator-direct-measure',[python,ao/'evaluator_measure.py',
         '--before',evaluator/'before-direct/plain','--after',evaluator/'after-direct/plain',
         '--predictor',prediction,'--out',evaluator/'direct-measure','--iterations','100000','--samples','7'],
         ['evaluator-direct-verify','candidate'])
    step('evaluator-pair-measure',[python,ao/'measure.py',
         '--before',evaluator/'verify/before-evaluation','--after',evaluator/'verify/after-evaluation',
         '--native',evaluator/'verify/native-evaluation','--before-period',before/'bin/getPeriod-rust',
         '--after-period',after/'bin/getPeriod-rust','--native-period',predictor/'qualify-after/native-getPeriod',
         '--predictor',prediction,'--out',evaluator/'measure','--iterations','100000','--samples','7'],
         ['evaluator-pair-verify','candidate'])
    step('recurrence-direct-measure',[python,ao/'evaluator_measure.py',
         '--before',evaluator/'after-direct/plain','--after',recurrence/'after-direct/plain',
         '--predictor',prediction,'--out',recurrence/'direct-measure','--iterations','100000','--samples','7'],
         ['recurrence-direct-verify','candidate'])
    step('recurrence-pair-measure',[python,ao/'measure.py',
         '--before',recurrence/'verify/before-evaluation','--after',recurrence/'verify/after-evaluation',
         '--native',recurrence/'verify/native-evaluation','--before-period',recurrence_before/'bin/getPeriod-rust',
         '--after-period',recurrence_after/'bin/getPeriod-rust','--native-period',evaluator/'qualify-after/native-getPeriod',
         '--predictor',prediction,'--out',recurrence/'measure','--iterations','100000','--samples','7'],
         ['recurrence-pair-verify','candidate'])
    step('coefficient-direct-measure',[python,ao/'evaluator_measure.py',
         '--before',recurrence/'after-direct/plain','--after',coefficient/'after-direct/plain',
         '--predictor',prediction,'--out',coefficient/'direct-measure','--iterations','100000','--samples','7'],
         ['coefficient-direct-verify','candidate'])
    step('coefficient-pair-measure',[python,ao/'measure.py',
         '--before',coefficient/'verify/before-evaluation','--after',coefficient/'verify/after-evaluation',
         '--native',coefficient/'verify/native-evaluation','--before-period',coefficient_before/'bin/getPeriod-rust',
         '--after-period',coefficient_after/'bin/getPeriod-rust','--native-period',recurrence/'qualify-after/native-getPeriod',
         '--predictor',prediction,'--out',coefficient/'measure','--iterations','100000','--samples','7'],
         ['coefficient-pair-verify','coefficient-witness','candidate'])
    step('temporary-direct-measure',[python,ao/'evaluator_measure.py',
         '--before',coefficient/'after-direct/plain','--after',temporary/'after-direct/plain',
         '--predictor',prediction,'--out',temporary/'direct-measure','--iterations','100000','--samples','7'],
         ['temporary-direct-verify','candidate'])
    step('temporary-pair-measure',[python,ao/'measure.py',
         '--before',temporary/'verify/before-evaluation','--after',temporary/'verify/after-evaluation',
         '--native',temporary/'verify/native-evaluation','--before-period',temporary_before/'bin/getPeriod-rust',
         '--after-period',temporary_after/'bin/getPeriod-rust','--native-period',coefficient/'qualify-after/native-getPeriod',
         '--predictor',prediction,'--out',temporary/'measure','--iterations','100000','--samples','7'],
         ['temporary-pair-verify','temporary-witness','candidate'])
    from native_corpora import STAGES
    closure=script('evidence_closure.py','--out',out/'evidence-closure','--audit',out/'source-audit/receipt.json',
                   '--matrix',out/'receipt.json')
    if (corpora/'receipt.json').exists():
        for stage in json.loads((corpora/'receipt.json').read_text()).get('stages',{}):
            assert stage in STAGES
            closure+=['--native',stage+'='+str(corpora)]
    comparison=out/'corpus/receipt.json'
    if comparison.exists():
        record=json.loads(comparison.read_text())
        for stage in [*record.get('stages',{}),*record.get('blocked',{})]:
            closure+=['--comparison',stage+'='+str(comparison)]
    step('evidence-closure',closure,['source-audit'])
    failed=[name for name,row in records.items() if row['exit'] not in (0,None)]
    blocked=[name for name,row in records.items() if row['exit'] is None]
    pending=['Per-function/branch gaps and numerical failures remain explicit in evidence-closure and stage receipts',
             'Central acceptance of the recorded platform qualification']
    run.save({'status':'failure' if failed or blocked else 'pass', 'qualification':'incomplete' if pending else 'matrix executed; central acceptance separate',
        'candidate':args.revision,'stages':records,'failed':failed,'blocked':blocked,'pending':pending,
        'predictor_performance':{'revisions':PREDICTOR_REVISIONS,'cargo_home':str(cargo_home),
            'scope':'Frozen AO installed pair; combined-candidate gates remain separate; supplemental native failures retained'},
        'evaluator_performance':{'revisions':EVALUATOR_REVISIONS,'before_install':str(before),
            'before_qualification':str(predictor/'qualify-after'),'predictor':str(prediction),
            'scope':'0128 owner reuse; direct and complete C timings separate; supplemental native failures retained'},
        'recurrence_performance':{'revisions':RECURRENCE_REVISIONS,'before_install':str(recurrence_before),
            'before_qualification':str(evaluator/'qualify-after'),'before_direct':str(evaluator/'after-direct'),
            'predictor':str(prediction),
            'scope':'0137 recurrence; installed borrow and platform range checks; supplemental native failures retained'},
        'coefficient_performance':{'revisions':COEFFICIENT_REVISIONS,'before_install':str(coefficient_before),
            'before_qualification':str(recurrence/'qualify-after'),'before_direct':str(recurrence/'after-direct'),
            'predictor':str(prediction),
            'scope':'0142 coefficient view; exact C witness, installed borrows and platform range; inherited native failures retained'},
        'temporary_performance':{'revisions':TEMPORARY_REVISIONS,'before_install':str(temporary_before),
            'before_qualification':str(coefficient/'qualify-after'),'before_direct':str(coefficient/'after-direct'),
            'predictor':str(prediction),
            'scope':'0145 temporary reuse; coefficient/borrow contracts and actual Linux midpoint/range; inherited native failures retained'},
        'gui':{'scope':'0142 Linux X11/OpenGL and control/session checks; native image inspection remains central',
            'enabled':platform.system()=='Linux','runtime':str(a2/'runtime'),'outputs':str(out/'gui')},
        'allocation':{'revisions':ALLOCATION_REVISIONS,'scope':'Frozen allocator-only pair; combined candidate uses the ordinary matrix',
            'cargo_home':env.get('CARGO_HOME','Cargo default'),'gmp_cache':env['GMP_MPFR_SYS_CACHE'],
            'allocator_environment':{k:v for k,v in env.items() if k in ['LD_PRELOAD','DYLD_INSERT_LIBRARIES','GLIBC_TUNABLES']
                or k.startswith(('MALLOC_','Malloc'))}},
        'runner':sha(__file__),'environment':{k:env[k] for k in ['FC','NATIVE_CC','NATIVE_CXX','CALCEPH_DIR',
             'CARGO_TARGET_DIR','TEMPO2_ACCELERATE','TEMPO2_OPENBLAS','VECLIB_MAXIMUM_THREADS','OPENBLAS_NUM_THREADS'] if k in env}})
    return bool(failed or blocked or pending)


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--out',type=Path,required=True);p.add_argument('--revision',required=True)
    p.add_argument('--native-repository',type=Path);p.add_argument('--cmake',default='cmake')
    p.add_argument('--provider-source',type=Path);p.add_argument('--ephemeris',type=Path)
    raise SystemExit(qualify(p.parse_args()))
