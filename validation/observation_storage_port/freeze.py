"""Pre-implementation source/caller freeze; do not regenerate over accepted evidence."""
import hashlib,json,re,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def save(name,obj):
 p=HERE/name
 assert not p.exists(),str(p)+' already frozen'
 p.write_text(json.dumps(obj,indent=2)+'\n')
manifest=json.loads((ROOT/'decisions/evidence/observation-storage/released-fields.json').read_text())
raw=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',manifest['source_revision']+':tempo2.h'])
header=raw.decode()
assert hashlib.sha256(raw).hexdigest()==manifest['source_header_sha256']
body=header.split('typedef struct observation {')[1].split('} observation;')[0]
body=re.sub(r'/\*.*?\*/|//[^\n]*','',body,flags=re.S)
fields=[]
for d in body.split(';'):
 if not d.strip():continue
 m=re.fullmatch(r'\s*(long long|longdouble|double|int|char|clock_correction)\s+(\w+)((?:\[[^]]+\])*)\s*',d)
 assert m,d
 fields.append(dict(name=m[2],source_type=m[1],dimensions=re.findall(r'\[([^]]+)\]',m[3])))
assert fields==manifest['fields'] and len(fields)==95
sizes=manifest['source_constants']|{'MAX_FILELEN+1':501}
base={'longdouble':'Option<Float> (128 significant bits)','double':'Option<f64>','int':'Option<i32>','long long':'Option<i64>','char':'Option<u8>','clock_correction':'SourceClockCorrection'}
for f in fields:
 f.update(rust_name=f['name'],rust_cell=base[f['source_type']],extents=[sizes[x] if x in sizes else int(x) for x in f['dimensions']],native_allocation='unwritten',frontend_write={'sat':'strict parse_longdouble unchanged','freq':'strict parse_binary64 unchanged','toaErr':'strict parse_binary64 unchanged','fname':'UTF-8 bytes through NUL; capacity 501','telID':'UTF-8 bytes through NUL; capacity 100'}.get(f['name'],'unwritten'))
save('fields.json',dict(source_revision=manifest['source_revision'],source_header_sha256=manifest['source_header_sha256'],fields=fields,clock_correction_fields=manifest['clock_correction_fields'],constants=sizes))
protected=json.loads((ROOT/'validation/parameter_storage_port/protected_hashes.json').read_text())
tracked=subprocess.check_output(['git','ls-files'],cwd=ROOT,text=True).splitlines()
for s in tracked:
 p=ROOT/s
 if s.startswith(('validation/preprocessing/','validation/par_specialized/','validation/pulsar_lifecycle/')):continue
 if s.startswith(('validation/','rust/verification/','rust/tests/reference/','decisions/','handoffs/')) or s in ['rust/PARAMETER_STORAGE_REPORT.md','rust/CLOCK_REPORT.md','rust/JPL_REPORT.md','rust/M1A_REPORT.md','rust/M1B_REPORT.md','rust/M2_REPORT.md','rust/Cargo.toml','rust/Cargo.lock','rust/build.rs']:
  if s in ['validation/parameter_storage_port/run.py','validation/parameter_storage_port/native.py','validation/parameter_storage_port/verify.py']:continue
  protected[s]=sha(p)
assert all(not s.startswith(('validation/preprocessing/','validation/par_specialized/','validation/pulsar_lifecycle/')) for s in protected)
assert all(sha(ROOT/s)==h for s,h in protected.items())
save('protected_hashes.json',protected)
save('baseline.json',dict(head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),sources={str(p.relative_to(ROOT)):sha(p) for p in sorted((ROOT/'rust/src').glob('*.rs'))},nested={s:dict(head=subprocess.check_output(['git','-C',s,'rev-parse','HEAD'],cwd=ROOT,text=True).strip(),status=subprocess.check_output(['git','-C',s,'status','--porcelain'],cwd=ROOT,text=True)) for s in ['tempo2','validation/reference']}))
cs=json.loads((ROOT/'validation/numeric_toolkit/observations.json').read_text())
proj=[]
for c in cs:
 if c['id'].startswith('copypsr_'):
  r=c['events'][1]['result']
  assert all([f['field'] for f in obs]==[f['name'] for f in fields] for obs in r['observations'])
  proj.append(dict(id=c['id'],actions=c['actions'],source_unchanged=r['source_observations_unchanged'],observations=[ [{k:v for k,v in f.items() if k!='offset'} for f in obs] for obs in r['observations']]))
save('m_projection.json',proj)
print('frozen',len(fields),'fields;',len(protected),'protected files;',len(proj),'M histories')
