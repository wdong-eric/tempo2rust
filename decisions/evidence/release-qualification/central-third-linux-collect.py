#!/usr/bin/env python3
"""Wait for the existing third Linux run, preserve its artifact, and verify membership."""
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path, PurePosixPath
import shutil
import stat
import subprocess
import time
import zipfile

ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
OUT=ROOT/'decisions/evidence/release-qualification/build/linux-run-41069548581-review-20261030'
RUN=41069548581
JOB=124109733733
CANDIDATE='a35104a611abb5c1d5c48dc4d1866f239ccd3ab9'
API='repos/wdong-eric/tempo2rust'
OUT.mkdir(parents=True,exist_ok=False)
shutil.copyfile(__file__,OUT/'executed-collector.py')
commands=[]

def sha(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream,'sha256').hexdigest()

def command(label,argv,timeout=60):
    start=datetime.now(timezone.utc).isoformat()
    with (OUT/(label+'.stdout')).open('xb') as stdout, (OUT/(label+'.stderr')).open('xb') as stderr:
        try:
            code=subprocess.run(argv,cwd=ROOT,stdout=stdout,stderr=stderr,timeout=timeout).returncode
        except subprocess.TimeoutExpired:
            code=124
    commands.append({'label':label,'argv':argv,'started_utc':start,'exit':code})
    (OUT/'collection-commands.json').write_text(json.dumps(commands,indent=2)+'\n')
    return code

last=None
for attempt in range(240):
    label=f'run-poll-{attempt:03d}'
    code=command(label,['gh','api',f'{API}/actions/runs/{RUN}'],timeout=45)
    if code:
        print(json.dumps({'observation_error':code,'label':label,'run':RUN}),flush=True)
        time.sleep(45)
        continue
    run=json.loads((OUT/(label+'.stdout')).read_text())
    assert run['id']==RUN and run['head_sha']==CANDIDATE
    state=(run['status'],run['conclusion'])
    if state!=last:
        print(json.dumps({'run':RUN,'state':state,'head':CANDIDATE}),flush=True)
        last=state
    if run['status']=='completed':
        shutil.copyfile(OUT/(label+'.stdout'),OUT/'run.json')
        break
    time.sleep(45)
else:
    raise RuntimeError('Observation window ended; preserve the same run and inspect it again, never redispatch')

assert command('artifacts',['gh','api',f'{API}/actions/runs/{RUN}/artifacts'])==0
shutil.copyfile(OUT/'artifacts.stdout',OUT/'artifacts.json')
artifacts=json.loads((OUT/'artifacts.json').read_text())['artifacts']
assert len(artifacts)==1
artifact=artifacts[0]
assert artifact['workflow_run']['head_sha']==CANDIDATE and not artifact['expired']
assert command('artifact-download',['gh','api',f"{API}/actions/artifacts/{artifact['id']}/zip"],timeout=600)==0
archive=OUT/'artifact.zip'
(OUT/'artifact-download.stdout').rename(archive)
assert archive.stat().st_size==artifact['size_in_bytes']
assert 'sha256:'+sha(archive)==artifact['digest']
print(json.dumps({'artifact':artifact['id'],'archive_bytes':archive.stat().st_size,'digest_verified':True}),flush=True)

selected=OUT/'artifact';selected.mkdir()
seen=set()
with zipfile.ZipFile(archive) as z:
    for member in z.infolist():
        name=PurePosixPath(member.filename)
        assert not name.is_absolute() and '..' not in name.parts and member.filename not in seen
        assert not stat.S_ISLNK(member.external_attr>>16)
        seen.add(member.filename)
        dest=selected.joinpath(*name.parts)
        assert dest.resolve().is_relative_to(selected.resolve())
        if member.is_dir():
            dest.mkdir(parents=True,exist_ok=True)
        else:
            dest.parent.mkdir(parents=True,exist_ok=True)
            with z.open(member) as src,dest.open('xb') as dst:
                shutil.copyfileobj(src,dst,1024*1024)
files=json.loads((selected/'manifest.json').read_text())['files']
for name,identity in files.items():
    path=selected/name
    assert path.resolve().is_relative_to(selected.resolve()) and path.is_file() and not path.is_symlink()
    assert path.stat().st_size==identity['bytes'] and sha(path)==identity['sha256'],name
assert {str(p.relative_to(selected)) for p in selected.rglob('*') if p.is_file()}==set(files)|{'manifest.json'}
assert command('job',['gh','api',f'{API}/actions/jobs/{JOB}'])==0
shutil.copyfile(OUT/'job.stdout',OUT/'job.json')
log_exit=command('job-log',['gh','api','--allow-escape-sequences',f'{API}/actions/jobs/{JOB}/logs'],timeout=120)
if log_exit==0:shutil.copyfile(OUT/'job-log.stdout',OUT/'job.complete.raw.log')
receipt={'scope':'Collection and archive/file identity verification only; scientific review and acceptance remain separate',
    'run':RUN,'candidate':CANDIDATE,'conclusion':run['conclusion'],'artifact':artifact['id'],
    'archive_sha256':sha(archive),'archive_bytes':archive.stat().st_size,
    'manifest_sha256':sha(selected/'manifest.json'),'verified_files':len(files),
    'verified_bytes':sum(row['bytes'] for row in files.values()),'job_log_exit':log_exit,
    'collector_sha256':sha(OUT/'executed-collector.py'),'completed_utc':datetime.now(timezone.utc).isoformat()}
(OUT/'collection-receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(receipt),flush=True)
