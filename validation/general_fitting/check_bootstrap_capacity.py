"""Natural IBOOT edge cases, guarding the source before result-array overflow."""
from pathlib import Path
import argparse, json, os, subprocess
from decimal import Decimal as D
from check_bootstrap import ROOT, HERE, build, sha

def run(profile):
    native,commands,identity,src=build()
    rust=ROOT/'rust/target'/profile/'examples/fitting_bootstrap'
    base=HERE/'build/bootstrap-capacity'/profile;base.mkdir(parents=True,exist_ok=True)
    runtime=ROOT/'validation/reference/build/source/T2runtime';tim=ROOT/'validation/fitting/inputs/bary.tim'
    original=HERE/'inputs/bootstrap.par';records=[]
    for exponent in [-1,0,12,13]:
        out=base/str(exponent);out.mkdir(exist_ok=True)
        par=out/'input.par';par.write_text(original.read_text().replace('IBOOT 2',f'IBOOT {exponent}'))
        dirs={e:out/e for e in ['native','rust']}
        for d in dirs.values():d.mkdir(exist_ok=True)
        trace=dirs['native']/'trace.tsv'
        cmds=[[str(native),str(trace),str(par),str(tim)],[str(rust),str(runtime),str(par),str(tim),str(dirs['rust'])]]
        exits={}
        for e,cmd in zip(dirs,cmds):
            proc=subprocess.run(cmd,cwd=dirs[e],env={**os.environ,'TEMPO2':str(runtime)},capture_output=True,timeout=300)
            (dirs[e]/'stdout').write_bytes(proc.stdout);(dirs[e]/'stderr').write_bytes(proc.stderr);exits[e]=proc.returncode
        failures=[]
        if exponent==13:
            assert exits['native']==86 and exits['rust']!=0
            assert 'BOUNDARY MAX_ITER 4096' in trace.read_text()
            assert not (dirs['rust']/'stdout').read_bytes() and not (dirs['rust']/'testout.tim').exists()
        else:
            assert exits=={'native':0,'rust':0},exits
            got=json.loads((dirs['rust']/'stdout').read_text());n=0 if exponent<0 else 2**exponent
            assert len(got['iterations'])==n
            lines=[r.split()for r in trace.read_text().splitlines()]
            samples=[tuple(map(int,r[1:]))for r in lines if r[0]=='S']
            assert samples==[(it,i,k)for it,row in enumerate(got['iterations'])for i,k in enumerate(row['sampled_indices'])]
            scale=D(160)*86400/D('10.12345678902')
            for r in lines:
                if r[0] not in ['P','F']:continue
                values=got['values']if r[0]=='F'else got['iterations'][int(r[1])]['fitted_values']
                delta=(D(values[0])-D(r[4]))*scale
                if abs(delta)>D('1e-11'):failures.append({'quantity':'value','iteration':int(r[1]),'difference_s':str(delta)})
                if r[0]=='F':
                    delta=(D(got['errors'][0])-D(r[5]))*scale
                    if abs(delta)>D('1e-11')+D('1e-6')*abs(D(r[5])*scale):failures.append({'quantity':'error','difference_s':str(delta)})
            assert (dirs['native']/'testout.tim').exists()==(n>0)
            assert (dirs['rust']/'testout.tim').exists()==(n>0)
            if n and (dirs['native']/'testout.tim').read_bytes()!=(dirs['rust']/'testout.tim').read_bytes():failures.append({'quantity':'writeTim_bytes'})
        records.append({'exponent':exponent,'commands':cmds,'input_sha256':sha(par),'exits':exits,'failures':failures})
        print('IBOOT',exponent,'failures',len(failures),flush=True)
    (base/'receipt.json').write_text(json.dumps({'profile':profile,'commands':commands,'source_sha256':sha(src),'builder_sha256':sha(__file__),'native_sha256':sha(native),'rust_sha256':sha(rust),'records':records},indent=2)+'\n')

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--profile',choices=['debug','release'],default='debug');run(p.parse_args().profile)
