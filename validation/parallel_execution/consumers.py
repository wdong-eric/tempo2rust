"""Run the separately compiled public callers, retaining complete products."""
from pathlib import Path
import argparse,hashlib,json,os,subprocess
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
p=argparse.ArgumentParser();p.add_argument('out',type=Path);p.add_argument('--client',type=Path,required=True);a=p.parse_args()
out=a.out.resolve();out.mkdir(parents=True,exist_ok=False);client=a.client.resolve()
runtime=ROOT/'validation/reference/build/source/T2runtime';fit=ROOT/'validation/fitting/inputs';general=ROOT/'validation/general_fitting/inputs';pred=ROOT/'validation/predictors/fixtures/isolated.par'
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
records=[];binaries={}
for profile in ['debug','release']:
 baselines={}
 for workers in [1,2,4]:
  for mode in ['core','toolkit','predictor_workflows','parallel_execution']:
   exe=client/'target'/profile/mode;binaries[str(exe)]=sha(exe)
   wd=out/f'{profile}-{mode}-{workers}';wd.mkdir()
   if mode=='core':args=[runtime,fit/'pks_all.par',fit/'pks.tim',fit/'pks_all.par',fit/'pks.tim']
   elif mode=='toolkit':args=[]
   elif mode=='predictor_workflows':args=[pred,runtime,wd/'products']
   else:args=['fit',workers,2,runtime,wd/'fit.json',general/'covariance.model',general/'global0.par',general/'multi-frequency.tim',general/'global1.par',general/'multi-frequency.tim']
   cmd=list(map(str,[exe,*args]));env=dict(os.environ,AK_WORKERS=str(workers),TEMPO2=str(runtime),LC_ALL='C',VECLIB_MAXIMUM_THREADS='1')
   r=subprocess.run(cmd,cwd=wd,env=env,capture_output=True,timeout=120)
   (wd/'stdout').write_bytes(r.stdout);(wd/'stderr').write_bytes(r.stderr)
   records.append(dict(name=wd.name,command=cmd,workers=workers,exit=r.returncode));assert r.returncode==0,(wd,r.stderr[-1500:])
   # Working directory is explicit fit provenance; only the generated caller
   # directory differs. Retain the raw bytes and normalize that path identity.
   if mode=='parallel_execution':value=json.loads((wd/'fit.json').read_text().replace(str(wd),'$CALLER_OUTPUT'))
   elif mode=='predictor_workflows':
    value={str(f.relative_to(wd)):f.read_bytes().replace(str(wd).encode(),b'$CALLER_OUTPUT') for f in [wd/'stdout',*(wd/'products').rglob('*')] if f.is_file()}
   else:value=r.stdout
   if mode in baselines:assert value==baselines[mode],(profile,mode,workers)
   else:baselines[mode]=value
   print(wd.name,'passed',flush=True)
 exe=client/'target'/profile/'concurrent';binaries[str(exe)]=sha(exe)
 wd=out/f'{profile}-concurrent';wd.mkdir()
 cmd=list(map(str,[exe,runtime,fit/'pks_all.par',fit/'pks.tim',pred,general/'covariance.model',wd/'products']))
 r=subprocess.run(cmd,cwd=wd,capture_output=True,timeout=120,env=dict(os.environ,LC_ALL='C',VECLIB_MAXIMUM_THREADS='1'))
 (wd/'stdout').write_bytes(r.stdout);(wd/'stderr').write_bytes(r.stderr);records.append(dict(name=wd.name,command=cmd,exit=r.returncode));assert r.returncode==0,(wd,r.stderr[-2000:])
 print(wd.name,'passed',flush=True)
(out/'receipt.json').write_text(json.dumps(dict(records=records,binaries=binaries,runner={str(Path(__file__)):sha(__file__)},files={str(p.relative_to(out)):sha(p) for p in out.rglob('*') if p.is_file()}),indent=2)+'\n')
