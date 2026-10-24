"""Authored initialized caller histories; all native preconditions precede execution."""
from decimal import Decimal
import math

VECTORS = ('sun_ssb','sun_earth','jupiter_earth','saturn_earth','venus_earth',
           'uranus_earth','neptune_earth','earth_ssb','earthMoonBary_ssb','earthMoonBary_earth','nutations')
PLANETS = ('planet_ssb','planet_ssb_tmr','planet_ssb_derv')
RESOURCES = dict(le=dict(order='<'),be=dict(order='>'),shift=dict(order='<',shift=7000),
                 absent=dict(order='<',optional=False),real=dict(real=True),missing=dict(missing=True),short=dict(short=True))
UNRUN = [
 dict(path='JPL out-of-range/short-read planetary calls',classification='native-invalid',reason='pleph continues into unwritten local pv; no post-failure vector oracle'),
 dict(path='JPL malformed counts/layout, >2000 constants',classification='native-invalid',reason='driver fixed nams/vals capacity and reader indexing; preflight excludes invalid accesses'),
 dict(path='random2 index error',classification='unreachable-defined-domain',reason='nonnegative congruential states give 0 <= (97*ix3)/243000 <= 96'),
 dict(path='gasdev rsq == 0',classification='defined-unsampled',reason='no artificial state corruption to force an exact zero pair'),
 dict(path='overflowing seed arithmetic and oversized counts/unterminated strings',classification='native-invalid',reason='caller contract excludes signed overflow, invalid conversion and out-of-bounds access'),
 dict(path='allocation/seek faults and concurrent file mutation',classification='fault-unrun',reason='no fault injection or concurrent resource writes; H evidence remains separately available'),
 dict(path='CALCEPH/global wrappers/propagation/phase lifecycle',classification='outside-stage',reason='actual callers mapped but not executed'),
 dict(path='other ABI/compiler/real ephemerides',classification='host-unrun',reason='one declared macOS arm64 GCC binary128 profile'),
]

def cases():
    out=[]
    def add(label,actions,status=0,classification='host-qualified-native-JPL'):
        out.append(dict(id=f'c{len(out):03}_{label}',label=label,status=status,classification=classification,
                        actions=[[str(v) for v in a] for a in [['profile'],*actions]]))
    def p(i=0,n=1,units=2,ecl=0,res='le'):return ['pulsar',i,n,units,ecl,res]
    def o(i=0,t='50010.25',site='A',teph='0'):return ['observation',i,t,site,teph]
    def par(kind,j,flag=1,value='0.0001',owner=0):return ['parameter',owner,kind,j,flag,value]
    def d(n=1,noise=0):return ['driver',n,noise]
    add('noops',[p(n=-1),p(1,0,res='be'),d(-1),d(0),d(2),d(2,7),d(0,-2)])
    add('cvs_debug',[p(),['debug',1,1],d(),d(),['debug',0,2],d()])
    for units in (1,2,0,-1):
      for ecl in (0,1,2,-1):
        add(f'units_{units}_frame_{ecl}',[p(n=2,units=units,ecl=ecl),o(1,'50030.125'),d(),d()])
    for kind in ('phase','mass'):
      for j in range(9):
        add(f'{kind}_{j}',[p(ecl=j%2),par(kind,j),d(),par(kind,j,0,'9'),d(),par(kind,j,1,'-0.0002'),d()])
    for flag in (-2,-1,2,3):
      add(f'nonone_{flag}',[p(ecl=1),*[par('mass',j,flag,str((j+1)*.00001)) for j in range(9)],
          *[par('phase',j,flag,'0.5') for j in range(9)],d(),*[par('phase',j,1) for j in range(9)],d()])
    for frame in (0,1):
      add(f'all_planets_{frame}',[p(n=2,ecl=frame),o(1,'50033'),
          *[par('phase',j,1,str(j+1)) for j in range(9)],d(),
          *[par('mass',j,1,str((j+1)*.00001)) for j in range(9)],d(),
          *[par('phase',j,0) for j in range(9)],d()])
    add('phase_value_inactive',[p(),par('phase',4,1,'0'),d(),par('phase',4,1,'900'),d()])
    add('multi_resources',[p(n=2),p(1,2,1,1,'shift'),o(6,'50020'),o(7,'50050'),d(2),
        p(0,1,2,0,'be'),p(1,0,res='absent'),d(2),p(1,1,res='real'),d(2)])
    add('clock_teph_history',[p(n=3),o(0,'50010.12345678901234567890123456789',teph='.1234567890123456789'),
         o(1,'50010.25',teph='-32.184'),o(2,'50010.25',teph='86400'),
         ['tt',0,3,'1e16','-1e16','0.375'],['tt',1,2,'37','32.184'],['tt',2,-1,'9','8','7'],d(),
         ['tt',0,3,'1e16','0.375','-1e16'],o(2,'50010.25',teph='0'),d()])
    dates=['50000','50000.000000001','50004','50008','50016','50032','50064','50096','50127.999998']
    for jd in (2450004.5,2450032.5,2450010.75):
      for x in (math.nextafter(jd,-math.inf),jd,math.nextafter(jd,math.inf)):
        dates.append(str(Decimal.from_float(x)-Decimal('2400000.5')))
    # Straddle a binary64 JD midpoint using exact longdouble decimal inputs.
    mid=(Decimal.from_float(2450010.75)+Decimal.from_float(math.nextafter(2450010.75,math.inf)))/2-Decimal('2400000.5')
    dates += [str(mid-Decimal('1e-20')),str(mid),str(mid+Decimal('1e-20'))]
    for k in range(0,len(dates),6):
      batch=dates[k:k+6]
      add(f'dates_{k:02}',[p(n=len(batch)),*[o(i,t) for i,t in enumerate(batch)],*[par('phase',j) for j in range(9)],d(),d()])
    add('end_no_selection',[p(),o(t='50128'),d()])
    for axis in range(3):
      for count in (-1,0,1,2,4):
        add(f'offset_{axis}_{count}',[p(n=6,ecl=1),*[o(i,t) for i,t in enumerate(('50001','50004','50008','50012','50016','50020'))],
             ['offset',0,axis,count,'50004','1','50008','5','50016','-3','50020','9'],d(),d()])
      add(f'offset_overlap_{axis}',[p(n=2),o(0,'50012'),o(1,'50008'),
          ['offset',0,axis,4,'50000','1','50016','17','50004','500','50020','600'],d()])
    add('offset_independent_modes',[p(n=3),o(0,'50008'),o(1,'50016'),o(2,'50018'),
          *[['offset',0,ax,c,'50000',ax+1,'50016',ax+17,'50020',ax+21,'50024',99] for ax,c in enumerate((0,1,4))],
          ['offset_mode',0,0,2],['offset_mode',0,1,-1],['offset_mode',0,2,0],d(),['offset_mode',0,2,2],d()])
    for i,site in enumerate(('STL_BAT','stl_bat','Stl_Bat','STL','STL_FBAT','A')):
      add(f'station_{i}',[p(ecl=1),o(site=site),['flag',0,'-telx','12.25'],['flag',0,'-tely','-0'],['flag',0,'-telz','77suffix'],d()])
    add('stl_scan_history',[p(),o(site='STL_BAT'),['flag',0,'-telx','2'],['flag',0,'-telx','bad'],
        ['flag',0,'-tely','bad'],['flag',0,'-telz','3tail'],['flag',0,'-telz','4'],['flag',0,'-TELX','999'],d(),
        ['clear_flags',0],d(),['flag',0,'-tely','nan'],['flag',0,'-telz','-inf'],d()])
    add('absent_nutation',[p(res='absent',ecl=1),*[par('mass',j) for j in range(9)],
         ['offset',0,1,2,'50000','3','50020','7','50030','9','50040','11'],d(),o(site='STL_BAT'),['flag',0,'-telx','9'],d()])
    for res in ('missing','short'):
      add('open_'+res,[p(n=0,res=res),d()],1,'defined-exit')
      add('open_prefix_'+res,[p(),p(1,0,res=res),d(2)],1)
    add('noise_independence',[p(),['seed',0,-19],['random',0],d(1,1),['random',0],d(1,-9),['gaussian',0],d(1,0),['gaussian',0]])
    for seed in (-1,-19,-12345,-1000000000,0,1,54773):
      add('random_'+str(seed),[['seed',0,seed],*[['random',0] for _ in range(110)],['seed',0,0],['random',0],['seed',0,-19],['random',0]],classification='defined-random-helper')
    add('shared_seeds',[['seed',0,-19],['seed',1,123],['random',0],['random',1],['seed',1,-7],['random',1],['random',0],['seed',0,0],['random',0]],classification='defined-random-helper')
    for seed in (-1,-19,-12345,0,1):
      add('gaussian_'+str(seed),[['seed',0,seed],*[['gaussian',0] for _ in range(20)]],classification='defined-random-helper')
    add('gaussian_cached_negative',[['seed',0,-19],['gaussian',0],['seed',1,-12345],['gaussian',1],['random',0],['gaussian',1],['seed',0,-1],['gaussian',0],['random',0],['gaussian',1]],classification='defined-random-helper')
    for c in out:validate_inputs(c)
    assert len({c['id'].casefold() for c in out})==len(out)
    return out

def validate_inputs(case):
    """Bound only realistic native caller hazards in this authored finite corpus."""
    for a in case['actions']:
        op=a[0]
        if op=='pulsar':assert 0<=int(a[1])<2 and -1<=int(a[2])<=6 and a[5] in RESOURCES
        elif op=='driver':assert -1<=int(a[1])<=2
        elif op=='observation':assert 0<=int(a[1])<12 and 50000<=Decimal(a[2])<=50128 and len(a[3])<100
        elif op=='parameter':assert 0<=int(a[1])<2 and a[2] in ('phase','mass') and 0<=int(a[3])<9
        elif op=='offset':assert 0<=int(a[1])<2 and 0<=int(a[2])<3 and -1<=int(a[3])<=4
        elif op=='tt':assert 0<=int(a[1])<12 and -1<=int(a[2])<=3
        elif op=='seed':assert 0<=int(a[1])<2 and -1000000000<=int(a[2])<=54773
        elif op in ('random','gaussian'):assert 0<=int(a[1])<2
        elif op=='flag':assert 0<=int(a[1])<12 and len(a[2])<32 and len(a[3])<32
        elif op=='offset_mode':assert 0<=int(a[1])<2 and 0<=int(a[2])<3
        elif op=='clear_flags':assert 0<=int(a[1])<12
        else:assert op in ('profile','debug'),op
