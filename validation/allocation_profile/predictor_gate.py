#!/usr/bin/env python3
"""Run the existing isolated-PKS predictor gates with the released ABI adapter.

Only resource routing and the case selection differ from check_predictor_science.
The selected existing case covers Chebyshev and polyco, including cross-readers.
"""
import argparse
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'validation/release'))
from common import sha
from af_compare import base, NativeChecks, load
import polyco_serialization


if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    for name in ['out', 'a2', 'native', 'rust', 'abi-receipt']:
        p.add_argument('--' + name, type=Path, required=True)
    a = p.parse_args()
    abi = json.loads(a.abi_receipt.read_text())
    profile = abi['long_double_profile'].split()
    assert profile[0] == 'LONG_DOUBLE' and profile[1:] in [['53', '-1021', '1024'], ['64', '-16381', '16384']]
    NativeChecks.bits = int(profile[1])
    alias = ROOT / 'validation/reference/build/source/T2runtime'
    alias.parent.mkdir(parents=True, exist_ok=True)
    if not alias.exists():
        alias.symlink_to(a.a2.resolve() / 'runtime', target_is_directory=True)
    assert alias.resolve() == (a.a2.resolve() / 'runtime').resolve()
    sys.modules['compare'] = base
    witness = polyco_serialization.native_witness(a.out.with_name(a.out.name + '-serialization'))
    original = ROOT / 'validation/predictors/run_public.py'
    source = original.read_text()
    old = 'serialization(kind,models,parsed_models,filename)'
    assert source.count(old) == 1
    adapted = a.out.with_name(a.out.name + '-runner.py')
    assert not adapted.exists()
    adapted.write_text(source.replace(old, 'serialization(kind,models,parsed_models,file)'))
    base.serialization = polyco_serialization.serialization
    runner = load('an_predictor_run', adapted)
    runner.ROOT = ROOT; runner.HERE = original.parent
    sys.argv = [str(original), '--out', str(a.out), '--native', str(a.native), '--rust', str(a.rust),
                '--case', 'isolated-pks']
    status = runner.main()
    receipt = a.out / 'receipt.json'; record = json.loads(receipt.read_text())
    record['allocation_gate'] = {'selection': 'existing isolated-pks only; no gate changes',
        'status': 'failure' if status else 'pass', 'abi_receipt': sha(a.abi_receipt),
        'profile': abi['long_double_profile'], 'serialization_witness': witness,
        'sources': {str(path): sha(path) for path in [Path(__file__), original, adapted,
          ROOT / 'validation/release/check_predictor_science.py', ROOT / 'validation/release/af_compare.py',
          ROOT / 'validation/release/polyco_serialization.py']}}
    receipt.write_text(json.dumps(record, indent=2) + '\n')
    raise SystemExit(status)
