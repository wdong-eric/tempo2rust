"""Actual combined provider build and existing AI comparisons, under AK scopes."""
from pathlib import Path
import argparse,hashlib,json,os,shutil,subprocess
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
p=argparse.ArgumentParser();p.add_argument('out',type=Path);p.add_argument('--client',type=Path,required=True);p.add_argument('--reuse',type=Path);a=p.parse_args()
out=a.out.resolve();out.mkdir(parents=True,exist_ok=False)
client=(a.reuse.resolve() if a.reuse else out)/'client'
paired=(a.reuse.resolve() if a.reuse else out)/'paired'
if not a.reuse:shutil.copytree(a.client,client,ignore=shutil.ignore_patterns('target'))
env=dict(os.environ,TEMPO2=str(ROOT/'validation/reference/build/source/T2runtime'),CALCEPH_DIR=str(ROOT/'validation/native_backends/build/provider'),TEMPO2_ACCELERATE='1',AK_WORKERS='4',VECLIB_MAXIMUM_THREADS='1',LC_ALL='C')
records=[];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
def run(name,cmd,cwd=ROOT):
 cmd=list(map(str,cmd))
 with (out/(name+'.stdout')).open('wb') as stdout,(out/(name+'.stderr')).open('wb') as stderr:
  r=subprocess.run(cmd,cwd=cwd,env=env,stdout=stdout,stderr=stderr,timeout=600)
 records.append(dict(name=name,command=cmd,exit=r.returncode));assert r.returncode==0,(name,(out/(name+'.stderr')).read_text()[-2000:])
 print(name,'passed',flush=True)
if not a.reuse:run('build',['cargo','build','--offline','--locked','--release','--features','calceph,lapack','--manifest-path',client/'Cargo.toml','--bins'])
exe=client/'target/release/native_backends'
if not a.reuse:run('paired',[ 'python3',ROOT/'validation/native_backends/check.py',paired,'--profile','combined','--rust',exe,'--native',ROOT/'validation/native_backends/build/profiles-final-v3/combined-release/native/probe'])
inputs=paired/'inputs';cov=ROOT/'validation/general_fitting/inputs/covariance.model'
for mode in [0,1,2]:
 baseline=None
 for workers in [1,2,4]:
  env['AK_WORKERS']=str(workers);dest=out/f'worker-{workers}-mode-{mode}.tsv'
  run(f'worker-{workers}-mode-{mode}',[exe,'pipeline',dest,mode,cov,inputs/'calceph.par',inputs/'pks.tim',inputs/'calceph.par',inputs/'pks2.tim'])
  data=dest.read_bytes()
  if baseline is None:baseline=data
  else:assert data==baseline,(mode,workers)
run('concurrent',[client/'target/release/concurrent',ROOT/'validation/reference/build/source/T2runtime',inputs/'calceph.par',inputs/'pks.tim',inputs/'calceph.par',cov,out/'concurrent-products'])
run('linkage',['otool','-L',exe])
(out/'receipt.json').write_text(json.dumps(dict(records=records,paired_receipt={str(paired/'receipt.json'):sha(paired/'receipt.json')},environment={k:env[k] for k in ['TEMPO2','CALCEPH_DIR','TEMPO2_ACCELERATE','AK_WORKERS','VECLIB_MAXIMUM_THREADS','LC_ALL']},binaries={str(p):sha(p) for p in [exe,client/'target/release/concurrent']},files={str(p.relative_to(out)):sha(p) for p in out.rglob('*') if p.is_file() and 'target' not in p.parts}),indent=2)+'\n')
