"""Normal-verifier negative controls, including actual fresh-process failures."""
import copy,tempfile
from pathlib import Path
from run import validate,validate_identity,identity,execute,check_launch,OUT

def run(raw,native,ident,exe,native_identity):
 results=[]
 def reject(name,fn):
  try:fn()
  except (AssertionError,FileNotFoundError,KeyError,ValueError,TypeError):results.append(dict(name=name,rejected=True));return
  raise AssertionError('control accepted: '+name)
 validate(raw,native);results.append(dict(name='positive',accepted=True))
 def mutate(name,fn):
  value=copy.deepcopy(raw);fn(value);reject(name,lambda:validate(value,native))
 def field(r,name,case=0,side='destination_after'):
  return next(f for f in r['cases'][case][side] if f['name']==name)
 mutate('field_omission',lambda r:r['cases'][0]['destination_after'].pop())
 mutate('field_permutation',lambda r:r['cases'][0]['destination_after'].reverse())
 mutate('type_change',lambda r:field(r,'pulseN').update(type='int'))
 mutate('extent_change',lambda r:field(r,'jump').update(dimensions=[39]))
 mutate('count_change',lambda r:field(r,'obsNjump').update(value=0))
 mutate('integer_sign',lambda r:field(r,'deleted').update(value=-field(r,'deleted')['value']))
 mutate('integer_limit',lambda r:field(r,'pulseN',1).update(value=str(-(1<<63)+1)))
 mutate('numeric_change',lambda r:field(r,'sat')['value'].update(integer='1'))
 mutate('negative_zero_lost',lambda r:field(r,'sat',1)['value'].update(negative=False))
 mutate('class_change',lambda r:field(r,'sat_sec',1).update(value={'class':'+Infinity'}))
 mutate('fabricated_initialization',lambda r:r['unwritten'][0].update(value={'class':'finite','integer':'0','exponent':0,'negative':False}))
 mutate('fabricated_byte',lambda r:next(f for f in r['unwritten'] if f['name']=='fname')['value'].__setitem__(500,0))
 mutate('lost_filename_tail',lambda r:field(r,'fname')['value'].__setitem__(500,0))
 mutate('lost_flag_tail',lambda r:field(r,'flagVal')['value'][39].__setitem__(31,0))
 mutate('lost_jump_tail',lambda r:field(r,'jump')['value'].__setitem__(39,0))
 mutate('lost_correction_tail',lambda r:field(r,'correctionsTT')['value'][29]['corrects_to'].__setitem__(31,0))
 mutate('lost_correction_slot',lambda r:field(r,'correctionsTT')['value'].pop())
 mutate('source_mutated',lambda r:field(r,'freq',side='source_after')['value'].update(integer='8'))
 mutate('ownership_false',lambda r:r['cases'][0].update(independent=False))
 mutate('case_omission',lambda r:r['cases'].pop())
 mutate('case_order',lambda r:r['cases'].reverse())
 mutate('M_retained_tail',lambda r:r['m_cases'][2]['destination_after'][2][0].update(value={'class':'finite','integer':'0','exponent':0,'negative':False}))
 mutate('boundary_order',lambda r:r['boundaries'].reverse())
 mutate('boundary_prefix_lost',lambda r:r['boundaries'][2]['after'].__setitem__(0,None))
 mutate('boundary_renamed',lambda r:r['boundaries'][2].update(error='observation_unwritten_fname[3]'))
 mutate('terminal_false',lambda r:next(x for x in r['boundaries'] if x['case']=='hole').update(terminal=False))
 changed_native=copy.deepcopy(native);changed_native['abi'][0]['cell_bytes']=8
 reject('native_ABI_width',lambda:validate(raw,changed_native))
 changed_native=copy.deepcopy(native);changed_native['signed_types']['int']=False
 reject('native_signedness',lambda:validate(raw,changed_native))
 import native as native_runner
 altered=copy.deepcopy(native_identity);altered['harness_sha256']='0'*64
 reject('native_harness_identity',lambda:native_runner.validate(altered))
 altered=copy.deepcopy(native_identity);altered['compiler_sha256']='0'*64
 reject('native_compiler_identity',lambda:native_runner.validate(altered))
 reject('stale_token',lambda:check_launch(raw,'stale',raw['input_sha256']))
 reject('wrong_input_hash',lambda:check_launch(raw,raw['token'],'wrong'))
 changed=copy.deepcopy(ident);changed['compiler_sha256']='0'*64
 reject('compiler_identity',lambda:validate_identity(changed,identity(exe)))
 changed=copy.deepcopy(ident);changed['sources']['rust/src/observation.rs']='0'*64
 reject('source_identity',lambda:validate_identity(changed,identity(exe)))
 plan=dict(schema=1,cases=[0,1,2],m_cases=[f'copypsr_{i}' for i in range(4)])
 with tempfile.TemporaryDirectory(prefix='R-controls-',dir='/private/tmp') as td:
  native_copy=Path(td)/'native';native_copy.write_bytes(Path(native_identity['executable']).read_bytes()+b'changed')
  altered=copy.deepcopy(native_identity);altered['executable']=str(native_copy)
  reject('changed_native_executable',lambda:native_runner.validate(altered))
  fake=Path(td)/'probe';fake.write_bytes(exe.read_bytes()+b'changed');fake.chmod(0o755)
  reject('changed_executable_before_launch',lambda:execute(fake,ident,plan))
  for name,script in [('nonzero_exit','#!/bin/sh\nexit 7\n'),('missing_output',"#!/bin/sh\nprintf 'test observation_storage_tests::probe ... ok\\n1 passed; 0 failed\\n'\n")]:
   fake.write_text(script);fake.chmod(0o755);fake_id=identity(fake)
   reject(name,lambda:execute(fake,fake_id,plan))
 return results
