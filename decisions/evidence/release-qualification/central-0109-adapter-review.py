#!/usr/bin/env python3
"""Freeze and run focused release-adapter checks for AL's first 0109 commits."""
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile

root=Path('/Users/Eric.Dong/Downloads/tempo2rust')
out=Path('/tmp/tempo2-central-0109-adapter-review-20261101');out.mkdir()
shutil.copyfile(__file__,out/'executed-review.py')
revision='92862b0dc71323608928d0d1c8c9e857fbecdfd1'
archive=out/'candidate.tar'
with archive.open('xb') as stream:subprocess.run(['git','archive','--format=tar',revision],cwd=root,stdout=stream,check=True)
source=out/'source';source.mkdir()
with tarfile.open(archive) as bundle:bundle.extractall(source,filter='data')
command=[sys.executable,'-m','unittest','discover','-s',str(source/'validation/release'),'-p','test_*.py','-v']
with (out/'tests.stdout').open('xb') as stdout,(out/'tests.stderr').open('xb') as stderr:
 result=subprocess.run(command,cwd=source,stdout=stdout,stderr=stderr,env={**__import__('os').environ,'PYTHONDONTWRITEBYTECODE':'1'},timeout=180)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
receipt={'scope':'Frozen AL release-adapter unit tests and source review; no native/Rust integrated or Linux execution',
    'revision':revision,'archive_sha256':sha(archive),'command':command,'status':result.returncode,
    'stdout_sha256':sha(out/'tests.stdout'),'stderr_sha256':sha(out/'tests.stderr'),
    'recipe_sha256':sha(Path(__file__)),'completed_utc':datetime.now(timezone.utc).isoformat()}
(out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print((out/'tests.stderr').read_text()[-6500:])
print(json.dumps(receipt,indent=2))
assert result.returncode==0
