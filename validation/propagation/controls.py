"""Evidence corruption controls and compiled source behavior mutations; never bug fixes."""
import copy
import json
import shutil
from inventory import HERE,sha,save,check
from profile import BUILD,compile_probe,actual_identity
from run import OUTPUT,read_gzip,write_gzip,compare,execute,validate,check_inputs,frozen_cases

MUTATIONS=[
 ('bclt_first_owner','bclt_tropo_0','calculate_bclt.C','psr[0].obsn[i].troposphericDelay=0;','psr[p].obsn[i].troposphericDelay=0;'),
 ('bypass_frequency','bclt_A_0','calculate_bclt.C','freqSSB = psr[p].obsn[i].freq;','freqSSB = psr[p].obsn[i].freq*1e6;'),
 ('radial_flag','bclt_A_1','calculate_bclt.C','param_pmrv].paramSet[0]==1','param_pmrv].paramSet[0]==0'),
 ('convergence_limit','iteration_limit','calculate_bclt.C','loop < 100','loop < 2'),
 ('iteration_fast','iteration_fast_1_deleted_0','calculate_bclt.C','if (veryFast==1) break;','if (veryFast==7) break;'),
 ('shapiro_bypass_retention','shapiro_bypass','shapiro_delay.C','psr[p].obsn[i].shapiroDelaySun = 0.0;','psr[p].obsn[i].shapiroDelaySaturn = 0.0; psr[p].obsn[i].shapiroDelaySun = 0.0;'),
 ('shapiro_planet_factor','shapiro_A_2','shapiro_delay.C','shapiroDelaySaturn = psr[p].planetShapiro*delay;','shapiroDelaySaturn = delay;'),
 ('dm_velocity_flags','dm_velocity_STL_FBAT_3','dm_delays.C','psr[p].param[param_telz].paramSet[1] == 1','psr[p].param[param_telz].paramSet[1] != 0'),
 ('dm_dilate','dm_frequency_1400000000.0_-1','dm_delays.C','freqf /= psr[p].obsn[i].einsteinRate;','freqf *= 1.0;'),
 ('dm_series_owner','derivative_cross_selector','dm_delays.C','if (psr->dm_series_type == series_taylor_pn) {','if (psr[p].dm_series_type == series_taylor_pn) {',2),
 ('dm_flag_add','ordered_flags','dm_delays.C','dmval += dmval2;','dmval = dmval2;'),
 ('dm_warning_lifetime','dmmodel_1','dm_delays.C','static int t=0xFFFF;','static int t=0xFFFF; t=0xFFFF;'),
 ('dm_warning_owner','dmmodel_cross_warning','dm_delays.C','psr->param[param_dmmodel].linkTo[0] != param_dm','psr[p].param[param_dmmodel].linkTo[0] != param_dm'),
 ('dm_shape_owner','shape_cross_pulsar','dm_delays.C','psr->TNShapeletEvCoef[iTNShape],','psr[p].TNShapeletEvCoef[iTNShape],'),
 ('dm_shape_epoch','shape_event_2_0','dm_delays.C','if (t==0) t = (double)psr[p].obsn[i].sat;','if (t==0) t = (double)psr[p].obsn[i].sat+1;'),
 ('dmx_boundary','dmx_chromx_58000','dm_delays.C','sat > psr[p].param[param_dmxr1].val[k]','sat >= psr[p].param[param_dmxr1].val[k]'),
 ('cm_boundary','dm_frequency_1_1','dm_delays.C','if (freqf>=1)','if (freqf>1)'),
 ('solar_cycle_flag','solar_cycle_2','dm_delays.C','if (psr[p].param[param_ne_sw_sin].paramSet[0])','if (psr[p].param[param_ne_sw_sin].paramSet[0]==1)'),
 ('solar_ifunc','solar_cycle_1','dm_delays.C','tdis2 += ne_sw_ifunc *','tdis2 -= ne_sw_ifunc *'),
 ('bat_shapiro_gate','bats_-1_1','formBats.C','if (psr[p].calcShapiro==-1)','if (psr[p].calcShapiro==7)'),
 ('bat_bypass_retention','bats_retained_bypass','formBats.C','psr[p].obsn[i].bat = psr[p].obsn[i].sat;','{psr[p].obsn[i].batCorr=0; psr[p].obsn[i].bat = psr[p].obsn[i].sat;}'),
 ('bat_assembly','bats_0_1','formBats.C','psr[p].obsn[i].bat = psr[p].obsn[i].sat +','psr[p].obsn[i].bat = psr[p].obsn[i].sat + batcorr +'),
 ('secular_first_gate','secular_flags_1_0','secularMotion.C','psr[0].param[param_dshk].paramSet[0]==1','psr[p].param[param_dshk].paramSet[0]==1'),
 ('phase_clean_owner','phase_cross_0_1','refphs.C','if(psr->refphs==REFPHS_TZR)','if(psr[p].refphs==REFPHS_TZR)'),
 ('phase_zero_tail','phase_A','refphs.C','memset(obs,0,sizeof(observation));','obs->sat=0;'),
 ('phase_decrement','phase_A','refphs.C','psr[p].nobs--;','psr[p].nobs-=0;'),
 ('factorial','factorial_10','shapelet.C','ret *= i;','ret *= (i+1);'),
 ('hermite_two_writes','hermite_0','shapelet.C','pl[1]=2.0*x;','pl[1]=3.0*x;'),
 ('recurrence_retention','recurrence_0','shapelet.C','y0 = 1.0;','y1=0; y0 = 1.0;'),
 ('shape_normalization','shapelet_8_2','shapelet.C','result += shapeNorm *','result += 2*shapeNorm *'),
 ('ifunc_endpoint','ifunc_-1','ifunc.C','return yoffs[0];','return yoffs[0]+1;'),
 ('ifunc_basis','basis_2.5','ifunc.C','yoffs[ioff]=1;','yoffs[ioff]=2;'),
 ('sinc_limit','sinc_zero','ifunc.C','return 1.0;','return 2.0;'),
 ('solar_density','solar_model_0_grid','sw_delay.C','double slow_ne = 10;','double slow_ne = 11;'),
 ('solar_wind_speed','mcl_1','sw_delay.C','/3600.0/vel;','/3600.0/(vel+1);'),
 ('solar_tail_fill','mcl_1','sw_delay.C','helat[i] = helat[lp-1];','helat[i] = 0;'),
 ('sheet_count','sheet_full','sw_delay.C','return n;','return n-1;'),
 ('amod_zero','amod_ZERO','sw_delay.C','if (p==0) return 0;','if (p==0) return 1;'),
 ('elsun_float_narrowing','elsun_float_seconds','sw_delay.C','fday= (float)(secs)/86400.0;','fday= secs/86400.0;'),
 ('rotation_boundary','rotation_PI','sw_delay.C','if (crlne <= M_PI)','if (crlne < M_PI)'),
 ('calendar_leap','calendar_2000','sw_delay.C','else if(*mm==3)*iday=60 +(*dd);','else if(*mm==3)*iday=59 +(*dd);'),
 ('ecliptic_wrap','ecliptic_-2','sw_delay.C','lambda=lambdap+2*M_PI;','lambda=lambdap;'),
 ('precision_empty','precision_noop','storePrecision.C','{  \n    /*','{ psr->nStorePrecision++; \n    /*'),
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
 except (AssertionError,KeyError,ValueError,IndexError) as e:rows.append(dict(name=name,status='REJECTED',witness=str(e)));return
 raise AssertionError('control unexpectedly passed: '+name)
def main():
 check();catalog=frozen_cases();baseline=read_gzip(OUTPUT/'reference.json.gz');refhash=sha((OUTPUT/'reference.json.gz').read_bytes())
 by={c['id']:(c,r) for c,r in zip(catalog,baseline)};compare(list(by),[c['id'] for c in catalog])
 rows=[];compare(baseline,copy.deepcopy(baseline));rows.append(dict(name='positive_exact_copy',status='PASS'))
 original=by['composed'][1]
 def damage(name,fn):
  x=copy.deepcopy(original);fn(x);rejected(name,lambda:compare(original,x),rows)
 for name,fn in [
  ('missing_action',lambda x:x['events'].pop()),('extra_action',lambda x:x['events'].append(x['events'][-1])),
  ('action_order',lambda x:x['events'].reverse()),('field_membership',lambda x:x.pop('classification')),
  ('field_order',lambda x:x.update(id=x.pop('id'))),('status',lambda x:x.update(status=True)),
  ('stdout_truncation',lambda x:x.update(stdout=x['stdout'][:-1])),('stderr_append',lambda x:x.update(stderr=x['stderr']+'x')),
  ('state_tail',lambda x:x['events'][0]['state']['changes'].pop()),
  ('state_count_type',lambda x:x['events'][0]['state'].update(field_count=float(x['events'][0]['state']['field_count'])))]:damage(name,fn)
 for path in ['obs0.telID','obs0.observatory_earth','p0.obsn','p1.tzrobs.sat']:
  def corrupt(x):
   record=next(r for r in x['events'][0]['state']['changes'] if r[0]==path);record[1]={'changed':True}
  damage('typed_field_'+path,corrupt)
 for a,b in [(False,0),(True,1),(0,0.),(1,True),(None,False),('0',0)]:rejected('nested_type_'+repr(a)+'_'+repr(b),lambda:compare({'x':[a]},{'x':[b]}),rows)
 for name,fn in [('drop',lambda x:x.pop()),('extra',lambda x:x.append(x[-1])),('order',lambda x:x.reverse())]:
  x=copy.deepcopy(baseline);fn(x);rejected('case_'+name,lambda:compare(baseline,x),rows)
 exe=BUILD/'ordinary/probe';profile=json.loads((exe.parent/'profile.json').read_text());actual_identity(exe,profile)
 rows.append(dict(name='positive_launch_identity',status='PASS'));tmp=BUILD/'controls';tmp.mkdir(exist_ok=True)
 bad=tmp/'changed';shutil.copyfile(exe,bad);bad.write_bytes(bad.read_bytes()+b'changed');rejected('actual_binary_changed',lambda:actual_identity(bad,profile),rows)
 dep=tmp/'dependency';dep.write_bytes(b'original');fake=copy.deepcopy(profile);fake['dependencies'][str(dep)]=sha(dep.read_bytes());dep.write_bytes(b'changed');rejected('actual_dependency_changed',lambda:actual_identity(exe,fake),rows)
 commands=tmp/'actions';commands.write_text('original');ih=sha(commands.read_bytes());resource=tmp/'resource';resource.write_text('original');c=dict(files={'resource':'original'})
 check_inputs(tmp,c,commands,ih);rows.append(dict(name='positive_inputs',status='PASS'));resource.write_text('changed');rejected('resource_changed',lambda:check_inputs(tmp,c,commands,ih),rows);resource.write_text('original');commands.write_text('changed');rejected('input_changed',lambda:check_inputs(tmp,c,commands,ih),rows)
 for label,body in [('no_execution','#!/bin/sh\nexit 0\n'),('stale_nonce','#!/bin/sh\nprintf \'{"launch":"stale"}\\n\' > "$2"\n')]:
  fakeexe=tmp/label;fakeexe.write_text(body);fakeexe.chmod(0o755);fake=copy.deepcopy(profile);fake['executable_sha256']=sha(fakeexe.read_bytes())
  rejected(label,lambda:execute(catalog[0],fakeexe,fake,OUTPUT/'control_raw'),rows)
 mutants=[]
 for i,item in enumerate(MUTATIONS):
  name,label,file,old,new,*count=item
  if label=='ordered_flags':label=next(c['id'] for c in catalog if c['id'].startswith('flags_') and any(a[:3]==['flag','0','-dmo'] for a in c['actions']))
  if label=='amod_ZERO':label=next(c['id'] for c in catalog if c['actions'][-1]==['amod','1','0'])
  if label=='rotation_PI':label=next(c['id'] for c in catalog if c['actions'][-1][:2]==['rotation',str(__import__('math').pi)])
  if count:
   # Mutate the first identical DM selector only: CM remains the original neighboring path.
   old=old+'\n                series_fac *= k;\n            }\n            if (psr[p].param[param_dm]'
   new=new+'\n                series_fac *= k;\n            }\n            if (psr[p].param[param_dm]'
  c,expected=by[label];supporting=None
  if name=='radial_flag':
   # The nominal PMRV term rounds away at the short base epoch separation.
   # This predeclared finite input makes the selected source term observable.
   c=copy.deepcopy(c);c['id']='sensitivity_radial_flag'
   for a in c['actions']:
    if a[:3]==['parameter','0','pmrv']:a[4]='1000000000000'
   freeze=OUTPUT/'mutation_cases.json'
   if freeze.exists():compare(json.loads(freeze.read_text()),[c])
   else:save(freeze,[c])
   expected,first=execute(c,exe,profile,OUTPUT/'mutation_original')
   replay,second=execute(c,exe,profile,OUTPUT/'mutation_original_replay');compare(expected,replay);validate(c,expected)
   supporting=dict(first=first,replay=second,original=expected)
  ex,p=compile_probe(f'mutant{i:02}',(file,old,new));actual,receipt=execute(c,ex,p,OUTPUT/'mutations'/name)
  witness=differences(expected,actual);assert witness,(name,'no behavior witness')
  assert any(not w['path'].endswith('private') for w in witness)
  mutants.append(dict(name=name,case=label,classification='deliberate-control-not-correction',witnesses=witness,profile=p,receipt=receipt,result=actual,supporting_original=supporting))
  write_gzip(OUTPUT/'mutations.json.gz',mutants);print('PASS mutation',name,witness[0]['path'],flush=True)
 check();assert sha((OUTPUT/'reference.json.gz').read_bytes())==refhash
 save(OUTPUT/'controls.json',dict(status='PASS',script_sha256=sha((HERE/'controls.py').read_bytes()),controls=rows,mutations=len(mutants),mutation_sha256=sha((OUTPUT/'mutations.json.gz').read_bytes())))
 print('PASS',len(rows),'controls;',len(mutants),'compiled behavior mutations')
if __name__=='__main__':main()
