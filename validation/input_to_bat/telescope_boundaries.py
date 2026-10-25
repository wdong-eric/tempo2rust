#!/usr/bin/env python3
"""Decision 0072: bounded Q probe reuse, with no native out-of-bounds execution."""
import gzip, hashlib, json, os, subprocess, sys, uuid
from pathlib import Path
HERE=Path(__file__).resolve().parent; ROOT=HERE.parent.parent; Q=ROOT/'validation/par_specialized'; BUILD=HERE/'build/telescope_boundaries'
FAMILIES=['TEL_CLK_OFFS','TEL_DX','TEL_DY','TEL_DZ','TEL_VX','TEL_VY','TEL_VZ']
def sha(b):return hashlib.sha256(b).hexdigest()
def main():
    sys.path.insert(0,str(Q));import run as q
    profile=json.loads((Q/'profile.json').read_text());exe=Path(profile['executable_path'])
    identity=q.execution_identity(exe,profile,True)
    original=json.loads(gzip.decompress((Q/'observations.json.gz').read_bytes()))
    initial=json.loads(gzip.decompress((Q/'initial_state.json.gz').read_bytes()))
    BUILD.mkdir(exist_ok=True,parents=True);(BUILD/'receipt.json').unlink(missing_ok=True)
    cases=[];receipts=[]
    for family in FAMILIES:
        old=next(c for c in original if c['id']=='samples_'+family);lines=old['actions'][0][2].splitlines(keepends=True)
        assert family+'1000' in lines[3]
        actions=[['dispatch','0',''.join(lines[:3])],['dispatch','0',lines[3].replace(family+'1000',family+'500')+''.join(lines[4:])]]
        work=BUILD/family;work.mkdir(exist_ok=True);ap=work/'actions.tsv';ep=work/'events.jsonl';ep.unlink(missing_ok=True)
        ap.write_text('\n'.join('\t'.join('x'+s.encode().hex() for s in a) for a in actions)+'\n');ah=sha(ap.read_bytes());nonce=uuid.uuid4().hex
        cmd=[str(exe),str(ap),str(ep),nonce];run=subprocess.run(cmd,capture_output=True,env={**os.environ,'LC_ALL':'C','LANG':'C','TEMPO2':str(Q/'build/runtime')},timeout=90)
        assert run.returncode==0,(family,run.stdout,run.stderr)
        q.execution_identity(exe,profile,True);assert sha(ap.read_bytes())==ah
        events=[json.loads(s) for s in ep.read_text().splitlines()];assert events.pop(0)=={'phase':'launch','nonce':nonce};assert events[0].pop('state')==initial
        (work/'stdout.bin').write_bytes(run.stdout);(work/'stderr.bin').write_bytes(run.stderr)
        cases.append(dict(id='bounded_'+family,actions=actions,files={},returncode=0,events=events))
        prefix=[e for e in events if e.get('action')==0 and e['phase'] in ['begin','end','dispatch_begin','dispatch_end']]
        for invalid in [501,1000]:
            cases.append(dict(id=f'boundary_{family}_{invalid}',actions=[actions[0],['dispatch','0',lines[3].replace(family+'1000',family+str(invalid))]],files={},returncode=1,events=prefix,expected_boundary=True))
        receipts.append(dict(family=family,command=cmd,actions_sha256=ah,events_sha256=sha(ep.read_bytes()),stdout_sha256=sha(run.stdout),stderr_sha256=sha(run.stderr),returncode=run.returncode,nonce=nonce))
        print('Bounded native',family,flush=True)
    (BUILD/'cases.json').write_text(json.dumps(cases)+'\n')
    (BUILD/'receipt.json').write_text(json.dumps(dict(identity=identity,profile_sha256=sha((Q/'profile.json').read_bytes()),initial_sha256=sha((Q/'initial_state.json.gz').read_bytes()),original_sha256=sha((Q/'observations.json.gz').read_bytes()),cases_sha256=sha((BUILD/'cases.json').read_bytes()),runs=receipts),indent=2)+'\n')
if __name__=='__main__':main()
