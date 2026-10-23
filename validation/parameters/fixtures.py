"""Bounded controlled caller cases. No implicit replacement of canonical defaults."""
import json
import re
from profile import HERE, ROOT
from inventory import inventory


def cases():
    out=[]
    init=['init','1','0','0']
    def add(id,actions,files=None,code=0,init_first=True,notes=''):
        out.append(dict(id=id,actions=([init] if init_first else [])+actions,files=files or {},returncode=code,notes=notes,
                        state='canonical initializer followed by explicit controlled prefit=17,prefitErr=19, strings and counters; see probe.cc::seed' if init_first else 'declared actions only'))
    add('registry', [['init','1','0','0'],['registry'],['destroy']],init_first=False)
    add('init_full', [['init','1','1','2'],['destroy']],init_first=False)
    add('init_all', [['init','2','1','1','all'],['destroy']],init_first=False)
    for flag in [0,1]:add('allocate_'+str(flag),[['allocate',str(flag)]],init_first=False)
    add('numeric', [['parse',s] for s in ['1.234567890123456789012345678901234567','1D3','1d3','1junk','junk','0x1.8p2','-0','+0','nan','-nan','inf','-Infinity','1e5000','-1e5000','1e-5000','-1e-5000','  23.5tail','']],init_first=False)
    add('coordinates',[[ 'coord',kind,s] for kind in ['hms','dms'] for s in ['12:34:56.7890123456789','12 34 56.789','-01:30:00','-00:30:00','12:-30:00','370:00:00','720:00:00','12','12:30','12:bad','bad','', 'nan:0:0','1D1:0:0']],init_first=False)
    add('remove_cr',[['remove',s] for s in ['abc\n','abc\r\n','abc\r','abc','\n']],init_first=False)
    add('value_forms', [['read','F0','param_f','0',s] for s in ['1\n','2 0\n','3 1\n','4 2\n','5 1.0\n','6 +1\n','7 01\n','8 -0.5\n','9 7junk 2D-3\n','10 nope -3d-4\n','11 1 2 surplus token\n','12junk 2.0D-5\n','0x1.8p2 1 2\n','-0 0 0\n','nan 0 0\n','inf 0 0\n','1e5000 0 0\n','1e-5000 0 0']])
    add('coordinate_duplicate', [['read','RAJ','param_raj','0','12:30:00 1 3\n'],['read','RAJ','param_raj','0','13:00:00\n'],['read','DECJ','param_decj','0','-30:00:00 0 5\n'],['read','DECJ','param_decj','0','-31:00:00 2\n'],['read','RAJ','param_raj','0','-01:00:00 1 4\n']])
    add('periods', [['read','P1','param_f','1','2 1 3\n'],['read','P','param_f','0','0.5 1 4\n'],['prior','param_f','0','2','2','0.125','4','1','1'],['read','Pdot','param_f','1','3 1 5\n'],['read','P0','param_f','0','0 0 0\n']])
    for v in ['2','5','TEMPO1','tempo2','3']:
        add('ephver_'+v,[['read','EPHVER','param_ephver','0',v+' 0 0\n']],code=1 if v=='3' else 0)
    # Every exact-match readValue alias, each in a fresh case to avoid accidental frame mixing.
    seen=set()
    for b in inventory()['dispatch']:
        if not b['readValue_calls']:continue
        aliases=re.findall(r'strcasecmp\s*\(\s*str\s*,\s*"([^"]+)"\s*\)\s*==\s*0',b['condition'])
        for key in aliases:
            if key in seen:continue
            seen.add(key)
            val='5' if key=='EPHVER' else '12:34:56.5' if key in ['RA','RAJ','DEC','DECJ'] else '1.25'
            add('alias_'+key,[['stream','F0 2 0 0\n'+key+' '+val+' 1 0.125\n']])
    # Indexed families: safe endpoints and source ignored upper bounds. No negative/OOB indices.
    families={'F':[1,12,13], 'DM':[1,9,10], 'CM':[1,9,10], 'FD':[1,9,13], 'FB':[0,19],
              'TELX':[1,3,4], 'TELY':[1,3,4], 'TELZ':[1,3,4], 'DMASSPLANET':[1,9,10]}
    for prefix in ['GLEP_','GLPH_','GLF0_','GLF1_','GLF2_','GLF0D_','GLF0D2_','GLF0D3_','GLTD_','GLTD2_','GLTD3_','EXPEP_','EXPPH_','EXPTAU_','EXPINDEX_','GAUSEP_','GAUSAMP_','GAUSSIG_','GAUSINDEX_']:
        families[prefix]=[1,39,40]
    for prefix in ['NUDOT_AMP_','NUDOT_TIME_','NUDOT_EPOCH_']:families[prefix]=[1,31,32]
    for prefix in ['DMX_','DMXR1_','DMXR2_','SX_','SXR1_','SXR2_','SXER_','CHROMX_','CHROMXR1_','CHROMXR2_']:families[prefix]=[1,512,513]
    for prefix in ['A1_','ECC_','E_','T0_','OM_','PB_','PBDOT_','XDOT_','X2DOT_']:families[prefix]=[1,4,5]
    for prefix,indices in families.items():add('index_'+prefix,[['stream',''.join(prefix+str(j)+' 1.125 1 0.25\n' for j in indices)]])
    add('dispatch_case_order',[['stream','fB1 2 0 0\nfb1 3 0 0\nFB1 4 0 0\nglf0d2_1 5 0 0\nGlEp_1 6 0 0\nF1junk 7 0 0\nGLEP_1junk 8 0 0\n# comment F0 999\nCcomment F0 999\nIcomment F0 999\nccomment\nUNKNOWN 0\nMODE 1 WEIGHT 2\nNITS -3\nF0 10 1 2 MODE 7\nF1 11 0 0']])
    controls='PSR FIRST PSRB SECOND psrj THIRD\nCLK TT(TAI)\nEOP_FILE caller.eop\nMODE 2 WEIGHT -1\nNITS 0 NITS -2 NITS 3\nIPM 0 SWM 1 IBOOT 4\nROBUST 0\nROBUST H\nTRACK 3 1 0\nNOTRACK\nNO_SS_SHAPIRO\nWHITE_NOISE_MODEL model.dat\nFJUMP backend\nTEMPO1\nUNITS TCB UNITS TDB UNITS SI UNITS unknown\nTIMEEPH FB90 TIMEEPH IF99 TIMEEPH invalid\nDILATEFREQ yes PLANET_SHAPIRO no CORRECT_TROPOSPHERE Y\nT2CMETHOD TEMPO\nEPH_FILE custom.eph\nT2CMETHOD TEMPO\nEPHEM DE414 EPHEM INPOP06 EPHEM de405 EPHEM DE405\nEPHEM_FILE caller.eph\nTZRSITE @ REFPHS TZR REFPHS MEAN REFPHS invalid\nNTOA ignored 123\nDM_SERIES P\nDM_SERIES Taylor\nDM_SERIES strange\n'
    add('controls',[['defaults'],['stream',controls]])
    add('threshold_scaling',[['stream',''.join(k+' '+v+' 1 0.25\n' for k in ['PBDOT','XPBDOT','A1DOT','XDOT','EPS1DOT','EPS2DOT','EDOT','ECCDOT'] for v in ['1e-7','1.0000001e-7','-2','1e-15'])]])
    target={'DSHK':('PX','param_dshk','0.5'),'SINI':('KIN','param_sini','30'),'KIN':('STIG','param_kin','0.5'),'D_AOP':('PBDOT','param_daop','1e-12'),'DMMODEL':('DM','param_dmmodel','12.5')}
    for key,(dest,param,val) in target.items():
        prereq='PB 3 0 0\nPMRA 4 0 0\nPMDEC 5 0 0\n'
        for order in ['before','after']:
            lines=[key+' '+dest+' 2\n',dest+' '+val+' 0 0\n']
            if order=='after':lines.reverse()
            add('link_'+key+'_'+order,[['stream',prereq+''.join(lines)],['get',param,'0']])
        # Duplicate DSHK terminates inside readValue; other links terminate when evaluated.
        actions=[['stream',dest+' '+val+' 0 0\n'],['stream',key+' '+dest+'\n'],['stream',key+' '+dest+'\n']]
        if key!='DSHK':actions.append(['get',param,'0'])
        add('link_repeat_'+key,actions,code=1)
    add('link_capacity',[['stream','KIN 30 0 0\n'+'SINI KIN\n'*5],['get','param_sini','0']],code=1)
    add('copy_parameter',[['prior','param_pepoch','0','54000','54001','0.5','0.75','2','1'],['copy','param_pepoch','param_posepoch']])
    add('link_nonrecursive',[['stream','STIG 0.5 0 0\nKIN STIG\nSINI KIN\n'],['get','param_kin','0'],['get','param_sini','0']])
    base='PEPOCH 54000.125 2 0.25\nF0 2 0 0\n'
    add('final_defaults',[['stream',base+'WAVE_OM 0.1 0 0\nWAVDM_OM 0.2 0 0\n'],['final'],['final']])
    add('final_explicit',[['stream','POSEPOCH 53000 1 0.5\nDMEPOCH 53001 0 0.75\nWAVEEPOCH 53002 0 0.25\n'+base],['final']])
    add('final_no_pepoch',[['final']],code=1)
    for clk in ['UNCORR','uncorr','UTC(NIST)','UTC(BIPM)','PTB','AT1','TT(TAI)','tt(tai)','XYZ']:
        add('clock_'+re.sub(r'\W','_',clk),[['stream',base+'CLK '+clk+'\n'],['final']])
    for suffix,coords,code in [('pair','ELONG 120 1 0.5\nELAT -30 0 0.25\n',0),('globalfit','ELAT -30 2 0.25\nELONG 120 2 0.5\n',0),('lon','ELONG 120 1 0\n',1),('lat','ELAT -30 0 0\n',1),('mixed','RAJ 12:00:00 1 0\nPMLAMBDA 1 0 0\n',1)]:
        add('ecliptic_'+suffix,[['stream',base+coords],['final']],code=code)
    add('binary_ddgr',[['stream',base+'BINARY DDGR\nFB19 1e-20 0 0\n'],['final']],code=1)
    add('binary_ell1',[['stream',base+'BINARY ell1\nFB1 1e-20 1 1e-22\nPBDOT 1e-12 2 2e-14\n'],['final'],['stream','PBDOT 2e-12 1 0\n'],['final']])
    add('binary_aliases',[['stream','BINARY BT1P\nBINARY BT2P\nBINARY ELL1\n']])
    add('defaults_missing',[['env','unset'],['defaults']])
    files={'a.par':base+'DM 12 1 0.5\n','b.par':'PEPOCH 55000 0 0.125\nF0 4 0 0\n'}
    for reader in ['full','simple','global']:
        actions=[['defaults'],['counts']]
        actions += [[reader,'{case}/'+f] if reader=='simple' else [reader,'1','{case}/'+f,'{case}/missing.tim'] for f in ['a.par','b.par','a.par']]
        add('repeat_'+reader,actions,files)
    for reader in ['full','global']:
        add('multi_'+reader,[['init','2','0','0'],[reader,'2','{case}/a.par','{case}/missing.tim','{case}/b.par','{case}/missing.tim']],files,init_first=False)
        add('missing_'+reader,[[reader,'1','{case}/missing.par','{case}/missing.tim']],code=1)
    add('full_missing_env',[['env','unset'],['full','1','{case}/a.par','{case}/missing.tim']],files,code=1)
    add('simple_missing_epoch',[['defaults'],['simple','{case}/bad.par']],{'bad.par':'F0 2 0 0\n'},code=1)
    add('global_without_epoch',[['global','1','{case}/bad.par','{case}/missing.tim']],{'bad.par':'F0 2 0 0\n'})
    for reader in ['full','simple']:
        for tail in ['pair','partial']:
            coords='ELONG 120 1 0.25\n'+('ELAT -30 0 0.5\n' if tail=='pair' else '')
            a=[['defaults'],[reader,'{case}/ecl.par'] if reader=='simple' else [reader,'1','{case}/ecl.par','{case}/missing.tim']]
            add(reader+'_ecliptic_'+tail,a,{'ecl.par':base+coords},code=0 if tail=='pair' else 1)
    add('head_fallback',[['full','1','{case}/missing.par','{case}/head.tim']],{'head.tim':'HEAD\n'+base+'TOAS\nF0 999 0 0\n'})
    add('head_noread_multi',[['init','2','0','0'],['full','2','{case}/missing.par','{case}/head.tim','{case}/b.par','{case}/missing.tim']],{'head.tim':'HEAD\n'+base+'TOAS\n','b.par':base+'TOAS\nF0 999 0 0\n'},init_first=False)
    add('invalid_header',[['full','1','{case}/missing.par','{case}/bad.tim']],{'bad.tim':'BAD\n'},code=1)
    def record(fields):
        chars=[' ']*90
        for lo,hi,s in fields: chars[lo-1:hi]=list(s.ljust(hi-lo+1))
        return ''.join(chars)+'\n'
    fixed='111111111111111111111110000\n'+record([(1,12,'JTEST'),(21,40,' 123456.789'),(41,60,' -123456.7'),(61,70,'3.25'),(71,80,'-4.25')])+record([(2,20,'0.5'),(21,40,'1.25'),(41,60,'2454000.5'),(61,70,'2.5'),(71,80,'0.75')])+record([(9,20,'12.5')])
    add('fixed_header',[['full','1','{case}/missing.par','{case}/fixed.tim']],{'fixed.tim':fixed})
    add('fixed_noread_multi',[['init','2','0','0'],['full','2','{case}/missing.par','{case}/fixed.tim','{case}/a.par','{case}/missing.tim']],{'fixed.tim':fixed,'a.par':base},init_first=False)
    add('fixed_values',[['fixed','param_f','0','1','12','0.5         '],['fixed','param_f','1','1','12','1.25        '],['fixed','param_f','2','1','12','2.5         '],['fixed','param_pepoch','0','1','14','2454000.5     '],['fixed','param_dm','0','1','10','1D3       '],['fixed','param_px','0','1','10','          ']])
    # Read G input text without invoking G's timing calculations or modifying its evidence.
    g=json.loads((ROOT/'validation/binary/cases.json').read_text()); models=set()
    for case in g:
        if case['model'] in models:continue
        models.add(case['model']);data=(ROOT/'validation/binary/fixtures'/case['par']).read_text()
        add('G_'+case['model'],[['defaults'],['stream',data],['final']],notes='G '+case['par']+'; parser only, no binary-model numerical claim')
    assert len(models)==14
    assert len({c['id'] for c in out})==len(out)
    return out
