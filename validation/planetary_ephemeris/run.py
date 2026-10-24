"""Fresh twice-run strict ordinary verification. Recording is explicit and one-shot."""
import argparse
import gzip
import json
import struct
import subprocess
from pathlib import Path
from inventory import HERE,ROOT,sha,save,check
from profile import BUILD,ENV,resources,compile_probe,actual_identity,stable
from fixtures import cases,validate_inputs,VECTORS
OUTPUT=HERE/'outputs'
def packed(x):return json.dumps(x,separators=(',',':'),ensure_ascii=True,allow_nan=False).encode()
def write_gzip(p,x):p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(gzip.compress(packed(x),mtime=0))
def read_gzip(p):return json.loads(gzip.decompress(p.read_bytes()))
def compare(a,b,path='$'):
    if type(a) is not type(b):raise AssertionError(path+': type mismatch')
    if isinstance(a,dict):
        if list(a)!=list(b):raise AssertionError(path+': ordered field membership')
        for k in a:compare(a[k],b[k],path+'.'+k)
    elif isinstance(a,list):
        if len(a)!=len(b):raise AssertionError(path+': length')
        for i in range(len(a)):compare(a[i],b[i],f'{path}[{i}]')
    elif a!=b:raise AssertionError(path+': '+repr(a)[:140]+' != '+repr(b)[:140])
def validate_numbers(x):
    if isinstance(x,dict):
        if 'class' in x:
            assert x['class'] in ('finite','NaN','+Infinity','-Infinity')
            if x['class']=='finite':
                if 'bits' in x:
                    assert list(x)==['class','bits'] and len(x['bits'])==16
                    u=int(x['bits'],16);assert ((u>>52)&2047)!=2047
                else:
                    assert list(x)==['class','little_endian_bytes'] and len(x['little_endian_bytes'])==32
                    u=int.from_bytes(bytes.fromhex(x['little_endian_bytes']),'little');assert ((u>>112)&32767)!=32767
            else:assert list(x)==['class']
        for v in x.values():validate_numbers(v)
    elif isinstance(x,list):
        for v in x:validate_numbers(v)
def verify_resources(manifest):
    for n,v in manifest.items():
        p=BUILD/'resources'/(n+'.bin')
        if v.get('exists') is False:assert not p.exists()
        else:assert len(p.read_bytes())==v['size'] and sha(p.read_bytes())==v['sha256'],'resource changed: '+n

def execute(case,exe,profile,raw,resource_manifest):
    validate_inputs(case);work=BUILD/'cases'/case['id'];work.mkdir(parents=True,exist_ok=True)
    commands=work/'actions.tsv';data=('\n'.join('\t'.join('x'+v.encode().hex() for v in a) for a in case['actions'])+'\n').encode();commands.write_bytes(data);assert commands.read_bytes()==data
    before=actual_identity(exe,profile);verify_resources(resource_manifest)
    event_file=work/'events.jsonl';event_file.unlink(missing_ok=True)
    proc=subprocess.run([str(exe),str(commands),str(event_file),str(BUILD/'resources')],capture_output=True,env=ENV,timeout=60)
    after=actual_identity(exe,profile);verify_resources(resource_manifest);assert commands.read_bytes()==data
    event_bytes=event_file.read_bytes() if event_file.exists() else b'';raw.mkdir(parents=True,exist_ok=True)
    (raw/(case['id']+'.stdout')).write_bytes(proc.stdout);(raw/(case['id']+'.stderr')).write_bytes(proc.stderr);(raw/(case['id']+'.jsonl.gz')).write_bytes(gzip.compress(event_bytes,mtime=0))
    # Resources always use the same actual path. No numeric/diagnostic substitutions.
    result=dict(id=case['id'],classification=case['classification'],actions=case['actions'],status=proc.returncode,events=[json.loads(s) for s in event_bytes.decode().splitlines()],stdout=proc.stdout.decode('latin1'),stderr=proc.stderr.decode('latin1'))
    receipt=dict(id=case['id'],case_sha256=sha(packed(case)),input_sha256=sha(data),resources=resource_manifest,launch_before=before,launch_after=after,raw_sha256={k:sha(v) for k,v in [('events',event_bytes),('stdout',proc.stdout),('stderr',proc.stderr)]})
    return result,receipt

def validate(case,result,observer=False):
    assert result['status']==case['status'],(case['id'],result['status'],result['stderr'][-1000:])
    expected=[]
    for i,a in enumerate(case['actions']):
        expected.append([i,'begin',a[0]])
        if case['status']==0 or i<len(case['actions'])-1:expected.append([i,'end',a[0]])
        elif observer:expected.append([i,'exit',a[0]])
    compare(expected,[[e['event'],e['phase'],e['operation']] for e in result['events']],case['id']+'.membership')
    for e in result['events']:
        assert list(e)==['event','phase','operation','result','seeds','globals','caller','private','calls','open_handles']
        if not observer:assert e['private'] is e['calls'] is e['open_handles'] is None
        if e['caller'] is not None:
            caller=e['caller'];assert list(caller)==['pulsars','observations']
            assert len(caller['pulsars'])==2 and len(caller['observations'])==12
            for p in caller['pulsars']:
                assert list(p)==['owner','observation_owner','nobs','units','eclCoord','ephemeris','ephemeris_tail','parameters','offsets']
                assert len(p['parameters'])==5 and len(p['offsets'])==3
                for parameter in p['parameters']:
                    assert list(parameter)==['index','owner','aSize','values','set']
                    assert parameter['aSize']==9 and len(parameter['values'])==len(parameter['set'])==9
                for offset in p['offsets']:
                    assert list(offset)==['count','backing'] and len(offset['backing'])==500
                    assert all(len(row)==2 for row in offset['backing'])
            for o in caller['observations']:
                assert list(o)==['owner','sat','teph','telID','nclock','clock_steps','nFlags','flags','vectors','planets']
                assert list(o['vectors'])==list(VECTORS) and all(len(v)==6 for v in o['vectors'].values())
                assert len(o['planets'])==3 and all(len(group)==9 and all(len(row)==6 for row in group) for group in o['planets'])
                assert len(o['clock_steps'])==30 and len(o['flags'])==40
                assert o['telID']['initialized_bytes']==100 and len(o['telID']['hex'])==200
        if observer:
            assert list(e['private'])==['random','gaussian','random_entries','attempts']
            if e['private']['random'] is not None:
                assert list(e['private']['random'])==['iff','ix','result','r']
                assert len(e['private']['random']['ix'])==3 and len(e['private']['random']['r'])==100
            if e['private']['gaussian'] is not None:assert list(e['private']['gaussian'])==['iset','gset']
    p=result['events'][1]['result'];assert (p['double_digits'],p['longdouble_digits'],p['longdouble_size'],p['long_size'])==(53,113,16,8)
    validate_numbers(result)
def harness():return {p.name:sha(p.read_bytes()) for p in sorted(HERE.iterdir()) if p.suffix in ('.py','.cc')}
def frozen_cases():
    c=cases();compare(json.loads((OUTPUT/'frozen/cases.json').read_text()),c,'catalog');return c

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--record',action='store_true');args=parser.parse_args()
    check();catalog=frozen_cases();oracle=OUTPUT/'reference.json.gz';mp=OUTPUT/'manifest.json'
    if args.record:assert not oracle.exists() and not mp.exists(),'record never overwrites oracle'
    old=sha(oracle.read_bytes()) if oracle.exists() else None
    resource_manifest=resources();exe,profile=compile_probe()
    current=dict(cases_sha256=sha(packed(catalog)),harness=harness(),native=stable(profile),resources=resource_manifest,frozen={p.name:sha(p.read_bytes()) for p in sorted((OUTPUT/'frozen').iterdir())})
    baseline=None
    if not args.record:
        compare(json.loads(mp.read_text()),current,'manifest');baseline=read_gzip(oracle);compare([c['id'] for c in catalog],[r['id'] for r in baseline],'case membership')
    results=[];receipts=[]
    for i,c in enumerate(catalog):
        r,receipt=execute(c,exe,profile,OUTPUT/('record_raw' if args.record else 'check_raw'),resource_manifest);validate(c,r)
        if baseline is not None:compare(baseline[i],r,c['id'])
        results.append(r);receipts.append(dict(first=receipt));print('PASS first',c['id'],flush=True)
    second_exe,second_profile=compile_probe()
    compare(stable(profile),stable(second_profile),'second fresh build profile')
    for i,(c,r) in enumerate(zip(catalog,results)):
        again,rr=execute(c,second_exe,second_profile,OUTPUT/'replay_raw',resource_manifest);validate(c,again);compare(r,again,c['id']+'.fresh_rebuild_replay');receipts[i]['replay']=rr;print('PASS rebuilt replay',c['id'],flush=True)
    profile=second_profile;exe=second_exe
    if args.record:write_gzip(oracle,results);save(mp,current)
    else:assert sha(oracle.read_bytes())==old
    write_gzip(OUTPUT/'execution_receipts.json.gz',receipts);check();actual_identity(exe,profile)
    save(OUTPUT/'last_execution.json',dict(status='PASS',mode='record' if args.record else 'verify',cases=len(results),events=sum(len(r['events']) for r in results),reference_sha256=sha(oracle.read_bytes()),manifest_sha256=sha(mp.read_bytes()),profile=profile,harness=harness()))
    print('PASS fresh twice-run exact verification:',len(results),'cases')
if __name__=='__main__':main()
