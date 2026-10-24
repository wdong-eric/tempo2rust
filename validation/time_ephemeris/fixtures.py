"""Authored inputs and independently decoded binary layouts; no Rust oracle."""
import math
import struct
from pathlib import Path
from profile import ROOT, sha, blob
START=2450000.5

def ifte(order='>',ncf=4,na=4,scale=1,records=6,ncon=2):
    offset=max(3+ncf*na,41-3*ncf*na)
    count=offset-1+3*ncf*na
    assert 40<=count<=322 and 0<=ncf<=18 and na>=1
    data=bytearray(count*8*(2+records))
    data[:252]=b'TITLE '*42;data[252:264]=b'VERSN L_C   '
    struct.pack_into(order+'3di6i',data,264,START,START+16,4,ncon,3,ncf,na,offset,ncf,na)
    struct.pack_into(order+'2d',data,count*8,405.875,1.48082686742e-8)
    for r in range(records):
        values=[700+r+i/32 for i in range(count)]
        values[:2]=[START+4*r,START+4*(r+1)]
        for start,ncm in [(2,1),(offset-1,3)]:
            for sub in range(na):
                for comp in range(ncm):
                    for k in range(ncf):
                        values[start+(sub*ncm+comp)*ncf+k]=scale*(1+r*2+sub/8+comp/32)*(2**(-25-k))
        struct.pack_into(order+str(count)+'d',data,(r+2)*count*8,*values)
    # Separate decoder checks absolute record/descriptor offsets and constants.
    h=struct.unpack_from(order+'3di6i',data,264)
    assert h==(START,START+16,4,ncon,3,ncf,na,offset,ncf,na)
    assert struct.unpack_from(order+'2d',data,count*8)==(405.875,1.48082686742e-8)
    assert len(data)==(2+records)*count*8
    return bytes(data)

def fb(ncf=4,scale=1,width=4):
    assert -1<=ncf<=16 and width>0
    data=struct.pack('>2d2i5d',START,START+16,width,ncf,11,12,13,14,15)
    for r in range(8):
        data+=struct.pack('>'+str(max(ncf,0))+'d',*[scale*(r+1)*(k+1)/1024 for k in range(max(ncf,0))])
    assert struct.unpack('>2d2i5d',data[:64])==(START,START+16,width,ncf,11,12,13,14,15)
    return data

def catalog():
    cases=[]
    def add(name,actions,files=None,status=0,kind='defined',note=''):
        assert name.casefold() not in {c['id'].casefold() for c in cases}, 'case-insensitive artifact collision'
        cases.append(dict(id=name,actions=actions,files=files or {},status=status,classification=kind,note=note))
    A={'A/ephemeris/TIMEEPH_short.te405':ifte(),'A/ephemeris/TDB.1950.2050':fb()}
    B={'B/ephemeris/TIMEEPH_short.te405':ifte('<',scale=3),'B/ephemeris/TDB.1950.2050':fb(scale=7)}
    for mode in ['4','int','ints','8','double','8N','doubles']:
        counts=[-1,0,2] if mode in ['ints','8N','doubles'] else [1]
        for n in counts:add('swap_'+mode+'_'+str(n),[['swap',mode,n,1],['swap',mode,n,2]],kind='host-bytes')
    for order,label in [('>','big'),('<','little')]:
        actions=[['close'],['init','@eph']]
        for d,f in [(START,0),(2450000,0.5),(START+1,-1),(START,-0.0),(START+1,-0.25),(START,0.5),(START,1),(START,3.999),(START,4),(START,16),(START,16.125),(START,20)]:
            actions.append(['get','values',d,f,1])
        for mode,tail in [('values',[0]),('values',[-4]),('dt',[]),('dot',[]),('dtboth',[0,1]),('dtboth',[0,0]),('v',[]),('vdot',[]),('vboth',[0,3]),('vboth',[0,0]),('vboth',[0,1]),('vboth',[1,0])]:
            actions.append(['get',mode,START,1.5,*tail])
        actions += [['ifdelta','50001.125'],['close'],['init','@eph'],['get','dt',START,0.5],['close']]
        add('ifte_'+label,actions,{'eph':ifte(order)})
    for n in [0,1,2,3,7,18]:
        add('ifte_count_'+str(n),[['init','@eph'],['get','values',START,0.5,1],['get','values',START,1.5,2],['close']],{'eph':ifte(ncf=n,na=1)})
    add('ifte_reinit',[['init','@eph'],['get','values',START,0.25,1],['save'],['init','@other'],['get','values',START,0.5,1],['get','values',START,0.5,2],['cleanup',0],['close']],{'eph':ifte(),'other':ifte('<',scale=2)})
    negative=bytearray(ifte());struct.pack_into('>d',negative,280,-4)
    add('ifte_negative_step_cold',[['init','@eph'],['get','values',START,12,1],['get','values',START,12,2],['close']],{'eph':bytes(negative)},note='irec=-1 exactly; skips read, valid static-zero coefficient backing')
    add('ifte_negative_step_retained',[['init','@positive'],['get','values',START,4,1],['save'],['init','@negative'],['get','values',START,12,1],['get','values',START,12,2],['close'],['cleanup',0]],{'positive':ifte(),'negative':bytes(negative)},note='irec=-1 reuses retained initialized buf from earlier file')
    add('ifte_missing',[['init','@missing']],status=1)
    add('ifte_bad_ncon',[['init','@eph']],{'eph':ifte(ncon=3)},status=1)
    add('ifte_before',[['init','@eph'],['get','dt',START,-0.01]],{'eph':ifte()},status=1)
    add('ifte_short',[['init','@eph'],['get','dt',START,0]],{'eph':ifte(records=0)},status=1)
    add('ifte_replace',[['init','@eph'],['get','dt',START,0],['replace','@eph','@other'],['get','dt',START,0],['get','dt',START,4],['save'],['init','@eph'],['get','dt',START,0],['cleanup',0],['close']],{'eph':ifte(),'other':ifte(scale=7)},kind='host-stdio')
    add('ifte_truncate',[['init','@eph'],['get','dt',START,0],['truncate','@eph',0],['get','dt',START,0],['get','dt',START,20]],{'eph':ifte()},status=1,kind='host-stdio')
    for n in [0,1,2,3,4,18]:
        add('interp_count_'+str(n),[['interp',n,1,1,2,t,4,'simple'] for t in [0,.25,.5,.5,1]])
    for ncm in [0,2,3,4]:
        add('interp_components_'+str(ncm),[['interp',4,ncm,4,flag,t,8,'dense'] for flag,t in [(1,0),(0,.125),(2,.25),(3,.5),(2,1)]])
    add('interp_retention',[['interp',18,3,2,2,.125,4,'dense'],['interp',2,1,1,1,.25,4,'dense'],['interp',18,2,1,2,.25,4,'dense'],['interp',3,1,1,2,.75,4,'simple']])
    add('interp_extrapolate',[['interp',3,1,1,2,t,4,'simple'] for t in [-.125,1.125]],note='na=1 gives l=0 for both finite extrapolated times')
    add('interp_zero_interval',[['interp',3,1,1,2,.25,0,'simple']],note='Floating division by zero; no resulting value is cast to int')
    add('interp_negative_interval',[['interp',3,1,1,2,.25,-4,'simple']])
    primitive=lambda order:struct.pack(order+'ifd',0x01020304,1.25,-2.5)+b'ABC\x00\xff'
    for order,label,swap in [('>','big',1),('<','little',0)]:
        add('fortran_'+label,[['open','@primitive suffix remains'],['swapmode',swap],['read','int'],['read','float'],['read','double'],['read','chars',0],['read','chars',-3],['read','chars',4],['read','char'],['read','char'],['read','int'],['read','float'],['read','double'],['fclose']],{'primitive':primitive(order)},kind='host-union-EOF')
        add('fortran_record_'+label,[['open','@primitive'],['swapmode',swap],['read','record'],['fclose']],{'primitive':primitive(order)},kind='host-native-record')
    add('fortran_missing',[['open','@missing suffix']],kind='host-diagnostic')
    add('fortran_partial',[['open','@partial'],['read','double'],['read','chars',3],['fclose']],{'partial':b'\x3f\xf0'},kind='host-union-EOF')
    for n in [-1,0,1,2,3,8,16]:
        files={'A/ephemeris/TDB.1950.2050':fb(ncf=n)}
        add('fb_count_'+str(n),[['root','@A'],*([['fb',t] for t in [50000,50000.499,50000.5,50000.501,50002,50004,50016]])],files)
    add('fb_cache',[['root','@A'],['fb',50000],['fb',50000.125],['root','@B'],['fb',50000],['fb',50004],['root','@A'],['fb',50004],['fb',50000],['fb',49996],['fb',49996.125]],{**A,**B})
    add('fb_count_tail',[['root','@A'],['fb',50000],['root','@B'],['fb',50000],['fb',50004]],{'A/ephemeris/TDB.1950.2050':fb(ncf=4),'B/ephemeris/TDB.1950.2050':fb(ncf=8,scale=2)})
    add('fb_nr1_cold',[['root','@A'],['fb',49996]],A)
    add('fb_before',[['root','@A'],['fb',49992]],A,status=1)
    add('fb_after',[['root','@A'],['fb',50016.125]],A,status=1)
    for eph in [1,2,0,-7]:
        for units in [1,2,0]:
            add('tt_e'+str(eph)+'_u'+str(units),[['root','@A'],['p',0,'timeEphemeris',eph],['p',0,'units',units],['tt',1,'present']],A)
    for dilation in [0,-1,2]:
        add('tt_dilation_'+str(dilation),[['root','@A'],['p',0,'dilateFreq',dilation],['tt',1,'present']],A)
    for calceph in [0,1,2]:
        add('tt_multi_calc_'+str(calceph),[['root','@A'],['p',0,'useCalceph',calceph],['p',1,'useCalceph',calceph],['p',1,'timeEphemeris',0],['p',1,'units',2],['tt',2,'present']],A)
    add('tt_mixed_calceph_backing',[['root','@A'],['p',0,'nobs',1],['p',1,'nobs',4],['p',1,'timeEphemeris',2],['p',1,'useCalceph',1],['p',1,'units',2],['tt',2,'present']],A,note='p0 unused logical slots remain initialized backing for p1 CALCEPH reads')
    add('tt_delta_retention',[['root','@A'],['p',0,'timeEphemeris',2],['p',1,'timeEphemeris',0],['tt',2,'present'],['tt',2,'present']],A)
    for tel in ['PKS','STL','stl','STLx']:
        add('tt_bypass_'+({'STL':'STL_exact','stl':'stl_lower'}.get(tel,tel)),[['root','@A'],*([['obs',0,i,'clockCorr',0] for i in range(3)]),*([['obs',0,i,'telID',tel] for i in range(3)]),['tt',1,'present']],A)
    add('tt_negative_clock',[['root','@A'],['obs',0,0,'clockCorr',-1],['obs',0,0,'nclock_correction',-1],['obs',0,1,'nclock_correction',0],['clock',0,2,0,1e16],['clock',0,2,1,-1e16],['clock',0,2,2,.125],['tt',1,'present']],A)
    for n in [-1,0,2]:
        add('tt_count_'+str(n),[['root','@A'],['p',0,'nobs',0],['p',1,'nobs',-1],['tt',n,'present']],A)
    add('tt_missing_zero',[['root','@missingroot'],['tt',0,'missing']],status=1)
    add('tt_missing_bypass',[['root','@missingroot'],['obs',0,0,'clockCorr',0],['tt',1,'missing']],status=1)
    add('tt_repeat_fb',[['root','@A'],['p',0,'timeEphemeris',2],['tt',1,'present','fb'],['root','@B'],['tt',1,'present','fb'],['p',0,'nobs',1],['obs',0,0,'sat',50005],['tt',1,'present','fb']],{**A,**B})
    add('tt_cvs',[['root','@A'],['global','cvs',1],['tt',1,'present']],A,kind='host-diagnostic')
    add('tt_zero_vectors',[['root','@A'],*([['vector',0,i,v,j,0] for i in range(3) for v,n in [('earth',6),('site',3),('obs',3)] for j in range(n)]),['tt',1,'present']],A)
    real={n:(ROOT/'tempo2/T2runtime/ephemeris'/n).read_bytes() for n in ['TIMEEPH_short.te405','TDB.1950.2050']}
    for n,b in real.items():assert b==blob('T2runtime/ephemeris/'+n), 'real resource differs from pin'
    assert struct.unpack_from('>3di6i',real['TIMEEPH_short.te405'],264)==(2438736.5,2469808.5,32,2,3,7,8,59,7,8)
    assert struct.unpack_from('>2d2i',real['TDB.1950.2050'])==(2433282,2469794,16,8)
    files={'R/ephemeris/'+n:b for n,b in real.items()}
    add('real_helpers',[['root','@R'],['initroot'],['ifdelta',58000],['get','vboth',2458000.5,0,0,3],['fb',58000],['fb',58000.5],['fb',58016],['close']],files)
    add('real_tt',[['root','@R'],*([['obs',p,i,'sat',58000+i*.125] for p in range(2) for i in range(4)]),['p',1,'timeEphemeris',2],['p',1,'units',2],['tt',2,'present']],files)
    return cases

def membership(cases):
    return [{**{k:v for k,v in c.items() if k!='files'},'files':{n:dict(size=len(b),sha256=sha(b)) for n,b in c['files'].items()}} for c in cases]
