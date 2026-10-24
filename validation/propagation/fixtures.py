"""Authored cases and safety bounds, frozen before reference recording."""
import math
import datetime

def p(j,f,v,i=0):return ['p',j,f,i,v]
def o(j,f,v,i=0):return ['o',j,f,i,v]
def par(j,f,v,i=0,flag=1):return ['parameter',j,f,i,v,flag]
def table(j,name,ts,vs):return [p(j,name+'_mjd',t,i) for i,t in enumerate(ts)]+[p(j,name,v,i) for i,v in enumerate(vs)]
def shape(j,scale=2,n=4,pos=58000,width=2,coef=None):return ['shape_event',j,0,scale,n,pos,width,*(coef or [1,.2,-.3,.4,0,0,0,0])]
def resources():return {
 'root/observatory/observatories.dat':'6378237 0 0 North A UTC\n0 0 0 COE COE UTC\n',
 'root/observatory/aliases':'A alpha\nCOE coe\n',
 'root/atmosphere/a.sap':'# A pressure\n57990 90\n58000 100\n58010 110\n',
 'root/atmosphere/a.zwd':'# A wet\n57990 .1\n58000 .2\n58010 .3\n'}
def solar_files(kind):
 data='0 89\n180 89\n' if kind=='polar' else ''.join(f'{lon} {lat}\n' for lon in range(0,360,30) for lat in [-60,-30,0,30,60])
 return {f'root/solarWindModel/CS{rot}.txt':data for rot in range(2100,2301)}
def cases():
 result=[]
 def add(id,actions,files=None,status=0,classification='defined-numerics-host-diagnostics',timeout=30):
  result.append(dict(id=id,classification=classification,status=status,timeout=timeout,files=files or {},actions=[[str(v) for v in a] for a in [['profile'],['env','TEMPO2','{case}/root'],*actions]]))
 for count in [-1,0,1,2]:
  add(f'counts_{count}',[['bclt',count],['bats',count],['secular',count],['init',count],['clean',count]])
 for tro in [-1,0,1,2]:
  add(f'bclt_tropo_{tro}',[p(0,'nobs',1),p(1,'nobs',2),p(0,'correctTroposphere',tro),p(1,'correctTroposphere',tro),['bclt',2],['bclt',2]],resources())
 for site in ['A','STL_FBAT','STL_BAT']:
  for delay in [0,1,2]:
   add(f'bclt_{site}_{delay}',[o(0,'telID',site),o(0,'delayCorr',delay),par(0,'pmrv',12),['bclt',1],['bats',1]])
 for fast in [-1,0,1,2]:
  for deleted in [0,1,2]:add(f'iteration_fast_{fast}_deleted_{deleted}',[['fast',fast],o(0,'deleted',deleted),par(0,'pmrv',7),['bclt',1]])
 add('iteration_nonfinite',[p(0,'posPulsar',0,0),p(0,'posPulsar',0,1),p(0,'velPulsar',0,0),p(0,'velPulsar',0,1),['bclt',1]])
 # Large but finite acceleration feeds back a steep time derivative; no integer conversion.
 add('iteration_steep',[p(0,'accPulsar',1e14,0),['bclt',1]],classification='defined-floating-nonfinite-possible')
 add('iteration_limit',[p(0,'posPulsar',0,0),p(0,'posPulsar',1,1),p(0,'velPulsar',-31557600,0),p(0,'velPulsar',0,1),p(0,'accPulsar',0,2),p(0,'ipm',0),p(0,'planetShapiro',0),par(0,'dm',0),par(0,'px',0),*[o(0,'earth_ssb',100 if k==0 else 0,k) for k in range(3)],*[o(0,'observatory_earth',0,k) for k in range(3)],['bclt',1]])
 add('diagnostic_cvs',[['debug',1,1,0],['bclt',1],['bats',1],['secular',1],['init',2],['clean',2]])
 for site in ['A','STL_BAT','STL_FBAT','stl_bat']:
  for flag in [0,1,2,-1]:
   add(f'shapiro_{site}_{flag}',[o(0,'telID',site),p(0,'planetShapiro',flag),['shapiro',0,0,.25,0],['shapiro',0,0,.25,999]])
 add('shapiro_bypass',[o(0,'delayCorr',0),['shapiro',0,0,.25,0]])
 add('shapiro_zero_norm',[p(0,'posPulsar',0,0),p(0,'posPulsar',0,1),['shapiro',0,0,0,0]],classification='defined-nonfinite')
 add('shapiro_zero_radius',[*[o(0,'sun_ssb',100+20*k+.01*(k+1),k) for k in range(3)],['shapiro',0,0,0,0]],classification='defined-nonfinite')
 add('shapiro_exact_zero_radius',[[*a] for a in [*[o(0,f,0,k) for f in ['earth_ssb','sun_ssb','observatory_earth'] for k in range(3)],['shapiro',0,0,0,0]]],classification='defined-nonfinite')
 add('shapiro_log_zero',[p(0,'posPulsar',1,0),p(0,'posPulsar',0,1),*[o(0,'earth_ssb',0,k) for k in range(3)],*[o(0,'observatory_earth',0,k) for k in range(3)],*[o(0,'sun_ssb',1 if k==0 else 0,k) for k in range(3)],['shapiro',0,0,0,0]],classification='defined-nonfinite')
 add('nonpositive_observation_counts',[p(0,'nobs',0),p(1,'nobs',-1),['bclt',2],['bats',2],['secular',2]])
 add('dm_bypass',[o(0,'delayCorr',0),['dm',0,0,0,0]])
 for site in ['A','STL_FBAT']:
  for mask in [0,1,3,7,8]:
   add(f'dm_velocity_{site}_{mask}',[o(0,'telID',site),*[par(0,f,(i+1)*1e-4,1,1 if mask&(1<<i) else 2) for i,f in enumerate(['telx','tely','telz'])],['dm',0,0,.5,0]])
 for hz in [-1,0,.9999999999999999,1,1.0000000000000002,100,1.4e9]:
  for rate in [0,1,-1]:
   add(f'dm_frequency_{hz}_{rate}',[*[o(0,'earth_ssb',0,k+3) for k in range(3)],*[o(0,'siteVel',0,k) for k in range(3)],o(0,'freq',hz/1e6),o(0,'einsteinRate',rate),p(0,'TNChromIdx',4),['dm',0,0,0,0]])
 for series in [0,1,2]:
  for order in range(1,9):
   add(f'derivative_{series}_{order}',[p(0,'dm_series_type',series),o(0,'sat',58365.5),par(0,'dm',.01,order),par(0,'cm',.001,order),p(0,'TNChromIdx',3.5),['dm',0,0,0,0]])
 add('derivative_cross_selector',[p(0,'dm_series_type',2),p(1,'dm_series_type',1),o(6,'sat',58730.5),par(1,'dm',2,3),par(1,'cm',.5,3),p(1,'TNChromIdx',2),['dm',1,0,0,0],p(0,'dm_series_type',1),['dm',1,0,0,0]])
 for epoch in [57999,58000,58000.25,58001,58002]:
  add(f'dmx_chromx_{epoch}',[o(0,'sat',epoch),p(0,'ndmx',2),p(0,'nchromx',2),p(0,'TNChromIdx',3),*[a for i in range(2) for a in [par(0,'dmxr1',58000,i),par(0,'dmxr2',58001+i,i),par(0,'dmx',i+1,i),par(0,'chromxr1',58000,i),par(0,'chromxr2',58001+i,i),par(0,'chromx',.01*(i+1),i)]],['dm',0,0,0,0]])
 for flags in [[('-dm','12suffix'),('-dmo','2'),('-dmo','3')],[('-dmo','2'),('-dm','20'),('-dmo','3')],[('-dmo','2'),('-dmo','bad')],[('-dm','bad'),('-dm','-0')],[('-unused','50'),('-dm','nan')]]:
  add('flags_'+str(len(result)),[['flag',0,*f] for f in flags]+[['dm',0,0,0,0]])
 for mean in [0,-1,1]:
  add('dmmodel_'+str(mean),[par(0,'dm',10),par(0,'dmmodel',11),p(0,'dmoffsDMnum',3),*table(0,'dmoffsDM',[57999,58000,58001],[mean]*3),['dm',0,0,0,0],['dm',0,0,0,0],['link',0,'dmmodel',1,'dm'],['dm',0,0,0,0]])
 add('dmmodel_cross_warning',[['link',0,'dmmodel',0,'dm'],par(1,'dm',10),par(1,'dmmodel',12),p(1,'dmoffsDMnum',1),*table(1,'dmoffsDM',[58000],[1]),['dm',1,0,0,0],['link',0,'dmmodel',0,'px'],['dm',1,0,0,0]])
 add('dmmodel_bypasses_flags_dmx',[par(0,'dmmodel',12),p(0,'dmoffsDMnum',1),*table(0,'dmoffsDM',[58000],[0]),p(0,'ndmx',1),par(0,'dmxr1',57999),par(0,'dmxr2',58001),par(0,'dmx',999),['flag',0,'-dm','999'],['flag',0,'-dmo','bad'],['dm',0,0,0,0]])
 for scale in [0,2,2.00001]:
  for bat in [0,58001]:add(f'shape_event_{scale}_{bat}',[shape(0,scale),o(0,'bat',bat),['dm',0,0,0,0]])
 add('shape_cross_pulsar',[shape(0,0,4,58000,2),shape(1,2,1,58100,9,[100]*8),['dm',1,0,0,0],shape(0,0,1,58000,3,[2]*8),['dm',1,0,0,0]])
 add('annual',[par(0,'dm_sin1yr',.2),par(0,'dm_cos1yr',.3),['dm',0,0,0,0],o(0,'sat',58091.3125),['dm',0,0,0,0]])
 for swm in [-1,0,1,2]:
  for ipm in [0,1,2]:
   fs=solar_files('grid') if swm==1 and ipm else {}
   add(f'solar_selector_{swm}_{ipm}',[p(0,'swm',swm),p(0,'ipm',ipm),['dm',0,0,0,0]],fs)
 for flag in [-1,0,1,2]:
  add(f'solar_cycle_{flag}',[par(0,'ne_sw_sin',2,0,flag),p(0,'ne_sw_ifuncN',2),p(0,'ne_sw_ifuncT',57999,0),p(0,'ne_sw_ifuncT',58001,1),p(0,'ne_sw_ifuncV',1,0),p(0,'ne_sw_ifuncV',3,1),['dm',0,0,0,0]])
 for coord in [0,1]:
  for kind in ['polar','grid']:add(f'solar_model_{coord}_{kind}',[p(0,'eclCoord',coord),['solar',0,0],['solar',0,0]],solar_files(kind))
 add('solar_missing',[['solar',0,0]],status=1)
 for flag in [-1,0,1,2]:
  for planet in [-1,0,1,2]:add(f'bats_{flag}_{planet}',[p(0,'calcShapiro',flag),p(0,'planetShapiro',planet),['bats',1],['bats',1]])
 add('bats_retained_bypass',[o(0,'delayCorr',0),['bats',1],['secular',1]])
 for first in [0,1,2]:
  for second in [0,1]:
   add(f'secular_flags_{first}_{second}',[par(0,'dshk',1,flag=first),par(0,'pmra',3,flag=first),par(1,'dshk',2,flag=second),par(1,'pmdec',7,flag=second),o(0,'bat',59000.25),o(6,'bat',58500.25),['secular',2]])
 add('secular_link',[par(0,'dshk',2),par(0,'pmdec',4),['link',0,'dshk',1,'px'],par(0,'px',.5),o(0,'bat',59000.25),['secular',1]])
 for site in ['@','@x','bat','BaT','STL','STL_FBAT','STL_BAT','stl','A']:
  add('phase_'+site,[p(0,'refphs',1),p(0,'tzrsite',site),['init',2],['clean',1],['init',1],['clean',1]])
 for a,b in [(0,1),(1,0),(1,1),(0,0)]:
  add(f'phase_cross_{a}_{b}',[p(0,'refphs',a),p(1,'refphs',b),['init',2],['clean',2]])
 add('phase_capacity',[p(0,'refphs',1),p(0,'nobs',6),['init',1]],status=1)
 add('phase_empty_init',[p(0,'refphs',1),p(0,'nobs',0),['init',1],['clean',1]])
 add('phase_repeated_clean',[p(0,'refphs',1),p(0,'nobs',2),['clean',1],['clean',1]])
 add('composed',[p(0,'refphs',1),p(1,'refphs',1),['init',2],*[o(j,f,val,k) for j in [1,7] for f,k,val in [('earth_ssb',0,100),('earth_ssb',1,120),('earth_ssb',2,140),('sun_ssb',0,2),('sun_ssb',1,3),('sun_ssb',2,4),('jupiter_earth',0,1000),('saturn_earth',0,2000),('venus_earth',0,100),('uranus_earth',0,3000),('neptune_earth',0,4000)]],['bclt',2],['bats',2],['secular',2],['bclt',2],['bats',2],['secular',2],['clean',2]])
 add('precision_noop',[['precision','0.0000000001','routine','comment'],['precision','nan','other','tail']])
 tv=[0,1,1,3,2,-2,3,5,4,7,5,9]
 for op in ['ifunc','basis','sinc']:
  for t in [-1,0,.5,1,2.5,5,6]:add(f'{op}_{t}',[[op,t,6,2,*tv]])
 add('ifunc_one',[['ifunc',t,1,0,*tv] for t in [-1,0,1]])
 add('ifunc_nonpositive_early',[['ifunc',-1,n,0,*tv] for n in [-1,0]])
 add('basis_negative',[['basis',.5,6,-1,*tv]])
 add('basis_assert',[['basis',.5,6,6,*tv]],status=-6,classification='defined-assertion')
 add('sinc_zero',[['sinc',2,6,2,*tv]])
 for n in [-2,0,1,2,3,8,12]:
  add(f'hermite_{n}',[['hermite',n,.7]])
  add(f'recurrence_{n}',[['recurrence',n,.7,1,1.4]])
 for n in [0,1,2,10,170,171,200]:add(f'factorial_{n}',[['factorial',n]])
 for n in [-1,0,1,2,8,12]:
  for width in [-2,0,2]:add(f'shapelet_{n}_{width}',[['shapelet',n,1,width,1.5,*[(-.2)**i for i in range(12)]]])
 for v in [(0,0,0,0),(0,0,math.pi,0),(0,0,math.pi/2,0),(1,.3,2,-.7)]:add('angle_'+str(len(result)),[['angle',*v]])
 for a,b in [(4,3),(-4,3),(4,-3),(-4,-3),(0,0),(-0.,1),(1,0)]:add('amod_'+str(len(result)),[['amod',a,b]])
 for lon in [-1,0,math.pi,math.nextafter(math.pi,math.inf),2*math.pi,4*math.pi]:add('rotation_'+str(len(result)),[['rotation',lon,2193.75]])
 for y in [0,1,99,100,199,200]:add(f'elsun_{y}',[['elsun',y,60,12345.678901]],status=1 if y in [0,200] else 0)
 add('elsun_float_seconds',[['elsun',117,247,s] for s in [0,12345.678901,12345.678902,86399.999]])
 for y in [1901,2000,2017,2099]:
  add(f'calendar_{y}',[['date',(datetime.date(y,m,15)-datetime.date(1858,11,17)).days] for m in range(1,13)])
 for lon in [-2,0,2,4]:add(f'ecliptic_{lon}',[['ecliptic',lon,.3]])
 for lon in [0,1,2,3,4,5,6]:add(f'mcl_{lon}',[['mcl',lon,.3,117,247,21600,400]])
 add('output_wrapping',[['output',-4*math.pi,.4,-1,.05]])
 for name,data in [('empty',''),('one','1 2\n'),('prefix','1 2\n3'),('full','1 2\n3 4\n'),('nonfinite','nan inf\n')]:add('sheet_'+name,[['sheet','{case}/sheet']],{'sheet':data})
 add('sheet_capacity_safe',[['sheet','{case}/sheet']],{'sheet':''.join(f'{i} {i/2}\n' for i in range(499))})
 add('sheet_missing',[['sheet','{case}/absent']],status=1)
 add('sheet_nonconsuming',[['sheet','{case}/sheet']],{'sheet':'bad\n'},status='timeout',classification='bounded-nontermination',timeout=3)
 return result
UNRUN={
 'bad_first_dmo':'uninitialized dmval2 read after first failed sscanf',
 'invalid_eclCoord':'solarWindModel automatic eclon/eclat uninitialized outside exact 0/1',
 'empty_solar_sheet':'solarWindModel reads uninitialized first current cells, unlike direct reader',
 'sheet_over_capacity':'unbounded fscanf destination would exceed 500 cells',
 'phase_underflow':'refphs_clean active first flag and current nobs<=0 would access before backing',
 'phase_over_capacity':'nobs greater than MAX_OBSN evades equality check and can exceed backing',
 'ifunc_invalid_lookahead':'nonfinite/nonmonotone/no bracket paths may read outside initialized array',
 'basis_nonpositive_count':'debug argument reads mjd[N-1] before assertion',
 'integer_conversion_bounds':'amod/rotation/calendar/mcl nonfinite or out-of-range integer conversions',
 'factorial_UINT_MAX':'unsigned loop wraps forever; no native return',
 'OpenMP':'serial only; races/concurrency and other platforms unverified',
 'global_wrappers':'full upstream ephemeris/clock/formBatsAll/residual/fitting not executed'}
def validate_inputs(case):
 assert case['id'] and case['actions'][0]==['profile']
 assert len({case['id']})==1
 for name,value in case['files'].items():
  assert not name.startswith('/') and '..' not in name.split('/')
  if 'CS' in name:
   rows=value.splitlines();assert 0<len(rows)<500
   assert all(len(r.split())==2 and all(math.isfinite(float(v)) for v in r.split()) for r in rows)
 for a in case['actions']:
  if a[0] in ['p','parameter','link','shape_event','solar','dm','shapiro']:assert 0<=int(a[1])<2
  if a[0]=='o':assert 0<=int(a[1])<12
  if a[0] in ['hermite','shapelet']:assert int(a[1])<=12
  if a[0]=='factorial':assert 0<=int(a[1])<=200
