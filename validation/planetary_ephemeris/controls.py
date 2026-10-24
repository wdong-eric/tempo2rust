"""Integrity challenges and real compiled semantic mutations, never oracle updates."""
import copy
import json
import shutil
from pathlib import Path
from inventory import HERE,sha,save,check
from profile import BUILD,compile_probe,resources,actual_identity
from run import OUTPUT,execute,validate,frozen_cases,read_gzip,write_gzip,compare,validate_numbers,verify_resources

MUTATIONS=[
 ('selection_nonzero','nonone_2','if ((psr[p].param[param_dphaseplanet].paramSet[iplanet] == 1) || (psr[p].param[param_dmassplanet].paramSet[iplanet] == 1))','if ((psr[p].param[param_dphaseplanet].paramSet[iplanet] != 0) || (psr[p].param[param_dmassplanet].paramSet[iplanet] != 0))'),
 ('sun_velocity_tail','units_2_frame_1','psr[p].obsn[i].sun_ssb[5]*=one_au/86400.0/SPEED_LIGHT;','psr[p].obsn[i].sun_ssb[5]*=one_au/SPEED_LIGHT;'),
 ('date_teph','clock_teph_history','psr[p].obsn[i].correctionTT_Teph/SECDAY+2400000.5;','0.0+2400000.5;'),
 ('difference_step','all_planets_0','jpl_pleph(ephem, jd+1e-6,','jpl_pleph(ephem, jd+2e-6,'),
 ('difference_unconditional','noops','planet_ssb_derv[iplanet][5]=   psr','planet_ssb_derv[iplanet][5]=  2.0 + psr'),
 ('si_scaling','units_1_frame_0','one_au = (double)one_au*IFTE_K;','one_au = (double)one_au;'),
 ('mass_flag_one','nonone_2','if (psr[p].param[param_dmassplanet].paramSet[iplanet])','if (psr[p].param[param_dmassplanet].paramSet[iplanet] == 1)'),
 ('mass_sign','all_planets_1','psr[p].obsn[i].earth_ssb[icomp] -= \n                            psr[p].param[param_dmassplanet]', 'psr[p].obsn[i].earth_ssb[icomp] += \n                            psr[p].param[param_dmassplanet]'),
 ('mass_frame_order','all_planets_1','equ2ecl(psr[p].obsn[i].planet_ssb[iplanet]);','(void)0;'),
 ('earth_velocity_rotation','units_2_frame_1','equ2ecl(psr[p].obsn[i].earth_ssb+3);','(void)0;'),
 ('ecliptic_predicate','units_2_frame_2','if (psr[p].eclCoord==1) equ2ecl(psr[p].obsn[i].sun_ssb);','if (psr[p].eclCoord!=0) equ2ecl(psr[p].obsn[i].sun_ssb);'),
 ('offset_right_edge','offset_0_4','(double)psr[p].obsn[i].sat < psr[p].telDX_t[k+1]','(double)psr[p].obsn[i].sat <= psr[p].telDX_t[k+1]'),
 ('stl_zero_velocity','station_0','for (k=3;k<6;k++) psr[p].obsn[i].earth_ssb[k]=0;','for (k=3;k<5;k++) psr[p].obsn[i].earth_ssb[k]=0;'),
 ('stl_failed_scan','stl_scan_history','sscanf(psr[p].obsn[i].flagVal[k],"%lf",&psr[p].obsn[i].earth_ssb[0]);','psr[p].obsn[i].earth_ssb[0]=0; sscanf(psr[p].obsn[i].flagVal[k],"%lf",&psr[p].obsn[i].earth_ssb[0]);'),
 ('random_zero_reseed','random_-19','if(*idum < 0 || iff == 0)','if(*idum <= 0 || iff == 0)'),
 ('random_shuffle','random_-19','j=(97*ix3)/m3;','j=(96*ix3)/m3;'),
 ('gaussian_cache_negative','gaussian_cached_negative','if  (iset == 0) {','if  (iset == 0 || *idum < 0) {'),
 ('noise_warning','noise_independence','if (addEphemNoise!=0)','if (addEphemNoise>0)'),
]
# noops intentionally has no observation; the unconditional-difference witness
# uses the unchanged initialized inactive arrays in an ordinary processing case.
MUTATIONS[4]=(MUTATIONS[4][0],'units_2_frame_0',*MUTATIONS[4][2:])

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
    except (AssertionError,KeyError,ValueError) as e:rows.append(dict(name=name,status='REJECTED',witness=str(e)));return
    raise AssertionError('control passed: '+name)

def main():
    check();catalog=frozen_cases();base=read_gzip(OUTPUT/'reference.json.gz');old=sha((OUTPUT/'reference.json.gz').read_bytes());by={c['label']:(c,r) for c,r in zip(catalog,base)}
    rows=[];original=by['station_0'][1]
    def damage(name,fn):
        x=copy.deepcopy(original);fn(x);rejected(name,lambda:compare(original,x),rows)
    compare(original,copy.deepcopy(original));rows.append(dict(name='positive_copy',status='PASS'))
    damage('drop_action',lambda x:x['events'].pop());damage('extra_action',lambda x:x['events'].append(x['events'][-1]));damage('reorder_actions',lambda x:x['events'].reverse())
    damage('drop_field',lambda x:x.pop('classification'));damage('field_order',lambda x:x.update(id=x.pop('id')))
    damage('status',lambda x:x.update(status=1));damage('stdout',lambda x:x.update(stdout=x['stdout']+'X'));damage('stderr',lambda x:x.update(stderr=x['stderr']+'X'))
    damage('nested_bool_for_int',lambda x:x['events'][-1]['caller']['pulsars'][0].update(nobs=True))
    damage('nested_float_for_int',lambda x:x['events'][-1]['caller']['pulsars'][0].update(nobs=1.0))
    damage('owner',lambda x:x['events'][-1]['caller']['pulsars'][1].update(observation_owner=0))
    damage('signed_zero',lambda x:x['events'][-1]['caller']['observations'][0]['vectors']['earth_ssb'][3].update(bits='8000000000000000'))
    damage('velocity_tail',lambda x:x['events'][-1]['caller']['observations'][0]['vectors']['sun_ssb'][5].update(bits='0000000000000000'))
    damage('inactive_planet',lambda x:x['events'][-1]['caller']['observations'][0]['planets'][0][8][5].update(bits='0000000000000000'))
    damage('offset_tail',lambda x:x['events'][-1]['caller']['pulsars'][0]['offsets'][2]['backing'][499][1].update(bits='0000000000000000'))
    damage('string_initializedness',lambda x:x['events'][-1]['caller']['observations'][0]['telID'].update(initialized_bytes=99))
    damage('seed',lambda x:x['events'][-1]['seeds'].__setitem__(0,-19))
    nonfinite=by['stl_scan_history'][1];bad=copy.deepcopy(nonfinite);bad['events'][-1]['caller']['observations'][0]['vectors']['earth_ssb'][1]['class']='+Infinity';rejected('nonfinite_class',lambda:compare(nonfinite,bad),rows)
    for name,fn in [('case_drop',lambda x:x.pop()),('case_order',lambda x:x.reverse())]:
        x=[r['id'] for r in base];y=x.copy();fn(y);rejected(name,lambda:compare(x,y),rows)
    rejected('finite_tag_nonfinite_bits',lambda:validate_numbers({'class':'finite','bits':'7ff0000000000000'}),rows)
    stored=json.loads((OUTPUT/'manifest.json').read_text())
    for section in stored:
        x=copy.deepcopy(stored);x[section]='changed';rejected('manifest_'+section,lambda:compare(stored,x),rows)
    exe,profile=compile_probe();actual_identity(exe,profile)
    tmp=BUILD/'controls';tmp.mkdir(exist_ok=True);badexe=tmp/'badprobe';shutil.copyfile(exe,badexe);badexe.write_bytes(badexe.read_bytes()+b'X')
    rejected('actual_executable',lambda:actual_identity(badexe,profile),rows)
    dep=tmp/'dependency';dep.write_bytes(b'a');fake=copy.deepcopy(profile);fake['dependencies'][str(dep)]=sha(b'a');dep.write_bytes(b'b');rejected('actual_dependency',lambda:actual_identity(exe,fake),rows)
    rm=resources();wrong=copy.deepcopy(rm);wrong['le']['sha256']='0'*64;rejected('actual_resource',lambda:verify_resources(wrong),rows)
    # A fresh altered-input run must fail behavioral comparison even under a valid executable identity.
    c,expected=by['units_2_frame_0'];changed=copy.deepcopy(c);changed['actions'][-2]=['driver','0','0']
    actual,receipt=execute(changed,exe,profile,OUTPUT/'challenge_raw',rm)
    rejected('fresh_altered_caller',lambda:compare(expected['events'],actual['events']),rows)
    witnesses=[]
    for i,(name,label,oldtext,newtext) in enumerate(MUTATIONS):
        c,expected=by[label];mut,mp=compile_probe(f'mutant{i:02}',(oldtext,newtext))
        actual,receipt=execute(c,mut,mp,OUTPUT/'mutations'/name,rm);validate(c,actual)
        diff=differences(expected,actual);assert diff,(name,'no numerical/state/diagnostic witness')
        witnesses.append(dict(name=name,case=c['id'],profile=mp,receipt=receipt,differences=diff,result=actual));print('PASS mutation',name,diff[0]['path'],flush=True)
    write_gzip(OUTPUT/'mutations.json.gz',witnesses);check();assert sha((OUTPUT/'reference.json.gz').read_bytes())==old
    save(OUTPUT/'controls.json',dict(status='PASS',controls=rows,mutations=len(witnesses),mutations_sha256=sha((OUTPUT/'mutations.json.gz').read_bytes())))
    print('PASS',len(rows),'controls,',len(witnesses),'compiled mutations')
if __name__=='__main__':main()
