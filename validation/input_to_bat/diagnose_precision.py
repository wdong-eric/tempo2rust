#!/usr/bin/env python3
"""Reproduce P's localized SIMUL precision diagnosis without changing production."""
import gzip
import hashlib
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
BUILD = ROOT / 'validation/input_to_bat/build/preprocessing'
reference = ROOT / 'validation/preprocessing/observations.json.gz'
case = next(c for c in json.load(gzip.open(reference, 'rt')) if c['id'] == 'simul_123')
event = next(e for e in case['events'] if e.get('action') == 1 and e['phase'] == 'end')
extracted = {'case': case['id'], 'sat': event['changes']['p0.obs1.sat']['after'],
             'reference_sha256': hashlib.sha256(reference.read_bytes()).hexdigest(),
             'actions': case['actions'], 'simul_events': [e for e in case['events'] if e.get('op') == 'simul']}
BUILD.mkdir(parents=True, exist_ok=True)
(BUILD / 'simul-native.json').write_text(json.dumps(extracted, indent=2) + '\n')
command = ['cargo', 'test', '--manifest-path', str(ROOT / 'rust/Cargo.toml'), '--locked', '--offline',
           '--release', '--lib', 'preprocessing_corpus_tests::p_precision_diagnosis', '--', '--ignored', '--exact']
run = subprocess.run(command, env={**os.environ, 'CARGO_HOME': str(ROOT / 'rust/.cargo-home'),
                                 'GMP_MPFR_SYS_CACHE': str(ROOT / 'rust/target/gmp-cache')})
raise SystemExit(run.returncode)
