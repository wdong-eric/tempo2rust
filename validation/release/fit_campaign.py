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
    p.add_argument('--public-case', help='Run one existing public case; omit the family campaign')
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
    campaigns=[('run_public.py', 'public')] if a.public_case else [('run_public.py', 'public'), ('check_families.py', 'families')]
    for script, directory in campaigns:
        path = candidate / 'validation/general_fitting' / script
        run.run([sys.executable, path, '--profile', 'release', *([a.public_case] if a.public_case else [])], directory, cwd=candidate)
        filename='receipt-'+a.public_case+'.json' if a.public_case else 'receipt.json'
        receipt = candidate / 'validation/general_fitting/build' / directory / 'release' / filename
        rows = json.loads(receipt.read_text())
        count = sum(r['failures'] if isinstance(r['failures'], int) else 1 for r in rows)
        failures += count
        evidence[directory] = {'cases': len(rows), 'failures': count, 'receipt': str(receipt),
                               'sha256': sha(receipt), 'runner_sha256': sha(path)}
    run.save({'status': 'failure' if failures else 'pass', 'failures': failures,
              'selection': a.public_case or 'complete public and family campaigns',
              'campaigns': evidence, 'native': sha(a.native), 'rust': sha(a.rust),
              'candidate_export_receipt': sha(candidate.parent / 'receipt.json')})
    raise SystemExit(bool(failures))
