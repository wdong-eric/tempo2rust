#!/usr/bin/env python3
"""Observe event temporaries and test one counterfactual; never replace the oracle."""
import argparse
import difflib
import json
import os
from pathlib import Path
import shutil
from common import ROOT, Run, sha
from check_timing import compare
from spin_profile import equal as binary_equal
from decimal import Decimal


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


def detailed_native(source):
    for kind,statement,amp,width,exponent,increment in [
        ('EXP','phase4 +=  dm*powl(freq, gamma)*exp(-dt/tau)*psr[p].param[param_f].val[0];','dm','tau','-dt/tau','dm*powl(freq,gamma)*exp(-dt/tau)*psr[p].param[param_f].val[0]'),
        ('GAUS','phase4 += val*psr[p].param[param_f].val[0];','amp','sig','-dt*dt/2./sig/sig','val*psr[p].param[param_f].val[0]'),
    ]:
        assert source.count(statement)==1
        fields=[('ordinary_bits','__LDBL_MANT_DIG__'),('freq','freq'),('dt','dt'),('amp',amp),
            ('width',width),('index','gamma'),('exponent',exponent),('exp','exp('+exponent+')'),
            ('power','powl(freq,gamma)'),*([('val','val')] if kind=='GAUS' else []),
            ('increment',increment),('phase_before','al_before'),('phase_after','phase4')]
        trace=' { longdouble al_before=phase4; '+statement
        for key,value in fields:
            trace+=' { char al_text[128]; quadmath_snprintf(al_text,sizeof al_text,"%.40Qg",(longdouble)('+value+')); '
            trace+='fprintf(stderr,"AL_DETAIL '+kind+' %d %d '+key+' %s\\n",i,k,al_text); }'
        source=source.replace(statement,trace+' }')
    return observed(source)


def detailed_rust(source):
    for kind,amp,width in [('EXP','dm','tau'),('GAUS','amp','sig')]:
        marker='phase4 += Float::with_val(PRECISION_BITS, &'+('dm' if kind=='EXP' else 'val')+')'
        assert source.count(marker)==1
        start=source.index(marker);end=source.index('.val;',start)+len('.val;')
        statement=source[start:end]
        power='Float::with_val(PRECISION_BITS, &freq).pow(&Float::with_val(PRECISION_BITS, &gamma))'
        increment=('Float::with_val(PRECISION_BITS, &dm) * &al_power * Float::with_val(PRECISION_BITS, &al_exp)' if kind=='EXP' else 'Float::with_val(PRECISION_BITS, &val)')+' * &psr.param.slots[param_f as usize].cells[0].val'
        trace='let al_before=phase4.clone(); '+statement+'\n'
        trace+=f'let al_power={power}; let al_exp=ordinary::exp(&exponent); let al_increment={increment};\n'
        trace+='eprintln!("AL_DETAIL '+kind+' {} {} ordinary_bits {}",i,k,ordinary::NATIVE_BITS);\n'
        fields=[('freq','freq'),('dt','dt'),('amp',amp),('width',width),('index','gamma'),
            ('exponent','exponent'),('exp','al_exp'),('power','al_power'),
            *([('val','val')] if kind=='GAUS' else []),('increment','al_increment'),
            ('phase_before','al_before'),('phase_after','phase4')]
        for key,value in fields:
            trace+='eprintln!("AL_DETAIL '+kind+' {} {} '+key+' {}",i,k,'+value+'.to_string_radix(10,Some(45)));\n'
        source=source[:start]+trace+source[end:]
    return source


def first_divergence(native, rust):
    def rows(path):return [line.split()[1:] for line in path.read_text().splitlines() if line.startswith('AL_DETAIL ')]
    left,right=rows(native),rows(rust);assert len(left)==len(right),(len(left),len(right))
    comparisons=[];first=None;bits=None
    for a,b in zip(left,right,strict=True):
        assert a[:4]==b[:4],(a,b)
        kind,observation,event,field=a[:4]
        if field=='ordinary_bits':
            assert a[4]==b[4] and int(a[4]) in [53,64]
            bits=int(a[4]);continue
        assert bits is not None
        precision=113 if field in ['power','increment','phase_before','phase_after'] else bits
        same=binary_equal(Decimal(a[4]),Decimal(b[4]),precision)
        row=dict(kind=kind,observation=int(observation),event=int(event),field=field,
            native=a[4],rust=b[4],comparison_bits=precision,equal=same)
        comparisons.append(row)
        if not same and first is None:first=row
    return dict(first=first,rows=comparisons,
        scope='Ordered observed assignments/subexpressions; wide fields compared at native 113 bits. Original residual/phase gates remain separate.')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ['out', 'core', 'science', 'timing', 'runtime', 'rust', 'candidate']:
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
    for name, text in [('observed', detailed_native(source)), ('f64-temporaries', observed(narrow))]:
        path = run.out/(name+'.C'); obj = run.out/(name+'.o'); exe = run.out/name
        path.write_text('#line 1 "'+str(original)+'"\n'+text)
        (run.out/(name+'.patch')).write_text(''.join(difflib.unified_diff(source.splitlines(True), text.splitlines(True), fromfile='original/formResiduals.C', tofile=name+'/formResiduals.C')))
        run.run([str(path) if v==str(original) else str(obj) if v==str(core/'formResiduals.C.o') else v for v in compile_command], name+'-compile')
        run.run([str(obj) if v==str(core/'formResiduals.C.o') else str(exe) if v==str(science/'timing') else v for v in link_command], name+'-link')
        executables[name] = exe
    candidate=a.candidate.resolve();private=run.out/'rust-observed'
    shutil.copytree(candidate/'rust',private,ignore=shutil.ignore_patterns('target','verification','.cargo-home'))
    rust_source=private/'src/residual_source.rs';before=rust_source.read_text();after=detailed_rust(before)
    rust_source.write_text(after)
    (run.out/'rust-observed.patch').write_text(''.join(difflib.unified_diff(before.splitlines(True),after.splitlines(True),fromfile='original/residual_source.rs',tofile='observed/residual_source.rs')))
    run.run(['cargo','build','--offline','--locked','--release','--manifest-path',private/'Cargo.toml',
        '--target-dir',run.out/'rust-target','--bin','tempo2-rust'],'rust-observer-build')
    observed_rust=run.out/'rust-target/release/tempo2-rust'
    fixture = ROOT/'validation/timing_models/fixtures/events.par'
    control = run.out/'without-events.par'
    control.write_text(''.join(line for line in fixture.read_text().splitlines(True) if not line.startswith(('EXP', 'GAUS'))))
    gaussian=run.out/'gaussian-only.par'
    gaussian.write_text(''.join(line for line in fixture.read_text().splitlines(True)
        if not line.startswith(('EXP','GAUSINDEX','GAUSAMP')))+'GAUSAMP_1 123456789.123456789123\n')
    records = []; invariant_failures = []
    product_controls=[];divergences=[]
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
            rust=run.out/f'{case}-{mean}-rust.json'
            command=[a.rust.resolve(),'timing','--par',par,'--tim',fixture.with_suffix('.tim'),
                     '--runtime',a.runtime.resolve(),*(['--no-mean'] if mean==0 else [])]
            rust.write_bytes(run.run(command,f'{case}-{mean}-rust',cwd=run.out))
            candidate=json.loads(rust.read_text())
            tag=f'{case}-{mean}-rust-observed'
            command=[observed_rust,'timing','--par',par,'--tim',fixture.with_suffix('.tim'),
                '--runtime',a.runtime.resolve(),*(['--no-mean'] if mean==0 else [])]
            raw=run.run(command,tag,cwd=run.out)
            neutral=json.loads(raw)==candidate
            product_controls.append(dict(case=case,mean=mean,rust_observer_neutral=neutral))
            if not neutral:invariant_failures.append(f'{case}-{mean}-rust-observer')
            divergences.append(dict(case=case,mean=mean,**first_divergence(
                run.out/f'{case}-{mean}-observed.stderr',run.out/(tag+'.stderr'))))
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
                  records=records,product_controls=product_controls,first_divergence=divergences,
                  historical_f64_profile='4accd26e2d51b3dc805d9a1ae0a2be7195e171ea',
                  gaussian_scope='Large-amplitude default-index numerical witness of ordinary val narrowing; not an astrophysical calibration',
                  inputs={str(p):sha(p) for p in [original, core/'receipt.json', science/'receipt.json', fixture, fixture.with_suffix('.tim'),gaussian,a.rust.resolve(),
                      *([a.baseline_rust.resolve()] if a.baseline_rust else []),
                      a.candidate.resolve()/'rust/src/residual_source.rs',rust_source,observed_rust,Path(__file__)]}))
    raise SystemExit(bool(invariant_failures or scientific_failures))
