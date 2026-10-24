"""Coverage-directed authored cases; a separate original-source oracle preserves the first freeze."""
import argparse
import copy
import datetime
import json
from inventory import HERE,save,sha,check
from fixtures import cases as primary_cases,p,o,par,table,resources,solar_files,validate_inputs
from profile import compile_probe,stable,actual_identity
from run import OUTPUT,execute,validate,compare,write_gzip,read_gzip,packed,frozen_cases as primary_frozen,harness
DEST=OUTPUT/'supplemental'

def cases():
 result=[]
 def add(name,actions,files=None,status=0,classification='defined-numerics-host-diagnostics',timeout=30):
  result.append(dict(id='supp_'+name,classification=classification,status=status,timeout=timeout,files=files or {},actions=[[str(v) for v in a] for a in [['profile'],['env','TEMPO2','{case}/root'],*actions]]))
 add('dilation_disabled',[p(0,'dilateFreq',0),['dm',0,0,0,0]])
 add('secular_no_motion_flags',[par(0,'dshk',1),['secular',1]])
 add('diagnostic_paths',[['debug',1,1,0],p(0,'refphs',1),['init',1],['clean',1],p(0,'correctTroposphere',1),['bclt',1]],resources())
 add('solar_cvs',[['debug',0,1,0],['solar',0,0]],solar_files('grid'))
 tv=[0,1,1,3,2,-2,3,5,4,7,5,9]
 add('basis_debug',[['debug',1,0,0],['basis',.5,6,2,*tv]])
 add('ifunc_nan_assert',[['ifunc','nan',6,0,*tv]],status=-6,classification='defined-assertion')
 add('error_storage_full',[o(0,'telID','STL_FBAT'),*[['dm',0,0,0,0] for _ in range(11)],par(0,'dm',10),par(0,'dmmodel',12),p(0,'dmoffsDMnum',1),*table(0,'dmoffsDM',[58000],[1]),['dm',0,0,0,0],['dm',0,0,0,0]])
 limit=next(c for c in primary_cases() if c['id']=='iteration_limit')
 add('warning_storage_full',limit['actions'][2:-1]+[['bclt',1] for _ in range(31)],timeout=180)
 add('output_positive_wrap',[['output',0,1,0,.01]])
 # Longitude is the frozen real elsun2 result for this exact epoch. Source
 # cos(ya-eclon)==1 makes dp=0 and the 36-cell loop reaches its defined exit.
 add('mcl_conjunction_exit',[['mcl','2.824104059214239',0,117,247,'12345.678901',400]],status=1)
 add('mcl_year_exit',[['mcl',1,.3,0,1,0,400]],status=1)
 add('calendar_century',[['date',(datetime.date(1900,m,15)-datetime.date(1858,11,17)).days] for m in range(1,13)],classification='defined-direct-helper-outside-solar-year-range')
 add('mcl_rotations',[['mcl',lon,lat,year,day,43200,400] for lon,lat,year,day in [(0,0,1,1),(.5,.7,50,100),(1,-.7,99,200),(2,0,100,365),(3,.7,150,1),(4,-.7,199,100),(5,0,1,200),(6,.7,199,365)]])
 return result

def frozen_cases():
 primary_frozen();catalog=cases();compare(json.loads((DEST/'cases.json').read_text()),catalog,'supplemental cases')
 stored=json.loads((DEST/'manifest.json').read_text());assert stored['script_sha256']==sha((HERE/'supplemental.py').read_bytes())
 compare(stored['harness'],harness(),'supplemental core harness')
 return catalog

def main():
 parser=argparse.ArgumentParser();parser.add_argument('--record',action='store_true');args=parser.parse_args();check();primary_frozen()
 catalog=cases();oracle=DEST/'reference.json.gz';mp=DEST/'manifest.json'
 if args.record:
  assert not oracle.exists() and not mp.exists(),'supplement never overwrites an oracle'
  if (DEST/'cases.json').exists():compare(json.loads((DEST/'cases.json').read_text()),catalog)
  else:save(DEST/'cases.json',catalog)
 else:catalog=frozen_cases()
 for c in catalog:validate_inputs(c)
 exe,pf=compile_probe('supplemental')
 for path in [HERE/'supplemental.py',DEST/'cases.json']:pf['dependencies'][str(path)]=sha(path.read_bytes())
 save(exe.parent/'profile.json',pf)
 manifest=dict(script_sha256=sha((HERE/'supplemental.py').read_bytes()),harness=harness(),cases_sha256=sha(packed(catalog)),native=stable(pf))
 baseline=None
 if not args.record:
  compare(json.loads(mp.read_text()),manifest,'supplemental manifest');baseline=read_gzip(oracle)
  compare([r['id'] for r in baseline],[c['id'] for c in catalog]);before_hash=sha(oracle.read_bytes())
 results=[];receipts=[]
 for i,c in enumerate(catalog):
  r,first=execute(c,exe,pf,DEST/('record_raw' if args.record else 'check_raw'));validate(c,r)
  replay,second=execute(c,exe,pf,DEST/'replay_raw');validate(c,replay);compare(r,replay,c['id']+'.fresh')
  if baseline is not None:compare(baseline[i],r,c['id'])
  results.append(r);receipts.append(dict(first=first,replay=second));print('PASS',c['id'],flush=True)
 if args.record:write_gzip(oracle,results);save(mp,manifest)
 else:assert sha(oracle.read_bytes())==before_hash
 check();actual_identity(exe,pf);write_gzip(DEST/'execution_receipts.json.gz',receipts)
 save(DEST/'last_execution.json',dict(status='PASS',mode='record' if args.record else 'check',cases=len(catalog),events=sum(len(r['events']) for r in results),profile=pf,oracle_sha256=sha(oracle.read_bytes())))
if __name__=='__main__':main()
