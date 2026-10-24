"""Exact original-to-corrected deltas, with each of the three fixes isolated."""
import gzip,json,hashlib
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];V=ROOT/'validation/time_ephemeris';OUT=HERE/'outputs/native'
def load(p):return json.loads(gzip.decompress(p.read_bytes()))
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def observable(raw,old=False):
 events=[json.loads(l) for l in raw['events'].splitlines()];assert events[0].pop('nonce')==raw['nonce']
 out={k:raw[k] for k in ['id','stdout','stderr','status','final_files']}|dict(events=events)
 if old:
  before=str(V/'build/cases');after=str(HERE/'work/x');assert len(before)==len(after)
  for key in ['stdout','stderr']:
   out[key]=bytes.fromhex(out[key]).replace(before.encode(),after.encode()).hex()
  for e in events:
   if 'state' not in e:continue
   s=e['state'];s['root']=s['root'].replace(before,after)
   s['filename']['hex']=bytes.fromhex(s['filename']['hex']).replace(before.encode(),after.encode()).hex()
 return out

def differences(a,b,path='$'):
 assert type(a)is type(b),(path,'type')
 if isinstance(a,dict):
  assert list(a)==list(b),(path,'ordered membership')
  return [z for k in a for z in differences(a[k],b[k],path+'.'+k)]
 if isinstance(a,list):
  assert len(a)==len(b),(path,'length')
  return [z for i,(v,w)in enumerate(zip(a,b))for z in differences(v,w,path+f'[{i}]')]
 return [] if a==b else [dict(path=path,original=a,corrected=b)]

def main():
 receipts={name:json.loads((OUT/(name+'-receipt.json')).read_text())for name in ['twot','twot_fb','corrected']}
 ids=[r['id'] for r in receipts['corrected']['records']];assert len(ids)==97
 changes={name:[]for name in receipts};total=[]
 for case in ids:
  original=observable(load(V/'outputs/reference-corrected'/(case+'.json.gz')),True);before=original
  for name,subdir in [('twot','twot'),('twot_fb','twot_fb'),('corrected','equalpaths')]:
   path=OUT/subdir/(case+'.json.gz');record=next(r for r in receipts[name]['records'] if r['id']==case);assert sha(path)==record['sha256']
   after=observable(load(path));delta=differences(before,after)
   # Correction experiments preserve all input, stream, lifetime and event
   # membership. Only source-written numeric cache/output leaves may change.
   for d in delta:
    assert '.state.'in d['path'] and any(tag in d['path'] for tag in ['.ifte.pc[','.ifte.vc[','.ifte.twot.','.output[','.correctionTT_TB.','.correctionTT_Teph.','.einsteinRate.']),d
   if delta:changes[name].append(dict(id=case,changes=delta))
   before=after
  delta=differences(original,before)
  if delta:total.append(dict(id=case,changes=delta))
 assert all(changes.values())
 (OUT/'correction-deltas.json').write_text(json.dumps(dict(cases=97,original_events=1368,correction_steps=changes,combined=total,original_oracle_sha256=sha(V/'outputs/reference-corrected/oracle.json'),receipts={n:sha(OUT/(n+'-receipt.json'))for n in receipts}),indent=2)+'\n')
 for name,cases in changes.items():print(name,len(cases),'cases',sum(len(c['changes'])for c in cases),'changed leaves',[c['id']for c in cases])
if __name__=='__main__':main()
