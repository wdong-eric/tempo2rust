"""Strict AG typed projection comparator. Unknown units, keys and duplicate rows fail."""
from decimal import Decimal,localcontext
from pathlib import Path
import math,struct,json

def read(path):
 rows={}
 for line in Path(path).read_text().splitlines():
  key,unit,value=line.split('\t')
  if key in rows:raise ValueError('duplicate key '+key)
  rows[key]=(unit,value)
 return rows

def compare(native,rust):
 a=read(native);b=read(rust);failures=[];maxima={};checks=0
 if a.keys()!=b.keys():failures.append({'membership':{'missing':sorted(a.keys()-b.keys()),'extra':sorted(b.keys()-a.keys())}})
 scales={}
 for key,(unit,value) in a.items():
  if unit=='text':continue
  d=Decimal(value);group=(key.rsplit('.',1)[0],unit)
  if d.is_finite():scales[group]=max(scales.get(group,Decimal(0)),abs(d))
 with localcontext() as ctx:
  ctx.prec=70
  for key in a.keys()&b.keys():
   checks+=1;unit,sv=a[key];ru,rv=b[key];reason=None
   if unit!=ru:reason='unit'
   elif unit=='text':
    if sv!=rv:reason='text'
   else:
    x,y=Decimal(sv),Decimal(rv)
    if x.is_nan() or y.is_nan():
     if not(x.is_nan() and y.is_nan() and x.is_signed()==y.is_signed()):reason='nan class/sign'
    elif x.is_infinite() or y.is_infinite():
     if x!=y:reason='infinity'
    elif x==0 or y==0:
     if x!=y or x.is_signed()!=y.is_signed():reason='zero class/sign'
    elif unit in ('exact','bits','class'):
     if x!=y:reason='exact'
    elif unit in ('uniform','f64_exact','f32_exact'):
     fmt='f' if unit=='f32_exact' else 'd'
     if struct.pack(fmt,float(x))!=struct.pack(fmt,float(y)):reason='source-width bits'
    else:
     scale=scales[(key.rsplit('.',1)[0],unit)]
     if unit=='gaussian':budget=Decimal('5e-14')
     elif unit=='seconds':budget=Decimal('1e-12')
     elif unit=='frequency_day':budget=Decimal('5e-14')*abs(x)
     elif unit=='log_sigma':budget=Decimal('5e-14')*max(Decimal(1),abs(x))
     elif unit=='f32':
      f=float(x);bits=struct.unpack('I',struct.pack('f',abs(f)))[0];nextf=struct.unpack('f',struct.pack('I',bits+1))[0];budget=Decimal.from_float(2*(nextf-abs(f)))
     elif unit=='stat':budget=Decimal('5e-14')*Decimal('5.25')**(2 if key.endswith('variance') else 1)
     elif unit in ('days','coefficient_s','s','s^2','yr^3','yr^(3/2)','sqrt_year3','dimensionless','yr^3 * yr / s^2'):budget=Decimal('5e-14')*scale
     elif unit in ('radians','geometry'):budget=Decimal('5e-14')
     elif unit in ('angular_frequency','amplitude','history'):budget=Decimal('5e-14')*abs(x)
     else:raise ValueError('unregistered unit '+unit)
     delta=abs(x-y);maxima[unit]=max(maxima.get(unit,Decimal(0)),delta)
     if delta>budget:reason=f'delta {delta} > {budget}'
   if reason:failures.append({'key':key,'unit':unit,'native':sv,'rust':rv,'reason':reason})
 return {'checks':checks,'failures':failures,'maxima':{k:str(v)for k,v in maxima.items()}}
if __name__=='__main__':
 import sys
 r=compare(sys.argv[1],sys.argv[2]);print(json.dumps(r,indent=2));sys.exit(bool(r['failures']))
