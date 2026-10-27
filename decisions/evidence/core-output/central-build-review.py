"""Build AH's committed production while the observer-only correction proceeds."""
from pathlib import Path
import hashlib,json,os,subprocess,tempfile,time
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
OUT=Path(tempfile.mkdtemp(prefix='tempo2-ah-committed-review-'))
Path('/tmp/tempo2-ah-committed-review-current').write_text(str(OUT)+'\n')
REV='6ef441a1a46a964770352bebdc3395ef12045c10'
PROD='0dbcf0842394eb9cfaf93cb31bfac428e2ef7d67'
WS=OUT/'workspace';WS.mkdir()
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
archive=OUT/'source.tar'
with archive.open('wb') as f:subprocess.run(['git','archive',REV],cwd=ROOT,stdout=f,check=True)
subprocess.run(['/usr/bin/tar','-xf',str(archive),'-C',str(WS)],check=True)
for name in ['tempo2','validation/reference/build']:
 p=WS/name
 if p.exists():p.rmdir()
 p.parent.mkdir(parents=True,exist_ok=True)
 p.symlink_to(ROOT/name,target_is_directory=True)
env=dict(os.environ,CARGO_HOME=str(ROOT/'rust/.cargo-home'),CARGO_TARGET_DIR=str(WS/'rust/target'),CARGO_NET_OFFLINE='true',FC='/opt/homebrew/bin/gfortran-16',PATH='/Users/Eric.Dong/.cargo/bin:/opt/homebrew/bin:'+os.environ.get('PATH',''),PYTHONDONTWRITEBYTECODE='1')
commands=[]
def run(name,cmd,extra=None):
 print('Running',name,'in',OUT,flush=True);t=time.monotonic()
 with (OUT/(name+'.log')).open('wb') as f:r=subprocess.run(list(map(str,cmd)),cwd=WS,env=dict(env,**(extra or {})),stdout=f,stderr=subprocess.STDOUT,timeout=1200)
 commands.append({'name':name,'command':list(map(str,cmd)),'exit':r.returncode,'seconds':time.monotonic()-t,'log_sha256':sha(OUT/(name+'.log'))})
 (OUT/'commands.json').write_text(json.dumps(commands,indent=2)+'\n')
 assert r.returncode==0,commands[-1]
 print('Completed',name,flush=True)
cargo=['cargo','test','--manifest-path',WS/'rust/Cargo.toml','--release','--locked','--offline']
run('build',['cargo','build','--manifest-path',WS/'rust/Cargo.toml','--release','--locked','--offline','--bin','tempo2-rust'])
run('core-tests',cargo+['--lib','core_','--','--test-threads=1'])
run('behavior',cargo+['--test','behavior','--','--test-threads=1'])
run('ut1-snapshot',cargo+['--lib','core_output_tests::write_ut1_evidence','--','--ignored','--test-threads=1'],{'AH_UT1_DIR':str(OUT/'ut1-after')})
run('ut1-check',['python3',WS/'validation/core_output/check_ut1.py','--native',ROOT/'validation/core_output/build/native-v9','--before',ROOT/'validation/core_output/build/ut1-before','--after',OUT/'ut1-after','--out',OUT/'ut1-check'])
result={'production':PROD,'validation':REV,'source_archive_sha256':sha(archive),'commands':commands,'rust_cli_sha256':sha(WS/'rust/target/release/tempo2-rust'),'ut1_receipt_sha256':sha(OUT/'ut1-check/receipt.json'),'scope':'fresh committed-source build and focused AH checks; matrix replay awaits observer correction'}
(OUT/'build-completion.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'out':str(OUT),'result_sha256':sha(OUT/'build-completion.json')},indent=2),flush=True)
