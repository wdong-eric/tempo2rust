#!/usr/bin/env python3
"""Independently bind AL's installed macOS cache campaigns and compare full products."""
import argparse
import ast
from datetime import datetime, timezone
import hashlib
import io
import json
from pathlib import Path
import shutil
import subprocess
import tarfile

p=argparse.ArgumentParser(description=__doc__)
p.add_argument('--root',type=Path,required=True)
p.add_argument('--out',type=Path,required=True)
a=p.parse_args();a.out.mkdir(parents=True,exist_ok=False)
shutil.copyfile(__file__,a.out/'executed-review.py')
root=a.root.resolve();build=root/'validation/release/build'
sha=lambda b:hashlib.sha256(b).hexdigest()
file_sha=lambda p:sha(p.read_bytes())
def git(*args):
    return subprocess.check_output(['git','-C',str(root),*args])
revision='a8ba6f15d2933b206f85ce70c99003b10f3f653d'
candidate=build/'third-cache-candidate-v1'
export=json.loads((candidate/'receipt.json').read_text())
assert export['candidate']==revision and export['status']=='pass'
archive=git('archive','--format=tar',revision)
assert sha(archive)==export['archive_sha256']
with tarfile.open(fileobj=io.BytesIO(archive)) as bundle:
    source={m.name:sha(bundle.extractfile(m).read()) for m in bundle if m.isfile()}
assert source==export['source']
for name,digest in source.items():
    assert file_sha(candidate/'source'/name)==digest,name
# Reconstruct the private guard measured in the third Linux run from its
# immutable original recipe. Compare every code line, ignoring only full-line
# comments and blank lines; do not reduce this check to a symbol search.
old=git('show','a35104a:rust/src/predictor_ffi.rs').decode()
recipe=ast.parse(git('show','a35104a:validation/release/cache_lifetime.py'))
guard=next(ast.literal_eval(n.value) for n in recipe.body if isinstance(n,ast.Assign)
           and any(isinstance(t,ast.Name) and t.id=='GUARD' for t in n.targets))
marker='fn finish(f: impl FnOnce() -> Result<(), String>) -> c_int {'
body='match catch_unwind(AssertUnwindSafe(f)) {'
assert old.count(marker)==old.count(body)==1
private=old.replace(marker,guard+marker).replace(body,
    'match catch_unwind(AssertUnwindSafe(|| {\n        FOREIGN_THREAD_CACHE.with(|_| {});\n        f()\n    })) {')
assert sha(private.encode())=='9cef7c936bb5e92bd5d991f125190ee24e4e7afccd821521cbee2a9683e8d91e'
production=git('show',revision+':rust/src/predictor_ffi.rs')
code=lambda text:[s for s in text.splitlines() if s.strip() and not s.lstrip().startswith('//')]
assert code(private)==code(production.decode())

expected_keys={(i,j) for i in range(4) for j in range(3)}
def products(directory,mode):
    # Sequential parsing independently checks complete, non-nested blocks and
    # exact key membership. Scheduling order is the only discarded information.
    blocks={};key=None;body=[];outside=[]
    for line in (directory/(mode+'-execute.stdout')).read_bytes().splitlines(keepends=True):
        if line.startswith(b'AK_BEGIN '):
            assert key is None
            _,i,j=line.split();key=(int(i),int(j));assert key not in blocks
            body=[]
        elif line.startswith(b'AK_END '):
            _,i,j=line.split();assert key==(int(i),int(j))
            assert body;blocks[key]=b''.join(body);key=None
        elif key is None:outside.append(line)
        else:body.append(line)
    assert key is None and set(blocks)==expected_keys
    assert b''.join(outside)==b'AK four caller threads completed twelve exclusive full workflows\n'
    storage={q.name:q.read_bytes() for q in (directory/mode).glob('*.tsv')}
    assert set(storage)=={f'storage-{i}-{j}.tsv' for i,j in expected_keys}
    stderr=(directory/(mode+'-execute.stderr')).read_bytes()
    assert not any(q in stderr for q in [b'ERROR: AddressSanitizer',b'runtime error:',b'WARNING: ThreadSanitizer',b'ERROR: LeakSanitizer'])
    return blocks,storage

reviews=[];all_products=[]
for prefix_name,check_name,features in [
    ('third-cache-default-v1','third-cache-default-check-v2',''),
    ('third-cache-combined-v2','third-cache-combined-check-v1','calceph,lapack')]:
    prefix=build/prefix_name;directory=build/check_name
    install_path=build/(prefix_name+'-build')/'install.json'
    install=json.loads(install_path.read_text());receipt_path=directory/'receipt.json'
    receipt=json.loads(receipt_path.read_text())
    assert install['prefix']==str(prefix) and install['features']==features
    assert install['profile']=='release' and 'arm64' in install['platform']
    installed={str(q.relative_to(prefix)):file_sha(q) for q in prefix.rglob('*') if q.is_file()}
    assert installed==install['files']
    installed_sources={n.removeprefix('share/tempo2-rust/'):d for n,d in installed.items()
                       if n.startswith('share/tempo2-rust/')}
    assert all(source[n]==d for n,d in installed_sources.items())
    manifest=str(candidate/'source/rust/Cargo.toml')
    assert all(manifest in cmd for cmd in install['commands'] if cmd[0]=='cargo')
    for name,digest in receipt['inputs'].items():
        q=Path(name);assert file_sha(q)==digest
        if not q.is_relative_to(prefix):assert source[str(q.relative_to(root))]==digest
    actual={str(q.relative_to(directory)):file_sha(q) for q in directory.rglob('*')
            if q.is_file() and q!=receipt_path}
    assert actual==receipt['files']
    records={r['label']:r for r in receipt['records']}
    assert len(records)==len(receipt['records'])==23
    assert all(r['exit']==0 for r in records.values())
    archive_path=str(prefix/'lib/libtempo2pred_rust.a')
    for mode in ['plain','asan','tsan','asan-cache-cleanup']:
        assert archive_path in records[mode+'-link']['command']
        assert records[mode+'-execute']['command']==[str(directory/mode/'callers')]
    assert '-fsanitize=address,undefined' in records['asan-link']['command']
    assert '-fsanitize=thread' in records['tsan-link']['command']
    values=[products(directory,mode) for mode in ['plain','asan','tsan','asan-cache-cleanup']]
    assert all(v==values[0] for v in values)
    all_products.append(values[0])
    reviews.append({'prefix':prefix_name,'checks':check_name,'features':features,
        'install_receipt_sha256':file_sha(install_path),'check_receipt_sha256':file_sha(receipt_path),
        'archive_sha256':file_sha(prefix/'lib/libtempo2pred_rust.a'),
        'installed_files':len(installed),'bound_installed_sources':len(installed_sources),
        'verified_check_files':len(actual),'commands':len(records),'all_commands_exit_zero':True,
        'complete_keyed_workflows_per_variant':len(values[0][0]),'storage_files_per_variant':len(values[0][1]),
        'four_variants_exact_products':True})
assert all_products[0]==all_products[1]
failed=[build/'third-followthrough-v1/cache-default-check.stderr',
        build/'third-followthrough-v1/cache-combined-install.stderr',
        build/'third-cache-combined-v1-build/build.log']
assert 'FileNotFoundError' in failed[0].read_text() and 'install.json' in failed[0].read_text()
assert 'TEMPO2_ACCELERATE=1' in failed[2].read_text()
summary={'scope':'Independent review of saved installed macOS C campaigns; no new execution or Linux acceptance',
    'candidate':revision,'candidate_receipt_sha256':file_sha(candidate/'receipt.json'),
    'archive_sha256':sha(archive),'verified_export_sources':len(source),
    'production_guard_sha256':sha(production),'measured_private_guard_sha256':sha(private.encode()),
    'private_production_code_equal_excluding_whole_line_comments':True,
    'reviews':reviews,'default_combined_exact_products':True,
    'sanitizer_scope':'C clients and C shim; Rust, MPFR and providers are not sanitizer-instrumented',
    'retained_initial_failures':{str(q.relative_to(root)):file_sha(q) for q in failed},
    'recipe_sha256':file_sha(Path(__file__)),'completed_utc':datetime.now(timezone.utc).isoformat()}
(a.out/'receipt.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(summary,indent=2))
print('receipt_sha256',file_sha(a.out/'receipt.json'))
