"""Bounded central AE handover review; never overwrites delivery evidence."""
from pathlib import Path
from decimal import Decimal
from copy import deepcopy
import argparse, hashlib, importlib.util, json, os, shutil, subprocess, sys
sys.dont_write_bytecode = True

def sha(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()

def main():
    p = argparse.ArgumentParser()
    p.add_argument('--root', type=Path, required=True)
    p.add_argument('--out', type=Path, required=True)
    p.add_argument('--rust', type=Path, required=True)
    p.add_argument('--ideal', type=Path, required=True)
    a = p.parse_args(); root = a.root.resolve(); out = a.out.resolve()
    out.mkdir(parents=True, exist_ok=False)
    base = root / 'validation/general_fitting'
    index_path = base / 'build/delivery/index-ddff7f82cfe0.json'
    assert sha(index_path) == '971f96f9ca9c8aafb519756235343d8bb369ec24a8842edf4f174797cfe6a7c9'
    index = json.loads(index_path.read_text()); identity = {}; missing = []
    for group in ['receipts', 'comparison_and_identity_files', 'authored_source', 'available_executables']:
        for name, expected in index[group].items():
            path = root / name
            actual = sha(path) if path.is_file() else None
            identity[name] = {'expected': expected, 'actual': actual, 'match': actual == expected}
            if actual is None and group == 'available_executables': missing.append(name)
            else: assert actual == expected, ('changed delivery evidence', name)
    (out/'identity-review.json').write_text(json.dumps(identity, indent=2)+'\n')
    # Snapshot the independent source enum and exact delivered comparator.
    comparison_path=out/'validation/general_fitting/compare.py'
    comparison_path.parent.mkdir(parents=True)
    shutil.copy2(base/'compare.py', comparison_path)
    (out/'tempo2').mkdir()
    shutil.copy2(root/'tempo2/tempo2.h', out/'tempo2/tempo2.h')
    spec=importlib.util.spec_from_file_location('central_ae_compare',comparison_path)
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    fit_inputs=root/'validation/fitting/inputs'; inputs=base/'inputs'
    native=base/'build/native/fit'; runtime=root/'validation/reference/build/source/T2runtime'
    env=dict(os.environ)
    for key in ['AE_COVARIANCE','AE_NORESCALE','AE_IDEAL','AE_WRITE_RESIDUALS','REFERENCE_RESOURCE_LOG','REFERENCE_TEST_STORAGE_PATTERN','TEMPO2_ALIAS','TEMPO2_CLOCK_DIR']:
        env.pop(key,None)
    env['TEMPO2']=str(runtime);env['LC_ALL']='C'
    cases=[('global',[inputs/'global0.par',inputs/'global1.par'],None),
           ('prior',[inputs/'parameter-prior.par'],None),
           ('robust-H',[inputs/'robust-H.par'],None),
           ('covariance',[fit_inputs/'all.par'],inputs/'covariance.model')]
    records=[]; mutations=[]
    for name,pars,cov in cases:
        d=out/name;d.mkdir(); local_env=dict(env)
        native_cmd=[str(native),str(d/'native.tsv'),str(len(pars))]
        rust_cmd=[str(a.rust),'timing','--fit','--runtime',str(runtime)]
        for par in pars:
            native_cmd.extend([str(par),str(fit_inputs/'bary.tim')])
            rust_cmd.extend(['--par',str(par),'--tim',str(fit_inputs/'bary.tim')])
        if cov:
            native_cmd_env=str(cov);local_env['AE_COVARIANCE']=native_cmd_env
            rust_cmd.extend(['--covariance',str(cov)])
        for engine,cmd in [('native',native_cmd),('rust',rust_cmd)]:
            r=subprocess.run(cmd,cwd=d,env=local_env,capture_output=True,timeout=120)
            (d/(engine+'.stderr')).write_bytes(r.stderr)
            (d/('rust.json' if engine=='rust' else 'native.stdout')).write_bytes(r.stdout)
            assert r.returncode==0,(name,engine,r.returncode,r.stderr[-1000:])
        c=module.compare(d/'native.tsv',d/'rust.json',d/'comparison.json')
        assert not c['failures'],(name,c['failures'][:3])
        records.append({'case':name,'commands':[native_cmd,rust_cmd],'comparison_sha256':sha(d/'comparison.json'),'checks':sum(v['count'] for v in c['stats'].values()),'failures':len(c['failures'])})
        baseline=json.loads((d/'rust.json').read_text())
        controls={}
        x=deepcopy(baseline);x['cycles'][0]['systems'][0]['noise'].pop();controls['missing_noise']=x
        x=deepcopy(baseline);x['cycles'][0]['systems'][0]['noise'][1]['observation']=x['cycles'][0]['systems'][0]['noise'][0]['observation'];controls['duplicated_noise_identity']=x
        x=deepcopy(baseline);x['cycles'][0]['postfit']['pulsars'][0]['observations'][1]['canonical_index']=0;controls['duplicated_timing_identity']=x
        x=deepcopy(baseline);x['cycles'][0]['systems'][0]['solver']['error_multiplier']='1e100';controls['wrong_error_multiplier']=x
        x=deepcopy(baseline);x['cycles'][0]['systems'][0]['updates'][1]['postfit_value']=None;controls['omitted_postfit_coordinate']=x
        if len(baseline['cycles'])>1:
            x=deepcopy(baseline);x['cycles'][1]=deepcopy(x['cycles'][0]);controls['duplicated_cycle']=x
        if name=='global':
            x=deepcopy(baseline);x['cycles'][0]['systems'][0]['updates'][-1]['pulsar']=0;controls['wrong_global_update_owner']=x
        if name=='prior':
            x=deepcopy(baseline);x['cycles'][0]['systems'][0]['constraint_targets'][0]='1e100';controls['changed_constraint_target']=x
        for label,x in controls.items():
            q=d/('mutant-'+label+'.json');q.write_text(json.dumps(x)+'\n')
            cc=module.compare(d/'native.tsv',q,d/('mutant-'+label+'-comparison.json'))
            assert cc['failures'],(name,label,'falsely accepted')
            mutations.append({'case':name,'mutation':label,'failures':len(cc['failures'])})
        print(name,'fresh checks',records[-1]['checks'],'mutants',len(controls),flush=True)
    ddh=[]
    central=root/'decisions/evidence/ddh-clock/build/review-20261026'
    par=central/'inputs/test3.par';tim=central/'inputs/test3.tim'
    for label,rt in [('current',runtime),('historical',base/'build/ddh/runtime-historical-gps')]:
        d=out/('ddh-'+label);d.mkdir()
        if label=='historical':assert sha(rt/'clock/gps2utc.clk')=='30727fdfaaec0f1775543804210d057ffe62076594bb8884d8f8ccafafb676be'
        cmd=[str(a.ideal),str(rt),str(par),str(tim)]
        local_env={**env,'TEMPO2':str(rt)}
        r=subprocess.run(cmd,cwd=d,env=local_env,capture_output=True,timeout=120)
        (d/'rust.json').write_bytes(r.stdout);(d/'rust.stderr').write_bytes(r.stderr)
        assert r.returncode==0,(label,r.stderr[-1000:])
        native_trace=base/'build/ddh/release'/label/'native.tsv'
        c=module.compare(native_trace,d/'rust.json',d/'comparison.json')
        result=json.loads(r.stdout);ideal={}
        for stage in ['prefit','postfit']:
            obs=result['cycles'][0][stage]['pulsars'][0]['observations'];assert len(obs)==284
            values=[module.decimal(o['residual_s'])for o in obs]
            ideal[stage]={'rows':len(obs),'maximum_s':str(max(map(abs,values))),'fails_1ns':sum(abs(v)>=Decimal('1e-9')for v in values)}
            if label=='historical':assert ideal[stage]['fails_1ns']==0
        ddh.append({'clock':label,'command':cmd,'native_trace_sha256':sha(native_trace),'fidelity_failures':len(c['failures']),'ideal':ideal})
        print('ddh',label,'fidelity failures',len(c['failures']),'ideal',ideal,flush=True)
    receipt={'reviewed_revision':subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(),
             'delivery_index_sha256':sha(index_path),'identity_count':len(identity),'missing_historical_executables':missing,
             'new_rust_executable':{'path':str(a.rust),'sha256':sha(a.rust)},'new_ideal_executable':{'path':str(a.ideal),'sha256':sha(a.ideal)},
             'native_executable_sha256':sha(native),'comparator_sha256':sha(comparison_path),'reviewer_sha256':sha(__file__),
             'public':records,'mutation_controls':mutations,'ddh':ddh,
             'interpretation':'Bounded handover review, not full AE scientific acceptance or Linux qualification. Fresh public runs use both engines; DDH uses newly built Rust and immutable delivered native traces.'}
    (out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
    print('receipt',out/'receipt.json',sha(out/'receipt.json'))

if __name__=='__main__':main()
