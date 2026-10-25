from pathlib import Path
import hashlib,json,os,shutil,subprocess
root=Path('/Users/Eric.Dong/Downloads/tempo2rust')
out=Path(Path('/tmp/tempo2-ddh-clock-current').read_text().strip())
evidence=root/'validation/binary/upstream_ddh.json'
record=json.loads(evidence.read_text())
work=root/'validation/binary/build/upstream'
exe=work/'testEndToEnd'
assert hashlib.sha256(exe.read_bytes()).hexdigest()==record['binary_sha256']
assert hashlib.sha256((root/'tempo2/tests/testEndToEnd.C').read_bytes()).hexdigest()==record['source_sha256']
for name,expected in record['inputs'].items():assert hashlib.sha256((work/name).read_bytes()).hexdigest()==expected
binary=out/'original-testEndToEnd';shutil.copy2(exe,binary)
receipt={'purpose':'Execute the unchanged full checkIdeal assertions before and after t2Fit with current and historical GPS clocks','source_sha256':record['source_sha256'],'original_G_evidence_sha256':hashlib.sha256(evidence.read_bytes()).hexdigest(),'binary_sha256':record['binary_sha256'],'compile_time_input_directory':str(work),'inputs':record['inputs'],'threshold_s':'1e-9','script_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'runs':{}}
for name,runtime in [('current',root/'validation/reference/build/source/T2runtime'),('legacy_gps_only',out/'runtime-legacy-gps')]:
    run=out/('original-test-'+name);run.mkdir()
    command=[str(binary),'--gtest_filter=testEndToEnd.checkIdeal']
    env=dict(os.environ,TEMPO2=str(runtime),REFERENCE_RESOURCE_LOG=str(run/'resources.tsv'),LC_ALL='C')
    for key in ['REFERENCE_TEST_STORAGE_PATTERN','TEMPO2_ALIAS','TEMPO2_CLOCK_DIR']:env.pop(key,None)
    result=subprocess.run(command,cwd=run,env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=60)
    (run/'test.log').write_bytes(result.stdout)
    receipt['runs'][name]={'command':command,'runtime':str(runtime),'returncode':result.returncode,'log_sha256':hashlib.sha256(result.stdout).hexdigest(),'passed':result.returncode==0 and b'[  PASSED  ] 1 test.' in result.stdout,'log_tail':result.stdout.decode(errors='replace')[-1600:]}
    print(name,'exit',result.returncode,'passed',receipt['runs'][name]['passed'],flush=True)
    print(result.stdout.decode(errors='replace')[-1000:],flush=True)
assert receipt['runs']['current']['returncode']==1
assert b'1.13759e-09' in (out/'original-test-current/test.log').read_bytes()
(out/'original-test-receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
shutil.copy2(Path(__file__),out/'original-test-review.py')
print('receipt_sha256',hashlib.sha256((out/'original-test-receipt.json').read_bytes()).hexdigest())
