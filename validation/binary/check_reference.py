#!/usr/bin/env python3
"""Contracts for *model.C callbacks, readParfile state, and formResiduals exports.
No standalone TEMPO2 counterpart: evidence validator, coverage and mutation checks.
"""
import argparse
from copy import deepcopy
from decimal import Decimal, getcontext
import json
from pathlib import Path
import subprocess
import sys
from prepare_fixtures import MODELS, inventory, save, sha
from run_reference import ROOT, BUILD, A2, SOURCE_PIN, A2_PIN

getcontext().prec=90
EXIT_CASES={'bt_ecc_exit','dd_ecc_exit','ddh_missing_stig','ddgr_negative_mass','ddgr_fb_rejected','t2_missing_epoch','t2pta_missing_t0'}
UNDEFINED={'ddk_derivatives_undefined','ell1h_h3_only','ell1h_h4_nharm','t2_ell1h_h4','ell1_orbifunc_step_undefined','t2_companions'}


def pairs(data):
    starts={}
    for row in data['records']:
        if row['record']=='start':starts[row['id'],row['repeat']]=row
        elif row['record']=='result':yield starts[row['id'],row['repeat']],row


def cls(value):
    x=Decimal(value)
    return 'NaN' if x.is_nan() else '+Infinity' if x==Decimal('Infinity') else '-Infinity' if x==Decimal('-Infinity') else 'finite'


def validate(data, case):
    assert data['schema_version']==1
    assert data['case']==case
    assert data['identity']['source_commit']==SOURCE_PIN and data['identity']['a2_revision']==A2_PIN
    for filename,digest in data['inputs'].items():assert sha(ROOT/'fixtures'/filename)==digest
    expected='source_exit' if case['name'] in EXIT_CASES else 'returned'
    assert data['process']['outcome']==expected
    assert data['process']['returncode']==(1 if expected=='source_exit' else 0)
    requests=[line.split() for line in (ROOT/'fixtures'/case['requests']).read_text().splitlines()]
    if expected=='source_exit':
        assert not any(r['record']=='result' for r in data['records'])
        assert data['diagnostics']
        return
    parsed,prepared=data['records'][:2]
    assert parsed['record']=='parsed' and parsed['model']==case['model']
    assert prepared['record']=='prepared' and len(prepared['observations'])==12
    for observation in prepared['observations']:
        for vector in ['earth_ssb_light_s','psr_position_unit']:
            assert len(observation[vector])==3 and all(cls(x)=='finite' for x in observation[vector])
            assert any(Decimal(x)!=0 for x in observation[vector]),'fabricated zero geometry'
    assert parsed['configuration']['units']=='TCB' and parsed['configuration']['refphs']==0
    assert parsed['configuration']['ne_sw_ifunc_count']==0
    assert {(s['label'],s['k']) for s in parsed['parameters']}.__len__()==len(parsed['parameters'])
    for s in parsed['parameters']:
        assert set(s['state'])=={'value','error','set','fit'}
        assert isinstance(s['state']['set'],int) and isinstance(s['state']['fit'],int)
        assert cls(s['state']['value'])=='finite' and cls(s['state']['error'])=='finite'
    expected_keys={(i,j) for i,r in enumerate(requests) for j in range(int(r[4]))}
    assert len(data['records'])==2+2*len(expected_keys)
    assert {(s['id'],s['repeat']) for s,r in pairs(data)}==expected_keys
    residuals={};results={}
    for start,row in pairs(data):
        request=requests[start['id']];op,lab,k,obs,repeats,inc,err,sem=request;k=int(k);obs=int(obs)
        assert [start['op'],start['label'],start['k'],start['obs']]==[op,lab,k,obs]
        assert start['repeat']<int(repeats)
        assert 'mutations' in row
        for m in row['mutations']:
            assert m['before']!=m['after'] and set(m['before'])==set(m['after'])=={'value','error','set','fit'}
        if op in ['native','wrapper','update_delay']:
            assert row['source_semantics']==sem
            if sem=='undefined_source':
                assert case['name'] in UNDEFINED
                assert not {'value','class','return_type','narrowed_binary64'}&row.keys()
            else:
                assert row['class']==cls(row['value'])
                assert row['return_type']==('binary128' if op!='wrapper' and case['model']=='DD' else 'binary64')
                assert cls(row['narrowed_binary64']) in ['finite','NaN','+Infinity','-Infinity']
        if op.startswith('update'):
            assert 'increment_binary64' in row and 'error_binary64' in row
            assert set(row['target_before'])==set(row['target_after'])=={'value','error','set','fit'}
            if row['target_before']!=row['target_after']:
                assert any(m['label']==lab and m['k']==k and m['before']==row['target_before'] and m['after']==row['target_after'] for m in row['mutations'])
            assert row['target_before']['set']==row['target_after']['set']
            assert row['target_before']['fit']==row['target_after']['fit']
        if op=='residual':
            assert case['integrated'] and len(row['observations'])==12
            residuals[k]=row['observations']
            for a,b in zip(prepared['observations'],row['observations']):
                for key in a:assert a[key]==b[key],(case['name'],key)
                for key in ['torb_s','pet_mjd','phase_turn','nphase_turn','residual_s','phase_integer_turn','reduced_phase_reconstructed_turn']:
                    assert cls(b[key])=='finite',(case['name'],key)
                # This is a decimal replay of the documented field relation, not a Rust gate.
                assert abs((Decimal(b['bbat_mjd'])-Decimal(b['pet_mjd']))*86400-Decimal(b['torb_s']))<Decimal('1e-23')
                assert abs(Decimal(b['phase_turn'])-Decimal(b['phase_integer_turn'])-Decimal(b['reduced_phase_reconstructed_turn']))<Decimal('1e-23')
                assert Decimal(b['pulse_number'])==Decimal(b['phase_integer_turn'])+Decimal(b['nphase_turn'])
        results[(op,lab,k,obs,err,start['repeat'])]=row
    if case['integrated']:
        assert set(residuals)=={0,1}
        raw,centered=residuals[0],residuals[1]
        assert Decimal(raw[0]['residual_s'])==0
        mean=sum(Decimal(x['residual_s']) for x in raw)/12
        for r,c in zip(raw,centered):
            for field in r:
                if field!='residual_s':assert r[field]==c[field]
            assert abs(Decimal(r['residual_s'])-mean-Decimal(c['residual_s']))<Decimal('1e-30')
    for (op,l,k,i,e,rep),r in results.items():
        if op=='wrapper':
            n=results['native',l,k,i,e,rep]
            assert r['mutations']==n['mutations']
            if r['source_semantics']=='defined':assert r['value']==n['narrowed_binary64']
        if op=='update_wrapper':
            n=results['update_native',l,k,i,e,rep]
            for field in ['mutations','target_before','target_after','increment_binary64','error_binary64']:assert r[field]==n[field]
    # Mutation omission is a scientific contract violation even when all numeric returns agree.
    if case['name'] in ['dd_sini_clamp','ell1_sini_clamp','ell1k_sini_clamp','t2_sini_clamp','t2_sini_negative','t2_ecc_clamp']:
        label='param_ecc' if case['name']=='t2_ecc_clamp' else 'param_sini'
        first=next(r for s,r in pairs(data) if s['op']=='native' and s['label']=='delay')
        assert any(m['label']==label for m in first['mutations'])
    if case['name']=='ddgr_derived':
        derived=[r for s,r in pairs(data) if s['label']=='derived']
        assert {m['label'] for m in derived[0]['mutations']}=={'param_sini','param_omdot','param_gamma','param_pbdot','param_dr','param_dtheta'}
        assert derived[1]['mutations']==[]
    if case['name']=='ell1_sini_rounding':assert all(not r['mutations'] for s,r in pairs(data))


def validate_inventory(inv, schema):
    assert set(inv)==set(MODELS)==set(schema['requirements']['models'])
    assert inv==inventory()
    assert set(schema['parameter_units'])>={p for m in inv.values() for p in m['parameter_reads']}
    assert schema['field_units']['torb_s']=='s; added to spin time, subtracted from pet'
    assert schema['parameter_units']['param_pb']=='PAR/storage days; local period seconds. BT/BTJ/T2 update +=val days; other PB updates +=val/86400 days.'
    assert schema['representation']['nonfinite']==['NaN','+Infinity','-Infinity']


def coverage(data, inv):
    models={}
    for model in MODELS:
        ds=[d for d in data.values() if d['case']['model']==model]
        labels={l:{'native':[],'wrapper':[],'update_native':[],'update_wrapper':[]} for l in sorted(set(inv[model]['derivative_labels'])|set(inv[model]['update_labels']))}
        presence={l:{'present':[],'absent':[]} for l in inv[model]['parameter_reads']}
        for d in ds:
            name=d['case']['name']
            if d['records'] and d['records'][0]['record']=='parsed':
                for p in d['records'][0]['parameters']:
                    if p['label'] in presence:
                        presence[p['label']]['present' if p['state']['set'] else 'absent'].append([name,p['k']])
            for s,r in pairs(d):
                if s['label'] in labels and s['op'] in labels[s['label']]:
                    labels[s['label']][s['op']].append({'case':name,'request':s['id'],'repeat':s['repeat'],'k':s['k'],'obs':s['obs'],
                                                      'source_semantics':r.get('source_semantics','defined'),'class':r.get('class','state_update' if s['op'].startswith('update') else 'withheld')})
        for label in inv[model]['derivative_labels']:
            assert labels[label]['native'] and labels[label]['wrapper'],(model,label)
        for label in inv[model]['update_labels']:
            assert labels[label]['update_native'] and labels[label]['update_wrapper'],(model,label)
        models[model]={'native_return':inv[model]['native_return'],'callbacks':labels,'parameter_presence':presence,
                       'cases':[d['case']['name'] for d in ds],'remaining_coverage':'BRANCHES.md; source_inventory.json conditions are not runtime branch coverage'}
    return {'counterparts':'*model.C callbacks and readParfile.C parameter presence','models':models,
            'source_condition_inventory_sha256':sha(ROOT/'source_inventory.json'),'branch_review_sha256':sha(ROOT/'BRANCHES.md'),
            'schema_sha256':sha(ROOT/'schema.json'),'case_count':len(data),'no_rust_acceptance':True}


def diagnostics(data):
    out={};narrow=[];scales={}
    def delay(name):return [r['value'] for s,r in pairs(data[name]) if s['op']=='native' and s['label']=='delay']
    assert delay('bt_baseline')==delay('bt_edot_zero')
    out['bt_edot_delay_max_difference_s']='0'
    assert delay('ell1k_baseline')==delay('ell1k_epsdot_zero')
    out['ell1k_epsdot_delay_max_difference_s']='0'
    for name,d in data.items():
        for s,r in pairs(d):
            if 'value' in r and r['class']=='finite':
                if s['label']=='delay':scales.setdefault(d['case']['model'],[]).append(abs(Decimal(r['value'])))
                if r['return_type']=='binary128':narrow.append((abs(Decimal(r['value'])-Decimal(r['narrowed_binary64'])),name,s['label']))
    out['largest_native_wrapper_narrowing']={'absolute_difference':str(max(narrow)[0]),'case':max(narrow)[1],'label':max(narrow)[2],
                                            'unit':'source derivative coordinate; see schema parameter map'}
    out['baseline_delay_max_abs_s']={m:str(max(abs(Decimal(v)) for v in delay(m.lower()+'_baseline'))) for m in MODELS}
    out['classification_counts']={c:sum(r.get('class')==c for d in data.values() for s,r in pairs(d)) for c in ['finite','NaN','+Infinity','-Infinity']}
    out['withheld_return_count']=sum(r.get('source_semantics')=='undefined_source' for d in data.values() for s,r in pairs(d))
    out['source_exit_cases']=sorted(EXIT_CASES)
    out['defined_integrated_cases']=sum(d['case']['integrated'] for d in data.values())
    out['all_native_vs_wrapper_and_update_comparisons']='exact within build for defined calls'
    # Capture observed PB updates to make the coordinate distinction independently inspectable.
    out['pb_update_observations']={}
    for model in MODELS:
        if model=='BTX':continue
        for s,r in pairs(data[model.lower()+'_baseline']):
            if s['op']=='update_native' and s['label']=='param_pb' and r['error_binary64'][0]!='-' and r['error_binary64']!='NaN':
                out['pb_update_observations'][model]={'increment':r['increment_binary64'],'before_days':r['target_before']['value'],'after_days':r['target_after']['value']};break
    return out


def mutation_checks(data,cases,inv,schema):
    passed=[]
    def rejected(name,call):
        try:call()
        except (AssertionError,KeyError,ValueError):passed.append(name)
        else:raise AssertionError('mutation accepted: '+name)
    byname={c['name']:c for c in cases}
    x=deepcopy(inv);del x['BT'];rejected('missing model',lambda:validate_inventory(x,schema))
    x=deepcopy(inv);x['DD']['derivative_labels'].remove('param_pb');rejected('missing derivative label',lambda:validate_inventory(x,schema))
    x=deepcopy(schema);x['parameter_units']['param_pb']='seconds';rejected('altered PB storage units',lambda:validate_inventory(inv,x))
    x=deepcopy(data['dd_sini_clamp']);next(r for s,r in pairs(x) if s['label']=='delay')['mutations']=[]
    rejected('omitted SINI mutation',lambda:validate(x,byname['dd_sini_clamp']))
    x=deepcopy(data['ddh_zero_h3_stig']);next(r for s,r in pairs(x))['class']='+Infinity'
    rejected('NaN changed to infinity class',lambda:validate(x,byname['ddh_zero_h3_stig']))
    x=deepcopy(data['dd_baseline']);x['records'].pop();rejected('omitted wrapper or residual result',lambda:validate(x,byname['dd_baseline']))
    x=deepcopy(data['ddk_derivatives_undefined']);r=next(r for s,r in pairs(x));r['value']='0';r['class']='finite'
    rejected('manufactured undefined-source numeric result',lambda:validate(x,byname['ddk_derivatives_undefined']))
    x=deepcopy(data['bt_baseline']);r=next(r for s,r in pairs(x) if s['op']=='update_native');r['mutations']=[]
    rejected('omitted update mutation',lambda:validate(x,byname['bt_baseline']))
    x=deepcopy(data['dd_baseline']);r=next(r for s,r in pairs(x) if s['op']=='residual');r['observations'][1]['pulse_number']=str(int(r['observations'][1]['pulse_number'])+1)
    rejected('one pulse changed',lambda:validate(x,byname['dd_baseline']))
    x=deepcopy(data['dd_baseline']);r=next(r for s,r in pairs(x) if s['op']=='wrapper');r['value']='0'
    rejected('wrapper narrowing changed',lambda:validate(x,byname['dd_baseline']))
    x=deepcopy(data['ddk_baseline']);x['records'][1]['observations'][0]['earth_ssb_light_s']=['0','0','0']
    rejected('fabricated zero geometry',lambda:validate(x,byname['ddk_baseline']))
    return passed


def main():
    parser=argparse.ArgumentParser();parser.add_argument('--regenerate',action='store_true');args=parser.parse_args()
    cases=json.loads((ROOT/'cases.json').read_text());inv=json.loads((ROOT/'source_inventory.json').read_text());schema=json.loads((ROOT/'schema.json').read_text())
    validate_inventory(inv,schema)
    data={c['name']:json.loads((ROOT/'outputs'/(c['name']+'.json')).read_text()) for c in cases}
    assert {p.stem for p in (ROOT/'outputs').glob('*.json') if p.name!='manifest.json'}==set(data)
    manifest=json.loads((ROOT/'outputs/manifest.json').read_text())
    assert manifest['cases_sha256']==sha(ROOT/'cases.json') and manifest['inventory_sha256']==sha(ROOT/'source_inventory.json')
    assert manifest['input_hashes_sha256']==sha(ROOT/'input_hashes.json')
    assert manifest['outputs']=={p.name:sha(p) for p in sorted((ROOT/'outputs').glob('*.json')) if p.name!='manifest.json'}
    for case in cases:validate(data[case['name']],case)
    cov=coverage(data,inv);diag=diagnostics(data);mutations=mutation_checks(data,cases,inv,schema)
    print('Validated',len(cases),'cases;',len(mutations),'meaningful mutations rejected.',flush=True)
    regen=None
    if args.regenerate:
        destination=BUILD/'regenerated';destination.mkdir(exist_ok=True)
        subprocess.run([sys.executable,str(ROOT/'run_reference.py'),'--output',str(destination)],check=True)
        for p in sorted((ROOT/'outputs').iterdir()):
            if p.suffix in ['.json','.log']:assert p.read_bytes()==(destination/p.name).read_bytes(),p.name
        regen={'files_compared':len(list((ROOT/'outputs').iterdir())),'all_bytes_identical':True,'max_numeric_difference':'0','scope':'same exporter build, source/library/runtime; includes diagnostics and geometry'}
        # Input generation is also checked without discarding the declared input identities.
        before={p.name:sha(p) for p in (ROOT/'fixtures').iterdir()}
        subprocess.run([sys.executable,str(ROOT/'prepare_fixtures.py')],check=True)
        assert before=={p.name:sha(p) for p in (ROOT/'fixtures').iterdir()}
        assert inv==json.loads((ROOT/'source_inventory.json').read_text())
        assert cases==json.loads((ROOT/'cases.json').read_text())
    save(ROOT/'coverage.json',cov);save(ROOT/'observations.json',diag)
    verification={'cases_validated':len(cases),'models':MODELS,'mutation_rejections':mutations,'regeneration':regen,
                  'scope':'Implementation/evidence integrity checks, not physical validation or Rust acceptance',
                  'diagnostic_relation_bounds':{'pet_seconds':'1e-23','reduced_phase_turn':'1e-23','centering_seconds':'1e-30'},
                  'schema_sha256':sha(ROOT/'schema.json'),'outputs_manifest_sha256':sha(ROOT/'outputs/manifest.json')}
    # Do not erase a completed repeatability result during a later read-only validation run.
    if not args.regenerate and (ROOT/'verification.json').exists():
        old=json.loads((ROOT/'verification.json').read_text())
        if old['outputs_manifest_sha256']==verification['outputs_manifest_sha256']:verification['regeneration']=old['regeneration']
    save(ROOT/'verification.json',verification)

if __name__=='__main__':main()
