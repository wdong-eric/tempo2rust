#!/usr/bin/env python3
"""AL successor to AD check_public.py: unchanged cases/projection/gates, explicit fresh products."""
from pathlib import Path
from decimal import Decimal,localcontext
import argparse,hashlib,importlib.util,json,os,re,subprocess,sys
from common import ROOT, sha
HERE=ROOT/'validation/timing_models'
sys.path.insert(0,str(HERE))
sys.path.insert(0,str(ROOT/'validation/input_to_bat'))
import check_pipeline as ac

AVERAGED_UNITS = {
 'bat':'MJD', 'residual':'s', 'uncertainty':'s',
 'dm_bat':'MJD', 'dm_residual':'s at 1400 MHz', 'dm_uncertainty':'s at 1400 MHz',
}

def compare(tsv,result):
 errors,measurements=ac.compare(tsv,result)
 state={}
 for line in tsv.splitlines():
  p,i,k,v=line.split('\t',3);state.setdefault((int(p),int(i)),{})[k]=v
 with localcontext() as c:
  c.prec=90
  for p,psr in enumerate(result['pulsars']):
   meta=state[p,-2];mutations=psr['parameter_mutations']
   if len(mutations)!=int(meta['mutations']):errors.append(f'p{p} mutation count')
   for j,m in enumerate(mutations):
    prefix=f'mutation{j}.'
    if meta.get(prefix+'label')!=m['label'] or int(meta[prefix+'index'])!=m['index']:errors.append(f'p{p} mutation identity')
    for side in ['before','after']:
     for field in ['value','error','set','fit']:
      aa=Decimal(meta[prefix+side+'.'+field]);raw=m[side][field]
      bb=Decimal(str(raw)) if not isinstance(raw,dict) else Decimal({'nan':'NaN','-nan':'-NaN','+infinity':'Infinity','-infinity':'-Infinity'}[raw['class']])
      if aa.is_finite() and bb.is_finite():
       gate=abs(aa)*Decimal('1e-30') if field in ['value','error'] else Decimal(0)
       if abs(aa-bb)>gate:errors.append(f'p{p} mutation {j} {side}.{field}')
      elif (aa.is_nan(),aa.is_infinite(),aa.is_signed())!=(bb.is_nan(),bb.is_infinite(),bb.is_signed()):errors.append('mutation floating class')
   rows=list(enumerate(psr['observations']))
   if psr['reference_observation'] is not None:rows.append((-1,psr['reference_observation']))
   for i,row in rows:
    ref=state[p,i]
    if i>=0 and row['input_index']!=int(ref['input_index']):errors.append(f'p{p}.{i} input identity')
    averaged=row.get('averaged')
    if not isinstance(averaged,dict):
     errors.append(f'p{p}.{i} averaged projection must be an object');averaged={}
    if set(averaged)!=set(AVERAGED_UNITS):errors.append(f'p{p}.{i} averaged field membership')
    for key,unit in AVERAGED_UNITS.items():
     item=averaged.get(key)
     if not isinstance(item,dict) or set(item)!={'value','unit'}:
      errors.append(f'p{p}.{i} {key} averaged quantity membership');continue
     if item['unit']!=unit:errors.append(f'p{p}.{i} {key} averaged unit')
     refkey='avg_'+key
     if (refkey in ref)!=(item['value'] is not None):errors.append(f'p{p}.{i} {key} averaged membership');continue
     if refkey in ref:
      aa=Decimal(ref[refkey]);raw=item['value']
      if isinstance(raw,dict):raw={'nan':'NaN','-nan':'-NaN','+infinity':'Infinity','-infinity':'-Infinity'}[raw['class']]
      bb=Decimal(raw)
      if not aa.is_finite() or not bb.is_finite():
       if (aa.is_nan(),aa.is_infinite(),aa.is_signed())!=(bb.is_nan(),bb.is_infinite(),bb.is_signed()):errors.append(f'p{p}.{i} {key} averaged floating class')
       continue
      delta=(aa-bb)*(86400 if key in ['bat','dm_bat'] else 1)
      if abs(delta)>Decimal('1e-12'):errors.append(f'p{p}.{i} {key} averaged delta {delta}')
    for k in ['pet_mjd','binary_delay_s','phase_turn','nphase_turn','pulse_number','residual_s','residualtn_s']:
     a=Decimal(ref[k]);raw=row[k]
     if isinstance(raw,dict):raw={'nan':'NaN','-nan':'-NaN','+infinity':'Infinity','-infinity':'-Infinity'}[raw['class']]
     b=Decimal(str(raw));key=f'p{p}.{i}.{k}'
     if not a.is_finite() or not b.is_finite():
      if (a.is_nan(),a.is_infinite(),a.is_signed())!=(b.is_nan(),b.is_infinite(),b.is_signed()):errors.append(key+': floating class differs')
      continue
     delta=(a-b)*(86400 if k=='pet_mjd' else 1)
     gate=Decimal('0' if k in ['pulse_number','nphase_turn'] else '1e-10' if k=='phase_turn' else '1e-12')
     measurements.append({'field':key,'difference':str(delta),'gate':str(gate)})
     if abs(delta)>gate:errors.append(f'{key}: {delta} exceeds {gate}')
 return errors,measurements

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--case',action='append')
 for name in ['native','rust','lifecycle','runtime','out']:
  ap.add_argument('--'+name,type=Path,required=True)
 a=ap.parse_args();a.out=a.out.resolve();a.out.mkdir(parents=True,exist_ok=False)
 exe=a.native.resolve()/'timing';identity=json.loads((a.native/'receipt.json').read_text());assert sha(exe)==identity['executables']['timing']
 rust=a.rust.resolve();runtime=a.runtime.resolve();g=ROOT/'validation/binary/fixtures';f=HERE/'fixtures';acfix=ROOT/'validation/input_to_bat/fixtures'
 cases=[(m,[(g/(m+'_baseline.par'),g/'barycentric.tim')],[]) for m in ['bt','btj','btx','ell1','dd','ddk','dds','mss','ddgr','t2','t2-pta','ddh','ell1h','ell1k']]
 cases += [('pks_dd',[(g/'pks_dd.par',g/'pks.tim')],[]),('t2-companions',[(g/'t2_companions.par',g/'barycentric.tim')],[]),('ordinary',[(acfix/'ordinary.par',acfix/'ordinary.tim')],[]),('ecliptic',[(acfix/'ecliptic.par',acfix/'ordinary.tim')],[])]
 for p in sorted(f.glob('*.par')):
  tim=p.with_suffix('.tim')
  if tim.exists():cases.append((p.stem,[(p,tim)],[]))
 cases += [('two-cut',[(f/'cut-sat.par',f/'cut-sat.tim'),(f/'cut-second.par',f/'cut-bat.tim')],[]),
 ('two-shapelet',[(f/'shapelet.par',f/'shapelet.tim'),(f/'shapelet-second.par',f/'shapelet.tim')],[]),
 ('two-averaging',[(f/'averaging.par',f/'averaging.tim'),(f/'averaging-second.par',f/'averaging.tim')],[]),
 ('two-tzr',[(f/'tzr.par',f/'tzr.tim'),(f/'cut-sat.par',f/'cut-sat.tim')],[]),
 ('tempo1',[(acfix/'ordinary.par',acfix/'ordinary.tim')],['-tempo1']),
 ('deleted-controls',[(acfix/'controls.par',acfix/'controls.tim')],['-select',str(acfix/'select.txt')])]
 lib=f/'library';scenarios={'library-state':('state',lib/'base.par',lib/'switches.txt'),'library-noise':('noise',lib/'noise.par',lib/'signals.txt'),'library-eccentric':('eccentric',lib/'base.par',lib/'eccentric.par'),'eccentric-both':('eccentric',lib/'base.par',lib/'eccentric-both.par'),'eccentric-pulsar':('eccentric',lib/'base.par',lib/'eccentric-pulsar.par'),'eccentric-coalescence':('eccentric',lib/'base.par',lib/'eccentric-coalescence.par')}
 for name,(mode,par,control) in scenarios.items():cases.append((name,[(par,lib/('noise.tim' if mode=='noise' else 'base.tim'))],[]))
 failures=[]
 for name,pairs,args in cases:
  if a.case and name not in a.case:continue
  for mean in [0,1]:
   work=a.out/f'{name}-mean{mean}';work.mkdir(parents=True,exist_ok=True);out=work/'native.tsv';out.unlink(missing_ok=True);(work/'native-identity.json').write_text(json.dumps(identity,indent=2)+'\n')
   cmd=[str(exe),str(out),str(mean),str(len(pairs)),*[str(v) for pair in pairs for v in pair],*args]
   before=sha(exe);env={**os.environ,'TEMPO2':str(runtime),'LC_ALL':'C'}
   if name in scenarios:env.update(AD_SCENARIO=scenarios[name][0],AD_CONTROL=str(scenarios[name][2]))
   (work/'avg.dat').unlink(missing_ok=True)
   n=subprocess.run(cmd,cwd=work,capture_output=True,env=env,timeout=60)
   native_avg=(work/'avg.dat').read_bytes() if (work/'avg.dat').exists() else None
   if native_avg is not None:(work/'native.avg.dat').write_bytes(native_avg)
   (work/'avg.dat').unlink(missing_ok=True)
   assert sha(exe)==before
   (work/'native.stdout').write_bytes(n.stdout);(work/'native.stderr').write_bytes(n.stderr)
   rc=[str(rust),'timing',*[v for pair in pairs for flag,path in zip(['--par','--tim'],pair) for v in (flag,str(path))],'--runtime',str(runtime),*(['--no-mean'] if mean==0 else []),'--',*args]
   if name in scenarios:rc=[str(a.lifecycle.resolve()),scenarios[name][0],str(runtime),str(pairs[0][0]),str(pairs[0][1]),str(scenarios[name][2]),*(['--no-mean'] if mean==0 else [])]
   r=subprocess.run(rc,cwd=work,capture_output=True,timeout=60);(work/'rust.json').write_bytes(r.stdout);(work/'rust.stderr').write_bytes(r.stderr)
   errors=[];measurements=[];consumed={str(p):sha(p) for pair in pairs for p in pair}
   if n.returncode or r.returncode:errors.append(f'process status native={n.returncode}, Rust={r.returncode}: {r.stderr.decode(errors="replace")}')
   else:
    result=json.loads(r.stdout);errors,measurements=compare(out.read_text(),result)
    for item in result['inputs']:
     path=Path(os.fsdecode(bytes(item['path']['bytes'])));assert sha(path)==item['sha256'];consumed[str(path)]=item['sha256']
    def canonical_log(data):
     def location(m):
      name=Path(m[1].decode()).name;line=m[2].decode();mapped=identity['diagnostic_line_maps'].get(name,{}).get(line,int(line))
      return f'[{name}:{mapped}]'.encode()
     return re.sub(rb'\[([^\]\n]+\.C):(\d+)\]',location,data)
    for channel,expected in [('stdout',n.stdout),('stderr',n.stderr)]:
     actual=b''.join(bytes(event['bytes']) for event in result['diagnostics'] if event['source_channel']==channel)
     if canonical_log(expected)!=actual:errors.append(channel+' diagnostic bytes differ after source-location mapping')
   rust_avg=(work/'avg.dat').read_bytes() if (work/'avg.dat').exists() else None
   if native_avg!=rust_avg:errors.append('avg.dat byte content or file membership differs')
   receipt={'native_command':cmd,'rust_command':rc,'native_executable_sha256':before,'rust_executable_sha256':sha(Path(rc[0])),'comparator_sha256':sha(Path(__file__)), 'rust_source_sha256':{str(p.relative_to(ROOT)):sha(p) for p in sorted((ROOT/'rust/src').rglob('*.rs'))},'input_sha256':consumed,'native_identity_sha256':sha(work/'native-identity.json'),'native_returncode':n.returncode,'rust_returncode':r.returncode,'native_output_sha256':sha(out) if out.exists() else None,'rust_output_sha256':hashlib.sha256(r.stdout).hexdigest(),'errors':errors,'measurements':measurements}
   (work/'comparison.json').write_text(json.dumps(receipt,indent=2)+'\n')
   print(name,mean,'PASS' if not errors else f'FAIL {errors[:3]}',flush=True)
   if errors:failures.append((name,mean,errors))
 (a.out/'receipt.json').write_text(json.dumps({'failures':failures,'status':'failure' if failures else 'pass','cases':len(list(a.out.glob('*/comparison.json'))),'native_receipt':sha(a.native/'receipt.json'),'checker':sha(Path(__file__))},indent=2)+'\n')
 return bool(failures)
if __name__=='__main__':raise SystemExit(main())
