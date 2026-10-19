#!/usr/bin/env python3
"""Summarize recorded numbers using decimal arithmetic, preserving tiny errors."""
import csv
import hashlib
import json
import subprocess
from collections import defaultdict
from decimal import Decimal, localcontext
from pathlib import Path

root = Path(__file__).resolve().parent
rows=list(csv.DictReader((root/'results/numerical.tsv').open(),delimiter='\t'))
groups=defaultdict(list)
for row in rows:
    groups[row['group'],row['backend'],row['unit']].append(row)
lines=['# Numerical summary','',
       'The normalized ratio is abs(actual-expected) / tolerance. A ratio <= 1 passes; discrete outputs use exact equality.',
       '', '| Group | Backend | Count | Max absolute error | Unit | Max ratio to budget | Failed |',
       '| --- | --- | ---: | ---: | --- | ---: | ---: |']
with localcontext() as context:
    context.prec=100
    for (group,backend,unit),items in sorted(groups.items()):
        maximum=max(Decimal(row['abs_error']) for row in items)
        ratios=[Decimal(r['abs_error'])/Decimal(r['tolerance']) for r in items if Decimal(r['tolerance'])>0]
        ratio=format(max(ratios),'.5E') if ratios else 'exact'
        failed=sum(r['status']!='PASS' for r in items)
        lines.append(f'| {group} | {backend} | {len(items)} | {maximum:.5E} | {unit} | {ratio} | {failed} |')
lines += ['', '# Observed qd boundary failures', '', '| Group / case | Actual | Expected | Absolute error | Unit |', '| --- | ---: | ---: | ---: | --- |']
for r in rows:
    if r['backend']=='qd_assisted' and r['status']=='FAIL':
        lines.append(f"| {r['group']}: {r['case']} | {Decimal(r['actual']):.24G} | {Decimal(r['expected']):.24G} | {Decimal(r['abs_error']):.5E} | {r['unit']} |")
(root/'results/summary.md').write_text('\n'.join(lines)+'\n')
print(f'{len(rows)} rows, {sum(r["status"]!="PASS" for r in rows)} failed candidate/control gates')

source=root.parent.parent/'tempo2'
commit=subprocess.check_output(['git','-C',str(source),'rev-parse','HEAD'],text=True).strip()
assert commit=='fda6ad4a9753ad7dc5d291406f2f310a9161423f', 'reference commit changed'
paths=[source/'tests/test_data'/name for name in ['test1.tim','test2.tim','test1.par']]
paths += [root/name for name in ['Cargo.toml','Cargo.lock','src/main.rs','run.py','summarize.py',
           'snapshot_reference.py','BUDGET.md','OPERATION_SUPPORT.md','README.md','REPORT.md']]
paths += list((root/'probes').glob('*.rs'))
paths += [root/'probes/quadmath'/name for name in ['Cargo.toml','Cargo.lock','src/main.rs']]
for pattern in ['qd-0.8.0/src/lib.rs','rug-1.30.0/src/float/big.rs','rug-1.30.0/src/float/arith.rs',
                'rug-1.30.0/src/ext/xmpfr.rs','gmp-mpfr-sys-1.7.1/build.rs',
                'f128_internal-0.2.2/src/f128_t.rs','f128_internal-0.2.2/src/f128_impl.rs','f128_internal-0.2.2/src/f128.c']:
    paths += list((root/'.cargo-home/registry/src').glob('*/'+pattern))
metadata={'source_commit':commit,'files':{str(p.relative_to(root)) if p.is_relative_to(root) else str(p):
          hashlib.sha256(p.read_bytes()).hexdigest() for p in paths if p.exists()}}
(root/'results/artifact_hashes.json').write_text(json.dumps(metadata,indent=2)+'\n')
