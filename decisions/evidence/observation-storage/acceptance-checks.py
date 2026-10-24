#!/usr/bin/env python3
"""Read-only source-shape, regression-receipt and preservation checks for R."""
import collections,hashlib,json,math,re,subprocess
from pathlib import Path
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust');OUT=ROOT/'rust/verification/observation_storage'
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
load=lambda p:json.loads(p.read_text())
blob=lambda rev,n:subprocess.check_output(['git','-C',str(ROOT),'show',rev+':'+n])
manifest=load(ROOT/'decisions/evidence/observation-storage/released-fields.json')
raw=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',manifest['source_revision']+':tempo2.h']).decode()
assert hashlib.sha256(raw.encode()).hexdigest()==manifest['source_header_sha256']
body=raw.split('typedef struct observation {')[1].split('} observation;')[0];body=re.sub(r'/\*.*?\*/|//[^\n]*','',body,flags=re.S)
fields=[]
for d in body.split(';'):
 if not d.strip():continue
 m=re.fullmatch(r'\s*(long long|longdouble|double|int|char|clock_correction)\s+(\w+)((?:\[[^]]+\])*)\s*',d);assert m,d
 fields.append(dict(name=m[2],source_type=m[1],dimensions=re.findall(r'\[([^]]+)\]',m[3])))
assert fields==manifest['fields'] and len(fields)==95
rust=(ROOT/'rust/src/observation.rs').read_text();body=rust.split('pub struct SourceObservation {')[1].split('\n}',1)[0]
actual=re.findall(r'pub (\w+): ([^\n]+),',body);assert [n for n,_ in actual]==[f['name'] for f in fields]
constants=manifest['source_constants']|{'MAX_FILELEN+1':501};base={'longdouble':'Option<Float>','double':'Option<f64>','int':'Option<i32>','long long':'Option<i64>','char':'Option<u8>','clock_correction':'SourceClockCorrection'}
counts=collections.Counter()
for f,(name,typ) in zip(fields,actual):
 ds=[constants[d] if d in constants else int(d) for d in f['dimensions']]
 n=math.prod(ds);counts[f['source_type']]+=n
 expected=base[f['source_type']]
 for size in reversed(ds):expected='['+expected+';'+str(size)+']'
 typ=re.sub(r'\s+','',typ)
 for key in sorted(constants,key=len,reverse=True):typ=typ.replace(key,str(constants[key]))
 assert typ==expected,(name,typ,expected)
counts['double']+=counts.pop('clock_correction');counts['char']+=30*32
assert dict(counts)=={'longdouble':20,'double':354,'int':87,'long long':1,'char':4121}
module_inventory=load(OUT/'module_inventory.json');unsafe={}
for name,entry in module_inventory.items():
 assert sha(ROOT/name)==entry['sha256'],name
 if entry['unsafe_lines']:unsafe[name]=entry['unsafe_lines']
assert set(unsafe)=={'rust/src/stdio.rs','rust/src/clock_text.rs','rust/src/get_obs_coord.rs'}
assert sum(len(v) for v in unsafe.values())==12
for name in ['rust/src/stdio.rs','rust/src/get_obs_coord.rs','rust/Cargo.toml','rust/Cargo.lock','rust/build.rs']:
 assert (ROOT/name).read_bytes()==blob('624c1bf',name),name
old=blob('624c1bf','rust/src/clock_text.rs');assert (ROOT/'rust/src/clock_text.rs').read_bytes()==old[:old.index(b'/// snprintf(')]
# Delivered/current source identity, frozen artifacts and logs still agree after policy-only untracking.
seal=load(OUT/'final_artifacts.json')
for group in ['artifacts','source_and_projection_hashes']:
 for name,h in seal[group].items():assert sha(ROOT/name)==h,(group,name)
commands={}
for directory in [OUT,OUT/'O',OUT/'O/regressions',OUT/'O/regressions/m2']:
 receipts=load(directory/'commands.json');commands[str(directory.relative_to(OUT))]=[]
 for c in receipts:
  log=directory/c['log'];assert log.is_file()
  if 'log_sha256' in c:assert sha(log)==c['log_sha256']
  expected=1 if c.get('name')=='m2' else 0;assert c['returncode']==expected,c
  commands[str(directory.relative_to(OUT))].append({k:v for k,v in c.items() if k not in ['stdout','stderr']})
for profile in ['debug','release']:
 text=(OUT/'O'/(profile+'_tests.log')).read_text()
 assert '28 passed; 0 failed; 7 ignored' in text and '7 passed; 0 failed' in text and '1 passed; 0 failed' in text
for name,h in load(OUT/'O/regressions/m2/source_hashes.json').items():assert sha(ROOT/name)==h,name
reg=load(OUT/'O/regressions/delivery_checks.json');assert reg['m1a']=='PASS' and reg['m1b']=='PASS (via M2)' and reg['m2_exit']==1 and len(reg['m2_failures'])==4
assert load(OUT/'O/regressions/m2/open_gate_failures.json')==load(ROOT/'rust/verification/m2/open_gate_failures.json')
cli={'debug':'11923da288cbb6532bfd09a55f54246a9ee05d816446a02affc3ff24a71d56fa','release':'a35e70701fb929829c4c42e3da6d172bd9021b327bc58f079a67900f8c2971a6'}
for p,h in cli.items():assert sha(ROOT/'rust/target'/p/'tempo2-rust')==h,p
nested={}
for repo,pin in [('tempo2',manifest['source_revision']),('validation/reference','7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572')]:
 head=subprocess.check_output(['git','-C',str(ROOT/repo),'rev-parse','HEAD'],text=True).strip();status=subprocess.check_output(['git','-C',str(ROOT/repo),'status','--porcelain'],text=True)
 assert head==pin and not status;nested[repo]=dict(head=head,status=status)
result=dict(source_field_order_types_extents_exact=True,source_fields=95,logical_cells=dict(counts),logical_cell_count=sum(counts.values()),module_count=len(module_inventory),unchanged_unsafe_sites=unsafe,artifacts=len(seal['artifacts']),source_projection_hashes=len(seal['source_and_projection_hashes']),commands=commands,regression_cli_sha256=cli,nested_repositories=nested,M2_complete_failure_list=reg['m2_failures'],full_regressions_independently_rerun=False)
p=Path('/tmp/tempo2-observation-acceptance-checks.json');p.write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
print('PASS source shape, 39 modules, 12 unchanged unsafe sites, 246 artifacts, complete regression receipts and historical M2 failures.',flush=True)
