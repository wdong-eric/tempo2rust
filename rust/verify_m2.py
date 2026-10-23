#!/usr/bin/env python3
"""Reproduce Assignment E checks without overwriting accepted M1 evidence.
Run from any directory: python3 rust/verify_m2.py
The independent reference is generated separately in validation/fitting/.
"""
from decimal import Decimal, localcontext
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

ROOT=Path(__file__).resolve().parent
FIT=ROOT.parent/'validation/fitting'
OUTPUT=ROOT/'verification/m2'
ENV=dict(os.environ,CARGO_HOME=str(ROOT/'.cargo-home'),GMP_MPFR_SYS_CACHE=str(ROOT/'target/gmp-cache'))
COMMANDS=[]

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def save(p,value):p.write_text(json.dumps(value,indent=2,sort_keys=True)+'\n')
def module(name,path):
    spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m

def run(name,cmd):
    r=subprocess.run(list(map(str,cmd)),cwd=ROOT,env=ENV,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
    (OUTPUT/(name+'.log')).write_text(r.stdout)
    COMMANDS.append({'command':list(map(str,cmd)),'returncode':r.returncode,'log':name+'.log'})
    save(OUTPUT/'commands.json',COMMANDS)
    print(name, r.returncode,flush=True)
    assert r.returncode==0,r.stdout
    return r

def cli_checks(profile):
    binary=ROOT/'target'/profile/'tempo2-rust';records=[]
    with tempfile.TemporaryDirectory(prefix='fit-cli-',dir=ROOT/'target') as td:
        d=Path(td);par=FIT/'inputs/square.par';tim=FIT/'inputs/square.tim';base=par.read_text()
        cases=[]
        for name,content in [('global',base.replace('F0 1 1','F0 1 2')),('unsupported-active',base.replace('PEPOCH 54000','PEPOCH 54000 1')),('bad-mode',base+'WEIGHT 2\n'),('bad-nits',base+'NITS 0\n'),('fractional-nits',base+'NITS 1.5\n'),('negative-nits',base+'NITS -2\n')]:
            p=d/(name+'.par');p.write_text(content);cases.append((name,p,tim,[]))
        for name in ['zero_error','tiny_error']:cases.append((name,par,FIT/'inputs'/f'{name}.tim',[]))
        for name,content in [('underdetermined','FORMAT 1\na 1400 54000 1 @\n'),('empty','FORMAT 1\n'),('negative-weight','FORMAT 1\na 1400 54000 -1 @\n'),('nonfinite-weight','FORMAT 1\na 1400 54000 NaN @\n')]:
            t=d/(name+'.tim');t.write_text(content);cases.append((name,par,t,[]))
        cases += [('missing-runtime',FIT/'inputs/pks_all.par',FIT/'inputs/pks.tim',[]),('missing-runtime-file',FIT/'inputs/pks_all.par',FIT/'inputs/pks.tim',['--runtime',d/'absent'])]
        for name,p,t,args in cases:
            r=subprocess.run([str(binary),'fit','--par',str(p),'--tim',str(t),*map(str,args)],cwd=ROOT,env=ENV,capture_output=True,text=True)
            assert r.returncode==1 and r.stdout=='',(name,r.stdout,r.stderr)
            records.append({'case':name,'returncode':r.returncode,'stdout_empty':True,'stderr':r.stderr})
        for args in [[],['--norescale']]:
            r=subprocess.run([str(binary),'fit','--par',str(par),'--tim',str(tim),*args],cwd=ROOT,env=ENV,capture_output=True,text=True)
            assert r.returncode==0,r.stderr;doc=json.loads(r.stdout)
            assert doc['status']==('completed' if args else 'completed_with_source_nonfinite')
            assert doc['cycles'][0]['solver']['errors'][0]=='NaN' if not args else doc['cycles'][0]['solver']['errors'][0]!='NaN'
            records.append({'case':'valid-square-after-errors','norescale':bool(args),'status':doc['status'],'stdout_complete':True,'stderr':r.stderr})
        # PKS public fit command and typed library must serialize the same model/results.
        r=subprocess.run([str(binary),'fit','--par',str(FIT/'inputs/pks_all.par'),'--tim',str(FIT/'inputs/pks.tim'),'--runtime',str(ROOT.parent/'tempo2/T2runtime')],cwd=ROOT,env=ENV,capture_output=True,text=True)
        assert r.returncode==0,r.stderr;doc=json.loads(r.stdout)
        library=json.loads((OUTPUT/profile/'pks_all.json').read_text())
        # Paths are supplied by the caller; numerical/provenance payload is identical otherwise.
        for item in [doc,library]:
            for key in ['par','tim']:item['input'][key].pop('path')
            for cycle in item['cycles']:
                for stage in ['prefit','postfit']:cycle[stage].pop('runtime_root')
        assert doc==library
        records.append({'case':'PKS CLI/library equality','complete_JSON':True,'warnings_stderr':'TROP1' in r.stderr and 'TROP2' in r.stderr})
    save(OUTPUT/(profile+'_cli.json'),records)

def main():
    OUTPUT.mkdir(parents=True,exist_ok=True)
    frozen=json.loads((FIT/'input_hashes.json').read_text())
    assert {p.name:sha(p) for p in (FIT/'inputs').iterdir() if p.is_file()}==frozen
    provenance=json.loads((FIT/'reference/manifest.json').read_text())
    assert sha(FIT/'export_reference.cc')==provenance['exporter_sha256']
    for name,value in provenance['output_hashes'].items():assert sha(FIT/'reference'/name)==value,name
    for name,value in provenance['source_hashes'].items():assert sha(ROOT.parent/'tempo2'/name)==value,name
    assert sha(ROOT.parent/'validation/reference/build/libtempo2-reference.a')==provenance['library_sha256']
    # Reuse the exact accepted M1 verifier, redirecting only its evidence destination.
    os.environ.update(CARGO_HOME=ENV['CARGO_HOME'],GMP_MPFR_SYS_CACHE=ENV['GMP_MPFR_SYS_CACHE'])
    m1=module('m1_regression',ROOT/'verify_m1b.py');m1.OUTPUT=OUTPUT/'m1b'
    oldargv=sys.argv;sys.argv=['verify_m1b.py','--runtime',str(ROOT.parent/'tempo2/T2runtime')]
    try:m1.main()
    finally:sys.argv=oldargv
    sys.path.insert(0,str(FIT));cmp=module('fit_comparator',FIT/'compare.py');mut=module('fit_mutations',FIT/'test_comparator.py')
    cases=json.loads((FIT/'cases.json').read_text());summary={};all_failures=[]
    for profile in ['debug','release']:
        flag=[] if profile=='debug' else ['--release']
        if flag:run('release_tests',['cargo','test',*flag,'--offline','--locked'])
        run(profile+'_fit_library',['cargo','run',*flag,'--offline','--locked','--example','compare_fitting'])
        dest=OUTPUT/profile;dest.mkdir(exist_ok=True)
        for p in (ROOT/'target/m2').glob('*.json'):
            if p.stem in {c['name'] for c in cases['timing']} or p.stem.startswith('shared_') or p.stem in ['callbacks','recovery']:shutil.copyfile(p,dest/p.name)
        checks={}
        for family in ['solver','timing']:
            for case in cases[family]:
                if case.get('expected')=='boundary':continue
                name=case['name'];r=json.loads((FIT/'reference'/f'{name}.json').read_text());a=json.loads((dest/f'shared_{name}.json').read_text())
                refs=[r] if family=='solver' else r['cycles']
                for i,(rc,ac) in enumerate(zip(refs,a)):
                    result=cmp.compare_shared(rc,ac,r['mode']);key=f'shared/{name}/{i+1}'
                    checks[key]={'status':'FAIL' if result.errors else 'PASS','measurements':result.measurements,'errors':result.errors,'maxima':result.maxima,'mode_permutations':result.mode_permutations}
                    all_failures += [profile+'/'+key+': '+s for s in result.errors]
                if family=='timing':
                    actual=json.loads((dest/f'{name}.json').read_text());epoch=next(line.split()[1] for line in (FIT/'inputs'/case['par']).read_text().splitlines() if line.startswith('PEPOCH'))
                    result=cmp.compare_timing(r,actual,epoch);key='end_to_end/'+name
                    checks[key]={'status':'FAIL' if result.errors else 'PASS','measurements':result.measurements,'errors':result.errors,'maxima':result.maxima,'mode_permutations':result.mode_permutations}
                    all_failures += [profile+'/'+key+': '+s for s in result.errors]
        cb=cmp.compare_callbacks(json.loads((FIT/'reference/callbacks.json').read_text()),json.loads((dest/'callbacks.json').read_text()))
        checks['callbacks']={'errors':cb.errors,'measurements':cb.measurements,'maxima':cb.maxima};all_failures+=cb.errors
        with localcontext() as ctx:ctx.prec=90;mutations=mut.mutations(dest)
        save(OUTPUT/(profile+'_comparator_mutations.json'),mutations)
        cli_checks(profile)
        summary[profile]=checks
    save(OUTPUT/'numerical_errors.json',summary)
    save(OUTPUT/'open_gate_failures.json',all_failures)
    # Every released timing and shared-input artifact must also agree between build modes.
    equivalence={}
    for p in sorted((OUTPUT/'debug').glob('*.json')):
        equal=p.read_bytes()==(OUTPUT/'release'/p.name).read_bytes();assert equal,p.name
        equivalence[p.name]={'byte_identical':True,'sha256':sha(p)}
    save(OUTPUT/'debug_release_equivalence.json',equivalence)
    run('final_format',['cargo','fmt','--check'])
    run('final_clippy',['cargo','clippy','--all-targets','--locked','--offline','--','-D','warnings'])
    sources=[ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'build.rs',ROOT/'verify_m2.py']
    for folder in [ROOT/'src',ROOT/'examples',ROOT/'tests',FIT]:
        sources.extend(p for p in folder.rglob('*') if p.is_file() and not any(x in p.parts for x in ['build','__pycache__','reference']))
    save(OUTPUT/'source_hashes.json',{str(p.relative_to(ROOT.parent)):sha(p) for p in sorted(set(sources))})
    print(f'M2: {len(all_failures)} open numerical gate failures; see verification/m2/open_gate_failures.json',flush=True)
    if all_failures:raise SystemExit(1)
if __name__=='__main__':main()
