"""Bounded source-extracted predictor C state/diagnostic witness for AJ planning."""
from pathlib import Path
import hashlib
import json
import subprocess
import tempfile

ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
PIN = 'fda6ad4a9753ad7dc5d291406f2f310a9161423f'
OUT = Path(tempfile.mkdtemp(prefix='tempo2-predictor-interface-state-'))
Path('/tmp/tempo2-predictor-interface-state-current').write_text(str(OUT) + '\n')
sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()


def body(source, signature):
    start = source.index(signature)
    brace = source.index('{', start)
    depth = 1
    stop = brace + 1
    while depth:
        depth += (source[stop] == '{') - (source[stop] == '}')
        stop += 1
    return source[start:stop] + '\n'


sources = {}
for name in ['tempo2pred.h', 'cheby2d.c', 't1polyco.c']:
    raw = subprocess.check_output(['git', '-C', str(ROOT / 'tempo2'), 'show', PIN + ':' + name])
    (OUT / name).write_bytes(raw)
    sources[name] = raw.decode()

functions = []
for name, signature in [
    ('cheby2d.c', 'int ChebyModelSet_GetNearestIndex('),
    ('cheby2d.c', 'ChebyModel *ChebyModelSet_GetNearest('),
    ('t1polyco.c', 'long double T1Polyco_GetPhase('),
    ('t1polyco.c', 'long double T1Polyco_GetFrequency('),
]:
    functions.append(body(sources[name], signature))

program = '''#include <stdio.h>
#include <math.h>
#include "tempo2pred.h"
int ChebyModelSet_OutOfRange = 0;
''' + '\n'.join(functions) + r'''
int main(void) {
    ChebyModel models[2] = {0};
    models[0].mjd_start = 58000; models[0].mjd_end = 58002;
    models[1].mjd_start = 58001; models[1].mjd_end = 58003;
    ChebyModelSet set = {models, 2};
    ChebyModelSet_OutOfRange = 17;
    int index = ChebyModelSet_GetNearestIndex(&set, 58004);
    printf("index_miss %d %d\n", index, ChebyModelSet_OutOfRange);
    ChebyModelSet_OutOfRange = 17;
    ChebyModel *nearest = ChebyModelSet_GetNearest(&set, 58004);
    printf("pointer_miss %d %d\n", nearest == NULL, ChebyModelSet_OutOfRange);
    index = ChebyModelSet_GetNearestIndex(&set, 58001.5L);
    printf("index_tie %d %d\n", index, ChebyModelSet_OutOfRange);
    nearest = ChebyModelSet_GetNearest(&set, 58002.5L);
    printf("pointer_hit %td %d\n", nearest - models, ChebyModelSet_OutOfRange);
    T1Polyco t1 = {0};
    t1.mjd_mid = 58000; t1.reference_phase = 7; t1.frequency_psr_0 = 100;
    t1.ncoeff = 2; t1.coeff[0] = .25L; t1.coeff[1] = 6;
    t1.frequency_obs = 1400;
    puts("frequency_begin");
    printf("frequency %.17Lg\n", T1Polyco_GetFrequency(&t1, 58000, 1400));
    puts("frequency_end");
    puts("phase_begin");
    printf("phase %.17Lg\n", T1Polyco_GetPhase(&t1, 58000, 1400));
    puts("phase_end");
    return 0;
}
'''
(OUT / 'probe.c').write_text(program)
compile_command = ['/opt/homebrew/bin/gcc-16', '-std=c11', '-O2', str(OUT / 'probe.c'), '-lm', '-o', str(OUT / 'probe')]
compiled = subprocess.run(compile_command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
(OUT / 'compile.log').write_text(compiled.stdout)
assert compiled.returncode == 0, compiled.stdout
executed = subprocess.run([str(OUT / 'probe')], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
(OUT / 'stdout.txt').write_text(executed.stdout)
(OUT / 'stderr.txt').write_text(executed.stderr)
assert executed.returncode == 0 and not executed.stderr
lines = executed.stdout.splitlines()
assert lines[:4] == ['index_miss -1 0', 'pointer_miss 1 1', 'index_tie 0 0', 'pointer_hit 1 0']
frequency_lines = lines[lines.index('frequency_begin') + 1:lines.index('frequency_end')]
phase_lines = lines[lines.index('phase_begin') + 1:lines.index('phase_end')]
assert len(frequency_lines) == 1 and frequency_lines[0].startswith('frequency ')
assert phase_lines == ['WARNIGN WARNGING: Silly update in polyco read', 'Have 58000 58000', 'Diff = 0', 'phase 7.25']
receipt = {
    'scope': 'Four exact source-extracted bodies, valid initialized caller storage, macOS arm64 C11. Checks nearest-index/pointer global-state distinction, ties and T1 phase-versus-frequency diagnostics. Not a full standalone library, Rust C ABI, Linux or numerical-coverage qualification.',
    'pin': PIN, 'source_hashes': {name: sha(OUT / name) for name in sources},
    'extracted_body_hashes': [hashlib.sha256(s.encode()).hexdigest() for s in functions],
    'compile_command': compile_command, 'compile_exit': compiled.returncode,
    'run_command': [str(OUT / 'probe')], 'run_exit': executed.returncode,
    'compiler': subprocess.check_output(['/opt/homebrew/bin/gcc-16', '--version'], text=True),
    'program_sha256': sha(OUT / 'probe.c'), 'executable_sha256': sha(OUT / 'probe'),
    'stdout_sha256': sha(OUT / 'stdout.txt'), 'stderr_sha256': sha(OUT / 'stderr.txt'),
    'state_rows': lines[:4], 'frequency_output': frequency_lines, 'phase_output': phase_lines,
}
(OUT / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
print(json.dumps({'directory': str(OUT), 'receipt_sha256': sha(OUT / 'receipt.json'), 'state_rows': lines[:4], 'phase_diagnostic_lines': len(phase_lines) - 1}, indent=2))
