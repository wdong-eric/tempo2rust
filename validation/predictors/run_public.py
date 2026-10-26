"""AF PAR-only generation, retained model/grid and both cross-reader directions.
All generated artifacts go to a new output directory. No accepted evidence writes.
"""
from pathlib import Path
import argparse,subprocess,json,hashlib,os,shutil
from decimal import Decimal,getcontext
from compare import read_tsv, Checks, constructed, phase_bound, hz_bound, cheby_scale, polyco_scale, evaluation_scale, native_models, plan, serialization
getcontext().prec=70
D=Decimal;EPS=D(2)**-52
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
CASES=[('isolated-bary','isolated','@',[]),('binary-bary','binary','@',[]),('isolated-pks','isolated','PKS',[]),('binary-pks','binary','PKS',[]),('tempo1-pks','tempo1','PKS',['-tempo1']),('ecliptic-tdb','ecliptic_tdb','PKS',[]),('defaults-odd','defaults','PKS',[])]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def run(cmd,cwd,env,name):
 with (cwd/(name+'.stdout')).open('w') as out,(cwd/(name+'.stderr')).open('w') as err:
  p=subprocess.run(list(map(str,cmd)),cwd=cwd,env=env,stdout=out,stderr=err)
 return {'command':list(map(str,cmd)),'returncode':p.returncode,'stdout':name+'.stdout','stderr':name+'.stderr'}
def main():
 p=argparse.ArgumentParser();p.add_argument('--native',type=Path,required=True);p.add_argument('--rust',type=Path,required=True);p.add_argument('--out',type=Path,required=True);p.add_argument('--case');a=p.parse_args();native=a.native.resolve();rust=a.rust.resolve();out=a.out.resolve();out.mkdir(parents=True,exist_ok=False)
 shutil.copy2(native,out/'native.snapshot');shutil.copy2(rust,out/'rust.snapshot');native=out/'native.snapshot';rust=out/'rust.snapshot'
 runtime=ROOT/'validation/reference/build/source/T2runtime';env=dict(os.environ,TEMPO2=str(runtime));receipt={'native':sha(native),'rust':sha(rust),'contract':sha(HERE/'CONTRACT.md'),'sources':{str(p.relative_to(ROOT)):sha(p) for p in [*HERE.glob('*.py'),*HERE.glob('*.cc'),*HERE.glob('fixtures/*.par'),* (ROOT/'rust/src').glob('*predictor*.rs'),*(ROOT/'rust/src').glob('*polyco*.rs'),*(ROOT/'rust/src').glob('cheby*.rs')]},'cases':[]}
 for name,model,site,pre in CASES:
  if a.case and name!=a.case:continue
  for kind in ['cheby','polyco']:
   row={'name':name,'kind':kind,'failures':[],'checks':0,'commands':[]};receipt['cases'].append(row);case=out/(name+'-'+kind);nr=case/'native';rr=case/'rust';nr.mkdir(parents=True);rr.mkdir();par=HERE/'fixtures'/ (model+'.par')
   start='58000';end='58000.1' if kind=='cheby' else '58000';extras=['--freq-start-mhz','1000','--freq-end-mhz','2000','--time-coeffs','8','--freq-coeffs','4','--segment-seconds','3600'] if kind=='cheby' else ['--span-minutes','60','--coefficients','12','--max-hour-angle','1','--frequency-mhz','1400']
   n_extra=['1000','2000','8','4','3600','unused'] if kind=='cheby' else ['60','12','1','1400','1','']
   if name=='defaults-odd' and kind=='cheby':extras[5]='7';extras[7]='3';n_extra[2]='7';n_extra[3]='3'
   commands=[(native,kind,nr/'result.tsv',par,site,start,end,*n_extra,*pre),(rust,'predictor',kind,'--par',par,'--runtime',runtime,'--site',site,'--start-mjd',start,'--end-mjd',end,*extras,'--output-dir',rr,'--',*pre)]
   for cmd,cwd,tag in [(commands[0],nr,'generate'),(commands[1],rr,'generate')]:row['commands'].append(run(cmd,cwd,env,tag))
   if any(r['returncode'] for r in row['commands']):row['failures'].append({'execution':row['commands']});continue
   data=json.loads((rr/'generate.stdout').read_text());ref=read_tsv(nr/'result.tsv');result=data['result']
   checks=constructed(kind,result,ref)
   compare=checks.number
   observations=data['observations'];native_n=sum(k=='sat' for k,i in ref);compare('observation_count',len(observations),native_n,0)
   for i,o in enumerate(observations):
    for key,field,unit in [('sat','sat_mjd',86400),('bat','bat_mjd',86400),('phase','phase_turn',1),('freq','frequency_mhz',1)]:compare(f'{key}[{i}]',D(str(o[field]))*unit,ref[key,i]*unit,'1e-10' if key=='phase' else '1e-12' if key in ('sat','bat') else 0)
   if kind=='cheby':
    models=result['predictor']['Cheby']['segments']
    row['approximation']={'rust':result['source_test'],'native':{k:str(ref[k,0]) for k in ['rms','mav']},'f0_hz':result['f0_hz'],'target_seconds':'1e-6'}
    for owner,mav in [('rust',result['source_test']['mav_turn']),('native',ref['mav',0])]:
     row['approximation'][owner+'_passes']=abs(D(mav)/D(result['f0_hz']))<D('1e-6')
     if not row['approximation'][owner+'_passes']:row['failures'].append({'approximation':owner,'target_seconds':'1e-6'})
    for key,field in [('rms','rms_turn'),('mav','mav_turn')]:compare('source_test.'+key,result['source_test'][field],ref[key,0],phase_bound(max(cheby_scale(m) for m in models)))
    files=['t2pred.dat'];times=['58000.001','58000.0375','58000.05','58000.099','58000.11'];freqs=['1100','1500','1900']
   else:
    models=result['models']['segments']
    files=['polyco_new.dat','newpolyco.dat'];mid=D(models[0]['mjd_mid']);times=[str(mid+D(v)/1440) for v in ['-20','0','17']];freqs=['1200','1400','1800']
   for filename in files:
    for owner,folder in [('native',nr),('rust',rr)]:
     file=folder/filename
     inspect_tag=f'inspect-{filename}-{owner}'
     row['commands'].append(run([native,'inspect',case/(inspect_tag+'.tsv'),file],case,env,inspect_tag))
     for k,(t,f) in enumerate((t,f) for t in times for f in freqs):
      tag=f'exchange-{filename}-{owner}-{k}';rn=run([native,'evaluate',case/(tag+'.tsv'),file,t,f],case,env,tag+'-native');rrun=run([rust,'predictor','evaluate','--input',file,'--mjd',t,'--frequency-mhz',f],case,env,tag+'-rust');row['commands'].extend([rn,rrun])
      if rn['returncode'] or rrun['returncode']:row['failures'].append({'exchange':tag,'native':rn['returncode'],'rust':rrun['returncode']});continue
      nrvalues=read_tsv(case/(tag+'.tsv'));readback=json.loads((case/(tag+'-rust.stdout')).read_text());rv=readback['prediction'];parsed=readback['input']['predictor'];key='Cheby' if kind=='cheby' else 'T1';checks.exact(tag+'.kind',list(parsed),[key])
      parsed_models=parsed[key]['segments']
      if k==0:
       inspection=read_tsv(case/(inspect_tag+'.tsv'));reread=constructed(kind,{'predictor':parsed} if kind=='cheby' else {'models':parsed['T1']},inspection);checks.count+=reread.count;checks.failures.extend(dict(v,field='reread.'+v['field']) for v in reread.failures)
       if owner=='rust':
        sc=serialization(kind,models,parsed_models,filename);checks.count+=sc.count;checks.failures.extend(sc.failures)
      selected=int(nrvalues['selected',0]);scale=(D(1),D(1)) if selected<0 else evaluation_scale(kind,parsed_models[selected],t,f)
      compare(tag+'.phase',rv['phase_turn'],nrvalues['phase',0],phase_bound(scale[0]));compare(tag+'.Hz',rv['frequency_hz'],nrvalues['frequency',0],hz_bound(scale[1]));compare(tag+'.range',int(rv['out_of_range']),nrvalues['out_of_range',0],0);checks.exact(tag+'.selected',rv['segment'] if rv['segment'] is not None else -1,selected)
     tag=f'plan-{filename}-{owner}';lo,hi=times[0],times[-2] if kind=='cheby' else times[-1];step='1000';freq='1400'
     rn=run([native,'plan',case/(tag+'.tsv'),file,lo,hi,step,freq],case,env,tag+'-native');rrun=run([rust,'predictor','plan','--input',file,'--start-mjd',lo,'--end-mjd',hi,'--step-seconds',step,'--frequency-mhz',freq],case,env,tag+'-rust');row['commands'].extend([rn,rrun])
     if rn['returncode'] or rrun['returncode']:row['failures'].append({'plan':tag,'native':rn['returncode'],'rust':rrun['returncode']});continue
     np=read_tsv(case/(tag+'.tsv'));rp=json.loads((case/(tag+'-rust.stdout')).read_text());pc=plan(kind,rp,np,native_models(kind,inspection),freq);checks.count+=pc.count;checks.failures.extend(dict(v,field=tag+'.'+v['field']) for v in pc.failures)
   row['checks']=checks.count;row['failures'].extend(checks.failures)
   row['files']={str(p.relative_to(case)):sha(p) for folder in [nr,rr] for p in folder.iterdir() if p.is_file()}
   (out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');print(name,kind,'checks',row['checks'],'failures',len(row['failures']),flush=True)
 (out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');return any(r['failures'] for r in receipt['cases'])
if __name__=='__main__':raise SystemExit(main())
