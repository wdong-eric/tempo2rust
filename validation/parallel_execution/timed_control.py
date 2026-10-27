"""Focused instrumented/uninstrumented native product equivalence control."""
from pathlib import Path
import hashlib,json,os,subprocess,sys
base=Path(sys.argv[1]).resolve();native=Path(sys.argv[2]).resolve();out=Path(sys.argv[3]).resolve();out.mkdir(parents=True,exist_ok=False)
ROOT=Path(__file__).resolve().parents[2];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
records=[]
for case in ['small','unequal','large','correlated-global','cheby','polyco']:
 for workers in [1,4]:
  old=base/f'{case}-native-{workers}-0';wd=out/f'{case}-{workers}';wd.mkdir()
  row=next(r for r in json.loads((base/'records.json').read_text()) if (r['case'],r['engine'],r['workers'],r['sample'])==(case,'native',workers,0))
  cmd=[str(native/Path(row['command'][0]).name),*[s.replace(str(old),str(wd)) for s in row['command'][1:]]]
  env=dict(os.environ,TEMPO2=str(ROOT/'validation/reference/build/source/T2runtime'),AK_REPEATS='5',OMP_NUM_THREADS=str(workers),OMP_DYNAMIC='FALSE',OMP_THREAD_LIMIT='4',LC_ALL='C')
  r=subprocess.run(cmd,cwd=wd,env=env,capture_output=True,timeout=120);assert r.returncode==0,(case,r.stderr[-1000:])
  (wd/'stdout').write_bytes(r.stdout);(wd/'stderr').write_bytes(r.stderr)
  ignored=['stdout','stderr','usage.txt'];before={p.name:sha(p) for p in old.iterdir() if p.is_file() and p.name not in ignored};after={p.name:sha(p) for p in wd.iterdir() if p.is_file() and p.name not in ignored}
  assert before==after,(case,workers,before,after)
  records.append(dict(case=case,workers=workers,command=cmd,executable=sha(cmd[0]),product_hashes=after))
(out/'receipt.json').write_text(json.dumps(dict(records=records,source={str(Path(__file__)):sha(__file__)},calibration_receipt=sha(base/'receipt.json'),files={str(p.relative_to(out)):sha(p) for p in out.rglob('*') if p.is_file()}),indent=2)+'\n')
print('Twelve instrumented/uninstrumented native product comparisons passed')
