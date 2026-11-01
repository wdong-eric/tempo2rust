#!/usr/bin/env python3
"""Verify AM's two README clarifications and preservation of accepted evidence."""
from datetime import datetime,timezone
import hashlib,json,re,shutil,subprocess
from pathlib import Path
from urllib.parse import urlsplit,unquote
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
OUT=Path('/tmp/tempo2-central-am-followup-review-20261101')
REV='f792b79974f387feb6dcf7bc4f3a88bcf4637f19'
BASE='8b323ba1df63cae83cdff78acc55429a16af0862'
PACKET=ROOT/'docs/build/am-readme-followup-20261101'
FILES=['docs/MIGRATION_REPORT.md','rust/README.md']
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def git(*args):return subprocess.check_output(['git','-C',str(ROOT),*args])
OUT.mkdir(exist_ok=False);shutil.copyfile(__file__,OUT/'executed-review.py')
assert git('rev-parse',REV+'^').decode().strip()==BASE
assert git('diff','--name-only',BASE,REV).decode().splitlines()==FILES
assert git('diff','--name-only','fecbc18',REV,'--','rust/','validation/release/','.github/workflows/tempo2-rust.yml').decode().splitlines()==['rust/README.md']
subprocess.run(['git','-C',str(ROOT),'diff','--check',BASE,REV],check=True)
assert len(re.findall(rb'^@@ ',git('diff','--unified=0',BASE,REV,'--','rust/README.md'),re.M))==2
old_report=git('show',BASE+':docs/MIGRATION_REPORT.md')
assert (ROOT/'docs/MIGRATION_REPORT.md').read_bytes().startswith(old_report)
tracked=set(git('ls-tree','-r','--name-only',REV).decode().splitlines());links=[];shells=[];hashes={}
for name in FILES:
 p=ROOT/name;current=p.read_bytes();assert current==git('show',REV+':'+name)
 prior=git('show',BASE+':'+name).decode();source=current.decode();hashes[name]=sha(p)
 assert re.findall(r'```[^\n]*\n(.*?)\n```',source,re.S)==re.findall(r'```[^\n]*\n(.*?)\n```',prior,re.S)
 for target in re.findall(r'(?<!!)\[[^\]\n]*\]\(([^)\s]+)\)',source):
  url=urlsplit(target)
  if url.scheme:continue
  assert not target.startswith('/')
  dest=(p.parent/unquote(url.path)).resolve() if url.path else p
  assert dest.relative_to(ROOT).as_posix() in tracked,(name,target)
  if url.fragment:
   headings=re.findall(r'^#{1,6}\s+(.+)$',dest.read_text(),re.M)
   assert unquote(url.fragment) in [re.sub(r'[^\w\- ]','',h.lower()).replace(' ','-') for h in headings],target
  links.append((name,target))
 for block in re.findall(r'```sh\n(.*?)\n```',source,re.S):
  result=subprocess.run(['sh','-n'],input=block.encode(),capture_output=True)
  assert result.returncode==0,result.stderr;shells.append(name)
assert len(links)==66 and len(shells)==12
previous=ROOT/'decisions/evidence/user-migration/build/am-review-20261101'
count=0
for saved,current in [('initial-attempt','am-migration-20261101'),('evidence-copy','am-migration-20261101-v2')]:
 a=previous/saved;b=ROOT/'docs/build'/current
 old={str(p.relative_to(a)):sha(p) for p in a.rglob('*') if p.is_file()}
 new={str(p.relative_to(b)):sha(p) for p in b.rglob('*') if p.is_file()}
 assert old==new,current;count+=len(old)
assert count==73
receipt=json.loads((PACKET/'receipt.json').read_bytes())
assert receipt['before_manifest_sha256']==sha(PACKET/'before.json')
assert receipt['recipe_sha256']==sha(PACKET/'check.py')
shutil.copytree(PACKET,OUT/'am-followup-evidence')
report={'scope':'Documentation-only follow-up review; no engine/build/scientific execution',
 'revision':REV,'parent':BASE,'changed_paths':FILES,'readme_hunks':2,'document_hashes':hashes,
 'relative_links':len(links),'shell_syntax_blocks':len(shells),'code_blocks_unchanged':True,
 'report_append_only':True,'original_evidence_files_preserved':count,
 'protected_tree_exceptions':['rust/README.md'],'am_receipt_sha256':sha(PACKET/'receipt.json'),
 'recipe_sha256':sha(OUT/'executed-review.py'),'completed_utc':datetime.now(timezone.utc).isoformat()}
(OUT/'receipt.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2));print('receipt_sha256',sha(OUT/'receipt.json'))
