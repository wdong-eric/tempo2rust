"""Controlled AF timing-precision diagnosis; production and accepted evidence stay read-only."""
from pathlib import Path
from decimal import Decimal as D
import argparse, difflib, hashlib, importlib.util, json, os, shutil, subprocess, sys, tempfile

sys.dont_write_bytecode = True
PRODUCTION = '742062270f0cd29af57c0a43299f23865d05d834'
OBSERVER = '543760f288dd7dffbcefb2d76efbf50892c84de1'
EXPECTED_NATIVE = '3bf82ca766e45599c23a575868d6977d97c2f25c46428c02c877ccca0276a27f'

def sha(path):
    h = hashlib.sha256()
    with path.open('rb') as f:
        for block in iter(lambda: f.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()

def write(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')

def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--root', type=Path, required=True)
    p.add_argument('--dependency-target', type=Path, required=True)
    p.add_argument('--output', type=Path)
    a = p.parse_args()
    root = a.root.resolve()
    out = a.output.resolve() if a.output else Path(tempfile.mkdtemp(prefix='tempo2-predictor-precision-'))
    if a.output:
        out.mkdir(parents=True, exist_ok=False)
    print('OUT', out, flush=True)
    Path('/tmp/tempo2-predictor-precision-current').write_text(str(out) + '\n')
    snapshot = out / 'snapshot'
    snapshot.mkdir()
    archives = {}
    for revision, area, label in [(PRODUCTION, 'rust', 'production'), (OBSERVER, 'validation/predictors', 'observer')]:
        archive = out / (label + '.tar')
        with archive.open('wb') as f:
            subprocess.run(['git', 'archive', revision, area], cwd=root, stdout=f, check=True)
        subprocess.run(['/usr/bin/tar', '-xf', str(archive), '-C', str(snapshot)], check=True)
        archives[label] = {'revision': revision, 'sha256': sha(archive)}
    sources = {str(x.relative_to(snapshot)): sha(x) for x in snapshot.rglob('*') if x.is_file()}
    write(out / 'source-identities.json', sources)
    sys.path.insert(0, str(snapshot / 'validation/predictors'))
    from compare import read_tsv, constructed
    from run_public import CASES

    dependency_target = a.dependency_target.resolve()
    deps = dependency_target / 'release/deps'
    externs = {}
    for name in ['rug', 'serde', 'serde_json', 'sha2']:
        paths = list(deps.glob('lib' + name + '-*.rlib'))
        assert len(paths) == 1, (name, paths)
        externs[name] = paths[0]
    native_archives = list((dependency_target / 'release/build').glob('tempo2-rust-*/out/libtiming_native.a'))
    assert len(native_archives) == 1, native_archives
    sofa = native_archives[0]
    native = root / 'validation/predictors/build/native-corrected/predictor'
    assert sha(native) == EXPECTED_NATIVE
    shutil.copy2(native, out / 'native.snapshot')
    native = out / 'native.snapshot'
    runtime = root / 'validation/reference/build/source/T2runtime'
    old_public = root / 'validation/predictors/build/public-release-corrected'
    protected_paths = [*externs.values(), sofa,
        root / 'decisions/0008-preservation-policy-and-stage-plan.md',
        root / 'validation/predictors/build/rms-localization-delivery/receipt.json',
        root / 'validation/predictors/build/native-corrected/identity.json',
        root / 'validation/predictors/build/native-corrected/predictor',
        root / 'validation/reference/build_manifest.json', old_public / 'receipt.json']
    protected_paths += [x for x in runtime.rglob('*') if x.is_file()]
    for name, _, _, _ in CASES:
        protected_paths += [old_public / (name + '-polyco') / v for v in ['native/result.tsv', 'rust/generate.stdout']]
    protected = {str(x): sha(x) for x in protected_paths}
    write(out / 'protected-before.json', protected)
    env = {**os.environ, 'PATH': '/Users/Eric.Dong/.cargo/bin:/opt/homebrew/bin:/usr/bin:/bin',
           'TEMPO2': str(runtime), 'CARGO_PKG_VERSION': '0.1.0',
           'CARGO_MANIFEST_DIR': str(snapshot / 'rust'), 'CARGO_PKG_NAME': 'tempo2-rust'}
    rustc = subprocess.check_output(['/Users/Eric.Dong/.cargo/bin/rustup', 'which', 'rustc'], text=True, env=env).strip()
    commands = []
    def run(command, cwd, label):
        command = list(map(str, command))
        with (cwd / (label + '.stdout')).open('w') as stdout, (cwd / (label + '.stderr')).open('w') as stderr:
            result = subprocess.run(command, cwd=cwd, env=env, stdout=stdout, stderr=stderr)
        record = {'command': command, 'cwd': str(cwd), 'returncode': result.returncode,
                  'stdout': str(cwd / (label + '.stdout')), 'stderr': str(cwd / (label + '.stderr'))}
        commands.append(record)
        write(out / 'commands.json', commands)
        assert result.returncode == 0, record
    source = snapshot / 'rust/src'
    scalar = source / 'longdouble.rs'
    original = scalar.read_text()
    marker = 'pub const PRECISION_BITS: u32 = 128;'
    assert original.count(marker) == 1
    records = []
    for bits in [128, 113]:
        work = out / str(bits)
        work.mkdir()
        changed = original.replace(marker, f'pub const PRECISION_BITS: u32 = {bits};')
        scalar.write_text(changed)
        (work / 'precision.patch').write_text(''.join(difflib.unified_diff(original.splitlines(True), changed.splitlines(True), fromfile='original/rust/src/longdouble.rs', tofile='diagnostic/rust/src/longdouble.rs')))
        library = work / 'libtempo2_rust.rlib'
        executable = work / 'tempo2-rust'
        common = ['--edition=2024', '-C', 'opt-level=3', '-L', 'dependency=' + str(deps), '-L', 'native=' + str(sofa.parent)]
        external = [v for name, path in externs.items() for v in ['--extern', name + '=' + str(path)]]
        run([rustc, '--crate-name', 'tempo2_rust', '--crate-type', 'rlib', source / 'lib.rs', *common, '-l', 'static=timing_native', *external, '-o', library], work, 'compile-library')
        run([rustc, '--crate-name', 'tempo2_rust', source / 'main.rs', *common, *external, '--extern', 'tempo2_rust=' + str(library), '-o', executable], work, 'compile-cli')
        print('compiled', bits, flush=True)
        variant = {'bits': bits, 'scalar_source_sha256': sha(scalar), 'executable_sha256': sha(executable), 'cases': []}
        records.append(variant)
        for name, model, site, pre in CASES:
            case = work / name
            nr, rr = case / 'native', case / 'rust'
            nr.mkdir(parents=True)
            rr.mkdir()
            par = snapshot / 'validation/predictors/fixtures' / (model + '.par')
            # Arguments and comparisons are the frozen AF public polyco workflow.
            run([native, 'polyco', nr / 'result.tsv', par, site, '58000', '58000', '60', '12', '1', '1400', '1', '', *pre], nr, 'generate')
            run([executable, 'predictor', 'polyco', '--par', par, '--runtime', runtime, '--site', site,
                 '--start-mjd', '58000', '--end-mjd', '58000', '--span-minutes', '60', '--coefficients', '12',
                 '--max-hour-angle', '1', '--frequency-mhz', '1400', '--output-dir', rr, '--', *pre], rr, 'generate')
            data = json.loads((rr / 'generate.stdout').read_text())
            ref = read_tsv(nr / 'result.tsv')
            checks = constructed('polyco', data['result'], ref)
            checks.number('observation_count', len(data['observations']), sum(k == 'sat' for k, i in ref), 0)
            maximum = D(0)
            for i, observation in enumerate(data['observations']):
                for key, field, unit in [('sat', 'sat_mjd', 86400), ('bat', 'bat_mjd', 86400), ('phase', 'phase_turn', 1), ('freq', 'frequency_mhz', 1)]:
                    checks.number(f'{key}[{i}]', D(str(observation[field])) * unit, ref[key, i] * unit,
                                  '1e-10' if key == 'phase' else '1e-12' if key in ('sat', 'bat') else 0)
                maximum = max(maximum, abs(D(observation['phase_turn']) - ref['phase', i]))
            prior = old_public / (name + '-polyco')
            assert (nr / 'result.tsv').read_bytes() == (prior / 'native/result.tsv').read_bytes(), ('native control changed', name)
            if bits == 128:
                old = json.loads((prior / 'rust/generate.stdout').read_text())
                assert data['observations'] == old['observations'], ('128 phase control changed', name)
                for field in ['coefficients', 'models', 'warnings']:
                    assert data['result'][field] == old['result'][field], ('128 model control changed', name, field)
            row = {'name': name, 'checks': checks.count, 'failures': checks.failures,
                   'maximum_phase_difference_turn': str(maximum),
                   'log10rms': [{'segment': i, 'rust': m['log10rms'], 'native': str(ref['log10rms', i]),
                                'same_binary64_rounding_cell': float(m['log10rms']) == float(ref['log10rms', i])}
                               for i, m in enumerate(data['result']['models']['segments'])],
                   'artifacts': {str(x.relative_to(case)): sha(x) for x in case.rglob('*') if x.is_file()}}
            variant['cases'].append(row)
            write(out / 'variants.json', records)
            print(bits, name, 'checks', checks.count, 'failures', len(checks.failures), 'maximum phase difference', maximum, flush=True)
        if bits == 128:
            failures = [x for c in variant['cases'] for x in c['failures']]
            assert len(failures) == 7 and all(x['field'].endswith('.log10rms') for x in failures)
    for path, expected in protected.items():
        assert sha(Path(path)) == expected, ('protected input changed', path)
    for name, expected in sources.items():
        if name != 'rust/src/longdouble.rs':
            assert sha(snapshot / name) == expected, ('unexpected snapshot change', name)
    receipt = {'scope': 'Central AF diagnosis only; unchanged comparator, no production precision switch or milestone acceptance.',
               'precision_change': 'Only longdouble::PRECISION_BITS, 128 to 113. Ordinary PredictorNumber and existing literal-128 constructors remain unchanged. MPFR113 is not a complete IEEE binary128 emulation.',
               'review_source_sha256': sha(Path(__file__)), 'archives': archives, 'native_sha256': sha(native),
               'compiler': subprocess.check_output([rustc, '-vV'], text=True),
               'environment': {k: env[k] for k in ['PATH', 'TEMPO2', 'CARGO_PKG_VERSION', 'CARGO_MANIFEST_DIR', 'CARGO_PKG_NAME']},
               'dependency_archives': {str(x): sha(x) for x in [*externs.values(), sofa]},
               'protected_files_verified': len(protected), 'protected_manifest_sha256': sha(out / 'protected-before.json'),
               'source_manifest_sha256': sha(out / 'source-identities.json'), 'commands': commands, 'variants': records}
    write(out / 'receipt.json', receipt)
    print('RECEIPT', out / 'receipt.json', sha(out / 'receipt.json'), flush=True)

if __name__ == '__main__':
    main()
