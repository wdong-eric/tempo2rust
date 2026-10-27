"""AJ native comparison: unchanged AF quantity gates; ABI/state/classes exact.

The caller has measured the macOS arm64 53-bit ordinary-long-double ABI.
These values are never advertised as a 64-bit Linux comparator profile.
"""
from pathlib import Path
from decimal import Decimal as D
import math
import sys
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/"predictors"))
from compare import phase_bound, hz_bound

def compare_values(actual, expected):
 assert len(actual)==len(expected)
 # AF's frozen source component/phase/statistic gates, with native-derived
 # coefficient scale. Exact ABI, membership, diagnostics and zero/sign classes.
 coeff=[D.from_float(float.fromhex(x.decode().split()[2])) for x in expected if x.startswith(b'V coefficient ')]
 scale=sum(abs(v)*(D('.5') if i%4==0 else 1)*(D('.5') if i//4==0 else 1) for i,v in enumerate(coeff))
 derivative=[D.from_float(float.fromhex(x.decode().split()[2])) for x in expected if x.startswith(b'V derivative-coefficient ')]
 derivative_scale=sum(abs(v)*(D('.5') if i%4==0 else 1)*(D('.5') if i//4==0 else 1) for i,v in enumerate(derivative))
 for aa,ee in zip(actual,expected):
  if aa==ee:continue
  av,ev=aa.decode().split(),ee.decode().split();assert av[:2]==ev[:2] and av[0]=='V'
  key=ev[1];x,y=map(float.fromhex,[av[2],ev[2]])
  assert math.isfinite(x) and math.isfinite(y) and x!=0 and y!=0 and math.copysign(1,x)==math.copysign(1,y)
  # Only these construction-dependent rows permit AF's bounded rounding.
  if key=='coefficient':bound=16*D(2)**-52*4*2*scale
  elif key=='derivative-coefficient':bound=16*D(2)**-52*4*2*scale*32
  elif key in ['rms','mav','polynomial','copy','model-phase','model-exchange','set-phase','plan-phase']:bound=phase_bound(scale+ D(8)/D(1000)**2)
  elif key=='derivative':bound=phase_bound(scale)*2*4*4
  elif key in ['model-frequency','set-frequency']:bound=hz_bound(derivative_scale*2/86400)
  elif key=='plan-frequency':
   # The three requested intervals are 300,300,and the final shortened span;
   # using its shortest measured duration is the conservative AF endpoint rule.
   elapsed=D.from_float((58000.01-(58000+600/86400))*86400)
   bound=2*phase_bound(scale+D(8)/D(1000)**2)/elapsed+8*D(2)**-52*abs(D.from_float(y))
  else:raise AssertionError(('unexpected nonexact row',aa,ee))
  assert abs(D.from_float(x)-D.from_float(y))<=bound,(key,x,y,str(bound))

def compare_storage(sr, nr):
 assert len(sr)==len(nr)
 for aa,ee in zip(sr,nr):
  if aa==ee:continue
  x,y=aa.split('\t'),ee.split('\t')
  assert x[0]=='EVAL' and x[:5]==y[:5] and x[6:]==y[6:]
  count,c,dm,offset=map(int,y[1:5]);dt=D(offset)/1024*1440
  coeff=[D(i+1)/16 for i in range(32)];coeff[0]=D('.25');coeff[1]=D([0,15,30][c]);coeff[2]=D('.5')
  scale=D(7)+abs(dt*60*100)+abs(coeff[0])+sum(abs(coeff[i]*dt**i) for i in range(1,count))
  if dm:scale+=abs(D.from_float(float.fromhex(y[6])))*8/D('2.41e-4')*(1/D(700)**2-1/D(1400)**2)
  av,ev=map(float.fromhex,[x[5],y[5]])
  assert math.isfinite(av) and math.isfinite(ev) and av!=0 and ev!=0 and math.copysign(1,av)==math.copysign(1,ev)
  assert abs(D.from_float(av)-D.from_float(ev))<=phase_bound(scale)

def mutation_controls(values, storage):
 """Reject consequential corruptions of otherwise passing real native traces."""
 checks=[]
 def reject(name, fn, changed, expected):
  try: fn(changed, expected)
  except (AssertionError, ValueError): checks.append(name)
  else: raise AssertionError('false acceptance: '+name)
 compare_values(values, values); compare_storage(storage, storage)
 reject('missing-value-row',compare_values,values[:-1],values)
 changed=values.copy();changed[0],changed[1]=changed[1],changed[0]
 reject('value-order',compare_values,changed,values)
 changed=values.copy();changed[0]+=b' altered'
 reject('ABI-metadata',compare_values,changed,values)
 index=next(i for i,v in enumerate(values) if v.startswith(b'V model-frequency '))
 original=float.fromhex(values[index].decode().split()[2])
 for name,number in [('finite-sign',-original),('zero',0.),('nan',float('nan')),('infinity',float('inf')),('finite-magnitude',original+1e6)]:
  changed=values.copy();changed[index]=b'V model-frequency '+float.hex(number).encode()
  reject(name,compare_values,changed,values)
 reject('missing-storage-row',compare_storage,storage[:-1],storage)
 index=next(i for i,v in enumerate(storage) if v.startswith('EVAL'))
 for name,column,text in [('declared-count',1,'31'),('phase',5,'0x1p+1000'),('frequency',6,'0x1p+1000')]:
  changed=storage.copy();fields=changed[index].split('\t');fields[column]=text;changed[index]='\t'.join(fields)
  reject(name,compare_storage,changed,storage)
 return checks
