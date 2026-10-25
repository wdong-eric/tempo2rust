#!/usr/bin/env python3
"""Mutation controls for the declared six-field averaged projection, without new pipelines."""
import copy
import json
import check_public as c

B = c.HERE/'build/comparator-controls'
B.mkdir(parents=True, exist_ok=True)
case = c.HERE/'build/public-debug/averaging-mean0'
original = json.loads((case/'rust.json').read_text())
native = (case/'native.tsv').read_text()
assert not c.compare(native, original)[0]
mutations = {
 'omit_active_field': lambda a:a.pop('residual'),
 'wrong_active_unit': lambda a:a['residual'].update(unit='ms'),
 'empty_projection': lambda a:a.clear(),
 'extra_field': lambda a:a.update(extra={'value':None,'unit':'s'}),
 'omit_inapplicable_field': lambda a:a.pop('dm_residual'),
 'wrong_inapplicable_unit': lambda a:a['dm_residual'].update(unit='s'),
 'non_null_inapplicable': lambda a:a['dm_residual'].update(value='0'),
 'omit_quantity_unit': lambda a:a['residual'].pop('unit'),
 'extra_quantity_field': lambda a:a['residual'].update(extra=0),
 'omit_quantity_value': lambda a:a['dm_residual'].pop('value'),
}
checks = []
for name, mutate in mutations.items():
    altered = copy.deepcopy(original)
    mutate(altered['pulsars'][0]['observations'][0]['averaged'])
    errors, _ = c.compare(native, altered)
    assert errors, name
    checks.append(dict(name=name, errors=errors))
# TZR carries all six inapplicable entries with the same declared units.
case_tzr = c.HERE/'build/public-debug/tzr-averaging-mean0'
tzr = json.loads((case_tzr/'rust.json').read_text())
assert not c.compare((case_tzr/'native.tsv').read_text(), tzr)[0]
del tzr['pulsars'][0]['reference_observation']['averaged']['bat']
errors, _ = c.compare((case_tzr/'native.tsv').read_text(), tzr)
assert errors
checks.append(dict(name='omit_tzr_inapplicable_field', errors=errors))
(B/'receipt.json').write_text(json.dumps(dict(
    comparator_sha256=c.native.sha(c.HERE/'check_public.py'),
    controls_sha256=c.native.sha(c.HERE/'check_comparator.py'),
    input_sha256={str(p):c.native.sha(p) for directory in [case,case_tzr] for p in [directory/'rust.json',directory/'native.tsv']},
    unchanged_baselines_pass=True, rejected_mutations=checks), indent=2)+'\n')
print('Two stored baselines pass; all eleven averaged schema/unit mutations rejected')
