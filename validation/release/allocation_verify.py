#!/usr/bin/env python3
"""Run AN equivalence against staged S inputs without hiding inherited native failures."""
import argparse
import json
from pathlib import Path
import sys
from common import Run, sha


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


def native_failures(record):
    # AN's pass_same_engine status protects equality, not native fidelity.
    assert record['status']=='pass_same_engine'
    failed=[{'scope':'lifecycle',**row} for row in record['lifecycle'] if row['native_gate']!='pass']
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
    record=json.loads((result/'receipt.json').read_text());failed=native_failures(record)
    for path,digest in inputs.items():assert sha(path)==digest,('native S input changed',path)
    run.save({'status':'failure' if failed else 'pass','same_engine':'pass',
        'native_failures':failed,'inputs':inputs,'checker':sha(checker),
        'verification_receipt':sha(result/'receipt.json'),
        'scope':'AN exact state/science equality and original native lifecycle/fitting gates; inherited failures remain failures'})
    return bool(failed)


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','before','after','a2','candidate','native-corpora','native-fit']:
        p.add_argument('--'+name,type=Path,required=True)
    raise SystemExit(verify(p.parse_args()))
