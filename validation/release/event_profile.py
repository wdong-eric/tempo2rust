#!/usr/bin/env python3
"""Native expression types, ordinary transcendentals, and all eight callbacks."""
import argparse
from decimal import Decimal as D
import json
import os
from pathlib import Path
import platform
import tarfile
from common import ROOT,Run,sha
from af_compare import round_binary
from spin_profile import equal as binary_equal


def machine_code(run, a2, binaries):
    """Bind the inspected callback object to the archive used by this probe."""
    reference=json.loads((a2/'receipt.json').read_text())
    library=a2/'libtempo2-reference.a';source=a2/'source/t2fit_glitch.C'
    assert sha(library)==reference['library'], 'event reference archive changed'
    assert sha(source)==reference['source']['t2fit_glitch.C'], 'event callback source changed'
    callback=a2/'t2fit_glitch.C.o'
    run.run(['ar','p',library,callback.name],'native-callback-archive-member')
    member=run.out/'native-callback-archive-member.stdout'
    assert sha(callback)==sha(member), 'event callback object differs from linked archive'
    compile_command=next(c for c in reference['commands'] if c['label']=='compile-t2fit_glitch.C')
    assert compile_command['status']==0
    argv=compile_command['argv']
    assert Path(argv[argv.index('-c')+1])==source and Path(argv[argv.index('-o')+1])==callback
    binaries={**binaries,'native-callback.o':(callback,sha(member))}
    products={};bundle=run.out/'machine-code.tar.gz'
    with tarfile.open(bundle,'w:gz') as archive:
        for name,(path,expected) in binaries.items():
            assert sha(path)==expected, name+' changed since execution/build'
            command=(['otool','-tvV',path] if platform.system()=='Darwin' else ['objdump','-drwC',path])
            dump=run.out/(name+'.s')
            dump.write_bytes(run.run(command,name+'-disassembly'))
            archive.add(path,arcname=name,recursive=False)
            assert sha(path)==expected, name+' changed during inspection'
            products[name]={'path':str(path),'sha256':expected,'bytes':path.stat().st_size,
                            'disassembly':str(dump),'disassembly_sha256':sha(dump)}
    return {'bundle':str(bundle),'bundle_sha256':sha(bundle),'products':products,
            'a2_receipt_sha256':sha(a2/'receipt.json'),'archive_sha256':sha(library),
            'callback_source_sha256':sha(source),'compile_command':compile_command,
            'scope':'Actual executed binaries and linked callback object; inspection does not change arithmetic or acceptance.'}


def comparison(native, rust):
    def table(path):
        result={}
        for line in path.read_text().splitlines():
            case,key,value=line.split('\t');assert (case,key) not in result
            result[case,key]=D(value)
        return result
    expected,actual=table(native),table(rust)
    assert expected.keys()==actual.keys()
    assert expected['profile','ordinary_bits']==actual['profile','ordinary_bits']
    assert expected['profile','timing_bits']==113 and actual['profile','timing_bits']==128
    bits=int(expected['profile','ordinary_bits']);rows=[]
    for (case,key),value in expected.items():
        if case=='profile':continue
        observed=actual[case,key];precision=53 if key.startswith('callback_') else bits
        if value.is_finite() and observed.is_finite() and value!=0 and observed!=0:
            equal=round_binary(value,precision)==round_binary(observed,precision)
        else:
            equal=(value.is_nan(),value.is_infinite(),value.is_zero(),value.is_signed())==(
                observed.is_nan(),observed.is_infinite(),observed.is_zero(),observed.is_signed()) and (value==observed or value.is_nan())
        rows.append(dict(case=case,field=key,native=str(value),rust=str(observed),bits=precision,equal=equal,
                         difference=str(observed-value) if value.is_finite() and observed.is_finite() else None))
    return rows


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','a2','candidate','target']:p.add_argument('--'+name,type=Path,required=True)
    p.add_argument('--nan-trace',action='store_true',help='0119 internal NaN diagnostic; ordinary and wide operation/assignment controls')
    a=p.parse_args();run=Run(a.out);a2=a.a2.resolve();candidate=a.candidate.resolve()
    source=ROOT/('validation/release/event_nan_probe.cc' if a.nan_trace else 'validation/release/event_types.cc');cases=ROOT/'validation/release/event_cases.tsv'
    compiler=os.environ.get('NATIVE_CXX','g++')
    run.run([compiler,'--version'],'native-compiler')
    run.run([os.environ.get('CC','clang'),'--version'],'adapter-compiler')
    run.run([compiler,'-std=gnu++17','-O2','-DHAVE_CONFIG_H',
        '-I'+str(a2/'source'),'-I'+str(ROOT/'validation/release/a2'),source,
        a2/'libtempo2-reference.a','-lquadmath','-lgfortran','-o',run.out/'native'],'native-build')
    native_identity=sha(run.out/'native')
    (run.out/'native.tsv').write_bytes(run.run([run.out/'native',cases],'native'))
    command=['cargo','test','--offline','--locked','--release','--message-format=json','--manifest-path',candidate/'rust/Cargo.toml',
        '--target-dir',a.target.resolve(),'--lib','event_profile_tests::'+('nan_arithmetic_profile' if a.nan_trace else 'source_profile'),'--','--ignored','--exact']
    build=run.run(command,'rust',env=dict(os.environ,AL_EVENT_CASES=str(cases),AL_EVENT_PROFILE_OUTPUT=str(run.out/'rust.tsv')))
    messages=[json.loads(line) for line in build.decode().splitlines() if line.startswith('{')]
    executable=Path(next(m['executable'] for m in messages if m.get('reason')=='compiler-artifact'
        and m.get('profile',{}).get('test') and m.get('executable') and m['target']['name']=='tempo2_rust'))
    native_object=Path(next(m['out_dir'] for m in messages if m.get('reason')=='build-script-executed'
        and '#tempo2-rust@' in m['package_id']))/'predictor.o'
    rust_identity=sha(executable);adapter_identity=sha(native_object)
    for name,path in [('native',run.out/'native'),('rust',executable)]:
        run.run(['otool','-L',path] if platform.system()=='Darwin' else ['ldd',path],name+'-libraries')
    run.run(['nm','-u',native_object],'adapter-symbols')
    code=machine_code(run,a2,{'native-event':(run.out/'native',native_identity),
                             'rust-event-tests':(executable,rust_identity),
                             'rust-adapter.o':(native_object,adapter_identity)})
    rows=comparison(run.out/'native.tsv',run.out/'rust.tsv');failed=[r for r in rows if not r['equal']]
    callbacks={(r['case'],r['field']):r for r in rows if r['field'].startswith('callback_')}
    traced_returns=[]
    if a.nan_trace:
        # Pinned param labels: compare each independently observed expression
        # with the actual source/production callback, not just its counterpart.
        labels={'trace_gaus_amp_return':47,'trace_gaus_epoch_return':46,
                'trace_gaus_width_return':48,'trace_gaus_index_return':49,
                'trace_exp_tau_div2':44,'trace_exp_epoch_return':42}
        for row in rows:
            if row['field'] not in labels:continue
            callback=callbacks[row['case'],'callback_'+str(labels[row['field']])]
            for engine in ['native','rust']:
                assert binary_equal(D(row[engine]),D(callback[engine]),53),(engine,row)
                traced_returns.append({'case':row['case'],'field':row['field'],'engine':engine,'equal':True})
    returns=[]
    return_path=run.out/'rust.tsv.returns.tsv'
    for line in return_path.read_text().splitlines():
        case,label,before,after=line.split('\t');row=callbacks[case,'callback_'+label]
        assert binary_equal(D(after),D(row['rust']),53),'return observer changed the emitted callback'
        returns.append(dict(case=case,label=int(label),before=before,after=after,native=row['native'],
            conversion_preserved=binary_equal(D(before),D(after),53),
            native_equal=row['equal']))
    run.save({'status':'failure' if failed else 'pass','rows':rows,'failed':failed,
        'callback_return_observations':returns,'callback_return_file':sha(return_path),
        'return_observation_scope':'Actual computed ordinary returns before/after f64 conversion; the twelve literal-zero branches have no ordinary intermediate.',
        'native_executable':native_identity,'rust_executable':rust_identity,'adapter_object':adapter_identity,
        'machine_code':code,
        'native_library_observation':sha(run.out/'native-libraries.stdout'),
        'rust_library_observation':sha(run.out/'rust-libraries.stdout'),
        'adapter_symbols':sha(run.out/'adapter-symbols.stdout'),
        'nan_trace':a.nan_trace,'trace_callback_equivalence':traced_returns,
        'scope':'Exact native ordinary fields and binary64 callbacks; wide timing mapping remains binary128 versus Rug128',
        'inputs':{str(p):sha(p) for p in [source,ROOT/'validation/release/event_types.cc',cases,a2/'receipt.json',candidate/'rust/src/ordinary_long_double.rs',
            candidate/'rust/native/predictor.c',candidate/'rust/build.rs',candidate/'rust/src/predictor_number.rs',
            candidate/'rust/src/tkfit.rs',candidate/'rust/src/event_profile_tests.rs',candidate/'rust/src/t2fit_glitch.rs',Path(__file__)]}})
    print(len(rows),'source-profile comparisons;',len(failed),'differences',flush=True)
    raise SystemExit(bool(failed))
