#!/usr/bin/env python3
"""0109 source spin assignments/literals, neutral observer and f64 counterfactual."""
import argparse
from decimal import Decimal
import difflib
import math
import os
from pathlib import Path
import re
from common import ROOT, Run, sha
from af_compare import round_binary


def observed(source):
    marker='bindex = 0;//psr[ipsr].param[param_brake].val[0];'
    assert source.count(marker)==1
    trace=''.join(f'ordinary("trace_{name}",{name});' for name in ['t2','arg3','arg4','f0','f1','bindex'])
    source=source.replace(marker,marker+'\n'+trace)
    marker='else if (k>9) scale=1.0e23L;'
    assert source.count(marker)==1
    return source.replace(marker,marker+'\nwide("trace_scale",scale);')


def table(path):
    rows={}
    for line in path.read_text().splitlines():
        case,key,value=line.split('\t');assert (case,key) not in rows
        rows[case,key]=Decimal(value)
    return rows


def equal(left,right,bits):
    if left.is_finite() and right.is_finite() and left!=0 and right!=0:
        return round_binary(left,bits)==round_binary(right,bits)
    return (left.is_nan(),left.is_infinite(),left.is_zero(),left.is_signed())==(
        right.is_nan(),right.is_infinite(),right.is_zero(),right.is_signed()) and (left==right or left.is_nan())


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','a2','candidate','target']:p.add_argument('--'+name,type=Path,required=True)
    a=p.parse_args();run=Run(a.out);a2=a.a2.resolve();candidate=a.candidate.resolve()
    source=a2/'source/t2fit_stdFitFuncs.C';probe=ROOT/'validation/release/spin_types.cc'
    text=source.read_text();begin=text.index('double t2FitFunc_stdFreq(');end=text.index('double t2FitFunc_binaryModels(')
    functions=text[begin:end].replace('t2FitFunc_stdFreq','al_spin').replace('t2UpdateFunc_stdFreq','al_update')
    instrumented=observed(functions)
    # Both declarations AND ordinary literals are changed in this diagnostic;
    # powq and timing-wide casts remain intact. Never use it as a native golden.
    narrow=instrumented.replace('long double','double')
    narrow=re.sub(r'(?<=[0-9.])L\b','',narrow)
    cases=run.out/'cases.tsv'
    xs=[math.nextafter(-1234.56789,-math.inf),-1234.56789,math.nextafter(-1234.56789,math.inf),
        math.nextafter(1234.56789,-math.inf),1234.56789,math.nextafter(1234.56789,math.inf),-0.0,0.0]
    cases.write_text(''.join(f'x{i}-b{brake}\t{x!r}\t{brake}\n' for i,x in enumerate(xs) for brake in [0,1]))
    products={};identities={}
    compiler=os.environ.get('NATIVE_CXX','g++')
    run.run([compiler,'--version'],'compiler')
    for name,body in [('original',None),('observed',instrumented),('f64-ordinary',narrow)]:
        work=run.out/name;work.mkdir();exe=work/'probe'
        if body is not None:
            (work/'spin_functions.inc').write_text('#line 25 "'+str(source)+'"\n'+body)
            (work/'source.patch').write_text(''.join(difflib.unified_diff(functions.splitlines(True),body.splitlines(True),fromfile='original/spin',tofile=name+'/spin')))
        run.run([compiler,'-std=gnu++17','-O2','-DHAVE_CONFIG_H','-I'+str(a2/'source'),
            '-I'+str(ROOT/'validation/release/a2'),'-I'+str(work),*(['-DAL_SPIN_COPY'] if body else []),
            probe,a2/'libtempo2-reference.a','-lquadmath','-lgfortran','-o',exe],name+'-build')
        path=work/'results.tsv';path.write_bytes(run.run([exe,cases],name))
        products[name]=table(path);identities[name]={'executable':sha(exe),'output':sha(path)}
    native=products['original'];observer=products['observed'];narrow=products['f64-ordinary']
    original_bytes=(run.out/'original/results.tsv').read_bytes()
    observer_bytes=b''.join(line for line in (run.out/'observed/results.tsv').read_bytes().splitlines(keepends=True) if b'\ttrace_' not in line)
    assert original_bytes==observer_bytes,'spin observer changed actual native returns/updates'
    trace={key:value for key,value in observer.items() if key[1].startswith('trace_')}
    assert trace.keys()=={key for key in narrow if key[1].startswith('trace_')}
    run.run(['cargo','test','--offline','--locked','--release','--manifest-path',candidate/'rust/Cargo.toml',
        '--target-dir',a.target.resolve(),'--lib','spin_profile_tests::source_profile','--','--ignored','--exact'],
        'rust',env=dict(os.environ,AL_SPIN_CASES=str(cases),AL_SPIN_OUTPUT=str(run.out/'rust.tsv')))
    rust=table(run.out/'rust.tsv');assert native.keys()==rust.keys()
    assert native['profile','ordinary_bits']==rust['profile','ordinary_bits']
    assert native['profile','timing_bits']==113 and rust['profile','timing_bits']==128
    rows=[]
    for (case,key),value in native.items():
        if case=='profile':continue
        bits=53 if key in ['callback','bounded'] else 113
        rows.append({'case':case,'field':key,'native':str(value),'rust':str(rust[case,key]),
            'f64_ordinary':str(narrow[case,key]),'comparison_bits':bits,
            'native_equal':equal(value,rust[case,key],bits),
            'f64_equal':equal(narrow[case,key],rust[case,key],bits)})
    first={}
    for (case,key),value in trace.items():
        if value!=narrow[case,key] and case not in first:
            first[case]={'field':key,'native':str(value),'f64_ordinary':str(narrow[case,key])}
    failed=[row for row in rows if row['field'] in ['callback','bounded'] and not row['native_equal']]
    wide_differences=[row for row in rows if row['field'] in ['value','error'] and not row['native_equal']]
    run.save({'status':'failure' if failed else 'pass','observer_neutral':True,'rows':rows,'failed':failed,
        'first_assignment_difference':first,'products':identities,'wide_differences':wide_differences,
        'scope':'Native F0-F13/fallback/brake, signed x, update 9/10; source-type diagnosis only. Wide update mapping is diagnostic, not a new scientific gate.',
        'counterfactual':'ordinary declarations and L literals become double; timing-wide operations unchanged',
        'inputs':{str(path):sha(path) for path in [source,probe,cases,a2/'receipt.json',candidate/'rust/src/std_freq.rs',
            candidate/'rust/src/t2fit_stdFitFuncs.rs',candidate/'rust/src/spin_profile_tests.rs',Path(__file__)]}})
    print(len(rows),'spin observations;',len(failed),'callback differences;',len(wide_differences),'wide update mapping differences (diagnostic only)',flush=True)
    raise SystemExit(bool(failed))
