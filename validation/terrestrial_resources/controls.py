"""Comparator, actual launch/input identity and defined compiled behavioral controls."""
import copy
import json
import shutil
from pathlib import Path
from inventory import HERE, sha, save, preservation_status
from profile import BUILD, compile_probe, actual_identity
from fixtures import cases
from run import OUTPUT, read_gzip, write_gzip, compare, execute, validate, check_inputs, frozen_cases

MUTATIONS = [
 ('alias_accumulation','observatory_lazy_aliases','observatory.C',
  '//DynamicArray_init(&list.aliases, 128);','list.aliases.clear();'),
 ('observatory_append','observatory_reinitialize','observatory.C',
  'readObservatoryFile(fname);','observatories.clear(); readObservatoryFile(fname);'),
 ('eop_first_cache','eop_first_use_cache','eop.C',
  'if (first)\n    {\n        load_EOP(EOPsamples,eopcFile);',
  'if (true)\n    {\n        EOPsamples.clear(); load_EOP(EOPsamples,eopcFile);'),
 ('positive_leap_strict','eop_interpolation_plus_half','eop.C',
  'samp[isamp].dut1 - samp[isamp-1].dut1 > 0.5','samp[isamp].dut1 - samp[isamp-1].dut1 >= 0.5'),
 ('negative_leap_strict','eop_interpolation_minus_half','eop.C',
  'samp[isamp].dut1 - samp[isamp-1].dut1 < -0.5','samp[isamp].dut1 - samp[isamp-1].dut1 <= -0.5'),
 ('eop_derivative_spacing','eop_interpolation_none','eop.C',
  ') / 86400.0;',') / (86400.0 * (samp[isamp].mjd - samp[isamp-1].mjd));'),
 ('meteorology_precedence','meteorology_precedence','tropo.C',
  'tables->push_back(func);','tables->insert(tables->begin(),func);'),
 ('pressure_zero_fallback','meteorology_precedence','tropo.C','sap = 101.325;','sap = 0.0;'),
 ('wet_mixed_coefficient','nmf_latitude_22.5','tropo.C',
  'c = frac*bs[ilat2] + (1.0-frac)*cs[ilat1];','c = frac*cs[ilat2] + (1.0-frac)*cs[ilat1];'),
 ('warning_delay_order','integrated_warning_order','tropo.C','&& !warned)','&& true)'),
 ('meteorology_first_cache','meteorology_lifetime','tropo.C',
  'if (meteorology_tables_initialized != 1)','if (true)'),
 ('clock_season','integrated_clock_season','tropo.C','+ getCorrection(psr[p].obsn+i,','+ 0.0 * getCorrection(psr[p].obsn+i,'),
 ('unconditional_delay_reset','integrated_flags','tropo.C',
  'psr[p].obsn[i].troposphericDelay = 0.0;\n            if','psr[p].obsn[i].troposphericDelay += 0.0;\n            if'),
]

def rejected(name,fn,rows):
    try: fn()
    except (AssertionError,KeyError,ValueError) as e:
        rows.append(dict(name=name,status='REJECTED',witness=str(e)));return
    raise AssertionError('control unexpectedly passed: '+name)

def first_difference(a,b,path='$'):
    if type(a)!=type(b):return path
    if isinstance(a,dict):
        if list(a)!=list(b):return path+'.keys'
        for k in a:
            d=first_difference(a[k],b[k],path+'.'+k)
            if d:return d
    elif isinstance(a,list):
        if len(a)!=len(b):return path+'.length'
        for i in range(len(a)):
            d=first_difference(a[i],b[i],f'{path}[{i}]')
            if d:return d
    elif a!=b:return path
    return None

def main():
    destination=OUTPUT/'controls.json';destination.unlink(missing_ok=True);preservation_before=preservation_status()
    catalog=frozen_cases();baseline=read_gzip(OUTPUT/'reference.json.gz');by_id={r['id']:r for r in baseline}
    compare([c['id'] for c in catalog],list(by_id),'oracle membership')
    reference_hash=sha((OUTPUT/'reference.json.gz').read_bytes());rows=[]
    compare(baseline,copy.deepcopy(baseline));rows.append(dict(name='positive_exact_copy',status='PASS'))
    source=by_id['observatory_lazy_aliases']
    def damage(name,fn):
        altered=copy.deepcopy(source);fn(altered);rejected(name,lambda:compare(source,altered),rows)
    damage('missing_action',lambda x:x['events'].pop())
    damage('extra_action',lambda x:x['events'].append(x['events'][-1]))
    damage('permuted_actions',lambda x:x['events'].reverse())
    damage('missing_field',lambda x:x.pop('classification'))
    damage('field_order',lambda x:x.update({'id':x.pop('id')}))
    damage('wrong_case',lambda x:x.update(id='different'))
    damage('status',lambda x:x.update(status=1))
    damage('stdout',lambda x:x.update(stdout=x['stdout']+'x'))
    damage('stderr',lambda x:x.update(stderr=x['stderr']+'x'))
    damage('state_omission',lambda x:x['events'][-1]['state']['observatories'].pop())
    damage('alias_history',lambda x:x['events'][-1]['state']['aliases'][0].update(warned=False))
    damage('known_retained_tail',lambda x:x['events'][-1]['state']['observatories'][1]['name'].update(known=1))
    damage('initialized_byte',lambda x:x['events'][-1]['state']['observatories'][0]['name'].update(bytes='00'))
    damage('finite_bits',lambda x:x['events'][-1]['state']['observatories'][0]['coordinates'][0].update(bits='0000000000000000'))
    damage('nonfinite_class',lambda x:x['events'][-1]['state']['observatories'][0]['coordinates'][0].update({'class':'NaN'}))
    altered=copy.deepcopy(baseline);altered.pop();rejected('case_omission',lambda:compare(baseline,altered),rows)
    altered=copy.deepcopy(baseline);altered.reverse();rejected('case_order',lambda:compare(baseline,altered),rows)
    # Check the normal manifest gate, not a separate permissive comparator.
    stored=json.loads((OUTPUT/'manifest.json').read_text())
    for section in ('cases_sha256','harness','native','contract_sha256','frozen_sha256'):
        altered=copy.deepcopy(stored);altered[section]='changed'
        rejected('manifest_'+section,lambda:compare(stored,altered),rows)
    exe=BUILD/'ordinary/probe';profile=json.loads((exe.parent/'profile.json').read_text())
    actual_identity(exe,profile);rows.append(dict(name='positive_actual_execution_identity',status='PASS'))
    tmp=BUILD/'controls';tmp.mkdir(exist_ok=True)
    bad=tmp/'modified-probe';shutil.copyfile(exe,bad);bad.write_bytes(bad.read_bytes()+b'changed')
    rejected('actual_changed_executable',lambda:actual_identity(bad,profile),rows)
    dep=tmp/'dependency';dep.write_bytes(b'original');fake=copy.deepcopy(profile)
    fake['dependencies'][str(dep)]=sha(dep.read_bytes());dep.write_bytes(b'changed')
    rejected('actual_changed_dependency',lambda:actual_identity(exe,fake),rows)
    c=dict(files={'resource':'original'});resource=tmp/'resource';resource.write_text('original')
    commands=tmp/'actions.tsv';commands.write_text('original');ih=sha(commands.read_bytes())
    check_inputs(tmp,c,commands,ih);resource.write_text('changed')
    rejected('actual_changed_resource',lambda:check_inputs(tmp,c,commands,ih),rows)
    resource.write_text('original');commands.write_text('changed')
    rejected('actual_changed_runner_input',lambda:check_inputs(tmp,c,commands,ih),rows)
    # Supporting behavioral assertions do not define replacement golden values.
    def ends(name,op):return [e for e in by_id[name]['events'] if e['phase']=='end' and e['operation']==op]
    assert ends('glob_nomatch_host','glob_nomatch')[0]['result']['pathv_null'] is True
    e=ends('eop_first_use_cache','eop');assert all(x['result']==e[0]['result'] for x in e)
    helper=ends('integrated_helper_prediction','predict')[0]['result']['delay']
    integrated=ends('integrated_multi','compute')[0]['result']['after']['observations'][0]['delay']
    compare(helper,integrated,'independent_helper_prediction')
    rows.append(dict(name='independent_helper_matches_integrated',status='PASS'))
    current={c['id']:c for c in catalog};mutants=[]
    for i,(name,case_id,file,old,new) in enumerate(MUTATIONS):
        exe,profile=compile_probe(f'mutant{i:02}',(file,old,new))
        result,receipt=execute(current[case_id],exe,profile,OUTPUT/'mutations'/name)
        validate(current[case_id],result)
        # Events contain state/value observations only, no executable/source hash.
        witness=first_difference(by_id[case_id]['events'],result['events'])
        assert witness,(name,'identity/stream differences alone are not a behavioral witness')
        mutants.append(dict(name=name,case=case_id,witness=witness,profile=profile,receipt=receipt,result=result))
        print('DETECTED',name,witness,flush=True)
    write_gzip(OUTPUT/'mutations.json.gz',mutants)
    preservation_after=preservation_status();assert sha((OUTPUT/'reference.json.gz').read_bytes())==reference_hash
    status='PRESERVATION_DRIFT' if preservation_before or preservation_after else 'PASS'
    save(destination,dict(status=status,native_status='PASS',preservation_before=preservation_before,preservation_after=preservation_after,controls=rows,mutations=[{k:m[k] for k in ('name','case','witness')} for m in mutants],
         mutations_sha256=sha((OUTPUT/'mutations.json.gz').read_bytes()),reference_sha256=reference_hash))
    print('PASS',len(rows),'controls and',len(mutants),'compiled behavioral witnesses')
    assert status=='PASS','native controls completed; preservation gate remains unresolved'

if __name__=='__main__':main()
