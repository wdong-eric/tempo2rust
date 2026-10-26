"""Frozen quantity gates. Constructed state is checked before serialization/exchange."""
from decimal import Decimal as D, getcontext
import math
getcontext().prec = 70
EPS = D(2) ** -52

def read_tsv(path):
    out = {}
    for line in path.read_text().splitlines():
        key, index, value = line.split('\t')
        assert (key, int(index)) not in out
        out[key, int(index)] = value if key.startswith('text.') else D(value)
    return out

class Checks:
    def __init__(self):
        self.count = 0
        self.failures = []
    def exact(self, label, actual, expected):
        self.count += 1
        if actual != expected:
            self.failures.append({'field': label, 'actual': str(actual), 'expected': str(expected), 'gate': 'exact'})
    def number(self, label, actual, expected, bound=0):
        self.count += 1
        actual, expected, bound = D(str(actual)), D(str(expected)), D(bound)
        # These are native binary64 fields on this measured host. Decimal strings
        # may differ while representing the same exact native rounding cell.
        if actual.is_nan() or expected.is_nan():
            ok = actual.is_nan() and expected.is_nan() and actual.is_signed() == expected.is_signed()
        elif not actual.is_finite() or not expected.is_finite():
            ok = actual == expected
        elif actual == 0 and expected == 0:
            ok = actual.is_signed() == expected.is_signed()
        elif bound == 0:
            ok = float(actual) == float(expected)
        else:
            ok = abs(actual - expected) <= bound
        if not ok:
            self.failures.append({'field': label, 'actual': str(actual), 'expected': str(expected), 'bound': str(bound)})

def phase_bound(scale): return D('1e-10') + 32 * EPS * scale
def hz_bound(scale): return D('1e-12') + 32 * EPS * scale

def cheby_scale(model, derivative=False):
    p = model['frequency_cheby' if derivative else 'cheby']
    nx = p['nx']
    scale = sum(abs(D(v)) * (D('.5') if i % nx == 0 else 1) * (D('.5') if i // nx == 0 else 1) for i, v in enumerate(p['coeff']))
    if derivative:
        return scale * 2 / (D(model['mjd_end']) - D(model['mjd_start'])) / 86400
    return scale + abs(D(model['dispersion_constant'])) / min(abs(D(model['freq_start'])), abs(D(model['freq_end']))) ** 2

def polyco_scale(model):
    half = D(model['span']) / 2
    return abs(D(model['reference_phase'])) + abs(D(model['frequency_psr_0'])) * half * 60 + sum(abs(D(v)) * half ** i for i, v in enumerate(model['coeff']))

def constructed(kind, result, native):
    c = Checks()
    models = result['predictor']['Cheby']['segments'] if kind == 'cheby' else result['models']['segments']
    count = int(native['segments', 0]) if kind == 'cheby' else sum(key == 'ncoeff' for key, _ in native)
    c.exact('constructed.segment_count', len(models), count)
    for k in range(min(len(models), count)):
        m = models[k]
        for field in ['psrname', 'sitename'] + ([] if kind == 'cheby' else ['date_string', 'utc_string']):
            c.exact(f'model[{k}].{field}', m[field], native['text.' + field, k])
        if kind == 'cheby':
            nx, ny = int(native['nx', k]), int(native['ny', k])
            for field in ['mjd_start', 'mjd_end', 'freq_start', 'freq_end']:
                c.number(f'model[{k}].{field}', m[field], native[field, k])
            # Scale comes exclusively from the native model, never mutant data.
            coeff = [native['coeff' + str(k), i] for i in range(nx * ny)]
            scale = sum(abs(v) * (D('.5') if i % nx == 0 else 1) * (D('.5') if i // nx == 0 else 1) for i, v in enumerate(coeff)) + abs(native['dispersion_constant', k]) / native['freq_start', k] ** 2
            bound = phase_bound(scale)
            c.number(f'model[{k}].dispersion_phase', D(m['dispersion_constant']) / native['freq_start', k] ** 2, native['dispersion_constant', k] / native['freq_start', k] ** 2, bound)
            for field, key, factor in [('cheby', 'coeff', 1), ('frequency_cheby', 'derivative', 2 * nx * nx)]:
                p = m[field]
                c.exact(f'model[{k}].{field}.nx', p['nx'], nx)
                c.exact(f'model[{k}].{field}.ny', p['ny'], ny)
                c.exact(f'model[{k}].{field}.count', len(p['coeff']), nx * ny)
                for i in range(min(len(p['coeff']), nx * ny)):
                    c.number(f'model[{k}].{field}[{i}]', p['coeff'][i], native[key + str(k), i], bound * factor)
        else:
            nc = int(native['ncoeff', k])
            c.exact(f'model[{k}].count', len(m['coeff']), nc)
            c.exact(f'model[{k}].span', m['span'], int(native['span', k]))
            for field in ['mjd_mid', 'frequency_psr_0', 'frequency_obs', 'dm', 'doppler', 'log10rms', 'binary_phase', 'binary_frequency']:
                c.number(f'model[{k}].{field}', m[field], native[field, k])
            half = native['span', k] / 2
            scale = abs(native['reference_phase', k]) + abs(native['frequency_psr_0', k]) * half * 60 + sum(abs(native['coeff' + str(k), i]) * half ** i for i in range(nc))
            bound = phase_bound(scale)
            c.number(f'model[{k}].reference_phase', m['reference_phase'], native['reference_phase', k], bound)
            for i in range(min(len(m['coeff']), nc)):
                c.number(f'model[{k}].coeff[{i}]_span_turn', D(m['coeff'][i]) * half ** i, native['coeff' + str(k), i] * half ** i, bound)
    if kind == 'polyco' and 'coefficients' in result:
        nc = int(native['ncoeff', count - 1])
        c.exact('last_timing_coefficient_count', len(result['coefficients']), nc)
        for i in range(min(nc, len(result['coefficients']))):
            c.number(f'last_timing_coefficient[{i}]_span_turn', D(result['coefficients'][i]) * half ** i, native['last_coeff', i] * half ** i, bound)
    return c


def evaluation_scale(kind, m, time, frequency):
    t, f = D(time), D(frequency)
    if kind == 'cheby':
        x = -1 + 2 * (t-D(m['mjd_start'])) / (D(m['mjd_end'])-D(m['mjd_start']))
        y = -1 + 2 * (f-D(m['freq_start'])) / (D(m['freq_end'])-D(m['freq_start']))
        def terms(v, size):
            a=[D(1),v]
            while len(a)<size:a.append(2*v*a[-1]-a[-2])
            a[0]=D('.5');return a[:size]
        p=m['cheby'];tx=terms(x,p['nx']);ty=terms(y,p['ny'])
        scale=lambda p:sum(abs(D(v)*tx[i%p['nx']]*ty[i//p['nx']]) for i,v in enumerate(p['coeff']))
        return scale(p)+abs(D(m['dispersion_constant'])/f**2),scale(m['frequency_cheby'])*2/(D(m['mjd_end'])-D(m['mjd_start']))/86400
    dt=(t-D(m['mjd_mid']))*1440;cs=[D(v) for v in m['coeff']]
    b=abs(D(m['frequency_psr_0']))+sum(abs(v*i*(D(1) if i==1 else dt**(i-1))/60) for i,v in enumerate(cs) if i)
    a=abs(D(m['reference_phase']))+abs(dt*60*D(m['frequency_psr_0']))+sum(abs(v*(D(1) if i==0 else dt**i)) for i,v in enumerate(cs))
    a+=b*abs(D(str(m['dm']))/D('2.41e-4')/(1+D(str(m['doppler']))*D('1e-4'))*(1/f**2-1/D(str(m['frequency_obs']))**2))
    return a,b


def native_models(kind, ref):
    result=[]
    for k in range(int(ref['segments',0])):
        if kind=='cheby':
            m={field:str(ref[field,k]) for field in ['mjd_start','mjd_end','freq_start','freq_end','dispersion_constant']}
            nx,ny=int(ref['nx',k]),int(ref['ny',k])
            for field,key in [('cheby','coeff'),('frequency_cheby','derivative')]:m[field]={'nx':nx,'ny':ny,'coeff':[str(ref[key+str(k),i]) for i in range(nx*ny)]}
        else:
            m={field:str(ref[field,k]) for field in ['mjd_mid','frequency_psr_0','reference_phase','frequency_obs','dm','doppler','span']}
            m['coeff']=[str(ref['coeff'+str(k),i]) for i in range(int(ref['ncoeff',k]))]
        m['psrname']=ref['text.psrname',k];m['sitename']=ref['text.sitename',k]
        result.append(m)
    return result

def plan(kind, actual, ref, models, frequency):
    c=Checks();count=int(ref['count',0]);segments=actual['plan']['segments']
    c.exact('plan.declared_count',actual['segment_count'],count)
    c.exact('plan.actual_count',len(segments),count)
    c.exact('plan.native_membership',{i for k,i in ref if k=='average_hz'},set(range(count)))
    c.number('plan.frequency_mhz',actual['plan']['frequency_mhz'],frequency)
    meta=actual['plan']['metadata'];first,last=models[0],models[-1]
    for field,value in [('kind','Cheby' if kind=='cheby' else 'T1'),('psrname',first['psrname']),('sitename',first['sitename'])]:c.exact('plan.metadata.'+field,meta[field],value)
    expected={'start_mjd':first['mjd_start'],'end_mjd':last['mjd_end'],'freq_start_mhz':first['freq_start'],'freq_end_mhz':first['freq_end']} if kind=='cheby' else {'start_mjd':float(first['mjd_mid'])-float(first['span'])/2880.,'end_mjd':float(last['mjd_mid'])+float(last['span'])/2880.,'freq_start_mhz':first['frequency_obs'],'freq_end_mhz':first['frequency_obs']}
    for field,value in expected.items():c.number('plan.metadata.'+field,meta[field],value)

    first_scale=D(1)
    for i in range(min(count,len(segments))):
        row=segments[i];scales=[]
        for side in ['start','end']:
            c.number(f'plan[{i}].{side}_mjd',row[side+'_mjd'],ref[side+'_mjd',i])
            selected=int(ref[side+'_segment',i])
            c.exact(f'plan[{i}].{side}_segment',row[side+'_segment'] if row[side+'_segment'] is not None else -1,selected)
            scales.append(D(1) if selected<0 else evaluation_scale(kind,models[selected],str(ref[side+'_mjd',i]),frequency)[0])
        if i==0:first_scale=scales[0]
        c.exact(f'plan[{i}].out_of_range',row['out_of_range'],bool(ref['out_of_range',i]))
        elapsed=(ref['end_mjd',i]-ref['start_mjd',i])*86400
        bound=(phase_bound(scales[0])+phase_bound(scales[1]))/elapsed+8*EPS*abs(ref['average_hz',i])
        c.number(f'plan[{i}].average_frequency_hz',row['average_frequency_hz'],ref['average_hz',i],bound)
    c.number('plan.phase0',actual['plan']['phase0_turn'],ref['phase0',0],phase_bound(first_scale))
    return c


def serialization(kind, original, parsed, filename):
    """Separate loss from source field precision; no engine tolerance is added."""
    c=Checks();c.exact('serialized.segment_count',len(parsed),len(original))
    for k,(a,b) in enumerate(zip(original,parsed)):
        if kind=='cheby':
            for field in ['mjd_start','mjd_end','freq_start','freq_end','dispersion_constant']:c.number(f'serialized[{k}].'+field,b[field],a[field])
            for field in ['cheby','frequency_cheby']:
                c.exact(f'serialized[{k}].{field}.count',len(b[field]['coeff']),len(a[field]['coeff']))
                for i,(v,w) in enumerate(zip(a[field]['coeff'],b[field]['coeff'])):c.number(f'serialized[{k}].{field}[{i}]',w,v)
        else:
            places={'mjd_mid':11,'dm':6,'doppler':3,'log10rms':3,'reference_phase':6,'frequency_psr_0':12,'frequency_obs':3,'binary_phase':4,'binary_frequency':4} if filename=='polyco_new.dat' else {'mjd_mid':20,'dm':10,'doppler':7,'log10rms':3,'reference_phase':15,'frequency_psr_0':20,'frequency_obs':3,'binary_phase':4,'binary_frequency':4}
            for field,digits in places.items():
                # Original timing value -> native model and final parse can each
                # contribute half a native ULP in addition to decimal rounding.
                bound=D(5)*D(10)**(-digits-1)+D(str(math.ulp(float(a[field]))))
                c.number(f'serialized[{k}].'+field,b[field],a[field],bound)
            c.exact(f'serialized[{k}].coefficient_count',len(b['coeff']),len(a['coeff']))
            for i,(v,w) in enumerate(zip(a['coeff'],b['coeff'])):c.number(f'serialized[{k}].coeff[{i}]',w,v)
    return c
