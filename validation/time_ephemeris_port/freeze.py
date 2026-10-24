"""Y immutable accepted inputs; central/Z evolution is deliberately not frozen."""
import hashlib,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
OUT=HERE/'build/freeze.json'
PIN='5920affdcb2de05df7b2101c6f53455e9ff31d1a'
def sha(p):
    h=hashlib.sha256()
    with p.open('rb') as f:
        for b in iter(lambda:f.read(1024*1024),b''):h.update(b)
    return h.hexdigest()
def git(*args):return subprocess.check_output(['git','-C',str(ROOT),*args],text=True).strip()
def check():
    x=json.loads(OUT.read_text())
    changed=[n for n,h in x['protected'].items() if not (ROOT/n).is_file() or sha(ROOT/n)!=h]
    assert not changed,changed
    for name,head in x['repositories'].items():
        assert git('-C',name,'rev-parse','HEAD')==head
        assert not git('-C',name,'status','--porcelain')
    return len(x['protected'])
if __name__=='__main__':
    import sys
    if '--check' in sys.argv:print('Preserved',check());raise SystemExit
    assert not OUT.exists(),'freeze is immutable'
    assert not git('diff',PIN,'--','rust'), 'Rust changed after release'
    # Central may commit dispatch documents concurrently; the context is PIN.
    paths=[]
    for base in ('validation','rust/verification'):
        for p in (ROOT/base).rglob('*'):
            parts=p.relative_to(ROOT).parts
            if p.is_file() and not any(v in parts for v in ('time_ephemeris_port','planetary_ephemeris','.git','target','.cargo-home','__pycache__')):
                # Reference build/dependency trees are pinned by their own Git and
                # accepted manifests. Protect authored/evidence files, not rebuild trees.
                if 'build' in parts and 'terrestrial_resources_port' not in parts:continue
                paths.append(p)
    OUT.parent.mkdir(parents=True,exist_ok=True)
    x=dict(release=PIN,context={n:git('rev-parse',n) for n in ('bd290cc','eb2a589','f179ce5','573d667','02234cd')},repositories={n:git('-C',n,'rev-parse','HEAD') for n in ('tempo2','validation/reference')},protected={str(p.relative_to(ROOT)):sha(p) for p in sorted(set(paths))},production={n:git('rev-parse',PIN+':'+n) for n in git('ls-tree','-r','--name-only',PIN,'rust').splitlines()},contract_sha256=sha(HERE/'CONTRACT.md'))
    OUT.write_text(json.dumps(x,indent=2)+'\n');print('Frozen',check(),sha(OUT))
