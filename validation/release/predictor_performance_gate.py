#!/usr/bin/env python3
"""Retain AO's supplemental native failures independently of same-engine equality."""
import argparse
import json
from pathlib import Path
from common import Run, sha


def gate(args):
    run=Run(args.out);path=args.verification.resolve()
    record=json.loads(path.read_text())
    passed=record['exact_before_after'] and not record['inherited_native_failures']
    run.save({'status':'pass' if passed else 'failure','verification_receipt':{str(path):sha(path)},
        'exact_before_after':record['exact_before_after'],
        'native_failures':record['inherited_native_failures'],
        'native_finite_AF_floor_passes':record['native_finite_AF_floor_passes'],
        'scope':'Existing AO supplemental comparison only; no new tolerance or nonfinite/sign waiver'})
    return 0 if passed else 1


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--verification',type=Path,required=True);p.add_argument('--out',type=Path,required=True)
    raise SystemExit(gate(p.parse_args()))
