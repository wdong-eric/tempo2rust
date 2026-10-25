"""0087 paired DDH checks: current-source fidelity and versioned 1 ns ideal test."""
from pathlib import Path
from decimal import Decimal as D, getcontext
import argparse, hashlib, json, os, subprocess
from compare import compare, decimal
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
BUILD=HERE/'build/ddh';CENTRAL=ROOT/'decisions/evidence/ddh-clock/build/review-20261026'
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
getcontext().prec=80

def run(profile):
    BUILD.mkdir(parents=True,exist_ok=True)
    runtime=ROOT/'validation/reference/build/source/T2runtime'
    historical=BUILD/'runtime-historical-gps';historical.mkdir(exist_ok=True)
    for entry in runtime.iterdir():
        if entry.name=='clock':continue
        target=historical/entry.name
        if not target.is_symlink():target.symlink_to(entry,target_is_directory=entry.is_dir())
        assert target.resolve()==entry.resolve()
    clock=historical/'clock';clock.mkdir(exist_ok=True)
    assert not (runtime/'clock/gps2utc.clk').exists(),'current runtime resource identity changed'
    for entry in (runtime/'clock').iterdir():
        target=clock/entry.name
        if not target.is_symlink():target.symlink_to(entry,target_is_directory=entry.is_dir())
        assert target.resolve()==entry.resolve()
    table=CENTRAL/'legacy-gps2utc.clk'
    assert sha(table)=='30727fdfaaec0f1775543804210d057ffe62076594bb8884d8f8ccafafb676be'
    (clock/'gps2utc.clk').write_bytes(table.read_bytes())
    par=CENTRAL/'inputs/test3.par';tim=CENTRAL/'inputs/test3.tim'
    for p in [par,tim]:assert p.read_bytes()==(runtime.parent/'tests/test_data'/p.name).read_bytes()
    native=HERE/'build/native/fit';rust=ROOT/'rust/target'/profile/'examples/fitting_ideal'
    records=[]
    for label,root in [('current',runtime),('historical',historical)]:
        out=BUILD/profile/label;out.mkdir(parents=True,exist_ok=True)
        trace=out/'native.tsv';candidate=out/'rust.json';resources=out/'native-resources.tsv'
        cmds=[[str(native),str(trace),'1',str(par),str(tim)],[str(rust),str(root),str(par),str(tim)]]
        env={**os.environ,'TEMPO2':str(root),'AE_IDEAL':'1','REFERENCE_RESOURCE_LOG':str(resources),'LC_ALL':'C'}
        for key in ['AE_COVARIANCE','AE_NORESCALE','AE_WRITE_RESIDUALS','REFERENCE_TEST_STORAGE_PATTERN','TEMPO2_ALIAS','TEMPO2_CLOCK_DIR']:env.pop(key,None)
        for engine,cmd in zip(['native','rust'],cmds):
            proc=subprocess.run(cmd,cwd=out,env=env,capture_output=True,timeout=120)
            (out/(engine+'.stderr')).write_bytes(proc.stderr)
            (candidate if engine=='rust' else out/'native.stdout').write_bytes(proc.stdout)
            assert proc.returncode==0,(label,engine,proc.returncode,proc.stderr.decode()[-3000:])
        result=json.loads(candidate.read_text());assert len(result['cycles'])==1
        c=result['cycles'][0];assert not result['remove_mean'] and not c['prefit']['remove_mean'] and not c['postfit']['remove_mean']
        for before,after in zip(c['prefit']['pulsars'][0]['observations'],c['postfit']['pulsars'][0]['observations']):
            for key in ['input_index','filename','site','flags','sat_mjd','frequency','uncertainty','deleted']:
                assert before[key]==after[key],('fit changed input metadata',key)
        native_values={}
        for line in trace.read_text().splitlines():
            it,stage,p,i,key,value=line.split('\t');native_values[int(it),stage,int(p),int(i),key]=D(value)
        for i in range(284):
            for key in ['sat','frequency','uncertainty','deleted']:
                assert native_values[1,'prefit',0,i,key]==native_values[1,'postfit',0,i,key]
        observations={};ideal={};input_identity=[]
        for stage in ['prefit','postfit']:
            rows=c[stage]['pulsars'][0]['observations'];assert len(rows)==284
            assert [r['input_index']for r in rows]==list(range(284))
            for source in c[stage]['inputs']:
                path=Path(source['path']['text'])
                assert sha(path)==source['sha256'] and bytes(source['bytes'])==path.read_bytes()
                input_identity.append({'path':str(path),'sha256':source['sha256']})
            values={
                'native':[native_values[1,stage,0,i,'residual']for i in range(284)],
                'rust':[decimal(r['residual_s'])for r in rows],
            }
            ideal[stage]={engine:{'maximum_absolute_s':str(max(map(abs,v))),
                'failing_rows':[i for i,x in enumerate(v)if abs(x)>=D('1e-9')],
                'passes_unchanged_1ns':all(abs(x)<D('1e-9')for x in v)}for engine,v in values.items()}
        central=json.loads((CENTRAL/('current' if label=='current' else 'legacy_gps_only')/'parsed.json').read_text())
        assert len(central['stages']['parsed'])==284
        for original,actual in zip(central['stages']['parsed'],c['prefit']['pulsars'][0]['observations']):
            assert original['fname']==actual['filename']['text'] and original['site']==actual['site']['text']
            for old,new in [('deleted','deleted'),('clockCorr','clock_correction'),('delayCorr','delay_correction')]:
                assert int(original[old])==actual[new]
            flags=[(original['flag_id_'+str(i)],original['flag_value_'+str(i)])for i in range(len(actual['flags']))]
            assert len(flags)==sum(key.startswith('flag_id_')for key in original)
            assert flags==[(f['id']['text'],f['value']['text'])for f in actual['flags']]
        central_prefit=[D(r['residual_s'])for r in central['stages']['residuals']]
        native_prefit=[native_values[1,'prefit',0,i,'residual']for i in range(284)]
        central_differences=[str(a-b)for a,b in zip(native_prefit,central_prefit)]
        fidelity=compare(trace,candidate,out/'comparison.json')
        recorded_resources={}
        for stage in ['prefit','postfit']:
            for item in c[stage]['resources']:
                path=item['path']['text']
                if item.get('sha256') is not None:
                    assert sha(path)==item['sha256'];recorded_resources[path]=item['sha256']
        native_resources={}
        for row in resources.read_text().splitlines():
            stage,path,mode,opened,error,caller=row.split('\t')
            if opened=='1' and mode.startswith('r'):
                native_resources[path]=sha(path)
        assert all(native_resources.get(path)==digest for path,digest in recorded_resources.items())
        if label=='historical':assert native_resources[str(clock/'gps2utc.clk')]==sha(table)
        records.append({'clock_case':label,'runtime':str(root),'commands':cmds,'ideal_checks':ideal,
            'fidelity_failures':fidelity['failures'],'fidelity_stats':fidelity['stats'],'unchecked':fidelity['unchecked'],
            'comparison_sha256':sha(out/'comparison.json'),'native_export_sha256':sha(trace),'rust_export_sha256':sha(candidate),
            'central_native_prefit_max_difference_s':str(max(abs(D(v))for v in central_differences)),
            'inputs':input_identity,'original_parsed_row_metadata_checked':284,'rust_resources':recorded_resources,'native_read_resources':native_resources,
            'native_resource_trace_sha256':sha(resources)})
        print(label,'fidelity failures',len(fidelity['failures']),'ideal',{
            stage:{engine:(v['maximum_absolute_s'],len(v['failing_rows']))for engine,v in engines.items()}for stage,engines in ideal.items()},flush=True)
    receipt={'profile':profile,'pin':'fda6ad4a9753ad7dc5d291406f2f310a9161423f','historical_clock_revision':'0759584ebf8315bbbe6a6d0b41c08ebbcfe527f2',
        'historical_clock_sha256':sha(table),'original_native_receipt_sha256':sha(CENTRAL/'receipt.json'),
        'original_full_test_sha256':sha(CENTRAL/'original-test-receipt.json'),'builder_sha256':sha(__file__),
        'native_identity_sha256':sha(HERE/'build/native/identity.json'),'native_executable_sha256':sha(native),'rust_executable_sha256':sha(rust),
        'input_sha256':{str(p):sha(p)for p in [par,tim]},'records':records}
    (BUILD/(profile+'.json')).write_text(json.dumps(receipt,indent=2)+'\n')

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--profile',choices=['debug','release'],default='debug');run(p.parse_args().profile)
