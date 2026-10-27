"""Recheck final receipt artifacts and source bindings without rewriting history."""
from pathlib import Path
import hashlib,json,subprocess,sys
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent;BUILD=HERE/'build'
out=Path(sys.argv[1]).resolve();out.mkdir(parents=True,exist_ok=False)
selected=['qualify-release-v2/receipt.json','makefile-v3/receipt.json','native-v3/identity.json','native-v4/identity.json','native-timed-v1/identity.json','native-warning-v1/receipt.json','timed-control-v1/receipt.json','consumers-v2/receipt.json','providers-v1/paired/receipt.json','providers-v2/receipt.json','c-concurrency-v2/receipt.json','performance-v2/receipt.json','evaluation-v1/receipt.json','installed-v1/receipt.json','rust-worker-calibration/receipt.json','mpfr-profile/receipt.json']
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();checked={};failures=[];relocations=[]
old_binary=ROOT/'rust/target/release/examples/parallel_execution';old_hash='666af72fe643dba9df55c3a1de9605a575c1d4bfe9773e795fb025d52da26585'
def check(path,expected,origin):
 path=Path(path)
 if path==old_binary and expected==old_hash:
  replacement=BUILD/'source-snapshots/parallel_execution-qualification';relocations.append(dict(original=str(path),retained=str(replacement),sha256=expected));path=replacement
 actual=sha(path) if path.is_file() else None
 if actual!=expected:failures.append(dict(path=str(path),expected=expected,actual=actual,receipt=origin))
 checked[str(path)]=actual
for name in selected:
 p=BUILD/name;r=json.loads(p.read_text());checked[str(p)]=sha(p)
 for key in ['files','artifacts','sources','source','inputs','binaries','dependencies','predecessor_receipt','calibration_identity','paired_receipt','sources_and_inputs']:
  values=r.get(key,{})
  if not isinstance(values,dict):continue
  for file,digest in values.items():
   if not isinstance(digest,str) or len(digest)!=64:continue
   path=Path(file)
   if not path.is_absolute():path=p.parent/path
   check(path,digest,name)
 if 'installed_source' in r:
  for file,digest in r['installed_source'].items():check(p.parent/'prefix/share/tempo2-rust/rust'/file,digest,name)
 if 'test_executable' in r:check(r['test_executable'],r['executable_sha256'],name)
# Bind the generated Makefiles/objects as well as the executed top-level logs.
for mode,object_name in [('original','calculate_bclt.o'),('corrected','libtempo2_bclt_la-calculate_bclt.o')]:
 for name in ['Makefile.am','Makefile','config.status',object_name]:
  p=BUILD/'makefile-v3'/mode/name;checked[str(p)]=sha(p)
production={}
paths=subprocess.check_output(['git','diff-tree','--no-commit-id','--name-only','-r','81de1e6','--','rust/src'],cwd=ROOT,text=True).splitlines()
for name in paths:
 expected=hashlib.sha256(subprocess.check_output(['git','show','81de1e6:'+name],cwd=ROOT)).hexdigest();check(ROOT/name,expected,'production 81de1e6');production[name]=expected
pins={}
for name in ['tempo2','validation/reference/build/source']:
 pins[name]=dict(head=subprocess.check_output(['git','-C',str(ROOT/name),'rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','-C',str(ROOT/name),'status','--short'],text=True))
 assert not pins[name]['status'],name
receipt=dict(selected=selected,checked=checked,unique_files=len(checked),relocations=relocations,production=production,pins=pins,failures=failures)
(out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');print('checked',len(checked),'unique files; mismatches',len(failures))
for failure in failures:print(failure)
assert not failures
