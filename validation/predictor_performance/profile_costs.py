#!/usr/bin/env python3
"""Attribute an existing macOS sample tree; never run an alternative evaluator."""
import argparse
import hashlib
import json
from pathlib import Path
import re
from sample import summarize

PATTERNS = {
    'evaluate': r'Cheby2D8evaluate',
    'read': r'predictor_ffi4read',
    'polynomial': r'predictor_ffi10polynomial',
    'allocators': r'(?:malloc|calloc|realloc|free|_xzm_)',
    'double_conversion': r'^mpfr_(?:get_d|set_d)$',
    'arithmetic': r'^mpfr_(?:add|sub|mul|div)(?:\d|_|$)',
    'range': r'ordinary_long_double7rounded|Float18subnormalize_round|^mpfr_(?:subnormalize|check_range|underflow|overflow)$',
    'init_clear': r'^mpfr_(?:init2|clear)$',
}


def costs(graph):
    nodes, stack = [], []
    for line in graph.splitlines():
        m = re.match(r'([\s+!:|]*)(\d+) (\S.*)', line)
        if not m: continue
        depth, count = len(m[1]), int(m[2])
        while stack and nodes[stack[-1]]['depth'] >= depth: stack.pop()
        parent = stack[-1] if stack else None
        if parent is not None: nodes[parent]['children'] += count
        nodes.append(dict(depth=depth, count=count, children=0, parent=parent,
                          symbol=m[3].split('  (in ')[0], description=m[3]))
        stack.append(len(nodes)-1)
    total = sum(n['count'] for n in nodes if n['parent'] is None)
    inclusive = dict.fromkeys(PATTERNS, 0)
    own = dict.fromkeys(['allocator', 'double_conversion', 'arithmetic', 'range',
                        'init_clear', 'other_mpfr', 'gmp', 'thread_local', 'other'], 0)
    evaluator_inclusive = dict.fromkeys(PATTERNS, 0)
    for index, node in enumerate(nodes):
        weight = node['count']-node['children']
        assert weight >= 0, node
        lineage = []; i = index
        while i is not None:
            lineage.append(nodes[i]['symbol']); i = nodes[i]['parent']
        hits = {key:any(re.search(pattern, s) for s in lineage) for key,pattern in PATTERNS.items()}
        for key, hit in hits.items():
            if hit:
                inclusive[key] += weight
                if hits['evaluate']: evaluator_inclusive[key] += weight
        symbol = node['symbol']
        if re.search(PATTERNS['allocators'], symbol) or '(in libsystem_malloc.dylib)' in node['description']:
            category = 'allocator'
        else:
            category = next((k for k in ['double_conversion', 'arithmetic', 'range', 'init_clear']
                             if re.search(PATTERNS[k], symbol)), None)
            if category is None:
                category = ('other_mpfr' if symbol.startswith('mpfr_') else 'gmp' if 'gmp' in symbol
                            else 'thread_local' if symbol == '_tlv_get_addr' else 'other')
        own[category] += weight
    assert sum(own.values()) == total
    return dict(total=total, inclusive=inclusive, within_evaluator=evaluator_inclusive,
                exclusive_reported_frames=own)


if __name__ == '__main__':
    # The nested allocator must count once; siblings and unsampled children
    # leave their own frame weights, and the exclusive partition sums exactly.
    control = costs('100 root\n  60 Cheby2D8evaluate\n    30 malloc\n      10 _xzm_free\n  20 mpfr_add\n')
    assert control['inclusive']['evaluate'] == 60 and control['inclusive']['allocators'] == 30
    assert control['within_evaluator']['allocators'] == 30
    assert control['exclusive_reported_frames']['allocator'] == 30
    assert control['exclusive_reported_frames']['arithmetic'] == 20
    assert control['exclusive_reported_frames']['other'] == 50
    p = argparse.ArgumentParser(description=__doc__); p.add_argument('sample', type=Path)
    a = p.parse_args()
    graph = a.sample.read_text().split('Call graph:\n')[1].split('Total number in stack')[0]
    result = costs(graph); previous = summarize(a.sample)
    assert result['total'] == previous['total']
    for key in ['evaluate', 'read', 'polynomial', 'allocators']:
        assert result['inclusive'][key] == previous[key]['samples'], key
    result['sample_sha256'] = hashlib.sha256(a.sample.read_bytes()).hexdigest()
    result['script_sha256'] = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    print(json.dumps(result, indent=2))
