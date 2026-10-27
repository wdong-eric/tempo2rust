"""Keep the original DMMODEL race as evidence, never a comparison oracle."""
from pathlib import Path
import hashlib,json,os,subprocess,sys
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
out=Path(sys.argv[1]).resolve();out.mkdir(parents=True,exist_ok=False)
native=Path(sys.argv[2]).resolve();fixture=ROOT/'validation/input_to_bat/fixtures'
par=out/'warnings.par';par.write_text((fixture/'dmmodel_numeric.par').read_text().replace('_DM 55005 0.002','_DM 55005 0.004').replace('_CM 55005 -0.00001','_CM 55005 0.00003'))
row=(fixture/'ordinary.tim').read_text().splitlines()[1].split()
tim=out/'observations.tim';tim.write_text('FORMAT 1\n'+'\n'.join(' '.join([f'obs{i}',*row[1:]]) for i in range(259))+'\n')
env=dict(os.environ,TEMPO2=str(ROOT/'validation/reference/build/source/T2runtime'),OMP_NUM_THREADS='4',OMP_DYNAMIC='FALSE',OMP_THREAD_LIMIT='4',LC_ALL='C')
records=[];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();baseline=None
for mode in ['serial','parallel','racy-original']:
 for sample in range(1 if mode=='serial' else 8):
  label=f'{mode}-{sample}';dest=out/(label+'.tsv');exe=native/(mode+'-pipeline');cmd=list(map(str,[exe,dest,1,par,tim]))
  r=subprocess.run(cmd,env=env,cwd=out,capture_output=True,timeout=60);assert r.returncode==0
  (out/(label+'.stdout')).write_bytes(r.stdout);(out/(label+'.stderr')).write_bytes(r.stderr)
  scientific=sha(dest);lines=sorted(r.stdout.decode().splitlines())
  if baseline is None:baseline=(scientific,lines)
  if mode=='parallel':assert baseline==(scientific,lines)
  records.append(dict(mode=mode,sample=sample,command=cmd,executable_sha256=sha(exe),scientific_equal=scientific==baseline[0],diagnostic_multiset_equal=lines==baseline[1]))
(out/'receipt.json').write_text(json.dumps(dict(records=records,source={str(Path(__file__)):sha(__file__)},files={str(p.relative_to(out)):sha(p) for p in out.iterdir() if p.is_file()}),indent=2)+'\n')
print('original diagnostic differences',sum(not r['diagnostic_multiset_equal'] for r in records if r['mode']=='racy-original'))
