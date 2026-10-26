"""Review AG's frozen exact min/max contract against its actual toolkit export.

This replays the comparator; it does not execute or validate a timing/RNG engine.
Every invocation preserves the comparator and input bytes before mutation.
"""
from pathlib import Path
import argparse
import hashlib
import importlib.util
import json
import math
import shutil
import struct
import tempfile

root = Path('/Users/Eric.Dong/Downloads/tempo2rust')
parser = argparse.ArgumentParser()
parser.add_argument('--native', type=Path, default=root/'validation/analysis_tools/build/debug/native-toolkit.tsv')
parser.add_argument('--rust', type=Path, default=root/'validation/analysis_tools/build/debug/toolkit.tsv')
parser.add_argument('--comparator', type=Path, default=root/'validation/analysis_tools/compare.py')
args = parser.parse_args()
out = Path(tempfile.mkdtemp(prefix='tempo2-ag-comparator-review-'))
Path('/tmp/tempo2-ag-comparator-review-current').write_text(str(out)+'\n')
for source, name in [(args.native, 'native.tsv'), (args.rust, 'rust.tsv'), (args.comparator, 'compare.py'), (root/'validation/analysis_tools/CONTRACT.md', 'CONTRACT.md'), (Path(__file__), 'central-ag-comparator-review.py')]:
    shutil.copy2(source, out/name)
spec = importlib.util.spec_from_file_location('central_ag_frozen_compare', out/'compare.py')
compare = importlib.util.module_from_spec(spec)
spec.loader.exec_module(compare)
control = compare.compare(out/'native.tsv', out/'rust.tsv')
assert not control['failures'], control
(out/'control.json').write_text(json.dumps(control, indent=2)+'\n')
rows = [line.split('\t') for line in (out/'rust.tsv').read_text().splitlines()]
records = []
for key, precision in [('stat.min', 64), ('stat.max', 64), ('float.min', 32), ('float.max', 32)]:
    matches = [i for i, row in enumerate(rows) if row[0] == key]
    assert len(matches) == 1, (key, matches)
    index = matches[0]
    value = float(rows[index][2])
    if precision == 64:
        changed = math.nextafter(value, math.inf)
    else:
        bits = struct.unpack('I', struct.pack('f', value))[0]
        changed = struct.unpack('f', struct.pack('I', bits + (1 if value >= 0 else -1)))[0]
    assert changed != value and math.isfinite(changed)
    mutant = [row.copy() for row in rows]
    mutant[index][2] = format(changed, '.17e')
    path = out/(key+'.tsv')
    path.write_text(''.join('\t'.join(row)+'\n' for row in mutant))
    result = compare.compare(out/'native.tsv', path)
    (out/(key+'.comparison.json')).write_text(json.dumps(result, indent=2)+'\n')
    records.append({'field': key, 'precision_bits': precision, 'original': rows[index][2], 'mutated': mutant[index][2], 'changed_rows': 1, 'checks': result['checks'], 'failures': len(result['failures']), 'falsely_accepted': not result['failures']})
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
receipt = {'scope': 'Frozen observer replay of actual preliminary toolkit exports; exactly one representable step changed in each contract-exact min/max field. No production change or engine rerun.', 'source_paths': {'native': str(args.native), 'rust': str(args.rust), 'comparator': str(args.comparator)}, 'control_checks': control['checks'], 'control_failures': len(control['failures']), 'mutations': records, 'files': {str(p.relative_to(out)): sha(p) for p in sorted(out.rglob('*')) if p.is_file() and '__pycache__' not in p.parts}}
(out/'receipt.json').write_text(json.dumps(receipt, indent=2)+'\n')
print(json.dumps({'evidence': str(out), 'receipt_sha256': sha(out/'receipt.json'), 'comparator_sha256': sha(out/'compare.py'), 'control_checks': control['checks'], 'mutations': records}, indent=2))
raise SystemExit(int(any(r['falsely_accepted'] for r in records)))
