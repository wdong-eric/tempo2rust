"""Reuse affected AE comparators with fresh AG destinations, preserving AE evidence."""
from pathlib import Path
import importlib,argparse,sys,json,hashlib,subprocess
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1]
sys.path.insert(0,str(ROOT/'validation/general_fitting'))
p=argparse.ArgumentParser();p.add_argument('--profile',required=True);p.add_argument('--output',required=True);p.add_argument('--suites',nargs='*',default=['automatic','solver_contract','legacy','svd_api']);a=p.parse_args();out=Path(a.output).resolve();out.mkdir(parents=True,exist_ok=False)
# The released legacy linker misses three observer-only stubs added in AE's
# later native objects. Supply them locally without editing its runner/evidence.
stubs=out/'legacy_stubs.cc';stubs.write_text("void ae_covariance(int,double**){}\nvoid ae_solver_multiplier(double){}\nvoid ae_robust_step(int,int,double,double,double,double,double*,double**,double*,double*,double*,double**,int,int){}\n")
original_run=subprocess.run
def with_stubs(command,*args,**kwargs):
 if isinstance(command,list) and any(str(v).endswith('/native_legacy.cc') for v in command):command.insert(command.index('-o'),str(stubs))
 return original_run(command,*args,**kwargs)
subprocess.run=with_stubs
for name in a.suites:
 m=importlib.import_module('check_'+name);m.BUILD=out/name;m.run(a.profile)
paths=list(out.rglob('*.json'));failures=[]
for path in paths:
 r=json.loads(path.read_text())
 if isinstance(r,dict):
  failures.extend(r.get('failures',[]))
  for record in r.get('records',[]):failures.extend(record.get('failures',[]))
(out/'index.json').write_text(json.dumps({'profile':a.profile,'failures':failures,'receipts':{str(p.relative_to(out)):hashlib.sha256(p.read_bytes()).hexdigest()for p in paths}},indent=2)+'\n');sys.exit(bool(failures))
