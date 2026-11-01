#!/usr/bin/env python3
"""Resume collection of the existing Linux artifact with curl; never dispatch."""
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import shutil
import stat
import subprocess
from urllib.parse import urlsplit
import zipfile

ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
PRIOR=ROOT/'decisions/evidence/release-qualification/build/linux-run-41069548581-review-20261030'
OUT=PRIOR/'recovery-20261101'
RUN=41069548581
JOB=124109733733
ARTIFACT=12991483041
CANDIDATE='a35104a611abb5c1d5c48dc4d1866f239ccd3ab9'
API='repos/wdong-eric/tempo2rust'
OUT.mkdir()
shutil.copyfile(__file__,OUT/'executed-collector.py')
commands=[]


def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream,'sha256').hexdigest()


def record(label,argv,code):
    commands.append({'label':label,'argv':argv,'exit':code,'recorded_utc':datetime.now(timezone.utc).isoformat()})
    (OUT/'collection-commands.json').write_text(json.dumps(commands,indent=2)+'\n')


def command(label,argv,timeout=60,stdin=None):
    with (OUT/(label+'.stdout')).open('xb') as stdout,(OUT/(label+'.stderr')).open('xb') as stderr:
        try:
            code=subprocess.run(argv,cwd=ROOT,input=stdin,stdout=stdout,stderr=stderr,timeout=timeout).returncode
        except subprocess.TimeoutExpired:
            code=124
    record(label,argv,code)
    assert code==0,(label,code)
    return OUT/(label+'.stdout')


run_path=command('run',['gh','api',f'{API}/actions/runs/{RUN}'])
run=json.loads(run_path.read_text())
assert run['id']==RUN and run['head_sha']==CANDIDATE and run['status']=='completed'
assert run['run_attempt']==1
shutil.copyfile(run_path,OUT/'run.json')
artifact_path=command('artifacts',['gh','api',f'{API}/actions/runs/{RUN}/artifacts'])
shutil.copyfile(artifact_path,OUT/'artifacts.json')
artifacts=json.loads(artifact_path.read_text())['artifacts']
assert len(artifacts)==1 and artifacts[0]['id']==ARTIFACT
artifact=artifacts[0]
assert artifact['workflow_run']['head_sha']==CANDIDATE and not artifact['expired']
old=json.loads((PRIOR/'artifacts.json').read_text())['artifacts'][0]
assert all(artifact[k]==old[k] for k in ['id','size_in_bytes','digest'])
job_path=command('job',['gh','api',f'{API}/actions/jobs/{JOB}'])
shutil.copyfile(job_path,OUT/'job.json')
assert json.loads(job_path.read_text())['run_id']==RUN
log_path=command('job-log',['gh','api','--allow-escape-sequences',f'{API}/actions/jobs/{JOB}/logs'],120)
shutil.copyfile(log_path,OUT/'job.complete.raw.log')
print(json.dumps({'run':RUN,'conclusion':run['conclusion'],'artifact':ARTIFACT,'job_log_ready':True}),flush=True)

# Use curl's native Range handling. Keep authentication and the short-lived
# redirect URL in memory/stdin; neither belongs in command logs or Git.
token=subprocess.run(['gh','auth','token','--hostname','github.com'],capture_output=True,text=True,check=True).stdout.strip()
config='url = '+json.dumps('https://api.github.com/'+API+f'/actions/artifacts/{ARTIFACT}/zip')+'\nheader = '+json.dumps('Authorization: Bearer '+token)+'\n'
argv=['curl','--config','-','--silent','--show-error','--fail','--dump-header','-','--output','/dev/null','--max-time','60']
redirect=subprocess.run(argv,input=config.encode(),capture_output=True,timeout=65)
record('artifact-redirect',argv,redirect.returncode)
assert redirect.returncode==0,'Artifact redirect request failed'
headers=redirect.stdout.decode('latin1')
location=next(line.split(':',1)[1].strip() for line in headers.splitlines() if line.lower().startswith('location:'))
assert urlsplit(location).scheme=='https'
del token,config,headers,redirect
archive=OUT/'artifact.zip'
partial=PRIOR/'artifact-download.stdout'
offset=partial.stat().st_size
assert 0<offset<artifact['size_in_bytes']
shutil.copyfile(partial,archive)
assert sha(archive)==sha(partial)
print(json.dumps({'resume_offset':offset,'expected_bytes':artifact['size_in_bytes']}),flush=True)
argv=['curl','--config','-','--fail','--location','--continue-at','-','--output',str(archive),'--max-time','1800']
command('artifact-resume',argv,1850,('url = '+json.dumps(location)+'\n').encode())
del location
assert archive.stat().st_size==artifact['size_in_bytes']
assert 'sha256:'+sha(archive)==artifact['digest']
print(json.dumps({'archive_verified':True,'bytes':archive.stat().st_size}),flush=True)

# Preserve the original collector's path, symlink and exact-membership checks.
selected=OUT/'artifact';selected.mkdir()
seen=set()
with zipfile.ZipFile(archive) as bundle:
    total=sum(member.file_size for member in bundle.infolist())
    assert shutil.disk_usage(OUT).free>total
    for member in bundle.infolist():
        name=PurePosixPath(member.filename)
        assert not name.is_absolute() and '..' not in name.parts and member.filename not in seen
        assert not stat.S_ISLNK(member.external_attr>>16)
        seen.add(member.filename)
        dest=selected.joinpath(*name.parts)
        assert dest.resolve().is_relative_to(selected.resolve())
        if member.is_dir():dest.mkdir(parents=True,exist_ok=True)
        else:
            dest.parent.mkdir(parents=True,exist_ok=True)
            with bundle.open(member) as source,dest.open('xb') as target:
                shutil.copyfileobj(source,target,1024*1024)
files=json.loads((selected/'manifest.json').read_text())['files']
for name,identity in files.items():
    path=selected/name
    assert path.resolve().is_relative_to(selected.resolve()) and path.is_file() and not path.is_symlink()
    assert path.stat().st_size==identity['bytes'] and sha(path)==identity['sha256'],name
assert {str(p.relative_to(selected)) for p in selected.rglob('*') if p.is_file()}==set(files)|{'manifest.json'}
receipt={'scope':'Same-run collection recovery and archive/file verification; scientific review remains separate',
    'run':RUN,'candidate':CANDIDATE,'conclusion':run['conclusion'],'artifact':ARTIFACT,
    'resumed_from_bytes':offset,'archive_sha256':sha(archive),'archive_bytes':archive.stat().st_size,
    'manifest_sha256':sha(selected/'manifest.json'),'verified_files':len(files),
    'verified_bytes':sum(row['bytes'] for row in files.values()),'job_log_sha256':sha(OUT/'job.complete.raw.log'),
    'collector_sha256':sha(OUT/'executed-collector.py'),'completed_utc':datetime.now(timezone.utc).isoformat()}
(OUT/'collection-receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(receipt),flush=True)
