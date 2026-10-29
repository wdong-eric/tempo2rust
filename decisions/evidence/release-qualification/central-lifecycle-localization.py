from pathlib import Path
import copy,gzip,hashlib,importlib.util,json,shutil,tempfile
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
STAGE=ROOT/'validation/release/build/compare-v3/wxxxxxxxxxxxxxxx'
NATIVE=STAGE/'validation/pulsar_lifecycle/expected/seeded_reinitialise.json.gz'
RUST=STAGE/'rust/verification/pulsar_lifecycle/release/seeded_reinitialise/events.jsonl'
COMPARATOR=STAGE/'validation/pulsar_lifecycle_port/run.py'
OUT=Path(tempfile.mkdtemp(prefix='tempo2-al-lifecycle-central-'))
Path('/tmp/tempo2-al-lifecycle-central-current').write_text(str(OUT)+'\n')
shutil.copyfile(__file__,OUT/'executed-script.py')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
identity={str(p):sha(p) for p in [NATIVE,RUST,COMPARATOR]}
spec=importlib.util.spec_from_file_location('frozen_lifecycle',COMPARATOR)
module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
with RUST.open() as f:
 launch=json.loads(next(f));actual=[module.project(json.loads(line)) for line in f]
expected=module.project_expected(json.loads(gzip.decompress(NATIVE.read_bytes())))
assert len(actual)==len(expected)
differences=[];localized=copy.deepcopy(actual)
for i,(a,b) in enumerate(zip(actual,expected,strict=True)):
 assert len(a['state'])==len(b['state'])
 for j,(x,y) in enumerate(zip(a['state'],b['state'],strict=True)):
  assert x['path']==y['path']
  if x!=y:
   differences.append({'event':i,'phase':a['phase'],'path':x['path'],'rust':x,'raw_native':y})
   assert x['path'].rsplit('.',1)[-1] in {'nchromx','ne_sw_ifuncN','nWhite_dm','AverageDMResiduals'}
   assert x==dict(y,runs=[[1,'00000000']])
   localized[i]['state'][j]=y
module.compare(localized,expected)
assert all(sha(Path(p))==h for p,h in identity.items())
summary={'scope':'Read-only localization of one retained seeded case, not a new accepted comparison or native correction run','inputs':identity,'events':len(actual),'state_records':sum(len(x['state']) for x in actual),'differences':differences,'unaffected_projection_exact':True,'raw_comparison':'failure preserved','script_sha256':sha(OUT/'executed-script.py')}
(OUT/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(OUT);print(json.dumps({k:v for k,v in summary.items() if k!='inputs'},indent=2))
