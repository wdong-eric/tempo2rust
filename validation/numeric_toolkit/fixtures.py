"""Fixed bounded inputs. Unsafe operations are inventoried, never runnable fixtures."""
import math


def cases():
    out=[]
    def add(name,actions,functions,classification='defined',rc=0):
        out.append(dict(id=name,actions=[[str(x) for x in a] for a in actions],functions=list(dict.fromkeys(functions)),classification=classification,returncode=rc))
    add('native_profile',[['profile']],[])
    values=['',' ','\t\n\r\v\f +1.25','+0','-0','-0.000','+12.5','-.125','1e3','1E-3',
            '1D3','1d-3','D4','1.2xyz','12 34','0x1.8p+2','-0x0p0','0x1p-16494',
            '0x1p-16495','0x1.0000000000000000000000000001p-16495','0x1p-16382',
            '0x0.ffffffffffffffffffffffffffffp-16382','0x1.ffffffffffffffffffffffffffffp+16383','0x1p+16384',
            '1e-6000','-1e-6000','1e6000','-1e6000','nan','NaN(payload)','+inf','-Infinity',
            '.','+','-','abc','0x','0x1p','1e+','1e','00.25',
            '1.0000000000000000000000000000000002','1.0000000000000000000000000000000001',
            '3.141592653589793238462643383279502884','999999.5','999999.4','0.0001','0.00009999995',
            '1000000','0.00001','1.23456789','-123456.789','1,25']
    for i,v in enumerate(values):add(f'parse_{i:02d}',[['parse',v]],['parse_longdouble','print_longdouble'])
    for entry,name in enumerate(['sprintf','vsprintf','fprintf','printf']):
        for style in range(5):add(f'format_{name}_{style}',[['format',entry,style]],['ld_'+name,'ld_vsprintf'], 'host_abi_formatter')
    for kind in ['hms','dms']:
        values=['',' ','abc','12','12:30','12:30:45.5','-12:30:45.5','12:-30:45','12:30:-45','+0:0:0','-0:0:0','0:60:60','24:0:0','360:0:0','720:0:0','-720:0:0','1::2','1 junk','1:2:3 trailing','1e1:2:3','nan:0:0','inf:0:0']
        for i,v in enumerate(values):add(f'coord_{kind}_{i:02d}',[['coord',kind,v]], [kind+'_turn','turn_deg'])
        turns=[0.0,-0.0,0.25,-0.25,0.5,1.0,2.0,-1.0,1/24,1/360]
        # Adjacent binary64 values around source carry thresholds, at minute/hour/day boundaries.
        units=86400 if kind=='hms' else 1296000
        threshold=59.99995 if kind=='hms' else 59.9995
        for minutes in [0,59,1439 if kind=='hms' else 21599]:
            center=(minutes*60+threshold)/units
            neighbors=[center]
            for direction in [-math.inf,math.inf]:
                x=center
                for _ in range(4):
                    x=math.nextafter(x,direction);neighbors.append(x)
            turns += sorted(neighbors)
        for i,v in enumerate(turns):add(f'turn_{kind}_{i:02d}',[['turn',kind,v.hex()]], ['turn_'+kind])
    operands=[('5.5','2'),('-5.5','2'),('5.5','-2'),('-5.5','-2'),('4','2'),('-4','2'),('0','2'),('-0','2'),('0','-2'),('-0','-2'),('0.125','0.5'),('0.3','0.1'),('1','inf'),('0','inf')]
    for precision in ['64','128']:
        pairs=operands+([(math.nextafter(2147483648.0,0).hex(),'1'),(math.nextafter(-2147483649.0,0).hex(),'1')] if precision=='64' else [('2147483647.999999999999','1'),('-2147483648.999999999999','1'),('1.0000000000000000000000000000000002','0.3')])
        for i,(x,p) in enumerate(pairs):add(f'mod{precision}_{i:02d}',[['mod'+precision,x,p]],['fortran_mod_'+precision])
    vals=[0.0,-0.0,-1.,1.,-0.5,0.5,-1.5,1.5]
    for x in [-0.5,0.5,-1.5,1.5]:vals += [math.nextafter(x,-math.inf),math.nextafter(x,math.inf)]
    vals += [2147483647.,-2147483648.,math.nextafter(2147483647.5,0),math.nextafter(-2147483648.5,0)]
    for i,x in enumerate(vals):add(f'nint_{i:02d}',[['nint',x.hex()]],['fortran_nint'])
    qvals=['0','-0','0.5','-0.5','1.5','-1.5','0x1.ffffffffffffffffffffffffffffp-2','0x1.0000000000000000000000000001p-1','-0x1.ffffffffffffffffffffffffffffp-2','-0x1.0000000000000000000000000001p-1','9223372036854775807','-9223372036854775808','9223372036854775807.499999999999','-9223372036854775808.499999999999']
    for i,x in enumerate(qvals):add(f'nlong_{i:02d}',[['nlong',x]],['fortran_nlong'])
    for i,(x,y,p) in enumerate([('0','-0','2'),('-0','1','0.5'),('0.5','1','1.0000000000000001'),('-1','-1','2'),('2','3','0.5'),('inf','1','-1'),('nan','2','2')]):
        add(f'inline_{i}',[['inline',x,y,p]],['inline_'+n for n in ['cos','sin','fabs','sqrt','asin','atan2','pow','exp','log10','log','tan']])
    vectors=[([1,0,0,0,1,0,7,8],5,0,2),([1e16,-1e16,1,1,1,1,7,8],5,0,-0.5),([1,2,3,4,5,6,7,8],0,0,2),([1,2,3,4,5,6,7,8],1,1,2),([1,2,3,4,5,6,7,8],3,3,2),([-0.0,0.0,-0.0,0.0,-0.0,0.0,7,8],5,0,-1),(['inf',2,3,0,1,2,7,8],5,0,0),(['nan',2,3,4,5,6,7,8],5,0,2)]
    for i,(v,res,start,k) in enumerate(vectors):add(f'vector_{i}',[['vector',*v,res,start,k]],['dotproduct','vectorsum','vectorscale'])
    for i,v in enumerate([[1,0,0],[0,1,0],[0,0,1],[1.25,-2.5,3.75],[-0.0,0.0,-0.0],[1e150,-1e-150,1e150],['inf',1,2],[1,'inf',2],[1,'nan',2]]):
        for direction in ['forward','reverse']:add(f'rotation_{i}_{direction}',[['rotation',*v,84381.4059,direction]],['equ2ecl','ecl2equ'])
    for angle in [0,324000,-84381.4059]:add(f'rotation_angle_{angle}',[['rotation',1,2,3,angle,'forward']],['equ2ecl','ecl2equ'])
    for count in [0,1,3,4]:add(f'copyparam_{count}',[['copyparam',count]],['copyParam'])
    for i,args in enumerate([(2,2,0),(2,0,2),(0,0,0),(3,3,3)]):add(f'copypsr_{i}',[['copypsr',*args]],['copyPSR'])
    # Indices refer to the documented fixed ids[] in probe.cc; no source metadata is inferred.
    links=[(0,1),(5,6),(7,5),(8,9),(10,11)]
    for i,(target,source) in enumerate(links):
        add(f'link_formula_{i}',[['link',target,source,1,1]],['getParameterValue'])
        add(f'link_unlinked_{i}',[['link',target,source,0,1]],['getParameterValue'])
        add(f'link_unknown_{i}',[['link',target,12,1,1]],['getParameterValue'])
    for name,act in [('zero_px',[8,9,1,0,'0']),('neg_px',[8,9,1,0,'-2']),('zero_stig',[5,6,1,0,'0']),('negative_stig',[5,6,1,0,'-2']),('right_stig',[5,6,1,0,'1']),('nan_stig',[5,6,1,0,'nan']),('unknown_target',[12,11,1,1]),('repeat_exit',[8,9,2,0])]:
        add('link_'+name,[['link',*act]],['getParameterValue'],'source_exit' if name=='repeat_exit' else 'defined',1 if name=='repeat_exit' else 0)
    def msg(kind,key='K',nw=0,text='message',var='value'):return ['message',kind,key,text,var,nw]
    # TKlog colored output has a source pointer-before-buffer formation; these are host observations.
    for quiet in [0,1]:
        for debug in [0,1]:
            for nw in [-1,0,1,2,3]:
                add(f'diagnostic_q{quiet}_d{debug}_n{nw}',[['controls',quiet,debug],msg(1,nw=nw),msg(3,'k',nw),msg(3,'MINOR',nw),msg(2,'ERR',nw),msg(0,'OTHER',nw),msg(1,'K',nw),['checklogs'],['checklogs']],['displayMsg','logerr_check','_TKchklog'],'host_diagnostic')
    add('diagnostic_suppressed_history',[msg(1,nw=2),msg(1,nw=0),msg(1,'k',0),msg(1,'k',0),msg(1,'other',0)],['displayMsg','_TKchklog'],'host_diagnostic')
    add('diagnostic_capacity',[msg(1,f'K{i:02d}') for i in range(50)]+[msg(1,'K50'),msg(3,'K51'),msg(1,'k00'),msg(1,'k00'),['checklogs'],msg(1,'k01'),msg(1,'K52')],['displayMsg','logerr_check','_TKchklog'],'host_diagnostic')
    add('diagnostic_error_storage',[msg(2,f'E{i:02d}') for i in range(32)]+[['checklogs']],['displayMsg','logerr_check','_TKchklog'],'host_diagnostic')
    add('diagnostic_truncation',[msg(1,'LONG',0,'A'*500,'B'*100),msg(2,'LONG',0,'C'*500,'D'*100),['checklogs']],['displayMsg','logerr_check','_TKchklog'],'host_diagnostic')
    assert len({c['id'] for c in out})==len(out)
    return out
