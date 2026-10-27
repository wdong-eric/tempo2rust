"""Review committed AK artifact identities and fresh installed consumers."""
from pathlib import Path
import hashlib
import json
import os
import re
import shutil
import subprocess
import tempfile
import time

ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
BUILD = ROOT/'validation/parallel_execution/build'
REV = subprocess.check_output(['git','rev-parse','eaa30c5'],cwd=ROOT,text=True).strip()
OUT = Path(tempfile.mkdtemp(prefix='tempo2-ak-handover-review-'))
Path('/tmp/tempo2-ak-handover-review-current').write_text(str(OUT)+'\n')
env = dict(os.environ, PATH='/Users/Eric.Dong/.cargo/bin:/opt/homebrew/bin:'+os.environ.get('PATH',''),
           CARGO_HOME=str(ROOT/'rust/.cargo-home'), CARGO_NET_OFFLINE='true', CARGO_BUILD_JOBS='2',
           FC='/opt/homebrew/bin/gfortran-16', CC='/usr/bin/clang', LC_ALL='C', VECLIB_MAXIMUM_THREADS='1')
for key in ['CARGO_TARGET_DIR','CALCEPH_DIR','TEMPO2_ACCELERATE','TEMPO2','TEMPO2_CLOCK_PATH',
            'RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','AK_WORKERS','TEMPO2_AC_TEST']:
    env.pop(key,None)

def sha(path):
    h=hashlib.sha256()
    with Path(path).open('rb') as f:
        for block in iter(lambda:f.read(1048576),b''):
            h.update(block)
    return h.hexdigest()

commands=[]
def run(label,cmd,cwd=OUT,timeout=180):
    cmd=list(map(str,cmd));start=time.monotonic()
    with (OUT/(label+'.stdout')).open('wb') as stdout,(OUT/(label+'.stderr')).open('wb') as stderr:
        r=subprocess.run(cmd,cwd=cwd,env=env,stdout=stdout,stderr=stderr,timeout=timeout)
    commands.append(dict(label=label,command=cmd,cwd=str(cwd),exit=r.returncode,seconds=time.monotonic()-start,
                         stdout_sha256=sha(OUT/(label+'.stdout')),stderr_sha256=sha(OUT/(label+'.stderr'))))
    (OUT/'commands.json').write_text(json.dumps(commands,indent=2)+'\n')
    print(label,r.returncode,flush=True)
    assert r.returncode==0,label

print('Central handover review: '+str(OUT),flush=True)
run('owner-audit',['python3','-B',ROOT/'validation/parallel_execution/audit.py',OUT/'owner-audit'],ROOT)
owner=json.loads((OUT/'owner-audit/receipt.json').read_text())
checked=dict(owner['checked']);supplement={}
for name in owner['selected']:
    p=BUILD/name;d=json.loads(p.read_text())
    for key in ['identities','runner']:
        for location,expected in d.get(key,{}).items():
            assert re.fullmatch('[0-9a-f]{64}',expected),(name,key,location)
            path=Path(location)
            if not path.is_absolute():path=p.parent/path
            actual=sha(path);assert actual==expected,(name,key,location)
            checked[str(path)]=actual;supplement[str(path)]=actual

git_files={}
for line in subprocess.check_output(['git','ls-tree','-r',REV,'rust','validation/parallel_execution'],cwd=ROOT,text=True).splitlines():
    meta,name=line.split('\t',1);mode,kind,oid=meta.split();assert kind=='blob',name
    data=(ROOT/name).read_bytes()
    assert hashlib.sha1(b'blob '+str(len(data)).encode()+b'\0'+data).hexdigest()==oid,name
    git_files[name]=sha(ROOT/name);checked[str(ROOT/name)]=git_files[name]
prefix=BUILD/'installed-v1/prefix'
installed=json.loads((BUILD/'installed-v1/receipt.json').read_text())
for name,expected in installed['installed_source'].items():
    assert git_files['rust/'+name]==expected,name
    path=prefix/'share/tempo2-rust/rust'/name
    assert sha(path)==expected,name
    checked[str(path)]=expected
pins={}
for name,expected in [('tempo2','fda6ad4a9753ad7dc5d291406f2f310a9161423f'),
                      ('validation/reference','7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572')]:
    head=subprocess.check_output(['git','-C',str(ROOT/name),'rev-parse','HEAD'],text=True).strip()
    status=subprocess.check_output(['git','-C',str(ROOT/name),'status','--porcelain'],text=True)
    assert head==expected and not status,name
    pins[name]=dict(head=head,status=status)

qualification=json.loads((BUILD/'qualify-release-v2/receipt.json').read_text())
assert len(qualification['records'])==99 and len(qualification['results'])==33
assert all(r['exit']==0 for r in qualification['records'])
assert sum(r['checks'] for r in qualification['results'])==484314
assert all(not r['failures'] for r in qualification['results'])
perf=json.loads((BUILD/'performance-v2/receipt.json').read_text())
assert len(perf['records'])==108 and all(r['exit']==0 for r in perf['records'])
for r in perf['records']:
    expected='matched CLI input/output workload' if r['case'] in ['correlated-global','cheby','polyco'] else 'distinct post-evaluation output workloads; compare load/evaluation intervals only'
    assert r['process_comparison']==expected
control=json.loads((BUILD/'timed-control-v1/receipt.json').read_text())
assert len(control['records'])==12
for r in control['records']:
    assert sha(r['command'][0])==r['executable']
    folder=BUILD/'timed-control-v1'/f"{r['case']}-{r['workers']}"
    for name,digest in r['product_hashes'].items():assert sha(folder/name)==digest
serial=(BUILD/'native-timed-v1/serial-symbols.log').read_text()
parallel=(BUILD/'native-timed-v1/parallel-symbols.log').read_text()
assert 'ak_' not in serial+parallel and 'GOMP_parallel' not in serial and 'GOMP_parallel' in parallel
audit=dict(revision=REV,owner_audit_sha256=sha(OUT/'owner-audit/receipt.json'),checked=checked,
           unique_files=len(checked),supplement=supplement,git_files=git_files,installed_git_bindings=len(installed['installed_source']),
           pins=pins,failures=[],script_sha256=sha(__file__))
(OUT/'central-audit.json').write_text(json.dumps(audit,indent=2)+'\n')
print('Bound',len(checked),'files,',len(git_files),'Git files and',len(installed['installed_source']),'installed source files',flush=True)

client=OUT/'client';(client/'src/bin').mkdir(parents=True)
for name in ['Cargo.toml','Cargo.lock']:
    shutil.copy2(BUILD/'installed-v1/client'/name,client/name)
shutil.copy2(ROOT/'validation/parallel_execution/concurrent.rs',client/'src/bin/concurrent.rs')
assert str(prefix/'share/tempo2-rust/rust') in (client/'Cargo.toml').read_text()
assert str(ROOT/'rust') not in (client/'Cargo.toml').read_text()
run('fresh-installed-consumer-build',['cargo','build','--offline','--locked','--release','--manifest-path',client/'Cargo.toml','--bin','concurrent'],timeout=1200)
fit=ROOT/'validation/fitting/inputs'
run('fresh-installed-concurrent',[client/'target/release/concurrent',ROOT/'validation/reference/build/source/T2runtime',
                                 fit/'pks_all.par',fit/'pks.tim',ROOT/'validation/predictors/fixtures/isolated.par',
                                 ROOT/'validation/general_fitting/inputs/covariance.model',OUT/'products'])
run('fresh-c-build',['clang','-std=c11','-O1','-I'+str(prefix/'include'),ROOT/'validation/library_interfaces/consumer.c',
                     '-L'+str(prefix/'lib'),'-ltempo2pred_rust','-Wl,-rpath,'+str(prefix/'lib'),'-o',OUT/'c-consumer'])
run('fresh-c-consumer',[OUT/'c-consumer'])
run('fresh-c-linkage',['otool','-L',OUT/'c-consumer'])
assert str(prefix/'lib/libtempo2pred_rust.dylib') in (OUT/'fresh-c-linkage.stdout').read_text() or '@rpath/libtempo2pred_rust.dylib' in (OUT/'fresh-c-linkage.stdout').read_text()
completion=dict(revision=REV,audit_sha256=sha(OUT/'central-audit.json'),commands_sha256=sha(OUT/'commands.json'),
                source_files=len(git_files),installed_source_files=len(installed['installed_source']),checked_files=len(checked),
                consumer_binaries={str(p):sha(p) for p in [client/'target/release/concurrent',OUT/'c-consumer']},
                scope='independent identity audit and freshly compiled default-release installed Rust/C consumers; no new benchmark, Linux or inherited numerical acceptance',
                outputs={str(p.relative_to(OUT)):sha(p) for p in OUT.rglob('*') if p.is_file() and 'target' not in p.parts})
(OUT/'completion.json').write_text(json.dumps(completion,indent=2)+'\n')
print('Central installed Rust/C replay passed; '+str(OUT),flush=True)
