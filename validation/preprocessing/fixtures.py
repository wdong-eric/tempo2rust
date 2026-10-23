"""Declared bounded caller histories. Generated before any native case execution."""
from profile import HERE,save

def A(*v):return list(map(str,v))
def param(n,v,j=0,p=0,pre=17,err=0.125,pe=0.25,flag=0,isset=1):return A('param',p,'param_'+n,j,v,pre,err,pe,isset,flag)
def obs(i,k,v,p=0):return A('obs',p,i,k,v)
def setting(k,v,p=0):return A('set',p,k,v)
def wrapper(*args,n=1):return A('wrapper',n,'-setdm','dm.txt',*args)
def stage2(p=0,start=0,n=0,trim=0,values=()):return A('stage2',p,start,n,trim,*values)

def cases():
    cs=[]
    def add(id,actions,files=None,host=False,exit=0,note=''):
        fs={'dm.txt':'# explicit ndm=0 input\n'};fs.update(files or {})
        cs.append(dict(id=id,actions=actions,files={n:b.encode().hex() for n,b in fs.items()},outputs=[],returncode=exit,classification=('host observation' if host else 'normal source exit' if exit else 'defined result'),contract=note))
    add('simple_repeat',[A('simple',0),A('simple',0),A('simple',1)])
    add('wrapper_repeat',[wrapper(n=2),wrapper(n=2)])
    add('wrapper_zero',[A('wrapper',0,'-name','unused','-global','missing')],exit=0)
    # Stage1 frequency and emulation controls.
    for nw in [0,1,4,-4]:
        add('wave_'+str(nw),[param('wave_om',0),param('wave_dm',0),setting('nWhite',nw),setting('nWhite_dm',13),obs(0,'deleted',1),A('stage1',0,0,-1),A('stage1',0,0,-1)])
    for count in [0,1]:
        add('wave_span_'+str(count),[setting('nobs',count),param('wave_om',0),param('wave_dm',0),setting('nWhite',4),A('stage1',0,0,-1)])
    add('wave_deleted',[param('wave_om',0),param('wave_dm',0),setting('nWhite',2)]+[obs(i,'deleted',1) for i in range(4)]+[A('stage1',0,0,-1)])
    add('wave_sentinel',[param('wave_om',0),param('wave_dm',0),setting('nWhite',2),obs(0,'sat',-1),obs(1,'sat',-2),obs(2,'sat',2),obs(3,'sat',0),A('stage1',0,0,-1)])
    for eph in [None,2,5,4.999,6]:
        for mode in [0,1]:
            add(f'emulation_{eph}_{mode}',([] if eph is None else [param('ephver',eph)])+[A('stage1',0,mode,-1),A('stage1',1,1-mode,-1),A('stage1',0,mode,-1)])
    add('last_stored',[obs(0,'sat',59000),obs(3,'deleted',1),A('stage1',0,0,20)])
    add('units100',[setting('units',100),A('stage1',0,0,-1)])
    # Stage2 order, repeat append and source selector defects.
    selectors=['MJD 58000 58002','FREQ 700 1400','NAME CALLER_0_1 unused','TEL PKS unused','-be B unused','SCALED -n unused','UNKNOWN a b']
    for typ in ['ordinary','fd']:
        for k,sel in enumerate(selectors):
            add(f'jump_{typ}_{k}',[A('jump',0,typ,sel),obs(1,'deleted',1),stage2(),stage2()],host=False)
    add('stage2_order',[param('f',2),obs(0,'sat',58000),obs(0,'phaseOffset',2),A('offset',0,1,0,0,'58000.000005',0,'0',''),A('jump',0,'ordinary','MJD 58000 58000.00003'),A('flag',0,0,'-to','10'),A('flag',0,0,'-to','-2'),stage2(),stage2()])
    for field in ['f1','f2','t1','t2']:
        vals={'f1':0,'f2':0,'t1':0,'t2':0};vals[field]=700 if field.startswith('f') else 58000
        add('offset_bound_'+field,[A('offset',0,86400,*vals.values(),'0',''),stage2()])
    for k,(site,flags) in enumerate([('pks_alias',''),('JBO',''),('missing',''),('0','-be A -n 3'),('0','-be X -n 3'),('0','-be X'),('0','12 -3 -be B'),('0','! -be A')]):
        add('offset_'+str(k),[A('offset',0,3,0,0,0,0,site,flags),stage2(),stage2()],host=site in ['pks_alias'])
    for field,txt in [('filterStr','-be A B'),('filterStr','-be A -n 3'),('passStr','-be A -n 3'),('passStr','-absent X'),('passStr','-be')]:
        add('filter_'+str(len(cs)),[setting(field,txt),stage2()])
    add('filter_pass_deleted',[setting('filterStr','-be A'),setting('passStr','-be A'),A('jump',0,'ordinary','-be A x'),stage2()])
    for mode in [0,1,2]:
        add('zero_error_'+str(mode),[setting('fitMode',mode),obs(1,'toaErr',0),stage2()],exit=1 if mode==1 else 0)
    add('phase_divzero',[param('f',0),obs(0,'phaseOffset',1),stage2()])
    for trim in [0,1]:
        add('dm_direct_'+str(trim),[obs(0,'sat','57999.5'),obs(1,'sat','58000.499999999999999999999'),obs(2,'sat','58000.5'),obs(3,'sat','58004.5'),stage2(start=58000,n=4,trim=trim,values=[0.1,0.2,0.3,0.4]),stage2(start=58000,n=4,trim=trim,values=[0.1,0.2,0.3,0.4])])
    for start,n,values in [(0,3,[1,2,3]),(58000,0,[]),(58000.003,3,[1,2,3])]:
        add('dm_guard_'+str(len(cs)),[stage2(start=start,n=n,values=values)])
    add('dm_capacity',[obs(0,'sat',58001),obs(0,'nFlags',39),stage2(start=58000,n=3,values=[0,2,3])],exit=1)
    # FJUMP is bounded by observation flag and jump capacities, including repeats.
    add('fjump_value_only',[setting('fjumpID','-be'),A('flag',0,2,'-other','B'),obs(2,'deleted',1),A('flag',0,3,'-be','B'),A('stage3',0),A('stage3',0)])
    add('fjump_reverse',[setting('fjumpID','-be'),A('reverse',0),A('stage3',0)])
    add('fjump_absent',[setting('fjumpID','-absent'),A('stage3',0)])
    add('phase_assignment',[A('phase',0,'57999',7),A('phase',0,'58001',2),A('phase',0,'59000',0),A('stage3',0),A('reverse',0),A('stage3',0)])
    add('phase_narrow',[obs(1,'sat','58001.000000000000000001'),A('phase',0,'58001',7),A('stage3',0)])
    add('phase_empty',[setting('nobs',0),A('phase',0,58000,4),A('stage3',0)])
    # Selection: full files and direct helpers, all modes, keyword cases and EOF.
    for kind,lo,hi in [('MJD',58001,58002),('FREQ',700,1400),('TOAERR',2,3)]:
        for op in ['PASS','REJECT']:
            for suffix in ['', '\n']:
                add(f'select_{op}_{kind}_{bool(suffix)}',[obs(0,'deleted',1),A('select','select.txt',2),A('select','select.txt',2)],{'select.txt':f'{op} {kind} {lo} {hi}'+suffix})
    add('select_flags',[A('select','select.txt',2)],{'select.txt':'PASS -be A B\n'})
    add('select_flags_fail',[A('select','select.txt',1)],{'select.txt':'PASS -absent A\n'})
    add('select_capacity',[A('select','select.txt',1)],{'select.txt':'PASS -be '+' '.join('v'+str(i) for i in range(20))+'\n'},exit=1)
    add('select_missing',[A('select','absent',2)],exit=1)
    add('select_case',[A('select','select.txt',2)],{'select.txt':'pass MJD 0 1\nPASS mjd 0 1\nUNKNOWN thing\n# comment\n'})
    add('select_narrow',[obs(0,'sat','58000.000000000000000001'),A('select','select.txt',1)],{'select.txt':'REJECT MJD 58000 58000\n'})
    for cond in ['EXIST PASS unused','EXIST REJECT unused','EXIST unknown unused','> 1 REJECT','< 2 REJECT','= 1 REJECT','>= 1 REJECT','> 1 PASS']:
        for id in ['-n','-missing']:
            add('logic_'+str(len(cs)),[obs(0,'deleted',1),A('logic',f'LOGIC {id} {cond}',2)])
    add('logic_nonnumeric',[A('logic','logic -be > 1 REJECT',2)])
    for mode,lo,hi in [('mjdpass',58001,58002),('mjdreject',58001,58002),('reject',0,0),('errpass',2,3),('errreject',2,3),('freqpass',700,1400),('freqreject',700,1400),('UNKNOWN',0,0)]:
        add('process_'+mode,[obs(1,'deleted',1),A('process',f'PROCESS -be A {mode} {lo} {hi}',2)])
    for mode in ['only','toaerr','1','2','unknown']:
        for thresh in ['', ' 0',' 60',' 61']:
            for rev in [False,True]:
                setup=[obs(0,'sat',58000),obs(1,'sat','58000.0006944444444444444444444444444444'),obs(2,'sat','58000.0003'),obs(3,'sat',58001),obs(0,'toaErr',2),obs(1,'toaErr',2)]
                if rev:setup+=[A('reverse',0)]
                add('simul_'+str(len(cs)),setup+[A('simul',f'PROCESS SIMUL -be A B {mode}{thresh}',1),A('simul',f'PROCESS SIMUL -be A B {mode}{thresh}',1)])
    add('simul_bad',[A('simul','short',2)])
    add('simul_carried',[obs(0,'sat',58000),obs(1,'sat',58000),obs(2,'sat',58000),obs(3,'sat',58000),A('flag',0,0,'-be','B'),A('simul','PROCESS SIMUL -be A B only',1)])
    add('select_dispatch',[A('select','select.txt',2)],{'select.txt':'LOGIC -n > 2 REJECT\nPROCESS -be A errpass 1 3\nPROCESS SIMUL -be A B only 100000\n'})
    # White noise raw token order, complete tokens and -tobs semantics.
    for i,txt in enumerate(['JITTER -be A -tobs 0.000003 10\n','START_TIME_JITTER -be B 0.000004\n','TOAERR_ZERO JITTER -be A -tobs 0.000003 10\n','JITTER -be A -tobs 0.000003 10 TOAERR_ZERO\n','ignored JITTER -be A -tobs 0.000003 -10\n']):
        add('white_'+str(i),[setting('whiteNoiseModelFile','noise.txt'),A('flag',0,0,'-tobs','10'),A('flag',0,0,'-tobs','20'),A('flag',0,2,'-tobs','0'),obs(2,'deleted',1),A('white',0),A('white',0)],{'noise.txt':txt})
    add('white_missing',[setting('whiteNoiseModelFile','absent'),A('white',0)],exit=1)
    noise=[A('noise',0,'T2equad','-be','A',3),A('noise',0,'T2efac','-be','A',2),setting('T2globalEfac',1.5),A('noise',0,'TNEF','-be','A',2),A('noise',0,'TNEQ','-be','A',-6),setting('TNGlobalEF',1.25),setting('TNGlobalEQ',-6.5)]
    add('noise_order',noise+[A('flag',0,0,'-be','A'),wrapper(),wrapper()])
    for tf in [[],['UNKNOWN'],['10','20'],['10','UNKNOWN'],['0'],['-1']]:
        add('tnsq_'+str(len(cs)),[A('noise',0,'TNSQ','-be','A',-6)]+[A('flag',0,0,'-tobs',t) for t in tf]+[wrapper()])
    add('tnsecorr',[setting('nTNSECORR',1)]+[A('flag',0,i,'-tobs','UNKNOWN' if i%2 else '20') for i in range(4)]+[A('flag',0,0,'-TOBS','30'),wrapper()])
    # Wrapper argument history and final PAR sequencing.
    add('wrapper_options',[param('start',57999,flag=1),param('finish',58005,flag=2),param('pepoch',58000,flag=1),param('posepoch',58000,flag=1),param('dmepoch',58000,flag=1),param('track',1,flag=1),A('jump',0,'ordinary','-be B x'),A('jump',0,'fd','-be A x'),wrapper('-nofit','-fit','F1','-fit','PEPOCH','-fit','unknown','-set','F1','0.00001','-set','NITS','3','-name','new','-clock','CLOCK_A','-clock','CLOCK_B','-simulate','0.2','-last','10'),wrapper('-nojump','-set','F0','3')])
    add('wrapper_nonconsumed',[wrapper('-name','-nofit','-fit','-tempo1','-simulate','0','-set','UNRECOGNIZED','12')])
    add('wrapper_link_chain',[param('stig',0.5),param('kin',10),A('link',0,'param_sini','param_kin'),A('link',0,'param_kin','param_stig'),wrapper(),wrapper()])
    add('wrapper_dm_warning',[param('dm',0.1,j=2),wrapper(),wrapper()],host=True)
    add('wrapper_white_noise', [setting('whiteNoiseModelFile','noise.txt')]+noise+[wrapper()],{'noise.txt':'START_TIME_JITTER -be A 0.000004\n'})
    add('wrapper_modify',[obs(2,'sat',58001),wrapper('-modify','modify.txt','-select','select.txt',n=2)],{'modify.txt':'-be A 58000 58002 C\n','select.txt':'PASS -be A C\n'})
    add('wrapper_modify_missing',[wrapper('-modify','absent')],exit=1)
    add('wrapper_global',[wrapper('-set','F0','7','-name','local','-global','global.par',n=2),wrapper(n=2)],{'global.par':'PSR GLOBAL\nF0 11 2 0.5\nNITS 8\n'})
    add('wrapper_global_missing',[wrapper('-global','absent')],exit=1)
    for trim in [False,True]:
        for suffix in ['', '\n','# end\n']:
            add('wrapper_dm_'+str(len(cs)),[setting('nobs',5),wrapper(*(['-trimonly'] if trim else [])),wrapper(*(['-trimonly'] if trim else []))],{'dm.txt':'# header\n58000.001 0.1\n58001 0.2\n58002 0.3\n58003 0.4'+suffix})
    for option,values in [('-fit',['F0']),('-set',['F0','2'])]:
        add('wrapper_capacity_'+option,[A('wrapper',0,*sum(([option,*values] for _ in range(101)),[]))],exit=1)
    for name in ['ok','noversion','wrong']:
        add('plugin_'+name,[A('plugins','../../plugins'),wrapper('-splug',name,n=2),wrapper('-splug',name,n=2)] if name!='wrong' else [A('plugins','../../plugins'),wrapper('-splug',name)],exit=1 if name=='wrong' else 0)
    add('plugin_search',[A('plugins','../../missing','../../plugins'),wrapper('-splug','ok','-select','absent')],host=True)
    # Unit helpers: indices, mixed precision constants, untouched arrays, source bug.
    for fn in ['scale','mjd']:
        for j in [0,1]:
            for f in ['0','1','1.0000000155051979176','-2','inf']:
                add(f'{fn}_{j}_{f}',[param('bpjep',58000,j=0,pre=57000),param('bpjep',59000,j=1,pre=56000),A(fn,0,'param_bpjep',j,f)])
    for fr,to in [(2,1),(1,2),(1,1),(2,2),(0,1),(2,100),(100,2)]:
        for site in ['@','PKS']:
            add(f'units_{fr}_{to}_{site}',[A('seed_units',0),setting('tzrsite',site),A('units',0,fr,to),A('units',0,to,fr)])
    add('units_site_case',[A('seed_units',0),setting('tzrsite','@ '),A('units',0,2,1)])
    # Epoch arithmetic and wrappers, original frame controls deliberately differ.
    ep=[param('f','1e-10',j=1),param('f','2e-18',j=2),param('dm',10),param('dm',0.1,j=1),param('dm',0.01,j=2)]
    for keyword in ['CENTER','CENTRE','left','RIGHT','58001.25suffix','-1']:
        add('epoch_'+keyword,ep+[A('epochstr',0,keyword),A('epochstr',0,keyword)])
    for flag in [1,2,0,3]:
        add('epoch_selection_'+str(flag),ep+[param('start',58001,flag=flag),param('finish',58002,flag=flag),param('start',59000,p=1,flag=1),obs(0,'deleted',1,p=1),A('epochstr',1,'CENTER')])
    for mode in ['empty','deleted','sentinel']:
        setup=[setting('nobs',0)] if mode=='empty' else [obs(i,'deleted',1) for i in range(4)] if mode=='deleted' else [obs(0,'sat',-1),obs(1,'sat',-2),obs(2,'sat',1),obs(3,'sat',2)]
        add('epoch_'+mode,ep+setup+[A('epochstr',0,'CENTER')])
    for flag in [1,2]:
        key='sat' if flag==1 else 'bat'
        add('epoch_boundary_'+str(flag),[param('start',58001,flag=flag),param('finish',58002,flag=flag),obs(0,key,'58000.999999999990000000001'),obs(1,key,'58000.999999999989999999999'),obs(2,key,'58002.000000000009999999999'),obs(3,key,'58002.000000000010000000001'),A('epochstr',0,'CENTER')])
    for mask in range(16):
        pos=[param(n,(j+1)*2) for j,n in enumerate(['pmra','pmdec','pmra2','pmdec2']) if mask&(1<<j)]
        add('epoch_motion_'+str(mask),ep+pos+[param('f','1e-25',j=3),param('f','1e-30',j=4),param('dm',0.5,j=3),A('epoch',0,'58365.25'),A('epoch',0,'58366.25')])
    add('epoch_fitwaves_binary',ep+[param('waveepoch',58000),param('waveepoch_dm',58000),param('pb',10),param('t0',58000),param('pbdot','1e-12'),A('epoch',0,58010)])
    add('cvs',[setting('cvs',1),A('simple',0),A('units',0,1,1),wrapper()])
    add('debug',[setting('debug',1),A('simple',0),wrapper()])
    add('wrapper_epoch_multi',[param('start',58010,flag=1),param('finish',58012,flag=1),param('start',59000,p=1,flag=1),wrapper('-epoch','CENTER',n=2)])
    add('wrapper_forbidden',[param(n,58000,flag=1) for n in ['pepoch','posepoch','dmepoch','track']]+[wrapper()])
    add('wrapper_overwritten_label',[wrapper('-fit','DAOP','-fit','IPERHARM','-set','IPERHARM','3','-set','DAOP','9')])
    add('wrapper_multi_select_order',[setting('nobs',1),setting('nobs',1,p=1),obs(0,'sat',58011),obs(0,'phaseOffset',259200,p=1),wrapper('-select','select.txt',n=2)],{'select.txt':'PASS MJD 58011 58011\n'})
    add('wrapper_dm_dates',[setting('nobs',5),wrapper()],{'dm.txt':'# explicit start\n58000 0.1\n59000 0.2\n51000 0.3\n58003 0.4\n'})
    add('simul_exact_threshold',[setting('nobs',2),obs(0,'sat',58000),obs(1,'sat','58000.5'),A('simul','PROCESS SIMUL -be A B only 43200',1)])
    add('simul_only_carried',[setting('nobs',2),obs(1,'sat',58000),obs(1,'nFlags',0),A('flag',0,1,'-be','A'),A('simul','PROCESS SIMUL -be A B only',1)])
    for flag in [1,2]:
        add('epoch_cross_'+str(flag),[param('start',58010,flag=flag),param('finish',58012,flag=flag),param('start',59000,p=1,flag=1),A('epochstr',1,'CENTER')])
    add('simul_order_chain',[setting('nobs',3),obs(0,'sat',58000),obs(1,'sat','58000.0005787037037037037037037037'),obs(2,'sat','58000.0011574074074074074074074074'),obs(0,'toaErr',3),obs(1,'toaErr',2),obs(2,'toaErr',1),A('simul','PROCESS SIMUL -be A B toaerr 60',1)])
    add('epoch_precision',ep+[A('epoch',0,'58001.0000000000000000001')])
    add('fjump_capacity',[A('fjump_capacity'),A('stage3',0)],exit=1)
    add('white_no_tobs',[setting('whiteNoiseModelFile','noise.txt'),A('white',0)],{'noise.txt':'JITTER -be A -tobs 0.000003 10\n'})
    add('wave_reverse',[param('wave_om',0),param('wave_dm',0),setting('nWhite',4),A('reverse',0),A('stage1',0,0,-1)])
    add('epoch_reverse',[A('reverse',0),A('epochstr',0,'CENTER')])
    add('offset_inside_upper',[A('offset',0,2,0,2101,0,58004,'0',''),stage2()])
    add('offset_punctuation',[A('offset',0,2,0,0,0,0,'0','-! X'),stage2()])
    add('dm_null_guard',[A('dmnull')])
    add('wrapper_dm_ignored_line',[wrapper()],{'dm.txt':'# assigned file\njunk\n'})
    for kind in ['ordinary','fd']:
        add('jump_lower_'+kind,[obs(0,'sat',57999),obs(0,'freq',699),A('jump',0,kind,'MJD 58000 58002'),A('jump',0,kind,'FREQ 700 1400'),stage2()])
    add('select_frequency_lower',[obs(0,'freq',699),A('select','select.txt',1)],{'select.txt':'PASS FREQ 700 1400\nREJECT FREQ 700 1400\n'})
    add('select_keyword_fallthrough',[A('select','select.txt',2)],{'select.txt':'REJECT OTHER 1 2\nPROCESS OTHER a b\nlogic token x y\nlogic -n EXIST PASS unused\nPASS -be\n'})
    add('process_boundaries',[obs(0,'toaErr',4),obs(0,'freq',699),A('process','PROCESS -be A errreject 2 3',1),A('process','PROCESS -be A errpass 2 3',1)])
    for mode in ['freqpass','freqreject']:
        add('process_lower_'+mode,[obs(0,'freq',699),A('process','PROCESS -be A '+mode+' 700 1400',1)])
    add('process_deleted',[obs(0,'deleted',1),A('process','PROCESS -be A reject 0 0',1)])
    for prefix in ['process SIMUL','PROCESS simul']:
        add('simul_prefix_'+prefix,[A('simul',prefix+' -be A B only 100000',1)])
    add('simul_deleted',[obs(0,'deleted',1),obs(1,'sat',58000),obs(2,'sat',58000),A('simul','PROCESS SIMUL -be A B only',1)])
    add('wrapper_debug_hooks',[setting('debug',1),A('plugins','../../plugins'),wrapper('-select','select.txt'),wrapper('-splug','ok')],{'select.txt':'PASS MJD 57999 58003\n'})
    add('plugin_quiet',[setting('quiet',1),A('plugins','../../plugins'),wrapper('-splug','ok')])
    for typ in ['T2efac','T2equad','TNEF','TNEQ']:
        add('noise_only_'+typ,[A('noise',0,typ,'-be','A',2 if typ in ['T2efac','T2equad','TNEF'] else -6),wrapper()])
    for typ in ['T2globalEfac','TNGlobalEF','TNGlobalEQ']:
        add('noise_only_'+typ,[setting(typ,-6 if typ=='TNGlobalEQ' else 2),wrapper()])
    assert len({c['id'] for c in cs})==len(cs)
    return cs

if __name__=='__main__':save(HERE/'cases.json',cases())
