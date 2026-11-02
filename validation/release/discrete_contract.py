"""0127's bounded targets, derived from inputs rather than Rust output.

Reuse the existing exact rational binary rounding and complete P state codec.
The finite witnesses need no IEEE exponent-range or general numeric backend.
"""
import copy
from fractions import Fraction
from af_compare import load, round_binary
from common import ROOT
from predictor_compare import native_number

p_state=load('contract_p_state',ROOT/'validation/preprocessing/inspect.py')
exact=load('contract_exact',ROOT/'validation/pulsar_lifecycle_port/run.py').compare
SIMUL_IDS=tuple('simul_'+str(i) for i in [123,124,127,128,139,140,143,144])


def utility_target(name, operation, left, right):
    """Return only adopted targets; all other rows retain their original gate."""
    x,y=native_number(left),native_number(right)
    if name in ['half_below','negative_half_inside']:
        sign=1 if name=='half_below' else -1
        assert operation=='nlong' and Fraction(x)==sign*(Fraction(1,2)-Fraction(2)**-114)
        result=round_binary(Fraction(x)+sign*Fraction(1,2),128)
        return ('finite',sign<0,Fraction(int(result)))
    if name in ['mod_rounded_quotient','mod_negative_quotient']:
        sign=1 if name=='mod_rounded_quotient' else -1
        p=Fraction(4153837486827862102824397063376077,2**115)
        assert operation=='mod' and Fraction(x)==sign and Fraction(y)==p
        quotient=int(round_binary(Fraction(x)/p,128))
        result=round_binary(Fraction(x)-round_binary(quotient*p,128),128)
        return ('finite',result<0,result)
    return None


def simul_targets(cases, initial):
    """Copy complete native histories, changing only independently proved writes."""
    targets=copy.deepcopy(cases);deltas=[];proofs=[]
    layout=initial['abi']['layout'];deleted=layout['observation.deleted']
    assert deleted['bytes']==4
    for case in targets:
        if case['id'] not in SIMUL_IDS:continue
        number=int(case['id'].split('_')[1]);reverse=number%2==0
        mode='toaerr' if number<139 else '2';threshold=' 60' if number in [127,128,143,144] else ''
        tokens=['58000','58000.0006944444444444444444444444444444','58000.0003','58001']
        actions=[['obs','0',str(i),'sat',s] for i,s in enumerate(tokens)]
        actions += [['obs','0','0','toaErr','2'],['obs','0','1','toaErr','2']]
        if reverse:actions += [['reverse','0']]
        action=['simul','PROCESS SIMUL -be A B '+mode+threshold,'1'];actions += [action,action]
        exact(case['actions'],actions,case['id']+'.bound_actions')
        _,states=p_state.reconstruct(case,initial)
        first=len(actions)-2
        before=next(state for event,state in states if event.get('action')==first-1)
        sats=[round_binary(Fraction(s),128) for s in tokens]
        if reverse:sats.reverse()
        i,j=(3,2) if reverse else (0,1)
        separation=abs(sats[i]-sats[j])*86400
        assert separation<60 and separation.denominator & (separation.denominator-1)==0
        assert separation.numerator.bit_length()<=113,'late rounding must leave this witness unchanged'
        # The equal-error toaerr branch and explicit mode 2 both delete the A
        # row. Bind its flags and errors from the complete pre-call native state.
        for row,label in [(i,b'A'),(j,b'B')]:
            prefix=f'p0.obs{row}.';raw=p_state.unpack(before[prefix+'whole_record'])
            assert before[prefix+'nFlags']==2
            exact(before[prefix+'toaErr'],{'type':'binary64','class':'finite','bits_le':'0000000000000040'})
            for field,want in [('flagID',b'-be'),('flagVal',label)]:
                span=layout['observation.'+field];width=span['bytes']//initial['abi']['MAX_FLAGS']
                values=[raw[span['offset']+k*width:span['offset']+(k+1)*width].split(b'\0')[0] for k in range(2)]
                assert values[0]==want
                if field=='flagID':assert values[1]==b'-n'
        key=f'p0.obs{i}.deleted';whole=f'p0.obs{i}.whole_record';offset=deleted['offset']
        assert before[key]==0
        old=before[whole];raw=p_state.unpack(old)
        assert raw[offset:offset+4]==b'\0'*4
        updated=copy.deepcopy(old);updated['nonzero'].append([offset,'01']);updated['nonzero'].sort()
        first_event=next(e for e in case['events'] if e['phase']=='end' and e['action']==first)
        assert key not in first_event['changes'] and whole not in first_event['changes']
        first_event['changes'][key]={'before':0,'after':1}
        first_event['changes'][whole]={'before':old,'after':updated}
        # The second call retains the first deletion; verify the complete chain.
        p_state.reconstruct(case,initial)
        for index in [first,first+1]:
            tag=f'{case["id"]} {index} simul'
            deltas += [{'tag':tag,'field':key,'rust':1,'native':0},
                       {'tag':tag,'field':whole,'byte_changes':[[offset,1,0]],'numbers':[]}]
        proofs.append({'case':case['id'],'separation_seconds':str(separation),
                       'deleted_row':i,'threshold_seconds':60,'actions':actions})
    return targets,deltas,proofs


def resolve_simul(failures, expected):
    """Require the complete predicted mismatch list; retain every other failure."""
    names={row['tag'].split()[0] for row in expected}
    selected=[row for row in failures if row['tag'].split()[0] in names]
    exact(selected,expected,'0127.original_SIMUL_differences')
    return [row for row in failures if row['tag'].split()[0] not in names]
