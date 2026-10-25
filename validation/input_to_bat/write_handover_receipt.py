#!/usr/bin/env python3
"""Index completed AC checks and preserve every unresolved input/check explicitly.

This records existing executions; it does not run checks or turn an expected
nonzero result into a pass. Generated evidence stays below the owned build root.
"""
import collections
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
BUILD = ROOT / 'validation/input_to_bat/build'


def read(relative):
    return json.loads((BUILD / relative).read_text())


def identity(path):
    return {'path': str(path.relative_to(ROOT)),
            'sha256': hashlib.sha256(path.read_bytes()).hexdigest()}


components = []
for component, count, code in [('parameters', 304, 0), ('par', 121, 0), ('n', 157, 0),
                               ('preprocessing', 296, 101), ('planetary', 82, 0),
                               ('random', 15, 0), ('propagation', 324, 101),
                               ('propagation-corrected', 3, 0)]:
    for profile in ['debug', 'release']:
        path = BUILD / component / ('receipt-' + profile + '.json')
        receipt = json.loads(path.read_text())
        assert receipt['cases'] == count and receipt['returncode'] == code, path
        components.append({**identity(path), 'component': component, 'profile': profile,
                           'cases': count, 'returncode': code,
                           'outcome': 'pass' if code == 0 else 'failed; open ledger'})

public = []
for profile in ['debug', 'release']:
    paths = sorted((BUILD / 'pipeline').glob('*/comparison-' + profile + '.json'))
    assert len(paths) == 18
    for path in paths:
        assert json.loads(path.read_text())['errors'] == [], path
        public.append({**identity(path), 'case': path.parent.name, 'profile': profile})
    controls = read('pipeline/comparator-controls-' + profile + '.json')
    assert len(controls['cases']) == 18
    failures = read('lifecycle-' + profile + '/cli-failures.json')
    assert len(failures['cases']) == 3
    assert all(c['returncode'] != 0 and c['stdout_bytes'] == 0 for c in failures['cases'])
    assert 'PASS: library histories' in (BUILD / ('check_lifecycle-' + profile + '-final.log')).read_text()
    assert (BUILD / ('bounded-' + profile + '.log')).read_text().splitlines()[-2:] == ['PASS', 'PASS']
    standard = (BUILD / ('standard-' + profile + '.log')).read_text()
    assert '56 passed; 0 failed; 26 ignored' in standard
    assert '7 passed; 0 failed' in standard and '1 passed; 0 failed' in standard

assert 'Finished' in (BUILD / 'clippy-final.log').read_text()
assert len(read('parameters/qualified-rounding.json')) == 17
for component in ['parameters', 'par', 'n', 'preprocessing']:
    assert read(component + '/diagnostic-differences.json') == []

groups = {
    'simul': ('AC-P-SIMUL', 'selection::process_simultaneous; strict <60 boundary',
              'SIMUL selection and deletion', 'Diagnosed 113/128-bit discrete branch difference; 0074'),
    'units': ('AC-P-UNITS', 'units::transform_units / scale_param / xform_mjd',
              'Unit conversion and downstream parameter values/errors', 'Continuous mismatch; no P allowance'),
    'stage2': ('AC-P-STAGE2', 'preprocess::pre_process_simple2; ordered SAT shifts',
               'Preprocessed SAT', 'Continuous mismatch; no P allowance'),
    'wrapper': ('AC-P-LINK', 'preprocess::pre_process; linked-value and unit transformations',
                'Linked preprocessing values', 'Continuous mismatch; K-only 0073 does not apply'),
    'epoch': ('AC-P-EPOCH', 'update_epoch::update_epoch; spin/DM/position arithmetic',
              'Epoch-transformed parameters', 'Continuous mismatch; no P allowance'),
}
open_checks = []
for difference in read('preprocessing/differences.json'):
    case, action = difference['tag'].split(' ', 1)
    group, = [details for prefix, details in groups.items() if case.startswith(prefix)]
    key, localization, scope, disposition = group
    open_checks.append({'id': key, 'input': case, 'check': action + ' ' + difference['field'],
                        'observed': difference, 'localization': localization, 'scope': scope,
                        'disposition': disposition,
                        'owner': 'Central numerical decision; current Rust owner diagnoses/implements correction'})
counts = dict(collections.Counter(item['id'] for item in open_checks))
assert counts == {'AC-P-SIMUL': 32, 'AC-P-UNITS': 71, 'AC-P-STAGE2': 2, 'AC-P-LINK': 4, 'AC-P-EPOCH': 45}
original_ab = {}
for profile in ['debug', 'release']:
    differences = read('propagation/differences-' + profile + '.json')
    counts_ab = dict(collections.Counter(d['case'] for d in differences))
    assert counts_ab == {'bclt_tropo_0': 9, 'phase_cross_0_1': 149, 'phase_cross_1_0': 119}
    original_ab[profile] = counts_ab
    for difference in differences:
        open_checks.append({'id': 'AC-AB-ORIGINAL', 'profile': profile, 'input': difference['case'],
                            'check': {k: difference[k] for k in ['action', 'phase', 'field'] if k in difference},
                            'observed': difference, 'localization': '0053 per-pulsar troposphere/reference cleanup',
                            'scope': 'Original AB compatibility; separate corrected histories pass',
                            'owner': 'Central correction/acceptance review'})
(BUILD / 'open-checks.json').write_text(json.dumps(open_checks, indent=2) + '\n')

evidence = [p for component in ['parameters', 'par', 'n', 'preprocessing', 'planetary', 'random',
                               'propagation', 'propagation-corrected']
            for p in (BUILD / component).glob('receipt-*.json')]
evidence += [BUILD / relative for relative in [
    'pipeline/identity.json', 'pipeline/comparator-controls-debug.json', 'pipeline/comparator-controls-release.json',
    'telescope_boundaries/receipt.json', 'jbo-corrected/receipt.json', 'propagation-corrected/native-proof.json',
    'preprocessing/simul-native.json', 'preprocessing/simul-precision-diagnosis.json',
    'preprocessing/transport-differences.json', 'numeric-classes/receipt.json', 'native-numerics.json',
    'standard-debug.log', 'standard-release.log', 'clippy-final.log', 'open-checks.json',
    'lifecycle-debug/cli-failures.json', 'lifecycle-release/cli-failures.json',
    'check_lifecycle-debug-final.log', 'check_lifecycle-release-final.log',
    'bounded-debug.log', 'bounded-release.log']]
authored = subprocess.check_output(['git', 'ls-files', '-z', '--cached', '--others', '--exclude-standard',
                                    '--', 'rust', 'validation/input_to_bat'], cwd=ROOT).split(b'\0')
source_paths = sorted({ROOT / raw.decode() for raw in authored if raw and (ROOT / raw.decode()).is_file()})
receipt = {
    'status': 'implementation delivery; acceptance pending; failures retained',
    'observed_git_head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
    'native_pin': 'fda6ad4a9753ad7dc5d291406f2f310a9161423f',
    'rustc': subprocess.check_output(['rustc', '--version', '--verbose'], text=True),
    'components': components, 'public_workflows': public,
    'open_P_record_counts': counts, 'original_AB_record_counts': original_ab,
    'inherited_open_gates': ['M2 exact raw SVD order', '0049 four utility boundaries', 'DDH original 1 ns'],
    'evidence': [identity(p) for p in sorted(set(evidence))],
    'authored_tree_at_receipt': [identity(p) for p in source_paths],
    'chronology': 'Per-run receipts retain actual executable identities. Final negative-NaN parser repair '
                  'has separate native/debug/release evidence and final standard tests; earlier finite public '
                  'workflows retain their original executable hashes. AB final batches use the repaired build.',
}
(BUILD / 'handover-receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
print('Indexed completed checks, 36 public comparisons, and all unresolved P/AB records')
