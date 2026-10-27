"""Serial library regressions, real-provider recovery, formatting and Clippy."""
import json, os, subprocess
from check import ROOT, HERE, sha

if __name__=='__main__':
    out=HERE/'build/final-checks';out.mkdir(exist_ok=False)
    env={**os.environ,'CALCEPH_DIR':str(HERE/'build/provider'),'TEMPO2_ACCELERATE':'1','AI_EPHEMERIS':str(HERE/'build/inpop17a_TDB_m100_p100_tt.dat'),'AI_POSITION_ONLY':str(ROOT/'validation/reference/build/source/T2runtime/ephemeris/DE405.1950.2050')}
    commands=[]
    commands.append(('format',['cargo','fmt','--manifest-path','rust/Cargo.toml','--','--check']))
    for name,features in [('disabled',[]),('combined',['--features','calceph,lapack'])]:
        commands.append(('clippy-'+name,['cargo','clippy','--offline','--manifest-path','rust/Cargo.toml',*features,'--all-targets','--','-D','warnings']))
    for label,extra in [('disabled-debug',[]),('disabled-release',['--release']),('combined-release',['--release','--features','calceph,lapack'])]:
        commands.append(('lib-'+label,['cargo','test','--offline','--manifest-path','rust/Cargo.toml',*extra,'--lib','--','--test-threads=1']))
    for profile in [[],['--release']]:
        label='release' if profile else 'debug'
        commands.append(('recovery-'+label,['cargo','test','--offline','--manifest-path','rust/Cargo.toml','--features','calceph,lapack',*profile,'--test','native_backends','--','--include-ignored','--test-threads=1']))
    commands.append(('qr-default-only',['cargo','test','--offline','--manifest-path','rust/Cargo.toml','--features','qr-default','--test','native_backends','--','--test-threads=1']))
    records=[]
    for label,cmd in commands:
        log=out/(label+'.log')
        with log.open('wb') as f:r=subprocess.run(cmd,cwd=ROOT,env=env,stdout=f,stderr=subprocess.STDOUT)
        records.append({'label':label,'argv':cmd,'exit':r.returncode,'log_sha256':sha(log)})
        print(label,r.returncode,flush=True)
        assert r.returncode==0,label
    (out/'receipt.json').write_text(json.dumps({'records':records,'script':sha(__file__)},indent=2)+'\n')
