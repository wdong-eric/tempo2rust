"""Reviewable library-owned cache lifetime counterfactual; production stays bound."""
import difflib
import json
import shutil
from pathlib import Path
from common import sha

GUARD='''// MPFR caches belong to the calling thread. Foreign pthread callers do not
// run Rug's Rust-thread examples, so register cleanup at the numeric C boundary.
struct ForeignThreadCache;
impl Drop for ForeignThreadCache {
    fn drop(&mut self) {
        rug::float::free_cache(rug::float::FreeCache::Local);
    }
}
thread_local! {
    static FOREIGN_THREAD_CACHE: ForeignThreadCache = const { ForeignThreadCache };
}
'''


def prepare(prefix, out, run):
    source=prefix/'share/tempo2-rust/rust';candidate=out/'library-owned-cache-source'
    shutil.copytree(source,candidate)
    path=candidate/'src/predictor_ffi.rs';before=path.read_text()
    marker='fn finish(f: impl FnOnce() -> Result<(), String>) -> c_int {'
    old='match catch_unwind(AssertUnwindSafe(f)) {'
    assert before.count(marker)==before.count(old)==1
    after=before.replace(marker,GUARD+marker).replace(old,
        'match catch_unwind(AssertUnwindSafe(|| {\n        FOREIGN_THREAD_CACHE.with(|_| {});\n        f()\n    })) {')
    path.write_text(after)
    (out/'library-owned-cache.patch').write_text(''.join(difflib.unified_diff(before.splitlines(True),after.splitlines(True),fromfile='installed/src/predictor_ffi.rs',tofile='diagnostic/src/predictor_ffi.rs')))
    install=json.loads((prefix.parent/(prefix.name+'-build')/'install.json').read_text())
    command=['cargo','build','--offline','--locked','--release','--lib','--manifest-path',candidate/'Cargo.toml','--target-dir',out/'cache-target']
    if install['features']:command+=['--features',install['features']]
    result=run('library-owned-cache-build',command,out,timeout=900)
    assert result.returncode==0,result.stderr
    archive=out/'cache-target/release/libtempo2_rust.a'
    identities={str(p.relative_to(source)):sha(p) for p in source.rglob('*') if p.is_file()}
    actual={str(p.relative_to(candidate)):sha(p) for p in candidate.rglob('*') if p.is_file()}
    changed=[p for p in identities if identities[p]!=actual[p]]
    assert identities.keys()==actual.keys() and changed==['src/predictor_ffi.rs'],changed
    return archive,{'source':identities,'changed':changed,'guard_source':sha(path),'archive':sha(archive),
        'patch':sha(out/'library-owned-cache.patch'),'scope':'Diagnostic private copy, one thread-local destructor; no production or public API change'}
