"""Freeze raw source, complete primary membership and protected evidence before execution."""
import hashlib
import json
import re
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
PIN = 'fda6ad4a9753ad7dc5d291406f2f310a9161423f'
PRIMARY = {
    'get_obsCoord.C': ['get_obsCoord_IAU2000B', 'get_obsCoord', 'remove_white', 'lmst', 'get_precessionMatrix', 'ang'],
    'vectorPulsar.C': ['vectorPulsar'],
    'tai2ut1.C': ['tai2ut1', 'ut1red'],
}

def sha(b): return hashlib.sha256(b).hexdigest()
def blob(n): return subprocess.check_output(['git', '-C', str(ROOT/'tempo2'), 'show', PIN+':'+n])
def save(p, x):
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(json.dumps(x, indent=2, allow_nan=False)+'\n')
def uncomment(s):
    return re.sub(r'"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\'|//[^\n]*|/\*[\s\S]*?\*/',
                  lambda m: re.sub(r'[^\n]', ' ', m[0]) if m[0].startswith('/') else m[0], s)

def sofa_closure():
    direct = ['cp','pom00','rxp','sxp','pxp','c2t00b','trxpv','trxp']
    pending = list(direct); closure = {}
    while pending:
        name = pending.pop()
        if name in closure: continue
        raw = blob('sofa/'+name+'.for')
        active = '\n'.join(l for l in raw.decode().splitlines() if l and l[0] not in 'cC*!')
        calls = sorted(set(x.lower() for x in re.findall(r'IAU_([A-Z0-9]+)\s*\(', active, re.I))-{name})
        closure[name] = dict(sha256=sha(raw), calls=calls)
        pending.extend(calls)
    return dict(direct=direct, closure=dict(sorted(closure.items())), declared_uncalled='c2t00a')

def source_map():
    expected = {'get_obsCoord.C':'40184b982564c13a4c02865d04045139c078edbd08a9ec40fc25c2265c7bf1d3', 'vectorPulsar.C':'46091a30f1bab414599acf0db631e57db6ad4d77396d3618a4ddcdd81c7df7df', 'tai2ut1.C':'95b4e823ec95984bdd45de6526238f593ab6a1678aa5312880f2ce50bee4782b', 'sofa/Makefile.am':'2d0f7ba86115bcb0bd53ba5b6a8ad3d2dca2e716be94097c1937efb353e259d4'}
    assert all(sha(blob(f)) == h for f,h in expected.items())
    rows = []
    for file, names in PRIMARY.items():
        raw = blob(file).decode(); text = uncomment(raw)
        for name in names:
            m = re.search(r'\b'+name+r'\s*\([^;{}]*\)\s*\{', text)
            assert m, name
            end = m.end(); depth = 1
            while depth:
                depth += (text[end] == '{') - (text[end] == '}'); end += 1
            lo = text[:m.start()].count('\n')+1; hi = text[:end].count('\n')+1
            rows.append(dict(file=file, name=name, start=lo, end=hi,
                source=raw.splitlines()[lo-1:hi],
                decisions=[dict(line=i+1, text=line.strip()) for i, line in enumerate(text.splitlines())
                    if lo <= i+1 <= hi and re.search(r'\b(if|else|for|while)\b|\?|^\s*#', line)]))
    assert len(rows) == 9
    # Search raw repository blobs, including plugins. Occurrences are evidence,
    # not a claim that every textual occurrence is an active runtime caller.
    names = [r['name'] for r in rows]
    paths = subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'ls-tree','-r','--name-only',PIN], text=True).splitlines()
    callers = []
    for path in paths:
        if not path.endswith(('.C','.h','.c','.cpp')): continue
        for i,line in enumerate(uncomment(blob(path).decode(errors='replace')).splitlines()):
            hits = [n for n in names if re.search(r'\b'+n+r'\s*\(',line)]
            if hits: callers.append(dict(file=path,line=i+1,routines=hits,text=line.strip()))
    return dict(pin=PIN, release='64cab7fc47f72d054a5d3af61441f20f069d9962', routines=rows, callers=callers,
        preprocessor={f:[dict(line=i+1,text=l) for i,l in enumerate(blob(f).decode().splitlines()) if l.lstrip().startswith('#')] for f in PRIMARY},
        hashes={f:sha(blob(f)) for f in [*PRIMARY,'tempo2.h','tabulatedfunction.h','sofa/Makefile.am']},
        sofa=sofa_closure(),
        rust_context={f:sha(subprocess.check_output(['git','-C',str(ROOT),'show','64cab7f:'+f])) for f in ('rust/src/get_obs_coord.rs','rust/src/form_bats.rs')},
        types=json.loads((ROOT/'validation/pulsar_lifecycle/inventory.json').read_text())['types'])

def protected_paths():
    paths = []
    for directory in (ROOT/'validation').iterdir():
        if not directory.is_dir() or directory.name in ('observer_coordinates','terrestrial_resources_port'): continue
        if directory.name == 'reference':
            tracked = subprocess.check_output(['git','-C',str(directory),'ls-files'],text=True).splitlines()
            paths.extend(directory/p for p in tracked)
        else:
            paths.extend(p for p in directory.rglob('*') if p.is_file() and not any(
                x in ('build','target','__pycache__','.git','.cargo-home') for x in p.relative_to(directory).parts))
    return sorted(set(paths))

def freeze():
    out = HERE/'outputs/frozen'
    assert not out.exists(), 'freeze is one-shot; preserve the existing baseline'
    save(out/'source_map.json', source_map())
    save(out/'protected.json', {str(p.relative_to(ROOT)):sha(p.read_bytes()) for p in protected_paths()})
    save(out/'repositories.json', {n:dict(head=subprocess.check_output(['git','-C',str(ROOT/n),'rev-parse','HEAD'],text=True).strip(),
        status=subprocess.check_output(['git','-C',str(ROOT/n),'status','--porcelain'],text=True)) for n in ('tempo2','validation/reference')})
    print('Frozen 9 routines, raw branch/caller/type map and protected accepted evidence')

class PreservationDrift(AssertionError):
    def __init__(self, changes):
        self.changes=changes
        super().__init__('protected evidence changed: '+', '.join(changes))

def check():
    base = HERE/'outputs/frozen'
    assert source_map() == json.loads((base/'source_map.json').read_text())
    changes={}
    for p,h in json.loads((base/'protected.json').read_text()).items():
        actual=sha((ROOT/p).read_bytes())
        if actual!=h:changes[p]=dict(frozen=h,actual=actual)
    for n,v in json.loads((base/'repositories.json').read_text()).items():
        assert subprocess.check_output(['git','-C',str(ROOT/n),'rev-parse','HEAD'],text=True).strip() == v['head']
        assert subprocess.check_output(['git','-C',str(ROOT/n),'status','--porcelain'],text=True) == v['status'] == ''
    if changes:raise PreservationDrift(changes)

def preservation_status():
    """Keep a failing ownership gate while allowing independent native work.

    Source-map and independent-repository failures still abort immediately.
    No mismatched member is removed, rewritten or silently rebaselined.
    """
    try:check()
    except PreservationDrift as e:return e.changes
    return {}

if __name__ == '__main__': freeze()
