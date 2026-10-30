#!/usr/bin/env python3
"""Native expression types, ordinary transcendentals, and all eight callbacks."""
import argparse
from decimal import Decimal as D
import json
import os
from pathlib import Path
from common import ROOT,Run,sha
from af_compare import round_binary


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
    a=p.parse_args();run=Run(a.out);a2=a.a2.resolve();candidate=a.candidate.resolve()
    source=ROOT/'validation/release/event_types.cc';cases=ROOT/'validation/release/event_cases.tsv'
    run.run([os.environ.get('NATIVE_CXX','g++'),'-std=gnu++17','-O2','-DHAVE_CONFIG_H',
        '-I'+str(a2/'source'),'-I'+str(ROOT/'validation/release/a2'),source,
        a2/'libtempo2-reference.a','-lquadmath','-lgfortran','-o',run.out/'native'],'native-build')
    (run.out/'native.tsv').write_bytes(run.run([run.out/'native',cases],'native'))
    command=['cargo','test','--offline','--locked','--release','--manifest-path',candidate/'rust/Cargo.toml',
        '--target-dir',a.target.resolve(),'--lib','event_profile_tests::source_profile','--','--ignored','--exact']
    run.run(command,'rust',env=dict(os.environ,AL_EVENT_CASES=str(cases),AL_EVENT_PROFILE_OUTPUT=str(run.out/'rust.tsv')))
    rows=comparison(run.out/'native.tsv',run.out/'rust.tsv');failed=[r for r in rows if not r['equal']]
    run.save({'status':'failure' if failed else 'pass','rows':rows,'failed':failed,
        'scope':'Exact native ordinary fields and binary64 callbacks; wide timing mapping remains binary128 versus Rug128',
        'inputs':{str(p):sha(p) for p in [source,cases,a2/'receipt.json',candidate/'rust/src/ordinary_long_double.rs',
            candidate/'rust/src/event_profile_tests.rs',candidate/'rust/src/t2fit_glitch.rs',Path(__file__)]}})
    print(len(rows),'source-profile comparisons;',len(failed),'differences',flush=True)
    raise SystemExit(bool(failed))
