"""Central fresh Q replay and verifier in an exact-source disposable caller copy."""
import copy,gzip,hashlib,json,shutil,subprocess,sys,tempfile
from pathlib import Path
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
ORIGINAL=ROOT/'validation/par_specialized'
OUT=Path(tempfile.mkdtemp(prefix='tempo2-par-specialized-central-',dir='/private/tmp'))
CALLER=OUT/'caller';CALLER.mkdir()
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
files=[p for p in ORIGINAL.rglob('*') if p.is_file() and 'build' not in p.parts and '__pycache__' not in p.parts]
before={str(p.relative_to(ROOT)):digest(p) for p in files}
for p in [*ORIGINAL.glob('*.py'),ORIGINAL/'probe.cc',*[ORIGINAL/n for n in ['profile.json','protected.json','cases.json','inventory.json','observations.json.gz','initial_state.json.gz']]]:
 shutil.copyfile(p,CALLER/p.name)
sys.path.insert(0,str(CALLER))
import profile as profile_module
profile_module.ROOT=ROOT
import run,verify,inventory,protected
# Raw native warning buffers include absolute PAR paths. Preserve the declared
# original input/runtime locations; original source and evidence stay untouched.
run.BUILD=ORIGINAL/'build'
verify.BUILD=run.BUILD
from fixtures import cases
frozen=json.loads((ORIGINAL/'profile.json').read_text())
expected=copy.deepcopy(frozen)
deps=expected['stable']['header_dependencies']
assert str(ORIGINAL/'probe.cc') in deps
assert deps[str(ORIGINAL/'probe.cc')]==digest(CALLER/'probe.cc')
deps[str(CALLER/'probe.cc')]=deps.pop(str(ORIGINAL/'probe.cc'))
profile_module.save(CALLER/'profile.json',expected)
run.compare(frozen['stable']['runner_sources'],profile_module.runners(),'exact copied runner sources')
for n,h in profile_module.runners().items():
 data=subprocess.check_output(['git','-C',str(ROOT),'show','b61896f:validation/par_specialized/'+n])
 assert hashlib.sha256(data).hexdigest()==h,n
run.compare(json.loads((CALLER/'protected.json').read_text()),protected.snapshot(),'protected_before')
cs=cases();run.compare(json.loads((ORIGINAL/'cases.json').read_text()),cs,'case definitions')
run.compare(json.loads((ORIGINAL/'inventory.json').read_text()),inventory.inventory(),'source inventory')
exe,p=profile_module.compile_probe('central_probe');p['stable']['runtime_fixtures']=run.runtime()
run.check_identity(expected,p,exe)
run.INITIAL=json.loads(gzip.decompress((ORIGINAL/'initial_state.json.gz').read_bytes()))
gold=run.read_observations();assert [c['id'] for c in cs]==[r['id'] for r in gold]
rows=[]
for i,(c,g) in enumerate(zip(cs,gold)):
 run.execution_identity(exe,p,True)
 actual=run.execute(c,exe,p);run.validate(c,actual);run.compare(g,actual,c['id'])
 run.execution_identity(exe,p,True)
 rows.append(dict(id=c['id'],events=len(actual['events']),status=actual['returncode'],projection_sha256=profile_module.sha(json.dumps(actual,sort_keys=True,separators=(',',':')).encode())))
 if i%10==0 or i==len(cs)-1:print('REPLAY',i+1,len(cs),c['id'],'exact',flush=True)
profile_module.save(OUT/'replay.json',dict(profile=p,cases=rows,events=sum(x['events'] for x in rows),execution=run.execution_identity(exe,p,True)))
# The exact copied normal runner needs the same root binding in its child process.
# No source file is edited. All scientific/source/comparison functions are unchanged.
real_run=subprocess.run
bootstrap='import sys;sys.path.insert(0,'+repr(str(CALLER))+');import profile;from pathlib import Path;profile.ROOT=Path('+repr(str(ROOT))+');import run;run.BUILD=Path('+repr(str(ORIGINAL/'build'))+');run.main()'
def child_transport(command,*args,**kwargs):
 if isinstance(command,list) and len(command)>=2 and command[:2]==[sys.executable,str(CALLER/'run.py')]:
  command=[sys.executable,'-c',bootstrap,*command[2:]]
 return real_run(command,*args,**kwargs)
subprocess.run=child_transport
try:verify.main()
finally:subprocess.run=real_run
verification=json.loads((CALLER/'verification.json').read_text())
# Recheck every compiled witness without its initial identity and event metadata.
mutants=[];gold_by_id={r['id']:r for r in gold}
for m in verification['compiled_mutations']:
 artifact=CALLER/m['artifact'];assert digest(artifact)==m['sha256']
 x=json.loads(gzip.decompress(artifact.read_bytes()))
 run.execution_identity(Path(x['profile']['executable_path']),x['profile'],True)
 ordinary=gold_by_id[m['case']]
 observable=lambda r:dict(events=r['events'][1:],returncode=r['returncode'])
 difference=verify.rejected(lambda:run.compare(observable(ordinary),observable(x['result']),'post_initial_native'))
 mutants.append(dict(name=m['name'],case=m['case'],post_initial_difference=difference,executable_sha256=x['profile']['executable_sha256']))
# Independently rerun the delivered semantic and raw-byte reconstruction checks,
# redirecting only the generated summary so original evidence remains untouched.
import analyze
analyze.HERE=ORIGINAL
analyze.save=lambda path,value:profile_module.save(OUT/path.name,value)
analyze.main()
run.compare(json.loads((CALLER/'protected.json').read_text()),protected.snapshot(),'protected_after')
run.compare(before,{str(p.relative_to(ROOT)):digest(p) for p in files},'original_Q_files_preserved')
receipt=dict(status='PASS',output=str(OUT),source_commit='b61896f',profiles={'replay':p,'verifier':verification['profile']},cases=len(cs),events=sum(x['events'] for x in rows),identity_controls=verification['identity_controls'],comparison_controls=verification['comparison_controls'],compiled_mutations=mutants,analysis=json.loads((OUT/'analysis.json').read_text()),preserved_files=len(before),input_hashes=before,transport='Exact runner bytes copied; only original probe.cc dependency path rebound to the identical disposable copy; ROOT bound to original repository before imports in parent and normal-runner child; native case/runtime paths retained at original build location for byte-exact raw warning buffers.')
profile_module.save(OUT/'central-review.json',receipt)
Path('/tmp/tempo2-par-specialized-central-output').write_text(str(OUT)+'\n')
print('PASS',len(cs),'cases',receipt['events'],'events',len(verification['identity_controls'])+len(verification['comparison_controls']),'controls',len(mutants),'post-initial compiled witnesses',flush=True)
print('OUTPUT',OUT,flush=True)
