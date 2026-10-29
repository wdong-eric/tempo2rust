#!/usr/bin/env python3
"""Isolate explicit Rust fusion in a generated diagnostic, preserving the library."""
import argparse
import difflib
import json
from pathlib import Path
from common import ROOT, Run, sha
from contraction import comparator


if __name__ == '__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    for name in ['out','prefix','contraction','target']:
        parser.add_argument('--'+name,type=Path,required=True)
    a=parser.parse_args();run=Run(a.out)
    installed=a.prefix.resolve()/'share/tempo2-rust/rust'
    source=installed/'src/cholesky_spectrum.rs';text=source.read_text()
    body=text[text.index('pub fn lomb('):text.index('/// TK_weightLS')].strip()
    unfused=body
    for before,after in [
        ('st.mul_add(ct, s1)','(st * ct + s1)'),
        ('(ct - st).mul_add(ct + st, s2)','((ct - st) * (ct + st) + s2)'),
        ('ctau.mul_add(ct, st * stau)','(ctau * ct + st * stau)'),
        ('ctau.mul_add(st, -(ct * stau))','(ctau * st - ct * stau)'),
        ('c.mul_add(value, s1)','(c * value + s1)'),
        ('s.mul_add(value, s2)','(s * value + s2)'),
        ('(-ctau).mul_add(st, ct * stau)','(ct * stau - st * ctau)'),
        ('alpha[i].mul_add(ct, -(beta[i] * st))','(alpha[i] * ct - beta[i] * st)'),
        ('alpha[i].mul_add(st, beta[i] * ct)','(alpha[i] * st + beta[i] * ct)'),
    ]:
        assert unfused.count(before)==1,before
        unfused=unfused.replace(before,after)
    (run.out/'unfused.patch').write_text(''.join(difflib.unified_diff(body.splitlines(True),unfused.splitlines(True),fromfile='installed/lomb',tofile='diagnostic/lomb')))
    crate=run.out/'client';(crate/'src').mkdir(parents=True)
    (crate/'Cargo.toml').write_text('[package]\nname="al-lomb-diagnostic"\nversion="0.1.0"\nedition="2024"\n[dependencies]\ntempo2-rust={path='+json.dumps(str(installed))+'}\n')
    driver=(ROOT/'rust/examples/release_contraction.rs').read_text()
    driver=driver.replace('fn main() -> Result<(), Box<dyn std::error::Error>> {', '''fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kernel = match std::env::args().nth(1).as_deref() {
        Some("copied") => copied_lomb,
        Some("unfused") => unfused_lomb,
        _ => tempo2_rust::spectrum::lomb,
    };''').replace('tempo2_rust::spectrum::lomb(&x, &y, 1.0, 1.0)?','kernel(&x, &y, 1.0, 1.0)?')
    preamble='''use tempo2_rust::{TimingError,toolkit};
use std::f64::consts::PI;
fn boundary(message: &str) -> TimingError {
    TimingError { path: "lomb-diagnostic".into(), line: None, field: "dimensions".into(), message: message.into() }
}
'''
    # Existing public toolkit leaves remain canonical; only nine explicit FMA
    # sites differ. The copied baseline must equal the installed API exactly.
    (crate/'src/main.rs').write_text(driver+'\n'+preamble+'\n'+body.replace('pub fn lomb(', 'fn copied_lomb(')+'\n'+unfused.replace('pub fn lomb(', 'fn unfused_lomb('))
    run.run(['cargo','generate-lockfile','--offline','--manifest-path',crate/'Cargo.toml'],'lock')
    run.run(['cargo','build','--offline','--locked','--release','--manifest-path',crate/'Cargo.toml','--target-dir',a.target.resolve()],'build')
    exe=a.target.resolve()/'release/al-lomb-diagnostic';outputs={}
    for mode in ['installed','copied','unfused']:
        outputs[mode]=run.run([exe,mode],mode).decode()
    assert outputs['installed']==outputs['copied']
    assert outputs['installed']==(a.contraction/'lomb-rust.stdout').read_text()
    units={'x':'f64_exact','y':'f64_exact','count':'exact','variance':'s^2','frequency':'frequency_day','power':'dimensionless'}
    table=run.out/'unfused.tsv'
    table.write_text(''.join(f'{case}.{kind}.{index}\t{units[kind]}\t{value}\n' for case,kind,index,bits,value in (line.split('\t') for line in outputs['unfused'].splitlines())))
    compare=comparator('al_lomb_diagnostic_compare',ROOT/'validation/analysis_tools/compare.py').compare
    comparisons={profile:compare(a.contraction/('lomb-'+profile+'.tsv'),table) for profile in ['default','off','fast']}
    run.save(dict(status='pass',scope='Diagnostic execution only. Copied baseline exactly equals installed API; unfused counterfactual does not replace the failed production comparison.',
                  comparisons=comparisons,inputs={str(p):sha(p) for p in [source,ROOT/'rust/examples/release_contraction.rs',a.contraction/'receipt.json',Path(__file__)]}))
