"""Independent rational polynomial/RNG and source-order caller-contract diagnostics."""
from decimal import Decimal,localcontext
from fractions import Fraction as F
import json
import math
import struct
from inventory import save,check,sha
from run import OUTPUT,read_gzip,compare


def f64(x):
    if x['class']=='NaN':return math.nan
    if x['class']=='+Infinity':return math.inf
    if x['class']=='-Infinity':return -math.inf
    return struct.unpack('>d',bytes.fromhex(x['bits']))[0]
def power2(e):return F(2**e) if e>=0 else F(1,2**(-e))
def qround(x):
    x=F(x)
    if not x:return F(0)
    sign=-1 if x<0 else 1;x=abs(x);n,d=x.numerator,x.denominator;e=n.bit_length()-d.bit_length()
    if x<power2(e):e-=1
    scale=e-112;y=x/power2(scale);m,rem=divmod(y.numerator,y.denominator)
    if 2*rem>y.denominator or (2*rem==y.denominator and m%2):m+=1
    return sign*m*power2(scale)
def qvalue(x):
    assert x['class']=='finite';u=int.from_bytes(bytes.fromhex(x['little_endian_bytes']),'little');exp=(u>>112)&32767;mant=u&((1<<112)-1)
    if exp:mant+=1<<112
    return (-1 if u>>127 else 1)*mant*power2((exp or 1)-16383-112)
def double_record(x):return dict(**{'class':'finite'},bits=struct.pack('>d',x).hex())
def quad_record(x):
    x=qround(x)
    if not x:u=0
    else:
        sign=x<0;x=abs(x);e=x.numerator.bit_length()-x.denominator.bit_length()
        if x<power2(e):e-=1
        m=int(x/power2(e-112));u=(int(sign)<<127)|((e+16383)<<112)|(m-(1<<112))
    return {'class':'finite','little_endian_bytes':u.to_bytes(16,'little').hex()}

class RandomModel:
    def __init__(self):self.iff=0;self.ix=[0,0,0];self.r=[F(0)]*100;self.result=F(0);self.iset=0;self.gset=0.;self.draws=0;self.rejections=0;self.registered=False;self.gregistered=False;self.attempts=[]
    def random(self,seeds,which):
        self.registered=True;a,b,c=self.ix;rm1=F(1./259100);rm2=F(1./134456)
        if seeds[which]<0 or not self.iff:
            self.iff=1;a=(54773-seeds[which])%259100;a=(7141*a+54773)%259100;b=a%134456;a=(7141*a+54773)%259100;c=a%243000
            for j in range(97):
                a=(7141*a+54773)%259100;b=(8121*b+28411)%134456;self.r[j]=qround(qround(F(a)+qround(b*rm2))*rm1)
            seeds[which]=1
        a=(7141*a+54773)%259100;b=(8121*b+28411)%134456;c=(4561*c+51349)%243000;j=(97*c)//243000
        self.result=self.r[j];self.r[j]=qround(qround(F(a)+qround(b*rm2))*rm1);self.ix=[a,b,c];self.draws+=1
        return float(self.result)
    def gaussian(self,seeds,which):
        self.gregistered=True
        if self.iset:self.iset=0;return self.gset
        while True:
            v1=2*self.random(seeds,which)-1;v2=2*self.random(seeds,which)-1
            # GCC's ordinary binary64 multiply/add may contract; analytic value
            # is independently rounded here, with the bounded check below.
            rsq=v1*v1+v2*v2;self.attempts.append((v1,v2,rsq))
            if 0<rsq<1:break
            self.rejections+=1
        fac=math.sqrt(-2*math.log(rsq)/rsq);self.gset=v1*fac;self.iset=1;return v2*fac
    def check_state(self,actual):
        if self.registered:
            r=actual['random'];assert r['iff']==self.iff and r['ix']==self.ix
            compare([quad_record(x) for x in self.r],r['r'],'all 100 random cells');compare(quad_record(self.result),r['result'],'random result')
        else:assert actual['random'] is None
        if self.gregistered:assert actual['gaussian']['iset']==self.iset
        else:assert actual['gaussian'] is None

# Explicit expanded T_n polynomials, differentiated algebraically; no native
# recurrence/cache/reverse accumulation is reused for the independent check.
CHEB=[[1],[0,1],[-1,0,2],[0,-3,0,4],[1,0,-8,0,8],[0,5,0,-20,0,16],[-1,0,18,0,-48,0,32],[0,-7,0,56,0,-112,0,64]]
def polynomial(spec,jd,body):
    block=(F(jd)-F(spec['start']))/F(spec['step']);rec=block.numerator//block.denominator;t=block-rec
    if not t and rec:rec-=1;t=F(1)
    _,ncf,na=spec['ipt'][body];z=na*t;sub=int(z);tc=2*(z-sub)-1
    if sub==na:sub-=1;tc=F(1)
    dim=2 if body==11 else 3;out=[]
    for derivative in (0,1):
      for comp in range(dim):
        base=F((rec+1)*1000+(body+1)*10+comp+spec['shift'])+F(sub,16);total=F(0)
        for k in range(ncf):
            coeff=base if k==0 else (-1)**k*base/power2(k+3)
            poly=sum(F(c)*(tc**i) for i,c in enumerate(CHEB[k])) if not derivative else sum(F(i*c)*tc**(i-1) for i,c in enumerate(CHEB[k]) if i)
            total+=coeff*poly
        if derivative:total*=F(2*na)/F(spec['step'])
        out.append(total)
    return out

def target(spec,jd,index):
    au=F(149597870.691)
    if index==12:return [F(0)]*6
    if index in (3,13):
        emb=[x/au for x in polynomial(spec,jd,2)]
        if index==13:return emb
        moon=[x/au for x in polynomial(spec,jd,9)];return [x-y/F(1.0+81.30056) for x,y in zip(emb,moon)]
    body=10 if index==11 else index-1
    return [x/au for x in polynomial(spec,jd,body)]
def rotation():
    with localcontext() as c:
        c.prec=95
        pi=Decimal('3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679')
        x=Decimal.from_float(84381.4059)*pi/Decimal(648000);sin=x;term=x;cos=Decimal(1);cterm=Decimal(1)
        for k in range(1,50):term*=-x*x/Decimal((2*k)*(2*k+1));sin+=term;cterm*=-x*x/Decimal((2*k-1)*(2*k));cos+=cterm
        return F(cos),F(sin)
CE,SE=rotation()
def rotate(v):
    v=list(v);y,z=v[1:3];v[1]=float(CE*F(y)+SE*F(z));v[2]=float(-SE*F(y)+CE*F(z));return v

def main():
    check();observed=read_gzip(OUTPUT/'observed.json.gz');resources=json.loads((OUTPUT/'manifest.json').read_text())['resources']
    counts=dict(random_returns=0,random_state_snapshots=0,gaussian_returns=0,rejected_gaussian_pairs=0,polynomial_components=0,date_calls=0,driver_components=0,retained_cells=0,call_order_actions=0,offset_components=0)
    errors=dict(polynomial=0.,driver=0.,gaussian=0.)
    def close(a,b,kind):
        if math.isnan(a) or math.isnan(b):assert math.isnan(a) and math.isnan(b);return
        if math.isinf(a) or math.isinf(b):assert a==b;return
        delta=abs(a-b);errors[kind]=max(errors[kind],delta)
        # Diagnostic envelopes, never native replay or Rust acceptance gates.
        limit=(2e-18+2e-12*abs(b)) if kind=='polynomial' else (3e-12+3e-14*abs(b))
        assert delta<=limit,(kind,a,b,delta,limit)
    for case in observed:
        model=RandomModel();seeds=[0,0]
        for e in case['events']:
            if e['phase']!='end':continue
            a=case['actions'][e['event']];op=a[0];model.attempts=[]
            if op=='seed':seeds[int(a[1])]=int(a[2])
            elif op=='random':
                value=model.random(seeds,int(a[1]));compare(double_record(value),e['result']['value'],'independent random return');counts['random_returns']+=1
            elif op=='gaussian':
                value=model.gaussian(seeds,int(a[1]));close(f64(e['result']['value']),value,'gaussian');counts['gaussian_returns']+=1
            assert e['private']['random_entries']==model.draws
            assert len(e['private']['attempts'])==len(model.attempts)
            for actual_attempt,expected_attempt in zip(e['private']['attempts'],model.attempts):
                compare(actual_attempt[:2],[double_record(v) for v in expected_attempt[:2]],'actual Gaussian draw pair')
                close(f64(actual_attempt[2]),expected_attempt[2],'gaussian')
                assert (f64(actual_attempt[2])>=1 or f64(actual_attempt[2])==0)==(expected_attempt[2]>=1 or expected_attempt[2]==0)
            compare(seeds,e['seeds'],'native seed transitions');model.check_state(e['private']);counts['random_state_snapshots']+=model.registered
            if model.gregistered:close(f64(e['private']['gaussian']['gset']),model.gset,'gaussian')
            if op!='driver':continue
            pre=next(x for x in case['events'] if x['event']==e['event'] and x['phase']=='begin')['caller'];post=e['caller'];calls=e['calls'];ci=0
            expected_order=[]
            for p in pre['pulsars'][:max(0,int(a[1]))]:
                open_call=calls[ci];ci+=1;assert open_call['op']=='open';expected_order.append(('open',));assert open_call['path']==p['ephemeris']
                resource=resources[p['ephemeris'].rsplit('/',1)[1][:-4]];spec=resource['spec'];units=p['units'];ecl=p['eclCoord']==1
                au=149597870.691*1000 if 'real_path' not in spec else None
                for i in range(max(0,p['nobs'])):
                    index=p['observation_owner']+i;b=pre['observations'][index];z=post['observations'][index]
                    correction=0.
                    for clock in b['clock_steps'][:max(0,b['nclock'])]:correction+=f64(clock[0])
                    jd=qround(qround(qround(qvalue(b['sat'])+F(correction/86400.))+qround(qvalue(b['teph'])/86400))+F(2400000.5))
                    selected=[j for j in range(9) if p['parameters'][0]['set'][j]==1 or p['parameters'][1]['set'][j]==1]
                    requested=[(11,12,'sun_ssb',None),(11,3,'sun_earth',None)]
                    requested += [(j+1,12,0,j) for j in selected]+[(j+1,12,1,j) for j in selected]
                    requested += [(t,3,n,None) for t,n in [(5,'jupiter_earth'),(6,'saturn_earth'),(2,'venus_earth'),(7,'uranus_earth'),(8,'neptune_earth')]]
                    requested += [(3,12,'earth_ssb',None),(13,12,'earthMoonBary_ssb',None),(13,3,'earthMoonBary_earth',None),(14,3,'nutations',None)]
                    loaded={}
                    for t,centre,name,j in requested:
                        call=calls[ci];ci+=1;expected_order.append(('pleph',t,centre));assert (call['op'],call['target'],call['centre'],call['velocity'])==('pleph',t,centre,1)
                        et=float(qround(jd+F(1e-6))) if name==1 else float(jd);compare(double_record(et),call['jd'],'complete JD conversion');counts['date_calls']+=1
                        raw=[f64(v) for v in call['output']]
                        if 'real_path' not in spec:
                            if t==14:
                                expected=[float(x) for x in polynomial(spec,et,11)]+[0.,0.] if spec['optional'] else [0.]*6
                            else:expected=[float(x-y) for x,y in zip(target(spec,et,t),target(spec,et,centre))]
                            for x,y in zip(raw,expected):close(x,y,'polynomial');counts['polynomial_components']+=1
                            assert call['status']==(-3 if t==14 and not spec['optional'] else 0)
                        else:assert call['status']==0
                        if t==14:loaded[name]=raw;continue
                        # Read the actual real AU in the pinned resource (big endian).
                        if au is None:
                            from pathlib import Path
                            au=struct.unpack_from('>d',Path(spec['real_path']).read_bytes(),2680)[0]*1000.
                        converted=float(qround(F(au)*qround(1+F(1.55051979176e-8)))) if units==1 else au
                        v=[x*(converted/299792458.) if c<3 else x*(converted/86400./299792458.) for c,x in enumerate(raw)]
                        if ecl and name!='earth_ssb':v=rotate(v)
                        loaded[(name,j) if isinstance(name,int) else name]=v
                    for group in (0,1):
                        for j in range(9):
                            if j not in selected:compare(b['planets'][group][j],z['planets'][group][j],'inactive complete planet');counts['retained_cells']+=6
                    expected_planets=[[[f64(x) for x in v] for v in group] for group in b['planets']]
                    for group in (0,1):
                        for j in selected:expected_planets[group][j]=loaded[(group,j)]
                    for j in range(9):
                        # Difference uses the actual rounded stored arrays; independent
                        # polynomial and frame checks already constrain their values.
                        expected_diff=[f64(x)-f64(y) for x,y in zip(z['planets'][1][j],z['planets'][0][j])]
                        compare([double_record(x) for x in expected_diff],z['planets'][2][j],'unconditional 1e-6-day difference')
                    earth=loaded['earth_ssb']
                    for j in range(9):
                        if p['parameters'][1]['set'][j]:
                            mass=qvalue(p['parameters'][1]['values'][j]);pv=[f64(x) for x in z['planets'][0][j]]
                            earth=[float(qround(F(x)-qround(mass*F(y)))) for x,y in zip(earth,pv)]
                    if ecl:earth=rotate(earth[:3])+rotate(earth[3:])
                    for axis,table in enumerate(p['offsets']):
                        delta=0.;sat=float(qvalue(b['sat']))
                        for k in range(max(0,table['count']-1)):
                            t0,v0=map(f64,table['backing'][k]);t1,v1=map(f64,table['backing'][k+1])
                            if t0<=sat<t1:
                                # Stable mathematical interpolation; native uses
                                # slope/intercept with possible fused multiply/add.
                                delta=v0+(v1-v0)*(sat-t0)/(t1-t0);break
                        if table['count']>0:earth[axis]+=delta
                        counts['offset_components']+=1
                    tel=bytes.fromhex(b['telID']['hex']).split(b'\0')[0]
                    if tel==b'STL_BAT':
                        import re
                        for flag in b['flags'][:b['nFlags']]:
                            key=bytes.fromhex(flag[0]['hex']).split(b'\0')[0].decode();value=bytes.fromhex(flag[1]['hex']).split(b'\0')[0].decode()
                            if key in ('-telx','-tely','-telz'):
                                match=re.match(r'\s*([+-]?(?:nan|inf(?:inity)?|(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?))',value,re.I)
                                if match:earth[('-telx','-tely','-telz').index(key)]=float(match[1])
                        earth[3:]=[0.,0.,0.]
                    loaded['earth_ssb']=earth
                    for name,actual in z['vectors'].items():
                        for x,y in zip(map(f64,actual),loaded[name]):close(x,y,'driver');counts['driver_components']+=1
                    for group in (0,1):
                        for j in selected:
                            for x,y in zip(map(f64,z['planets'][group][j]),expected_planets[group][j]):close(x,y,'driver');counts['driver_components']+=1
                assert calls[ci]['op']=='close' and calls[ci]['handle']==open_call['handle'];ci+=1;expected_order.append(('close',))
            assert ci==len(calls);counts['call_order_actions']+=1
            # Every unprocessed observation, including allocated tails, is exact.
            processed={p['observation_owner']+i for p in pre['pulsars'][:max(0,int(a[1]))] for i in range(max(0,p['nobs']))}
            for i in range(12):
                if i not in processed:compare(pre['observations'][i],post['observations'][i],'unprocessed observation');counts['retained_cells']+=228
        counts['rejected_gaussian_pairs']+=model.rejections
    assert counts['rejected_gaussian_pairs']>0
    save(OUTPUT/'independent.json',dict(status='PASS',counts=counts,max_absolute_errors=errors,limits=dict(polynomial='2e-18 + 2e-12 * abs(expected) AU or native optional units',driver='3e-12 + 3e-14 * abs(expected) seconds or light-seconds/second',gaussian='3e-12 + 3e-14 * abs(expected), dimensionless'),observed_sha256=sha((OUTPUT/'observed.json.gz').read_bytes())))
    print('PASS independent',counts,errors)
if __name__=='__main__':main()
