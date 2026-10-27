#!/usr/bin/env python3
"""Read-only final AJ receipt/source/artifact audit; writes a fresh audit directory."""
from pathlib import Path
import argparse, hashlib, json, os, subprocess
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent;BUILD=HERE/'build'
p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True);p.add_argument('--production',required=True);a=p.parse_args()
out=a.out.resolve();out.mkdir(parents=True,exist_ok=False);checked={};receipts={}
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def check(path,expected):
 path=Path(path).resolve();actual=sha(path);assert actual==expected,(str(path),expected,actual);checked[str(path)]=actual

def receipt(name):
 path=BUILD/name/'receipt.json';d=json.loads(path.read_text());receipts[str(path)]=sha(path);return d
prefixes={profile:BUILD/'profiles-final-v1'/(profile+'-prefix') for profile in ['debug','release']}
for name in ['final-abi-debug-v4','final-abi-release-v3']:
 d=receipt(name)
 for key in ['observer_sources','prefix_artifacts']:
  for path,digest in d[key].items():check(path,digest)
 for path,digest in d['evidence'].items():check(BUILD/name/path,digest)
 check(HERE/'qualify.py',d['script']);assert len(d['symbols'])==64 and len(d['rejected_controls'])==12
for name,profile in [('final-rust-debug','debug'),('final-rust-release','release'),('final-rust-combined','debug')]:
 d=receipt(name);installed=prefixes[profile]/'share/tempo2-rust/rust'
 for path,digest in d['source'].items():check(HERE/(path+'.rs') if path in ['resources','core'] else path,digest)
 for path,digest in d['installed_tree'].items():check(installed/path,digest)
 binary_dir=BUILD/name/'client/target'/('debug' if name=='final-rust-debug' else 'release')
 for path,digest in d['binaries'].items():check(binary_dir/path,digest)
 check(HERE/'qualify_rust.py',d['script'])
 # Bind output and generated scientific files now; Cargo build caches are not evidence.
 for path in (BUILD/name).rglob('*'):
  if path.is_file() and 'client' not in path.relative_to(BUILD/name).parts:checked[str(path)]=sha(path)
for name,script in [('final-sanitizers-v3','sanitize.py'),('final-checks','verify.py')]:
 d=receipt(name);check(HERE/script,d['script'])
 for key in ['source','artifacts']:
  for path,digest in d.get(key,{}).items():check(path,digest)
 for path,digest in d['evidence'].items():check(BUILD/name/path,digest)
d=receipt('final-installed-backend-pairs');assert d['failures']==0 and not d['changed_artifacts']
for key in ['sources_and_inputs','files']:
 for path,digest in d[key].items():check(path,digest)
# Verify installed source membership and exact Git blob content at the production commit.
listing=subprocess.check_output(['git','ls-tree','-r',a.production,'rust'],cwd=ROOT,text=True)
blobs={line.split('\t')[1]:line.split()[2] for line in listing.splitlines()}
for profile,prefix in prefixes.items():
 installed=prefix/'share/tempo2-rust/rust'
 for path in installed.rglob('*'):
  if path.is_file():
   name='rust/'+str(path.relative_to(installed));data=path.read_bytes()
   assert hashlib.sha1(b'blob '+str(len(data)).encode()+b'\0'+data).hexdigest()==blobs[name],name
   checked[str(path)]=sha(path)
 for name in ['LICENSE','MT19937_LICENSE.txt','jpl_source_manifest.json']:assert (installed/name).read_bytes()==(ROOT/'rust'/name).read_bytes()
# Record actual loaded Accelerate images and provider identities for the external client.
binary=BUILD/'final-rust-combined/client/target/release/native_backends'
env={**os.environ,'DYLD_PRINT_LIBRARIES':'1'}
r=subprocess.run([binary,'kernels',out/'loaded-kernels.tsv'],capture_output=True,env=env)
assert r.returncode==0 and b'Accelerate' in r.stderr
(out/'loaded.stdout').write_bytes(r.stdout);(out/'loaded.stderr').write_bytes(r.stderr)
r=subprocess.run(['otool','-L',binary],capture_output=True);assert r.returncode==0
(out/'dependencies.txt').write_bytes(r.stdout)
for path in [ROOT/'validation/native_backends/build/provider/lib/libcalceph.a',ROOT/'validation/native_backends/build/provider/include/calceph.h',ROOT/'validation/native_backends/build/inpop17a_TDB_m100_p100_tt.dat']:
 checked[str(path)]=sha(path)
for repo in ['tempo2','validation/reference']:
 assert not subprocess.check_output(['git','-C',ROOT/repo,'status','--porcelain'])
(out/'audit.json').write_text(json.dumps({'production_commit':a.production,'script':sha(Path(__file__)),'receipts':receipts,'files':checked,'unique_files':len(checked),'failures':[],'backend_cases':len(d['records']),'backend_comparisons':sum(row['comparison']['checks'] for row in d['records']),'loaded_evidence':{p.name:sha(p) for p in out.iterdir() if p.is_file()}},indent=2)+'\n')
print(len(checked),'unique files checked; no hash/source mismatches')
