"""Natural NITS fitting diagnostic files, independent of solver-only controls."""
from pathlib import Path
import argparse, hashlib, json, os, subprocess
from run_public import CASES, RUNTIME, FIT_INPUTS
from compare import compare
ROOT = Path(__file__).resolve().parents[2];HERE = Path(__file__).resolve().parent
BUILD = HERE / 'build/public-files';sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()


def run(profile, names=()):
    BUILD.mkdir(parents=True, exist_ok=True)
    identity = json.loads((HERE / 'build/native/identity.json').read_text())
    native = BUILD / 'native';cmd = identity['commands'][-1].copy();cmd[cmd.index('-o') + 1] = str(native)
    subprocess.run(cmd, capture_output=True, check=True)
    rust = ROOT / 'rust/target' / profile / 'tempo2-rust';records = []
    for name, pars, opts in CASES:
        if name not in (names or ['spin', 'global', 'red', 'dm-noise', 'chromatic-noise', 'robust-H']): continue
        dest = BUILD / profile / (name + ('-defined-dm' if name in ['dm-noise', 'chromatic-noise'] else ''));dest.mkdir(parents=True, exist_ok=True)
        original_pars = pars
        if name in ['dm-noise', 'chromatic-noise']:
            supplied = dest / 'supplied-dm.par';supplied.write_text(pars[0].read_text() + 'DM1 0\nDM2 0\n');pars = [supplied]
        dirs = {e: dest / e for e in ['native', 'rust']}
        for p in dirs.values(): p.mkdir(exist_ok=True)
        trace = dirs['native'] / 'trace.tsv'
        native_cmd = [str(native), str(trace), str(len(pars))]
        rust_cmd = [str(rust), 'timing', '--fit', '--runtime', str(RUNTIME), '--fit-diagnostics', str(dirs['rust'])]
        for par in pars:
            native_cmd += [str(par), str(FIT_INPUTS / 'bary.tim')]
            rust_cmd += ['--par', str(par), '--tim', str(FIT_INPUTS / 'bary.tim')]
        env = {**os.environ, 'TEMPO2': str(RUNTIME), 'AE_WRITE_RESIDUALS': '7'}
        for key in ['AE_COVARIANCE', 'AE_NORESCALE']: env.pop(key, None)
        for e, command in zip(dirs, [native_cmd, rust_cmd]):
            p = subprocess.run(command, cwd=dirs[e], env=env, capture_output=True, timeout=120)
            (dirs[e] / 'stdout').write_bytes(p.stdout);(dirs[e] / 'stderr').write_bytes(p.stderr)
            assert p.returncode == 0, (name, e, p.returncode, p.stderr.decode())
        result = json.loads((dirs['rust'] / 'stdout').read_text())
        expected = {p.name for p in dirs['native'].iterdir()} - {'trace.tsv', 'stdout', 'stderr'}
        actual = {p.name for p in dirs['rust'].iterdir()} - {'stdout', 'stderr'}
        failures = []
        if actual != expected: failures.append({'quantity': 'file_membership', 'native': sorted(expected), 'rust': sorted(actual)})
        for file in actual & expected:
            if (dirs['native'] / file).read_bytes() != (dirs['rust'] / file).read_bytes(): failures.append({'quantity': file, 'kind': 'file_bytes'})
        exported = {Path(p).name for c in result['cycles'] for s in c['systems'] for p in s['diagnostic_files']}
        if exported != actual: failures.append({'quantity': 'exported_membership', 'files': sorted(actual), 'exported': sorted(exported)})
        comparison = compare(trace, dirs['rust'] / 'stdout', dest / 'comparison.json')
        records.append({'case': name, 'inputs': {str(p): sha(p) for p in original_pars + pars + [FIT_INPUTS/'bary.tim']}, 'commands': [native_cmd, rust_cmd], 'files': {f: {'native': sha(dirs['native'] / f), 'rust': sha(dirs['rust'] / f)} for f in actual & expected}, 'file_failures': failures, 'numerical_failures': comparison['failures']})
        print(name, len(failures), 'file failures;', len(comparison['failures']), 'existing numerical checks')
    (BUILD / profile / ('receipt-'+'-'.join(names)+'.json' if names else 'receipt.json')).write_text(json.dumps({'profile': profile, 'compile': cmd, 'native_sha256': sha(native), 'rust_sha256': sha(rust), 'builder_sha256': sha(__file__), 'records': records}, indent=2) + '\n')


if __name__ == '__main__':
    p = argparse.ArgumentParser();p.add_argument('--profile', choices=['debug', 'release'], default='debug');p.add_argument('cases', nargs='*');args=p.parse_args();run(args.profile,args.cases)
