"""Focused final IAU/frontend regression; preserves the earlier complete receipts."""
import hashlib,json,os,shutil,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];RUST=ROOT/'rust';OUT=HERE/'build/followup'
ENV=dict(os.environ,CARGO_HOME=str(RUST/'.cargo-home'),GMP_MPFR_SYS_CACHE=str(RUST/'target/gmp-cache'),RUST_TEST_THREADS='1')
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def identity():return {str(p.relative_to(ROOT)):sha(p) for p in sorted((RUST/'src').glob('*.rs'))}
OUT.mkdir(parents=True,exist_ok=True);before=identity();records=[]
def command(name,args):
 r=subprocess.run(args,cwd=RUST,env=ENV,capture_output=True,text=True);log=OUT/(name+'.log');log.write_text(r.stdout+r.stderr);records.append(dict(name=name,command=args,status=r.returncode,log_sha256=sha(log)));assert r.returncode==0,r.stdout+r.stderr;print('PASS',name,flush=True);return r
command('format',['cargo','fmt','--check'])
command('clippy',['cargo','clippy','--all-targets','--locked','--offline','--','-D','warnings'])
profiles={}
for profile in ['debug','release']:
 r=command(profile+'-build',['cargo','test','--lib','--locked','--offline','--no-run','--message-format=json']+(['--release'] if profile=='release' else []))
 built=Path(next(x['executable'] for l in r.stdout.splitlines() if l.startswith('{') and (x:=json.loads(l)).get('executable') and x['target']['kind']==['lib']))
 exe=OUT/(profile+'-'+sha(built));shutil.copy2(built,exe);profiles[profile]=dict(executable=str(exe),sha256=sha(exe))
 command(profile+'-ownership-boundaries',[str(exe),'observer_coordinates_tests::','--test-threads=1'])
 command(profile+'-live-frontend',[str(exe),'observer_coordinates_tests::existing_frontend_observes_imag_alias_mutation','--exact','--ignored','--nocapture'])
assert identity()==before
rustc=Path(subprocess.check_output(['rustup','which','rustc'],text=True).strip())
(OUT/'receipt.json').write_text(json.dumps(dict(status='PASS',sources=before,commands=records,profiles=profiles,compiler=str(rustc),compiler_sha256=sha(rustc),script_sha256=sha(__file__),scope='single IAU boundary, three ownership/boundary tests and isolated public frontend IMAG history per profile'),indent=2)+'\n')
