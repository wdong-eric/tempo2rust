"""Fresh native serial/OpenMP and exact Rust worker-count comparisons.

Reuse AC's unchanged native/Rust quantity gates. Rust count changes use exact
complete JSON equality, including messages and provenance. Native SWM effects
are additionally tagged per observation by the executed observer.
"""
from pathlib import Path
import argparse,hashlib,json,os,re,subprocess,sys,time
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
sys.path.insert(0,str(ROOT/'validation/input_to_bat'))
from check_pipeline import compare
p=argparse.ArgumentParser();p.add_argument('out',type=Path);p.add_argument('--native',type=Path,required=True);p.add_argument('--rust',type=Path,required=True);a=p.parse_args()
out=a.out.resolve();out.mkdir(parents=True,exist_ok=False);native=a.native.resolve();rust=a.rust.resolve()
fixture=ROOT/'validation/input_to_bat/fixtures';runtime=ROOT/'validation/reference/build/source/T2runtime'
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
records=[]
def run(name,cmd,workers,rt):
 wd=out/name;wd.mkdir();usage=wd/'usage.txt';start=time.perf_counter()
 env=dict(os.environ,LC_ALL='C',TEMPO2=str(rt),OMP_NUM_THREADS=str(workers),OMP_DYNAMIC='FALSE',OMP_THREAD_LIMIT='4',OMP_MAX_ACTIVE_LEVELS='1',VECLIB_MAXIMUM_THREADS='1',OPENBLAS_NUM_THREADS='1')
 with (wd/'stdout').open('wb') as stdout,(wd/'stderr').open('wb') as stderr:
  r=subprocess.run(['/usr/bin/time','-l','-o',str(usage),*map(str,cmd)],cwd=wd,env=env,stdout=stdout,stderr=stderr,timeout=180)
 rec=dict(name=name,command=list(map(str,cmd)),exit=r.returncode,wall_s=time.perf_counter()-start,environment={k:env[k] for k in ['OMP_NUM_THREADS','OMP_DYNAMIC','OMP_THREAD_LIMIT','OMP_MAX_ACTIVE_LEVELS','VECLIB_MAXIMUM_THREADS','LC_ALL']},usage=usage.read_text())
 records.append(rec);assert r.returncode==0,(name,(wd/'stderr').read_text()[-1200:]);return wd
ordinary=(fixture/'ordinary.par',fixture/'ordinary.tim')
cases=[('ordinary',[ordinary],runtime),('ecliptic',[(fixture/'ecliptic.par',ordinary[1])],runtime),('mixed-disabled',[ordinary,(fixture/'fb90.par',ordinary[1])],runtime)]
for mode in ['dispersion','chromx','dmmodel_numeric','shapelet','secular']:
 cases.append((mode,[(fixture/(mode+'.par'),ordinary[1])],runtime))
solar=out/'solar-runtime';solar.mkdir()
for child in runtime.iterdir():
 if child.name!='solarWindModel':(solar/child.name).symlink_to(child,target_is_directory=child.is_dir())
(solar/'solarWindModel').mkdir()
for n in range(2100,2301):(solar/'solarWindModel'/f'CS{n}.txt').write_bytes((fixture/'synthetic-current-sheet.txt').read_bytes())
cases.append(('solarwind',[(fixture/'solarwind.par',fixture/'solarwind.tim')],solar))
# Larger deterministic populations repeat actual validated epochs/frequencies;
# only row names change. No extrapolated ephemeris or fabricated physics.
generated=out/'inputs';generated.mkdir()
base=(ROOT/'validation/fitting/inputs/pks.tim').read_text().splitlines()[1:]
for n in [17,259,2051,4099]:
 rows=[]
 for i in range(n):
  fields=base[i%len(base)].split();fields[0]=f'ak{i}';rows.append(' '.join(fields))
 (generated/f'n{n}.tim').write_text('FORMAT 1\n'+'\n'.join(rows)+'\n')
par=ROOT/'validation/fitting/inputs/pks_all.par'
cases += [('unequal',[(par,generated/'n259.tim'),(par,generated/'n17.tim')],runtime),('large',[(par,generated/'n4099.tim'),(par,generated/'n2051.tim')],runtime)]
results=[]
for name,pairs,rt in cases:
 baseline=None;rust_base=None;events=None;stdout=None
 for workers in [1,2,4]:
  exe=native/('serial-pipeline' if workers==1 else 'parallel-pipeline')
  dest=out/(name+f'-native-{workers}.tsv')
  wd=run(name+f'-native-{workers}',[exe,dest,len(pairs),*[x for pair in pairs for x in pair]],workers,rt)
  text=dest.read_text();diag=(wd/'stderr').read_text().splitlines();actual=[list(map(int,m)) for m in re.findall(r'AK_WORKERS pulsar=(\d+) actual=(\d+) observations=(\d+)',(wd/'stderr').read_text())]
  assert actual and all(w==min(workers,n) for _,w,n in actual),(name,workers,actual)
  effect=[line for line in diag if line.startswith('AK_EVENT')]
  lines=sorted((wd/'stdout').read_text().splitlines())
  if baseline is None:baseline=text;events=effect;stdout=lines
  else:assert baseline==text,(name,'native scientific values');assert events==effect,(name,'native per-observation effects');assert stdout==lines,(name,'native diagnostic multiset')
  dest_r=out/(name+f'-rust-{workers}.json')
  run(name+f'-rust-{workers}',[rust,'bats',workers,1,rt,dest_r,'-',*[x for pair in pairs for x in pair]],workers,rt)
  result=json.loads(dest_r.read_text())[0]
  if rust_base is None:rust_base=result
  else:assert result==rust_base,(name,'Rust complete output')
  failures,measurements=compare(baseline,result)
  results.append(dict(case=name,workers=workers,actual_native=actual,checks=len(measurements),failures=failures,native_effect_events=len(effect)))
  print(name,workers,'checks',len(measurements),'failures',len(failures),flush=True)
 # Repeated evaluation timing is separately measured on the same immutable inputs.
 for workers in [1,2,4]:
  dest=out/(name+f'-repeated-{workers}.json')
  run(name+f'-repeated-{workers}',[rust,'bats',workers,3,rt,dest,'-',*[x for pair in pairs for x in pair]],workers,rt)
  value=json.loads(dest.read_text())
  if workers==1:repeated=value
  else:assert repeated==value,(name,'repeated exact Rust histories')
receipt=dict(sources={str(p):sha(p) for p in [Path(__file__),ROOT/'validation/input_to_bat/check_pipeline.py',rust,*native.glob('*-pipeline')]},records=records,results=results,files={str(p.relative_to(out)):sha(p) for p in out.rglob('*') if p.is_file() and not p.is_symlink()},failures=sum(len(v['failures']) for v in results))
(out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');assert receipt['failures']==0
