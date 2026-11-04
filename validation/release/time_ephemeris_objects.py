#!/usr/bin/env python3
"""Retain disassembly of U's actual native and executed Rust binaries (0142)."""
import argparse
import json
from pathlib import Path
import platform
from common import Run, sha


def capture(run, native_receipt, rust_receipt):
    inputs = {}; products = {}; failures = {}
    # A bad native object must not suppress capture of a separately bound Rust
    # executable, or conversely. Scientific differences never veto inspection.
    for engine, receipt in [('native', native_receipt), ('rust', rust_receipt)]:
        try:
            record = json.loads(receipt.read_text()); inputs[str(receipt)] = sha(receipt)
            if engine == 'native':
                identity = record['profile']; executable = Path(identity['executable_path'])
            else:
                assert record['complete'] and len(record['cases']) == 97, 'incomplete U histories'
                assert record['native_receipt_sha256'] == sha(native_receipt), 'native receipt differs'
                identity = record['identity']; executable = Path(identity['executable'])
            expected = identity['executable_sha256']
            assert sha(executable) == expected, engine + ' executable differs from execution receipt'
            inputs[str(executable)] = expected
            command = (['otool', '-tvV', executable] if platform.system() == 'Darwin'
                       else ['objdump', '-Cd', executable])
            dump = run.out / (engine + '.s')
            dump.write_bytes(run.run(command, engine + '-disassembly'))
            assert sha(executable) == expected, engine + ' executable changed during inspection'
            products[engine] = {'execution_identity': identity, 'receipt_sha256': inputs[str(receipt)],
                                'disassembly': str(dump), 'disassembly_sha256': sha(dump)}
        except Exception as error:
            failures[engine] = str(error)
        run.save({'status': 'running', 'inputs': inputs, 'products': products, 'failures': failures})
    run.save({'status': 'failure' if failures else 'pass', 'inputs': inputs,
              'products': products, 'failures': failures, 'producer': sha(__file__),
              'scope': 'Actual receipt-bound U executables; instruction interpretation and scientific acceptance remain separate.'})
    return bool(failures)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ['out', 'native-receipt', 'rust-receipt']:
        parser.add_argument('--' + name, type=Path, required=True)
    args = parser.parse_args()
    raise SystemExit(capture(Run(args.out), args.native_receipt.resolve(), args.rust_receipt.resolve()))
