"""Complete ordinary projection audit of initialized-state observer instrumentation."""
import copy
from inventory import HERE,sha,save,check
from profile import actual_identity
from observer_profile import build
from combined import load
from run import OUTPUT,execute,read_gzip,write_gzip,compare,frozen_cases

def project(result):
 r=copy.deepcopy(result);r['events']=[e for e in r['events'] if e['phase']!='exit']
 for e in r['events']:
  if 'private' in e:e['private']=None
 return r

def main():
 check();catalog,ordinary=load();refhash=sha((OUTPUT/'reference.json.gz').read_bytes())
 compare([c['id'] for c in catalog],[r['id'] for r in ordinary]);exe,p=build();receipts=[];observed=[]
 for c,expected in zip(catalog,ordinary):
  actual,receipt=execute(c,exe,p,OUTPUT/'observer_raw');again,replay=execute(c,exe,p,OUTPUT/'observer_replay_raw')
  compare(actual,again,c['id']+'.observer_replay');compare(expected,project(actual),c['id']+'.ordinary_projection')
  if c['status']==1:assert actual['events'][-1]['phase']=='exit'
  receipts.append(dict(first=receipt,replay=replay));observed.append(actual);print('PASS observer',c['id'],flush=True)
 by={r['id']:r for r in observed}
 private=lambda name:[e['private'] for e in by[name]['events'] if e.get('private')]
 loops=[e for x in private('iteration_limit') for e in x['events'] if e['kind']=='iteration'];assert len(loops)==100
 assert loops[0]['loop']==0 and loops[-1]['loop']==99
 assert loops[0]['old']=='0000000000000000'
 call_order=[(e['kind'],e.get('routine')) for x in private('iteration_limit') for e in x['events']]
 assert call_order==[('TT',None),('call','shapiro_delay'),('call','dm_delays'),('iteration',None)]*100
 for name,count in [('bats_-1_1',2),('bats_retained_bypass',0)]:
  for e in by[name]['events']:
   if e['phase']=='end' and e['op']=='bats':assert len(e['private']['events'])==count

 fast=[e for x in private('iteration_fast_1_deleted_0') for e in x['events'] if e['kind']=='iteration'];assert len(fast)==1
 mask=[e['private']['mask'] for e in by['dmmodel_1']['events'] if e['phase']=='end' and e['op']=='dm'];assert mask==[65280]*3
 for r in observed:
  for e in r['events']:
   if e.get('private'):
    assert list(e['private'])==['mask','events','mcl_outputs','diagnostic_buffers']
    assert all(x['kind'] in ['iteration','TT','sheet','call'] for x in e['private']['events'])
 import struct
 for name in ['supp_mcl_conjunction_exit','supp_mcl_year_exit']:
  aux=by[name]['events'][-1]['private']['mcl_outputs'];assert aux is not None
  assert struct.unpack('<i',bytes.fromhex(aux['count']['runs'][0][1]))[0]==-1
  assert all(aux[k]['count']==36 for k in ['helat','crlon','rotations']) and aux['scalars']['count']==7
 errors=[e['private']['diagnostic_buffers']['errors'] for e in by['supp_error_storage_full']['events'] if e['phase']=='end' and e['op']=='dm']
 for record in errors[10:]:compare(errors[10],record,'error buffer retained after capacity')
 warnings=[e['private']['diagnostic_buffers']['warnings'] for e in by['supp_warning_storage_full']['events'] if e['phase']=='end' and e['op']=='bclt']
 compare(warnings[29],warnings[30],'warning buffer retained after capacity')
 year=by['supp_mcl_year_exit']['events'][-1]['private']['mcl_outputs']
 for key,value in [('helat',81.),('crlon',82.),('rotations',83.),('scalars',84.)]:
  assert year[key]['runs']==[[year[key]['count'],struct.pack('<d',value).hex()]]
 check();assert sha((OUTPUT/'reference.json.gz').read_bytes())==refhash
 write_gzip(OUTPUT/'observer_snapshots.json.gz',observed);write_gzip(OUTPUT/'observer_receipts.json.gz',receipts)
 save(OUTPUT/'observer_audit.json',dict(status='PASS',script_sha256=sha((HERE/'observer_audit.py').read_bytes()),profile=p,actual_identity=actual_identity(exe,p),matches=len(observed),fresh_replays=len(observed),assertions=['100 iterations','veryFast first pass','static warning mask','defined exit snapshot','complete ordinary projection','initial zero and dependency call order','two real TT sums per BAT','initialized mcl2 loop-limit and year-exit prefixes','full static diagnostic buffers and saturation retention'],snapshot_sha256=sha((OUTPUT/'observer_snapshots.json.gz').read_bytes()),ordinary_sha256=refhash))
if __name__=='__main__':main()
