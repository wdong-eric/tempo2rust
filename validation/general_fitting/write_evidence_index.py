"""Index the AE delivery receipts; this records evidence, not acceptance."""
from pathlib import Path
import argparse
import hashlib
import json
import subprocess

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
BUILD = HERE / 'build'


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--regressions', type=Path, required=True,
                        help='Completed build/regressions/run-* directory')
    args = parser.parse_args()
    regression = args.regressions.resolve()
    assert regression.is_relative_to(BUILD / 'regressions')
    revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
    tracked = subprocess.check_output(['git', 'ls-files', '-z', 'rust', 'validation/general_fitting'], cwd=ROOT).decode().split('\0')
    authored = [ROOT / name for name in tracked if name and Path(name).suffix in
                {'.rs', '.py', '.cc', '.h', '.toml', '.lock', '.par', '.tim'}]
    # A report-only follow-up can remain dirty; implementation/validation bytes
    # must match the revision being recorded.
    subprocess.run(['git', 'diff', '--exit-code', 'HEAD', '--', *[str(p.relative_to(ROOT)) for p in authored]], cwd=ROOT, check=True, capture_output=True)
    receipts = [BUILD / name / 'receipt.json' for name in [
        'solver', 'order', 'cache', 'callbacks', 'fitinfo', 'copy',
        'source-boundaries', 'bootstrap-stack', 'gw-boundary', 'legacy-layout',
        'component-campaign',
    ]]
    for profile in ['debug', 'release']:
        receipts += [BUILD / name / profile / 'receipt.json' for name in [
            'public', 'families', 'solver-contract', 'svd-api', 'callback-grid',
            'standalone-callbacks', 'constraint-payloads', 'api-lifecycle',
            'components', 'automatic', 'bootstrap', 'bootstrap-capacity',
            'bootstrap-history', 'legacy', 'public-files',
        ]]
        receipts += [BUILD / name / (profile + '.json') for name in [
            'kernels', 'covariance-models', 'spectral-session', 'ddh', 'pipeline-solvers',
        ]]
    receipts += [BUILD / 'comparator-controls' / name / 'receipt.json'
                 for name in ['spin', 'robust-H', 'covariance', 'central']]
    receipts += [BUILD / 'contraction/debug.json', BUILD / 'fft/debug.json',
                 BUILD / 'native/identity.json', regression / 'receipt.json',
                 regression / 'compatibility/m2-open-gates.json',
                 regression / 'compatibility/commands.json']
    artifacts = []
    for profile in ['debug', 'release']:
        for name in ['public', 'families', 'constraint-payloads', 'ddh']:
            for filename in ['comparison.json', 'identity.json', 'native.tsv', 'rust.json']:
                artifacts += sorted((BUILD / name / profile).glob('*/' + filename))
    artifacts += sorted(regression.glob('timing/*/*/comparison.json'))
    executables = [BUILD / 'native/fit', ROOT / 'validation/reference/build/libtempo2-reference.a']
    for profile in ['debug', 'release']:
        executables.append(ROOT / 'rust/target' / profile / 'tempo2-rust')
        executables += [ROOT / 'rust/target' / profile / 'examples' / p.stem
                        for p in sorted((ROOT / 'rust/examples').glob('fitting_*.rs'))]
    def hashes(paths):
        result = {}
        for path in sorted(set(paths)):
            assert path.is_file(), path
            result[str(path.relative_to(ROOT))] = hashlib.sha256(path.read_bytes()).hexdigest()
        return result
    index = {
        'implementation_revision': revision,
        'source_pin': 'fda6ad4a9753ad7dc5d291406f2f310a9161423f',
        'interpretation': 'Evidence index only. Per-run receipts identify tested binaries and failed gates; current executables do not retroactively replace those identities.',
        'receipts': hashes(receipts), 'comparison_and_identity_files': hashes(artifacts),
        'authored_source': hashes(authored), 'available_executables': hashes(executables),
    }
    destination = BUILD / 'delivery' / ('index-' + revision[:12] + '.json')
    destination.parent.mkdir(exist_ok=True)
    with destination.open('x') as stream:
        stream.write(json.dumps(index, indent=2) + '\n')
    print(destination)
    print(hashlib.sha256(destination.read_bytes()).hexdigest())
    print({name: len(index[name]) for name in ['receipts', 'comparison_and_identity_files', 'authored_source', 'available_executables']})


if __name__ == '__main__':
    main()
