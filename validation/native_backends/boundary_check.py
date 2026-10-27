"""Public disabled/missing-resource errors followed by a valid native-style run."""
import json, os, subprocess
from pathlib import Path
from check import HERE, RUNTIME, sha

if __name__=='__main__':
    base=HERE/'build/public-boundaries-final-v3';base.mkdir(exist_ok=False)
    profiles=HERE/'build/profiles-final-v3'
    valid=profiles/'disabled-debug/paired/inputs';calceph=HERE/'build/paired-combined-debug/inputs'
    missing=base/'missing.par';missing.write_text((calceph/'calceph.par').read_text().split('EPH_FILE ')[0]+'EPH_FILE /missing-ai-ephemeris\n')
    records=[];env={**os.environ,'TEMPO2':str(RUNTIME)}
    for profile,folder,bad in [('disabled',profiles/'disabled-debug',calceph/'calceph.par'),('enabled',profiles/'combined-debug',missing)]:
        for engine in ['native','rust']:
            exe=folder/('native/tempo2' if engine=='native' else 'rust-cli')
            for label,par,expected in [('failure',bad,1),('recovery',valid/'calceph.par',0)]:
                out=base/f'{profile}-{engine}-{label}';out.mkdir()
                cmd=[str(exe),'-npsr','1','-nobs','32','-f',str(par),str(valid/'pks.tim')]
                r=subprocess.run(cmd,cwd=out,env=env,capture_output=True,timeout=90)
                (out/'stdout').write_bytes(r.stdout);(out/'stderr').write_bytes(r.stderr)
                records.append({'argv':cmd,'exe_sha256':sha(exe),'exit':r.returncode,'expected':expected})
                assert r.returncode==expected,records[-1]
    files={str(p):sha(p) for p in base.rglob('*') if p.is_file()}
    (base/'receipt.json').write_text(json.dumps({'records':records,'files':files,'script':sha(__file__)},indent=2)+'\n')
    print(len(records),'passed')
