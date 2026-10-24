"""Central verification of corrected S, preserving every delivered source/evidence file."""
import copy,gzip,hashlib,json,shutil,subprocess,sys,tempfile
from pathlib import Path
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
ORIGINAL=ROOT/'validation/pulsar_lifecycle'
OUT=Path(tempfile.mkdtemp(prefix='tempo2-pulsar-lifecycle-central-',dir='/private/tmp'))
CALLER=OUT/'caller';CALLER.mkdir()
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
files=[p for p in ORIGINAL.rglob('*') if p.is_file() and 'build' not in p.parts and '__pycache__' not in p.parts]
before={str(p.relative_to(ROOT)):digest(p) for p in files}
for source in files:
 dest=CALLER/source.relative_to(ORIGINAL);dest.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(source,dest)
sys.path.insert(0,str(CALLER))
import profile as pm
pm.ROOT=ROOT
import run,verify,inventory,correction,evidence
inventory.ROOT=ROOT
run.protected()
frozen=json.loads((ORIGINAL/'profile.json').read_text());expected=copy.deepcopy(frozen)
run.compare(frozen['stable']['runner_sources'],pm.runners(),'copied runner identity')
for n,h in pm.runners().items():
 if n.endswith(('.py','.cc')):
  data=subprocess.check_output(['git','-C',str(ROOT),'show','6cc0fd0:validation/pulsar_lifecycle/'+n])
  assert hashlib.sha256(data).hexdigest()==h,n
run.compare(json.loads((CALLER/'inventory.json').read_text()),inventory.build(),'source-derived inventory')
rebound=[]
for name,h in list(expected['stable']['header_dependencies'].items()):
 if name.startswith(str(ORIGINAL)+'/'):
  target=CALLER/Path(name).relative_to(ORIGINAL);assert digest(target)==h
  expected['stable']['header_dependencies'][str(target)]=expected['stable']['header_dependencies'].pop(name)
  rebound.append(dict(original=name,copy=str(target),sha256=h))
exe,p=pm.compile_probe()
macro_order_only=False
if frozen['stable']['macros_sha256']!=p['stable']['macros_sha256']:
 a=(ORIGINAL/'build/probe-macros.txt').read_bytes();b=(pm.BUILD/'probe-macros.txt').read_bytes()
 assert pm.sha(a)==frozen['stable']['macros_sha256'] and pm.sha(b)==p['stable']['macros_sha256']
 run.compare(sorted(a.splitlines()),sorted(b.splitlines()),'complete macro definitions')
 expected['stable']['macros_sha256']=p['stable']['macros_sha256'];macro_order_only=True
run.identity_gate(expected,p,exe)
pm.save(CALLER/'profile.json',expected)
print('PASS exact copied caller sources and fresh stable profile',flush=True)
verify.main()
regeneration=json.loads((CALLER/'regeneration.json').read_text())
verification=json.loads((CALLER/'verification.json').read_text())
# Confirm every delivered mutation has a native state witness after the profile event.
mutants=[]
for m in verification['mutants']:
 result=json.loads(gzip.decompress((CALLER/'mutations'/(m['name']+'.json.gz')).read_bytes()))
 ordinary=run.reference(m['case'])
 a=[e['state'] for e in ordinary['events'][1:]];b=[e['state'] for e in result['events'][1:]]
 difference=verify.rejected(lambda:run.compare(a,b,'post_profile_native_state'))
 mutants.append(dict(name=m['name'],case=m['case'],difference=difference,executable_sha256=m['profile']['executable_sha256']))
# Baseline is read in place; only the fresh audit result is written to the copy.
correction.BUILD=ORIGINAL/'build'
correction.main()
evidence.main()
for n in ['correction.json','raw_validation.json']:
 run.compare(json.loads((ORIGINAL/n).read_text()),json.loads((CALLER/n).read_text()),'recomputed.'+n)
for original in (ORIGINAL/'deltas').glob('*.json.gz'):
 assert digest(original)==digest(CALLER/'deltas'/original.name),original.name
run.protected();run.compare(before,{str(p.relative_to(ROOT)):digest(p) for p in files},'original files preserved')
receipt=dict(status='PASS',output=str(OUT),source_commit='6cc0fd0',delivery='58ddc07',cases=regeneration['cases'],events=regeneration['events'],field_records=regeneration['field_records'],profile=regeneration['profile'],execution=regeneration['execution'],controls=verification['controls'],mutants=mutants,retained_link_records_checked=verification['retained_link_records_checked'],raw_validated_records=verification['raw_validated_records'],correction=json.loads((CALLER/'correction.json').read_text()),raw_validation=json.loads((CALLER/'raw_validation.json').read_text()),preserved_files=len(before),input_hashes=before,transport=dict(header_path_rebindings=rebound,macro_dump_order_only=macro_order_only,source='Exact copied bytes; ROOT bound to actual repository before imports. No native function, logical state or comparator changed.'))
pm.save(OUT/'central-review.json',receipt)
Path('/tmp/tempo2-pulsar-lifecycle-central-output').write_text(str(OUT)+'\n')
print('PASS',receipt['cases'],'cases',receipt['events'],'events',receipt['field_records'],'field records',len(receipt['controls']),'controls',len(mutants),'post-profile mutation witnesses',flush=True)
print('OUTPUT',OUT,flush=True)
