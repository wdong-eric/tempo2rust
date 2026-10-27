#!/usr/bin/env python3
"""Run unchanged AH gates with the recorded native compilation directory mapped."""
import argparse
import importlib.util
import json
from pathlib import Path
import sys
from common import ROOT, sha

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','native','core','candidate']:p.add_argument('--'+name,type=Path,required=True)
    a=p.parse_args();native=a.native.resolve();core=a.core.resolve()
    identity=json.loads((native/'receipt.json').read_text())
    assert identity['core_receipt']==sha(core/'receipt.json')
    for name in json.loads((core/'receipt.json').read_text())['source']:
        assert sha(core/'source'/name)==sha(native/'source'/name)
    path=a.candidate.resolve()/'validation/core_output/check.py'
    spec=importlib.util.spec_from_file_location('al_ah',path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    original=m.compare
    def compare(left,right,normalize=lambda data,kind:data,diff_dir=None):
        def normalize_source(data,kind):
            if kind=='native':
                data=data.replace(b'['+str(core/'source').encode()+b'/',b'['+str(native/'source').encode()+b'/')
            return normalize(data,kind)
        return original(left,right,normalize_source,diff_dir)
    m.compare=compare
    sys.argv=[str(path),'--native',str(native),'--out',str(a.out),'--release']
    status=m.main()
    receipt=a.out/'receipt.json';record=json.loads(receipt.read_text())
    record['al_source_directory_binding']={'wrapper':sha(__file__),'compiled':str(core/'source'),
        'observer_copy':str(native/'source'),'core_receipt':sha(core/'receipt.json'),
        'meaning':'Exact known __FILE__ directory relocation before the unchanged pinned-line mapping; no numerical/text-value normalization'}
    receipt.write_text(json.dumps(record,indent=2)+'\n')
    raise SystemExit(status)
