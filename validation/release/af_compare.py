"""AF gates with measured native-width decoding for exact fields.

Tolerance formulas, including EPS=2**-52 and serialization allowances, are
unchanged. Only exact scalar decoding and the source metadata arithmetic use
ordinary-long-double precision. Explicit C double fields remain binary64.
"""
from decimal import Decimal as D, localcontext
from fractions import Fraction
import importlib.util
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
def load(name,path):
    spec=importlib.util.spec_from_file_location(name,path);module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module);return module
base=load('al_af_original',ROOT/'validation/predictors/compare.py')
round_binary=load('al_binary_decode',ROOT/'validation/fitting/compare.py').round_binary
BaseChecks=base.Checks

class NativeChecks(BaseChecks):
    bits=53
    def number(self,label,actual,expected,bound=0):
        a,b=D(str(actual)),D(str(expected))
        if D(bound)==0 and a.is_finite() and b.is_finite() and (a!=0 or b!=0):
            self.count+=1
            double=label.rsplit('.',1)[-1] in {'dm','doppler','log10rms','frequency_obs','binary_phase','binary_frequency'} or label.startswith('freq[')
            bits=53 if double else self.bits
            if round_binary(a,bits)!=round_binary(b,bits):
                self.failures.append({'field':label,'actual':str(a),'expected':str(b),'bound':'0','native_bits':bits})
        else:super().number(label,actual,expected,bound)

base.Checks=NativeChecks

def span(mid,width,sign):
    bits=NativeChecks.bits
    value=round_binary(mid,bits)+sign*round_binary(Fraction(width)/2880,bits)
    result=round_binary(value,bits)
    with localcontext() as context:
        context.prec=20000
        return D(result.numerator)/D(result.denominator)


def plan(kind, actual, ref, models, frequency):
    # Same AF plan checks; metadata follows native rounded division then add/sub.
    c=NativeChecks();count=int(ref['count',0]);segments=actual['plan']['segments']
    c.exact('plan.declared_count',actual['segment_count'],count)
    c.exact('plan.actual_count',len(segments),count)
    c.exact('plan.native_membership',{i for k,i in ref if k=='average_hz'},set(range(count)))
    c.number('plan.frequency_mhz',actual['plan']['frequency_mhz'],frequency)
    meta=actual['plan']['metadata'];first,last=models[0],models[-1]
    for field,value in [('kind','Cheby' if kind=='cheby' else 'T1'),('psrname',first['psrname']),('sitename',first['sitename'])]:c.exact('plan.metadata.'+field,meta[field],value)
    expected={'start_mjd':first['mjd_start'],'end_mjd':last['mjd_end'],'freq_start_mhz':first['freq_start'],'freq_end_mhz':first['freq_end']} if kind=='cheby' else {'start_mjd':span(first['mjd_mid'],first['span'],-1),'end_mjd':span(last['mjd_mid'],last['span'],1),'freq_start_mhz':first['frequency_obs'],'freq_end_mhz':first['frequency_obs']}
    for field,value in expected.items():c.number('plan.metadata.'+field,meta[field],value)
    first_scale=D(1)
    for i in range(min(count,len(segments))):
        row=segments[i];scales=[]
        for side in ['start','end']:
            c.number(f'plan[{i}].{side}_mjd',row[side+'_mjd'],ref[side+'_mjd',i])
            selected=int(ref[side+'_segment',i])
            c.exact(f'plan[{i}].{side}_segment',row[side+'_segment'] if row[side+'_segment'] is not None else -1,selected)
            scales.append(D(1) if selected<0 else base.evaluation_scale(kind,models[selected],str(ref[side+'_mjd',i]),frequency)[0])
        if i==0:first_scale=scales[0]
        c.exact(f'plan[{i}].out_of_range',row['out_of_range'],bool(ref['out_of_range',i]))
        elapsed=(ref['end_mjd',i]-ref['start_mjd',i])*86400
        bound=(base.phase_bound(scales[0])+base.phase_bound(scales[1]))/elapsed+8*base.EPS*abs(ref['average_hz',i])
        c.number(f'plan[{i}].average_frequency_hz',row['average_frequency_hz'],ref['average_hz',i],bound)
    c.number('plan.phase0',actual['plan']['phase0_turn'],ref['phase0',0],base.phase_bound(first_scale))
    return c

base.plan=plan
