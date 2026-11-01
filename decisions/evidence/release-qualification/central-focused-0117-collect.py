#!/usr/bin/env python3
"""Collect the frozen focused 0117 Linux run and verify its case-sensitive ZIP evidence."""
from collections import Counter, defaultdict
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path, PurePosixPath
import shutil
import stat
import subprocess
from urllib.parse import urlsplit
import zipfile

ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
OUT = ROOT / 'decisions/evidence/release-qualification/build/linux-run-41312212554-review-20261102'
RUN = 41312212554
JOB = 124864766765
CANDIDATE = '0d968bf48f4f5b48cd12e3d2256d2fb60f95e967'
API = 'repos/wdong-eric/tempo2rust'


def sha(path):
    with path.open('rb') as source:
        return hashlib.file_digest(source, 'sha256').hexdigest()


def main():
    # A live run is a wait, never a reason to dispatch again. Avoid creating an
    # incomplete collection directory when this recipe is invoked too early.
    response = subprocess.run(['gh', 'api', f'{API}/actions/runs/{RUN}'],
                              capture_output=True, check=True, timeout=45)
    run = json.loads(response.stdout)
    assert run['id'] == RUN and run['head_sha'] == CANDIDATE
    assert run['run_attempt'] == 1
    if run['status'] != 'completed':
        print(json.dumps({'run': RUN, 'status': run['status'], 'collection': 'waiting'}))
        return
    OUT.mkdir(parents=True, exist_ok=False)
    shutil.copyfile(__file__, OUT / 'executed-collector.py')
    (OUT / 'run.json').write_bytes(response.stdout)
    commands = []

    def record(label, argv, code):
        commands.append({'label': label, 'argv': argv, 'exit': code,
                         'recorded_utc': datetime.now(timezone.utc).isoformat()})
        (OUT / 'collection-commands.json').write_text(json.dumps(commands, indent=2) + '\n')

    def command(label, argv, timeout=60, stdin=None):
        with (OUT / (label + '.stdout')).open('xb') as stdout, (OUT / (label + '.stderr')).open('xb') as stderr:
            try:
                code = subprocess.run(argv, cwd=ROOT, input=stdin, stdout=stdout,
                                      stderr=stderr, timeout=timeout).returncode
            except subprocess.TimeoutExpired:
                code = 124
        record(label, argv, code)
        return code

    record('run', ['gh', 'api', f'{API}/actions/runs/{RUN}'], 0)
    assert command('job', ['gh', 'api', f'{API}/actions/jobs/{JOB}']) == 0
    job = json.loads((OUT / 'job.stdout').read_bytes())
    assert job['id'] == JOB and job['run_id'] == RUN and job['status'] == 'completed'
    shutil.copyfile(OUT / 'job.stdout', OUT / 'job.json')
    assert command('artifacts', ['gh', 'api', f'{API}/actions/runs/{RUN}/artifacts']) == 0
    shutil.copyfile(OUT / 'artifacts.stdout', OUT / 'artifacts.json')
    inventory = json.loads((OUT / 'artifacts.json').read_bytes())
    assert inventory['total_count'] == len(inventory['artifacts']) == 1
    artifact = inventory['artifacts'][0]
    assert artifact['name'] == f'tempo2-linux-{RUN}-1'
    assert artifact['workflow_run']['id'] == RUN
    assert artifact['workflow_run']['head_sha'] == CANDIDATE and not artifact['expired']
    log_exit = command('job-log', ['gh', 'api', '--allow-escape-sequences',
                                  f'{API}/actions/jobs/{JOB}/logs'], timeout=120)
    if log_exit == 0:
        shutil.copyfile(OUT / 'job-log.stdout', OUT / 'job.complete.raw.log')
    print(json.dumps({'run': RUN, 'conclusion': run['conclusion'],
                      'artifact': artifact['id'], 'expected_bytes': artifact['size_in_bytes']}), flush=True)

    # Reuse the third-run recovery's Range download. Authentication and signed
    # redirect URLs stay in memory/stdin and never enter the retained command log.
    archive = OUT / 'artifact.zip'
    for attempt in range(3):
        token = subprocess.run(['gh', 'auth', 'token', '--hostname', 'github.com'],
                               capture_output=True, text=True, check=True, timeout=45).stdout.strip()
        config = 'url = ' + json.dumps('https://api.github.com/' + API + f"/actions/artifacts/{artifact['id']}/zip") + '\n'
        config += 'header = ' + json.dumps('Authorization: Bearer ' + token) + '\n'
        argv = ['curl', '--config', '-', '--silent', '--show-error', '--fail',
                '--dump-header', '-', '--output', '/dev/null', '--max-time', '60']
        redirect = subprocess.run(argv, input=config.encode(), capture_output=True, timeout=65)
        record(f'artifact-redirect-{attempt}', argv, redirect.returncode)
        assert redirect.returncode == 0, 'Artifact redirect request failed'
        headers = redirect.stdout.decode('latin1')
        location = next(line.split(':', 1)[1].strip() for line in headers.splitlines()
                        if line.lower().startswith('location:'))
        assert urlsplit(location).scheme == 'https'
        del token, config, headers, redirect
        offset = archive.stat().st_size if archive.exists() else 0
        print(json.dumps({'download_attempt': attempt, 'resume_offset': offset}), flush=True)
        argv = ['curl', '--config', '-', '--fail', '--location', '--continue-at', '-',
                '--output', str(archive), '--max-time', '1200']
        code = command(f'artifact-download-{attempt}', argv, timeout=1210,
                       stdin=('url = ' + json.dumps(location) + '\n').encode())
        del location
        if code == 0:
            break
        assert archive.stat().st_size < artifact['size_in_bytes'], 'Retain download and inspect before retrying'
    else:
        raise RuntimeError('Download attempts exhausted; retain the partial archive and resume this artifact, never redispatch')
    digest = sha(archive)
    assert archive.stat().st_size == artifact['size_in_bytes']
    assert artifact['digest'] == 'sha256:' + digest

    # Reuse the third-run ZIP verifier: direct member reads preserve Linux names
    # such as DM.txt/dm.txt on a case-insensitive macOS filesystem. Do not extract.
    with zipfile.ZipFile(archive) as bundle:
        members = bundle.infolist()
        names = [member.filename for member in members]
        assert len(names) == len(set(names))
        for member in members:
            path = PurePosixPath(member.filename)
            assert not path.is_absolute() and '..' not in path.parts
            assert not stat.S_ISLNK(member.external_attr >> 16)
        manifest_bytes = bundle.read('manifest.json')
        files = json.loads(manifest_bytes)['files']
        assert {m.filename for m in members if not m.is_dir()} == set(files) | {'manifest.json'}
        groups = defaultdict(list)
        for member in members:
            if member.is_dir():
                continue
            name = member.filename
            groups[name.casefold()].append(name)
            if name == 'manifest.json':
                continue
            identity = files[name]
            assert member.file_size == identity['bytes'], name
            with bundle.open(member) as source:
                assert hashlib.file_digest(source, 'sha256').hexdigest() == identity['sha256'], name
        matrix_bytes = bundle.read('linux/receipt.json')
        matrix = json.loads(matrix_bytes)
        assert matrix['candidate'] == CANDIDATE
    collisions = [sorted(group) for group in groups.values() if len(group) > 1]
    stages = matrix['stages']
    assert len(stages) == 26 and matrix['scope'].startswith('Partial 0117')
    failed = [name for name, row in stages.items() if row['exit'] not in (0, None)]
    blocked = [name for name, row in stages.items() if row['exit'] is None]
    assert failed == matrix['failed'] and blocked == matrix['blocked']
    counts = Counter('blocked' if r['exit'] is None else 'passed' if r['exit'] == 0 else 'failed'
                     for r in stages.values())
    (OUT / 'manifest.json').write_bytes(manifest_bytes)
    (OUT / 'matrix-receipt.json').write_bytes(matrix_bytes)
    (OUT / 'case-distinct-paths.json').write_text(json.dumps(collisions, indent=2) + '\n')
    receipt = {
        'scope': 'Collection and exact ZIP membership/hash verification; scientific and milestone acceptance remain separate',
        'run': RUN, 'job': JOB, 'candidate': CANDIDATE, 'conclusion': run['conclusion'], 'artifact': artifact['id'],
        'archive_sha256': digest, 'archive_bytes': archive.stat().st_size,
        'manifest_sha256': hashlib.sha256(manifest_bytes).hexdigest(),
        'matrix_receipt_sha256': hashlib.sha256(matrix_bytes).hexdigest(),
        'verified_files': len(files), 'verified_bytes': sum(row['bytes'] for row in files.values()),
        'case_distinct_file_groups': len(collisions), 'stage_counts': dict(counts),
        'stage_count': len(stages), 'expected_stage_count': 26, 'failed': failed, 'blocked': blocked,
        'job_log_exit': log_exit, 'collector_sha256': sha(OUT / 'executed-collector.py'),
        'completed_utc': datetime.now(timezone.utc).isoformat(),
    }
    (OUT / 'collection-receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
    print(json.dumps(receipt, indent=2), flush=True)


if __name__ == '__main__':
    main()
