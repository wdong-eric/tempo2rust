#!/usr/bin/env python3
"""Observe event temporaries and test one counterfactual; never replace the oracle."""
import argparse
import difflib
import json
import os
from pathlib import Path
from common import ROOT, Run, sha
from check_timing import compare


def observed(source):
    for name, statement, fields in [
        ('EXP', 'phase4 +=  dm*powl(freq, gamma)*exp(-dt/tau)*psr[p].param[param_f].val[0];', ['freq', 'dt', 'dm', 'tau', 'gamma']),
        ('GAUS', 'phase4 += val*psr[p].param[param_f].val[0];', ['freq', 'dt', 'amp', 'sig', 'gamma', 'val']),
    ]:
        assert source.count(statement) == 1
        trace = (' { char al_phase[128]; quadmath_snprintf(al_phase, sizeof al_phase, "%.36Qg", phase4); '
                 'fprintf(stderr, "AL_EVENT_' + name + ' %d %d' + ' %.21Lg' * len(fields) + ' %s\\n", i, k, '
                 + ', '.join('(long double)' + f for f in fields) + ', al_phase); }')
        source = source.replace(statement, statement + trace)
    return source


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ['out', 'core', 'science', 'timing', 'runtime', 'rust']:
        parser.add_argument('--' + name, type=Path, required=True)
    parser.add_argument('--baseline-rust',type=Path,help='Retained pre-correction executable for exact macOS before/after products')
    a = parser.parse_args()
    run = Run(a.out)
    core, science, timing = a.core.resolve(), a.science.resolve(), a.timing.resolve()
    original = core/'source/formResiduals.C'
    source = original.read_text()
    narrow = source
    # formResiduals.C:675,705 use ordinary long double. This counterfactual
    # isolates the historical 4accd26 Rust f64 temporary boundary; it is not an approved
    # native correction or a change to the production Rust precision policy.
    for declaration in ['long double freq, dt, dm, tau, gamma;', 'long double freq, dt, amp, sig, gamma,val;']:
        assert narrow.count(declaration) == 1
        narrow = narrow.replace(declaration, declaration.replace('long double', 'double'))
    core_id = json.loads((core/'receipt.json').read_text())
    science_id = json.loads((science/'receipt.json').read_text())
    compile_command = next(c['argv'] for c in core_id['commands'] if c['label'] == 'compile-formResiduals.C')
    link_command = next(c['argv'] for c in science_id['commands'] if c['label'] == 'timing-link')
    executables = {'original': science/'timing'}
    for name, text in [('observed', observed(source)), ('f64-temporaries', observed(narrow))]:
        path = run.out/(name+'.C'); obj = run.out/(name+'.o'); exe = run.out/name
        path.write_text('#line 1 "'+str(original)+'"\n'+text)
        (run.out/(name+'.patch')).write_text(''.join(difflib.unified_diff(source.splitlines(True), text.splitlines(True), fromfile='original/formResiduals.C', tofile=name+'/formResiduals.C')))
        run.run([str(path) if v==str(original) else str(obj) if v==str(core/'formResiduals.C.o') else v for v in compile_command], name+'-compile')
        run.run([str(obj) if v==str(core/'formResiduals.C.o') else str(exe) if v==str(science/'timing') else v for v in link_command], name+'-link')
        executables[name] = exe
    fixture = ROOT/'validation/timing_models/fixtures/events.par'
    control = run.out/'without-events.par'
    control.write_text(''.join(line for line in fixture.read_text().splitlines(True) if not line.startswith(('EXP', 'GAUS'))))
    gaussian=run.out/'gaussian-only.par'
    gaussian.write_text(''.join(line for line in fixture.read_text().splitlines(True)
        if not line.startswith(('EXP','GAUSINDEX','GAUSAMP')))+'GAUSAMP_1 123456789.123456789123\n')
    records = []; invariant_failures = []
    product_controls=[]
    for case, par in [('events', fixture), ('without-events', control), ('gaussian-only',gaussian)]:
        for mean in [0, 1]:
            outputs = {}
            for variant, exe in executables.items():
                tag = f'{case}-{mean}-{variant}'; work = run.out/tag; work.mkdir(); table = work/'native.tsv'
                run.run([exe, table, mean, 1, par, fixture.with_suffix('.tim')], tag,
                        cwd=work, env=dict(os.environ, TEMPO2=str(a.runtime.resolve()), LC_ALL='C'))
                outputs[variant] = table.read_text()
            # The new ordinary-width witness is separate from the unchanged
            # historical fixture and its original gate. Compare installed output.
            if case=='events':rust=timing/f'events-mean{mean}/rust.json'
            else:
                rust=run.out/f'{case}-{mean}-rust.json'
                command=[a.rust.resolve(),'timing','--par',par,'--tim',fixture.with_suffix('.tim'),
                         '--runtime',a.runtime.resolve(),*(['--no-mean'] if mean==0 else [])]
                rust.write_bytes(run.run(command,f'{case}-{mean}-rust',cwd=run.out))
            candidate=json.loads(rust.read_text())
            errors,measurements=compare(outputs['original'],candidate)
            records.append(dict(case=case,mean=mean,variant='original',rust_errors=errors,measurements=measurements,rust_output_sha256=sha(rust)))
            if a.baseline_rust:
                command=[a.baseline_rust.resolve(),'timing','--par',par,'--tim',fixture.with_suffix('.tim'),
                         '--runtime',a.runtime.resolve(),*(['--no-mean'] if mean==0 else [])]
                before=run.run(command,f'{case}-{mean}-baseline',cwd=run.out)
                equal=json.loads(before)==candidate
                product_controls.append(dict(case=case,mean=mean,exact_before_after=equal))
                if not equal:invariant_failures.append(f'{case}-{mean}-macos-before-after')
            for variant in ['observed', 'f64-temporaries']:
                equal = outputs['original'] == outputs[variant]
                if not equal and (variant=='observed' or case=='without-events'):
                    invariant_failures.append(f'{case}-{mean}-{variant}')
                row = dict(case=case, mean=mean, variant=variant, native_state_equal=equal)
                errors, measurements = compare(outputs[variant], candidate)
                row.update(rust_errors=errors, measurements=measurements, rust_output_sha256=sha(rust))
                records.append(row)
            if case=='events':
                assert outputs['original'] == (timing/f'events-mean{mean}/native.tsv').read_text()
    scientific_failures=[r for r in records if r['variant']=='original' and r['rust_errors']]
    run.save(dict(status='failure' if invariant_failures or scientific_failures else 'pass', invariant_failures=invariant_failures,
                  scientific_failures=scientific_failures,
                  scope='Diagnostic execution and observer neutrality only; f64-temporaries is an unaccepted counterfactual. The original timing scientific gate is unchanged.',
                  records=records,product_controls=product_controls,
                  historical_f64_profile='4accd26e2d51b3dc805d9a1ae0a2be7195e171ea',
                  gaussian_scope='Large-amplitude default-index numerical witness of ordinary val narrowing; not an astrophysical calibration',
                  inputs={str(p):sha(p) for p in [original, core/'receipt.json', science/'receipt.json', fixture, fixture.with_suffix('.tim'),gaussian,a.rust.resolve(),
                      *([a.baseline_rust.resolve()] if a.baseline_rust else []),Path(__file__)]}))
    raise SystemExit(bool(invariant_failures or scientific_failures))
