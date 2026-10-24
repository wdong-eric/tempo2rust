"""Challenge real U records and live W execution without mutating an oracle."""
import copy,gzip,json
from pathlib import Path
import run

def checks(profile,exe,ident,catalog):
 expected=json.loads(gzip.decompress((run.OUT/'corrected_native/reference.json.gz').read_bytes()))
 rows=[]
 def rejected(name,f):
  try:f()
  except (AssertionError,KeyError,ValueError) as e:rows.append(dict(name=name,rejected=True,witness=str(e)[:300]))
  else:raise AssertionError('control accepted: '+name)
 def corrupt(name,case,edit):
  a=next(c for c in expected if c['id']==case);b=copy.deepcopy(a);edit(b);rejected(name,lambda:run.compare(run.project(a),run.project(b),case))
 corrupt('missing_action','eop_first_use_cache',lambda r:r['events'].pop(7))
 corrupt('permuted_actions','eop_first_use_cache',lambda r:r['events'].reverse())
 corrupt('missing_field','observatory_lazy_aliases',lambda r:r['events'][7]['state'].pop('aliases'))
 corrupt('permuted_fields','observatory_lazy_aliases',lambda r:r['events'][7]['state'].update({'aliases':r['events'][7]['state'].pop('aliases')}))
 corrupt('nested_integer_boolean','observatory_lazy_aliases',lambda r:r['events'][7]['state'].__setitem__('warnings',True))
 corrupt('nested_integer_float','observatory_lazy_aliases',lambda r:r['events'][7]['state'].__setitem__('warnings',1.0))
 corrupt('string_tail','observatory_lazy_aliases',lambda r:r['events'][7]['state']['observatories'][2]['name'].__setitem__('bytes',r['events'][7]['state']['observatories'][2]['name']['bytes'][:8]+'00'+r['events'][7]['state']['observatories'][2]['name']['bytes'][10:]))
 corrupt('knownness','observatory_bounded_tokens',lambda r:r['events'][-1]['state']['observatories'][-1]['name'].__setitem__('known',1))
 corrupt('source_capacity','observatory_bounded_tokens',lambda r:r['events'][-1]['state']['observatories'][-1]['name'].__setitem__('capacity',33))
 corrupt('alias_list_order','observatory_lazy_aliases',lambda r:r['events'][-1]['state']['aliases'].reverse())
 corrupt('alias_warning_lifetime','observatory_lazy_aliases',lambda r:r['events'][-1]['state']['aliases'][0].__setitem__('warned',False))
 corrupt('eop_cache_lifetime','eop_first_use_cache',lambda r:r['events'][-1]['result']['outputs'][2].__setitem__('bits','0000000000000000'))
 corrupt('eop_alias_write_order','eop_outputs_alias_2',lambda r:r['events'][-1]['result']['outputs'].reverse())
 corrupt('weather_cache_lifetime','meteorology_lifetime',lambda r:r['events'][-1]['state'].__setitem__('meteorology_initialized',0))
 corrupt('table_tail','meteorology_direct',lambda r:r['events'][23]['result']['table']['header'].__setitem__('bytes',r['events'][23]['result']['table']['header']['bytes'][:-2]+'00'))
 corrupt('warning_stream','meteorology_warning_history_0',lambda r:r.__setitem__('stdout',r['stdout'].replace('TROP1','TROP9')))
 corrupt('stderr_prefix','eop_missing',lambda r:r.__setitem__('stderr',r['stderr'][:-1]))
 corrupt('exit_prefix','eop_bad_after_valid',lambda r:r['events'][-1]['caller_eop_samples'].clear())
 corrupt('non_target_retention','integrated_multi',lambda r:r['events'][-1]['result'].__setitem__('all_other_typed_fields_retained',False))
 original=json.loads(gzip.decompress((run.U/'outputs/reference.json.gz').read_bytes()))
 i=next(i for i,r in enumerate(expected) if r['id']=='nmf_latitude_22.5')
 rejected('original_wet_bug_rejected',lambda:run.compare(run.project(expected[i]),run.project(original[i])))
 rejected('missing_case',lambda:run.compare([c['id'] for c in expected],[c['id'] for c in expected][1:]))
 rejected('permuted_cases',lambda:run.compare([c['id'] for c in expected],list(reversed([c['id'] for c in expected]))))
 for field in ['executable_sha256','compiler_sha256','sources']:
  stale=copy.deepcopy(ident);stale[field]='stale'
  rejected('stale_'+field,lambda:run.execute(catalog[0],exe,stale,run.OUT/profile/'controls'/'never_launched'))
 # Fresh unchanged positive is an actual execution with a new nonce and current
 # source/compiler/executable/resource identity, not equality of stored objects.
 case=next(c for c in catalog if c['id']=='eop_first_use_cache')
 fresh=run.execute(case,exe,ident,run.OUT/profile/'controls'/'fresh_positive')
 run.compare(run.project(next(r for r in expected if r['id']==case['id'])),run.project(fresh))
 rows.append(dict(name='fresh_unchanged_case',passed=True))
 launch=json.loads((run.OUT/profile/'controls/fresh_positive/events.jsonl').read_text().splitlines()[0])
 for field in ['nonce','input_sha256']:
  stale=copy.deepcopy(launch);stale[field]='stale'
  rejected('stale_'+field,lambda:run.validate_launch(stale,launch['nonce'],launch['input_sha256']))
 # U's controls and all thirteen actual source mutations remain accepted evidence.
 controls=json.loads((run.U/'outputs/controls.json').read_text())
 mutants=json.loads(gzip.decompress((run.U/'outputs/mutations.json.gz').read_bytes()))
 assert len(controls['controls'])==29 and len(mutants)==13
 run.save(run.OUT/profile/'controls.json',dict(status='PASS',controls=rows,accepted_U_controls_sha256=run.sha(run.U/'outputs/controls.json'),accepted_U_mutations_sha256=run.sha(run.U/'outputs/mutations.json.gz'),accepted_U_mutations=13))
 return rows
