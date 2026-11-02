#!/usr/bin/env python3
"""0127 bounded exact targets, with every original 0049 comparison retained."""
import argparse
from fractions import Fraction
import os
from pathlib import Path
from common import ROOT,Run,sha
from predictor_compare import native_number
from discrete_contract import utility_target

NAMES=['half_below','negative_half_inside','half','negative_half','half_above',
       'one_half','negative_one_half','integer_limit','mod_integer','mod_negative_exact',
       'mod_negative_zero_divisor','mod_decimal','mod_rounded_quotient',
       'mod_negative_quotient','mod_infinite_divisor','mod_zero_infinite_divisor']


def comparison(rows,results):
    assert [r[0] for r in rows]==[r[0] for r in results]==NAMES,'utility case membership/order'
    records=[]
    for native,rust in zip(rows,results,strict=True):
        value=native_number(native[-1]);name,kind,negative,integer,exponent=rust
        # MPFR's zero exponent is a sentinel, not a billion-bit denominator.
        actual=Fraction(int(integer))*Fraction(2)**int(exponent) if int(integer) else Fraction(0)
        if value.is_nan():equal=kind=='NaN' # Existing 0049 NaN-class convention.
        elif value.is_infinite():equal=kind=='Infinity' and value.is_signed()==(negative=='true')
        else:
            equal=kind=='finite' and Fraction(value)==actual
            if value==0 and int(integer)==0:equal=value.is_signed()==(negative=='true')
        target=utility_target(*native[:4])
        if target is not None:
            original={'half_below':1,'negative_half_inside':-1,
                      'mod_rounded_quotient':0,'mod_negative_quotient':0}[name]
            assert value.is_finite() and Fraction(value)==original and value.is_signed()==(original<0),'bound native witness'
        valid=negative in ['true','false'] and (kind!='finite' or not actual or (actual<0)==(negative=='true'))
        accepted=valid and (equal if target is None else (kind,negative=='true',actual)==target)
        records.append({'name':name,'native':native[-1],'rust':rust[1:],'equal':equal,
                        'contract_equal':accepted,'target':None if target is None else
                        {'class':target[0],'negative':target[1],'exact_value':str(target[2])}})
    return {'status':'pass' if all(r['contract_equal'] for r in records) else 'failure',
            'failures':sum(not r['contract_equal'] for r in records),
            'original_failures':sum(not r['equal'] for r in records),'records':records,
            'authority':'0127; four exact Rug128 targets, twelve unchanged native gates; no timing-path correction claimed'}

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','a2','rust']:p.add_argument('--'+name,type=Path,required=True)
    a=p.parse_args();run=Run(a.out);out=run.out;a2=a.a2.resolve()
    run.run([os.environ.get('NATIVE_CXX','g++'),'-O2','-DHAVE_CONFIG_H','-I'+str(a2/'source'),
             ROOT/'validation/release/boundaries.cc',a2/'libtempo2-reference.a','-lquadmath','-lgfortran','-o',out/'native'],'compile')
    rows=[row.split('\t') for row in run.run([out/'native'],'native').decode().splitlines()]
    inputs=out/'inputs.tsv';inputs.write_text(''.join('\t'.join([name,kind,str(native_number(x)),str(native_number(y))])+'\n' for name,kind,x,y,z in rows))
    results=[row.split('\t') for row in run.run([a.rust.resolve(),inputs],'rust').decode().splitlines()]
    report=comparison(rows,results)
    run.save({**report,'comparator':sha(__file__),'targets':sha(ROOT/'validation/release/discrete_contract.py'),
        'inputs':{str(p):sha(p) for p in [a2/'receipt.json',a.rust,ROOT/'rust/src/tempo2_util.rs',ROOT/'validation/release/boundaries.cc']}})
    raise SystemExit(bool(report['failures']))
