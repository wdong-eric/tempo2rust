#!/usr/bin/env python3
"""Scope an existing sample trace to C reconstruction; no evaluator or timings."""
import argparse
import hashlib
import json
from pathlib import Path
import re

from profile_costs import PATTERNS, costs
from sample import summarize


def call_subtrees(graph, pattern):
    """Keep disjoint selected call subtrees, rebasing each root to depth zero."""
    selected, root_depth = [], None
    for line in graph.splitlines():
        match = re.match(r'([\s+!:|]*)(\d+) (\S.*)', line)
        if not match:
            continue
        depth = len(match[1])
        if root_depth is not None and depth <= root_depth:
            root_depth = None
        if root_depth is None and re.search(pattern, match[3].split('  (in ')[0]):
            root_depth = depth
        if root_depth is not None:
            selected.append(line[root_depth:])
    return '\n'.join(selected)


if __name__ == '__main__':
    # Shared read/allocator frames belong to both responsibilities; selected
    # roots at different depths must remain disjoint in the existing parser.
    control = ('100 root\n  60 predictor_ffi10polynomial\n'
               '    40 predictor_ffi4read\n      20 malloc\n'
               '      10 predictor_ffi10polynomial\n'
               '  20 predictor_ffi4read\n    5 malloc\n'
               '  20 wrapper\n    10 predictor_ffi10polynomial\n'
               '      6 predictor_ffi4read\n')
    whole = costs(control)
    selected = costs(call_subtrees(control, PATTERNS['polynomial']))
    assert selected['total'] == whole['inclusive']['polynomial'] == 70
    assert selected['inclusive']['read'] == 46
    assert selected['inclusive']['allocators'] == 20
    assert whole['inclusive']['read'] - selected['inclusive']['read'] == 20
    assert sum(selected['exclusive_reported_frames'].values()) == 70
    assert costs(call_subtrees(control, r'absent'))['total'] == 0

    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('sample', type=Path)
    args = parser.parse_args()
    graph = args.sample.read_text().split('Call graph:\n')[1].split('Total number in stack')[0]
    result = costs(graph)
    original = summarize(args.sample)
    assert result['total'] == original['total']
    for name in ['evaluate', 'read', 'polynomial', 'allocators']:
        assert result['inclusive'][name] == original[name]['samples']
    scopes = {name: costs(call_subtrees(graph, pattern)) for name, pattern in {
        'polynomial': PATTERNS['polynomial'], 'read': PATTERNS['read'],
        'evaluate': PATTERNS['evaluate'], 'native_parts': r'^t2r_ld_parts$',
        'native_write': r'^t2r_ld_from_parts$',
    }.items()}
    for name in ['polynomial', 'read', 'evaluate']:
        assert scopes[name]['total'] == result['inclusive'][name]
    outside_reads = result['inclusive']['read'] - scopes['polynomial']['inclusive']['read']
    assert outside_reads >= 0
    result.update(scoped_call_trees=scopes, scalar_read_stacks_outside_polynomial=outside_reads,
                  interpretation='Inclusive scopes overlap; disjoint current-frame partitions conserve each scope. Inlined frames and destruction outside construction remain subject to attribution limits.',
                  inputs={str(p.resolve()): hashlib.sha256(p.read_bytes()).hexdigest()
                          for p in [args.sample, Path(__file__), Path(__file__).with_name('profile_costs.py'),
                                    Path(__file__).with_name('sample.py')]})
    print(json.dumps(result, indent=2))
