#!/usr/bin/env python3
"""Bind AO's 8x4 benchmark to the existing successful native AK worker case."""
import argparse
import json
from pathlib import Path
from common import Run, sha


def workload(performance, candidate):
    receipt=performance/'receipt.json';record=json.loads(receipt.read_text())
    rows=[r for r in record['records'] if (r['case'],r['engine'],r['workers'],r['sample'])==('cheby','native',1,2)]
    if len(rows)!=1 or rows[0]['exit']!=0:raise ValueError('native cheby/1/2 case did not complete successfully')
    row=rows[0];command=row['command']
    if command[command.index('-pred')+1]!='PKS 58000 58000.03 1000 2000 8 4 3600':
        raise ValueError('native predictor workload differs from the reviewed 8x4 case')
    model=command[command.index('-f')+1]
    fixture=candidate/'validation/predictors/fixtures/isolated.par'
    if record['identities'][model]!=sha(fixture):raise ValueError('isolated PAR identity differs')
    relative='cheby-native-1-2/t2pred.dat';predictor=performance/relative
    if sha(predictor)!=record['files'][relative]:raise ValueError('generated predictor identity differs')
    return {'performance_receipt':{str(receipt):sha(receipt)},'native_case':row,
            'generator':{command[0]:record['identities'][command[0]]},
            'isolated_par':{str(fixture):sha(fixture)},'predictor':{str(predictor):sha(predictor)}}


def prepare(args):
    run=Run(args.out)
    try:bound=workload(args.performance.resolve(),args.candidate.resolve())
    except (OSError,KeyError,ValueError) as error:
        run.save({'status':'blocked','prerequisite':'successful native AK 8x4 predictor workload','reason':str(error)})
        return 1
    run.save({'status':'pass',**bound,'scope':'Matched AO workload identity; no numerical or timing acceptance'})
    return 0


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['performance','candidate','out']:p.add_argument('--'+name,type=Path,required=True)
    raise SystemExit(prepare(p.parse_args()))
