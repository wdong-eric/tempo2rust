"""Matched model/input work, bounded repeated runs; no performance pass gate.

raw-bats compares the complete loaded formBatsAll call without output projection.
Native still writes its TSV after all timed evaluations. CLI cases request the
same formatted files from both engines. Every process/resource record is kept.
"""
from pathlib import Path
import argparse,datetime,hashlib,json,os,random,re,shutil,statistics,subprocess,time
ROOT=Path(__file__).resolve().parents[2]
p=argparse.ArgumentParser();p.add_argument('out',type=Path);p.add_argument('--native',type=Path,required=True);p.add_argument('--cli',type=Path,required=True);p.add_argument('--rust',type=Path,required=True);p.add_argument('--inputs',type=Path,required=True);a=p.parse_args()
out=a.out.resolve();out.mkdir(parents=True,exist_ok=False)
native=a.native.resolve();cli=a.cli.resolve();rust=a.rust.resolve();generated=a.inputs.resolve()
bins=out/'bin';bins.mkdir()
shutil.copy2(cli,bins/'cli');shutil.copy2(rust,bins/'raw-bats')
cli=bins/'cli';rust=bins/'raw-bats'
shutil.copy2(__file__,out/'executed-performance.py')
runtime=ROOT/'validation/reference/build/source/T2runtime';fit=ROOT/'validation/fitting/inputs';general=ROOT/'validation/general_fitting/inputs';pred=ROOT/'validation/predictors/fixtures/isolated.par'
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
cases={
 'small':('raw',[fit/'pks_all.par',fit/'pks.tim']),
 'unequal':('raw',[fit/'pks_all.par',generated/'n259.tim',fit/'pks_all.par',generated/'n17.tim']),
 'large':('raw',[fit/'pks_all.par',generated/'n4099.tim',fit/'pks_all.par',generated/'n2051.tim']),
 'correlated-global':('cli',['-f',general/'global0.par',general/'multi-frequency.tim','-f',general/'global1.par',general/'multi-frequency.tim','-dcf',general/'covariance.model','-outpar','fitted.par']),
 'cheby':('cli',['-f',pred,'-pred','PKS 58000 58000.03 1000 2000 8 4 3600']),
 'polyco':('cli',['-f',pred,'-polyco','58000 58000.01 60 8 12 PKS 1400','-polyco_file','literal-'])}
jobs=[(case,engine,workers,sample) for case in cases for engine in ['native','rust'] for workers in [1,2,4] for sample in range(3)]
random.Random(853).shuffle(jobs);records=[];products={};identities={str(p):sha(p) for p in [Path(__file__),rust,cli,*native.glob('*-pipeline'),*native.glob('*-tempo2')]}
for case,engine,workers,sample in jobs:
 kind,args=cases[case];wd=out/f'{case}-{engine}-{workers}-{sample}';wd.mkdir()
 env=dict(os.environ,TEMPO2=str(runtime),LC_ALL='C',AK_WORKERS=str(workers),AK_REPEATS='5',OMP_NUM_THREADS=str(workers),OMP_DYNAMIC='FALSE',OMP_THREAD_LIMIT='4',OMP_MAX_ACTIVE_LEVELS='1',VECLIB_MAXIMUM_THREADS='1',OPENBLAS_NUM_THREADS='1')
 if kind=='raw':
  cmd=[native/('serial-pipeline' if workers==1 else 'parallel-pipeline'),wd/'result.tsv',len(args)//2,*args] if engine=='native' else [rust,'raw-bats',workers,5,runtime,wd/'result.json','-',*args]
 else:
  exe=native/('serial-tempo2' if workers==1 else 'parallel-tempo2') if engine=='native' else cli
  cmd=[exe,'-npsr',2,'-nobs',10000,'-noaccel',*args]
 cmd=list(map(str,cmd));usage=wd/'usage.txt';start=time.perf_counter();started=datetime.datetime.now(datetime.timezone.utc).isoformat()
 with (wd/'stdout').open('wb') as stdout,(wd/'stderr').open('wb') as stderr:
  r=subprocess.run(['/usr/bin/time','-l','-o',str(usage),*cmd],cwd=wd,env=env,stdout=stdout,stderr=stderr,timeout=180)
 wall=time.perf_counter()-start;raw=usage.read_text();m=re.search(r'([\d.]+) real\s+([\d.]+) user\s+([\d.]+) sys',raw);rss=int(re.search(r'(\d+)\s+maximum resident set size',raw)[1])
 record=dict(case=case,engine=engine,workers=workers,sample=sample,command=cmd,started_utc=started,wall_s=wall,process_cpu_s=float(m[2])+float(m[3]),maxrss_bytes=rss,exit=r.returncode)
 if kind=='raw':
  if engine=='rust':
   v=json.loads((wd/'stdout').read_text());record.update(load_s=v['load_s'],evaluate_s=v['evaluate_and_project_s'])
  else:
   s=(wd/'stderr').read_text();record.update(load_s=float(re.search(r'AK_LOAD ([\d.]+)',s)[1]),evaluate_s=[float(v) for v in re.findall(r'AK_EVALUATE \d+ ([\d.]+)',s)])
 record['worker_evidence']='requested count; actual observation visits proven separately in instrumented calibration'
 record['process_comparison']='matched CLI input/output workload' if kind=='cli' else 'distinct post-evaluation output workloads; compare load/evaluation intervals only'
 records.append(record);(out/'records.json').write_text(json.dumps(records,indent=2)+'\n')
 assert r.returncode==0,(wd,(wd/'stderr').read_text()[-1800:])
 if kind=='cli':
  product={p.name:sha(p) for p in wd.iterdir() if p.is_file() and p.name not in ['stdout','stderr','usage.txt']}
  assert product,(wd,'no output product')
  key=(case,engine)
  if key in products:assert product==products[key],(wd,'worker/sample output mismatch')
  else:products[key]=product
 for value in args:
  f=Path(str(value))
  if f.is_absolute() and f.is_file():identities[str(f)]=sha(f)
 print(wd.name,f'{wall:.3f}s {rss/2**20:.1f}MiB',flush=True)
summary=[]
for case in cases:
 for engine in ['native','rust']:
  for workers in [1,2,4]:
   rows=[r for r in records if (r['case'],r['engine'],r['workers'])==(case,engine,workers)]
   item=dict(case=case,engine=engine,workers=workers)
   for field in ['wall_s','process_cpu_s','maxrss_bytes','load_s']:
    if field in rows[0]:
     values=[r[field] for r in rows];item[field]=dict(median=statistics.median(values),min=min(values),max=max(values))
   if 'evaluate_s' in rows[0]:
    for field,indexes in [('first_evaluation_s',[0]),('repeated_evaluation_s',[1,2,3,4])]:
     values=[r['evaluate_s'][i] for r in rows for i in indexes];item[field]=dict(median=statistics.median(values),min=min(values),max=max(values))
   summary.append(item)
(out/'receipt.json').write_text(json.dumps(dict(identities=identities,records=records,summary=summary,environment={k:env[k] for k in ['LC_ALL','AK_REPEATS','OMP_DYNAMIC','OMP_THREAD_LIMIT','OMP_MAX_ACTIVE_LEVELS','VECLIB_MAXIMUM_THREADS','OPENBLAS_NUM_THREADS']},files={str(p.relative_to(out)):sha(p) for p in out.rglob('*') if p.is_file()}),indent=2)+'\n')
