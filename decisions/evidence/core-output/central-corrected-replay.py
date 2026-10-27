"""Replay AH's committed corrected observer against the fresh production build."""
from pathlib import Path
import hashlib,json,os,subprocess,time
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
OUT=Path(Path('/tmp/tempo2-ah-committed-review-current').read_text().strip())
WS=OUT/'workspace'
REV='50f2bf33baaa3a0bfea0666c062ab5b59ba6b63b'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
archive=OUT/'corrected-validation.tar'
with archive.open('wb') as f:subprocess.run(['git','archive',REV,'validation/core_output'],cwd=ROOT,stdout=f,check=True)
subprocess.run(['/usr/bin/tar','-xf',str(archive),'-C',str(WS)],check=True)
for name,digest in json.loads((OUT/'production-source-check.json').read_text()).items():
 assert sha(WS/name)==digest,name
 assert hashlib.sha256(subprocess.check_output(['git','show',REV+':'+name],cwd=ROOT)).hexdigest()==digest,name
authored={}
for name in subprocess.check_output(['git','ls-tree','-r','--name-only',REV,'validation/core_output'],cwd=ROOT,text=True).splitlines():
 digest=hashlib.sha256(subprocess.check_output(['git','show',REV+':'+name],cwd=ROOT)).hexdigest()
 assert sha(WS/name)==digest,name
 authored[name]=digest
env=dict(os.environ,CARGO_HOME=str(ROOT/'rust/.cargo-home'),CARGO_TARGET_DIR=str(WS/'rust/target'),CARGO_NET_OFFLINE='true',FC='/opt/homebrew/bin/gfortran-16',PATH='/Users/Eric.Dong/.cargo/bin:/opt/homebrew/bin:'+os.environ.get('PATH',''),PYTHONDONTWRITEBYTECODE='1')
commands=[]
for name,cmd,expected in [('checker-controls',['python3',WS/'validation/core_output/test_check.py'],0),('matrix',['python3',WS/'validation/core_output/check.py','--native',ROOT/'validation/core_output/build/native-v9','--out',OUT/'matrix-release','--release'],1)]:
 print('Running',name,flush=True);t=time.monotonic()
 with (OUT/(name+'.log')).open('wb') as f:r=subprocess.run(list(map(str,cmd)),cwd=WS,env=env,stdout=f,stderr=subprocess.STDOUT,timeout=1200)
 commands.append({'name':name,'command':list(map(str,cmd)),'exit':r.returncode,'expected':expected,'seconds':time.monotonic()-t,'log_sha256':sha(OUT/(name+'.log'))})
 (OUT/'replay-commands.json').write_text(json.dumps(commands,indent=2)+'\n')
 assert r.returncode==expected,commands[-1]
 print('Completed',name,'exit',r.returncode,flush=True)
d=json.loads((OUT/'matrix-release/receipt.json').read_text())
assert len(d['comparison'])==37 and len(d['runs'])==79
assert d['summary']=={'unexpected_status':0,'changed_inputs':0,'artifact_mismatches':0,'different_files':17,'missing_files':3,'extra_files':0,'failed_cases':12,'exit_status':1},d['summary']
assert d['comparison']==json.loads((ROOT/'validation/core_output/build/release-final/receipt.json').read_text())['comparison']
assert 'Ran 4 tests' in (OUT/'checker-controls.log').read_text()
assert d['checker_sha256']==authored['validation/core_output/check.py']
for name,digest in authored.items():assert sha(WS/name)==digest,name
result={'validation_revision':REV,'overlay_archive_sha256':sha(archive),'authored_files':authored,'commands':commands,'summary':d['summary'],'matrix_receipt_sha256':sha(OUT/'matrix-release/receipt.json'),'rust_cli_sha256':sha(WS/'rust/target/release/tempo2-rust'),'checker_control_tests':4,'note':'Production built from 6ef441a archive, exactly equals 0dbcf08 and this revision. Only committed validation/core_output files overlaid; originals and both archives retained.'}
(OUT/'replay-completion.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'out':str(OUT),'result_sha256':sha(OUT/'replay-completion.json'),'summary':d['summary']},indent=2),flush=True)
