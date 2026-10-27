#!/usr/bin/env python3
"""Verify clean-export and private-install bytes against their recorded receipts."""
import argparse
import json
from pathlib import Path
from common import Run,sha

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--out',type=Path,required=True);p.add_argument('--candidate',type=Path,required=True)
    p.add_argument('--prefix',type=Path,action='append',required=True)
    a=p.parse_args();run=Run(a.out);candidate=a.candidate.resolve()
    export=json.loads((candidate.parent/'receipt.json').read_text())
    mismatches=[];checked=0;packages=[]
    for name,digest in export['source'].items():
        path=candidate/name;checked+=1
        if not path.is_file() or sha(path)!=digest:mismatches.append(str(path))
    for prefix in a.prefix:
        prefix=prefix.resolve();receipt=prefix.parent/(prefix.name+'-build')/'install.json'
        record=json.loads(receipt.read_text())
        actual={str(p.relative_to(prefix)) for p in prefix.rglob('*') if p.is_file()}
        if actual!=set(record['files']):mismatches.append(str(prefix)+': file membership')
        for name,digest in record['files'].items():
            path=prefix/name;checked+=1
            if not path.is_file() or sha(path)!=digest:mismatches.append(str(path))
        installed=prefix/'share/tempo2-rust/rust';source_count=0
        for path in installed.rglob('*'):
            if path.is_file():
                source_count+=1;expected=candidate/'rust'/path.relative_to(installed)
                if not expected.is_file() or sha(path)!=sha(expected):mismatches.append(str(path)+': candidate source')
        required=['bin/tempo2-rust','bin/getPeriod-rust','lib/libtempo2pred_rust.a','include/tempo2pred.h',
                  'include/tempo2pred_int.h','share/tempo2-rust/rust/Cargo.lock',
                  'share/tempo2-rust/rust/LICENSE','share/tempo2-rust/rust/MT19937_LICENSE.txt',
                  'share/tempo2-rust/rust/RELEASE_QUALIFICATION_REPORT.md']
        for name in required:
            if name not in actual:mismatches.append(str(prefix/name)+': required product absent')
        sofa=list((installed/'native/sofa').glob('*.for'))
        if len(sofa)!=36:mismatches.append(str(installed)+': SOFA membership')
        packages.append({'prefix':str(prefix),'files':len(actual),'source_files':source_count,'receipt':sha(receipt)})
    run.save({'status':'failure' if mismatches else 'pass','candidate':export['candidate'],'checked_files':checked,
              'packages':packages,'mismatches':mismatches,'export_receipt':sha(candidate.parent/'receipt.json'),'auditor':sha(__file__)})
    print(checked,'files checked;',len(mismatches),'mismatches');raise SystemExit(bool(mismatches))
