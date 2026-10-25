#!/usr/bin/env python3
"""Read-only Q corpus adapter; initial state and all field deltas remain complete."""
import argparse,gzip,hashlib,json,os,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parent.parent;Q=ROOT/'validation/par_specialized';BUILD=HERE/'build'/'par'
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--release',action='store_true');ap.add_argument('--case',action='append');a=ap.parse_args()
 BUILD.mkdir(exist_ok=True,parents=True)
 initial=json.loads(gzip.decompress((Q/'initial_state.json.gz').read_bytes()))
 (BUILD/'initial.json').write_text(json.dumps(initial)+'\n')
 reference=Q/'observations.json.gz';cases=json.loads(gzip.decompress(reference.read_bytes()))
 families=['TEL_CLK_OFFS','TEL_DX','TEL_DY','TEL_DZ','TEL_VX','TEL_VY','TEL_VZ']
 bounded=HERE/'build/telescope_boundaries'
 if not (bounded/'receipt.json').exists():subprocess.run(['python3',str(HERE/'telescope_boundaries.py')],check=True)
 proof=json.loads((bounded/'receipt.json').read_text())
 assert proof['cases_sha256']==hashlib.sha256((bounded/'cases.json').read_bytes()).hexdigest()
 assert proof['identity']['sha256']==hashlib.sha256(Path(proof['identity']['path']).read_bytes()).hexdigest()
 bounded_cases=json.loads((bounded/'cases.json').read_text())
 source=next(x[2:] for x in json.loads((Q/'profile.json').read_text())['command'] if x.startswith('-I'))
 for c in bounded_cases:
  if not c['id'].startswith('bounded_'):continue
  family=c['id'].removeprefix('bounded_');receipt=next(r for r in proof['runs'] if r['family']==family)
  for channel in ['stdout','stderr']:
   raw=(bounded/family/(channel+'.bin')).read_bytes();assert hashlib.sha256(raw).hexdigest()==receipt[channel+'_sha256']
   c[channel]=raw.decode('latin1').replace(source,'{source}').replace(str(Q/'build'),'{build}').replace(str(Q),'{par_specialized}')
 cases=[c for c in cases if c['id'] not in ['samples_'+f for f in families]]+bounded_cases
 selected=[]
 for c in cases:
  if a.case and c['id'] not in a.case and not any(c['id'].startswith(('bounded_'+k.removeprefix('samples_'),'boundary_'+k.removeprefix('samples_')+'_')) for k in a.case):continue
  work=BUILD/c['id'];work.mkdir(exist_ok=True)
  for name,data in c['files'].items():
   p=work/name;p.parent.mkdir(exist_ok=True,parents=True);p.write_bytes(bytes.fromhex(data));assert (Q/'build/cases'/c['id']/name).read_bytes()==bytes.fromhex(data)
  selected.append(c)
 (BUILD/'cases.json').write_text(json.dumps(selected)+'\n')
 env={**os.environ,'CARGO_HOME':str(ROOT/'rust/.cargo-home'),'GMP_MPFR_SYS_CACHE':str(ROOT/'rust/target/gmp-cache'),'AC_PAR_CASES':str(BUILD)}
 cmd=['cargo','test','--manifest-path',str(ROOT/'rust/Cargo.toml'),'--locked','--offline',*(['--release'] if a.release else []),'--lib','source_par_corpus_tests::q_corpus','--','--ignored','--exact','--nocapture','--test-threads=1']
 run=subprocess.run(cmd,env=env)
 (BUILD/('receipt-'+('release' if a.release else 'debug')+'.json')).write_text(json.dumps({'reference_sha256':hashlib.sha256(reference.read_bytes()).hexdigest(),'cases':len(selected),'historical_telescope_cases_withheld':7,'bounded_native_proof_sha256':hashlib.sha256((bounded/'receipt.json').read_bytes()).hexdigest(),'returncode':run.returncode},indent=2)+'\n')
 raise SystemExit(run.returncode)
if __name__=='__main__':main()
