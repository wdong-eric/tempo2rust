#!/usr/bin/env python3
"""Native signed-class witness and the corresponding Rust parser/formatter check."""
import hashlib
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
BUILD = ROOT / 'validation/input_to_bat/build/numeric-classes'
BUILD.mkdir(parents=True, exist_ok=True)
source = BUILD / 'probe.cc'
source.write_text('''#include "tempo2.h"
#include <cstdio>
int main() {
    for (const char* input : {"nan", "-nan", "nan(payload)", "-nan(payload)", "0", "-0", "inf", "-inf"}) {
        longdouble value=parse_longdouble(input);
        char out[80];quadmath_snprintf(out,sizeof out,"%.3Qf",value);
        printf("%s %d %s\\n",input,signbitq(value)!=0,out);
    }
}
''')
archive = ROOT / 'validation/reference/build/libtempo2-reference.a'
manifest = json.loads((ROOT / 'validation/reference/build_manifest.json').read_text())
sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
assert sha(archive) == manifest['static_library_sha256']
executable = BUILD / 'probe'
command = ['/opt/homebrew/bin/g++-16', '-O2', '-std=gnu++11', '-DHAVE_CONFIG_H',
           '-I' + str(ROOT / 'validation/reference/build/source'), str(source), str(archive),
           '-lquadmath', '-lgfortran', '-o', str(executable)]
subprocess.run(command, check=True)
native = subprocess.check_output([str(executable)], text=True)
assert native == ('nan 0 nan\n-nan 1 -nan\nnan(payload) 0 nan\n-nan(payload) 1 -nan\n'
                  '0 0 0.000\n-0 1 -0.000\ninf 0 inf\n-inf 1 -inf\n')
env = {**os.environ, 'CARGO_HOME': str(ROOT / 'rust/.cargo-home'),
       'GMP_MPFR_SYS_CACHE': str(ROOT / 'rust/target/gmp-cache')}
runs = []
for profile in ['debug', 'release']:
    test = ['cargo', 'test', '--manifest-path', str(ROOT / 'rust/Cargo.toml'), '--locked', '--offline',
            *(['--release'] if profile == 'release' else []), '--lib',
            'source_par_tests::extended_input_preserves_nonfinite_and_zero_signs', '--', '--exact']
    subprocess.run(test, env=env, check=True)
    runs.append({'profile': profile, 'command': test, 'returncode': 0})
(BUILD / 'receipt.json').write_text(json.dumps({
    'source_sha256': sha(source), 'archive_sha256': sha(archive),
    'executable_sha256': sha(executable), 'compile_command': command,
    'native_stdout': native, 'rust_runs': runs,
}, indent=2) + '\n')
print('PASS: eight native signed classes and both Rust profiles')
