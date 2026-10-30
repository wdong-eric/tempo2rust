#!/usr/bin/env python3
"""AF workload unchanged; exact fields decoded at the executed ABI precision."""
import argparse
import json
from pathlib import Path
import sys
from common import sha
from af_compare import base,NativeChecks,load
import polyco_serialization

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','candidate','native','rust','abi-receipt']:p.add_argument('--'+name,type=Path,required=True)
    a=p.parse_args();abi=json.loads(a.abi_receipt.read_text());profile=abi['long_double_profile'].split()
    assert profile[0]=='LONG_DOUBLE' and profile[1:] in [['53','-1021','1024'],['64','-16381','16384']]
    NativeChecks.bits=int(profile[1]);sys.modules['compare']=base
    witness=polyco_serialization.native_witness(a.out.with_name(a.out.name+'-serialization'))
    path=a.candidate.resolve()/'validation/predictors/run_public.py'
    source=path.read_text();old='serialization(kind,models,parsed_models,filename)'
    assert source.count(old)==1
    a.out.parent.mkdir(parents=True,exist_ok=True)
    adapted=a.out.with_name(a.out.name+'-runner.py')
    assert not adapted.exists()
    adapted.write_text(source.replace(old,'serialization(kind,models,parsed_models,file)'))
    base.serialization=polyco_serialization.serialization
    runner=load('al_af_run',adapted);runner.ROOT=a.candidate.resolve();runner.HERE=path.parent
    sys.argv=[str(path),'--out',str(a.out),'--native',str(a.native),'--rust',str(a.rust)]
    status=runner.main()
    receipt=a.out/'receipt.json';record=json.loads(receipt.read_text())
    record['al_native_decoding']={'abi_receipt':sha(a.abi_receipt),'profile':abi['long_double_profile'],
        'comparator':sha(Path(__file__).with_name('af_compare.py')),'gate_epsilon':str(base.EPS),
        'note':'Exact fields use their native significand; numeric gates and binary64 serialization allowances remain unchanged'}
    record['source_serialization']={'observer':sha(Path(polyco_serialization.__file__)),
        'native_writer_parser_witness':witness,
        'adapted_runner':sha(adapted),'historical_runner':sha(path),
        'raw_lossless_records':polyco_serialization.raw_records,
        'contract':'polyco.C double cast -> %25.17le -> ordinary %Lf; companion %.30Le bounds pre-write wide value; ambiguous casts fail'}
    receipt.write_text(json.dumps(record,indent=2)+'\n');raise SystemExit(status)
