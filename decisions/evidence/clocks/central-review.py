import hashlib,importlib.util,json,shutil,subprocess,sys,tempfile
from collections import Counter
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[3];REV='d70b075'
def sha(b):return hashlib.sha256(b).hexdigest()
def load_module(name,path):
 spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec);sys.modules[name]=m;spec.loader.exec_module(m);return m
files=subprocess.check_output(['git','ls-tree','-r','--name-only',REV,'--','validation/clocks'],cwd=ROOT,text=True).splitlines()
summary={'reviewed_revision':subprocess.check_output(['git','rev-parse',REV],cwd=ROOT,text=True).strip(),'snapshot_hashes':{},'scope':'Independent temporary build and complete executed-case replay; withheld cases are not executed.'}
with tempfile.TemporaryDirectory(prefix='tempo2-clocks-central-',dir='/tmp') as temp:
 here=Path(temp)/'clocks';here.mkdir();sys.path.insert(0,str(here))
 for f in files:
  data=subprocess.check_output(['git','show',REV+':'+f],cwd=ROOT);dst=here/Path(f).relative_to('validation/clocks');dst.parent.mkdir(parents=True,exist_ok=True);dst.write_bytes(data);summary['snapshot_hashes'][f]=sha(data)
 profile=load_module('profile',here/'profile.py');profile.ROOT=ROOT;profile.LIBRARY=ROOT/'validation/reference/build/libtempo2-reference.a'
 run=load_module('run',here/'run.py')
 src,cc,flags,sp=profile.prepare();exe,np=run.compile_probe(src,cc,flags,'central_probe')
 # Keep the executable's semantic name unchanged for the recorded command field.
 standard=profile.BUILD/'probe';exe.rename(standard);exe=standard
 recorded=json.loads((here/'outputs/manifest.json').read_text())
 for key in ['source_revision','sources','compiler','platform','machine','locale','support_archive_sha256','archive_selected_defined_symbols','support_policy']:
  run.compare(recorded['source'][key],sp[key],'source.'+key)
 for key in ['compiler','compiler_sha256','primary','dependencies','numeric_header_sha256','globals_prefix_sha256','globals_source_slices','runtime_hashes','table_value_instrumentation','profile','dynamic_libraries']:
  run.compare(recorded['native'][key],np[key],'native.'+key)
 cases=run.make_cases();run.compare(json.loads((here/'cases.json').read_text()),cases,'complete_cases')
 results=[];unrun=[]
 for c in cases:
  if c['classification'] in ('undefined','bounded'):unrun.append(c['id']);continue
  assert c['classification'] in ('ordinary','exit')
  r=run.execute(c,exe);run.validate_case(c,r);again=run.execute(c,exe);run.validate_case(c,again);run.compare(r,again,c['id']+'.repeat');results.append(r)
 payload={'cases':results};run.compare(json.loads((here/'outputs/reference.json').read_text()),payload)
 isolated=json.loads((here/'outputs/isolated.json').read_text());assert [c['id'] for c in isolated['cases']]==unrun
 run.compare(recorded['resource_manifest'],run.resource_manifest(cases),'resources')
 print('PASS: independent canonical replay',len(results),'cases;',len(unrun),'explicitly unrun',flush=True)
 verify=load_module('verify',here/'verify.py');checks=verify.verify(cases,payload,isolated,src,cc,flags)
 summary.update(status='PASS',case_classes=dict(Counter(c['classification'] for c in cases)),executed_cases=len(results),events=sum(len(r['events']) for r in results),unrun=unrun,artifact_mutations=len(checks['artifact_mutations']),source_mutations=[r['name'] for r in checks['source_mutations']],behavior_contracts=checks['behavior_contracts'],native_profile=np,source_profile=sp,reference_sha256=sha((here/'outputs/reference.json').read_bytes()),checks=checks)
out=Path('/tmp/tempo2-clocks-central-review.json');out.write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps({k:summary[k] for k in ['status','reviewed_revision','case_classes','executed_cases','events','artifact_mutations','source_mutations','reference_sha256']},indent=2))
