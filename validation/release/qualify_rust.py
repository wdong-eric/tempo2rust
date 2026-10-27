#!/usr/bin/env python3
"""Compile separate Cargo consumers against the installed source only."""
from pathlib import Path
import argparse, hashlib, json, os, shutil, subprocess
ROOT=Path(__file__).resolve().parents[2];HERE=ROOT/'validation/library_interfaces'
p=argparse.ArgumentParser();p.add_argument('--prefix',type=Path,required=True);p.add_argument('--out',type=Path,required=True);p.add_argument('--release',action='store_true');p.add_argument('--features',default='');p.add_argument('--runtime',type=Path,required=True);p.add_argument('--ephemeris',type=Path,required=True);p.add_argument('--source-root',type=Path,required=True);a=p.parse_args();ROOT=a.source_root.resolve();HERE=ROOT/'validation/library_interfaces'
out=a.out.resolve();out.mkdir(parents=True,exist_ok=False);crate=out/'client';(crate/'src/bin').mkdir(parents=True)
installed=a.prefix.resolve()/'share/tempo2-rust/rust';assert (installed/'data/help.txt').exists()
manifest='''[package]
name = "tempo2-external-consumers"
version = "0.1.0"
edition = "2024"
[features]
default = []
blas = ["tempo2-rust/blas"]
lapack = ["blas", "tempo2-rust/lapack"]
calceph = ["tempo2-rust/calceph"]
qr-default = ["tempo2-rust/qr-default"]
[dependencies]
'''+f'tempo2-rust = {{path = {json.dumps(str(installed))}}}\n'+'''rug = { version = "=1.30.0", default-features = false, features = ["float", "integer", "std"] }
serde = { version = "=1.0.229", features = ["derive"] }
serde_json = "=1.0.151"
sha2 = "=0.10.9"
'''
(crate/'Cargo.toml').write_text(manifest)
exclude={'compare_fixtures','compare_fitting','compare_terrestrial','check_terrestrial_resources','check_input_to_bat_lifecycle'}
source={}
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
for f in sorted((ROOT/'rust/examples').glob('*.rs')):
 if f.stem not in exclude:shutil.copy2(f,crate/'src/bin'/f.name);source[str(f)]=sha(f)
shutil.copytree(ROOT/'rust/examples/analysis_support',crate/'src/bin/analysis_support')
for f in sorted((ROOT/'rust/examples/analysis_support').glob('*.rs')):source[str(f)]=sha(f)
for name in ['resources','core']:shutil.copy2(HERE/(name+'.rs'),crate/'src/bin'/(name+'.rs'));source[name]=sha(HERE/(name+'.rs'))
shutil.copy2(ROOT/'validation/parallel_execution/concurrent.rs',crate/'src/bin/concurrent.rs')
source['concurrent']=sha(ROOT/'validation/parallel_execution/concurrent.rs')
commands=[]
def run(command,name,cwd=None):
 command=list(map(str,command));r=subprocess.run(command,cwd=cwd,capture_output=True);commands.append({'command':command,'cwd':str(cwd),'status':r.returncode})
 (out/(name+'.stdout')).write_bytes(r.stdout);(out/(name+'.stderr')).write_bytes(r.stderr)
 assert r.returncode==0,(name,r.stderr.decode(errors='replace'))
 return r
build=['cargo','build','--offline','--manifest-path',crate/'Cargo.toml','--target-dir',crate/'target','--bins']
if a.release:build+=['--release']
if a.features:build+=['--features',a.features]
run(['cargo','generate-lockfile','--offline','--manifest-path',crate/'Cargo.toml'],'lock');build+=['--locked'];run(build,'build');run(['cargo','tree','--offline','--manifest-path',crate/'Cargo.toml'],'dependency-tree')
profile='release' if a.release else 'debug';bins=crate/'target'/profile
runtime=a.runtime.resolve();fit=ROOT/'validation/general_fitting/inputs'
work=out/'run';work.mkdir()
executed=0
def execute(name,args=[]):
 global executed
 executed+=1
 wd=work/(name+'-'+str(len(commands)));wd.mkdir();return run([bins/name,*args],name+'-'+str(len(commands)),cwd=wd)
execute('resources')
execute('toolkit')
execute('fitting_bootstrap',['rng','-12345','1300'])
execute('fitting_svd')
execute('fitting_covariance',['fft','1','32'])
execute('fitting_covariance',['fft','-1','32'])
execute('core',[runtime,fit/'global0.par',fit/'multi-frequency.tim',fit/'global1.par',fit/'multi-frequency.tim'])
execute('fitting_prepared',[runtime,fit/'robust-H.par',fit/'multi-frequency.tim'])
(work/'fit-recovery').mkdir()
execute('fitting_recovery',[runtime,fit/'robust-H.par',fit/'multi-frequency.tim',work/'fit-recovery'])
for kind in ['power','beta','redpink','qp','sqexp','matern15','matern25','dmparam','dm','mask']:execute('fitting_kernels',[kind])
for kind in ['spectrum','sigma','gw']:execute('analysis_components',['--case',kind])
execute('predictor_components',['polynomial']);execute('predictor_subnormal')
execute('predictor_workflows',[ROOT/'validation/predictors/fixtures/isolated.par',runtime,work/'predictor-workflows'])
execute('native_backends',['kernels',work/'native-kernels.tsv'])
execute('native_backends',['failures',work/'native-failures.tsv'])
analysis_args=['--runtime',runtime,'--par',fit/'robust-H.par','--tim',fit/'multi-frequency.tim']
execute('spectrum',analysis_args)
execute('gw_simulation',analysis_args+['--output',work/'gw'])
if 'lapack' in a.features:
 for constraint in ['0','1']:
  for scale in ['0','1']:
   for mask in range(8):execute('native_backends',['solver',work/f'qr-{constraint}-{scale}-{mask}.tsv',constraint,scale,str(mask)])
if 'calceph' in a.features:
 ephemeris=a.ephemeris.resolve()
 for units in ['1','2']:
  for ecliptic in ['0','1']:execute('native_backends',['calceph',work/f'calceph-{units}-{ecliptic}.tsv',ephemeris,units,ecliptic])

execute('concurrent',[runtime,ROOT/'validation/fitting/inputs/pks_all.par',ROOT/'validation/fitting/inputs/pks.tim',ROOT/'validation/predictors/fixtures/isolated.par',fit/'covariance.model',work/'concurrent-products'])
receipt={'source':source,'installed_tree':{str(p.relative_to(installed)):sha(p) for p in sorted(installed.rglob('*')) if p.is_file()},
'commands':commands,'executed_clients':executed,'lockfile':sha(crate/'Cargo.lock'),'binaries':{p.name:sha(p) for p in bins.iterdir() if p.is_file() and p.suffix==''},'script':sha(Path(__file__))}
(out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');print(f'{len(source)} separately compiled sources; {executed} executed clients')
