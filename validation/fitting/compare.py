#!/usr/bin/env python3
"""Decision 0007 finite gates / 0008 exact nonfinite classes, using Decimal(90).
Compares independent reference outputs, never tolerances fitted to Rust results.
"""
from decimal import Decimal, localcontext
import copy
import json
import math
from fractions import Fraction
from pathlib import Path

D=Decimal
NONFINITE={'NaN','+Infinity','-Infinity'}

def round_binary(value, bits):
    """Decode a round-trip decimal into its exact binary significand (ties to even).
    Used only for mode classification, including the exactly represented equality
    control. Decimal spelling error must not move that control across its cutoff.
    """
    value=Fraction(value)
    if value==0:return value
    sign=-1 if value<0 else 1;value=abs(value)
    n,d=value.numerator,value.denominator
    exponent=n.bit_length()-d.bit_length()
    if value < Fraction(2)**exponent:exponent-=1
    shift=bits-1-exponent
    if shift>=0:n <<= shift
    else:d <<= -shift
    q,r=divmod(n,d)
    if 2*r>d or (2*r==d and q%2):q+=1
    result=Fraction(q,1<<shift) if shift>=0 else Fraction(q*(1<<-shift))
    return sign*result


class Comparison:
    def __init__(self):
        self.errors=[];self.measurements=0;self.maxima={};self.mode_permutations=[]
    def exact(self, ref, actual, path):
        if ref!=actual:self.errors.append(f'{path}: exact mismatch {ref!r} != {actual!r}')
    def shape(self, ref, actual, path):
        if not isinstance(actual,list) or len(actual)!=len(ref):
            self.errors.append(f'{path}: shape mismatch');return False
        return True
    def number(self, ref, actual, gate, path, group, scale=D(1), exact_zero=False):
        self.measurements+=1
        if not isinstance(ref,str) or not isinstance(actual,str):self.errors.append(f'{path}: numbers must be decimal strings');return
        if ref in NONFINITE or actual in NONFINITE:
            self.exact(ref,actual,path+' nonfinite class');return
        try:r,a=D(ref),D(actual)
        except Exception:self.errors.append(f'{path}: invalid decimal');return
        if not r.is_finite() or not a.is_finite():self.errors.append(f'{path}: noncanonical nonfinite spelling');return
        signed=(a-r)*scale;diff=abs(signed)
        if exact_zero and r==0:gate=D(0)
        entry={'path':path,'signed_error':str(signed),'absolute_error':str(diff),'gate':str(gate)}
        old=self.maxima.get(group)
        if old is None or diff>D(old['absolute_error']):self.maxima[group]=entry
        if diff>gate:self.errors.append(f'{path}: |difference| {diff} > {gate}')
    def vector(self, ref, actual, gate, path, group, exact_zero=False):
        if self.shape(ref,actual,path):
            for i,(r,a) in enumerate(zip(ref,actual)):self.number(r,a,gate,f'{path}[{i}]',group,exact_zero=exact_zero)
    def timing_snapshot(self,r,a,path):
        self.number(r['mean_s'],a.get('mean_s'),D('1e-12'),path+'.mean_s','mean_s')
        self.vector(r['centered_s'],a.get('centered_s'),D('1e-12'),path+'.centered_s','centered_s')
        if self.shape(r['observations'],a.get('observations'),path+'.observations'):
            for i,(ro,ao) in enumerate(zip(r['observations'],a['observations'])):
                for key in ['index','pulse_number','nphase_turn']:self.exact(ro[key],ao.get(key),f'{path}.observations[{i}].{key}')
                for key in ['sat_mjd','bat_mjd','bbat_mjd','phase_turn','residual_s']:
                    scale=D(86400) if key.endswith('_mjd') else D(1)
                    self.number(ro[key],ao.get(key),D('1e-10') if key=='phase_turn' else D('1e-12'),f'{path}.observations[{i}].{key}',key,scale)
    def solve(self,r,a,mode,shared,path):
        n=len(r['matrix']);m=len(r['matrix'][0]);ar=r['matrix'];aa=a.get('matrix')
        if not self.shape(ar,aa,path+'.matrix'):return
        for i in range(n):
            if not self.shape(ar[i],aa[i],f'{path}.matrix[{i}]'):return
        scales=[max(abs(D(ar[i][j])) for i in range(n)) for j in range(m)]
        for key in ['matrix','white_matrix']:
            if not self.shape(r[key],a.get(key),path+'.'+key):return
            maxima=[max(abs(D(r[key][i][j])) for i in range(n)) for j in range(m)]
            for i in range(n):
                if not self.shape(r[key][i],a[key][i],path+'.'+key):return
                for j in range(m):self.number(r[key][i][j],a[key][i][j],D('5e-14')*maxima[j],f'{path}.{key}[{i},{j}]',key,exact_zero=True)
        # White data comes from centered binary64 seconds. End-to-end gate propagates timing budget.
        for key in ['data_s','sigma_s','white_data']:
            if not self.shape(r[key],a.get(key),path+'.'+key):return
            for i in range(n):
                gate=D(0) if shared or key=='sigma_s' else D('1e-12')/(D(r['sigma_s'][i]) if key=='white_data' and mode else D(1))

                if shared or key=='sigma_s':self.exact(float(r[key][i]),float(a[key][i]),f'{path}.{key}[{i}] binary64')
                else:self.number(r[key][i],a[key][i],gate,f'{path}.{key}[{i}]',key)
        sr=r['solver'];sa=a.get('solver',{})
        for key in ['dof','rank']:self.exact(sr[key],sa.get(key),path+'.solver.'+key)
        for key in ['increments','errors','errors_unscaled','singular_values','retained','covariance','linearized_s']:
            if not self.shape(sr[key],sa.get(key),path+'.solver.'+key):return
        # Latest direct user instruction retains the exact raw-order gate pending
        # review. Decision 0010 pairing below remains diagnostic, not a waiver.
        self.exact(sr['retained'],sa['retained'],path+'.solver.strict_raw_retention_order')
        for j in range(m):
            if sr['retained'][j]:
                self.number(sr['singular_values'][j],sa['singular_values'][j],D('1e-10')*abs(D(sr['singular_values'][j])),path+f'.solver.strict_raw_singular_values[{j}]','strict_raw_singular_values')
        # Decision 0010 paired-mode diagnostic; no physical coordinate is permuted.
        orders=[]
        for engine,sol,bits in [('reference',sr,113),('candidate',sa,128)]:
            values=[D(v) for v in sol['singular_values']]
            if any(not v.is_finite() or v<0 for v in values):
                self.errors.append(path+'.solver.'+engine+': nonfinite or negative singular spectrum');return
            exact=[round_binary(v,bits) for v in values]
            cutoff=round_binary(Fraction.from_float(1e-27)*max(exact),bits)
            expected=[v!=0 and not v<cutoff for v in exact]
            self.exact(expected,sol['retained'],path+'.solver.'+engine+'.own_cutoff_flags')
            self.exact(sum(expected),sol['rank'],path+'.solver.'+engine+'.own_rank')
            orders.append(sorted(range(m),key=lambda i:-exact[i]))
        pairs=list(zip(*orders))
        self.mode_permutations.append({'path':path+'.solver','reference_to_candidate':pairs})
        for rj,aj in pairs:
            self.exact(sr['retained'][rj],sa['retained'][aj],path+f'.solver.matched_retained[{rj}->{aj}]')
            if sr['retained'][rj]:
                self.number(sr['singular_values'][rj],sa['singular_values'][aj],D('1e-10')*abs(D(sr['singular_values'][rj])),path+f'.solver.matched_singular_values[{rj}->{aj}]','singular_values')
        for j in range(m):
            prefix=f'{path}.solver'
            floor=D('1e-13') if shared else D('1e-11')
            p=D(sr['increments'][j]);gate=D(0) if sr['increments'][j] in NONFINITE else floor+D('1e-11')*scales[j]*abs(p)
            self.number(sr['increments'][j],sa['increments'][j],gate,prefix+f'.increments[{j}]','increment_effect_s',scales[j],exact_zero=scales[j]==0)
            if scales[j]==0:self.exact('0' if D(sa['increments'][j])==0 else 'nonzero','0',prefix+'.zero_column_increment')
            for key in ['errors','errors_unscaled']:
                ref=sr[key][j]
                if ref in NONFINITE:self.number(ref,sa[key][j],D(0),prefix+f'.{key}[{j}]',key);continue
                scale=D(1) if shared or key=='errors_unscaled' else scales[j]
                gate=D('1e-10')*abs(D(ref)) if shared or key=='errors_unscaled' else D('1e-11')+D('1e-6')*scales[j]*abs(D(ref))
                self.number(ref,sa[key][j],gate,prefix+f'.{key}[{j}]',key,scale,exact_zero=True)
            if not self.shape(sr['covariance'][j],sa['covariance'][j],prefix+'.covariance'):return
            for k in range(m):
                norm=(D(sr['covariance'][j][j])*D(sr['covariance'][k][k])).sqrt()
                self.number(sr['covariance'][j][k],sa['covariance'][j][k],D('1e-10')*norm,prefix+f'.covariance[{j},{k}]','covariance')
        if shared and all(v not in NONFINITE for v in sr['increments']+sa['increments']):
            dp=[D(a)-D(r) for a,r in zip(sa['increments'],sr['increments'])]
            for i in range(n):
                diff=sum(D(ar[i][j])*dp[j] for j in range(m))
                self.number('0',str(diff),D('1e-14'),path+f'.modeled_difference[{i}]','modeled_difference_s')
        chisq=D(sr['chi_square'])
        self.number(sr['chi_square'],sa.get('chi_square'),(D('1e-10') if shared else D('1e-6'))*abs(chisq)+(D('1e-12') if mode else D('1e-24')),path+'.solver.chi_square','chi_square')
        multiplier=sr['error_multiplier']
        self.number(multiplier,sa.get('error_multiplier'),D(0) if multiplier in NONFINITE else (D('1e-10') if shared else D('1e-6'))*abs(D(multiplier)),path+'.solver.error_multiplier','error_multiplier',exact_zero=True)
        self.vector(sr['linearized_s'],sa['linearized_s'],D('1e-12'),path+'.solver.linearized_s','linearized_s')
    def timing(self,r,a,pepoch):
        if 'input_identity' in r:
            for file in ['par','tim']:
                self.exact(r['input_identity'][file+'_sha256'],a.get('input',{}).get(file,{}).get('sha256'),'input.'+file+'.sha256')
                self.exact(r['input_identity'][file+'_text'],a.get('input',{}).get(file,{}).get('text'),'input.'+file+'.text')
            declarations=[];mode=0;nits=1;md=True;nd=True
            for i,line in enumerate(r['input_identity']['par_text'].splitlines(),1):
                t=line.split()
                if t and t[0].upper() in ['MODE','WEIGHT','NITS']:
                    declarations.append({'line':i,'keyword':t[0],'tokens':t[1:],'raw_line':line})
                    if t[0].upper()=='NITS':nits=int(t[1]);nd=False
                    else:mode=int(t[1]);md=False
            self.exact({'mode':mode,'nits':nits,'mode_defaulted':md,'nits_defaulted':nd,'declarations':declarations},a.get('controls'),'controls')
            for line in r['input_identity']['par_text'].splitlines():
                t=line.split()
                if t and t[0] in ['F0','F1','F2','F3']:
                    flag=int(t[2]) if len(t)>=3 and t[2] in ['0','1','2'] else 0
                    self.exact(flag,a['input']['parameters'][t[0]]['fit_flag'],'input.'+t[0]+'.fit_flag')
            original=[]
            for lineno,line in enumerate(r['input_identity']['tim_text'].splitlines(),1):
                t=line.split()
                if not t or t[0]=='FORMAT' or t[0]=='C' or t[0].startswith('#'):continue
                original.append({'index':len(original),'line':lineno,'name':t[0],'raw_line':line,'frequency_mhz_text':t[1],'sat_mjd_text':t[2],'toa_error_us_text':t[3],'site':t[4]})
            self.exact(original,[o['original'] for o in a['input']['observations']],'input.original_observations')
        for key in ['columns','mode','nits','rescale_requested']:self.exact(r[key],a.get(key),key)
        self.exact(True,a.get('fit_performed'),'fit_performed');self.exact(True,a.get('remove_mean'),'remove_mean')
        self.exact(r['mode']==0 or r['rescale_requested'],a.get('rescale_effective'),'rescale_effective')
        self.exact('tempo2-fit-v1',a.get('schema_version'),'schema_version')
        if not self.shape(r['cycles'],a.get('cycles'),'cycles'):return
        for ic,(rc,ac) in enumerate(zip(r['cycles'],a['cycles'])):
            path=f'cycles[{ic}]';self.exact(rc['iteration'],ac.get('iteration'),path+'.iteration')
            self.exact(list(range(len(rc['matrix']))),ac.get('row_indices'),path+'.row_indices')
            if not self.shape(rc['x_days'],ac.get('x_days'),path+'.x_days'):return
            for i,(x,y) in enumerate(zip(rc['x_days'],ac['x_days'])):self.number(x,y,4*D.from_float(math.ulp(float(x))),path+f'.x_days[{i}]','x_days',exact_zero=True)
            self.solve(rc,ac,r['mode'],False,path)
            for stage in ['prefit','postfit']:self.timing_snapshot(rc[stage],ac.get(stage,{}),path+'.'+stage)
            times=[abs((D(o['bbat_mjd'])-pepoch)*86400) for o in rc['prefit']['observations']]
            for k in range(4):
                b=max(times)**(k+1)/(D(math.factorial(k+1))*abs(D(rc['before_values'][0])))
                for key in ['before_values','after_values']:
                    change=abs(D(rc['after_values'][k])-D(rc['before_values'][k]))
                    gate=D('1e-11')+D('1e-10')*b*change
                    self.number(rc[key][k],ac[key][k],gate,path+f'.{key}[{k}]','physical_update_effect_s',b)
                    if b==0 or f'F{k}' not in r['columns']:self.number(rc[key][k],ac[key][k],D('1e-30')*abs(D(rc[key][k])),path+f'.unchanged_{key}[{k}]','unchanged_parameter',exact_zero=True)
                for key in ['before_errors','after_errors']:
                    value=rc[key][k]
                    gate=D(0) if value in NONFINITE else D('1e-11')+D('1e-6')*b*abs(D(value))
                    self.number(value,ac[key][k],gate,path+f'.{key}[{k}]','physical_error_effect_s',b,exact_zero=True)
            for key in ['offset_s','offset_error_s']:
                if rc[key] is None:self.exact(None,ac.get(key),path+'.'+key)
                else:self.number(rc[key],ac.get(key),D('1e-11')+D('1e-6')*abs(D(rc[key])),path+'.'+key,key)
        expected_nonfinite=any(e in NONFINITE for c in r['cycles'] for e in c['solver']['errors'])
        self.exact('completed_with_source_nonfinite' if expected_nonfinite else 'completed',a.get('status'),'status')

def compare_timing(reference,actual,pepoch):
    c=Comparison()
    with localcontext() as ctx:
        ctx.prec=90
        try:c.timing(reference,actual,D(pepoch))
        except (KeyError,IndexError,TypeError,ValueError,ArithmeticError) as exc:c.errors.append('malformed fit artifact: '+str(exc))
    return c

def compare_shared(reference,actual,mode):
    c=Comparison()
    with localcontext() as ctx:
        ctx.prec=90
        try:c.solve(reference,actual,mode,True,'shared')
        except (KeyError,IndexError,TypeError,ValueError,ArithmeticError) as exc:c.errors.append('malformed solver artifact: '+str(exc))
    return c


def compare_callbacks(reference, actual):
    c=Comparison()
    with localcontext() as ctx:
        ctx.prec=90
        for key in ['columns','after','physical_errors']:
            if c.shape(reference[key],actual.get(key),key):
                for i,(r,a) in enumerate(zip(reference[key],actual[key])):
                    c.number(r,a,D('1e-30')*abs(D(r)),f'{key}[{i}]',key,exact_zero=True)
    return c
