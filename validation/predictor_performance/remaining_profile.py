#!/usr/bin/env python3
"""Attribute an accepted complete-call trace without running another evaluator."""
import argparse
import hashlib
import json
from pathlib import Path

from profile_costs import costs
from reconstruction_profile import call_subtrees
from sample import summarize

SCOPES = {
    'model_entry': r'^t2r_model$',
    'selection': r'^t2r_nearest$',
    'scalar_vector': r'predictor_ffi6values',
    'read': r'predictor_ffi4read',
    'native_parts': r'^t2r_ld_parts$',
    'normalization': r'cheby2d14model_evaluate',
    'phase': r'cheby2d11model_phase',
    'frequency': r'cheby2d15model_frequency',
    'recurrence': r'cheby2d8evaluate|Cheby2D8evaluate',
    'publication': r'predictor_ffi5write|^t2r_ld_from_parts$',
    'native_publication': r'^t2r_ld_from_parts$',
}


def attribution(graph):
    graphs = {name: call_subtrees(graph, pattern) for name, pattern in SCOPES.items()}
    scopes = {name: costs(tree) for name, tree in graphs.items()}
    nested = {parent + '/' + child: costs(call_subtrees(graphs[parent], SCOPES[child]))
              for parent, child in [('model_entry', 'read'), ('selection', 'read'),
                                    ('model_entry', 'scalar_vector'), ('selection', 'scalar_vector'),
                                    ('normalization', 'recurrence'), ('phase', 'normalization'),
                                    ('frequency', 'normalization')]}
    regions = {label: scopes[parent]['total'] - nested[parent + '/' + child]['total']
               for label, parent, child in [
                   ('normalization_outside_recurrence', 'normalization', 'recurrence'),
                   ('phase_outside_normalization', 'phase', 'normalization'),
                   ('frequency_outside_normalization', 'frequency', 'normalization')]}
    assert all(count >= 0 for count in regions.values())
    return dict(scopes=scopes, nested_scopes=nested, nonoverlap_model_regions=regions)


if __name__ == '__main__':
    control = ('100 root\n  60 t2r_model\n'
               '    15 predictor_ffi6values\n      10 predictor_ffi4read\n        4 t2r_ld_parts\n'
               '    40 cheby2d11model_phase\n      30 cheby2d14model_evaluate\n'
               '        10 cheby2d8evaluate\n          4 mpfr_set_d\n          2 Cheby2D8evaluate\n'
               '      5 mpfr_div\n    5 predictor_ffi5write\n      2 t2r_ld_from_parts\n'
               '  30 t2r_nearest\n    15 predictor_ffi4read\n    10 __gmpn_copyi\n'
               '  10 other\n')
    check = attribution(control)
    assert check['scopes']['recurrence']['total'] == 10
    assert check['scopes']['read']['total'] == 25
    assert check['scopes']['publication']['total'] == 5
    assert check['nested_scopes']['model_entry/read']['total'] == 10
    assert check['nested_scopes']['selection/read']['total'] == 15
    assert check['nonoverlap_model_regions'] == dict(normalization_outside_recurrence=20,
        phase_outside_normalization=10, frequency_outside_normalization=0)
    assert costs(call_subtrees(control, r'absent'))['total'] == 0

    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('sample', type=Path)
    a = p.parse_args()
    graph = a.sample.read_text().split('Call graph:\n')[1].split('Total number in stack')[0]
    whole, original = costs(graph), summarize(a.sample)
    assert whole['total'] == original['total']
    for name in ['evaluate', 'read', 'polynomial', 'allocators']:
        assert whole['inclusive'][name] == original[name]['samples']
    result = attribution(graph)
    result.update(total=whole['total'],
        overall_inclusive_primitives={key:value for key,value in whole['inclusive'].items()
                                      if key not in ['evaluate','read','polynomial']},
        overall_reported_frame_partition=whole['exclusive_reported_frames'],
        interpretation='Scopes overlap. Model regions subtract nested recurrence/normalization trees only; they include function-local cleanup and do not cover all entry cleanup. Legacy sample.py evaluate matches only the owned symbol; the recurrence scope also recognizes the accepted generic C specialization. Inlining limits attribution.',
        inputs={str(path.resolve()):hashlib.sha256(path.read_bytes()).hexdigest() for path in
                [a.sample, Path(__file__), *[Path(__file__).with_name(name) for name in
                 ['profile_costs.py','reconstruction_profile.py','sample.py']]]})
    print(json.dumps(result, indent=2))
