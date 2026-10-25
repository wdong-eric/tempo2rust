#!/usr/bin/env python3
"""Affected bounded adapters and AC lifecycle; original comparators/gates unchanged."""
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
OUT = HERE / 'build/compatibility'
ENV = dict(os.environ, CARGO_HOME=str(ROOT/'rust/.cargo-home'),
           GMP_MPFR_SYS_CACHE=str(ROOT/'rust/target/gmp-cache'))

def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result

def main():
    OUT.mkdir(parents=True, exist_ok=True)
    fit = ROOT/'validation/fitting'
    sys.path.insert(0, str(fit))
    comparator = module('ad_fit_comparator', fit/'compare.py')
    verifier = module('ad_m2_membership', ROOT/'rust/verify_m2.py')
    cases = json.loads((fit/'cases.json').read_text())
    runtime = ROOT/'validation/reference/build/source/T2runtime'
    commands = []
    failures = []
    for profile in ['debug', 'release']:
        dest = OUT/profile
        dest.mkdir(exist_ok=True)
        binary = ROOT/f'rust/target/{profile}'
        def run(name, argv):
            r = subprocess.run(list(map(str, argv)), cwd=ROOT, env=ENV,
                               capture_output=True)
            (dest/(name+'.log')).write_bytes(r.stdout+r.stderr)
            commands.append(dict(profile=profile, name=name,
                                 command=list(map(str, argv)), returncode=r.returncode))
            (OUT/'commands.json').write_text(json.dumps(commands, indent=2)+'\n')
            assert r.returncode == 0, (name, r.returncode, r.stderr[-1000:])
        run('m1a-export', [binary/'examples/compare_fixtures'])
        for path in sorted((ROOT/'rust/target/reference-comparison').glob('*.json')):
            run('m1a-'+path.stem, ['python3', ROOT/'rust/tests/reference/compare.py',
                                 ROOT/'rust/tests/reference/outputs'/path.name, path])
        run('m1b-export', [binary/'examples/compare_terrestrial', runtime, dest/'terrestrial'])
        for name in ['terrestrial', 'terrestrial_4ns']:
            run(name, ['python3', ROOT/'rust/tests/terrestrial/compare.py',
                       dest/'terrestrial'/(name+'.json')])
        run('resources', [binary/'examples/check_terrestrial_resources', runtime,
                          dest/'resource-scratch', binary/'tempo2-rust'])
        run('ac-lifecycle', [binary/'examples/check_input_to_bat_lifecycle', runtime,
                             dest/'ac-lifecycle'])
        run('m2-shared', ['cargo', 'test', '--manifest-path', ROOT/'rust/Cargo.toml',
                          '--locked', '--offline', *(['--release'] if profile == 'release' else []),
                          'fitting_tests::independent_source_solver_inputs_and_callbacks'])
        run('m2-export', [binary/'examples/compare_fitting'])
        for path in (ROOT/'rust/target/m2').glob('*.json'):
            if path.stem in {c['name'] for c in cases['timing']} or path.stem.startswith('shared_') or path.stem == 'callbacks':
                shutil.copyfile(path, dest/path.name)
        shared = verifier.load_shared_cases(cases, dest)
        checks = {}
        for family in ['solver', 'timing']:
            for case in cases[family]:
                if case.get('expected') == 'boundary':
                    continue
                name = case['name']
                ref, refs, actual = shared[name]
                for i, (r, a) in enumerate(zip(refs, actual)):
                    result = comparator.compare_shared(r, a, ref['mode'])
                    checks[f'shared/{name}/{i+1}'] = result.__dict__
                if family == 'timing':
                    epoch = next(line.split()[1] for line in (fit/'inputs'/case['par']).read_text().splitlines() if line.startswith('PEPOCH'))
                    result = comparator.compare_timing(ref, json.loads((dest/(name+'.json')).read_text()), epoch)
                    checks['end_to_end/'+name] = result.__dict__
        checks['callbacks'] = comparator.compare_callbacks(
            json.loads((fit/'reference/callbacks.json').read_text()),
            json.loads((dest/'callbacks.json').read_text())).__dict__
        (dest/'m2-comparison.json').write_text(json.dumps(checks, indent=2, default=str)+'\n')
        failures += [profile+'/'+name+': '+error for name, c in checks.items() for error in c['errors']]
    (OUT/'m2-open-gates.json').write_text(json.dumps(failures, indent=2)+'\n')
    print(json.dumps(failures, indent=2))
    print('M1a/M1b/resource/AC lifecycle checks passed. M2 failures above remain open.')
    # This verifier reports the real gate result; historical differences are not passes.
    return bool(failures)

if __name__ == '__main__':
    raise SystemExit(main())
