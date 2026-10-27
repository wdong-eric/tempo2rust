#!/usr/bin/env python3
"""0087 current-clock fidelity and versioned historical strict 1 ns ideal checks."""
import argparse
from decimal import Decimal as D
import importlib.util
import json
import os
from pathlib import Path
from common import ROOT, Run, sha


def check(out, a2, native, rust, source_root):
    run = Run(out)
    out = run.out
    path = source_root / 'validation/general_fitting/compare.py'
    spec = importlib.util.spec_from_file_location('al_fit_compare', path)
    comparison = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(comparison)
    par, tim = (a2 / 'source/tests/test_data' / name for name in ['test3.par', 'test3.tim'])
    records = []
    for label, runtime in [('current', a2 / 'runtime'), ('historical', a2 / 'runtime-historical')]:
        if label == 'historical':
            assert sha(runtime / 'clock/gps2utc.clk') == '30727fdfaaec0f1775543804210d057ffe62076594bb8884d8f8ccafafb676be'
        work = out / label
        work.mkdir()
        env = dict(os.environ, TEMPO2=str(runtime), AE_IDEAL='1', LC_ALL='C',
                   REFERENCE_RESOURCE_LOG=str(work / 'native-resources.tsv'))
        for key in ['AE_COVARIANCE', 'AE_NORESCALE', 'AE_WRITE_RESIDUALS', 'REFERENCE_TEST_STORAGE_PATTERN',
                    'TEMPO2_ALIAS', 'TEMPO2_CLOCK_DIR', 'TEMPO2_CLOCK_PATH']:
            env.pop(key, None)
        run.run([native, work / 'native.tsv', '1', par, tim], label + '-native', cwd=work, env=env)
        data = run.run([rust, runtime, par, tim], label + '-rust', cwd=work, env=env)
        (work / 'rust.json').write_bytes(data)
        result = json.loads(data)
        assert not result['remove_mean'] and len(result['cycles']) == 1
        native_values = {}
        for line in (work / 'native.tsv').read_text().splitlines():
            iteration, stage, pulsar, index, key, value = line.split('\t')
            native_values[int(iteration), stage, int(pulsar), int(index), key] = D(value)
        ideal = {}
        for stage in ['prefit', 'postfit']:
            rows = result['cycles'][0][stage]['pulsars'][0]['observations']
            assert len(rows) == 284 and [r['input_index'] for r in rows] == list(range(284))
            assert not result['cycles'][0][stage]['remove_mean']
            values = {'rust': [comparison.decimal(r['residual_s']) for r in rows],
                      'native': [native_values[1, stage, 0, i, 'residual'] for i in range(284)]}
            ideal[stage] = {engine: {'maximum_s': str(max(map(abs, v))),
                'failing_rows': [i for i, x in enumerate(v) if abs(x) >= D('1e-9')]}
                for engine, v in values.items()}
        fidelity = comparison.compare(work / 'native.tsv', work / 'rust.json', work / 'comparison.json')
        # The original upstream current-clock ideal failure remains a failure.
        # Record the actual exit without turning it into a successful assertion.
        run.run([a2 / 'testEndToEnd', '--gtest_filter=testEndToEnd.checkIdeal'],
                label + '-original-ideal', cwd=work, env=env, expected=None)
        resources = {}
        for row in (work / 'native-resources.tsv').read_text().splitlines():
            stage, filename, mode, opened, error, caller = row.split('\t')
            if opened == '1' and mode.startswith('r'):
                resources[filename] = sha(filename)
        records.append({'clock': label, 'ideal': ideal, 'fidelity_failures': len(fidelity['failures']),
                        'fidelity_stats': fidelity['stats'], 'unchecked': fidelity['unchecked'],
                        'resources': resources, 'comparison_sha256': sha(work / 'comparison.json')})
    failures = sum(r['fidelity_failures'] for r in records)
    historical_failures = sum(len(v['failing_rows']) for stage in records[1]['ideal'].values() for v in stage.values())
    original_historical_exit = next(c['status'] for c in run.commands if c['label'] == 'historical-original-ideal')
    run.save({'status': 'failure' if failures or historical_failures or original_historical_exit else 'pass',
              'records': records, 'fidelity_failures': failures, 'historical_ideal_failures': historical_failures,
              'inputs': {str(p): sha(p) for p in [par, tim, native, rust, path, a2 / 'receipt.json']}})
    return failures + historical_failures + bool(original_historical_exit)


if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    for name in ['out', 'a2', 'native', 'rust', 'source-root']:
        p.add_argument('--' + name, required=True, type=Path)
    a = p.parse_args()
    raise SystemExit(bool(check(a.out, a.a2.resolve(), a.native.resolve(), a.rust.resolve(), a.source_root.resolve())))
