#!/usr/bin/env python3
"""Public CLI error publication and following valid processing in owned directories."""
import json
import subprocess
import native_timing as n

f = n.HERE/'fixtures'
runtime = n.ROOT/'validation/reference/build/source/T2runtime'
for profile in ['debug', 'release']:
    exe = n.ROOT/f'rust/target/{profile}/tempo2-rust'
    work = n.HERE/f'build/cli-recovery/{profile}'
    work.mkdir(parents=True, exist_ok=True)
    base = (f/'high-spin.par').read_text()
    cases = [('unknown-binary', base+'BINARY UNKNOWN\nPB 1\n', (f/'high-spin.tim').read_text(), b'Unknown binary model'),
             ('missing-noise', base+'TNsubtractRed 1\n', (f/'high-spin.tim').read_text(), b'TNRedSignal'),
             ('missing-average-flag', (f/'averaging.par').read_text(), (f/'averaging.tim').read_text().replace(' -f A','').replace(' -f B',''), b'averagebat')]
    records = []
    def run(par, tim):
        cmd = [str(exe),'timing','--par',str(par),'--tim',str(tim),'--runtime',str(runtime)]
        r = subprocess.run(cmd, cwd=work, capture_output=True, timeout=60)
        return cmd,r
    for name, par, tim, diagnostic in cases:
        p = work/(name+'.par');t = work/(name+'.tim')
        p.write_text(par);t.write_text(tim)
        cmd,r = run(p,t)
        assert r.returncode != 0 and not r.stdout and diagnostic in r.stderr, (name,r.returncode,r.stderr)
        records.append(dict(name=name,command=cmd,returncode=r.returncode,stdout_empty=True,stderr=r.stderr.decode()))
        _,good = run(f/'high-spin.par',f/'high-spin.tim')
        assert good.returncode == 0 and json.loads(good.stdout)['status'] == 'complete'
    avg = work/'avg.dat'
    avg.unlink(missing_ok=True)
    avg.mkdir()
    cmd,r = run(f/'averaging.par',f/'averaging.tim')
    assert r.returncode != 0 and not r.stdout and b'avg.dat' in r.stderr
    records.append(dict(name='unwritable-average',command=cmd,returncode=r.returncode,stdout_empty=True,stderr=r.stderr.decode()))
    avg.rmdir()
    _,good = run(f/'averaging.par',f/'averaging.tim')
    assert good.returncode == 0 and json.loads(good.stdout)['status'] == 'complete' and avg.is_file()
    (work/'receipt.json').write_text(json.dumps(dict(executable_sha256=n.sha(exe),checks=records),indent=2)+'\n')
    print(profile, 'four failure/recovery pairs passed; failures publish no JSON')
