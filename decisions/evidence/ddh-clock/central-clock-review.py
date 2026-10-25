from pathlib import Path
from decimal import Decimal as D, localcontext
import hashlib,importlib.util,json,os,shutil,subprocess,sys,tempfile
sys.dont_write_bytecode=True
root=Path('/Users/Eric.Dong/Downloads/tempo2rust')
ref=root/'validation/reference'
runtime=ref/'build/source/T2runtime'
original_path=ref/'checks/ddh.json'
original_bytes=original_path.read_bytes()
assert hashlib.sha256(original_bytes).hexdigest()=='d1fdd35a4cdacc386cd91bfce2b55e09994c914fc2c2ea3c7194ed5e223cca97'
original=json.loads(original_bytes)
manifest=json.loads((ref/'build_manifest.json').read_text())
exe=ref/'build/export_reference'
assert hashlib.sha256(exe.read_bytes()).hexdigest()==manifest['binaries']['export_reference']['sha256']
identities={}
for label,item in original['resources']['files'].items():
    path=Path(label.replace('$RUNTIME',str(runtime)).replace('$REFERENCE',str(ref)))
    digest=hashlib.sha256(path.read_bytes()).hexdigest()
    assert digest==item['sha256'],(label,digest,item['sha256'])
    identities[str(path)]=digest
out=Path(tempfile.mkdtemp(prefix='tempo2-ddh-clock-'))
(out/'original-evidence.json').write_bytes(original_bytes)
(out/'reference-build-manifest.json').write_bytes((ref/'build_manifest.json').read_bytes())
shutil.copy2(exe,out/'export_reference')
inputs=out/'inputs';inputs.mkdir()
for name,item in original['source_inputs'].items():
    data=(ref/'build/source/tests/test_data'/name).read_bytes()
    assert hashlib.sha256(data).hexdigest()==item['sha256']
    (inputs/name).write_bytes(data)
legacy_commit=subprocess.check_output(['git','-C',str(root/'tempo2'),'rev-parse','0759584'],text=True).strip()
legacy=subprocess.check_output(['git','-C',str(root/'tempo2'),'show',legacy_commit+':T2runtime/clock/gps2utc.clk'])
legacy_runtime=out/'runtime-legacy-gps';legacy_runtime.mkdir()
for p in runtime.iterdir():
    if p.name!='clock':(legacy_runtime/p.name).symlink_to(p,target_is_directory=p.is_dir())
clock=legacy_runtime/'clock';clock.mkdir()
for p in (runtime/'clock').iterdir():
    (clock/p.name).symlink_to(p,target_is_directory=p.is_dir())
replacement=clock/'gps2utc.clk'
current_gps=(runtime/'clock/gps2utc.clk')
original_gps_sha=hashlib.sha256(current_gps.read_bytes()).hexdigest() if current_gps.exists() else None
if replacement.exists() or replacement.is_symlink():replacement.unlink()
replacement.write_bytes(legacy)
(out/'legacy-gps2utc.clk').write_bytes(legacy)
spec=importlib.util.spec_from_file_location('tempo2_a2_reference_reader',ref/'run_reference.py')
reader=importlib.util.module_from_spec(spec);spec.loader.exec_module(reader)
rows=[]
for line_number,line in enumerate((inputs/'test3.tim').read_text().splitlines(),1):
    f=line.split()
    if len(f)<5 or f[0]!='FAKE':continue
    rows.append({'index':len(rows),'line':line_number,'raw_line':line,'name':f[0],'frequency_mhz_text':f[1],'sat_mjd_text':f[2],'toa_error_us_text':f[3],'site':f[4]})
receipt={'purpose':'Bounded DDH clock-resource localization; original gate and fixtures preserved','source_revision':manifest['source_commit'],'legacy_clock_revision':legacy_commit,'legacy_clock_source':'T2runtime/clock/gps2utc.clk','original_ddh_sha256':hashlib.sha256(original_bytes).hexdigest(),'executable_sha256':manifest['binaries']['export_reference']['sha256'],'original_gps2utc_sha256':original_gps_sha,'legacy_gps2utc_sha256':hashlib.sha256(legacy).hexdigest(),'original_read_resources_verified':len(identities),'script_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'runs':{}}
all_data={}
for label,selected_runtime in [('current',runtime),('legacy_gps_only',legacy_runtime)]:
    work=out/label;work.mkdir()
    command=[str(out/'export_reference'),str(inputs/'test3.par'),str(inputs/'test3.tim'),'preprocess',str(work/'export.tsv')]
    env=dict(os.environ,TEMPO2=str(selected_runtime),REFERENCE_RESOURCE_LOG=str(work/'resources.tsv'),LC_ALL='C')
    for name in ['REFERENCE_TEST_STORAGE_PATTERN','TEMPO2_ALIAS','TEMPO2_CLOCK_DIR']:env.pop(name,None)
    result=subprocess.run(command,cwd=work,env=env,capture_output=True,timeout=90)
    (work/'stdout.log').write_bytes(result.stdout);(work/'stderr.log').write_bytes(result.stderr)
    assert result.returncode==0,(label,result.returncode,result.stderr[-1000:])
    data=reader.parse_export(work/'export.tsv',rows)
    all_data[label]=data
    (work/'parsed.json').write_text(json.dumps(data,indent=2)+'\n')
    values=[D(o['residual_s']) for o in data['stages']['residuals']]
    failure_indices=[i for i,x in enumerate(values) if abs(x)>=D('1e-9')]
    worst=max(range(len(values)),key=lambda i:abs(values[i]))
    observed=[]
    for line in (work/'resources.tsv').read_text().splitlines():
        stage,path,mode,opened,error,caller=line.split('\t')
        if opened=='1' and mode.startswith('r'):
            p=Path(path)
            observed.append((str(p),hashlib.sha256(p.read_bytes()).hexdigest()))
    receipt['runs'][label]={'command':command,'runtime':str(selected_runtime),'returncode':result.returncode,'observations':len(values),'failure_count':len(failure_indices),'first_failure':failure_indices[0] if failure_indices else None,'maximum_absolute_residual_index':worst,'maximum_absolute_residual_s':str(abs(values[worst])),'export_sha256':hashlib.sha256((work/'export.tsv').read_bytes()).hexdigest(),'opened_read_resource_identities':dict(observed)}
    if label=='current':assert [o['residual_s'] for o in data['stages']['residuals']]==original['all_residuals_s'],'original evidence did not regenerate'
    print(label,'rows',len(values),'failures',len(failure_indices),'max_ns',str(abs(values[worst])*10**9),flush=True)
with localcontext() as ctx:
    ctx.prec=90
    a=all_data['current']['stages']['residuals'];b=all_data['legacy_gps_only']['stages']['residuals']
    change0=D(b[0]['clock_tt_s'])-D(a[0]['clock_tt_s'])
    deltas=[]
    for i,(x,y) in enumerate(zip(a,b)):
        clock_delta=D(y['clock_tt_s'])-D(x['clock_tt_s'])
        residual_delta=D(y['residual_s'])-D(x['residual_s'])
        deltas.append({'index':i,'sat_mjd':x['sat_mjd'],'clock_delta_s':str(clock_delta),'relative_clock_delta_s':str(clock_delta-change0),'residual_delta_s':str(residual_delta),'residual_minus_relative_clock_s':str(residual_delta-(clock_delta-change0)),'current_chain':[x[k] for k in sorted(x) if k.startswith('clock_to_')],'legacy_chain':[y[k] for k in sorted(y) if k.startswith('clock_to_')]})
    (out/'clock-deltas.json').write_text(json.dumps(deltas,indent=2)+'\n')
    receipt['comparisons']={'parameters_equal':all_data['current']['parameters']==all_data['legacy_gps_only']['parameters'],'parsed_observations_equal':all_data['current']['stages']['parsed']==all_data['legacy_gps_only']['stages']['parsed'],'pulse_counts_equal':all(x['pulse_number']==y['pulse_number'] for x,y in zip(a,b)),'max_residual_minus_relative_clock_s':str(max(abs(D(x['residual_minus_relative_clock_s'])) for x in deltas)),'selected_clock_deltas':[deltas[i] for i in [0,35,38]]}
for path,digest in identities.items():assert hashlib.sha256(Path(path).read_bytes()).hexdigest()==digest,path
assert original_path.read_bytes()==original_bytes
receipt['original_resources_unchanged_after_runs']=True
(out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
Path('/tmp/tempo2-ddh-clock-current').write_text(str(out)+'\n')
print('output',out)
print('receipt_sha256',hashlib.sha256((out/'receipt.json').read_bytes()).hexdigest())
print('comparisons',json.dumps(receipt['comparisons']))
