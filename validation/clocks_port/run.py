#!/usr/bin/env python3
"""L-only fresh-process execution against immutable J; no native probe execution."""
import argparse, copy, hashlib, json, os, platform, secrets, shutil, struct, subprocess, tempfile
from pathlib import Path
HERE=Path(__file__).resolve().parent
ROOT=HERE.parent.parent
RUST=ROOT/'rust'
J=ROOT/'validation/clocks'
OUT=RUST/'verification/clocks'
ENV=dict(os.environ,CARGO_HOME=str(RUST/'.cargo-home'),GMP_MPFR_SYS_CACHE=str(RUST/'target/gmp-cache'),LC_ALL='C')
ENV.pop('TEMPO2_ALIAS',None)
BOUNDS={'table_singleton_exact':'table_bracket_storage','table_singleton_nan':'table_bracket_storage','table_empty_value':'table_sample_storage','table_empty_start':'table_sample_storage','table_empty_end':'table_sample_storage','table_header_256':'table_header_storage','table_header_800':'table_header_storage','toa_capacity_29':'clock_correction_storage'}
def load(p):return json.loads(p.read_text())
def save(p,v):p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(v,indent=2,sort_keys=True)+'\n')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def build(profile):
    cmd=['cargo','test','--lib','--locked','--offline','--no-run','--message-format=json']+(['--release'] if profile=='release' else [])
    p=subprocess.run(cmd,cwd=RUST,env=ENV,capture_output=True,text=True)
    save(OUT/(profile+'_build.json'),dict(command=cmd,returncode=p.returncode,stderr=p.stderr))
    assert p.returncode==0,p.stderr
    exe=[json.loads(line)['executable'] for line in p.stdout.splitlines() if line.startswith('{') and json.loads(line).get('reason')=='compiler-artifact' and json.loads(line).get('executable') and json.loads(line)['target']['kind']==['lib']]
    assert len(exe)==1,exe
    return Path(exe[0])
def compare(a,b,path='$',counts=None):
    if counts is None:counts=dict(finite=0,nonfinite=0,discrete=0)
    assert type(a)==type(b),(path,'type',type(a),type(b))
    if isinstance(b,dict):
        assert a.keys()==b.keys(),(path,'fields',a.keys()^b.keys())
        if 'class' in b:
            assert a['class']==b['class'],(path,'nonfinite class',a,b)
            if b['class']=='finite':
                for n in (a,b):assert struct.pack('>d',float(n['decimal'])).hex()==n['bits'],(path,'decimal roundtrip',n)
                assert a['bits']==b['bits'],(path,'finite bits',a,b)
                counts['finite']+=1
            else:counts['nonfinite']+=1
            return counts
        for k in b:
            if k in ('longdouble_size','longdouble_digits','table_size','observation_size') and path.endswith('.result'):continue
            if k=='sat':continue  # exact narrowed sat_binary64 is compared, Rug precision stays 128
            compare(a[k],b[k],path+'.'+k,counts)
    elif isinstance(b,list):
        assert len(a)==len(b),(path,'length',len(a),len(b))
        for i in range(len(b)):compare(a[i],b[i],f'{path}[{i}]',counts)
    else:
        assert a==b,(path,repr(a)[:500],repr(b)[:500]);counts['discrete']+=1
    return counts

def validate_identity(expected,actual):
    assert expected==actual,('execution identity mismatch',expected,actual)

def frozen_inputs():
    manifest=load(J/'outputs/manifest.json');cases=load(J/'cases.json')
    assert sha(J/'build/probe')==manifest['native']['executable_sha256']
    assert hashlib.sha256(json.dumps(cases,sort_keys=True).encode()).hexdigest()==manifest['cases_sha256']
    assert sha(J/'fixtures.py')==manifest['fixtures_sha256'] and sha(J/'probe.cc')==manifest['harness_sha256']
    for case in cases:
        observed={p:dict(sha256=hashlib.sha256(t.encode()).hexdigest(),size=len(t.encode())) for p,t in case['files'].items()}
        validate_identity(manifest['resource_manifest']['synthetic'][case['id']],observed)
    for path,identity in manifest['resource_manifest']['real'].items():
        p=ROOT/'tempo2'/path;validate_identity(identity,dict(sha256=sha(p),size=p.stat().st_size))
    source=manifest['source'];pin=source['source_revision']
    assert subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'rev-parse','HEAD'],text=True).strip()==pin
    for name,identity in source['sources'].items():
        if identity['origin']=='pinned Git blob':
            data=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',pin+':'+name])
            assert hashlib.sha256(data).hexdigest()==identity['sha256'],name
    return {str(p.relative_to(ROOT)):sha(p) for p in J.rglob('*') if p.is_file() and not any(x in p.parts for x in ['build','__pycache__'])}

def executed_sources():
    paths=[*RUST.joinpath('src').glob('*.rs'),*RUST.joinpath('examples').glob('*.rs'),*HERE.glob('*.py')]
    paths += [RUST/n for n in ['Cargo.toml','Cargo.lock','build.rs']]+[HERE/'projection.json']
    return {str(p.relative_to(ROOT)):sha(p) for p in sorted(paths)}

def execute(case,exe,build_root):
    work=build_root/'cases'/case['id']
    if work.exists():shutil.rmtree(work)
    work.mkdir(parents=True)
    for name,text in case['files'].items():p=work/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(text)
    actions=[[str(x).replace('{case}',str(work)).replace('{real}',str(ROOT/'tempo2/T2runtime')) for x in a] for a in case['actions']]
    inp=work/'input.json';output=work/'output.json';save(inp,actions);output.unlink(missing_ok=True)
    command=[str(exe),'--exact','clock_tests::probe','--ignored','--nocapture']
    token=secrets.token_hex(16);before_sha=sha(exe)
    expected_identity=dict(case_id=case['id'],run_token=token,input_sha256=sha(inp),executable_sha256=before_sha)
    env=dict(ENV,CLOCK_INPUT=str(inp),CLOCK_OUTPUT=str(output),CLOCK_SOURCE_LOCATION=str(J/'build/source')+'/',CLOCK_CASE_ID=case['id'],CLOCK_RUN_TOKEN=token)
    proc=subprocess.run(command,cwd=RUST,env=env,capture_output=True,text=True,timeout=20)
    assert proc.returncode==0 and 'test clock_tests::probe ... ok' in proc.stdout and '1 passed; 0 failed' in proc.stdout,(case['id'],'probe execution',proc.stdout,proc.stderr)
    assert output.is_file(),(case['id'],'missing fresh output')
    raw=load(output)
    validate_identity(expected_identity,dict(case_id=raw["case_id"],run_token=raw["run_token"],input_sha256=raw["input_sha256"],executable_sha256=sha(exe)))
    # J captures subprocess text with Python universal-newline translation.
    for channel in ("stdout","stderr"):raw[channel]=raw[channel].replace("\r\n","\n").replace("\r","\n")
    def normalize(v):
        if isinstance(v,str):return v.replace(str(build_root),'{build}').replace(str(J/'build'),'{build}').replace(str(ROOT/'tempo2/T2runtime'),'{real}').replace(str(J),'{clocks}')
        if isinstance(v,list):return [normalize(x) for x in v]
        if isinstance(v,dict):return {k:normalize(x) for k,x in v.items()}
        return v
    identity=dict(id=case['id'],run_token=token,input_sha256=sha(inp),executable_sha256=sha(exe),output_sha256=sha(output),command=command,actual_process_returncode=proc.returncode,probe_confirmed=True)
    return normalize(raw),identity

def comparison_record(case,raw):
    return dict(id=case['id'],classification=case['classification'],actions=case['actions'],returncode=raw['returncode'],timeout=False,events=raw['events'],stdout=raw['stdout'],stderr=raw['stderr'])
def expected_record(ref):return {k:v for k,v in ref.items() if k!='command'}
def validate_boundary(c,raw):
    assert raw['failure']['field']==BOUNDS[c['id']],(c['id'],'boundary',raw['failure'])
    assert raw['failure']['event']==c['last_action']
    before=[e for e in raw['events'] if e['phase']=='end']
    expected_events=[]
    for i,a in enumerate(c['actions'][:c['last_action']+1]):
        expected_events.append((i,'begin',a[0]))
        if i<c['last_action']:expected_events.append((i,'end',a[0]))
    assert [(e['event'],e['phase'],e['operation']) for e in raw['events']]==expected_events
    if c['id']=='toa_capacity_29':
        assert raw['failure']['observations'][0]['count']==30
        initial=next(e['result']['observation'] for e in before if e['operation']=='obs')
        compare(raw['failure']['observations'][0]['slots'],initial['slots'])
    if c['id'].startswith('table_header_'):
        header=c['files']['sample.clk'].encode()[:256]
        assert raw['failure']['table_header_initialized_bytes']==header.hex()
    assert raw['failure']['reentry']=='same terminal state'

def validate_delivery(payload):
    cases=load(J/'cases.json');refs=load(J/'outputs/reference.json')['cases']
    results=payload['results']
    assert [r['id'] for r in results]==[c['id'] for c in cases], 'missing/extra/reordered cases'
    assert not payload['failures'], 'recorded failures'
    totals=dict(finite=0,nonfinite=0,discrete=0)
    for c,r in zip(cases,results):
        if c['id']=='weights_cycle':
            assert r==dict(id=c['id'],status='unrun cycle');continue
        raw=r['raw']
        assert raw.keys()=={'case_id','run_token','input_sha256','events','stdout','stderr','returncode','failure','logs'}, 'raw fields'
        assert raw['case_id']==c['id'] and len(raw['run_token'])==32 and len(raw['input_sha256'])==64
        assert raw['logs'].keys()=={'keys','warnings','errors','warned_multiple'}
        if c['id'] in BOUNDS:
            assert r['status']=='safe Rust boundary only'
            validate_boundary(c,raw);continue
        assert r['status']=='PASS'
        counts=compare(comparison_record(c,raw),expected_record(next(x for x in refs if x['id']==c['id'])),c['id'])
        assert r['counts']==counts
        if c['classification']=='exit':
            assert raw['failure']['field']=='source_exit' and raw['failure']['event']==c['last_action']
            assert raw['failure']['reentry']=='same terminal state'
        else:assert raw['failure'] is None
        for k,v in counts.items():totals[k]+=v
    assert totals==payload['counts'],'comparison counts'
    ids=payload['identities']
    assert [i['id'] for i in ids]==[c['id'] for c in cases if c['id']!='weights_cycle'],'execution identity set'
    assert len({i['run_token'] for i in ids})==150,'reused run tokens'
    for i,r in zip(ids,[r for r in results if r['id']!='weights_cycle']):
        validate_identity({k:i[k] for k in ['run_token','input_sha256']},{k:r['raw'][k] for k in ['run_token','input_sha256']})
        assert i['executable_sha256']==payload['executable_sha256'] and i['probe_confirmed'] and i['actual_process_returncode']==0
    return totals

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--profile',choices=['debug','release'],default='debug');parser.add_argument('--case');args=parser.parse_args()
    projection=load(HERE/'projection.json');cases=load(J/'cases.json');refs=load(J/'outputs/reference.json')['cases']
    assert sha(J/'outputs/reference.json')==projection['reference_sha256']
    assert [c['id'] for c in cases]==[c['id'] for c in projection['cases']] and len(cases)==151
    for case,rule in zip(cases,projection['cases']):
        validate_identity({k:case[k] for k in ['id','classification','actions','last_action']},{k:rule[k] for k in ['id','classification','actions','last_action']})
    assert len(refs)==142
    frozen=frozen_inputs();sources=executed_sources()
    exe=build(args.profile);results=[];failures=[];identities=[];totals=dict(finite=0,nonfinite=0,discrete=0)
    with tempfile.TemporaryDirectory(prefix='clockL-',dir='/private/tmp') as tmp:
        # Equal source pathname lengths preserve TKlog's pre-strip truncation.
        length=len(str(J/'build'))-len(tmp)-1
        assert length>0
        base=Path(tmp)/('x'*length);base.mkdir()
        assert len(str(base))==len(str(J/'build'))
        for c in cases:
            if args.case and c['id']!=args.case:continue
            if c['id']=='weights_cycle':results.append(dict(id=c['id'],status='unrun cycle'));continue
            try:
                raw,identity=execute(c,exe,base);identities.append(identity)
                if c['id'] in BOUNDS:
                    validate_boundary(c,raw)
                    results.append(dict(id=c['id'],status='safe Rust boundary only',raw=raw));print(c['id'],'RUST BOUNDARY',flush=True);continue
                expected=next(r for r in refs if r['id']==c['id']);actual=comparison_record(c,raw)
                counts=compare(actual,expected_record(expected),c['id'])
                if c['classification']=='exit':assert raw['failure']['field']=='source_exit' and raw['failure']['event']==c['last_action'],(c['id'],'source terminal')
                else:assert raw['failure'] is None
                for k,v in counts.items():totals[k]+=v
                results.append(dict(id=c['id'],status='PASS',counts=counts,raw=raw));print(c['id'],'PASS',flush=True)
            except (AssertionError,subprocess.TimeoutExpired) as e:
                failures.append(dict(id=c['id'],error=str(e)));results.append(dict(id=c['id'],status='FAIL',raw=locals().get('raw')));print(c['id'],'FAIL',str(e)[:700],flush=True)
    assert sources==executed_sources(),'executed sources changed during comparison'
    assert frozen==frozen_inputs(),'frozen J inputs changed'
    payload=dict(source_hashes=sources,frozen_hashes=frozen,profile=args.profile,results=results,counts=totals,failures=failures,identities=identities,executable_sha256=sha(exe),executable=str(exe),runner_sha256=sha(Path(__file__)),host=platform.platform())
    if not args.case:validate_delivery(payload)
    save(OUT/(args.profile+('_'+args.case if args.case else '')+'_direct.json'),payload)
    assert not failures,failures
if __name__=='__main__':main()
