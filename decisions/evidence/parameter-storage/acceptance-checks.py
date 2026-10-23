#!/usr/bin/env python3
"""Pin O's acceptance evidence without modifying implementation or references."""
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
BASE = ROOT / 'rust/verification/parameter_storage'
def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def load(path):
    return json.loads(Path(path).read_text())

central = load('/tmp/tempo2-parameter-central-review.json')
assert central['status'].startswith('PASS:')
regression = load(BASE / 'regressions/delivery_checks.json')
cli = {}
for profile in ['debug', 'release']:
    path = ROOT / 'rust/target' / profile / 'tempo2-rust'
    actual = sha(path)
    assert actual == regression['native_symbols'][profile]['binary_sha256']
    assert regression['native_symbols'][profile]['retained_C_clock_or_JPL_symbols'] is False
    cli[profile] = {'path': str(path), 'sha256': actual}
inventory = load(BASE / 'module_inventory.json')
for rel, info in inventory.items():
    assert sha(ROOT / rel) == info['sha256']
unsafe = {path: info['unsafe_lines'] for path, info in inventory.items() if info['unsafe_lines']}
assert {path: len(lines) for path, lines in unsafe.items()} == {
    'rust/src/clock_text.rs': 5, 'rust/src/get_obs_coord.rs': 2, 'rust/src/stdio.rs': 5,
}
for rel in [*unsafe, 'rust/Cargo.toml', 'rust/Cargo.lock', 'rust/build.rs']:
    prior = subprocess.check_output(['git', 'show', '84b6b86:' + rel], cwd=ROOT)
    assert prior == (ROOT / rel).read_bytes(), ('prior ABI/dependency/unsafe changed', rel)
for rel in ['validation/clocks_port/run.py', 'validation/clocks_port/regressions.py']:
    assert (ROOT/rel).read_bytes() == subprocess.check_output(['git', 'show', '8d2be6f:' + rel], cwd=ROOT)
failures = load(BASE / 'regressions/m2/open_gate_failures.json')
assert failures == load(ROOT / 'rust/verification/m2/open_gate_failures.json')
result = {
    'decision': '0036',
    'delivery_commit': central['delivery_commit'],
    'production_commit': '8bad0be9ebdc649d431ee60aaf9d6cd52e845c7e',
    'evidence_commit': subprocess.check_output(['git', 'rev-parse', '7d24a91'], cwd=ROOT, text=True).strip(),
    'reviewed_current_sources': central['committed_executed_sources_verified'],
    'verified_current_and_committed_delivery_artifacts': central['committed_artifacts_verified'],
    'protected_files': central['protected_files_verified'],
    'current_module_inventory': len(inventory),
    'current_cli': cli,
    'unsafe_sites_unchanged_from_L': unsafe,
    'Cargo_manifest_lock_and_build_unchanged_from_L': True,
    'central_execution': {
        'profiles': ['debug', 'release'], 'fresh_build_and_direct_comparison': True,
        'exact_cases_per_profile': 10, 'ordered_events_per_profile': 32,
        'slots': 2000, 'cells': 9616, 'boundary_cases': 8,
        'supplemental_copies': 2, 'controls_per_profile': 30,
        'focused_tests_per_profile': 4, 'all_raw_data_match_delivery': True,
        'comparison_counts_per_profile': central['profiles'][0]['counts'],
    },
    'full_regression_evidence': regression,
    'central_did_not_rerun': ['full Rust test suite', 'formatting', 'Clippy', 'clock/glob/provenance', 'JPL', 'M1a', 'M1b', 'M2'],
    'M2_failure_content': failures,
    'nested_repositories': central['nested_repositories'],
    'central_driver_sha256': sha('/tmp/tempo2-parameter-central-review.py'),
    'central_receipt_sha256': sha('/tmp/tempo2-parameter-central-review.json'),
    'central_log_sha256': sha('/tmp/tempo2-parameter-central-review.log'),
    'acceptance_checker_sha256': sha(__file__),
    'scope': 'Decision 0033 parameter-only storage, initialization, copy, strict frontend and existing evaluator/fitter integration; no full PAR/TIM/pulsar or M2 acceptance',
}
Path('/tmp/tempo2-parameter-acceptance-checks.json').write_text(json.dumps(result, indent=2, sort_keys=True) + '\n')
print('Acceptance checks PASS:', len(inventory), 'modules;', sum(map(len, unsafe.values())), 'unchanged unsafe sites')
