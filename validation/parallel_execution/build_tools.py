"""Build pinned GNU autotools into an ignored, task-local prefix; no host install."""
import hashlib, json, os, subprocess, tarfile
from pathlib import Path
HERE = Path(__file__).resolve().parent
BUILD = HERE / 'build' / 'tools'
PREFIX = BUILD / 'prefix'
commands = []
for package in ['m4-1.4.20', 'autoconf-2.72', 'automake-1.17', 'libtool-2.5.4']:
    archive = BUILD / (package + '.tar.xz')
    subprocess.run(["tar", "-xf", str(archive), "-C", str(BUILD)], check=True)
    source = BUILD / package
    env = dict(os.environ, PATH=str(PREFIX / 'bin') + ':' + os.environ['PATH'])
    for stage, args in [('configure', ['./configure', '--prefix=' + str(PREFIX)]),
                        ('build', ['make', '-j4']), ('install', ['make', 'install'])]:
        log = BUILD / f'{package}-{stage}.log'
        with log.open('wb') as output:
            r = subprocess.run(args, cwd=source, env=env, stdout=output, stderr=subprocess.STDOUT, timeout=300)
        commands.append(dict(package=package, archive_sha256=hashlib.sha256(archive.read_bytes()).hexdigest(),
                             command=args, cwd=str(source), log=str(log), exit=r.returncode))
        (BUILD/'receipt.json').write_text(json.dumps(commands, indent=2)+'\n')
        assert r.returncode == 0, log
    print(package, 'installed in', PREFIX, flush=True)
