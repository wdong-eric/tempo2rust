#!/usr/bin/env python3
"""Replay AB original and supplemental full-state histories in owned staging."""
import argparse
from concurrent.futures import ThreadPoolExecutor
import gzip
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
AB = ROOT / 'validation/propagation'
BUILD = HERE / 'build/propagation'


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--release', action='store_true')
    parser.add_argument('--case', action='append')
    parser.add_argument('--corrected', action='store_true', help='check only separately proved 0053 corrections')
    args = parser.parse_args()
    build = HERE / ('build/propagation-corrected' if args.corrected else 'build/propagation')
    build.mkdir(parents=True, exist_ok=True)
    paths = [AB / 'outputs/reference.json.gz', AB / 'outputs/supplemental/reference.json.gz']
    references = sum((json.loads(gzip.decompress(p.read_bytes())) for p in paths), [])
    catalogs = json.loads((AB / 'outputs/frozen/cases.json').read_text()) + json.loads((AB / 'outputs/supplemental/cases.json').read_text())
    if args.corrected:
        proof = json.loads((build / 'native-proof.json').read_text())
        paths = [build / 'reference.json.gz']
        assert hashlib.sha256(paths[0].read_bytes()).hexdigest() == proof['reference_sha256']
        references = json.load(gzip.open(paths[0], 'rt'))
        selected_ids = {c['id'] for c in references}
        catalogs = [c for c in catalogs if c['id'] in selected_ids]
    selected = []
    observer_inputs = {}

    for native, case in zip(references, catalogs, strict=True):
        assert native['id'] == case['id']
        if args.case and case['id'] not in args.case:
            continue
        work = build / case['id']
        work.mkdir(exist_ok=True)
        for name, content in case['files'].items():
            path = work / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(content)
        original_root = str(AB / 'build/cases' / case['id'])
        # Observer adds defined exit snapshots. Every shared ordinary event
        # must agree exactly before using the observer-only terminal state.
        if native['status'] == 1:
            observed = AB / 'outputs/observer_raw' / (case['id'] + '.jsonl.gz')
            raw = gzip.decompress(observed.read_bytes()).decode()
            raw = raw.replace(str(AB / 'build/observer/source'), '{source}').replace(str(AB / 'build/cases'), '{cases}').replace(str(AB), '{propagation}')
            events = [json.loads(line) for line in raw.splitlines()[1:]]
            assert len(events) == len(native['events']) + 1
            for ordinary, instrumented in zip(native['events'], events):
                assert {k:v for k,v in ordinary.items() if k != 'private'} == {k:v for k,v in instrumented.items() if k != 'private'}, case['id']
            assert events[-1]['phase'] == 'exit'
            native['events'].append(events[-1])
            observer_inputs[str(observed)] = hashlib.sha256(observed.read_bytes()).hexdigest()
        # Keep actual oracle bytes and identify the one exact caller path mapping.
        native['work'] = str(work)
        native['original_root'] = original_root
        native['actions'] = [[str(a).replace('{case}', str(work)) for a in row] for row in case['actions']]
        selected.append(native)
    path = build / 'cases.json'
    path.write_text(json.dumps(selected) + '\n')
    env = {**os.environ, 'CARGO_HOME': str(ROOT / 'rust/.cargo-home'),
           'GMP_MPFR_SYS_CACHE': str(ROOT / 'rust/target/gmp-cache'),
           'AC_PROPAGATION_CASES': str(path)}
    profile = 'release' if args.release else 'debug'
    command = ['cargo', 'test', '--manifest-path', str(ROOT / 'rust/Cargo.toml'), '--locked', '--offline',
               *(['--release'] if args.release else []), '--lib', '--no-run', '--message-format=json']
    compiled = subprocess.run(command, env=env, capture_output=True, text=True, check=True)
    artifacts = [json.loads(line) for line in compiled.stdout.splitlines()]
    executable, = [a['executable'] for a in artifacts if a.get('reason') == 'compiler-artifact'
                   and a.get('executable') and a.get('profile', {}).get('test')]
    # Cargo can rebuild its target while these long comparisons run. Freeze the
    # actual artifact in owned staging before launching any verification process.
    staged = build / ('probe-' + profile)
    shutil.copy2(executable, staged)
    executable = str(staged)
    executable_sha = hashlib.sha256(Path(executable).read_bytes()).hexdigest()
    # Every case resets its process session and owns a distinct fixture directory.
    # Keep each complete action history together; only independent cases run in
    # parallel. The identical debug/release test binary performs every comparison.
    groups = [selected[i::4] for i in range(min(4, len(selected)))]
    def run_group(item):
        index, cases = item
        work = build / (profile + '-batch-' + str(index))
        work.mkdir(exist_ok=True)
        case_path = work / 'cases.json'
        case_path.write_text(json.dumps(cases) + '\n')
        test = [executable, 'propagation_corpus_tests::ab_histories', '--ignored', '--exact',
                '--nocapture', '--test-threads=1']
        with (work / 'run.log').open('w') as log:
            result = subprocess.run(test, env={**env, 'AC_PROPAGATION_CASES': str(case_path)},
                                    stdout=log, stderr=subprocess.STDOUT)
        completed = [json.loads(name) for name in re.findall(r'AB (".*?"): \d+ state differences;', (work / 'run.log').read_text())]
        assert completed == [c['id'] for c in cases], (index, 'incomplete case history', result.returncode)
        differences = json.loads((work / 'differences.json').read_text())
        assert (result.returncode == 0) == (len(differences) == 0), (index, result.returncode)
        print(profile, 'batch', index, len(cases), 'cases;', len(differences), 'differences', flush=True)
        return ({'cases': completed, 'command': test, 'returncode': result.returncode,
                 'input_sha256': hashlib.sha256(case_path.read_bytes()).hexdigest(),
                 'log_sha256': hashlib.sha256((work / 'run.log').read_bytes()).hexdigest()}, differences)
    with ThreadPoolExecutor(max_workers=4) as executor:
        results = list(executor.map(run_group, enumerate(groups)))
    order = {case['id']: i for i, case in enumerate(selected)}
    differences = sorted([d for _, values in results for d in values], key=lambda d: order[d['case']])
    (build / 'differences.json').write_text(json.dumps(differences, indent=2) + '\n')
    (build / f'differences-{profile}.json').write_text(json.dumps(differences, indent=2) + '\n')
    assert hashlib.sha256(Path(executable).read_bytes()).hexdigest() == executable_sha
    returncode = next((r['returncode'] for r, _ in results if r['returncode']), 0)
    (build / f'receipt-{profile}.json').write_text(json.dumps({
        'reference_sha256': {str(p): hashlib.sha256(p.read_bytes()).hexdigest() for p in paths},
        'observer_sha256': observer_inputs,
        'corrected_native': args.corrected, 'cases': len(selected), 'profile': profile,
        'build_command': command, 'executable_sha256': executable_sha,
        'batches': [r for r, _ in results], 'differences': len(differences), 'returncode': returncode,
    }, indent=2) + '\n')
    raise SystemExit(returncode)


if __name__ == '__main__':
    main()
