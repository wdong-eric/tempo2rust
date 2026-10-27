"""Fresh private install, external source consumer and separately linked C client."""
from pathlib import Path
import hashlib,json,os,shutil,subprocess,sys
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
out=Path(sys.argv[1]).resolve();out.mkdir(parents=True,exist_ok=False);prefix=out/'prefix';records=[]
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
def run(label,cmd,cwd=ROOT):
 cmd=list(map(str,cmd))
 with (out/(label+'.stdout')).open('wb') as stdout,(out/(label+'.stderr')).open('wb') as stderr:
  r=subprocess.run(cmd,cwd=cwd,stdout=stdout,stderr=stderr,timeout=600,env=dict(os.environ,LC_ALL='C',VECLIB_MAXIMUM_THREADS='1'))
 records.append(dict(label=label,command=cmd,exit=r.returncode));assert r.returncode==0,(label,(out/(label+'.stderr')).read_text()[-2000:]);print(label,'passed',flush=True)
run('install',['python3',ROOT/'rust/install.py','--prefix',prefix])
source=prefix/'share/tempo2-rust/rust';installed={}
for directory in ['src','native','include','data']:
 for original in (ROOT/'rust'/directory).rglob('*'):
  if original.is_file():
   relative=original.relative_to(ROOT/'rust');assert sha(original)==sha(source/relative),relative;installed[str(relative)]=sha(source/relative)
for name in ['Cargo.toml','Cargo.lock','build.rs','LICENSE','MT19937_LICENSE.txt','jpl_source_manifest.json']:
 assert sha(ROOT/'rust'/name)==sha(source/name);installed[name]=sha(source/name)
client=out/'client';shutil.copytree(HERE/'build/external-v4/client',client,ignore=shutil.ignore_patterns('target'))
manifest=client/'Cargo.toml';manifest.write_text(manifest.read_text().replace(json.dumps(str(ROOT/'rust')),json.dumps(str(source))))
run('external-build',['cargo','build','--offline','--locked','--release','--manifest-path',manifest,'--bin','concurrent'])
fit=ROOT/'validation/fitting/inputs'
run('external-concurrent',[client/'target/release/concurrent',ROOT/'validation/reference/build/source/T2runtime',fit/'pks_all.par',fit/'pks.tim',ROOT/'validation/predictors/fixtures/isolated.par',ROOT/'validation/general_fitting/inputs/covariance.model',out/'products'])
run('c-build',['clang','-std=c11','-O1','-I'+str(prefix/'include'),ROOT/'validation/library_interfaces/consumer.c','-L'+str(prefix/'lib'),'-ltempo2pred_rust','-Wl,-rpath,'+str(prefix/'lib'),'-o',out/'consumer'])
run('c-consumer',[out/'consumer'],out)
run('linkage',['otool','-L',out/'consumer'])
(out/'receipt.json').write_text(json.dumps(dict(records=records,installed_source=installed,inputs={str(p):sha(p) for p in [Path(__file__),manifest,ROOT/'rust/install.py',ROOT/'validation/library_interfaces/consumer.c']},files={str(p.relative_to(out)):sha(p) for p in out.rglob('*') if p.is_file() and 'target' not in p.parts}),indent=2)+'\n')
