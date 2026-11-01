#!/usr/bin/env python3
"""Verify Linux paths inside the ZIP without merging case-distinct macOS files."""
from collections import defaultdict
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path, PurePosixPath
import shutil
import stat
import zipfile

ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
PACKET=ROOT/'decisions/evidence/release-qualification/build/linux-run-41069548581-review-20261030/recovery-20261101'
OUT=PACKET/'verified-archive'
OUT.mkdir()
shutil.copyfile(__file__,OUT/'executed-verifier.py')
archive=PACKET/'artifact.zip'
with archive.open('rb') as stream:digest=hashlib.file_digest(stream,'sha256').hexdigest()
run=json.loads((PACKET/'run.json').read_text())
artifact=json.loads((PACKET/'artifacts.json').read_text())['artifacts'][0]
assert run['id']==41069548581 and run['head_sha']=='a35104a611abb5c1d5c48dc4d1866f239ccd3ab9'
assert run['status']=='completed' and artifact['id']==12991483041
assert artifact['digest']=='sha256:'+digest and archive.stat().st_size==artifact['size_in_bytes']
with zipfile.ZipFile(archive) as bundle:
    members=[m for m in bundle.infolist() if not m.is_dir()]
    names=[m.filename for m in members]
    assert len(names)==len(set(names))
    manifest_bytes=bundle.read('manifest.json')
    files=json.loads(manifest_bytes)['files']
    assert set(names)==set(files)|{'manifest.json'}
    groups=defaultdict(list)
    for member in members:
        name=member.filename;path=PurePosixPath(name)
        assert not path.is_absolute() and '..' not in path.parts
        assert not stat.S_ISLNK(member.external_attr>>16)
        groups[name.casefold()].append(name)
        if name=='manifest.json':continue
        identity=files[name]
        assert member.file_size==identity['bytes'],name
        with bundle.open(member) as stream:
            assert hashlib.file_digest(stream,'sha256').hexdigest()==identity['sha256'],name
    collisions=[sorted(names) for names in groups.values() if len(names)>1]
    matrix=json.loads(bundle.read('linux/receipt.json'))
    assert matrix['candidate']==run['head_sha']
(OUT/'manifest.json').write_bytes(manifest_bytes)
(OUT/'case-distinct-paths.json').write_text(json.dumps(collisions,indent=2)+'\n')
(OUT/'matrix-receipt.json').write_text(json.dumps(matrix,indent=2)+'\n')
receipt={'scope':'All original ZIP entries verified by case-sensitive archive name; no scientific or milestone acceptance',
    'run':run['id'],'candidate':run['head_sha'],'conclusion':run['conclusion'],'artifact':artifact['id'],
    'archive':str(archive),'archive_sha256':digest,'archive_bytes':archive.stat().st_size,
    'manifest_sha256':hashlib.sha256(manifest_bytes).hexdigest(),'verified_files':len(files),
    'verified_bytes':sum(row['bytes'] for row in files.values()),'case_distinct_file_groups':len(collisions),
    'original_extraction_stop':'Exclusive creation stopped at linux/corpus/preprocessing/simul_prefix_PROCESS simul/dm.txt on case-insensitive macOS; partial extraction retained',
    'verifier_sha256':hashlib.sha256((OUT/'executed-verifier.py').read_bytes()).hexdigest(),
    'completed_utc':datetime.now(timezone.utc).isoformat()}
(OUT/'collection-receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(receipt,indent=2),flush=True)
print(json.dumps({'failed':matrix['failed'],'blocked':matrix['blocked'],'stages':len(matrix['stages'])}),flush=True)
