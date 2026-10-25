#!/usr/bin/env python3
"""Final component executions with the actual test executable and source identities."""
import json
import os
from pathlib import Path
import re
import subprocess
import native_timing as n

B = n.HERE/'build'
commands = []
sources = {str(p.relative_to(n.ROOT)): n.sha(p) for p in sorted((n.ROOT/'rust/src').rglob('*.rs'))}
env = {**os.environ, 'CARGO_HOME':str(n.ROOT/'rust/.cargo-home'),
       'GMP_MPFR_SYS_CACHE':str(n.ROOT/'rust/target/gmp-cache')}
env.pop('AD_CORRECTED_CORPUS', None)
for profile in ['debug', 'release']:
    for name, arguments, corpus in [
        ('standard', [], None),
        ('g-original', ['binary::tests::g_components','--','--ignored','--nocapture'], None),
        ('g-corrected', ['binary::tests::g_components','--','--ignored','--nocapture'], str(B/'corrected-binary/corpus')),
        ('lifecycle-tests', ['timing::tests','--','--include-ignored'], None)]:
        cmd = ['cargo','test','--manifest-path',str(n.ROOT/'rust/Cargo.toml'),'--locked','--offline',*(['--release'] if profile == 'release' else []),*arguments]
        runenv = {**env, **({'AD_CORRECTED_CORPUS':corpus} if corpus else {})}
        r = subprocess.run(cmd, cwd=n.ROOT, env=runenv, capture_output=True)
        log = B/f'{name}-{profile}-final.log'
        log.write_bytes(r.stdout+r.stderr)
        executables = [n.ROOT/Path(p) for p in re.findall(r'Running [^\n]+ \(([^)]+)\)', log.read_text())]
        commands.append(dict(name=name, profile=profile, command=cmd, returncode=r.returncode,
                             corpus=corpus, log_sha256=n.sha(log),
                             executable_sha256={str(p):n.sha(p) for p in executables}))
        (B/'components.json').write_text(json.dumps(dict(commands=commands, source_sha256=sources), indent=2)+'\n')
        print(name, profile, r.returncode, flush=True)
        assert r.returncode == 0, log
assert sources == {str(p.relative_to(n.ROOT)):n.sha(p) for p in sorted((n.ROOT/'rust/src').rglob('*.rs'))}, 'production source changed during execution'
