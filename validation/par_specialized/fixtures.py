"""Declared safe histories. No dynamically fuzzed or undefined native input."""
import re
from inventory import released,clean
from profile import blob

def cases():
    out=[]
    def d(s,z=0):return ['dispatch',str(z),s]
    def setv(k,v,z=0):return ['set',str(z),k,str(v)]
    def add(id,actions,files=None,code=0,notes='',host=False):
        out.append(dict(id=id,actions=actions,files={n:s.encode().hex() for n,s in (files or {}).items()},outputs=[],returncode=code,
          classification=('normal-source-exit' if code else 'host-diagnostic-observation' if host else 'defined-controlled-state'),notes=notes,
          caller='two pulsars; deterministic typed field/array caller seeds then unchanged initialiseOne; explicit prefit/link tails; ne_sw_ifuncN/nWhite_dm caller zero; full projection in initial event'))
    add('initial_and_helpers',[['helpers',v,str(r),str(dm),str(c)] for v in ['0','0xffffffffff0504ff','0x030201aa'] for r,dm,c in [(-2,0,1),(2,3,4),(99,-1,2)]])
    # All simple destination-only scans can safely retain seeded destinations on failure.
    selected=released()[2]
    scalar=[]
    for b in selected:
        body='\n'.join(blob('readParfile.C').decode().splitlines()[b['line']-1:b['end_line']-1])
        if len(b['scans'])==1 and 'fscanf' in b['scans'][0] and 'psr->' in b['scans'][0] and '[' not in b['scans'][0] and b['id'] not in ['D032','D168']:
            fmt=re.search(r'fscanf\s*\(\s*fin,\s*"([^"]+)',b['scans'][0])[1]
            fs=re.findall(r'%\w+',fmt)
            if all(f in ['%lf','%lg','%d'] for f in fs):
                for key in b['keywords']:
                    values=['-3' if f=='%d' else '1.234567890123456789' for f in fs]
                    scalar.append(key+' '+' '.join(values)+'\n')
                    scalar.append(key.lower()+' '+' '.join('2' for f in fs)+'\n')
                    scalar.append(key+' '+' '.join(values[:1] if len(fs)>1 else [])+' BAD\n')
    # Lowercase QIFUNC_POS would be caught by IFUNC with an undefined index.
    scalar=[s for s in scalar if not s.startswith('qifunc')]
    add('scalar_controls',[d(''.join(scalar))],host=True,notes='Known unknown-token diagnostics on failed destination conversions; no uninitialized locals.')
    add('average_offsets',[d('AVERAGERES -backend 0.10000000000000001\nAVERAGERES -x BAD\nAVERAGERES\n'),d('TOFFSET 1 2 3 4 PKS 0.25 -be abc\r\nTOFFSET -1 2 3 4 JBO -0.5\nTOFFSET 1 BAD\nTOFFSET\n'),setv('nToffset',9),d('TOFFSET 10 20 30 40 PKS 1\n')],host=True)
    selectors=['MJD 50000 60000','FREQ 100 200','NAME file','TEL PKS','-be backend','SCALED -be']
    for prefix in ['JUMP','SATJUMP','FDJUMP2','FD3JUMP','FDJUMPDM']:
        lines=[]
        for sel in selectors:
            for value,fit in [('1.25','1'),('-0.0','0'),('BAD','-2'),('3.25','BAD')]:lines.append(f'{prefix} {sel} {value} {fit} extra\n')
            # No value => no attempt to read the missing fit token.
            lines.append(f'{prefix} {sel}\n')
        lines.append(prefix+' UNKNOWN one two three\r\n')
        add('jump_'+prefix,[d(''.join(lines))])
    add('jump_retention',[d('SATJUMP TEL PKS 2 1\n'),setv('nJumps',0),d('JUMP TEL JBO BAD 1\n'),setv('nJumps',1998),d('JUMP NAME last 3 0\n'),setv('nfdJumps',1998),d('FDJUMP2 NAME last 4 1\n')])
    add('fd_order',[d('FD1 0.25 0 0\nFDJUMP_SCALE LINEAR\nFDJUMP_SCALE linearX\nFDJUMPLOG N\nFDJUMPLOG NO\nFDJUMPLOG Y\nFDJUMPDMtail TEL PKS 1 0\nFDJUMP-3suffix TEL PKS 2 1\nFD4junk TEL PKS 3 1\nFDJUMP0 MODE 2\nFDjunk MODE 3\nFDJUMP_SCALE_suffix MODE 4\n')],host=True)
    families=[('WAVE_SIN',300,1),('WAVE_COS',300,1),('WAVE',300,2),('WAVDM',300,2),('QUAD',150,4),('ORBIFUNC',1000,3),('IFUNC',1000,3),('TEL_CLK_OFFS',1000,3),('TEL_DX',1000,3),('TEL_DY',1000,3),('TEL_DZ',1000,3),('TEL_VX',1000,2),('TEL_VY',1000,2),('TEL_VZ',1000,2),('QIFUNC_p',1000,3),('QIFUNC_c',1000,3)]
    for key,cap,n in families:
        text=''
        for idx in [1,3,1,cap]:text+=key+str(idx)+' '+' '.join(str(50000+j+idx) for j in range(n))+'\n'
        if key in ['WAVE_SIN','WAVE_COS','QUAD','IFUNC','TEL_CLK_OFFS','TEL_DX','TEL_DY','TEL_DZ','TEL_VX','TEL_VY','TEL_VZ']:text+=key.lower()+'4 '+' '.join('9' for _ in range(n))+'\n'
        text+=key+'3 7 BAD\n'+key+'2 BAD\n'
        add('samples_'+key,[d(text)],host=True)
    add('substring_precedence',[d('wave_cos1 7\nWAVE1 1 2\nwave_sin2 8\nQIFUNC_p8 52000 5 6\nQUAD_POS 1 2\nQIFUNC_POS_p 3 4\nQIFUNC_POS_c 5 6\nSTEL_DX 0 0 0\nSTEL_DY 0 0 0\nSTEL_DZ 0 0 0\nSTEL_CLK_OFFS 0 0 0\nT2EQUAD -be x 0.25\nDMEQUAD 0.125\n')],host=True)
    # A lowercase ifunc substring after a valid fixed-offset index demonstrates precedence safely.
    add('qifunc_precedence',[d('aaaaa1qifunc_p9 51000 1 2\nQIFUNC_p9 52000 3 4\n')])
    for key,idx in [('ORBIFUNC',1001),('IFUNC',1001),('TEL_DY',0),('TEL_VY',-1),('TEL_DZ',0)]:add('exit_'+key,[d(key+str(idx)+' 1 2 3\n')],code=1)
    add('dm_histories',[setv('dmoffsDMnum',2),setv('dmoffsCMnum',4),setv('ne_sw_ifuncN',3),d('DMOFF 51000 1 9\n_DM 51001 2 8\n_CM 51002 3 7\n_NE_SW 51003 4 6\nDMMODEL 10 1 0\nDMOFF 52000 11 19\n_DM 52001 12\n_CM 52002 13\n_NE_SW 52003 14\n')],host=True)
    for key,counters in [('DMOFF',['dmoffsDMnum','dmoffsCMnum']),('_DM',['dmoffsDMnum']),('_CM',['dmoffsCMnum']),('_NE_SW',['ne_sw_ifuncN'])]:add('last_'+key,[*[setv(c,999) for c in counters],d(key+' 58000 0.5 0.25\n')])
    cbody=next(b for b in selected if b['id']=='D143');raw='\n'.join(blob('readParfile.C').decode().splitlines()[cbody['line']-1:cbody['end_line']-1]);names=list(dict.fromkeys(re.findall(r'strcasecmp\(cname,"([^"]+)"\)',raw)))
    special={'PARAM','NE_SW_IFUNC_SIN','NE_SW_IFUNC_SIGMA','IFUNC_COV'}
    for name in names:
        arg=' F0 1 2 raw remainder\r\n' if name in special else '\n'
        add('constraint_'+name,[d('CONSTRAIN '+name+arg+'CONSTRAIN '+name+arg)])
    add('constraint_unknown_auto',[d('CONSTRAIN UNKNOWN\nCONSTRAIN AUTO\nCONSTRAINT_EFACTOR 0.1\nCONSTRAIN DMMODEL_DM1\n')])
    for flag in [0,1,2,-1]:add('gwecc_flag_'+str(flag),[setv('gwecc_paramSet',flag),d('GWECC_PSRTERM -3\nGWECC_AMP 2 1 0\nGWECC_PSRTERM 2\n')])
    add('gw_parameter_interactions',[d('GW_SINGLE 0.1 0 0\nCGW_FREQ 0.2 1 0\nGWECC_PSRTERM 1\nGWECC_AMP 3 0 0\nGWM_A1 4 1 0\nGWM_A2 5 0 0\nGWCS_A1 6 0 0\nGWCS_A2 7 1 0\nGWB_A1 8 0 0\nGWB_A2 9 1 0\n')])
    flaglists=[('T2EFAC','nT2efac',100),('T2EQUAD','nT2equad',100),('TNEF','nTNEF',100),('TNEQ','nTNEQ',100),('TNSQ','nTNSQ',50),('ECORR','nTNECORR',50),('TNECORR','nTNECORR',50),('ESCORR','nTNSECORR',50),('TNSECORR','nTNSECORR',50)]
    for key,count,cap in flaglists:add('flags_'+key,[d(key+' -be x 0.1\n'+key+' -be x -0.2\n'+key+' -be\n'),setv(count,cap-1),d(key+' -be last 3\n')])
    add('subtract_order',[setv('TNsubtractPoly','0xffffffffff0504aa'),d('TNRedSubPolyOrd -1\nTNDMSubPolyOrd 3\nTNChromSubPolyOrd 9\nTNsubtractPoly 0x102\nTNRedSubPolyOrd 2\nTNsubtractPoly 010\nTNsubtractPoly -1\nTNsubtractPoly xyz\nTNsubtractPoly 0xffsuffix\n')])
    add('rn_conversion',[d('TNRedC 7\nRNAMP 2D3\nRNAMP 2d-3\nRNAMP BAD\nRNAMP 0\nRNAMP -2\nRNIDX 4\nRNIDX BAD\nTNRedAmp 2D3\n')],host=True)
    ev=[('TNBandNoise','nTNBandNoise',50,'100 200 -13 4 30'),('TNGroupNoise','nTNGroupNoise',50,'-be group -12 3 40'),('TNDMEvent','nDMEvents',10,'51000 200 -14 4 1 0 -1')]
    for key,count,cap,vals in ev:add('event_'+key,[d(key+' '+vals+'\n'+key+' '+vals.split()[0]+' BAD\n'),setv(count,cap-1),d(key+' '+vals+'\n')],host=True)
    for n in [-2,0,1,3,10,12]:
        coefs=' '.join(str(i+0.25) for i in range(max(0,n)))
        add('shape_count_'+str(n),[d(f'TNShapeletEvent {n} 51000 30 2 {coefs} 1\n')],host=n>10)
    add('shape_retention',[d('TNShapeletEvent 3 51000 10 0 2.5 BAD\nTNShapeletEvent 0 52000 20 2\nTNShapeletEvent -1 53000 30 0 -3\n'),setv('nTNShapeletEvents',9),d('TNShapeletEvent 2 54000 40 2 1 2 0\n')],host=True)
    add('shape_extra_consumption',[d('TNShapeletEvent 1 51000 10 2 1 2 3 MODE 7\nTNShapeletEvent 3 52000 20 0 1\nMODE 8\n')],host=True)
    add('shape_invalid_scale',[d('TNShapeletEvent 2 51000 10 1 1 2 3\n')],code=1,host=True)
    add('phase_precision',[d('PHASE 1 54000.12345678901234567890123456789\nPHASE -2 -0\n'),setv('nPhaseJump',1999),d('PHASE 0 55000\n')])
    base='PEPOCH 54000.125 2 0.25\nF0 2 0 0\nDM 12 1 0.5\n'
    mixed='WAVE_OM 0.1 0 0\nWAVDM_OM 0.2 0 0\nWAVE1 1 2\nWAVDM2 3 4\nDMMODEL 10 1 0\nDMOFF 51000 1 2\n_CM 51001 3 4\n_NE_SW 51002 5 6\nJUMP TEL PKS 0.5 1\nFD2JUMP -be x 0.25 0\nTNEF -be x 1.2\nCONSTRAIN DMMODEL_DM1\nTNShapeletEvent 1 51000 20 2 0.5 1\nTNRedSubPolyOrd 3\n'
    files={'a.par':base+mixed,'b.par':base.replace('54000','55000')+'JUMP NAME b 1 0\nFDJUMPDM TEL JBO 2 1\nIFUNC3 53000 1 2\nPHASE 1 54001\n'}
    for reader in ['simple','full','global']:
        acts=[d('SATJUMP TEL PKS 9 1\nFDJUMP2 TEL PKS 8 0\nPHASE 1 54000\nSINI KIN\n')]
        for f in ['a.par','b.par','a.par']:acts.append([reader,'0','{case}/'+f] if reader=='simple' else [reader,'2','{case}/'+f,'{case}/b.par'])
        add('reader_'+reader,acts,files,host=True)
    clocks='CLK_CORR_CHAIN a.clk b.clk\n'
    add('clock_history',[d(clocks),['route','55000','A','C'],d('CLK_CORR_CHAIN direct.clk\n',1),['route','55000','A','C'],d(clocks,1),['route','55000','A','C']])
    add('clock_reverse',[d('CLK_CORR_CHAIN direct.clk\n'),['route','55000','A','C'],d(clocks,1),['route','55000','A','C']])
    add('average_eof',[d('AVERAGERES')])
    for label,tail in [('empty','\n'),('missing',' missing.clk\n'),('disconnected',' b.clk a.clk\n')]:add('clock_'+label,[d('CLK_CORR_CHAIN'+tail)],code=1,host=label=='empty')
    assert len({c['id'] for c in out})==len(out)
    return out
