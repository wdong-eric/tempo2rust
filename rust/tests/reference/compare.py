#!/usr/bin/env python3
"""Compare the adopted barycentric scientific projection using Decimal arithmetic."""
import argparse
from decimal import Decimal, localcontext
import json
from pathlib import Path

SCHEMA = json.loads((Path(__file__).with_name('schema.json')).read_text())
PROJECTION = SCHEMA['barycentric_projection']


def scientific_projection(data):
    if data.get('schema_version') != 2 or data.get('status') != 'completed':
        raise ValueError('schema-v2 completed calculation required')
    if data.get('availability') != 'available' or data.get('comparison_profile') != 'barycentric-v1':
        raise ValueError('provisional, terrestrial-candidate and smoke-only results are not barycentric goldens')
    fixture=data['fixture']
    if fixture not in PROJECTION['fixtures']:
        raise ValueError('unsupported fixture')
    config=data['configuration']
    if set(config) != set(PROJECTION['configuration_fields']):
        raise ValueError('scientific configuration fields differ from the declared projection')
    if any(config[k] != v for k,v in {'units':'TCB','binary_model':'NONE','refphs':'0','remove_mean':'0',
                                     'fit_performed':'false'}.items()):
        raise ValueError('unsupported scientific configuration')
    parameters={}
    for key,value in data['parameters'].items():
        if key in PROJECTION['omittable_legacy_parameters']:
            if value['set'] != 0 or value['fit_flag'] != 0 or Decimal(value['value']) != 0:
                raise ValueError('unsupported supplied parameter: '+key)
        elif key in PROJECTION['parameters']:
            parameters[key]={k:value[k] for k in ['set','fit_flag','value']}
        else:
            raise ValueError('unknown scientific parameter: '+key)
    if set(parameters) != set(PROJECTION['parameters']):
        raise ValueError('missing projected parameter')
    if set(data['stages']) != set(PROJECTION['stages_by_fixture'][fixture]):
        raise ValueError('unexpected or missing scientific stage')
    stages={}
    for stage,rows in data['stages'].items():
        fields=PROJECTION['parsed_fields'][:]
        if stage != 'parsed': fields += PROJECTION['bats_fields']
        if stage not in {'parsed','bats'}: fields += PROJECTION['residual_fields']
        projected=[]
        if len(rows) != len(data['original_observations']): raise ValueError('stage observation count differs')
        for row in rows:
            if any(row[k] != v for k,v in {'site':'@','deleted':'0','clockCorr':'0','delayCorr':'0'}.items()):
                raise ValueError('unsupported observation flags or site')
            for key in row.keys()-set(fields):
                if key not in PROJECTION['omittable_zero_fields'] or Decimal(row[key]) != 0:
                    raise ValueError('unsupported observation field: '+key)
            projected.append({key:row[key] for key in fields})
        stages[stage]=projected
    return {'fixture':fixture,'configuration':config,'original_observations':data['original_observations'],
            'parameters':parameters,'stages':stages}


def compare(expected, actual):
    identity=expected.get('reference_identity',{})
    for key in ['base_commit','correction_sha256']:
        if identity.get(key) != SCHEMA['reference_identity'][key]:
            raise ValueError('expected reference lacks the adopted initializer correction identity')
    if not all(identity.get(k) for k in ['instrumentation_sha256','build_manifest_sha256']):
        raise ValueError('expected reference lacks build/instrumentation identity')
    expected,actual=map(scientific_projection,[expected,actual])
    errors=[]
    for key in ['fixture','configuration','original_observations']:
        if expected[key] != actual[key]: errors.append(f'{key}: exact mismatch')
    with localcontext() as ctx:
        ctx.prec=90
        for key,value in expected['parameters'].items():
            other=actual['parameters'][key]
            for name in ['set','fit_flag']:
                if value[name] != other[name]: errors.append(f'{key}.{name}: exact mismatch')
            a,b=Decimal(other['value']),Decimal(value['value'])
            if not a.is_finite() or not b.is_finite() or abs(a-b)>abs(b)*Decimal('1e-30'):
                errors.append(f'{key}: parsed value mismatch')
        for stage,rows in expected['stages'].items():
            other=actual['stages'].get(stage,[])
            if len(rows) != len(other): errors.append(f'{stage}: observation count mismatch'); continue
            for i,(row,new) in enumerate(zip(rows,other)):
                for key,value in row.items():
                    spec=SCHEMA['observation_fields'][key]
                    if spec['comparison']=='exact':
                        if value!=new[key]: errors.append(f'{stage}[{i}].{key}: exact mismatch')
                    else:
                        a,b=Decimal(new[key]),Decimal(value)
                        if not a.is_finite() or not b.is_finite():
                            errors.append(f'{stage}[{i}].{key}: nonfinite value'); continue
                        error=abs(a-b)*Decimal(spec.get('difference_scale','1'))
                        if error>Decimal(spec['absolute_tolerance']):
                            errors.append(f'{stage}[{i}].{key}: |difference|={error} {spec["comparison_unit"]}')
    return errors


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('expected',type=Path); parser.add_argument('actual',type=Path)
    args=parser.parse_args()
    try:
        failures=compare(json.loads(args.expected.read_text()),json.loads(args.actual.read_text()))
    except (ValueError,KeyError) as error:
        failures=[str(error)]
    for failure in failures: print(failure)
    print('PASS' if not failures else 'FAIL')
    raise SystemExit(bool(failures))
