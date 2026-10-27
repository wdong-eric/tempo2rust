#!/usr/bin/env python3
"""Execute unchanged AE public/family runners only inside an AL source export."""
import argparse
import json
from pathlib import Path
import sys
from common import Run, sha

if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    for name in ['out', 'candidate', 'native', 'rust']:
        p.add_argument('--' + name, type=Path, required=True)
    a = p.parse_args()
    run = Run(a.out)
    candidate = a.candidate.resolve()
    # A Git working tree is not an isolated export: refuse to route these
    # fixed-output predecessor runners into a live or accepted workspace.
    export_receipt = json.loads((candidate.parent / 'receipt.json').read_text())
    assert 'candidate' in export_receipt and not (candidate / '.git').exists()
    aliases = [(candidate / 'rust/target/release/tempo2-rust', a.rust.resolve()),
               (candidate / 'validation/general_fitting/build/native/fit', a.native.resolve())]
    for target, source in aliases:
        target.parent.mkdir(parents=True, exist_ok=True)
        target.symlink_to(source)
    failures = 0
    evidence = {}
    for script, directory in [('run_public.py', 'public'), ('check_families.py', 'families')]:
        path = candidate / 'validation/general_fitting' / script
        run.run([sys.executable, path, '--profile', 'release'], directory, cwd=candidate)
        receipt = candidate / 'validation/general_fitting/build' / directory / 'release/receipt.json'
        rows = json.loads(receipt.read_text())
        count = sum(r['failures'] if isinstance(r['failures'], int) else 1 for r in rows)
        failures += count
        evidence[directory] = {'cases': len(rows), 'failures': count, 'receipt': str(receipt),
                               'sha256': sha(receipt), 'runner_sha256': sha(path)}
    run.save({'status': 'failure' if failures else 'pass', 'failures': failures,
              'campaigns': evidence, 'native': sha(a.native), 'rust': sha(a.rust),
              'candidate_export_receipt': sha(candidate.parent / 'receipt.json')})
    raise SystemExit(bool(failures))
