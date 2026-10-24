"""Fresh central T build/replay; all generated outputs use a disposable directory."""
from pathlib import Path
import hashlib,importlib,json,os,shutil,subprocess,sys,tempfile
sys.dont_write_bytecode=True
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust');HERE=ROOT/'validation/pulsar_lifecycle_port'
OUT=Path(tempfile.mkdtemp(prefix='tempo2-t-central-'));print('OUT',OUT,flush=True)
sys.path.insert(0,str(HERE));import run
accepted=run.OUT
before_identity=run.identity();before_protected=run.protected()
freeze={p.name:run.sha(p) for p in (accepted/'freeze').iterdir() if p.is_file()}
shutil.copytree(accepted/'freeze',OUT/'freeze')
run.OUT=OUT
run.ENV=dict(run.ENV,CARGO_TARGET_DIR=str(OUT/'cargo-target'),CARGO_BUILD_JOBS='2')
import controls,supplement
cases=json.loads((OUT/'freeze/membership.json').read_text())['S']
assert len(cases)==30
checks=[];profiles=[]
def command(name,args,env):
 p=subprocess.run(list(map(str,args)),cwd=ROOT/'rust',env=env,capture_output=True,text=True)
 log=OUT/(name+'.log');log.write_text(p.stdout+p.stderr)
 checks.append(dict(name=name,args=list(map(str,args)),returncode=p.returncode,log=str(log),log_sha256=run.sha(log)))
 assert p.returncode==0,(name,p.stdout[-2000:],p.stderr[-2000:])
 print('PASS',name,flush=True)
for mode in ['debug','release']:
 print('BUILD',mode,flush=True);exe=run.build(mode);ident=run.identity(exe)
 assert exe.is_relative_to(OUT/'cargo-target')
 rows=[]
 for case in cases:
  rows.append(run.execute(case,exe,ident,OUT/mode/case['id']))
  print('PASS',mode,case['id'],flush=True)
 negative=controls.run(mode,exe,ident)
 assert len(negative)==31 and sum(x['events'] for x in rows)==133 and sum(x['records'] for x in rows)==5329542
 sys.argv=['supplement.py','--profile',mode];supplement.main()
 command(mode+'_focused_tests',[exe,'pulsar_lifecycle_tests::','--test-threads=1'],run.ENV)
 command(mode+'_isolated_diagnostics',[exe,'pulsar_lifecycle_tests::lifecycle_logging_after_clock_fault','--exact','--ignored','--test-threads=1'],run.ENV)
 assert run.identity(exe)==ident and run.protected()==before_protected
 data=dict(profile=mode,identity=ident,cases=rows,controls=negative,helper_receipt_sha256=run.sha(OUT/mode/'helpers/receipt.json'))
 run.save(OUT/mode/'central-profile.json',data);profiles.append(data)
assert run.identity()==before_identity
assert {p.name:run.sha(p) for p in (accepted/'freeze').iterdir() if p.is_file()}==freeze
assert run.protected()==before_protected
result=dict(status='PASS',scope='Fresh debug/release T builds, all S cases and controls, Q/helpers/native mutations and focused lifecycle tests; delivered wider regressions audited separately',output=str(OUT),profiles=profiles,commands=checks,protected_files=before_protected,freeze_sha256=freeze,source_identity=before_identity,build_environment_overrides={k:run.ENV[k] for k in ['CARGO_TARGET_DIR','CARGO_BUILD_JOBS','CARGO_HOME','GMP_MPFR_SYS_CACHE','LC_ALL','RUST_TEST_THREADS']})
run.save(OUT/'central-review.json',result)
print('PASS central T: 30 cases / 133 events / 5329542 records and 31 controls per profile; helpers and focused tests pass',flush=True)
print('RECEIPT',OUT/'central-review.json',run.sha(OUT/'central-review.json'),flush=True)
