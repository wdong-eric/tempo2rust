#!/usr/bin/env python3
"""Preserve AN's raw gates and separately check the accepted lifecycle contract."""
import argparse
import gzip
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
from common import ROOT, Run, sha


def lifecycle_inputs(corpora):
    receipt=corpora/'receipt.json'
    if not receipt.is_file():raise ValueError('native-corpora receipt is unavailable')
    record=json.loads(receipt.read_text())
    stage=record.get('stages',{}).get('pulsar_lifecycle')
    if stage is None or stage['record_exit']!=0:raise ValueError('native S ordinary recording did not pass')
    expected=corpora/'staging/validation/pulsar_lifecycle/expected'
    if Path(stage['directory']).resolve()!=expected.parent.resolve() or not expected.is_dir():
        raise ValueError('native S expected directory does not match its recorded staging')
    return expected,{str(receipt):sha(receipt),**{str(p):sha(p) for p in sorted(expected.glob('*.json.gz'))}}


def corrected_inputs(corrected, original):
    receipt=corrected/'receipt.json'
    proof=json.loads(receipt.read_text())
    if proof['status']!='pass':raise ValueError('corrected-native lifecycle proof did not pass')
    if proof['producer']!=sha(ROOT/'validation/release/lifecycle_correction.py'):
        raise ValueError('corrected-native lifecycle producer differs from the accepted proof')
    if proof['copy_transform']['sha256']!=sha(ROOT/'validation/general_fitting/fitting_corrections.py'):
        raise ValueError('corrected-native lifecycle copy transformation differs')
    inputs={str(receipt):sha(receipt)};references={}
    for row in proof['records']:
        case=row['case'];path=corrected/'expected'/(case+'.json.gz')
        for source,digest in [(original/(case+'.json.gz'),row['original_sha256']),
                              (path,row['corrected_sha256'])]:
            if sha(source)!=digest:raise ValueError('lifecycle proof input differs: '+str(source))
            inputs[str(source)]=digest
        references[case]=dict(row,path=path)
    return references,inputs


def corrected_lifecycle(record, result, references, candidate):
    path=candidate/'validation/pulsar_lifecycle_port/run.py'
    spec=importlib.util.spec_from_file_location('allocation_lifecycle',path)
    lifecycle=importlib.util.module_from_spec(spec);spec.loader.exec_module(lifecycle)
    checks=[]
    for row in record['lifecycle']:
        case=row['case'];engine=row['engine'];gate='pass'
        try:
            reference=references[case]
            assert row['reference_sha256']==reference['original_sha256'],'original S reference identity'
            raw=gzip.decompress((result/case/(engine+'.jsonl.gz')).read_bytes())
            assert hashlib.sha256(raw).hexdigest()==row['events_sha256'],'AN lifecycle event identity'
            events=[json.loads(line) for line in raw.splitlines()]
            inp=result/case/'input.json'
            lifecycle.validate_launch(events[0],json.loads(inp.read_text())['nonce'],sha(inp))
            expected=json.loads(gzip.decompress(reference['path'].read_bytes()))
            lifecycle.compare([lifecycle.project(e) for e in events[1:]],
                              lifecycle.project_expected(expected),case+'.'+engine)
        except (AssertionError,KeyError,OSError,ValueError) as error:
            gate=str(error)
        checks.append({**row,'corrected_gate':gate,
                       'corrected_reference_sha256':references.get(case,{}).get('corrected_sha256')})
    return checks


def native_failures(record, lifecycle):
    # AN's pass_same_engine status protects equality, not native fidelity.
    assert record['status']=='pass_same_engine'
    failed=[{'scope':'lifecycle',**row} for row in lifecycle if row['corrected_gate']!='pass']
    failed += [{'scope':'fitting','engine':engine,'failures':row['failures']}
               for engine,row in record['native_fitting'].items() if row['failures']]
    return failed


def verify(a):
    run=Run(a.out)
    try:
        expected,inputs=lifecycle_inputs(a.native_corpora.resolve())
    except ValueError as error:
        run.save({'status':'blocked','prerequisite':'native S ordinary expected records','reason':str(error)})
        return 1
    checker=a.candidate.resolve()/'validation/allocation_profile/verify.py'
    result=run.out/'same-engine'
    run.run([sys.executable,checker,'--before',a.before.resolve(),'--after',a.after.resolve(),
        '--a2',a.a2.resolve(),'--lifecycle-expected',expected,'--native-fit',a.native_fit.resolve(),
        '--out',result],'same-engine',expected=None)
    if run.commands[-1]['status']:
        run.save({'status':'failure','same_engine':'failed','inputs':inputs,'checker':sha(checker)})
        return 1
    record=json.loads((result/'receipt.json').read_text())
    original=[row for row in record['lifecycle'] if row['native_gate']!='pass']
    fields={'same_engine':'pass','original_lifecycle_failures':original,'inputs':inputs,
            'checker':sha(checker),'verification_receipt':sha(result/'receipt.json')}
    try:
        references,proof_inputs=corrected_inputs(a.lifecycle_corrected.resolve(),expected)
    except (OSError,ValueError) as error:
        run.save({**fields,'status':'blocked','prerequisite':'accepted corrected-native lifecycle proof',
                  'reason':str(error),'native_failures':native_failures(record,[])})
        return 1
    inputs.update(proof_inputs)
    lifecycle=corrected_lifecycle(record,result,references,a.candidate.resolve())
    failed=native_failures(record,lifecycle)
    for path,digest in inputs.items():assert sha(path)==digest,('native S input changed',path)
    run.save({**fields,'status':'failure' if failed else 'pass',
        'native_failures':failed,'corrected_lifecycle':lifecycle,
        'lifecycle_comparator':sha(a.candidate.resolve()/'validation/pulsar_lifecycle_port/run.py'),
        'scope':'AN exact state/science equality; full accepted corrected-native lifecycle comparison; original S results retained; original fitting gates unchanged'})
    return bool(failed)


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','before','after','a2','candidate','native-corpora','native-fit','lifecycle-corrected']:
        p.add_argument('--'+name,type=Path,required=True)
    raise SystemExit(verify(p.parse_args()))
