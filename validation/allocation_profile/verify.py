#!/usr/bin/env python3
"""AN focused controls and exact scientific/state comparison; existing gates reused."""
import argparse
import gzip
import importlib.util
import json
import os
from pathlib import Path
import secrets
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'validation/release'))
from common import Run, sha
from build import cargo_environment


def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


def execute(a):
    run = Run(a.out); out = run.out
    builds = {'before': a.before.resolve(), 'after': a.after.resolve()}
    env = dict(os.environ, LC_ALL='C', RUST_TEST_THREADS='1', VECLIB_MAXIMUM_THREADS='1', OPENBLAS_NUM_THREADS='1')
    lifecycle = module('an_lifecycle', ROOT / 'validation/pulsar_lifecycle_port/run.py')
    tests = {}
    for name, build in builds.items():
        # Existing unit controls resolve the runtime next to CARGO_MANIFEST_DIR.
        # Add only a resource alias outside the frozen crate source membership.
        runtime_alias = build / 'source/tempo2/T2runtime'
        runtime_alias.parent.mkdir(exist_ok=True)
        if not runtime_alias.exists():
            runtime_alias.symlink_to(a.a2.resolve() / 'runtime', target_is_directory=True)
        assert runtime_alias.resolve() == (a.a2.resolve() / 'runtime').resolve()
        manifest = build / 'source/rust/Cargo.toml'
        build_env = dict(cargo_environment(build / 'source'), LC_ALL='C')
        raw = run.run(['cargo', 'test', '--offline', '--locked', '--release', '--manifest-path', manifest,
                       '--lib', '--no-run', '--message-format=json'], name + '-test-build', env=build_env)
        tests[name] = next(v['executable'] for l in raw.decode().splitlines() if l.startswith('{')
                           and (v := json.loads(l)).get('executable') and v['target']['name'] == 'tempo2_rust')
        run.run([tests[name], 'pulsar_lifecycle_tests::', '--test-threads=1'], name + '-lifecycle-unit', env=env)
        run.run([tests[name], 'pulsar_lifecycle_tests::lifecycle_logging_after_clock_fault', '--exact', '--ignored'],
                name + '-clock-recovery', env=env)
        run.run([build / 'bin/failure', 'failure'], name + '-allocation-failure', env=env)
    # Normal and terminal matrix boundaries plus real full-size creation, retained
    # owners, two-pulsar initialization and source destruction histories.
    cases = {f'matrix_{r}_{c}': ['matrix', str(r), str(c)] for r, c in [(1, 1), (2, 3), (3, 2), (4, 4)]}
    cases.update({'matrix_uinv': ['uinv', '3'], 'matrix_zero_rows': ['matrix', '0', '3'],
                  'matrix_zero_cols': ['matrix', '3', '0'], 'natural_1': ['natural', '1', '2'],
                  'wrapper_2': ['wrapper', '2', '-3'], 'seeded_reinitialise': ['repeat', '-4', '7'],
                  'destroy_constraints': ['destroy']})
    state_checks = []
    for case, args in cases.items():
        wd = out / case; wd.mkdir()
        token = secrets.token_hex(16)
        inp = wd / 'input.json'; inp.write_text(json.dumps({'nonce': token, 'args': args}) + '\n')
        values = {}
        native = json.loads(gzip.decompress((a.lifecycle_expected / (case + '.json.gz')).read_bytes()))
        for name in builds:
            events = wd / (name + '.jsonl')
            run.run([tests[name], 'pulsar_lifecycle_tests::lifecycle_probe', '--exact', '--ignored', '--nocapture'],
                    case + '-' + name, env=dict(env, TEMPO2_T_INPUT=str(inp), TEMPO2_T_OUTPUT=str(events),
                                               TEMPO2_T_DIAGNOSTICS=str(wd / (name + '-diagnostics.json'))))
            raw = events.read_bytes(); rows = [json.loads(l) for l in raw.splitlines()]
            lifecycle.validate_launch(rows[0], token, sha(inp))
            values[name] = [lifecycle.project(v) for v in rows[1:]]
            try:
                lifecycle.compare(values[name], lifecycle.project_expected(native))
                native_gate = 'pass'
            except AssertionError as e:
                native_gate = str(e)
            state_checks.append({'case': case, 'engine': name, 'native_gate': native_gate,
                'reference_sha256': sha(a.lifecycle_expected / (case + '.json.gz')),
                'events_sha256': sha(events), 'executable_sha256': sha(tests[name])})
            events.with_suffix('.jsonl.gz').write_bytes(gzip.compress(raw, mtime=0)); events.unlink()
        lifecycle.compare(values['before'], values['after'])
        (out / 'lifecycle.json').write_text(json.dumps(state_checks, indent=2) + '\n')
        print(case, 'exact before/after state', flush=True)

    runtime = a.a2.resolve() / 'runtime'
    fit = ROOT / 'validation/general_fitting/inputs'
    pred = ROOT / 'validation/predictors/fixtures/isolated.par'
    fit_args = ['timing', '--fit', '--runtime', runtime, '--par', fit / 'global0.par', '--tim', fit / 'multi-frequency.tim',
                '--par', fit / 'global1.par', '--tim', fit / 'multi-frequency.tim', '--covariance', fit / 'covariance.model',
                '--fit-diagnostics', 'diagnostics']
    pred_args = ['predictor', 'cheby', '--runtime', runtime, '--par', pred, '--site', 'PKS', '--start-mjd', '58000',
                 '--end-mjd', '58000.03', '--freq-start-mhz', '1000', '--freq-end-mhz', '2000', '--time-coeffs', '8',
                 '--freq-coeffs', '4', '--segment-seconds', '3600', '--output-dir', 'products']
    scientific = {}
    # Identical cwd and relative output names make all public metadata equal too;
    # no scientific fields, provenance, messages or file paths are projected out.
    for case, args in [('correlated-global', fit_args), ('cheby', pred_args)]:
        base = out / ('science-' + case); base.mkdir()
        wd = base / 'work'; wd.mkdir()
        values, files = {}, {}
        for name, build in builds.items():
            if case == 'correlated-global':
                (wd / 'diagnostics').mkdir()
            values[name] = run.run([build / 'bin/tempo2-rust', *args], case + '-science-' + name, cwd=wd,
                                  env=dict(env, TEMPO2=str(runtime)))
            files[name] = {str(p.relative_to(wd)): sha(p) for p in sorted(wd.rglob('*')) if p.is_file()}
            destination = base / name; destination.mkdir()
            for p in wd.iterdir():
                shutil.move(str(p), destination / p.name)
        assert values['before'] == values['after'], (case, 'complete JSON bytes differ')
        assert files['before'] == files['after'] and files['before'], (case, 'file membership/bytes differ')
        scientific[case] = {'complete_json_equal': True, 'product_bytes_and_membership_equal': True,
                            'json_sha256': sha(out / (case + '-science-after.stdout')), 'files': files['after']}
    # Same existing AE gate on the full two-pulsar correlated-global raw state.
    # It is intentionally unmodified; inherited failures remain recorded.
    comparison = module('an_general_compare', ROOT / 'validation/general_fitting/compare.py')
    wd = out / 'native-fit'; wd.mkdir()
    native_fit = a.native_fit.resolve()
    run.run([native_fit, wd / 'native.tsv', '2', fit / 'global0.par', fit / 'multi-frequency.tim',
             fit / 'global1.par', fit / 'multi-frequency.tim'], 'native-fit', cwd=wd,
            env=dict(env, TEMPO2=str(runtime), AE_COVARIANCE=str(fit / 'covariance.model')))
    fit_checks = {}
    for name in builds:
        result = comparison.compare(wd / 'native.tsv', out / ('correlated-global-science-' + name + '.stdout'),
                                    wd / (name + '-comparison.json'))
        fit_checks[name] = {'failures': result['failures'], 'stats': result['stats']}
    assert fit_checks['before'] == fit_checks['after'], 'native fitting outcomes changed'
    for name, build in builds.items():
        frozen = json.loads((build / 'freeze.json').read_text())
        receipt = json.loads((build / 'receipt.json').read_text())
        assert all(sha(build / 'source' / p) == h for p, h in frozen['sources'].items())
        assert all(sha(build / 'bin' / p) == h for p, h in receipt['binaries'].items())
    run.save({'status': 'pass_same_engine', 'lifecycle': state_checks, 'scientific': scientific,
              'native_fitting': fit_checks, 'native_fit_sha256': sha(native_fit),
              'checkers': {str(p): sha(p) for p in [Path(__file__), ROOT / 'validation/pulsar_lifecycle_port/run.py',
                        ROOT / 'validation/general_fitting/compare.py']},
              'scope': 'Exact same-engine state/products and inherited native gate outcomes; no milestone acceptance'})


if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    for name in ['before', 'after', 'a2', 'out', 'lifecycle-expected', 'native-fit']:
        p.add_argument('--' + name, type=Path, required=True)
    execute(p.parse_args())
