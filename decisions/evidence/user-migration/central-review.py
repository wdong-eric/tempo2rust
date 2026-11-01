#!/usr/bin/env python3
"""Review AM's committed documentation and replay its bound operational evidence."""
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
from urllib.parse import unquote, urlsplit

ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
OUT=Path('/tmp/tempo2-central-am-review-20261101')
REV='8f293a2667c20d80cd6b1b608f6a5b79734ba576'
FILES=['README.md','docs/MIGRATION.md','docs/MIGRATION_REPORT.md']
PACKET=ROOT/'docs/build/am-migration-20261101-v2'
EXPECTED={
 'identity.json':'e0297e9225e7015171ad031483415dd99468c1cb9b817caf7b71630dc2fc70e8',
 'commands.json':'b0b57ae3ede6b82e67ec13ee6e636627e8196597cebef1eb3d89336637c9eea6',
 'checks.json':'3e01220d4efd57465e7a66f40e482e5684da0efef47734ea3ffccc4f77b28d25',
 'verify_examples.py':'cac19a2add1ce87aa72caf5ae8f9d8a052a8de221e7b0fe2902b219ff0bde3ce',
 'check_examples.py':'1cebd210030dbace7cbb9acfd044db77397ce9e42084a819a9bf23b1561cb921',
}

def sha(path):
 with path.open('rb') as stream:return hashlib.file_digest(stream,'sha256').hexdigest()

def git(*args):
 return subprocess.check_output(['git','-C',str(ROOT),*args])

OUT.mkdir(exist_ok=False)
shutil.copyfile(__file__,OUT/'executed-review.py')
changed=git('diff','--name-only','1fad8ce2d2588496845bdd6928353906f2aed5e1',REV).decode().splitlines()
assert set(changed)==set(FILES),changed
assert not git('diff','fecbc18af95196a16db2046306099b6345a50976','--','rust/','validation/release/','.github/workflows/tempo2-rust.yml')
tracked=set(git('ls-tree','-r','--name-only',REV).decode().splitlines())
docs={};links=[];shells=[]
for name in FILES:
 path=ROOT/name;data=git('show',REV+':'+name)
 assert path.read_bytes()==data,name
 docs[name]=hashlib.sha256(data).hexdigest();source=data.decode()
 for target in re.findall(r'(?<!!)\[[^\]\n]*\]\(([^)\s]+)\)',source):
  url=urlsplit(target);assert not url.scheme and not target.startswith('/'),target
  destination=(path.parent/unquote(url.path)).resolve() if url.path else path
  relative=destination.relative_to(ROOT).as_posix()
  assert relative in tracked, (name,target,'missing or wrong-case tracked path')
  if url.fragment:
   headings=re.findall(r'^#{1,6}\s+(.+)$',destination.read_text(),re.M)
   assert unquote(url.fragment) in [re.sub(r'[^\w\- ]','',h.lower()).replace(' ','-') for h in headings],target
  links.append({'document':name,'target':target})
 for index,block in enumerate(re.findall(r'```sh\n(.*?)\n```',source,re.S),1):
  completed=subprocess.run(['sh','-n'],input=block.encode(),capture_output=True)
  assert completed.returncode==0,(name,index,completed.stderr)
  shells.append({'document':name,'block':index})
assert len(links)==68 and len(shells)==8
for name,digest in EXPECTED.items():assert sha(PACKET/name)==digest,name
identity=json.loads((PACKET/'identity.json').read_bytes())
commands=json.loads((PACKET/'commands.json').read_bytes())
assert commands['identity_sha256']==EXPECTED['identity.json']
assert commands['script_sha256']==EXPECTED['verify_examples.py']
assert identity['executed_source']=='a8ba6f15d2933b206f85ce70c99003b10f3f653d'
assert identity['profile']=='release' and identity['features']==''
assert sha(Path(identity['binary']))==identity['binary_sha256']=='df523407e7db73ba3dabe27377252627fafe7ae466d9922fabe95a6865c62e5f'
for name,digest in identity['receipts'].items():assert sha(ROOT/name)==digest,name
for name,digest in identity['inputs'].items():assert sha(ROOT/name)==digest,name
labels=['rustc','cargo','cc','fc','sw-vers','binary-dependencies','installer-help','help',
        'native-h','native-long-help','version-v','version-long','no-arguments',
        'native-default','native-robust','native-files-version','timing','timing-no-mean',
        'timing-fit','timing-fit-no-mean','bats']
assert [r['label'] for r in commands['commands']]==labels
fail_status={'native-h','native-long-help','version-v','version-long','no-arguments','timing-fit-no-mean'}
bound_outputs={}
for row in commands['commands']:
 label=row['label'];assert row['exit_code']==row['expected_exit']==int(label in fail_status),label
 work=PACKET/label/'work';assert work==Path(row['cwd'])
 for stream in ['stdout','stderr']:
  path=PACKET/label/stream
  assert sha(path)==row[stream+'_sha256'] and path.stat().st_size==row[stream+'_bytes'],(label,stream)
  bound_outputs[str(path.relative_to(PACKET))]={'sha256':sha(path),'bytes':path.stat().st_size}
 products={str(p.relative_to(work)):sha(p) for p in work.rglob('*') if p.is_file()}
 assert products==row['created_files'],label
 for name,digest in products.items():bound_outputs[str((work/name).relative_to(PACKET))]={'sha256':digest,'bytes':(work/name).stat().st_size}
 if label in labels[7:]:assert row['argv'][0]==identity['binary']
assert commands['unchanged_after']==['frozen tracked files','executable','input files','208 runtime files']
# Replay the unchanged checker on a private copy, preserving AM's saved files.
copy=OUT/'evidence-copy';shutil.copytree(PACKET,copy)
replay=subprocess.run([sys.executable,str(copy/'check_examples.py')],capture_output=True)
(OUT/'replay.stdout').write_bytes(replay.stdout);(OUT/'replay.stderr').write_bytes(replay.stderr)
assert replay.returncode==0,replay.stderr
assert sha(copy/'checks.json')==EXPECTED['checks.json']
checks=json.loads((copy/'checks.json').read_bytes())
assert checks['count']==50 and len(checks['diagnostic_sites'])==24
for resource in checks['observed_resources']:
 path=Path(resource['path']['text']);assert sha(path)==resource['sha256']
# Retain the initial case-insensitive directory-label failure separately.
initial=ROOT/'docs/build/am-migration-20261101'
initial_commands=json.loads((initial/'commands.json').read_bytes())
assert len(initial_commands['commands'])==9 and 'completed_utc' not in initial_commands
shutil.copytree(initial,OUT/'initial-attempt')
report={'scope':'Committed user documentation, source contract review and saved operational-evidence replay; no fresh engine/build/scientific qualification',
 'revision':REV,'authored_paths':changed,'document_hashes':docs,'case_sensitive_relative_links':len(links),
 'shell_syntax_blocks':len(shells),'recorded_commands':len(labels),'main_cli_invocations':14,
 'expected_statuses_and_complete_outputs_verified':True,'replayed_output_assertions':checks['count'],
 'actual_installed_emit_sites':len(checks['diagnostic_sites']),'retained_initial_commands':9,
 'bound_output_files':len(bound_outputs),'executed_installation_source':identity['executed_source'],
 'binary_sha256':identity['binary_sha256'],'frozen_implementation_unchanged':True,
 'evidence_hashes':EXPECTED,'recipe_sha256':sha(OUT/'executed-review.py'),
 'limitations':['No new build/install or Linux execution','Prior complete installation audit reused; only recorded receipts, executable and consumed inputs/resources rehashed here',
               'No Markdown renderer run','Installed package does not include the new root/docs pages','Scientific and milestone acceptance remain separate'],
 'completed_utc':datetime.now(timezone.utc).isoformat()}
(OUT/'selected-outputs.json').write_text(json.dumps(bound_outputs,indent=2)+'\n')
(OUT/'links.json').write_text(json.dumps(links,indent=2)+'\n')
(OUT/'receipt.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
print('receipt_sha256',sha(OUT/'receipt.json'))
