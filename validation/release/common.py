"""Release evidence identity and command recording; never overwrite an old run."""
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess

ROOT = Path(__file__).resolve().parents[2]
PIN = 'fda6ad4a9753ad7dc5d291406f2f310a9161423f'


def sha(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()


class Run:
    def __init__(self, out):
        self.out = Path(out).resolve()
        self.out.mkdir(parents=True, exist_ok=False)
        self.commands = []

    def run(self, args, label, *, cwd=None, env=None, expected=0, stdin=None):
        args = list(map(str, args))
        result = subprocess.run(args, cwd=cwd, env=env, input=stdin, capture_output=True)
        (self.out / (label + '.stdout')).write_bytes(result.stdout)
        (self.out / (label + '.stderr')).write_bytes(result.stderr)
        self.commands.append({'argv': args, 'cwd': str(cwd), 'status': result.returncode,
                              'expected': expected, 'label': label})
        self.save({'status': 'running'})
        if expected is not None and result.returncode != expected:
            self.save({'status': 'failure', 'failed_command': label})
            raise RuntimeError(f'{label}: exit {result.returncode}; see {self.out}')
        return result.stdout

    def save(self, fields):
        record = {'host': platform.platform(), 'machine': platform.machine(),
                  'commands': self.commands,
                  'ci': {k: os.environ[k] for k in ('GITHUB_SHA', 'GITHUB_RUN_ID',
                         'GITHUB_RUN_ATTEMPT', 'ImageOS', 'ImageVersion', 'RUNNER_ARCH')
                         if k in os.environ}, **fields}
        (self.out / 'receipt.json').write_text(json.dumps(record, indent=2) + '\n')


def identities(directory):
    return {str(p.relative_to(directory)): sha(p) for p in sorted(Path(directory).rglob('*'))
            if p.is_file() and '.git' not in p.parts}
