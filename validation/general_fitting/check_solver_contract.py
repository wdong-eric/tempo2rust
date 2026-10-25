"""Native optional outputs, robust histories, diagnostic files and shared inputs.

Uses pinned corrected TKfit/TKrobust bodies and independently parsed binary64
input. Quantities retain 0007/0071 gates; generated records are never acceptance.
"""
from pathlib import Path
from decimal import Decimal as D, getcontext
import argparse, hashlib, json, os, subprocess
from fitting_corrections import corrected
getcontext().prec = 80
ROOT = Path(__file__).resolve().parents[2];HERE = Path(__file__).resolve().parent
BUILD = HERE / 'build/solver-contract';REF = ROOT / 'validation/reference/build'
PIN = 'fda6ad4a9753ad7dc5d291406f2f310a9161423f'
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()


def build():
    BUILD.mkdir(parents=True, exist_ok=True)
    commands = [];objects = []
    for name in ['TKfit.C', 'TKrobust.C']:
        text = corrected(name, subprocess.check_output(['git', '-C', str(ROOT / 'tempo2'), 'show', PIN + ':' + name]).decode())
        if name == 'TKfit.C':
            token = 'TKsingularValueDecomposition_lsq(augmented_DM,ndata+nconstraints,nparams,v,w,u);'
            assert text.count(token) == 1
            text = text.replace(token, token + '\nextern void ae_solver_svd(int,const longdouble*);ae_solver_svd(nparams,w);')
            token = '    if(computeErrors && rescale_errors){'
            assert text.count(token) == 1
            text = text.replace(token, '''    extern void ae_solver_details(int,int,int,const double*,bool,bool,double,const double*,bool,const double*,double**);
    ae_solver_details(ndata,nparams,nconstraints,e,computeErrors,rescale_errors,chisq,outP,computeParam,data,designMatrix);
''' + token)
        else:
            text = 'extern void ae_robust_step(int,int,double,double,double,double,double*,double**,double*,double*,double*,double**,int,int);\n' + text
            token = 'sigma = median_abs_deviation/0.6745;'
            assert text.count(token) == 1
            text = text.replace(token, token + '\nae_robust_step(count,0,sigma_old,sigma,count?sum_phi_sq:0,count?sum_dphi:0,modified_data,Mw,resid,outP,e,cvm,ndata,nparams);')
            token = '    double robust_chisq ='
            assert text.count(token) == 1
            text = text.replace(token, '    ae_robust_step(count,1,sigma,0,sum_phi_sq,sum_dphi,modified_data,Mw,resid,outP,e,Ocvm,ndata,nparams);\n' + token)
            token = '    if (rescale_errors)ecorrect = robust_chisq / nfree;'
            assert text.count(token) == 1
            text = text.replace(token, token + '\n    extern void ae_solver_multiplier(double);ae_solver_multiplier(sqrt(ecorrect));')
        src = BUILD / name;src.write_text(text);obj = src.with_suffix('.o');objects.append(obj)
        cmd = ['/opt/homebrew/bin/g++-16', '-O2', '-std=gnu++11', '-DHAVE_CONFIG_H', '-I' + str(REF / 'source'), '-c', str(src), '-o', str(obj)]
        subprocess.run(cmd, capture_output=True, check=True);commands.append(cmd)
    native = BUILD / 'native'
    cmd = [commands[0][0], '-O2', '-std=gnu++11', '-DHAVE_CONFIG_H', '-I' + str(REF / 'source'), str(HERE / 'solver_file.cc'), *map(str, objects), str(REF / 'libtempo2-reference.a'), '-lquadmath', '-lgfortran', '-o', str(native)]
    subprocess.run(cmd, capture_output=True, check=True);commands.append(cmd)
    return native, commands


def compare(output, got, matrix, data, mask, has_data, mode=0):
    failures = []
    def check(name, actual, expected, gate):
        actual = D(actual);expected = D(expected)
        if not actual.is_finite() or not expected.is_finite():
            equal = ((actual.is_nan() and expected.is_nan()) or actual == expected) and actual.is_signed() == expected.is_signed()
        else:
            equal = abs(actual - expected) <= gate
            if expected == 0: equal &= actual == 0 and actual.is_signed() == expected.is_signed()
        if not equal: failures.append({'quantity': name, 'rust': str(actual), 'native': str(expected), 'gate': str(gate)})
    lines = [line.split() for line in output.splitlines()]
    modes = [];history = [];current = None
    final = {}
    for r in lines:
        if r[0] == 'S':
            if int(r[1]) == 0: modes.append([])
            modes[-1].append(r[2])
        elif r[0] == 'R':
            current = {'head': r[1:], 'rows': []};history.append(current)
        elif r[0] in ['Y', 'A', 'I', 'K']: current['rows'].append(r)
        else: final[tuple(r[:-1])] = r[-1]
    scales = [max(abs(D(row[j])) for row in matrix) for j in range(len(matrix[0]))]
    errors = []
    for j, scale in enumerate(scales):
        if mask & 1 and has_data:
            ref = D(final['P', str(j)]);actual = D(got['increments'][j]);errors.append(actual - ref)
            check(f'p.{j}', actual * scale, ref * scale, D('1e-13') + D('1e-11') * abs(ref * scale))
        elif mask & 1: assert final['P', str(j)] == '-9876'
        if mask & 2: check(f'e.{j}', got['errors'][j], final['E', str(j)], D('1e-10') * abs(D(final['E', str(j)])))
        if mask & 4:
            for k in range(len(scales)):
                norm = (D(final['C', str(j), str(j)]) * D(final['C', str(k), str(k)])).sqrt()
                check(f'c.{j}.{k}', got['covariance'][j][k], final['C', str(j), str(k)], D('1e-10') * norm)
    assert len(got['increments']) == (len(scales) if mask & 1 and has_data else 0)
    assert len(got['errors']) == (len(scales) if mask & 2 else 0)
    assert len(got['covariance']) == (len(scales) if mask & 4 else 0)
    if errors:
        for i, row in enumerate(matrix):
            delta = sum(D(x) * e for x, e in zip(row, errors))
            if abs(delta) > D('1e-14'): failures.append({'quantity': f'joint_increment.{i}', 'difference': str(delta), 'gate': '1e-14'})
    if ('M',) in final:
        check('error_multiplier', got['error_multiplier'], final['M',], D('5e-14') * abs(D(final['M',])))
        assert len(got['errors_unscaled']) == (len(scales) if mask & 2 else 0)
        for j, value in enumerate(got['errors_unscaled']):
            check(f'errors_unscaled.{j}', value, final['U', str(j)], D('1e-10') * abs(D(final['U', str(j)])))
        assert len(got['linearized_s']) == (len(data) if mask & 1 and has_data else 0)
        for j, value in enumerate(got['linearized_s']):
            check(f'linearized.{j}', value, final['L', str(j)], D('1e-12'))
    check('chi', got['chi_square'], final['CHI',], D('1e-10') * abs(D(final['CHI',])) + D('1e-12' if mode==1 else '1e-24'))
    assert len(got['singular_values']) == len(modes[-1])
    for j, value in enumerate(modes[-1]): check(f'raw_mode.{j}', got['singular_values'][j], value, D('1e-10') * abs(D(value)))
    assert len(history) == len(got['robust_history'])
    for i, (ref, actual) in enumerate(zip(history, got['robust_history'])):
        count, final_step, scale, after, phi, dphi = ref['head']
        assert actual['iteration'] == int(count) and actual['phase'] == ('final' if int(final_step) else 'scale')
        for key, value in [('scale', scale), ('next_scale', after), ('phi', phi), ('dphi', dphi)]:
            if actual[key] is None:
                assert (key == 'next_scale' and int(final_step)) or (i == 0 and key in ['phi', 'dphi'])
            else: check(f'history.{i}.{key}', actual[key], value, D('5e-14') * max(D(1), abs(D(value))))
        for r in ref['rows']:
            j = int(r[1]);kind = r[0]
            if kind == 'Y':
                for key, value in [('white_data', r[2]), ('residual', r[3])]:
                    check(f'history.{i}.{key}.{j}', actual[key][j], value, D('5e-14') * max(abs(D(v)) for v in (a[2 if key == 'white_data' else 3] for a in ref['rows'] if a[0] == 'Y')))
            elif kind == 'A':
                k = int(r[2]);norm = max(abs(D(a[3])) for a in ref['rows'] if a[0] == 'A' and int(a[2]) == k)
                check(f'history.{i}.matrix.{j}.{k}', actual['white_matrix'][j][k], r[3], D('5e-14') * norm)
            elif kind == 'I':
                check(f'history.{i}.p.{j}', D(actual['solver']['increments'][j]) * scales[j], D(r[2]) * scales[j], D('1e-13') + D('1e-11') * abs(D(r[2]) * scales[j]))
                check(f'history.{i}.e.{j}', actual['solver']['errors'][j], r[3], D('1e-10') * abs(D(r[3])))
            elif kind == 'K':
                k = int(r[2]);cov = {(int(a[1]), int(a[2])): D(a[3]) for a in ref['rows'] if a[0] == 'K'}
                check(f'history.{i}.cov.{j}.{k}', actual['solver']['covariance'][j][k], r[3], D('1e-10') * (cov[j, j] * cov[k, k]).sqrt())
        for j, value in enumerate(modes[i]): check(f'history.{i}.raw_mode.{j}', actual['solver']['singular_values'][j], value, D('1e-10') * abs(D(value)))
    return failures


def run(profile):
    native, commands = build();rust = ROOT / 'rust/target' / profile / 'examples/fitting_solver'
    out = BUILD / profile;out.mkdir(exist_ok=True)
    records = []
    matrix = [[1., float(i - 3)] for i in range(7)];data = [3., 5., 7., 9., 11., 13., 100.]
    white = [1., .5, 2., .25, .5, 1., 2.]
    cases = [(f'optional-{rescale}-{mask}-{present}', mask, present, '0', rescale, 0, False) for rescale in [0, 1] for mask in range(8) for present in [0, 1]]
    cases += [(f'robust-{kind}-{rescale}', 7, 1, kind, rescale, 7, False) for kind in ['H', 'B', 'W', 'Z'] for rescale in [0, 1]]
    cases += [('files', 7, 1, '0', 1, 7, True), ('files-errors-only', 2, 0, '0', 0, 7, False)]
    for name, mask, present, kind, rescale, flags, debug in cases:
        case = out / name;case.mkdir(exist_ok=True)
        dirs = {e: case / e for e in ['native', 'rust']}
        for path in dirs.values(): path.mkdir(exist_ok=True)
        text = f'7 2 1 {rescale}\n' + ''.join(' '.join(map(repr, [y, y * weight, row[0], row[0] * weight, row[1], row[1] * weight])) + '\n' for row, y, weight in zip(matrix, data, white)) + '1.0 0.0 0.5\n'
        path = case / 'input.txt';path.write_text(text)
        trace = dirs['native'] / 'observations.txt'
        cmds = [[str(native), str(path), str(trace), str(mask), str(present), kind, str(flags), str(int(debug))], [str(rust), str(path), str(mask), str(present), kind, str(dirs['rust']), str(flags), str(int(debug))]]
        for engine, cmd in zip(dirs, cmds):
            proc = subprocess.run(cmd, cwd=dirs[engine], capture_output=True, timeout=120)
            (dirs[engine] / 'stdout').write_bytes(proc.stdout);(dirs[engine] / 'stderr').write_bytes(proc.stderr)
            assert proc.returncode == 0, (name, engine, proc.returncode, proc.stderr.decode())
        got = json.loads((dirs['rust'] / 'stdout').read_text())
        failures = compare(trace.read_text(), got, matrix, data, mask, present)
        file_names = [p.name for p in dirs['native'].iterdir() if p.name not in ['stdout', 'stderr', 'observations.txt']]
        assert set(file_names) == {Path(p).name for p in got['diagnostic_files']}
        for f in file_names:
            if (dirs['native'] / f).read_bytes() != (dirs['rust'] / f).read_bytes(): failures.append({'quantity': f, 'kind': 'file_bytes'})
        records.append({'case': name, 'commands': cmds, 'input_sha256': sha(path), 'native_output_sha256': sha(trace), 'rust_output_sha256': sha(dirs['rust'] / 'stdout'), 'failures': failures})
    receipt = {'pin': PIN, 'profile': profile, 'commands': commands, 'builder_sha256': sha(__file__), 'probe_sha256': sha(HERE / 'solver_file.cc'), 'native_sha256': sha(native), 'rust_sha256': sha(rust), 'records': records}
    (out / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
    print('solver contract cases', len(records), 'failures', sum(len(r['failures']) for r in records))
    for r in records:
        if r['failures']: print(r['case'], len(r['failures']), r['failures'][:2])


if __name__ == '__main__':
    p = argparse.ArgumentParser();p.add_argument('--profile', choices=['debug', 'release'], default='debug')
    run(p.parse_args().profile)
