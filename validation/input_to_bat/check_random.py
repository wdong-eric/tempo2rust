#!/usr/bin/env python3
"""Reuse all Z random histories, including complete observed private state.

The accepted observer is read-only and its ordinary projection was audited by Z.
Compare exact f64 results/seeds/cached values and every 100-cell binary128 table
projection; a precision mismatch is a failure, not a new tolerance.
"""
import argparse,gzip,hashlib,json,os,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parent.parent;BUILD=HERE/'build'/'random'
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--release',action='store_true');args=ap.parse_args()
 BUILD.mkdir(exist_ok=True,parents=True)
 source=ROOT/'validation/planetary_ephemeris/outputs/observed.json.gz'
 reference=json.loads(gzip.decompress(source.read_bytes()));selected=[]
 for case in reference:
  if not any(a[0] in ['random','gaussian'] for a in case['actions']):continue
  events=[]
  for event in case['events']:
   if event['phase']=='end' and event['operation'] in ['seed','random','gaussian']:
    events.append({'action':case['actions'][event['event']],'result':event['result'],'seeds':event['seeds'],'private':event['private']})
  selected.append({'id':case['id'],'events':events})
 path=BUILD/'cases.json';path.write_text(json.dumps(selected)+'\n')
 env={**os.environ,'CARGO_HOME':str(ROOT/'rust/.cargo-home'),'GMP_MPFR_SYS_CACHE':str(ROOT/'rust/target/gmp-cache'),'AC_RANDOM_CASES':str(path)}
 cmd=['cargo','test','--manifest-path',str(ROOT/'rust/Cargo.toml'),'--locked','--offline',*(['--release'] if args.release else []),'--lib','read_ephemeris::tests::z_random_histories','--','--ignored','--exact','--test-threads=1']
 run=subprocess.run(cmd,env=env)
 profile='release' if args.release else 'debug'
 (BUILD/('receipt-'+profile+'.json')).write_text(json.dumps({'reference_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'cases':len(selected),'profile':profile,'returncode':run.returncode},indent=2)+'\n')
 raise SystemExit(run.returncode)
if __name__=='__main__':main()
