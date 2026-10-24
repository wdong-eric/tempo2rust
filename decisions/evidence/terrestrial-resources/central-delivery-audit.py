"""Inspect U's delivered coverage/observer/history evidence without rerunning it."""
from pathlib import Path
import collections,gzip,hashlib,json,re,sys
sys.dont_write_bytecode=True
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust');HERE=ROOT/'validation/terrestrial_resources';O=HERE/'outputs'
OUT=Path('/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/u-xxxxxxxxxxx7x96fxui')
sys.path.insert(0,str(HERE))
import inventory,profile,run
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
load=lambda p:json.loads(p.read_text())
original={str(p):sha(p) for p in [HERE/'REPORT.md',HERE/'GAPS.md',*HERE.glob('*.py'),HERE/'probe.cc',HERE/'CONTRACT.md',O/'reference.json.gz',O/'manifest.json',*sorted((O/'frozen').iterdir())]}
assert inventory.preservation_status()=={}
catalog=run.frozen_cases();baseline=run.read_gzip(O/'reference.json.gz')
ids=[x['id'] for x in catalog];run.compare(ids,[x['id'] for x in baseline]);by_id={x['id']:x for x in baseline}
assert len(ids)==130 and sum(len(x['events']) for x in baseline)==2256
for n,c in zip(baseline,catalog):run.validate(c,n)
receipts={name:load(O/(name+'.json')) for name in ['last_execution','controls','coverage','observer_audit']}
for name,d in receipts.items():
 assert d['status']==d['native_status']=='PASS',name
 assert d['preservation_before']==d['preservation_after']=={},name
for kind,name in [('ordinary','last_execution'),('coverage','coverage'),('observer','observer_audit')]:
 profile.actual_identity(HERE/'build'/kind/'probe',receipts[name]['profile'])
commands=load(O/'commands.json');assert len(commands)==4 and all(x['returncode']==0 for x in commands)
# Independently reconstruct source-selected gcov accounting from raw gcov files.
cov=receipts['coverage'];inv=load(O/'frozen/source_map.json');raw=[]
for p in (HERE/'build/coverage').glob('*.gcov.json.gz'):raw.extend(run.read_gzip(p)['files'])
lines=[];branches=[];routines=[]
for r in inv['routines']:
 files=[f for f in raw if Path(f['file']).name==r['file']];assert len(files)==1,r['file'];f=files[0]
 fn=next(x for x in f['functions'] if x['demangled_name'].split('(')[0]==r['name'])
 routines.append(dict(file=r['file'],name=r['name'],measured=fn))
 for row in f['lines']:
  if r['start']<=row['line_number']<=r['end']:
   lines.append(dict(file=r['file'],function=r['name'],**row))
   branches.extend(dict(file=r['file'],function=r['name'],line=row['line_number'],index=i,**b) for i,b in enumerate(row.get('branches',[])))
assert routines==cov['routines']
strip=lambda x:{k:v for k,v in x.items() if k not in ['classification','reason']}
assert lines==[strip(x) for x in cov['lines']]
assert branches==[strip(x) for x in cov['branches']]
summary=dict(routines=len(routines),routines_entered=sum(x['measured']['execution_count']>0 for x in routines),lines=len(lines),lines_hit=sum(x['count']>0 for x in lines),branches=len(branches),branches_hit=sum(x['count']>0 for x in branches))
assert summary==cov['summary'];assert summary==dict(routines=22,routines_entered=22,lines=341,lines_hit=336,branches=252,branches_hit=217)
assert cov['ordinary_exact_cases']==ids and cov['ordinary_differences']==[]
assert receipts['observer_audit']['matches']==ids and receipts['observer_audit']['observed_prefix_assertions']==6
caller=(HERE/'build/observer/probe.cc').read_text();src=(HERE/'probe.cc').read_text();assert src.count('atexit(observeExit);')==1
assert caller==src.replace('atexit(observeExit);','(void)0;')
# Check all recorded raw streams and events against their hashes and the oracle.
for kind,receipt_name in [('coverage','coverage_receipts.json.gz'),('observer_audit','observer_audit_receipts.json.gz')]:
 rs=run.read_gzip(O/receipt_name);assert [x['id'] for x in rs]==ids
 exe=HERE/'build'/('coverage' if kind=='coverage' else 'observer')/'probe'
 for r in rs:
  name=r['id'];folder=O/(kind+'_raw');events=gzip.decompress((folder/(name+'.jsonl.gz')).read_bytes());stdout=(folder/(name+'.stdout')).read_bytes();stderr=(folder/(name+'.stderr')).read_bytes()
  assert r['raw_sha256']=={k:hashlib.sha256(v).hexdigest() for k,v in [('events',events),('stdout',stdout),('stderr',stderr)]}
  expected=by_id[name];actual=[json.loads(run.normalize(s,exe)) for s in events.decode().splitlines()]
  want=expected['events'] if kind=='coverage' else [e for e in expected['events'] if e['phase']!='exit']
  run.compare(want,actual,name+'.'+kind+'.events')
  run.compare(expected['stdout'],run.normalize(stdout.decode('latin1'),exe),name+'.'+kind+'.stdout')
  run.compare(expected['stderr'],run.normalize(stderr.decode('latin1'),exe),name+'.'+kind+'.stderr')
# Every non-throw gap must have one reviewed disposition in GAPS.md.
gaps={}
for m in re.finditer(r'\| ((?:observatory|eop|tropo)\.C):(\d+) / ([\d,]+) \| ([a-z-]+) \|',(HERE/'GAPS.md').read_text()):
 for index in map(int,m[3].split(',')):gaps[(m[1],int(m[2]),index)]=m[4]
missing={(x['file'],x['line'],x['index']) for x in branches if not x['count'] and not x.get('throw',False)}
assert missing==set(gaps)
dispositions=collections.Counter(gaps.values());dispositions['host-dependent']+=sum(not x['count'] and x.get('throw',False) for x in branches)
assert dict(dispositions)=={'still-defined-unverified':7,'inactive':2,'host-dependent':23,'native-invalid':3}
# Preserve the original failed evidence as a separate historical bundle.
historical=O/'preservation_drift_before_restore';snapshot=load(historical/'snapshot_sha256.json');assert len(snapshot)==2133
assert all(sha(historical/p)==h for p,h in snapshot.items())
assert sha(historical/'snapshot_sha256.json')=='93d545f4d8dd5c90275dc82618cc33359bf40a55975a9f5641937b4dddf5380b'
for name in receipts:
 d=load(historical/(name+'.json'));assert d['status']=='PRESERVATION_DRIFT' and d['native_status']=='PASS'
 assert set(d['preservation_before'])==set(d['preservation_after'])=={'validation/observation_storage_port/run.py'}
assert {p:sha(Path(p)) for p in original}==original
assert inventory.preservation_status()=={}
result=dict(status='PASS',scope='Delivered coverage/observer/raw/history audit; instrumented and observer-off native runs not repeated by central',checked_original_files=original,coverage=summary,coverage_gap_dispositions=dict(dispositions),coverage_raw_cases=130,observer_raw_cases=130,prior_failure_files=len(snapshot),protected_files=len(load(O/'frozen/protected.json')),receipt_hashes={n:sha(O/(n+'.json')) for n in receipts},report_sha256=sha(HERE/'REPORT.md'),snapshot_sha256=sha(historical/'snapshot_sha256.json'),source_revision='2cd8c91',delivery_revision='051d2ea')
p=OUT/'central-delivery-audit.json';p.write_text(json.dumps(result,indent=2)+'\n');print('PASS delivered source/profile/coverage/observer/history audit',summary,dict(dispositions));print('RECEIPT',p,sha(p))
