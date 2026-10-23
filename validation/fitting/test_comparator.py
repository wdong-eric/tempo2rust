#!/usr/bin/env python3
"""Mutate independently measured quantities at and beyond each M2 gate."""
import copy
from decimal import Decimal as D, localcontext
import json
import math
from pathlib import Path
from compare import compare_timing, compare_shared, compare_callbacks
ROOT=Path(__file__).resolve().parent

def mutations(actual_dir):
    ref=json.loads((ROOT/'reference/all.json').read_text())
    actual=json.loads((actual_dir/'all.json').read_text())
    assert not compare_timing(ref,actual,'54000').errors
    results=[]
    def change(name,path,value):
        mutated=copy.deepcopy(actual);target=mutated
        for key in path[:-1]:target=target[key]
        target[path[-1]]=value
        errors=compare_timing(ref,mutated,'54000').errors
        assert errors, name+' was not detected'
        results.append({'mutation':name,'detected':True,'first_error':errors[0]})
    rc=ref['cycles'][0];p=['cycles',0];s=p+['solver']
    change('increment sign',s+['increments',1],str(-D(rc['solver']['increments'][1])))
    change('column permutation',['columns'],['ZERO','F1','F0','F2','F3'])
    change('row permutation',p+['row_indices'],list(reversed(actual['cycles'][0]['row_indices'])))
    change('changed measurement weights',p+['sigma_s',0],'0.000002')
    change('raw instead of centered data',p+['data_s'],[o['residual_s'] for o in rc['prefit']['observations']])
    cov=[[str(D(x)*2) for x in row] for row in rc['solver']['covariance']]
    change('covariance incorrectly rescaled',s+['covariance'],cov)
    change('dropped column',['columns'],actual['columns'][:-1])
    change('extra column',['columns'],actual['columns']+['F4'])
    change('missing matrix column',p+['matrix'],[row[:-1] for row in actual['cycles'][0]['matrix']])
    change('changed rank',s+['rank'],4)
    change('changed DOF',s+['dof'],12)
    change('changed retained mode',s+['retained'],[False,True,True,True,True])
    change('missing iteration',['cycles'],actual['cycles'][:1])
    change('raw as postfit centered residual',p+['postfit','centered_s'],[o['residual_s'] for o in rc['postfit']['observations']])
    change('wrong pulse count',p+['postfit','observations',1,'pulse_number'],'123')
    change('changed source identity',['input','par','sha256'],'0'*64)
    change('changed original source',['input','tim','text'],'FORMAT 1\n')
    change('changed resolved control',['controls','mode'],0)
    change('missing mean-removal label',['remove_mean'],False)
    def above(name,path,value,gate):change(name,path,str(D(value)+gate*D('1.01')))
    # Gates calculated from the prospective contract, independent of measured Rust errors.
    a=max(abs(D(row[1])) for row in rc['matrix']);aw=max(abs(D(row[1])) for row in rc['white_matrix'])
    above('matrix just above gate',p+['matrix',0,1],rc['matrix'][0][1],D('5e-14')*a)
    above('white matrix just above gate',p+['white_matrix',0,1],rc['white_matrix'][0][1],D('5e-14')*aw)
    above('x just above four ULP',p+['x_days',0],rc['x_days'][0],4*D.from_float(math.ulp(float(rc['x_days'][0]))))
    above('retained singular value just above gate',s+['singular_values',0],rc['solver']['singular_values'][0],D('1e-10')*abs(D(rc['solver']['singular_values'][0])))
    above('increment effect just above gate',s+['increments',1],rc['solver']['increments'][1],(D('1e-11')+D('1e-11')*a*abs(D(rc['solver']['increments'][1])))/a)
    norm=(D(rc['solver']['covariance'][0][0])*D(rc['solver']['covariance'][1][1])).sqrt()
    above('covariance just above gate',s+['covariance',0,1],rc['solver']['covariance'][0][1],D('1e-10')*norm)
    above('fit error just above gate',s+['errors',1],rc['solver']['errors'][1],(D('1e-11')+D('1e-6')*a*abs(D(rc['solver']['errors'][1])))/a)
    above('unscaled error just above gate',s+['errors_unscaled',1],rc['solver']['errors_unscaled'][1],D('1e-10')*abs(D(rc['solver']['errors_unscaled'][1])))
    above('chi square just above gate',s+['chi_square'],rc['solver']['chi_square'],D('1e-12')+D('1e-6')*abs(D(rc['solver']['chi_square'])))
    for stage in ['prefit','postfit']:
        above(stage+' mean just above gate',p+[stage,'mean_s'],rc[stage]['mean_s'],D('1e-12'))
        above(stage+' centered just above gate',p+[stage,'centered_s',1],rc[stage]['centered_s'][1],D('1e-12'))
        for key,gate in [('residual_s',D('1e-12')),('bbat_mjd',D('1e-12')/86400),('phase_turn',D('1e-10'))]:
            above(stage+' '+key+' just above gate',p+[stage,'observations',1,key],rc[stage]['observations'][1][key],gate)
    above('linearized residual just above gate',s+['linearized_s',1],rc['solver']['linearized_s'][1],D('1e-12'))
    t=max(abs((D(o['bbat_mjd'])-54000)*86400) for o in rc['prefit']['observations'])
    for k in [0,1,2,3]:
        b=t**(k+1)/(D(math.factorial(k+1))*abs(D(rc['before_values'][0])))
        above(f'physical F{k} just above gate',p+['after_values',k],rc['after_values'][k],(D('1e-11')+D('1e-10')*b*abs(D(rc['after_values'][k])-D(rc['before_values'][k])))/b)
        above(f'physical F{k} error just above gate',p+['after_errors',k],rc['after_errors'][k],(D('1e-11')+D('1e-6')*b*abs(D(rc['after_errors'][k])))/b)
    # Large direct callback increments expose the updated-F0 requirement independently.
    cb=json.loads((ROOT/'reference/callbacks.json').read_text());ca=json.loads((actual_dir/'callbacks.json').read_text())
    assert not compare_callbacks(cb,ca).errors
    wrong=copy.deepcopy(ca);before=D(cb['before'][1]);new=D(cb['after'][1]);wrong['after'][1]=str(before-(before-new)*D(cb['before'][0])/D(cb['after'][0]))
    errors=compare_callbacks(cb,wrong).errors;assert errors
    results.append({'mutation':'using pre-update F0 for F1','detected':True,'first_error':errors[0]})
    for fixture,key,index,replacement in [('square_rescaled','errors',0,None),('infinite_covariance','errors',0,'-Infinity'),('infinite_increment','increments',0,'1'),('infinite_rescaled_error','errors',0,'0')]:
        r=json.loads((ROOT/'reference'/f'{fixture}.json').read_text());a=json.loads((actual_dir/f'shared_{fixture}.json').read_text())[0]
        assert not compare_shared(r,a,r['mode']).errors
        a['solver'][key][index]=replacement;errors=compare_shared(r,a,r['mode']).errors;assert errors
        results.append({'mutation':fixture+' nonfinite class','detected':True,'first_error':errors[0]})
    # Paired-mode analysis remains diagnostic; latest user choice retains strict order.
    for fixture in ['dependent_columns','at_cutoff']:
        r=json.loads((ROOT/'reference'/f'{fixture}.json').read_text())
        a=json.loads((actual_dir/f'shared_{fixture}.json').read_text())[0]
        # Align only this mutation baseline; keep each engine's own rounded values.
        reference_order=sorted(range(2),key=lambda j:D(r['solver']['singular_values'][j]))
        candidate_order=sorted(range(2),key=lambda j:D(a['solver']['singular_values'][j]))
        for key in ['singular_values','retained']:
            values=a['solver'][key].copy()
            for ri,ai in zip(reference_order,candidate_order):a['solver'][key][ri]=values[ai]
        assert not compare_shared(r,a,r['mode']).errors
        permuted=copy.deepcopy(a)
        for key in ['singular_values','retained']:permuted['solver'][key].reverse()
        c=compare_shared(r,permuted,r['mode']);assert c.errors
        assert all('strict_raw_' in error for error in c.errors),c.errors
        results.append({'mutation':fixture+' paired mode permutation','strict_order_rejected':True,'paired_diagnostic_valid':True,'permutations':c.mode_permutations})
        for name,mutate in [
            ('flag/value mismatch',lambda d:d['solver']['singular_values'].reverse()),
            ('negative singular value',lambda d:d['solver']['singular_values'].__setitem__(0,'-1')),
            ('nonfinite singular value',lambda d:d['solver']['singular_values'].__setitem__(0,'NaN')),
            ('changed rank',lambda d:d['solver'].__setitem__('rank',0)),
            ('changed retention',lambda d:d['solver']['retained'].__setitem__(0,not d['solver']['retained'][0])),
        ]:
            # Both at-cutoff modes are retained; a pure value permutation there is valid.
            if fixture=='at_cutoff' and name=='flag/value mismatch':continue
            bad=copy.deepcopy(a);mutate(bad);c=compare_shared(r,bad,r['mode']);assert c.errors,(fixture,name)
            results.append({'mutation':fixture+' '+name,'detected':True,'first_error':c.errors[0]})
        if fixture=='at_cutoff':
            bad=copy.deepcopy(a);j=min(range(2),key=lambda j:D(bad['solver']['singular_values'][j]));bad['solver']['retained'][j]=False
            c=compare_shared(r,bad,r['mode']);assert c.errors
            results.append({'mutation':'threshold equality wrongly dropped','detected':True,'first_error':c.errors[0]})
    change('physical parameter output permutation',s+['increments'],list(reversed(rc['solver']['increments'])))
    return results

if __name__=='__main__':
    import sys
    with localcontext() as ctx:
        ctx.prec=90
        result=mutations(Path(sys.argv[1]))
    print(json.dumps(result,indent=2))
