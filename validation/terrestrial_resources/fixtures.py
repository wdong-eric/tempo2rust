"""Authored bounded inputs and ordered histories; unsafe paths are never dispatched."""
import math
from inventory import blob

OLD = 'FORMAT(2X,I4,2X,A4,I3,2X,I5,2F9.6,F10.7,2X,F10.7,2X,2F9.6)'
NEW = 'FORMAT(3(I4),I7,2(F11.6),2(F12.7),2(F11.6),2(F11.6),2(F11.7),2F12.6)'
NEW2 = 'FORMAT(3(I4),I7,2(F11.6),2(F12.7),2(F11.6),2(F11.6),2(F11.7),2(F12.6))'
MAIN = ('# synthetic Cartesian coordinates, m\n'
        '6378237 0 0 North A LOCAL\n'
        '0 -4500000 -4500000 South B UTC\n'
        '0 0 0 COE COE UTC\n'
        '6378337 0 0 LongName ABCDEFGHIJKLMNO\n')
ALIASES = '# comments\nA alpha common\nB beta # trailing comment\nABCDEFGHIJKLMNO gamma\nEMPTY\n\n'

def fixed(year=2020,mjd='58000',xp='1.25',yp='-2.5',dut='0.125'):
    return f'{year:<17}{mjd:>5}{xp:>9}{yp:>9}{dut:>10}\n'

def eop(dut=(0,.25,.4), xs=(58000,58002,58005)):
    return NEW+'\n'+''.join(f'2020 1 {i+1} {t} {i+1} {-i-1} {d}\n' for i,(t,d) in enumerate(zip(xs,dut)))

def resources():
    a={'observatory/observatories.dat':MAIN,
       'observatory/a.dat':'6378437 0 0 Duplicate A UTC\n',
       'observatory/z.dat':'0 4500000 4500000 Extra C UTC\n',
       'observatory/aliases':ALIASES, 'observatory/choice.aliases':'B chosen\n',
       'atmosphere/a.zwd':'# A long first header\n57990 .1\n58000 .2\n58010 .3\n',
       'atmosphere/b.zwd':'# a\n57990 .9\n58010 .9\n',
       'atmosphere/c.zwd':'# B\n57990 0\n58010 0\n',
       'atmosphere/a.sap':'# A pressure\n57990 90\n58000 100\n58010 110\n',
       'atmosphere/b.sap':'# a\n57990 120\n58010 120\n',
       'atmosphere/c.sap':'# B\n57990 0\n58010 0\n',
       'clock/local.clk':'# LOCAL UTC 1\n57980 86400\n58020 86400\n',
       'eop':eop(), 'eop_alt':eop((.3,.4,.45))}
    return {'root/'+p:v for p,v in a.items()}

def cases():
    out=[]
    def add(name,actions,files=None,status=0,classification='defined',base=True):
        fs=resources() if base else {}
        fs.update(files or {})
        out.append(dict(id=name,classification=classification,status=status,files=fs,
            actions=[['profile'],['env','TEMPO2','{case}/root'],['layout',4,1,1],*actions]))
    add('observatory_lazy_aliases',[
        ['lookup','alpha'],['lookup','north'],['lookup','b'],['lookup','beta'],
        ['alias','GAMMA',0],['alias','missing',0],['alias','missing',1],['alias','alpha',1],
        ['alias','common',0],['lookup','ABCDEFGHIJKLMNO']])
    add('observatory_reinitialize',[
        ['init_obs'],['layout',4,1,1],['init_obs'],['lookup','A'],
        ['read_alias','{case}/root/observatory/aliases'],
        ['layout',4],['read_obs','{case}/root/observatory/observatories.dat'],['lookup','C']])
    alternate={p.replace('root/','other/'):v.replace('North','Replaced').replace('alpha','newalias') for p,v in resources().items()}
    add('observatory_roots',[
        ['lookup','alpha'],['env','TEMPO2','{case}/other'],['lookup','A'],['alias','newalias',0],
        ['layout',4,1,1],['init_obs'],['alias','newalias',0],['lookup','A']],alternate)
    add('observatory_changed_files',[
        ['lookup','alpha'],['copy','{case}/replacement','{case}/root/observatory/observatories.dat'],
        ['remove','{case}/root/observatory/aliases'],['lookup','A']],{'replacement':'6378537 0 0 Changed A UTC\n'})
    add('observatory_environment_selection',[
        ['root_key','U_TEMPO2'],['env','U_TEMPO2','{case}/root'],['env','TEMPO2_ALIAS','choice'],
        ['debug',1,1,0],['lookup','chosen'],['lookup','chosen']])
    add('observatory_scan_prefix_chunks',[
        ['layout',3],['read_obs','{case}/scan']],{'scan':'#'+'x'*1022+'\nignored\n\n6378237 0 0 LongerSite LONGER UTC(LONGER)\n6378237 0 0 S S\n6378237 0 0 Last L UTC trailing tokens\n'})
    add('observatory_bounded_tokens',[
        ['layout',2],['read_obs','{case}/bounds']],{'bounds':f'6378237 0 0 {"N"*31} {"C"*15} {"K"*15}\n6378237 0 0 N C UTC\n'})
    add('alias_bounded_tokens',[
        ['read_alias','{case}/bounds'],['alias','x'*127,0]],
        {'bounds':'C'*99+' '+'x'*127+'\nA short\n'})
    for count in (1,2,3,4):
        row=' '.join(['6378237','0','0','Name'][:count])+'\n'
        add(f'observatory_partial_after_valid_{count}',[['layout',1],['read_obs','{case}/bad']],
            {'bad':'6378237 0 0 Valid V UTC\n'+row},1)
    for z in ('1','-1','10000','-10000','0','-0'):
        add('observatory_cartesian_threshold_'+z,[['layout',1],['read_obs','{case}/edge']],{'edge':f'6378237 0 {z} Edge E UTC\n'})
    add('observatory_geodetic_diagnostic',[['read_obs','{case}/geo']],{'geo':'-330000 1480000 100 Geodetic G UTC\n'},1)
    add('observatory_geodetic_before_count',[['read_obs','{case}/geo']],{'geo':'-330000 1480000 100\n'},1)
    for op,path in [('read_obs','absent'),('read_alias','absent'),('lookup','UNKNOWN')]:
        add('observatory_exit_'+op,[[op,'{case}/'+path if op!='lookup' else path]],status=1,
            classification='host-qualified-diagnostic' if op=='read_alias' else 'defined')
    add('observatory_missing_main',[['env','TEMPO2','{case}/absent'],['init_obs']],status=1)
    add('observatory_missing_alias',[['remove','{case}/root/observatory/aliases'],['init_obs']],status=1,classification='host-qualified-diagnostic')
    for mode in (-1,0,1,2,3,4,5):
        add(f'fang_mode_{mode}',[['fang',mode,v] for v in ('0','-0','59.999999','60','5960','10000','123456.789','-123456.789','3595959.99')],base=False)
    for lat in (-90,-60,-30,0,30,60,90):
        add(f'geometry_latitude_{lat}',[['geometry','forward',math.radians(lat),math.radians(lon),h] for lon,h in [(-180,-500),(-90,0),(0,100),(90,10000),(180,1000000)]],base=False)
    add('geometry_cartesian', [['geometry','inverse',*v] for v in [(6378137,0,0),(-6378137,-0.0,0),(0,0,6356752),(0,0,-6356752),(0,0,0),(0,-6378137,0),(1e200,1e200,1e200)]],base=False)
    for marker in ('default','old','new','new2','transition'):
        data=(fixed() if marker=='default' else OLD+'\n'+fixed() if marker=='old' else
              (NEW if marker=='new' else NEW2)+'\n2020 1 1 58000 1.25 -2.5 .125suffix\n' if marker!='transition' else
              fixed()+NEW+'\n2020 1 2 58002 2 3 .2\n'+OLD+'\n'+fixed(mjd='58004')+NEW2+'\n2020 1 3 58005 4 5 .3\n')
        add('eop_format_'+marker,[['debug',1,1,0],['eop_seed',1,2,3,4],['load_eop','format'],['load_eop','format']],{'root/format':data})
    add('eop_years_prefix_rows',[['load_eop','years']],{'root/years':
        'ignored\n1966 ignored\n3000 ignored\n'+fixed(1967,xp='1x',yp='-0',dut='nan')+fixed(2999,mjd='58000',xp='inf',yp='-inf')+fixed(2020,mjd='57999')})
    for field in ('mjd','xp','yp','dut'):
        add('eop_bad_fixed_'+field,[['load_eop','bad']],{'root/bad':fixed(**{field:'BAD'})},1)
    add('eop_bad_modern',[['load_eop','bad']],{'root/bad':NEW+'\n2020 1 1 58000 1 2 BAD\n'},1)
    add('eop_bad_after_valid',[['load_eop','bad']],{'root/bad':NEW+'\n2020 1 1 58000 1 2 .1\n2020 1 2 58001 2 3 BAD\n'},1)
    add('eop_missing',[['load_eop','absent']],status=1)
    add('eop_empty_load',[['load_eop','empty']],{'root/empty':'# no data\n'})
    for label,d in [('none',.25),('positive',.75),('negative',-.75),('plus_half',.5),('minus_half',-.5),('above_half',.5000000000000001),('below_minus_half',-.5000000000000001)]:
        add('eop_interpolation_'+label,[['eop',t,0,'eop',0] for t in (57999,58000,58001,58002,58004.5)],{'root/eop':eop((0,d,d+.125))})
    for warning in (-1,1,2):
        add(f'eop_proxy_warnings_{warning}',[['eop',57998,warning,'eop',0],['eop',57999,warning,'eop',0]])
    for alias in (1,2):add('eop_outputs_alias_'+str(alias),[['eop',57999,2,'eop',alias],['eop',58001,2,'eop',alias]])
    add('eop_first_use_cache',[
        ['eop',58001,2,'eop',0],['eop',58001,2,'eop_alt',0],
        ['copy','{case}/root/eop_alt','{case}/root/eop'],['eop',58001,2,'eop',0],
        ['env','TEMPO2','{case}/other'],['eop',58001,2,'eop',0]],{**alternate,'other/eop':eop((.2,.3,.4))})
    add('meteorology_direct',[
        ['met_load','{case}/met'],['met_bounds'],
        *[['met_value',v] for v in (57989,57990,57995,58000,58010,58011)],
        ['met_load','{case}/short'],['met_bounds'],['met_value',58000]],
        {'met':'# A header with trailing text\n# skip\ninvalid\n58000\n57990 -0\n58000 2suffix\n58000 2\n58010 4 tail\n','short':'# B\n57990 5\n58010 7\n'})
    # Rows with suffix on the first numeric token are skipped; duplicate and
    # descending rows are load-only unless a safe bracket is explicitly chosen.
    add('meteorology_row_order',[['met_load','{case}/met'],['met_bounds']],
        {'met':'# B\n58000 1\n57990 2\n57990 nan\n58010 inf\n'},classification='host-qualified-diagnostic')
    add('meteorology_duplicate_zero_interval',[['met_load','{case}/met'],['met_value',58000]],{'met':'# B\n58000 1\n58000 2\n'})
    add('meteorology_singleton_proxy',[['met_load','{case}/met'],['met_bounds'],['met_value',57999],['met_value',58001]],{'met':'# B\n58000 7\n'})
    for label,data in [('empty',''),('header','bad\n58000 1\n58010 2\n'),('site','#\n58000 1\n58010 2\n'),('samples','# A\ninvalid\n')]:
        add('meteorology_exit_'+label,[['met_load','{case}/bad']],{'bad':data},1)
    add('meteorology_missing',[['met_load','{case}/missing']],status=1)
    add('meteorology_header_bound',[['met_load','{case}/met']],{'met':'# A '+'h'*250+'\n58000 1\n58010 2\n'})
    add('meteorology_site_bound',[['met_load','{case}/met']],{'met':'# '+'S'*252+'\n58000 1\n58010 2\n'})
    add('meteorology_filename_bound',[['met_load','{case}/'+'f'*255]],{'f'*255:'# A\n58000 1\n58010 2\n'})
    add('meteorology_precedence',[
        ['met_init',0],*[[kind,site,t,0] for kind in ('wet','pressure') for site in ('a','B','missing') for t in (57989,57990,58000,58010,58011)],
        ['met','zwd','A',58000,0],['met','sap','a',58000,0]])
    for warning in (-1,0,1,2):
        add('meteorology_warning_history_'+str(warning),[
            ['wet','B',58000,warning],['pressure','B',58000,warning],['wet','missing',58000,warning],['pressure','missing',58000,warning],['wet','B',58000,0],['pressure','B',58000,0]])
    add('meteorology_lifetime',[
        ['wet','A',58000,0],['pressure','A',58000,0],
        ['copy','{case}/replacement','{case}/root/atmosphere/a.zwd'],['wet','A',58000,0],
        ['env','TEMPO2','{case}/other'],['wet','A',58000,0],['met_init',0],['wet','A',58000,0],['pressure','A',58000,0]],
        {**alternate,'replacement':'# A\n57990 3\n58010 3\n','other/atmosphere/a.zwd':'# A\n57990 7\n58010 7\n'})
    add('meteorology_clear_and_lazy',[
        ['met_kind',0,'zwd'],['wet','A',58000,0],['met_kind',1,'sap'],['pressure','A',58000,0],
        ['env','TEMPO2','{case}/empty'],['met_init',0],['wet','A',58000,0],['pressure','A',58000,0]])
    add('glob_nomatch_host',[['glob_nomatch','{case}/nonexistent/*.zwd'],['env','TEMPO2','{case}/empty'],['met_init',0]])
    add('meteorology_missing_first_kind',[['env','TEMPO2','{case}/partial'],['met_init',0],['pressure','A',58000,0]],
        {'partial/atmosphere/only.sap':'# A\n57990 93\n58010 93\n'})
    add('meteorology_partial_initialization_exit',[['met_init',0]],{'root/atmosphere/a.sap':'bad header\n'},1)
    add('meteorology_quiet_history',[['debug',0,0,1],['wet','B',58000,0],['debug',0,0,0],['wet','B',58000,0],['pressure','B',58000,0]])
    for degrees in (-90,-75,-60,-45,-30,-15,-0.0,0,14.999999,15,22.5,29.999999,30,37.5,45,52.5,60,67.5,75,89.999,90):
        add('nmf_latitude_'+str(degrees),[['nmf',t,math.radians(degrees),h,e] for t,h,e in [(53398,0,math.pi/2),(53580.625,100,math.radians(5)),(58000,-500,math.radians(30)),(58000,10000,-.1)]],base=False)
    add('nmf_ieee',[['nmf',t,.6,h,e] for t,h,e in [(58000,0,0),(58000,100,-0.0),('nan',100,.1),(58000,'inf',.1),(58000,100,'nan'),(58000,100,'inf')]],base=False)

    def setup(order=('A','alpha','B','COE','STL_FBAT','A'),zenith=(50,50,50,50,50,50),warnings=0,override=''):
        return [['pulsar',p,6,1,warnings,override,0,0,1] for p in (0,1)]+[
            ['observation',p*6+i,'58000.1234567890123456789',site,1,900+p*6+i,0,0,zenith[i]] for p in (0,1) for i,site in enumerate(order)]
    add('integrated_multi',setup()+[['compute',2],['compute',2]])
    add('integrated_warning_order',setup(zenith=(0,0,50,50,50,0))+[['compute',2],['compute',2]])
    add('integrated_stl_warning_order',setup(order=('STL_FBAT','A','A','COE','B','A'),zenith=(50,0,0,50,50,50))+[['debug',1,1,0],['compute',2]])
    add('integrated_flags',setup()+[
        ['observation',0,58000,'UNKNOWN',0,123,0,0,0],['observation',1,58000,'A',-1,124,0,0,50],
        ['pulsar',1,6,0,0,'',0,0,1],['compute',2]])
    add('integrated_stl_disabled_warning',setup()+[
        ['observation',0,58000,'STL_FBAT',0,123,0,0,50],
        ['observation',1,58000,'A',1,124,0,0,0],['compute',2]])
    add('integrated_signed_flag_suppressed_warnings',setup(warnings=2)+[
        ['pulsar',0,6,-1,2,'',0,0,1],['compute',2]])
    for count in (-1,0,1):add('integrated_count_'+str(count),setup()+[['compute',count]])
    add('integrated_signed_nobs',setup()+[['pulsar',0,-1,1,0,'',0,0,1],['pulsar',1,0,1,0,'',0,0,1],['compute',2]])
    add('integrated_override',setup(override='UTC')+[['compute',2]])
    add('integrated_clock_season',setup(override='LOCAL')+[['compute',2]])
    add('integrated_clock_fallback',setup(override='MISSING')+[['compute',2]])
    add('integrated_missing_weather',setup()+[['observation',0,58015,'A',1,99,0,0,50],['compute',2]])
    add('integrated_elevation_ieee',setup()+[['observation',0,58000,'A',1,99,0,0,200],['observation',1,58000,'A',1,99,0,0,-50],['compute',2]])
    # An independent helper history with identical explicit clock/table arguments.
    add('integrated_helper_consistency',setup()+[['lookup','A'],['pressure','A',58000.123456789,0],['wet','A',58000.123456789,0],['correction',0,'',0],['compute',2]])
    add('integrated_helper_prediction',setup()+[['predict',0,0]])
    # Bounded real-resource smoke: immutable pinned raw resource bytes are copied
    # into the case, so the canonical runtime tree is never touched.
    real_files={}
    paths=['T2runtime/observatory/observatories.dat','T2runtime/observatory/aliases']
    import subprocess
    from inventory import ROOT,PIN
    paths+=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'ls-tree','-r','--name-only',PIN,'T2runtime/clock'],text=True).splitlines()
    for path in paths:
        if path.endswith('.clk') or '/observatory/' in path:real_files['root/'+path.split('T2runtime/')[1]]=blob(path).decode()
    # Source contains a comment-only main file plus real Parkes in observatories.dat;
    # preflight full records and use the actual count for the known-tail boundary.
    rows=[l for l in real_files['root/observatory/observatories.dat'].splitlines() if l.strip() and not l.startswith('#')]
    assert all(len(l.split()) in (5,6) for l in rows)
    real_files.update({'root/atmosphere/smoke.zwd':'# PKS\n57000 0\n59000 0\n','root/atmosphere/smoke.sap':'# PKS\n57000 101.325\n59000 101.325\n'})
    add('real_pks_smoke', [['layout',len(rows)],['lookup','PKS'],['pulsar',0,1,1,0,'',0,0,1],['observation',0,58000,'PKS',1,1,0,0,10],['correction',0,'',0],['compute',1]], real_files, base=False)
    assert len({c['id'] for c in out})==len(out)
    for case in out:
        case['actions']=[[str(v) for v in a] for a in case['actions']]
    return out

UNRUN = {
 'eop_final_or_later':'eop.C:159–193 proxy prefix then samp[size] invalid access',
 'eop_empty_singleton_malformed':'eop.C:157–199 no valid following bracket',
 'observatory_first_partial':'observatory.C:96 unwritten z before nread rejection',
 'eop_short_fixed':'eop.C:88–91 fixed offset reads beyond initialized line',
 'overlong_scan_strings':'observatory.C:90,173,176; tropo.C:229; tabulatedfunction.C:67: capacity overflow',
 'invalid_numeric_integer_casts':'observatory.C:373–374; tropo.C:97,185: unrepresentable conversion',
 'meteorology_nomatch_nonzero':'tropo.C:281–288 null host gl_pathv when warning early return is not taken',
 'table_empty_or_singleton_in_range':'tabulatedfunction.C invalid first/back or samp[-1]',
 'glob_allocation_read_failure':'host-dependent unverified; no unsafe synthetic result injection',
 'concurrency_and_other_hosts':'still-defined-unverified environments; global state is unsynchronized',
}
