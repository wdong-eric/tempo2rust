#!/usr/bin/env python3
"""0049 exact integer/zero/remainder gate, no production precision change."""
import argparse
from fractions import Fraction
import os
from pathlib import Path
from common import ROOT,Run,sha
from predictor_compare import native_number

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','a2','rust']:p.add_argument('--'+name,type=Path,required=True)
    a=p.parse_args();run=Run(a.out);out=run.out;a2=a.a2.resolve()
    run.run([os.environ.get('NATIVE_CXX','g++'),'-O2','-DHAVE_CONFIG_H','-I'+str(a2/'source'),
             ROOT/'validation/release/boundaries.cc',a2/'libtempo2-reference.a','-lquadmath','-lgfortran','-o',out/'native'],'compile')
    rows=[row.split('\t') for row in run.run([out/'native'],'native').decode().splitlines()]
    inputs=out/'inputs.tsv';inputs.write_text(''.join('\t'.join([name,kind,str(native_number(x)),str(native_number(y))])+'\n' for name,kind,x,y,z in rows))
    results=[row.split('\t') for row in run.run([a.rust.resolve(),inputs],'rust').decode().splitlines()]
    assert [r[0] for r in rows]==[r[0] for r in results]
    records=[]
    for native,rust in zip(rows,results):
        value=native_number(native[-1]);name,kind,negative,integer,exponent=rust
        if value.is_nan():equal=kind=='NaN' # 0049's named NaN-class convention
        elif value.is_infinite():equal=kind=='Infinity' and value.is_signed()==(negative=='true')
        else:
            equal=kind=='finite' and Fraction(value)==Fraction(int(integer))*Fraction(2)**int(exponent)
            if value==0 and int(integer)==0:equal=value.is_signed()==(negative=='true')
        records.append({'name':name,'native':native[-1],'rust':rust[1:],'equal':equal})
    failures=sum(not r['equal'] for r in records)
    run.save({'status':'failure' if failures else 'pass','failures':failures,'records':records,
        'inputs':{str(p):sha(p) for p in [a2/'receipt.json',a.rust,ROOT/'rust/src/tempo2_util.rs',ROOT/'validation/release/boundaries.cc']}})
    raise SystemExit(bool(failures))
