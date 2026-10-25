#!/usr/bin/env python3
"""Public PAR/TIM integration against explicitly corrected native source.

Comparison projection is fixed here before execution: exact ordered identity,
flags, selectors and clock targets; adopted terrestrial-v1 scalar field gates.
Only fields assigned by the source on the chosen branch are compared. Inactive
source output storage remains null in public Rust output, never a zero oracle.
"""
from decimal import Decimal,localcontext
from pathlib import Path
import argparse,hashlib,json,os,subprocess
import native_pipeline as native
HERE=native.HERE;ROOT=native.ROOT;BUILD=native.BUILD
FIELD={
 'tt_to_tb':'tt_tb_s','tt_to_teph':'tt_teph_s','roemer':'roemer_s','dispersion_chromatic':'dispersion_s','solar_wind':'solar_dispersion_s',
 'troposphere':'troposphere_s','bat_correction':'bat_correction_day','shklovskii':'shklovskii_s',
 **{'shapiro_'+body:'shapiro_'+body+'_s' for body in ['sun','jupiter','saturn','venus','uranus','neptune']},
 'frequency':'frequency_mhz','frequency_ssb':'frequency_ssb_hz','uncertainty':'toa_error_us',
 'sat_mjd':'sat_mjd','bat_mjd':'bat_mjd','bbat_mjd':'bbat_mjd'}
GATES=json.loads((ROOT/'rust/tests/terrestrial/contract.json').read_text())['fields']
def compare(reference,result):
 state={}
 for line in reference.splitlines():
  p,i,key,value=line.split('\t',3);state.setdefault((int(p),int(i)),{})[key]=value
 errors=[];measurements=[]
 def exact(a,b,label):
  if str(a)!=str(b):errors.append(f'{label}: {a!r} != {b!r}')
 native_pulsars=sorted(p for p,i in state if i==-2)
 exact(native_pulsars,list(range(len(result['pulsars']))),'ordered pulsar membership')
 if native_pulsars!=list(range(len(result['pulsars']))):return errors,measurements
 def numeric(a,b,key,label,scale=1):
  gate=GATES[key]
  with localcontext() as context:
   context.prec=90
   da,db=Decimal(str(a)),Decimal(str(b))
   if not da.is_finite() or not db.is_finite():
    errors.append(label+': nonfinite requires separate explicit class contract');return
   diff=(da-db)*Decimal(gate.get('difference_scale','1'))*Decimal(scale)
   measurements.append({'field':label,'difference':str(diff),'unit':gate['comparison_unit'],'gate':gate['absolute_tolerance']})
   if abs(diff)>Decimal(gate['absolute_tolerance']):errors.append(f'{label}: {diff} {gate["comparison_unit"]}')
 for p,pulsar in enumerate(result['pulsars']):
  meta=state[p,-2]
  for key,value in [('name',pulsar['name']['text']),('units',pulsar['units']['code']),('frame',pulsar['coordinate_frame']['code']),('time_ephemeris',pulsar['time_ephemeris']['code']),('nobs',len(pulsar['observations']))]:exact(meta[key],value,f'p{p}.{key}')
  observations=list(enumerate(pulsar['observations']))
  if pulsar['reference_observation'] is not None:observations.append((-1,pulsar['reference_observation']))
  exact(sorted(i for pp,i in state if pp==p and i!=-2),sorted(i for i,o in observations),f'p{p} membership')
  for i,o in observations:
   ref=state[p,i];label=f'p{p}.obs{i}'
   for key in ['filename','site']:exact(ref[key],o[key]['text'],label+'.'+key)
   for key in ['deleted','clock_correction','delay_correction']:exact(ref[key],o[key],label+'.'+key)
   exact(ref['nFlags'],len(o['flags']),label+'.nFlags')
   for k,flag in enumerate(o['flags']):exact(ref['flag_id_'+str(k)],flag['id']['text'],label+'.flagID');exact(ref['flag_value_'+str(k)],flag['value']['text'],label+'.flagVal')
   exact(ref['nclock'],len(o['clock_steps']),label+'.nclock')
   for k,step in enumerate(o['clock_steps']):exact(ref['clock_to_'+str(k)],step['corrects_to']['text'],label+'.clock_to');numeric(ref['clock_s_'+str(k)],step['seconds'],'clock_step_s_*',label+'.clock_s')
   for key,unit in [('frequency','MHz'),('frequency_ssb','MHz' if o['delay_correction']==0 else 'Hz'),('uncertainty','us')]:exact(o[key]['unit'],unit,label+'.'+key+'.unit')
   expected_units={key:'s' for key in ['tt_to_tb','tt_to_teph','ut1','roemer','shklovskii','troposphere','dispersion_chromatic','solar_wind','shapiro_sun','shapiro_jupiter','shapiro_saturn','shapiro_venus','shapiro_uranus','shapiro_neptune','clock_to_tt']}
   expected_units.update(bat_correction='day',einstein_rate='dimensionless')
   exact(sorted(expected_units),sorted(o['corrections']),label+'.correction membership')
   for key,unit in expected_units.items():exact(o['corrections'][key]['unit'],unit,label+'.'+key+'.unit')
   for key,gate in FIELD.items():
    if key not in ref:continue
    value=o[key] if key in ['sat_mjd','bat_mjd','bbat_mjd'] else o[key]['value'] if key in ['frequency','frequency_ssb','uncertainty'] else o['corrections'][key]['value']
    numeric(ref[key],value,gate,label+'.'+key,1000000 if key=='frequency_ssb' and o['delay_correction']==0 else 1)
 return errors,measurements

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--reuse-native',action='store_true');ap.add_argument('--release',action='store_true');ap.add_argument('--case',action='append');a=ap.parse_args()
 exe=BUILD/'probe' if a.reuse_native else native.build()
 identity=json.loads((BUILD/'identity.json').read_text());assert native.sha(exe.read_bytes())==identity['executable_sha256']
 env={**os.environ,'CARGO_HOME':str(ROOT/'rust/.cargo-home'),'GMP_MPFR_SYS_CACHE':str(ROOT/'rust/target/gmp-cache')}
 subprocess.run(['cargo','build','--manifest-path',str(ROOT/'rust/Cargo.toml'),'--locked','--offline',*(['--release'] if a.release else [])],env=env,check=True)
 rust=ROOT/'rust/target'/('release' if a.release else 'debug')/'tempo2-rust';runtime=ROOT/'validation/reference/build/source/T2runtime'
 fixture=HERE/'fixtures';ordinary=(fixture/'ordinary.par',fixture/'ordinary.tim');ecliptic=(fixture/'ecliptic.par',fixture/'ordinary.tim')
 cases=[('ordinary',[ordinary],[]),('ecliptic',[ecliptic],[]),('mixed_mean_tzr',[ordinary,ecliptic],[]),('mixed_tzr_mean',[ecliptic,ordinary],[])]
 cases.extend((name,[(fixture/(name+'.par'),ordinary[1])],[]) for name in ['dispersion','chromx','dmmodel','dmmodel_numeric','shapelet','secular','fb90'])
 cases.extend([
  ('mixed_tropo_disabled',[ordinary,(fixture/'fb90.par',ordinary[1])],[]),
  ('tempo1',[ordinary],['-tempo1']),
  ('tim_controls',[(fixture/'controls.par',fixture/'controls.tim')],['-select',str(fixture/'select.txt')]),
  ('dm_empty',[ordinary],['-setdm',str(fixture/'dm-empty.txt')]),
  ('dm_table',[ordinary],['-setdm',str(fixture/'dm-table.txt')]),
  ('epoch',[ordinary],['-epoch','55001']),
  ('solarwind',[(fixture/'solarwind.par',fixture/'solarwind.tim')],[])])
 if a.case:
  assert set(a.case)<=set(name for name,_,_ in cases)
  cases=[case for case in cases if case[0] in a.case]
 failed=[]
 for name,pairs,args in cases:
  work=BUILD/name;work.mkdir(exist_ok=True);out=work/'native.tsv'
  case_runtime=runtime
  if name=='solarwind':
   # Deliberately synthetic current sheets exercise the complete file pipeline;
   # their numerical agreement is not calibration of the solar-wind model.
   case_runtime=work/'runtime';case_runtime.mkdir(exist_ok=True)
   for child in runtime.iterdir():
    if child.name=='solarWindModel':continue
    target=case_runtime/child.name
    if not target.exists():target.symlink_to(child,target_is_directory=child.is_dir())
   sheets=case_runtime/'solarWindModel';sheets.mkdir(exist_ok=True)
   for rotation in range(2100,2301):(sheets/f'CS{rotation}.txt').write_bytes((fixture/'synthetic-current-sheet.txt').read_bytes())
  native_command=[str(exe),str(out),str(len(pairs)),*[str(path) for pair in pairs for path in pair],*args]
  native_before=native.sha(exe.read_bytes());assert native_before==identity['executable_sha256']
  out.unlink(missing_ok=True)
  proc=subprocess.run(native_command,capture_output=True,env={**os.environ,'LC_ALL':'C','TEMPO2':str(case_runtime)},timeout=60)
  (work/'native.stdout').write_bytes(proc.stdout);(work/'native.stderr').write_bytes(proc.stderr)
  assert proc.returncode==0,(name,proc.returncode,proc.stderr)
  assert native.sha(exe.read_bytes())==native_before
  native_execution={'command':native_command,'executable_sha256':native_before,'output_sha256':native.sha(out.read_bytes()),'returncode':proc.returncode,'inputs':{str(path):native.sha(path.read_bytes()) for pair in pairs for path in pair}}
  (work/'native-execution.json').write_text(json.dumps(native_execution,indent=2)+'\n')
  cmd=[str(rust),'bats',*[v for pair in pairs for flag,path in zip(['--par','--tim'],pair) for v in (flag,str(path))],'--runtime',str(case_runtime),'--',*args]
  proc=subprocess.run(cmd,capture_output=True,env=env,timeout=60);(work/'rust.stderr').write_bytes(proc.stderr)
  if proc.returncode:failed.append((name,proc.stderr.decode()));print(name,proc.stderr.decode(),flush=True);continue
  result=json.loads(proc.stdout);(work/('rust-release.json' if a.release else 'rust-debug.json')).write_bytes(proc.stdout)
  errors,measurements=compare(out.read_text(),result)
  receipt={'native_execution':native_execution,'native_sha256':native_before,'rust_sha256':native.sha(rust.read_bytes()),'errors':errors,'measurements':measurements,'profile':'release' if a.release else 'debug','comparator_sha256':native.sha(Path(__file__).read_bytes())}
  (work/('comparison-release.json' if a.release else 'comparison-debug.json')).write_text(json.dumps(receipt,indent=2)+'\n')
  print(name,errors[:5] or 'PASS',flush=True)
  if errors:failed.append((name,errors))
 assert not failed,failed
if __name__=='__main__':main()
