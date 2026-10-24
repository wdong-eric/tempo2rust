"""Independent limits, recurrence, calendar, normalization and state checks."""
import datetime
import math
import struct
from inventory import HERE,save,sha,check
from run import OUTPUT,read_gzip,frozen_cases,compare
from combined import load

def value(x):
 if x in ['NaN','+Inf','-Inf']:return {'NaN':math.nan,'+Inf':math.inf,'-Inf':-math.inf}[x]
 raw=bytes.fromhex(x)
 if len(raw)==16:
  bits=int.from_bytes(raw,'little');exponent=(bits>>112)&32767
  significand=(bits&((1<<112)-1))|((1<<112) if exponent else 0)
  return math.ldexp(float(significand),(exponent or 1)-16383-112)*(-1 if bits>>127 else 1)
 return struct.unpack('<d',raw)[0]
def states(result):
 fields={}
 for e in result['events']:
  fields.update(e['state']['changes'])
  if e['phase']=='end':yield e,fields.copy()
def cell(fields,path,index=0,kind='d'):
 record=fields[path];at=0
 for count,v in record['runs']:
  if at<=index<at+count:return value(v) if kind=='d' else struct.unpack('<'+kind,bytes.fromhex(v))[0]
  at+=count
 raise IndexError(path)
def check_record(record):
 if list(record)==['owner']:
  assert isinstance(record['owner'],str);return
 if list(record)==['width','count','runs']:
  count=record['count'];width=record['width'];assert type(width) is int and width in [1,4,8,16]
 elif list(record)==['pointer_count','runs']:count=record['pointer_count'];width=None
 else:
  assert list(record)==['parameter_count','runs'];count=record['parameter_count'];width='parameter'
 assert type(count) is int and count>=0
 assert sum(r[0] for r in record['runs'])==count
 for n,v in record['runs']:
  assert type(n) is int and n>0
  if width=='parameter':
   assert list(v)==['aSize','err','fitFlag','label','linkFrom','linkTo','nLinkFrom','nLinkTo','paramSet','prefit','prefitErr','shortlabel','val']
   for cell in v.values():check_record(cell)
  elif width is None:assert isinstance(v,str)
  else:
   assert isinstance(v,str)
   assert v in ['NaN','+Inf','-Inf'] or len(bytes.fromhex(v))==width

def main():
 check();catalog,results=load();compare([c['id'] for c in catalog],[r['id'] for r in results]);by={r['id']:r for r in results};cases={c['id']:c for c in catalog};checks=[]
 for result in results:
  for event in result['events']:
   for path,record in event['state']['changes']:check_record(record)
 checks.append(dict(name='complete_typed_RLE_structure_and_extents',exact=True))
 def ends(name,op):return [(e,s) for e,s in states(by[name]) if e['op']==op]
 def exact(name,condition):assert condition,name;checks.append(dict(name=name,exact=True))
 def near(name,actual,expected,bound):
  d=abs(actual-expected);assert d<=bound,(name,actual,expected,d,bound);checks.append(dict(name=name,absolute_difference=d,supporting_bound=bound))
 for n in [0,1,2,10,170]:near('factorial_'+str(n),value(ends('factorial_'+str(n),'factorial')[0][0]['result'][0]),float(math.factorial(n)),abs(float(math.factorial(n)))*3e-15)
 exact('factorial_overflow',math.isinf(value(ends('factorial_171','factorial')[0][0]['result'][0])))
 for n in [-2,0,1,2]:
  v=list(map(value,ends('hermite_'+str(n),'hermite')[0][0]['result']));exact('hermite_unconditional_pair_'+str(n),v[:2]==[1,1.4] and v[2:]==list(range(93,103)))
 x=.7;expected=[1,2*x]
 y0,y1=expected
 for n in range(2,8):
  # Source polynomial normalization rescales both preceding states at each step.
  y0/=math.sqrt(n);y1/=math.sqrt(n);yn=2*x*y1-2*(n-1)*y0;expected.append(yn);y0,y1=y1,yn
 actual=list(map(value,ends('hermite_8','hermite')[0][0]['result']))[:8]
 for i,(a,b) in enumerate(zip(actual,expected)):near('Hermite_recurrence_'+str(i),a,b,1e-14)
 for n in [-1,0]:exact('zero_coeff_shapelet_'+str(n),value(ends(f'shapelet_{n}_2','shapelet')[0][0]['result'][0])==0)
 expected=math.exp(-.5*(.5/2)**2)/(2*math.pi)**.25
 near('shapelet_single_gaussian',value(ends('shapelet_1_2','shapelet')[0][0]['result'][0]),expected,2e-16)
 exact('sinc_origin',value(ends('sinc_zero','sinc')[0][0]['result'][0])==1)
 for t,e in [(-1,1),(0,1),(.5,2),(1,3),(2.5,1.5),(5,9),(6,9)]:near('linear_ifunc_'+str(t),value(ends('ifunc_'+str(t),'ifunc')[0][0]['result'][0]),e,0)
 exact('negative_basis_zero',value(ends('basis_negative','basis')[0][0]['result'][0])==0)
 for name,c in cases.items():
  if name.startswith('calendar_'):
   for e,s in ends(name,'date'):
    mjd=int(c['actions'][e['event']][1]);date=datetime.date(1858,11,17)+datetime.timedelta(days=mjd)
    exact(name+'_'+str(mjd),e['result']==[date.year-1900,date.year,date.month,date.day,date.timetuple().tm_yday])
  if name.startswith('amod_'):
   a,b=map(float,c['actions'][-1][1:]);expected=0 if b==0 else a-math.trunc(a/b)*b
   near(name+'_truncation',value(ends(name,'amod')[0][0]['result'][0]),expected,0)
  if name.startswith('mcl_'):
   e=ends(name,'mcl')[0][0];v=e['result'];n=v[0];exact(name+'_bounded_count',1<=n<=36)
   for i in range(3):exact(name+'_tail_'+str(i),all(x==v[1+36*i+n-1] for x in v[1+36*i+n:1+36*(i+1)]))
 for name in ['solar_model_0_grid','solar_model_0_polar','solar_model_1_grid','solar_model_1_polar']:
  import re
  matches=re.findall(r'Integral = \S+ (\S+) (\S+) (\d+) (\d+) (\S+) (\S+)',by[name]['stdout'])
  exact(name+'_two_executions',len(matches)==2)
  for i,(integral,dm,slow,fast,elong,deg) in enumerate(matches):near(name+'_density_sum_'+str(i),float(integral),10*int(slow)+2.5*int(fast),0)
 for name in ['shapiro_A_1','shapiro_STL_FBAT_1']:
  e,s=ends(name,'shapiro')[0];norm=sum(cell(s,'obs0.psrPos',k)**2 for k in range(3));near(name+'_unit_direction',norm,1,3e-16)
 e,s=ends('bclt_A_0','bclt')[0];exact('bypass_frequency_MHz',cell(s,'obs0.freqSSB')==1400);exact('bypass_roemer',cell(s,'obs0.roemer')==0)
 e,s=ends('shapiro_bypass','shapiro')[0];exact('shapiro_retained_saturn',cell(s,'obs0.shapiroDelaySaturn')==.007)
 e,s=ends('bclt_tropo_0','bclt')[0];exact('first_owner_tropo_write',cell(s,'obs1.troposphericDelay')==0 and cell(s,'obs7.troposphericDelay')!=0)
 for name,counts in [('phase_cross_0_1',[1,2]),('phase_cross_1_0',[1,0]),('phase_cross_1_1',[1,1])]:
  e,s=ends(name,'clean')[0];exact(name+'_counts',[cell(s,'p0.nobs',kind='i'),cell(s,'p1.nobs',kind='i')]==counts)
 for series in [0,1,2]:
  for order in range(1,9):
   name=f'derivative_{series}_{order}';e,fields=ends(name,'dm')[0]
   freq=cell(fields,'obs0.freqSSB');years=365.5/365.25;factor=math.factorial(order) if series==2 else 1
   dm=10+.01*years**order/factor;cm=.0002+.001*years**order/factor
   # Independent dimensional decomposition: dispersion in seconds plus chromatic power law.
   from inventory import blob
   import re
   constant=float(re.search(r'#define DM_CONST\s+([0-9.eE+-]+)',blob('tempo2.h').decode())[1])
   expected=dm/constant*1e12/freq**2+cm*(freq/1.4e9)**(-3.5)
   near(name+'_dispersion_chromatic',cell(fields,'obs0.tdis1'),expected,abs(expected)*3e-15)
 phase=ends('phase_A','init')[0][1];copied=ends('phase_A','clean')[0][1]
 copied_paths=[k for k in phase if k.startswith('obs1.')]
 exact('phase_full_typed_copy',all(phase[k]==copied['p0.tzrobs.'+k[5:]] for k in copied_paths))
 for site,flags in [('@',(0,0)),('BaT',(0,0)),('STL',(0,1)),('STL_FBAT',(0,1)),('STL_BAT',(1,1)),('A',(1,1))]:
  e,fields=ends('phase_'+site,'init')[0]
  exact('phase_site_'+site,tuple(cell(fields,'obs1.'+f,kind='i') for f in ['clockCorr','delayCorr'])==flags)
 for name in ['precision_noop']:
  exact('empty_precision_body',all(not e['state']['changes'] for e in by[name]['events'] if e['phase']=='end' and e['op']=='precision'))
 check();save(OUTPUT/'independent.json',dict(status='PASS',script_sha256=sha((HERE/'independent.py').read_bytes()),checks=checks,oracle_sha256=sha((OUTPUT/'reference.json.gz').read_bytes())))
 print('PASS',len(checks),'independent supporting checks')
if __name__=='__main__':main()
