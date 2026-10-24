"""Fresh exact verification by default; explicit recording creates a new local oracle."""
import argparse
import gzip
import json
import shutil
import subprocess
from pathlib import Path
from inventory import HERE, ROOT, sha, save, preservation_status
from profile import BUILD, ENV, compile_probe, actual_identity, stable
from fixtures import cases, UNRUN, validate_inputs

OUTPUT = HERE/'outputs'

def packed(x): return json.dumps(x,separators=(',',':'),ensure_ascii=True,allow_nan=False).encode()
def write_gzip(path,x):
    path.parent.mkdir(parents=True,exist_ok=True)
    path.write_bytes(gzip.compress(packed(x),mtime=0))
def read_gzip(path): return json.loads(gzip.decompress(path.read_bytes()))

def compare(a,b,path='$'):
    if type(a) is not type(b): raise AssertionError(path+': type mismatch')
    if isinstance(a,dict):
        if list(a)!=list(b): raise AssertionError(path+': ordered field membership')
        for k in a: compare(a[k],b[k],path+'.'+k)
    elif isinstance(a,list):
        if len(a)!=len(b): raise AssertionError(path+': length')
        for i in range(len(a)): compare(a[i],b[i],f'{path}[{i}]')
    elif a!=b: raise AssertionError(f'{path}: {str(a)[:120]!r} != {str(b)[:120]!r}')

def normalize(s,exe):
    # Exact declared roots only, never diagnostics, line numbers or numbers.
    return s.replace(str(exe.parent/'source'),'{source}').replace(str(BUILD/'cases'),'{cases}').replace(str(HERE),'{observer}')

def file_identities(root):
    return {str(p.relative_to(root)):dict(size=p.stat().st_size,sha256=sha(p.read_bytes()))
            for p in sorted(root.rglob('*')) if p.is_file()}

def check_inputs(work,case,commands,input_sha):
    assert sha(commands.read_bytes())==input_sha, 'runner input changed'
    for name,value in case['files'].items():
        assert (work/name).read_bytes()==value.encode(), 'resource changed: '+name

def execute(case,exe,profile,raw):
    validate_inputs(case)
    work=BUILD/'cases'/case['id']
    if work.exists():shutil.rmtree(work)
    work.mkdir(parents=True)
    for name,value in case['files'].items():
        p=work/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(value)
    input_files=file_identities(work)
    actions=[[v.replace('{case}',str(work)) for v in a] for a in case['actions']]
    commands=work/'actions.tsv'
    commands.write_text('\n'.join('\t'.join('x'+v.encode().hex() for v in a) for a in actions)+'\n')
    input_sha=sha(commands.read_bytes()); before=actual_identity(exe,profile)
    # Verify the actual on-disk resource bytes immediately before launch.
    check_inputs(work,case,commands,input_sha)
    events=work/'events.jsonl'
    proc=subprocess.run([str(exe),str(commands),str(events)],capture_output=True,env=ENV,timeout=30)
    after=actual_identity(exe,profile)
    assert sha(commands.read_bytes())==input_sha
    raw.mkdir(parents=True,exist_ok=True)
    (raw/(case['id']+'.stdout')).write_bytes(proc.stdout)
    (raw/(case['id']+'.stderr')).write_bytes(proc.stderr)
    event_bytes=events.read_bytes() if events.exists() else b''
    (raw/(case['id']+'.jsonl.gz')).write_bytes(gzip.compress(event_bytes,mtime=0))
    payload=dict(id=case['id'],classification=case['classification'],actions=case['actions'],
        status=proc.returncode,events=[json.loads(normalize(s,exe)) for s in event_bytes.decode().splitlines()],
        stdout=normalize(proc.stdout.decode('latin1'),exe),stderr=normalize(proc.stderr.decode('latin1'),exe))
    receipt=dict(id=case['id'],case_sha256=sha(packed(case)),input_sha256=input_sha,
        resources_before=input_files,resources_after=file_identities(work),
        launch_before=before,launch_after=after,
        raw_sha256={k:sha(v) for k,v in [('events',event_bytes),('stdout',proc.stdout),('stderr',proc.stderr)]})
    return payload,receipt

def validate(case,result):
    assert result['id']==case['id']
    assert result['status']==case['status'],(case['id'],result['status'],result['stderr'][-1000:])
    expected=[]
    for i,a in enumerate(case['actions']):
        expected.append([i,'begin',a[0]])
        if case['status']==0 or i<len(case['actions'])-1:expected.append([i,'end',a[0]])
        # The unchanged ordinary executable has no exit-state observer.
    compare(expected,[[e['event'],e['phase'],e['operation']] for e in result['events']],case['id'])
    for e in result['events']:
        assert list(e)==(['event','phase','operation','caller'] if e['phase']=='begin' else
            ['event','phase','operation','result','state','caller','private'])
        if e['phase']=='end': assert e['private'] is None
    profile=result['events'][1]['result'];assert profile['double_digits']==53 and profile['longdouble_digits']==113 and profile['longdouble_size']==16

def harness():
    return {p.name:sha(p.read_bytes()) for p in sorted(HERE.iterdir()) if p.suffix in ('.py','.cc')}

def manifest(catalog,profile):
    return dict(cases_sha256=sha(packed(catalog)),harness=harness(),native=stable(profile),
        contract_sha256=sha((HERE/'CONTRACT.md').read_bytes()),
        frozen_sha256={p.name:sha(p.read_bytes()) for p in sorted((OUTPUT/'frozen').iterdir())})

def frozen_cases():
    catalog=cases();stored=json.loads((OUTPUT/'manifest.json').read_text())
    compare(json.loads((OUTPUT/'frozen/cases.json').read_text()),catalog,'catalog')
    compare(stored['cases_sha256'],sha(packed(catalog)),'case identity')
    compare(stored['harness'],harness(),'harness identity')
    compare(stored['contract_sha256'],sha((HERE/'CONTRACT.md').read_bytes()),'contract identity')
    compare(stored['frozen_sha256'],{p.name:sha(p.read_bytes()) for p in sorted((OUTPUT/'frozen').iterdir())},'frozen membership')
    return catalog

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--record',action='store_true')
    args=parser.parse_args();receipt=OUTPUT/'last_execution.json';receipt.unlink(missing_ok=True)
    preservation_before=preservation_status();catalog=cases();oracle=OUTPUT/'reference.json.gz';mp=OUTPUT/'manifest.json'
    if args.record:
        assert not oracle.exists() and not mp.exists(), 'record never overwrites an existing oracle'
        save(OUTPUT/'frozen/cases.json',catalog);save(OUTPUT/'unrun.json',UNRUN)
    else:
        oracle_before=sha(oracle.read_bytes())
        catalog=frozen_cases()
    exe,profile=compile_probe();current=manifest(catalog,profile)
    if not args.record:compare(json.loads(mp.read_text()),current,'manifest')
    results=[];receipts=[];baseline=None
    if not args.record:
        frozen_results=read_gzip(oracle)
        compare([c['id'] for c in catalog],[r['id'] for r in frozen_results],'exact oracle membership')
        baseline={r['id']:r for r in frozen_results}
    if baseline is not None:compare(list(baseline),[c['id'] for c in catalog],'oracle membership')
    for c in catalog:
        r,execution=execute(c,exe,profile,OUTPUT/('record_raw' if args.record else 'check_raw'))
        validate(c,r)
        replay,again=execute(c,exe,profile,OUTPUT/'replay_raw');validate(c,replay);compare(r,replay,c['id']+'.fresh')
        if baseline is not None:compare(baseline[c['id']],r,c['id'])
        results.append(r);receipts.append(dict(first=execution,replay=again));print('PASS',c['id'],flush=True)
    if args.record:write_gzip(oracle,results);save(mp,current)
    else:assert sha(oracle.read_bytes())==oracle_before, 'oracle changed during verification'
    write_gzip(OUTPUT/'execution_receipts.json.gz',receipts)
    preservation_after=preservation_status();actual_identity(exe,profile)
    status='PRESERVATION_DRIFT' if preservation_before or preservation_after else 'PASS'
    save(receipt,dict(status=status,native_status='PASS',preservation_before=preservation_before,preservation_after=preservation_after,mode='record' if args.record else 'check',cases=len(results),
        events=sum(len(r['events']) for r in results),profile=profile,harness=harness(),
        reference_sha256=sha(oracle.read_bytes()),manifest_sha256=sha(mp.read_bytes()),
        receipts_sha256=sha((OUTPUT/'execution_receipts.json.gz').read_bytes())))
    print('PASS native fresh execution and exact ordered replay',len(results))
    assert status=='PASS', 'native evidence completed; preservation gate remains unresolved: '+str(preservation_after or preservation_before)

if __name__=='__main__':main()
