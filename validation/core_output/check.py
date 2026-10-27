"""AH exact-output checks. Any required mismatch or unexpected status exits 1.

python3 check.py --native build/native-final --out build/debug-final [--release]
python3 test_check.py  # corruption and execution-status rejection checks
Raw outputs, numeric failures and receipts are retained under the requested out.
"""
from pathlib import Path
import argparse
import difflib
import hashlib
import json
import os
import pty
import subprocess
import termios

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def file_hashes(directory):
    return {str(p.relative_to(directory)): sha(p)
            for p in sorted(directory.rglob('*')) if p.is_file()}


def compare(left, right, normalize=lambda data, kind: data, diff_dir=None):
    """Compare complete file membership and bytes, including ordered records."""
    lf = {str(p.relative_to(left)) for p in left.rglob('*') if p.is_file()}
    rf = {str(p.relative_to(right)) for p in right.rglob('*') if p.is_file()}
    different = []
    for name in sorted(lf & rf):
        l = (left / name).read_bytes()
        r = (right / name).read_bytes()
        if name in ['stdout', 'stderr']:
            l = normalize(l, 'native')
            r = normalize(r, 'rust')
        if l != r:
            different.append(name)
            if diff_dir is not None:
                dest = diff_dir / (name + '.diff')
                dest.parent.mkdir(parents=True, exist_ok=True)
                dest.write_text(''.join(difflib.unified_diff(
                    l.decode(errors='replace').splitlines(True),
                    r.decode(errors='replace').splitlines(True))))
    return {'native_only': sorted(lf - rf), 'rust_only': sorted(rf - lf), 'different': different}


def summary(comparisons, runs):
    counts = {
        'unexpected_status': sum(r['status'] != r['expected'] for r in runs),
        'changed_inputs': sum(r.get('inputs_unchanged') is False for r in runs),
        'artifact_mismatches': sum(len(names) for r in runs
                                   for names in r.get('artifact_check', {}).values()),
        'different_files': sum(len(c['different']) for c in comparisons.values()),
        'missing_files': sum(len(c['native_only']) for c in comparisons.values()),
        'extra_files': sum(len(c['rust_only']) for c in comparisons.values()),
    }
    counts['failed_cases'] = sum(any(c.values()) for c in comparisons.values())
    counts['exit_status'] = int(any(counts.values()))
    return counts


def publish(manifest, path):
    """Recheck completed run artifacts immediately before publishing their receipt."""
    for record in manifest['runs']:
        if 'directory' not in record:
            continue  # The source/runtime identity entry is not an execution.
        current = file_hashes(Path(record['directory']))
        retained = record['files']
        record['artifact_check'] = {
            'missing': sorted(retained.keys() - current.keys()),
            'extra': sorted(current.keys() - retained.keys()),
            'changed': sorted(name for name in retained.keys() & current.keys()
                              if retained[name] != current[name]),
        }
    counts = summary(manifest['comparison'], manifest['runs'])
    manifest['summary'] = counts
    path.write_text(json.dumps(manifest, indent=2) + '\n')
    print('SUMMARY', counts, flush=True)
    return counts['exit_status']


def run(command, directory, env, expected=0, timeout=75):
    directory.mkdir(parents=True, exist_ok=True)
    inputs = {str(p): sha(p) for value in command[1:] if (p := Path(str(value))).is_absolute() and p.is_file()}
    try:
        r = subprocess.run(list(map(str, command)), cwd=directory, env=env,
                           capture_output=True, timeout=timeout)
        code, stdout, stderr = r.returncode, r.stdout, r.stderr
    except subprocess.TimeoutExpired as e:
        code, stdout, stderr = 'timeout', e.stdout or b'', e.stderr or b''
    except OSError as e:
        code, stdout, stderr = 'launch-error', b'', str(e).encode()
    (directory / 'stdout').write_bytes(stdout)
    (directory / 'stderr').write_bytes(stderr)
    return {'command': list(map(str, command)), 'directory': str(directory.resolve()),
            'status': code, 'expected': expected,
            'inputs': inputs, 'inputs_unchanged': all(Path(p).is_file() and sha(Path(p)) == value for p, value in inputs.items()),
            'files': file_hashes(directory)}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--native', type=Path)
    parser.add_argument('--out', type=Path)
    parser.add_argument('--release', action='store_true')
    parser.add_argument('--compare', nargs=2, type=Path,
                        help='exact comparison of two retained output directories')
    args = parser.parse_args()
    if args.compare:
        result = {'comparison': compare(*args.compare)}
        counts = summary(result, [])
        print(json.dumps({'checks': result, 'summary': counts}, indent=2))
        return counts['exit_status']
    if args.native is None or args.out is None:
        parser.error('--native and --out are required for the workflow matrix')
    out = args.out.resolve()
    out.mkdir(parents=True, exist_ok=False)
    native = args.native.resolve()
    rust = ROOT / 'rust/target' / ('release' if args.release else 'debug') / 'tempo2-rust'
    runtime = ROOT / 'validation/reference/build/source/T2runtime'
    env = dict(os.environ, TEMPO2=str(runtime))
    # Full scoped runtime and Rust-source identities, before/after the executions.
    runtime_before = {str(p.relative_to(runtime)): sha(p) for p in runtime.rglob('*') if p.is_file()}
    rust_sources = [*sorted((ROOT / 'rust/src').rglob('*.rs')),
                    *sorted((ROOT / 'rust/data').glob('*.txt')),
                    *[ROOT / 'rust' / p for p in ['Cargo.toml', 'Cargo.lock', 'build.rs']]]
    source_before = {str(p.relative_to(ROOT)): sha(p) for p in rust_sources}
    receipts, comparisons = [], {}
    maps = {}
    for p in (native / 'source').glob('*'):
        original = ROOT / 'tempo2' / p.name
        if original.exists():
            m = {}
            for block in difflib.SequenceMatcher(a=original.read_text().splitlines(),
                    b=p.read_text().splitlines(), autojunk=False).get_matching_blocks():
                for i in range(block.size):
                    m[block.b + i + 1] = block.a + i + 1
            maps[p.name] = m

    def normal(data, kind):
        import re
        # Only named source-location and CPU-time metadata; every numeric datum stays.
        def location(m):
            name, line = Path(m[1].decode()).name, int(m[2])
            if name == 'bounded_main.C':
                name = 'tempo2.C'
            if kind == 'native' and str(native).encode() in m[1]:
                line = maps.get(name, {}).get(line, line)
            return f'[{name}:{line}]'.encode()
        data = re.sub(rb'(?m)^\[([^\]\n]+\.(?:C|h)):(\d+)\]', location, data)
        return re.sub(rb'(?m)^Finishing off: time taken = [0-9.]+ \(s\)$',
                      b'Finishing off: time taken = <CPU> (s)', data)

    def execute(kind, name, command, directory, expected=0):
        receipt = run(command, directory, env, expected)
        receipts.append(dict(receipt, kind=kind, case=name))
        return receipt['status']

    def check(name, left, right):
        comparisons[name] = compare(left, right, normal, out / 'diffs' / name)
        print(name, comparisons[name], flush=True)

    for mode in ['format', 'helpers', 'log', 'snapshot']:
        extra = [HERE / 'inputs/snapshot.par'] if mode == 'snapshot' else []
        execute('native', mode, [native / 'probe', mode, *extra], out / 'probes' / mode)
    execute('original', 'helpers', [native / 'probe-original', 'helpers'], out / 'original-helpers')

    # _TKchklog uses stdout's actual terminal, including for stderr. Disable only
    # terminal newline translation; ANSI bytes remain evidence, not normalization.
    master, slave = pty.openpty()
    attrs = termios.tcgetattr(slave)
    attrs[1] &= ~termios.ONLCR
    termios.tcsetattr(slave, termios.TCSANOW, attrs)
    terminal = out / 'probes/terminal'
    terminal.mkdir()
    proc = subprocess.Popen([str(native / 'probe'), 'log'], stdout=slave, stderr=subprocess.PIPE, env=env)
    os.close(slave)
    buf = bytearray()
    while True:
        try:
            block = os.read(master, 8192)
        except OSError as exc:
            import errno
            if exc.errno != errno.EIO:
                raise
            break
        if not block:
            break
        buf.extend(block)
    os.close(master)
    err = proc.communicate(timeout=10)[1]
    (terminal / 'stdout').write_bytes(buf)
    (terminal / 'stderr').write_bytes(err)
    receipts.append({'kind': 'native', 'case': 'terminal', 'status': proc.returncode, 'expected': 0,
                     'directory': str(terminal), 'files': file_hashes(terminal)})

    probe_dir = out / 'rust-probe'
    command = ['cargo', 'test', '--manifest-path', str(ROOT / 'rust/Cargo.toml')]
    command += ['--release'] if args.release else []
    command += ['core_output_tests::write_oracle_evidence', '--', '--ignored', '--test-threads=1']
    receipt = run(command, out / 'probe-cargo', dict(env, AH_PROBE_DIR=str(probe_dir)))
    receipts.append(dict(receipt, kind='rust', case='direct-api-recovery'))
    for mode in ['format', 'helpers', 'log', 'terminal']:
        dest = probe_dir / mode
        dest.mkdir(parents=True, exist_ok=True)
        for channel in ['stdout', 'stderr']:
            source = probe_dir / (mode + '.' + channel)
            if source.exists():
                (dest / channel).write_bytes(source.read_bytes())
            elif channel == 'stderr' and mode in ['format', 'helpers']:
                (dest / channel).write_bytes(b'')
        check('probe-' + mode, out / 'probes' / mode, dest)
    check('snapshot', out / 'probes/snapshot', probe_dir / 'snapshot')

    ae = ROOT / 'validation/general_fitting/inputs'
    ad = ROOT / 'validation/timing_models/fixtures'
    af = ROOT / 'validation/predictors/fixtures'
    tim = ae / 'multi-frequency.tim'
    variants = out / 'inputs'
    variants.mkdir()
    base = (ae / 'robust-H.par').read_text()
    (variants / 'unweighted.par').write_text(base.replace('MODE 1', 'MODE 0').replace('ROBUST H', ''))
    (variants / 'nits.par').write_text(base.replace('NITS 1', 'NITS 2'))
    (variants / 'standalone.par').write_text(base)
    (variants / 'standalone.tim').write_bytes(tim.read_bytes())
    for name in ['waves', 'wave-dm-scale']:
        s = (ad / (name + '.par')).read_text()
        s = s.replace('WAVE_OM 0.3', 'WAVE_OM 0.3 1').replace('WAVDM_OM 0.4', 'WAVDM_OM 0.4 1')
        (variants / (name + '-fit.par')).write_text(s + '\nMODE 1\n')
    terrestrial = ROOT / 'validation/fitting/inputs'
    (variants / 'terrestrial-tempo1.par').write_text((terrestrial / 'pks_all.par').read_text().replace('EPHVER 5', 'EPHVER 2').replace('NITS 2', 'NITS 1'))
    cases = [
        ('robust', ae / 'robust-H.par', tim, ['-newpar', '-residuals']),
        ('unweighted', variants / 'unweighted.par', tim, ['-newpar']),
        ('pre-list-bary-unwritten', ae / 'robust-H.par', tim, ['-pre', '-list', '-newpar']),
        ('pre-list-tempo1', variants / 'terrestrial-tempo1.par', terrestrial / 'pks.tim', ['-pre', '-list', '-newpar']),
        ('pre-list-terrestrial', terrestrial / 'pks_all.par', terrestrial / 'pks.tim', ['-pre', '-list', '-newpar']),
        ('nits', variants / 'nits.par', tim, ['-newpar']),
        ('nits-pre', variants / 'nits.par', tim, ['-pre', '-newpar']),
        ('writetim', ae / 'robust-H.par', tim, ['-writetim', '-pre']),
    ]
    for name in ['dm-grid', 'parameter-prior', 'red-noise', 'dm-noise', 'band-noise', 'chromatic-noise']:
        cases.append((name, ae / (name + '.par'), tim, ['-newpar']))
    for name in ['ell1h-h4', 't2-h4', 'jumps-fd', 'glitches', 'ifunc-linear', 'clock-offsets',
                 'quad-interpolation', 'waves', 'wave-dm-scale', 'gw-memory']:
        cases.append((name, ad / (name + '.par'), ad / (name + '.tim'), ['-newpar', '-residuals']))
    for name in ['waves', 'wave-dm-scale']:
        cases.append((name + '-fit', variants / (name + '-fit.par'), tim, ['-newpar']))
    cases.append(('global', ae / 'global0.par', tim, ['-f', str(ae / 'global1.par'), str(tim),
                  '-dcf', str(ae / 'covariance.model'), '-outpar', 'fitted.par']))
    for name, par, timfile, options in cases:
        for kind, exe in [('native', native / 'tempo2'), ('rust', rust)]:
            boundary = name in ['pre-list-bary-unwritten', 'waves', 'wave-dm-scale'] and kind == 'rust'
            execute(kind, name, [exe, '-npsr', '2', '-nobs', '100', '-noaccel', '-f', par, timfile, *options],
                    out / name / kind, int(boundary))
        check(name, out / name / 'native', out / name / 'rust')

    for kind, exe in [('native', native / 'tempo2-bounded'), ('rust', rust)]:
        execute(kind, 'standalone', [exe, variants / 'standalone.tim'], out / 'standalone' / kind)
        execute(kind, 'reload', [exe, '-noaccel', '-f', out / 'robust' / kind / 'new.par', tim,
                '-nofit', '-pre', '-newpar'], out / 'reload' / kind)
    for name in ['standalone', 'reload']:
        check(name, out / name / 'native', out / name / 'rust')

    for name, par, options in [
        ('cheby-odd-defaults', af / 'defaults.par', ['-pred', '@ 58000 58000.01 1000 2000 7 3 3600']),
        ('cheby-ecliptic-tdb', af / 'ecliptic_tdb.par', ['-pred', '@ 58000 58000.01 1000 2000 8 4 3600']),
        ('polyco-prefix', af / 'isolated.par', ['-polyco', '58000 58000.01 60 8 12 @ 1400', '-polyco_file', 'literal-']),
    ]:
        for kind, exe in [('native', native / 'tempo2'), ('rust', rust)]:
            execute(kind, name, [exe, '-npsr', '2', '-nobs', '10000', '-f', par, *options], out / name / kind)
        check(name, out / name / 'native', out / name / 'rust')

    for name, flag in [('version', '-version'), ('help', '-h'), ('help-extra', '-H')]:
        execute('rust', flag, [rust, flag], out / 'controls' / name, 1)
    for name, options in [('missing-f', ['-f']), ('fit-plugin', ['-fitfunc', 'absent']),
                          ('output-plugin', ['-output', 'absent']), ('graphics', ['-gr', 'absent'])]:
        command = [rust, *options] if name == 'missing-f' else [rust, '-f', ae / 'robust-H.par', tim, *options]
        execute('rust', name, command, out / 'controls' / name, 1 if name in ['missing-f', 'fit-plugin'] else 255)
    runtime_after = {str(p.relative_to(runtime)): sha(p) for p in runtime.rglob('*') if p.is_file()}
    source_after = {str(p.relative_to(ROOT)): sha(p) for p in rust_sources}
    receipts.append({'kind': 'identity', 'case': 'runtime-and-source-unchanged',
                     'status': int(runtime_before != runtime_after or source_before != source_after), 'expected': 0})
    manifest = {'contract_sha256': sha(HERE / 'CONTRACT.md'), 'checker_sha256': sha(Path(__file__)),
        'rust': sha(rust), 'native': sha(native / 'tempo2'), 'native_identity': sha(native / 'identity.json'),
        'comparison': comparisons, 'runs': receipts,
        'runtime_before': runtime_before, 'runtime_after': runtime_after,
        'rust_sources_before': source_before, 'rust_sources_after': source_after,
        'note': 'All exact display/numerical failures count. No inherited gate is waived.'}
    return publish(manifest, out / 'receipt.json')


if __name__ == '__main__':
    raise SystemExit(main())
