"""Authored safe cases. Numeric case IDs avoid case-insensitive filename collisions."""
import math
from inventory import blob

NEW = 'FORMAT(3(I4),I7,2(F11.6),2(F12.7),2(F11.6),2(F11.6),2(F11.7),2F12.6)'

def eop(delta=0):
    return NEW+'\n'+''.join(f'2020 1 {i+1} {t} {i+1.25} {-i-2.5} {d+delta}\n' for i,(t,d) in enumerate([(57990,.1),(58010,.3),(58030,.45),(58060,.6)]))

def ut1(rows=None):
    if rows is None:rows=[(58000,[310000+37*i+i*i for i in range(6)]),(58030,[310000+37*i+i*i for i in range(6,12)]),(58060,[310000+37*i+i*i for i in range(12,18)])]
    lines=[]
    for mjd,values in rows:
        s=' '.join(map(str,[mjd,*values]));assert len(s)<=57
        lines.append(s.ljust(57)+' 6 999999\n')
    return 'ignored header one\nignored header two\n'+''.join(lines)

def resources():
    return {'root/observatory/observatories.dat':
        '6378237 0 0 Alpha A LOCAL\n0 -4500000 -4500000 Beta B UTC\n6378437 10 200000 Imaginary IMAG LOCAL\n1e-6 0 6356752 Pole POLE UTC\n6378237 0 0 SatelliteFallback stl UTC\n6378237 0 0 FbatFallback stl_fbat UTC\n6378237 0 0 BatFallback stl_bat UTC\n',
        'root/observatory/aliases':'A alpha\nB beta\nIMAG imag_alias\n',
        'root/eop':eop(), 'root/eop_other':eop(.125),
        'root/clock/ut1.dat':ut1(),
        'root/clock/local.clk':'# LOCAL UTC 1\n57900 21600\n58100 21600\n',
        'root/clock/utc.clk':'# UTC TAI 1\n57900 37\n58100 37\n'}

def cases():
    out=[]
    def add(label,actions,files=None,status=0):
        fs=resources();fs.update(files or {})
        out.append(dict(id=f'c{len(out):03}_{label}',label=label,classification='defined-host-profile',status=status,files=fs,
          actions=[[str(v) for v in a] for a in [['profile'],['env','TEMPO2','{case}/root'],*actions]]))
    def pulsar(p=0,n=1,method=1,ecl=0,warn=0,override='',eop_file='eop'):
        return ['pulsar',p,n,method,ecl,warn,override,eop_file]
    def observation(i=0,t='58010.25',site='A',delay=1,clock=1,ut=123):
        return ['observation',i,t,site,delay,clock,ut]
    def par(p,idx,j,val,flag=1):return ['parameter',p,idx,j,val,flag]
    # No shifts: every string has no leading, repeated whitespace or newline.
    add('white_defined', [['white',s] for s in ('','x','a b','a\tb','a b ','a\tb\t','a\rb')])
    for selector in (-3,0,1,2,3,4,5):
        add('ang_selector_'+str(selector),[['ang',selector,v] for v in ('0','-0','1','59.999','10000','123456.789','-123456.789','3595959.9')])
    add('lmst_dates',[['lmst',m,l] for m in ('-20000.75','-1.75','-0','0','0.25','51544.5','58010.99999999999','1000000.25') for l in (-1080,-360,0,360,1080)])
    add('lmst_wrap',[['lmst','58010.25',l] for l in ('0','-0','77.1','-77.1','720.125')])
    add('precession', [['precession',t,dp,de] for t in (15019.5,51544.5,58010.25,88069.5) for dp,de in [(0,0),(1e-4,-2e-4),(-3e-5,7e-5)]])
    add('noops',[pulsar(n=-1),pulsar(1,n=0),['vector',-1],['vector',0],['tai',-1],['tai',0],['driver',-1],['driver',0],['tai',2],['driver',2]])
    for ecl in (0,1,2,-1):
        acts=[pulsar(ecl=ecl),pulsar(1,ecl=ecl),['debug',0,1,0]]
        for alpha,delta in [(0,0),(math.pi/2,0),(math.pi,math.pi/6),(1.23456789,-.87654321),(0,math.pi/2-1e-12)]:
            acts += [par(p,idx,0,val) for p in (0,1) for idx,val in enumerate((alpha+0.1*p,delta,12.3456789,-76.54321,0.12345,-.6789))]
            acts += [['vector',2],['vector',1]]
        add('vector_ecl_'+str(ecl),acts)
    add('vector_nonfinite_pm',[pulsar(),par(0,0,0,0),par(0,1,0,0),par(0,2,0,'nan'),par(0,5,0,'inf'),['vector',1]])
    add('ut1_interpolation',[['ut1',t,0] for t in (57999,58005,58005.0001,58007.5,58010,58010.0001,58042.5,58054.9999,58055,58070,58005,58070)])
    for warning in (-1,1,2):add('ut1_warning_'+str(warning),[['ut1',58000,warning],['ut1',58070,warning],['ut1',58000,0]])
    add('ut1_cache',[['ut1',58012.5,0],['copy','{case}/replacement','{case}/root/clock/ut1.dat'],['ut1',58012.5,0],['env','TEMPO2','{case}/other'],['ut1',58012.5,0]],
      {'replacement':ut1([(58000,[100]*6),(58030,[200]*6),(58060,[300]*6)]),'other/clock/ut1.dat':ut1([(58000,[900]*6),(58030,[800]*6),(58060,[700]*6)])})
    add('ut1_partial_rows',[['ut1',t,0] for t in (58005,58007.5,58014.5,58015)],{'root/clock/ut1.dat':ut1([(58000,[11,22]),(58010,[33,44]),(58020,[55,66])])})
    add('ut1_zero_termination',[['ut1',t,0] for t in (58005,58012.5,58054.5,58055)],{'root/clock/ut1.dat':ut1([(58000,[11,22,33,44,55,66]),(58030,[77,88,99,111,122,133]),(58060,[144,0,999,888,777,666])])})
    add('ut1_negative',[['ut1',58012.5,0]],{'root/clock/ut1.dat':ut1([(58000,[-100]*6),(58030,[-200]*6),(58060,[-300]*6)])})
    add('ut1_single_mjd_stop',[['ut1',58012.5,0]],{'root/clock/ut1.dat':ut1()+'59999\n'})
    add('ut1_missing',[['remove','{case}/root/clock/ut1.dat'],['ut1',58010,0]],status=1)
    add('ut1_row_limit',[['ut1',58010,0]],{'root/clock/ut1.dat':ut1([(40000+30*i,[12345]*6) for i in range(1000)])},1)
    real=blob('T2runtime/clock/ut1.dat').decode()
    # Validate the real reader's complete table independently before dispatch.
    add('ut1_pinned_real',[['ut1',t,0] for t in (50000,51544.5,55000)],{'root/clock/ut1.dat':real})
    for method in (1,2,0,3):
        add('tai_method_'+str(method),[pulsar(n=4,method=method),pulsar(1,n=2,method=2,override='UTC'),
          *[observation(i,clock=c) for i,c in enumerate((0,1,2,-1))],observation(6,site='B'),observation(7,site='B',clock=0),['tai',2],['tai',2]])
    add('tai_warnings_fallback',[pulsar(method=2,override='MISSING'),observation(t=58070),['tai',1],['tai',1]])
    add('tai_failure_prefix',[pulsar(n=2,method=2),observation(clock=0),observation(1),['remove','{case}/root/clock/ut1.dat'],['tai',1]],status=1)
    def iau(t='58010.25',utc='58010.249',file='eop',xyz=(6378237,100,200),zen=(1,2,3)):
        return ['iau',t,utc,file,*xyz,*zen]
    add('iau_direct',[['debug',0,1,0],iau(),iau('58010.499999999999999999','58010.499999999999999999'),iau('58010.500000000000000001','58010.500000000000000001'),iau('58011','58011',xyz=(0,0,0),zen=(-1,-2,-3)),iau('57989','57989')])
    add('iau_eop_cache',[iau(),iau(file='eop_other'),['copy','{case}/root/eop_other','{case}/root/eop'],iau(),['env','TEMPO2','{case}/other'],iau()],{'other/eop':eop(.2)})
    add('iau_missing_eop',[iau(file='missing')],status=1)
    for method in (1,2,0,3):
      for ecl in (0,1,2):
        add(f'driver_method_{method}_ecl_{ecl}',[pulsar(n=4,method=method,ecl=ecl),pulsar(1,n=2,method=method,ecl=ecl),
          *[observation(i,site=site,clock=clock) for i,(site,clock) in enumerate([('A',0),('B',1),('A',2),('A',-1),('A',1),('A',1),('POLE',1),('beta',1)])],
          *[['tt',i,2,37,32.184] for i in range(8)],['driver',2],['driver',2]])
    for warning in (0,1):
        add('driver_delay_'+str(warning),[pulsar(n=3,warn=warning),observation(delay=0),observation(1,delay=0),observation(2,delay=-1),['driver',1],observation(delay=1),['driver',1]])
    for idx,site in enumerate(('STL','STL_FBAT','STL_BAT','stl','StL','stl_fbat','Stl_Fbat','stl_bat','Stl_Bat')):
        add('satellite_spelling_'+str(idx),[pulsar(n=2),pulsar(1,n=1),observation(site=site),observation(1),observation(6),['flag',0,'-telx','299792458'],['flag',0,'-tely','599584916'],['flag',0,'-telz','-299792458'],['driver',1],['driver',1]])
    for ax in range(3):
      for mask in range(16):
        acts=[pulsar(),observation(site='STL'),par(0,9,0,'58010.24999')]
        acts += [par(0,6+ax,k,v,1 if mask&(1<<k) else 0) for k,v in enumerate(('1.12345678901234567890123456789012','2.25','-3.125','.0625'))]
        add(f'polynomial_axis_{ax}_mask_{mask}',acts+[['driver',1]])
    add('polynomial_nonone_flags',[pulsar(),observation(site='STL_FBAT'),par(0,6,0,9,2),par(0,7,0,11,-1),['driver',1]])
    add('satellite_no_epoch_constant',[pulsar(),observation(site='STL'),*[par(0,6+ax,0,ax+1) for ax in range(3)],['driver',1]])
    add('satellite_no_epoch_yz_velocity',[pulsar(),observation(t='0.0001',site='STL_FBAT'),par(0,7,0,1),par(0,7,1,2),par(0,8,0,3),par(0,8,1,4),['driver',1]])
    add('satellite_no_epoch_x_velocity',[pulsar(),observation(site='STL'),par(0,6,0,1),par(0,6,1,2),['driver',1]],status=1)
    add('satellite_flags_history',[pulsar(),observation(site='STL'),par(0,6,0,10),['flag',0,'-telx','299792458'],['flag',0,'-telx','bad'],['flag',0,'-tely','bad'],['flag',0,'-telz','-599584916suffix'],['flag',0,'-telz','bad'],['flag',0,'-TELX','100'],['driver',1],['driver',1],['clear_flags',0],['driver',1]])
    for ax in range(3):
      for mode in (0,1,2,3):
        acts=[pulsar(n=6),* [observation(i,t,site='STL_FBAT') for i,t in enumerate((58000,58010,58015,58020,58030,58040))],['offset',0,ax,3,mode,58010,1,58020,3,58030,8,58040,99],['driver',1]]
        add(f'offset_axis_{ax}_mode_{mode}',acts)
    add('offset_counts',[pulsar(n=1),observation(site='STL'),* [['offset',0,ax,count,2,58000,1,58020,2,58030,3,58040,4] for ax,count in enumerate((0,1,2))],['driver',1]])
    add('imag_shared',[pulsar(n=3),pulsar(1,n=1,method=2),observation(site='IMAG'),observation(1,site='imag_alias'),observation(2,site='imag'),observation(6,site='IMAG'),['lookup','IMAG'],['flag',0,'-telx','7000000'],['flag',0,'-telx','7100000'],['flag',0,'-tely','54321'],['flag',0,'-tely','bad'],['flag',0,'-telz','42'],['flag',0,'-telz','bad'],['driver',2],['lookup','imag_alias'],['clear_flags',0],['driver',2]])
    add('imag_defined_failure_prefix',[pulsar(n=2),observation(site='IMAG'),observation(1,site='UNKNOWN'),['flag',0,'-telx','7123456'],['driver',1]],status=1)
    add('driver_clock_cache',[pulsar(),observation(),['driver',1],['copy','{case}/replacement','{case}/root/clock/local.clk'],['driver',1]],{'replacement':'# LOCAL UTC 1\n57900 0\n58100 0\n'})
    add('satellite_no_epoch_x_high_order',[pulsar(),observation(t='0.0001',site='STL_FBAT'),par(0,6,0,1),par(0,6,2,2),par(0,6,3,3),['driver',1]])
    add('driver_tai_history',[pulsar(n=2,method=2),pulsar(1,n=1,method=1),observation(),observation(1,clock=0),observation(6),['tai',2],['driver',2],['tai',2],['driver',2]])
    add('derivative_nonone_flags',[pulsar(),observation(site='STL_FBAT'),par(0,9,0,58010),*[par(0,6+ax,j,j+1,1 if j==0 else 2) for ax in range(3) for j in range(4)],['driver',1]])
    # All filenames are unique under host case folding, independent of telescope text.
    assert len({c['id'].casefold() for c in out})==len(out)
    for c in out:
        assert len({p.casefold() for p in c['files']})==len(c['files'])
    return out

UNRUN = [
 dict(area='remove_white',reason='All three shifting strcpy branches overlap; no native oracle.'),
 dict(area='integer bounds',reason='NaN/infinite/out-of-range MJD and packed angles, and nmjdu1-51545 overflow.'),
 dict(area='UT1 table',reason='Empty/header-only/count-zero, overflow, unbounded scans, insufficient interpolation backing or all-zero sample table.'),
 dict(area='EOP',reason='Empty table, final-row slope, invalid brackets; no invented final derivative.'),
 dict(area='buffers',reason='Out-of-range counts, invalid/null backing, string overflow, stale resource pointers and unproved C++/Fortran aliases.'),
 dict(area='host',reason='Allocation/I/O faults, locale/platform/concurrency and TKlog portability not established.'),
]

def validate_inputs(case):
    """Independent fixed-column/scan/storage bounds, before native execution."""
    import re
    assert len({p.casefold() for p in case['files']})==len(case['files'])
    for name,data in case['files'].items():
        if name.endswith('ut1.dat') or name=='replacement' and data.startswith('ignored header'):
            lines=data.splitlines();assert len(lines)>=3
            entries=[];mjds=[];count=0
            for line in lines[2:]:
                assert len(line)<999
                vals=[]
                # scanf stops at the first noninteger, including END.
                remain=line[:57]
                for _ in range(7):
                    m=re.match(r'\s*([+-]?\d+)',remain)
                    if not m:break
                    v=int(m[1]);assert -(2**31)<=v<2**31
                    vals.append(v);remain=remain[m.end():]
                if len(vals)<2:break
                mjds.append(vals[0]);count+=1
                for v in vals[1:]:
                    if v==0:break
                    entries.append(v)
                assert count<=1000 and len(entries)+6<=10000
                if count==1000:
                    assert case['status']==1;break
                if 0 in vals[1:]:break
            assert count>0 and entries
            if count<1000:
                lo=mjds[0]+5;hi=mjds[-1]-5
                assert -(2**31)<=lo<2**31 and -(2**31)<=hi<2**31
                # Complete interior domain fits initialized sample backing.
                if hi>lo:assert math.ceil((hi-lo)/5)-1+3 < len(entries)
    for a in case['actions']:
        op=a[0]
        if op=='white':assert len(a[1])<64 and not re.search(r'^[ \t]|[ \t][ \t]|\n',a[1])
        if op=='lmst':assert -(2**31)+51545<int(float(a[1]))<2**31
        if op=='ang':
            f=float(a[2]);ia=math.floor(f/1e4);ib=math.floor((f-ia*1e4)/100)
            assert -(2**31)<=ia<2**31 and -(2**31)<=ib<2**31
        if op=='iau':assert all(0<float(x)+2400000.5<2**31 for x in a[1:3])
        if op=='pulsar':assert 0<=int(a[1])<2 and -1<=int(a[2])<=6
        if op=='observation':assert 0<=int(a[1])<12 and len(a[3])<100
        if op=='offset':
            assert 0<=int(a[3])<=4
            times=list(map(float,a[5::2]));assert all(y>x for x,y in zip(times,times[1:]))
