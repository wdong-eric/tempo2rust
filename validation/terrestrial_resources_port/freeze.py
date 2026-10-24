"""One-shot W ownership, original U membership and pre-edit consumer freeze."""
import hashlib,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
OUT=HERE/'build/freeze.json'
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def members():
    paths=[]
    for base in [ROOT/'validation',ROOT/'decisions',ROOT/'handoffs']:
        for p in base.rglob('*'):
            if p.is_file() and not any(t in ('terrestrial_resources_port','build','target','__pycache__','.git','.cargo-home') for t in p.relative_to(ROOT).parts):paths.append(p)
    paths+=[ROOT/'HANDOFF.md']
    paths += [p for p in (ROOT/'rust/verification').rglob('*') if p.is_file() and 'terrestrial_resources' not in p.relative_to(ROOT/'rust/verification').parts]
    return sorted(set(paths))
def check():
    x=json.loads(OUT.read_text());changes=[p for p,h in x['protected'].items() if not (ROOT/p).is_file() or sha(ROOT/p)!=h]
    assert not changes, changes
    for name,v in x['repositories'].items():
        assert subprocess.check_output(['git','-C',str(ROOT/name),'rev-parse','HEAD'],text=True).strip()==v['head']
        assert subprocess.check_output(['git','-C',str(ROOT/name),'status','--porcelain'],text=True)==v['status']==''
    return len(x['protected'])
if __name__=='__main__':
    assert not OUT.exists(),'freeze is immutable'
    OUT.parent.mkdir(parents=True,exist_ok=True)
    u=ROOT/'validation/terrestrial_resources/outputs/frozen'
    source=json.loads((u/'source_map.json').read_text());cases=json.loads((u/'cases.json').read_text())
    assert len(source['routines'])==22 and len(cases)==130 and sum(len(c['actions']) for c in cases)==1128
    consumers=['runtime.rs','clkcorr.rs','form_bats.rs','get_obs_coord.rs','observatory.rs','eop.rs','tropo.rs','tabulated_function.rs','resource_tests.rs','clock_tests.rs']
    x=dict(protected={str(p.relative_to(ROOT)):sha(p) for p in members()},repositories={name:dict(head=subprocess.check_output(['git','-C',str(ROOT/name),'rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','-C',str(ROOT/name),'status','--porcelain'],text=True)) for name in ('tempo2','validation/reference')},source_map=source,cases=cases,consumers={n:dict(sha256=sha(ROOT/'rust/src'/n),source=(ROOT/'rust/src'/n).read_text()) for n in consumers},contract_sha256=sha(HERE/'CONTRACT.md'))
    OUT.write_text(json.dumps(x,indent=2)+'\n');print('Frozen',check(),'protected files; 22 routines; 130 cases;',sha(OUT))
