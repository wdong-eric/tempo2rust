"""Enumerate every active textOutput body and decision without rewriting Rust.

PYTHONPATH=build/python python3 source_inventory.py build/source-inventory.json
Uses pycparser 2.23, the same development-only parser as translate.py.
"""
import argparse
import hashlib
import json
import re
from pathlib import Path
from pycparser import c_parser, c_ast, c_generator

root = Path(__file__).resolve().parents[2]
p = argparse.ArgumentParser()
p.add_argument('out', type=Path)
a = p.parse_args()
source = root / 'tempo2/textOutput.C'
raw = source.read_text()
text = re.sub(r'/\*.*?\*/|//[^\n]*', lambda m: '\n' * m[0].count('\n'), raw, flags=re.S)
text = re.sub(r'^[ \t]*#.*', '', text, flags=re.M)
text = re.sub(r'\blongdouble\s*\(', 'ldconst(', text)
text = text.replace('const pulsar &psr', 'const pulsar *psr').replace('.c_str()', '')
ast = c_parser.CParser().parse('typedef double longdouble; typedef struct pulsar pulsar; typedef struct observation observation; typedef int bool; typedef unsigned long long uint64_t; typedef int param_label; typedef struct FILE FILE;\n' + text)
gen = c_generator.CGenerator()
bodies, decisions = [], []

class Inventory(c_ast.NodeVisitor):
    def generic_visit(self, n):
        if isinstance(n, (c_ast.If, c_ast.For, c_ast.While, c_ast.DoWhile, c_ast.Switch, c_ast.TernaryOp, c_ast.Case, c_ast.Default)):
            cond = getattr(n, 'cond', getattr(n, 'expr', None))
            decisions.append({'function': self.name, 'source_line': n.coord.line - 1,
                'kind': type(n).__name__, 'condition': gen.visit(cond) if cond is not None else 'default',
                'disposition': 'implemented by full typed source transfer; branch execution is not inferred from static presence'})
        super().generic_visit(n)

v = Inventory()
for node in ast.ext:
    if isinstance(node, c_ast.FuncDef):
        v.name = node.decl.name
        bodies.append({'function': v.name, 'source_line': node.decl.coord.line - 1})
        v.visit(node.body)
assert [b['function'] for b in bodies] == ['textOutput', 'calcRMS', 'm2', 'printGlitch', 'dglep_eval_phase', 'dglep']
result = {'source_sha256': hashlib.sha256(source.read_bytes()).hexdigest(),
          'rust_sha256': hashlib.sha256((root / 'rust/src/core_output_source.rs').read_bytes()).hexdigest(),
          'bodies': bodies, 'decisions': decisions,
          'coverage': 'Representative dynamic witnesses are separately listed in CORE_OUTPUT_SOURCE_MAP.md. No claim of full dynamic branch coverage.'}
a.out.parent.mkdir(parents=True, exist_ok=True)
a.out.write_text(json.dumps(result, indent=2) + '\n')
print(len(bodies), 'bodies;', len(decisions), 'branch/loop/case sites accounted for')
