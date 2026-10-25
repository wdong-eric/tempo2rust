"""Immutable accepted inputs and pre-edit AA source/caller/field map."""
import hashlib,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
OUT=HERE/'build/freeze.json'
PIN='4498cf1c4bb9167a57e4cb3b1226cbd73e3ee2d3'
def sha(p):
    h=hashlib.sha256()
    with p.open('rb') as f:
        for b in iter(lambda:f.read(1024*1024),b''):h.update(b)
    return h.hexdigest()
def git(*args):return subprocess.check_output(['git','-C',str(ROOT),*args],text=True).strip()
def check():
    x=json.loads(OUT.read_text())
    drift=[n for n,h in x['protected'].items() if not (ROOT/n).is_file() or sha(ROOT/n)!=h]
    assert not drift,drift
    for n,h in x['repositories'].items():
        assert git('-C',n,'rev-parse','HEAD')==h
        assert not git('-C',n,'status','--porcelain')
    return len(x['protected'])
if __name__=='__main__':
    import sys
    if '--check' in sys.argv:print('Preserved',check());raise SystemExit
    assert not OUT.exists()
    assert not git('diff',PIN,'--','rust')
    paths=[]
    for base in ('validation','rust/verification'):
        for p in (ROOT/base).rglob('*'):
            parts=p.relative_to(ROOT).parts
            if not p.is_file() or any(v in parts for v in ('observer_coordinates_port','propagation','.git','target','.cargo-home','__pycache__')):continue
            if 'build' in parts and not any(v in parts for v in ('observer_coordinates','time_ephemeris_port','terrestrial_resources_port')):continue
            paths.append(p)
    # Central may evolve its scripts; only the already completed correction bundle
    # is an accepted immutable numerical input, not the central directory.
    paths.extend(p for p in (ROOT/'decisions/evidence/observer-whitespace/build/central-il7tunxj').rglob('*') if p.is_file())
    mapping=json.loads((ROOT/'validation/observer_coordinates/outputs/frozen/source_map.json').read_text())
    consumers={n:git('show',PIN+':'+n) for n in ('rust/src/get_obs_coord.rs','rust/src/form_bats.rs','rust/src/runtime.rs','rust/src/pulsar_lifecycle.rs','rust/src/clkcorr.rs','rust/src/tt2tdb.rs','rust/src/observatory.rs')}
    OUT.parent.mkdir(parents=True,exist_ok=True)
    OUT.write_text(json.dumps(dict(release=PIN,source_map=mapping,accepted_y_consumers=consumers,
      rust_blobs={n:git('rev-parse',PIN+':'+n) for n in git('ls-tree','-r','--name-only',PIN,'rust').splitlines()},
      repositories={n:git('-C',n,'rev-parse','HEAD') for n in ('tempo2','validation/reference')},
      protected={str(p.relative_to(ROOT)):sha(p) for p in sorted(set(paths))},contract_sha256=sha(HERE/'CONTRACT.md')),indent=2)+'\n')
    print('Frozen',check(),sha(OUT))
