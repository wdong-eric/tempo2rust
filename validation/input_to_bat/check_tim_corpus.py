#!/usr/bin/env python3
"""Complete N action/state reuse in AC staging, preserving immutable N evidence."""
import argparse,hashlib,json,os,re,subprocess,sys,uuid
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parent.parent;N=ROOT/'validation/tim_io';BUILD=HERE/'build/n'
def sha(b):return hashlib.sha256(b).hexdigest()
def main():
    ap=argparse.ArgumentParser();ap.add_argument('--release',action='store_true');ap.add_argument('--case',action='append');ap.add_argument('--reuse-native',action='store_true');a=ap.parse_args()
    BUILD.mkdir(exist_ok=True,parents=True)
    sys.path.insert(0,str(N));import run as n
    profile=json.loads((N/'profile.json').read_text());exe=Path(profile['executable_path']);identity=n.execution_identity(exe,profile,True)
    selected=[];native=[]
    for index,case in enumerate(json.loads((N/'cases.json').read_text())):
        if a.case and case['id'] not in a.case:continue
        work=BUILD/f'{index:03d}';work.mkdir(exist_ok=True)
        actions=[[v.replace('{case}',str(work)) for v in action] for action in case['actions']]
        # Native caller strcpy targets a 100-byte deleteFileName. Staging paths
        # must remain within that source capacity; no UB-derived oracle is run.
        for action in actions:
            if action[:1]==['set'] and action[2]=='deleteFileName':
                assert len(action[3].encode())<100, (case['id'],'deleteFileName staging capacity')
        if not a.reuse_native:
            for name,raw in case['files'].items():
                path=work/name;path.parent.mkdir(exist_ok=True,parents=True);path.write_bytes(bytes.fromhex(raw))
            script=work/'actions.tsv';script.write_text('\n'.join('\t'.join('x'+v.encode().hex() for v in action) for action in actions)+'\n');ih=sha(script.read_bytes());nonce=uuid.uuid4().hex
            ep=work/'events.jsonl';ep.unlink(missing_ok=True);cmd=[str(exe),str(script),str(ep),nonce]
            proc=subprocess.run(cmd,env={**os.environ,'LC_ALL':'C','LANG':'C','TEMPO2':str(N/'build/runtime')},capture_output=True,timeout=90)
            assert proc.returncode==case['returncode'],(case['id'],proc.returncode,proc.stderr)
            events=[json.loads(line) for line in ep.read_text().splitlines()];assert events.pop(0)=={'phase':'launch','nonce':nonce}
            assert sha(script.read_bytes())==ih;n.execution_identity(exe,profile)
            outputs={name:(work/name).read_bytes().hex() if (work/name).exists() else None for name in case['outputs']}
            data={**case,'workdir':str(work),'actions':actions,'events':events,'outputs':outputs,'stdout_hex':proc.stdout.hex(),'stderr_hex':proc.stderr.hex()}
            (work/'reference.json').write_text(json.dumps(data)+'\n')
            receipt=dict(command=cmd,executable_sha256=identity['sha256'],events_sha256=sha(ep.read_bytes()),inputs_sha256=ih,nonce=nonce,returncode=proc.returncode,reference_sha256=sha((work/'reference.json').read_bytes()))
            (work/'native.json').write_text(json.dumps(receipt,indent=2)+'\n')
        data=json.loads((work/'reference.json').read_text());receipt=json.loads((work/'native.json').read_text());assert receipt['reference_sha256']==sha((work/'reference.json').read_bytes());assert receipt['executable_sha256']==identity['sha256']
        data['source_directory']=next(x[2:] for x in profile['command'] if x.startswith('-I'))+'/'
        selected.append(data);native.append(receipt)
    (BUILD/'cases.json').write_text(json.dumps(selected)+'\n')
    if any(c['id'].startswith('jbo') for c in selected) and not (HERE/'build/jbo-corrected/receipt.json').exists():
        # The correction needs the original histories just recorded above.
        subprocess.run([sys.executable,str(HERE/'corrected_jbo.py')],check=True)
    env={**os.environ,'CARGO_HOME':str(ROOT/'rust/.cargo-home'),'GMP_MPFR_SYS_CACHE':str(ROOT/'rust/target/gmp-cache'),'AC_TIM_CORPUS':str(BUILD)}
    cmd=['cargo','test','--manifest-path',str(ROOT/'rust/Cargo.toml'),'--locked','--offline',*(['--release'] if a.release else []),'--lib','source_tim_corpus_tests::n_corpus','--','--ignored','--exact','--nocapture','--test-threads=1']
    receipt=BUILD/('receipt-'+('release' if a.release else 'debug')+'.json');receipt.unlink(missing_ok=True)
    run=subprocess.run(cmd,env=env)
    differences=[];qualified=[]
    for case in selected:
        path=Path(case['workdir'])/'rust-streams.json'
        if not path.exists():differences.append(case['id']+': missing Rust streams');continue
        actual=json.loads(path.read_text());expected=case
        if case['id'].startswith('jbo'):
            correction=HERE/'build/jbo-corrected';proof=json.loads((correction/'receipt.json').read_text());r=next(r for r in proof['runs'] if r['id']==case['id'])
            cp=correction/(case['id']+'-streams.json');assert sha(cp.read_bytes())==r['streams_sha256'];assert sha((Path(case['workdir'])/'reference.json').read_bytes())==r['original_reference_sha256']
            expected=json.loads(cp.read_text());qualified.append({'id':case['id'],'correction':'0053 JBO aliasing; only word diagnostics changed','receipt_sha256':sha((correction/'receipt.json').read_bytes())})
        for channel in ['stdout_hex','stderr_hex']:
            def normalized(value):
                return re.sub(rb'(\['+re.escape(case['source_directory'].encode())+rb'sortToAs.C:\d+\] T=)[0-9.]+( s: )',rb'\1{cpu_seconds}\2',bytes.fromhex(value))
            if normalized(actual[channel])!=normalized(expected[channel]):
                differences.append({'id':case['id'],'channel':channel,'native':normalized(expected[channel]).hex(),'rust':normalized(actual[channel]).hex()})
    (BUILD/'diagnostic-differences.json').write_text(json.dumps(differences,indent=2)+'\n')
    (BUILD/'qualified-diagnostics.json').write_text(json.dumps(qualified,indent=2)+'\n')
    code=run.returncode or bool(differences)
    receipt.write_text(json.dumps({'native_identity':identity,'cases':len(selected),'native_runs':native,'returncode':int(code),'diagnostic_differences':len(differences),'qualified_diagnostics':qualified},indent=2)+'\n')
    raise SystemExit(code)
if __name__=='__main__':main()
