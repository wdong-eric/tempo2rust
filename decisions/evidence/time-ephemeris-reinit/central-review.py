"""Central causal check for IFTE_init's retained twot, using disposable native builds."""
from pathlib import Path
from fractions import Fraction
import copy
import hashlib
import json
import subprocess
import sys
import tempfile

sys.dont_write_bytecode = True
ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
HERE = ROOT / 'validation/time_ephemeris'
OUT = Path(tempfile.mkdtemp(prefix='tempo2-ifte-reinit-'))
print('OUT', OUT, flush=True)
sys.path.insert(0, str(HERE))
import profile
import fixtures
import run

sha, save = profile.sha, profile.save
protected = {str(p): sha(p.read_bytes()) for p in HERE.rglob('*')
             if p.is_file() and ('outputs' in p.relative_to(HERE).parts
                                or p.parent == HERE)}
assert subprocess.check_output(['git', '-C', str(ROOT/'tempo2'), 'rev-parse', 'HEAD'],
                               text=True).strip() == profile.PIN
original_runners = profile.runners()
profile.BUILD = OUT/'build'
profile.BUILD.mkdir()
run.BUILD = OUT/'execution'
run.OUT = OUT/'execution'
old = '    ifte.iinfo.pc[1] = 0.0;'
new = old + """
    // Pinned ifteph.C::IFTE_init:188-193 omits the twot reset.
    // Keep twot=2*pc[1] so a first tc=0 query cannot reuse the old argument.
    ifte.iinfo.twot = 0.0;"""
patch = ('ifteph.C', old, new)
raw = profile.blob('ifteph.C').decode()
assert raw.count(old) == 1
assert sha(raw.encode()) == 'ddf1f64bbff6ce3b10cefc8c7aefdf9fe42f38936b89f90359b9c6da04ed40d4'
catalog = {c['id']: c for c in fixtures.catalog()}
cases = [copy.deepcopy(catalog[n]) for n in
         ['ifte_reinit', 'ifte_big', 'ifte_little', 'ifte_negative_step_retained']]
fresh = dict(id='central_fresh_zero', actions=[
    ['init', '@other'], ['get', 'values', fixtures.START, 0.5, 1],
    ['get', 'values', fixtures.START, 0.5, 2], ['close']],
    files={'other': fixtures.ifte('<', scale=2)}, status=0,
    classification='defined', note='Fresh-file mathematical control at tc=0.')
cases.append(fresh)
neighbor = copy.deepcopy(catalog['ifte_reinit'])
neighbor['id'] = 'central_reinit_nonzero'
neighbor['actions'][4][3] = 0.25
neighbor['actions'][5][3] = 0.25
neighbor['note'] = 'A nonzero tc forces source recomputation in both builds.'
cases.append(neighbor)
linear = copy.deepcopy(catalog['ifte_reinit'])
linear['id'] = 'central_reinit_linear'
linear['files'] = {'eph': fixtures.ifte(ncf=2), 'other': fixtures.ifte('<', ncf=2, scale=2)}
linear['note'] = 'Linear position and derivative do not use the stale higher polynomials.'
cases.append(linear)
assert len({c['id'].casefold() for c in cases}) == len(cases)
save(OUT/'case-membership.json', fixtures.membership(cases))
records, profiles = {}, {}
for label, mutation in [('original', None), ('corrected', patch)]:
    exe, ident = profile.compile_probe(label, mutation)
    profiles[label] = ident
    records[label] = {}
    for case in cases:
        observed = run.launch(exe, ident, case)
        path = OUT/'raw'/label/(case['id']+'.json.gz')
        run.pack(path, observed)
        records[label][case['id']] = run.projected(observed)
    profile.check_live(exe, ident)
    print('PASS', label, len(cases), 'cases', sha(exe.read_bytes()), flush=True)

def differences(a, b, path=()):
    """Retain every changed typed leaf; no numeric or state normalization."""
    if type(a) is not type(b):
        return [dict(path=list(path), original=a, corrected=b)]
    if isinstance(a, dict):
        assert list(a) == list(b), ('membership/order', path)
        return [d for k in a for d in differences(a[k], b[k], path+(k,))]
    if isinstance(a, list):
        assert len(a) == len(b), ('length', path)
        return [d for i, (x, y) in enumerate(zip(a, b)) for d in differences(x, y, path+(i,))]
    return [] if a == b else [dict(path=list(path), original=a, corrected=b)]

deltas = {}
for case in cases:
    name = case['id']
    ds = differences(records['original'][name], records['corrected'][name])
    for d in ds:
        p = d['path']
        assert len(p) >= 4 and p[0] == 'events' and p[2] == 'state', p
        assert p[3] in ('ifte.twot', 'ifte.pc', 'ifte.vc', 'output'), p
    deltas[name] = ds
assert deltas['ifte_reinit'], 'Original defect must have a behavior witness.'

def get_outputs(label, name):
    return [[float.fromhex(v['hex']) for v in e['state']['output']]
            for e in records[label][name]['events']
            if e['phase'] == 'end' and e['op'] == 'get']

cold = get_outputs('original', 'central_fresh_zero')
fixed = get_outputs('corrected', 'ifte_reinit')[-2:]
original = get_outputs('original', 'ifte_reinit')[-2:]
assert fixed[0][:2] == cold[0][:2] and fixed[1][:6] == cold[1][:6]
assert original[0][:2] != cold[0][:2]
expected_pos, expected_derivative = [], []
for component in range(3):
    c = 2 * (Fraction(1) + Fraction(component, 32)) * Fraction(1, 2**25)
    # T0,T1,T2,T3 at zero = 1,0,-1,0; derivatives = 0,1,0,-3.
    # The fixture has coefficients c,c/2,c/4,c/8 and vfac=2*na/step=2.
    expected_pos.append(float(c * Fraction(3, 4)))
    expected_derivative.append(float(c * Fraction(1, 4)))
assert cold[0][:2] == [expected_pos[0], expected_derivative[0]]
assert cold[1][:6] == expected_pos + expected_derivative
for name in ['central_fresh_zero', 'central_reinit_nonzero', 'central_reinit_linear']:
    assert get_outputs('original', name) == get_outputs('corrected', name), name
assert profile.runners() == original_runners
assert all(Path(p).is_file() and sha(Path(p).read_bytes()) == h for p, h in protected.items())
assert protected.keys() == {str(p) for p in HERE.rglob('*')
                           if p.is_file() and ('outputs' in p.relative_to(HERE).parts
                                              or p.parent == HERE)}
result = dict(status='PASS', scope='Seven selected source-derived histories, original and minimal twot correction; no production edit or full V replay.',
              output=str(OUT), script_sha256=sha(Path(__file__).read_bytes()),
              profiles=profiles, patch=patch, case_count=len(cases),
              events_per_build={k: sum(len(v['events']) for v in rows.values()) for k, rows in records.items()},
              changed_leaves={k: len(v) for k, v in deltas.items()},
              deltas=deltas, protected_files=len(protected),
              witness=dict(original_scalar=[x.hex() for x in original[0][:2]],
                           corrected_scalar=[x.hex() for x in fixed[0][:2]],
                           analytic_positions=[x.hex() for x in expected_pos],
                           analytic_derivatives=[x.hex() for x in expected_derivative]),
              checks=['original bug witnessed', 'minimal reset matches fresh owner',
                      'exact rational Chebyshev values and derivatives',
                      'cold/nonzero/linear numerical neighbors unchanged',
                      'all other event membership, types, state, streams and statuses unchanged',
                      'accepted V evidence and sources unchanged'])
save(OUT/'central-review.json', result)
print('PASS causal checks', result['witness'], flush=True)
print('CHANGES', result['changed_leaves'], flush=True)
print('RECEIPT', OUT/'central-review.json', sha((OUT/'central-review.json').read_bytes()), flush=True)
