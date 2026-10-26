"""Bounded original/corrected witnesses. Original UB bytes are never goldens.

Usage: python3 witness_bugs.py --native build/native-v8 --out build/bugs
"""
from pathlib import Path
import argparse
import hashlib
import json
import os
import resource
import subprocess
from check import ROOT, HERE, run, sha

parser = argparse.ArgumentParser()
parser.add_argument('--native', type=Path, required=True)
parser.add_argument('--out', type=Path, required=True)
a = parser.parse_args()
native, out = a.native.resolve(), a.out.resolve()
out.mkdir(parents=True, exist_ok=False)
runtime = ROOT / 'validation/reference/build/source/T2runtime'
rust = ROOT / 'rust/target/debug/tempo2-rust'
env = dict(os.environ, TEMPO2=str(runtime))
records, checks = [], {}
for case in ['helpers', 'activation', 'snapshot-poison']:
    for kind, executable in [('original', 'probe-poison' if case == 'snapshot-poison' else 'probe-original'), ('corrected', 'probe')]:
        command = [native / executable, case]
        if case == 'snapshot-poison':
            command += [HERE / 'inputs/snapshot.par']
        r = run(command, out / case / kind, env)
        records.append(dict(r, case=case, kind=kind))
        assert r['status'] == 0
original = (out / 'helpers/original/stdout').read_text().splitlines()
corrected = (out / 'helpers/corrected/stdout').read_text().splitlines()
checks['RMS_current_owner_changes_second_pulsar'] = original[1] != corrected[1]
checks['RMS_first_pulsar_and_other_helpers_unchanged'] = original[0] == corrected[0] and original[2:] == corrected[2:]
old = (out / 'activation/original/stdout').read_text()
new = (out / 'activation/corrected/stdout').read_text()
checks['original_activation_reads_poison'] = '-12345' in old
checks['corrected_activation_snapshots_defined_prior_values'] = '-12345' not in new
checks['original_global_count_unwritten'] = 'globalNfit -12345' in old
checks['corrected_global_count_zero_without_global_fit'] = 'globalNfit 0' in new
old = (out / 'snapshot-poison/original/snapshot.par').read_text()
new = (out / 'snapshot-poison/corrected/snapshot.par').read_text()
line = lambda text: next(x for x in text.splitlines() if x.startswith('CHI2R'))
checks['original_unweighted_chisqr_poison'] = '-12345.0000' in line(old)
checks['corrected_unweighted_chisqr_uses_fit_result'] = '2.0000 1' in line(new)
old = (out / 'snapshot-poison/original/stdout').read_text()
new = (out / 'snapshot-poison/corrected/stdout').read_text()
checks['original_report_prefit_poison'] = '-12345' in old
checks['corrected_report_prefit_snapshots_prior_value'] = '-12345' not in new

# Original argument bugs can loop or corrupt storage. Limit only the test child,
# retaining its complete bounded prefix and termination status, never crashing
# the calling process or writing an unbounded core/log.
def limits():
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    resource.setrlimit(resource.RLIMIT_FSIZE, (1024 * 1024, 1024 * 1024))

def bounded(command, directory, environment):
    directory.mkdir(parents=True, exist_ok=True)
    with (directory / 'stdout').open('wb') as stdout, (directory / 'stderr').open('wb') as stderr:
        try:
            code = subprocess.run(list(map(str, command)), cwd=directory, env=environment,
                    stdout=stdout, stderr=stderr, timeout=2, preexec_fn=limits).returncode
        except subprocess.TimeoutExpired:
            code = 'timeout'
    records.append({'case': str(directory.relative_to(out)), 'command': list(map(str, command)),
                    'status': code, 'files': {p.name: sha(p) for p in directory.iterdir() if p.is_file()}})
    return code

for name, content in [('direct-cycle', 'a a\n'), ('growing-cycle', 'a a extra\n'),
                      ('indirect-cycle', 'a b\nb a\n'), ('empty-alias', 'a \n')]:
    local = out / name / 'runtime'
    local.mkdir(parents=True)
    (local / 'alias.dat').write_text(content)
    environment = dict(env, TEMPO2=str(local))
    for kind, exe in [('original', native / 'tempo2-bounded'), ('rust', rust)]:
        status = bounded([exe, 'a'], out / name / kind, environment)
        if kind == 'rust':
            checks[name + '_bounded_error'] = status == 1
for kind, exe in [('original', native / 'tempo2-bounded'), ('rust', rust)]:
    status = bounded([exe, '-f'], out / 'missing-argument' / kind, env)
    if kind == 'rust':
        checks['missing_operand_bounded_error'] = status == 1
# A path with whitespace/semicolon and a 'par' substring demonstrates the source
# shell word splitting and first-substring replacement. No shell command is user supplied.
par = ROOT / 'validation/general_fitting/inputs/robust-H.par'
tim = ROOT / 'validation/general_fitting/inputs/multi-frequency.tim'
for kind, exe in [('original', native / 'tempo2-bounded'), ('rust', rust)]:
    dest = out / 'directory-pairing' / kind
    dest.mkdir(parents=True)
    (dest / 'par space ; name.par').write_bytes(par.read_bytes())
    (dest / 'par space ; name.tim').write_bytes(tim.read_bytes())
    status = bounded([exe, '-npsr', '2', '-nobs', '100', '-allParTim'], dest, env)
    if kind == 'rust':
        # Fitting can take longer than a malformed-input witness: verify discovery
        # here; the unbounded valid runner below verifies completion and recovery.
        checks['byte_preserving_pair_discovery'] = b'Obtained files for 1 pulsars' in (dest / 'stdout').read_bytes()
command = [rust, '-npsr', '2', '-nobs', '100', '-noaccel', '-f', par, tim, '-newpar']
r = run(command, out / 'recovered', env)
records.append(dict(r, kind='rust', case='recovered'))
checks['valid_processing_after_errors'] = r['status'] == 0 and (out / 'recovered/new.par').is_file()
receipt = {'checks': checks, 'runs': records, 'native_identity': sha(native / 'identity.json'),
           'rust': sha(rust), 'harness': sha(Path(__file__)),
           'note': 'Poison values expose source reads with no producer. They are diagnostic instrumentation, not undefined-byte golden values.'}
(out / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
print(json.dumps(checks, indent=2))
raise SystemExit(int(not all(checks.values())))
