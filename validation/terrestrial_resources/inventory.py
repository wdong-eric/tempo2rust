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
    'observatory.C': ['readObservatoryFile', 'readAliases', 'initObservatories',
                     'lookup_observatory_alias', 'getObservatory', 'fang',
                     'ITRF_to_GRS80', 'GRS80_to_ITRF'],
    'eop.C': ['load_EOP', 'get_EOP'],
    'tropo.C': ['NMF_hydrostatic', 'NMF_wet', 'MeteorologyFunction_load',
                'MeteorologyFunction_getValue', 'MeteorologyFunction_getStartMJD',
                'MeteorologyFunction_getEndMJD', 'initialize_meteorology_table',
                'initialize_meteorology_tables', 'getMeteorologicalValue',
                'getZenithWetDelay', 'getSurfaceAtmosphericPressure',
                'compute_tropospheric_delays'],
}

def sha(b): return hashlib.sha256(b).hexdigest()
def blob(n): return subprocess.check_output(['git', '-C', str(ROOT/'tempo2'), 'show', PIN+':'+n])
def save(p, x):
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(json.dumps(x, indent=2, allow_nan=False)+'\n')
def uncomment(s):
    return re.sub(r'"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\'|//[^\n]*|/\*[\s\S]*?\*/',
                  lambda m: re.sub(r'[^\n]', ' ', m[0]) if m[0].startswith('/') else m[0], s)

def source_map():
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
    assert len(rows) == 22
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
    return dict(pin=PIN, release='43001af', routines=rows, callers=callers,
        preprocessor={f:[dict(line=i+1,text=l) for i,l in enumerate(blob(f).decode().splitlines()) if l.lstrip().startswith('#')] for f in PRIMARY},
        hashes={f:sha(blob(f)) for f in [*PRIMARY,'tempo2.h','tabulatedfunction.h']},
        types=json.loads((ROOT/'validation/pulsar_lifecycle/inventory.json').read_text())['types'])

def protected_paths():
    paths = []
    for directory in (ROOT/'validation').iterdir():
        if not directory.is_dir() or directory.name in ('terrestrial_resources','pulsar_lifecycle_port'): continue
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
    print('Frozen 22 routines, raw branch/caller/type map and protected accepted evidence')

def check():
    base = HERE/'outputs/frozen'
    assert source_map() == json.loads((base/'source_map.json').read_text())
    for p,h in json.loads((base/'protected.json').read_text()).items():
        assert sha((ROOT/p).read_bytes()) == h, 'protected evidence changed: '+p
    for n,v in json.loads((base/'repositories.json').read_text()).items():
        assert subprocess.check_output(['git','-C',str(ROOT/n),'rev-parse','HEAD'],text=True).strip() == v['head']
        assert subprocess.check_output(['git','-C',str(ROOT/n),'status','--porcelain'],text=True) == v['status'] == ''

if __name__ == '__main__': freeze()
