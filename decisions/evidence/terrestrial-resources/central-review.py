"""Central replay of U with actual fresh identities and separate generated output."""
from pathlib import Path
import hashlib, importlib, json, shutil, sys, tempfile
sys.dont_write_bytecode=True
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
HERE=ROOT/'validation/terrestrial_resources'
# TKlog truncates after formatting the source path. Match its byte width;
# do not trim diagnostics or weaken the exact output comparator.
base=Path(tempfile.gettempdir())
prefix_length=len(str(HERE))-len(str(base))-1-8
assert prefix_length>=2
OUT=Path(tempfile.mkdtemp(prefix='u-'+'x'*(prefix_length-2)))
assert len(str(OUT).encode())==len(str(HERE).encode())
print('OUT',OUT,flush=True)
sys.path.insert(0,str(HERE))
import inventory, profile, run
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def save(p,x):p.write_text(json.dumps(x,indent=2)+'\n')
accepted=HERE/'outputs'
frozen_paths=[p for p in HERE.glob('*') if p.suffix in ['.py','.cc']]+[HERE/'CONTRACT.md',accepted/'reference.json.gz',accepted/'manifest.json',*sorted((accepted/'frozen').iterdir())]
before={str(p):sha(p) for p in frozen_paths}
assert inventory.preservation_status()=={}
original=json.loads((accepted/'manifest.json').read_text())
macro_bytes=(HERE/'build/ordinary/macros.txt').read_bytes()
assert hashlib.sha256(macro_bytes).hexdigest()==original['native']['macros_sha256']
run.frozen_cases()
for name in ['reference.json.gz','manifest.json']:shutil.copy2(accepted/name,OUT/name)
shutil.copytree(accepted/'frozen',OUT/'frozen')
profile.BUILD=OUT/'build';run.BUILD=profile.BUILD;run.OUTPUT=OUT
exe,fresh=profile.compile_probe()
# Output location is the only path rebind. Keep every other profile value/hash.
def rebind(x,build):
 if isinstance(x,str):return x.replace(str(build),'{build}')
 if isinstance(x,list):return [rebind(v,build) for v in x]
 if isinstance(x,dict):return {rebind(k,build):rebind(v,build) for k,v in x.items()}
 return x
old=profile.stable(original['native']);new=profile.stable(fresh)
new_macros=(exe.parent/'macros.txt').read_bytes()
assert sorted(macro_bytes.splitlines())==sorted(new_macros.splitlines())
macro_evidence=dict(accepted_sha256=hashlib.sha256(macro_bytes).hexdigest(),fresh_sha256=hashlib.sha256(new_macros).hexdigest(),complete_sorted_lines_sha256=hashlib.sha256(b'\n'.join(sorted(macro_bytes.splitlines()))).hexdigest())
old.pop('macros_sha256');new.pop('macros_sha256')
a=rebind(old,HERE/'build');b=rebind(new,profile.BUILD)
assert a==b,[(k,a.get(k),b.get(k)) for k in set(a)|set(b) if a.get(k)!=b.get(k)]
print('Fresh native profile matches after exact build-root rebind and complete macro-line check',flush=True)
profile.actual_identity(exe,fresh)
catalog=run.frozen_cases();baseline=run.read_gzip(OUT/'reference.json.gz')
run.compare([c['id'] for c in catalog],[v['id'] for v in baseline],'membership')
receipts=[];count=0
for case,expected in zip(catalog,baseline):
 actual,receipt=run.execute(case,exe,fresh,OUT/'ordinary_raw');run.validate(case,actual);run.compare(expected,actual,case['id'])
 replay,again=run.execute(case,exe,fresh,OUT/'replay_raw');run.validate(case,replay);run.compare(actual,replay,case['id']+'.fresh')
 receipts.append(dict(first=receipt,replay=again));count+=len(actual['events'])
 if len(receipts)%20==0:print('PASS replay',len(receipts),'cases',flush=True)
run.write_gzip(OUT/'execution_receipts.json.gz',receipts)
import controls
controls.main()
control=json.loads((OUT/'controls.json').read_text());assert control['status']=='PASS'
mutants=run.read_gzip(OUT/'mutations.json.gz')
for m in mutants:
 profile.actual_identity(profile.BUILD/('mutant%02d'%[x[0] for x in controls.MUTATIONS].index(m['name']))/'probe',m['profile'])
 assert controls.first_difference(next(v for v in baseline if v['id']==m['case'])['events'],m['result']['events'])==m['witness']
assert inventory.preservation_status()=={}
assert {str(p):sha(p) for p in frozen_paths}==before
profile.actual_identity(exe,fresh)
result=dict(status='PASS',scope='Independent central U replay and controls; coverage/observer receipts inspected separately',output=str(OUT),cases=len(catalog),events=count,controls=len(control['controls']),compiled_mutations=len(mutants),fresh_profile=fresh,macro_projection=macro_evidence,protected_source_files=before,accepted_manifest_sha256=sha(accepted/'manifest.json'),accepted_reference_sha256=sha(accepted/'reference.json.gz'),control_receipt_sha256=sha(OUT/'controls.json'),execution_receipts_sha256=sha(OUT/'execution_receipts.json.gz'),source_head=__import__('subprocess').check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip())
save(OUT/'central-review.json',result)
print('PASS central U',len(catalog),'cases',count,'events',len(control['controls']),'controls',len(mutants),'compiled mutations',flush=True)
print('RECEIPT',OUT/'central-review.json',sha(OUT/'central-review.json'),flush=True)
