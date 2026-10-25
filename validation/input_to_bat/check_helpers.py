#!/usr/bin/env python3
"""Compare helper outputs with accepted AB bytes/classes, without rerunning AB."""
import gzip,hashlib,json,os,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parent.parent;BUILD=HERE/'build'/'helpers';BUILD.mkdir(parents=True,exist_ok=True)
path=ROOT/'validation/propagation/outputs/reference.json.gz';reference=json.loads(gzip.decompress(path.read_bytes()))
corpus=json.loads((ROOT/'validation/propagation/outputs/frozen/cases.json').read_bytes())
ops={'ifunc','basis','sinc','factorial','hermite','recurrence','shapelet','angle','amod','ecliptic','elsun','date','rotation','mcl','sheet'}
selected=[]
for case,result in zip(corpus,reference):
 assert case['id']==result['id']
 if case['status']!=0 or not any(a[0]in ops for a in case['actions']):continue
 work=BUILD/case['id'];work.mkdir(exist_ok=True)
 for name,value in case['files'].items():
  f=work/name;f.parent.mkdir(exist_ok=True,parents=True);f.write_text(value)
 actions=[]
 for event in result['events']:
  if event['phase']!='end' or event['op']not in ops:continue
  args=case['actions'][event['event']];actions.append({'args':[a.replace('{case}',str(work)) for a in args],'expected':event['result']})
 selected.append({'id':case['id'],'actions':actions})
path_out=BUILD/'cases.json';path_out.write_text(json.dumps(selected,indent=2)+'\n')
print(len(selected),'helper cases; exact native finite bits / nonfinite classes',flush=True)
env={**os.environ,'CARGO_HOME':str(ROOT/'rust/.cargo-home'),'GMP_MPFR_SYS_CACHE':str(ROOT/'rust/target/gmp-cache'),'AC_HELPER_CASES':str(path_out)}
args=['cargo','test','--manifest-path',str(ROOT/'rust/Cargo.toml'),'--locked','--offline','--lib','propagation_helper_tests::ab_direct_helpers','--','--ignored','--exact','--test-threads=1']
run=subprocess.run(args,env=env)
(BUILD/'receipt.json').write_text(json.dumps({'reference_sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'cases':len(selected),'returncode':run.returncode},indent=2)+'\n')
raise SystemExit(run.returncode)
