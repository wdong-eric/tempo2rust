"""Strict verifier controls and bounded native behavior mutations (not corrections)."""
import copy
import json
import shutil
from pathlib import Path
from inventory import HERE, sha, save, preservation_status
from profile import BUILD, compile_probe, actual_identity
from run import OUTPUT, read_gzip, write_gzip, compare, execute, validate, check_inputs, frozen_cases

MUTATIONS = [
 ('ut1_sign','ut1_interpolation','tai2ut1.C','return -ut1;','return ut1;'),
 ('ut1_boundary','ut1_interpolation','tai2ut1.C','mjd>mjd1 && mjd<mjd2','mjd>=mjd1 && mjd<mjd2'),
 ('ut1_cache_reload','ut1_cache','tai2ut1.C','if (read==0) /* Read data file */','count=0; count2=0; if (true) /* Read data file */'),
 ('tai_argument','tai_method_2','tai2ut1.C','ut1red((double)(psr[p].obsn[i].sat),','ut1red((double)(psr[p].obsn[i].sat)+0.25,'),
 ('vector_nonzero_ecliptic','vector_ecl_2','vectorPulsar.C','if (psr[p].eclCoord){','if (psr[p].eclCoord==1){'),
 ('vector_pm_scale','vector_ecl_0','vectorPulsar.C','M_PI/180.0*100.0;','M_PI/180.0;'),
 ('precession_transpose','precession','get_obsCoord.C','prn[j][i] = nut[i][0]','prn[i][j] = nut[i][0]'),
 ('satellite_units','satellite_spelling_0','get_obsCoord.C','observatory_earth[0]/=SPEED_LIGHT;','observatory_earth[0]*=1.0;'),
 ('satellite_epoch','polynomial_axis_0_mask_15','get_obsCoord.C','deltaT = (psr[p].obsn[i].sat - t0)*SECDAY;\t\t\n                        arg = deltaT;\n                        pos = psr[p].param[param_telx].val[0];','deltaT = (psr[p].obsn[i].sat - t0);\n                        arg = deltaT;\n                        pos = psr[p].param[param_telx].val[0];'),
 ('satellite_offset_mode','offset_axis_0_mode_2','get_obsCoord.C','ival = psr[p].telDX_v[k];','ival = m*(double)psr[p].obsn[i].sat+c;'),
 ('imag_retention','imag_shared','get_obsCoord.C','sscanf(psr[p].obsn[i].flagVal[k],"%lf",&obs->x);','double temporary_x=obs->x; sscanf(psr[p].obsn[i].flagVal[k],"%lf",&temporary_x);'),
 ('stl_bat_troposphere','satellite_spelling_2','get_obsCoord.C','psr[p].correctTroposphere = 0;','psr[p].correctTroposphere = 1;'),
 ('iau_rate','iau_direct','get_obsCoord.C','*(1.0+dut1dot)/86400.0;','/86400.0;'),
 ('iau_matrix_direction','iau_direct','get_obsCoord.C','iau_trxp_(t2c[0], zenith_trs, zenith_crs);','iau_rxp_(t2c[0], zenith_trs, zenith_crs);'),
 ('clock_two_bypass','driver_method_1_ecl_0','get_obsCoord.C','clockCorr!=0 && psr[p].obsn[i].clockCorr!=2','clockCorr!=0'),
]

def differences(a,b,path='$'):
    if type(a) is not type(b):return [dict(path=path,expected=a,actual=b)]
    if isinstance(a,dict):
        if list(a)!=list(b):return [dict(path=path+'.keys',expected=list(a),actual=list(b))]
        return [d for k in a for d in differences(a[k],b[k],path+'.'+k)]
    if isinstance(a,list):
        if len(a)!=len(b):return [dict(path=path+'.length',expected=len(a),actual=len(b))]
        return [d for i in range(len(a)) for d in differences(a[i],b[i],f'{path}[{i}]')]
    return [] if a==b else [dict(path=path,expected=a,actual=b)]

def rejected(name,fn,rows):
    try:fn()
    except (AssertionError,KeyError,ValueError) as e:
        rows.append(dict(name=name,status='REJECTED',witness=str(e)));return
    raise AssertionError('control unexpectedly passed: '+name)

def main():
    dest=OUTPUT/'controls.json';dest.unlink(missing_ok=True);before=preservation_status()
    catalog=frozen_cases();baseline=read_gzip(OUTPUT/'reference.json.gz');refhash=sha((OUTPUT/'reference.json.gz').read_bytes())
    by_label={c['label']:(c,r) for c,r in zip(catalog,baseline)}
    compare([c['id'] for c in catalog],[r['id'] for r in baseline],'membership')
    rows=[];compare(baseline,copy.deepcopy(baseline));rows.append(dict(name='positive_exact_copy',status='PASS'))
    original=by_label['imag_shared'][1]
    def damage(name,fn):
        x=copy.deepcopy(original);fn(x);rejected(name,lambda:compare(original,x),rows)
    damage('missing_action',lambda x:x['events'].pop())
    damage('extra_action',lambda x:x['events'].append(x['events'][-1]))
    damage('action_order',lambda x:x['events'].reverse())
    damage('field_membership',lambda x:x.pop('classification'))
    damage('field_order',lambda x:x.update(id=x.pop('id')))
    damage('status',lambda x:x.update(status=1))
    damage('diagnostic_stdout',lambda x:x.update(stdout=x['stdout']+'X'))
    damage('diagnostic_stderr',lambda x:x.update(stderr=x['stderr']+'X'))
    damage('retained_tail',lambda x:x['events'][-1]['caller']['observations'][0]['position'][5].update(bits='0000000000000000'))
    damage('signed_zero',lambda x:x['events'][-1]['caller']['observations'][0]['velocity'][0].update(bits='8000000000000000'))
    damage('shared_owner',lambda x:x['events'][-1]['caller']['pulsars'][1].update(observation_owner=0))
    damage('initialized_string',lambda x:x['events'][-1]['caller']['observations'][0]['telID'].update(bytes='00'))
    damage('shared_imag',lambda x:x['events'][-1]['state']['observatories'][2]['coordinates'][0].update(bits='0000000000000000'))
    for name,fn in [('case_drop',lambda x:x.pop()),('case_extra',lambda x:x.append(x[-1])),('case_order',lambda x:x.reverse())]:
        x=copy.deepcopy(baseline);fn(x);rejected(name,lambda:compare(baseline,x),rows)
    # Recursive scalar types: equal-valued Python scalars must still reject.
    for a,b in [(False,0),(True,1),(0,0.0),(1,True),(0.0,0),(None,False),('0',0)]:
        rejected('recursive_type_'+type(a).__name__+'_'+type(b).__name__+'_'+str(a),lambda:compare({'nested':[{'x':a}]},{'nested':[{'x':b}]}),rows)
    stored=json.loads((OUTPUT/'manifest.json').read_text())
    for section in ('cases_sha256','harness','native','contract_sha256','frozen_sha256'):
        x=copy.deepcopy(stored);x[section]='changed';rejected('identity_'+section,lambda:compare(stored,x),rows)
    exe=BUILD/'ordinary/probe';profile=json.loads((exe.parent/'profile.json').read_text());actual_identity(exe,profile)
    rows.append(dict(name='positive_actual_identity',status='PASS'))
    tmp=BUILD/'controls';tmp.mkdir(exist_ok=True)
    bad=tmp/'modified-probe';shutil.copyfile(exe,bad);bad.write_bytes(bad.read_bytes()+b'changed')
    rejected('changed_actual_executable',lambda:actual_identity(bad,profile),rows)
    dep=tmp/'dependency';dep.write_bytes(b'original');fake=copy.deepcopy(profile);fake['dependencies'][str(dep)]=sha(dep.read_bytes());dep.write_bytes(b'changed')
    rejected('changed_actual_dependency',lambda:actual_identity(exe,fake),rows)
    resource=tmp/'resource';resource.write_text('original');commands=tmp/'actions.tsv';commands.write_text('original');ih=sha(commands.read_bytes());c=dict(files={'resource':'original'})
    check_inputs(tmp,c,commands,ih);rows.append(dict(name='positive_input_identity',status='PASS'))
    resource.write_text('changed');rejected('changed_resource',lambda:check_inputs(tmp,c,commands,ih),rows)
    resource.write_text('original');commands.write_text('changed');rejected('changed_runner_input',lambda:check_inputs(tmp,c,commands,ih),rows)
    mutants=[]
    for i,(name,label,file,old,new) in enumerate(MUTATIONS):
        c,expected=by_label[label];exe,p=compile_probe(f'mutant{i:02}',(file,old,new))
        actual,receipt=execute(c,exe,p,OUTPUT/'mutations'/name);validate(c,actual)
        witness=differences(expected['events'],actual['events']);assert witness,(name,'no behavior witness')
        mutants.append(dict(name=name,case=c['id'],classification='deliberate-test-mutation-not-scientific-correction',witnesses=witness,profile=p,receipt=receipt,result=actual))
        print('PASS mutation',name,witness[0]['path'],flush=True)
    write_gzip(OUTPUT/'mutations.json.gz',mutants)
    after=preservation_status();assert sha((OUTPUT/'reference.json.gz').read_bytes())==refhash
    save(dest,dict(status='PASS' if not before and not after else 'PRESERVATION_DRIFT',preservation_before=before,preservation_after=after,controls=rows,mutations=len(mutants),mutation_sha256=sha((OUTPUT/'mutations.json.gz').read_bytes())))
    assert not before and not after
    print('PASS',len(rows),'controls;',len(mutants),'native behavior mutations')

if __name__=='__main__':main()
