#!/usr/bin/env python3
"""Reproduce the local study and bounded feasibility probes, without installs."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import time

root = Path(__file__).resolve().parent
os.chdir(root)
results = root / 'results'
results.mkdir(exist_ok=True)
(root / 'probes/bin').mkdir(exist_ok=True)
env = dict(os.environ, CARGO_HOME=str(root / '.cargo-home'), GMP_MPFR_SYS_CACHE=str(root / 'target/gmp-cache'))
commands = []

def run(args, log, extra=None, required=False):
    started = time.monotonic()
    with (results / log).open('w') as stream:
        status = subprocess.run(args, env=env | (extra or {}), stdout=stream, stderr=subprocess.STDOUT).returncode
    commands.append({'command': args, 'environment_overrides': extra or {}, 'log': log,
                     'exit_code': status, 'wall_seconds': time.monotonic()-started})
    if required and status:
        raise RuntimeError(f'{args[0]} failed; see results/{log}')
    return status

try:
    run(['rustc','-Vv'],'rustc.txt',required=True)
    run(['cargo','-V'],'cargo.txt',required=True)
    run(['uname','-a'],'host.txt',required=True)
    run(['rustup','target','list','--installed'],'targets.txt',required=True)
    run(['rustup','toolchain','list'],'toolchains.txt',required=True)
    run(['cargo','run','--release','--locked','--offline'],'run.log',required=True)
    qd_library = next((root / 'target/release/deps').glob('libqd-*.rlib'))
    run(['rustc','--edition','2024','probes/qd_missing.rs','--extern',f'qd={qd_library}',
         '-L','dependency=target/release/deps','-o','probes/bin/qd-missing'],'qd-missing.log')
    run(['rustc','probes/builtin_arithmetic.rs','-o','probes/bin/builtin-stable'],'builtin-stable.log')
    nightly = 'nightly-2026-09-28'
    if nightly in (results / 'toolchains.txt').read_text():
        run(['rustc',f'+{nightly}','-Vv'],'nightly.txt',required=True)
        status=run(['rustc',f'+{nightly}','--cfg','nightly_probe','probes/builtin_arithmetic.rs',
                    '-O','-o','probes/bin/builtin-nightly'],'builtin-nightly-build.log')
        if status==0: run(['probes/bin/builtin-nightly'],'builtin-nightly.log',required=True)
        run(['rustc',f'+{nightly}','probes/builtin_io_math.rs','-O','-o','probes/bin/builtin-io-math'],'builtin-io-math.log')
        status=run(['rustc',f'+{nightly}','probes/builtin_math.rs','-O','-o','probes/bin/builtin-math'],'builtin-math.log')
        if status==0: run(['probes/bin/builtin-math'],'builtin-math-run.log',required=True)
    else:
        commands.append({'probe':'built-in nightly f128','status':'SKIP: requested installed nightly unavailable'})
    quadmath=['cargo','build','--manifest-path','probes/quadmath/Cargo.toml','--locked','--offline']
    default_status=run(quadmath,'quadmath-default.log')
    gcc=shutil.which('gcc-16')
    if gcc:
        library=subprocess.check_output([gcc,'-print-file-name=libquadmath.dylib'],text=True).strip()
        extra={'CC':gcc,'LIBRARY_PATH':str(Path(library).parent)}
        status=run(quadmath,'quadmath-gcc.log',extra)
        if status==0:
            run(['probes/quadmath/target/debug/quadmath-feasibility'],'quadmath-run.log',required=True)
    elif default_status==0:
        run(['probes/quadmath/target/debug/quadmath-feasibility'],'quadmath-run.log',required=True)
    else:
        commands.append({'probe':'quadmath GCC retry','status':'SKIP: gcc-16 unavailable'})
    run(['cargo','fmt','--all','--check'],'format.log',required=True)
    run(['cargo','tree','--locked','--offline'],'dependencies.txt',required=True)
    run(['python3','summarize.py'],'summarize.log',required=True)
finally:
    (results / 'commands.json').write_text(json.dumps(commands,indent=2)+'\n')
print('Completed; see REPORT.md, results/summary.md, results/commands.json.')
