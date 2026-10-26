"""Verify UT1 initialization and bit-preserving timing/geometry before and after.

Run the ignored Rust write_ut1_evidence test before and after the producer edit,
then: python3 check_ut1.py --native build/native-v9 --before build/ut1-before
      --after build/ut1-after --out build/ut1-check
The original Rust snapshot is evidence of the preceding revision, not regenerated
from corrected code. Subsequent reproductions use its retained receipt/hash.
"""
import argparse
import json
import os
from pathlib import Path
from check import ROOT, HERE, run, sha

p = argparse.ArgumentParser()
for name in ['native', 'before', 'after', 'out']:
    p.add_argument('--' + name, type=Path, required=True)
a = p.parse_args()
native, before, after, out = (getattr(a, k).resolve() for k in ['native', 'before', 'after', 'out'])
out.mkdir(parents=True, exist_ok=False)
env = dict(os.environ, TEMPO2=str(ROOT / 'validation/reference/build/source/T2runtime'))
par = ROOT / 'validation/fitting/inputs/pks_all.par'
tim = ROOT / 'validation/fitting/inputs/pks.tim'
checks, runs = {}, []
for clock in [0, 1, 2]:
    for label, exe in [('original', 'probe-original'), ('corrected', 'probe')]:
        directory = out / str(clock) / label
        r = run([native / exe, 'ut1', par, tim, clock], directory, env)
        runs.append(dict(r, case=f'{clock}-{label}'))
        checks[f'{clock}-{label}-status'] = r['status'] == 0
    original, corrected = out / str(clock) / 'original', out / str(clock) / 'corrected'
    checks[f'{clock}-original-has-no-producer'] = all(
        line.split()[1] == '-12345' for line in (original / 'ut1.tsv').read_text().splitlines())
    checks[f'{clock}-native-science-unchanged'] = (original / 'state.tsv').read_bytes() == (corrected / 'state.tsv').read_bytes()
    checks[f'{clock}-rust-entire-observation-unchanged'] = (before / f'clock-{clock}.state').read_bytes() == (after / f'clock-{clock}.state').read_bytes()
    expected = (corrected / 'expected.tsv').read_bytes()
    checks[f'{clock}-native-complete-clock-plus-EOP'] = (corrected / 'ut1.tsv').read_bytes() == expected
    checks[f'{clock}-rust-complete-clock-plus-EOP'] = (after / f'clock-{clock}.ut1').read_bytes() == expected
receipt = {'checks': checks, 'runs': runs, 'before': {p.name: sha(p) for p in before.iterdir()},
           'after': {p.name: sha(p) for p in after.iterdir()}, 'native_identity': sha(native / 'identity.json'),
           'harness': sha(Path(__file__)), 'contract': sha(HERE / 'CONTRACT.md')}
(out / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
print(json.dumps(checks, indent=2))
raise SystemExit(int(not all(checks.values())))
