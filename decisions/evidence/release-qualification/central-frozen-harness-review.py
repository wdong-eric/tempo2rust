#!/usr/bin/env python3
"""Run release harness controls from an exact Git export before native bootstrap."""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile

p=argparse.ArgumentParser(description=__doc__)
p.add_argument('--root',type=Path,required=True)
p.add_argument('--revision',required=True)
p.add_argument('--out',type=Path,required=True)
a=p.parse_args();a.out.mkdir(parents=True,exist_ok=False)
shutil.copyfile(__file__,a.out/'executed-review.py')
revision=subprocess.check_output(['git','-C',str(a.root),'rev-parse',a.revision+'^{commit}'],text=True).strip()
archive=a.out/'candidate.tar'
with archive.open('xb') as stream:
    subprocess.run(['git','-C',str(a.root),'archive','--format=tar',revision],stdout=stream,check=True)
source=a.out/'source';source.mkdir()
with tarfile.open(archive) as bundle:bundle.extractall(source,filter='data')
assert not (source/'tempo2').exists()
command=[sys.executable,'-m','unittest','discover','-s',str(source/'validation/release'),'-p','test_*.py','-v']
with (a.out/'tests.stdout').open('xb') as stdout,(a.out/'tests.stderr').open('xb') as stderr:
    result=subprocess.run(command,cwd=source,stdout=stdout,stderr=stderr,
        env=dict(os.environ,PYTHONDONTWRITEBYTECODE='1'),timeout=180)
sha=lambda path:hashlib.sha256(path.read_bytes()).hexdigest()
receipt={'scope':'Frozen release-harness unit checks without a native checkout; no scientific or Linux qualification',
    'revision':revision,'archive_sha256':sha(archive),'command':command,'status':result.returncode,
    'stdout_sha256':sha(a.out/'tests.stdout'),'stderr_sha256':sha(a.out/'tests.stderr'),
    'recipe_sha256':sha(Path(__file__)),'completed_utc':datetime.now(timezone.utc).isoformat()}
(a.out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print((a.out/'tests.stderr').read_text())
print(json.dumps(receipt,indent=2));print('receipt_sha256',sha(a.out/'receipt.json'))
assert result.returncode==0
