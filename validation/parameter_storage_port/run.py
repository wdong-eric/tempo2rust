#!/usr/bin/env python3
"""Exact source field/case comparison, fresh build identity and O-only outputs."""
import argparse,copy,gzip,hashlib,importlib.util,json,os,platform,secrets,subprocess,sys,tempfile
from fractions import Fraction
from pathlib import Path
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];RUST=ROOT/'rust';OUT=RUST/'verification/parameter_storage'
ENV=dict(os.environ,CARGO_HOME=str(RUST/'.cargo-home'),GMP_MPFR_SYS_CACHE=str(RUST/'target/gmp-cache'),LC_ALL='C')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def load(p):return json.loads(p.read_text())
def save(p,v):p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(v,indent=2,sort_keys=True)+'\n')
def norm(v):
    if isinstance(v,dict):
        if 'class' in v:
            if v['class']!='finite':return {'class':v['class']}
            s=v.get('value',v.get('decimal'));f=Fraction(s)
            sign=s.startswith('-') if not f else f<0
            if 'bytes_le' in v:
                bits=int.from_bytes(bytes.fromhex(v['bytes_le']),'little');exponent=(bits>>112)&32767;mantissa=bits&((1<<112)-1)
                assert exponent!=32767
                exact=Fraction(mantissa+(1<<112) if exponent else mantissa)*Fraction(2)**((exponent or 1)-16383-112)
                if bits>>127:exact=-exact
                assert f==exact and sign==bool(bits>>127),'binary128 decoder/decimal mismatch'
                f=exact
            return {'class':'finite','exact':str(f),'negative':sign}
        return {k:norm(x) for k,x in v.items()}
    if isinstance(v,list):return [norm(x) for x in v]
    return v

def compare(actual,expected,path='$',counts=None):
    if counts is None:counts=dict(fields=0,numbers=0,scalars=0)
    assert type(actual)==type(expected),(path,'type')
    if isinstance(expected,dict):
        assert actual.keys()==expected.keys(),(path,'fields',actual.keys()^expected.keys())
        if set(expected)=={'class','exact','negative'}:counts['numbers']+=1
        counts['fields']+=len(expected)
        for k in expected:compare(actual[k],expected[k],path+'.'+k,counts)
    elif isinstance(expected,list):
        assert len(actual)==len(expected),(path,'extent',len(actual),len(expected))
        for i,(a,b) in enumerate(zip(actual,expected)):compare(a,b,f'{path}[{i}]',counts)
    else:assert actual==expected,(path,str(actual)[:150],str(expected)[:150]);counts['scalars']+=1
    return counts

def expected_cases():
    out=[]
    for rule in load(HERE/'case_projection.json'):
        c=next(c for c in load(ROOT/'validation'/rule['reference']/'observations.json') if c['id']==rule['id'])
        assert c['actions']==rule['actions']
        events=copy.deepcopy(c['events'])
        for e in events:
            if rule['reference']=='parameters' and e['phase']=='end':
                e['changes']={k:v for k,v in e['changes'].items() if '.param_' in k}
                if e['op']=='init':e['result']={'MAX_PARAMS':e['result']['MAX_PARAMS']}
        assert [{k:e[k] for k in ['action','op','phase']} for e in events]==rule['events']
        out.append(dict(id=c['id'],actions=c['actions'],events=events))
    return norm(out)

def expected_boundaries():
    # Start from M's immutable initialized callers, never from the Rust observation.
    reference=next(c for c in load(ROOT/'validation/numeric_toolkit/observations.json') if c['id']=='copyparam_3')['events'][1]['result']
    expected=[]
    kinds={'source_cell':('label','missing_source_cell'),'destination_cell':('label','missing_destination_cell'),'label_nul':('label','missing_string_terminator'),'shortlabel_nul':('shortlabel','missing_string_terminator'),'label_capacity':('label','insufficient_label_storage'),'shortlabel_capacity':('shortlabel','insufficient_label_storage'),'prefit':('prefit','unwritten_value'),'prefitErr':('prefitErr','unwritten_value')}
    for kind,(field,reason) in kinds.items():
        src=copy.deepcopy(reference['source_before']);before=copy.deepcopy(reference['destination_before'])
        if kind=='source_cell':src['cells']=src['cells'][:1];src['capacity']=1
        elif kind=='destination_cell':before['cells']=before['cells'][:1];before['capacity']=1
        elif kind.endswith('_nul'):src['cells'][1][field+'_bytes']='7a'*3
        elif kind.endswith('_capacity'):before['cells'][1][field+'_bytes']='71'*2
        else:src['cells'][1][field]=None
        dst=copy.deepcopy(before);dst['cells'][0]=copy.deepcopy(src['cells'][0])
        if kind not in ['source_cell','destination_cell']:
            for f in ['label','shortlabel']:
                a=bytes.fromhex(src['cells'][1][f+'_bytes']);b=bytearray.fromhex(dst['cells'][1][f+'_bytes'])
                n=min(len(a),len(b),a.index(0)+1 if 0 in a else len(a))
                b[:n]=a[:n];dst['cells'][1][f+'_bytes']=b.hex()
                if f==field:break
            if field in ['prefit','prefitErr']:
                for f in ['val','err','fitFlag','paramSet','prefit','prefitErr']:
                    if f==field:break
                    dst['cells'][1][f]=copy.deepcopy(src['cells'][1][f])
        expected.append(dict(case=kind,error=dict(cell=1,field=field,kind=reason),before=before,source=src,after=dst,reentry='same boundary'))
    return expected

def protected():
    frozen=load(HERE/'protected_hashes.json');assert {p:sha(ROOT/p) for p in frozen}==frozen,'protected evidence changed';return frozen

def identity(exe=None):
    paths=[*RUST.joinpath('src').glob('*.rs'),*RUST.joinpath('examples').glob('*.rs'),*RUST.joinpath('tests').glob('*.rs'),*HERE.glob('*.py'),*HERE.glob('*.json'),HERE/'probe.cc',RUST/'Cargo.toml',RUST/'Cargo.lock',RUST/'build.rs']
    rustc=Path(subprocess.check_output(['rustup','which','rustc'],text=True).strip());cargo=Path(subprocess.check_output(['rustup','which','cargo'],text=True).strip())
    return dict(**({'executable_sha256':sha(exe)} if exe is not None else {}),compiler_sha256=sha(rustc),cargo_sha256=sha(cargo),toolchain=subprocess.check_output(['rustc','-vV'],text=True),sources={str(p.relative_to(ROOT)):sha(p) for p in sorted(paths)})
def validate_identity(expected,actual):assert expected==actual,'execution identity mismatch'

def execute(exe,expected_identity,case_input):
    validate_identity(expected_identity,identity(exe))
    with tempfile.TemporaryDirectory(prefix='parameterO-',dir='/private/tmp') as td:
        work=Path(td);inp=work/'input.json';out=work/'output.json';save(inp,case_input);token=secrets.token_hex(16)
        cmd=[str(exe),'--exact','parameter_storage_tests::probe','--ignored','--nocapture']
        p=subprocess.run(cmd,cwd=RUST,env=dict(ENV,PARAMETER_INPUT=str(inp),PARAMETER_OUTPUT=str(out),PARAMETER_TOKEN=token),capture_output=True,text=True,timeout=60)
        assert p.returncode==0 and 'test parameter_storage_tests::probe ... ok' in p.stdout and '1 passed; 0 failed' in p.stdout,(p.returncode,p.stdout,p.stderr)
        raw=load(out);assert raw['token']==token and raw['input_sha256']==sha(inp),'stale/missing launch identity'
        validate_identity(expected_identity,identity(exe))
        return raw,dict(command=cmd,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr,token=token,input_sha256=sha(inp),output_sha256=sha(out))

def validate(raw,native):
    assert set(raw)=={'token','input_sha256','cases','registry','boundaries','supplemental','initialized_bytes'}
    counts=compare(norm(raw['cases']),expected_cases())
    compare(raw['registry'],load(HERE/'registry_projection.json'),counts=counts)
    compare(norm(raw['boundaries']),norm(expected_boundaries()),counts=counts)
    compare(norm(raw['supplemental']),norm(native['supplemental']),counts=counts)
    compare(norm(raw['initialized_bytes']),norm(native['initialized_bytes']),counts=counts)
    # Separate full K registry (all names, indices, cells, labels and unwritten classes).
    registry=load(ROOT/'validation/parameters/registry.json')
    projected=[]
    for entry,p in zip(raw['registry'],raw['initialized_bytes']):
        e=copy.deepcopy(entry);e.update(nLinkTo=p['nLinkTo'],nLinkFrom=p['nLinkFrom'])
        e['cells']=[dict(index=i,label=bytes.fromhex(c['label_bytes']).split(b'\0')[0].decode(),shortlabel=bytes.fromhex(c['shortlabel_bytes']).split(b'\0')[0].decode(),**{k:c[k] for k in ['val','err','fitFlag','paramSet']}) for i,c in enumerate(p['cells'])];projected.append(e)
    compare(norm(projected),norm(registry),counts=counts)
    return counts

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--profile',choices=['debug','release'],required=True);args=parser.parse_args();OUT.mkdir(parents=True,exist_ok=True)
    receipt=OUT/(args.profile+'_direct.json');receipt.unlink(missing_ok=True)
    frozen=protected();build_identity=identity();cmd=['cargo','test','--lib','--locked','--offline','--no-run','--message-format=json']+(['--release'] if args.profile=='release' else [])
    proc=subprocess.run(cmd,cwd=RUST,env=ENV,capture_output=True,text=True);save(OUT/(args.profile+'_build.json'),dict(command=cmd,returncode=proc.returncode,stderr=proc.stderr));assert proc.returncode==0,proc.stderr
    artifacts=[json.loads(line) for line in proc.stdout.splitlines() if line.startswith('{')];exe=Path(next(x['executable'] for x in artifacts if x.get('executable') and x.get('target',{}).get('kind')==['lib']))
    before=identity(exe)
    validate_identity(build_identity,{k:v for k,v in before.items() if k!='executable_sha256'})
    # Native module imports this module by name, with the same O paths.
    import native
    native_exe,native_id=native.build();token=secrets.token_hex(16)
    with tempfile.TemporaryDirectory(prefix='parameter-nativeO-',dir='/private/tmp') as td:
        native.validate(native_id)
        path=Path(td)/'output.json';p=subprocess.run([str(native_exe),str(path),token],env=ENV,capture_output=True,text=True,timeout=60)
        assert p.returncode==0 and sha(native_exe)==native_id['executable_sha256'],p.stderr
        native_raw=load(path);assert native_raw['token']==token
        native.validate(native_id)
    raw,launch=execute(exe,before,load(HERE/'case_projection.json'));counts=validate(raw,native_raw)
    import controls
    rejected=controls.run(raw,native_raw,before,exe)
    validate_identity(before,identity(exe));native.validate(native_id);assert frozen==protected()
    for name,data in [(args.profile+'_raw.json.gz',raw),(args.profile+'_native.json.gz',native_raw)]:
        (OUT/name).write_bytes(gzip.compress(json.dumps(data,separators=(',',':')).encode(),mtime=0))
    save(receipt,dict(status='PASS',profile=args.profile,counts=counts,cases=10,events=sum(len(c['events']) for c in raw['cases']),slots=2000,cells=9616,boundaries=8,supplemental_copies=2,identity=before,executable=str(exe),launch=launch,native=native_id,controls=rejected,host=platform.platform(),artifacts={n:sha(OUT/n) for n in [args.profile+'_raw.json.gz',args.profile+'_native.json.gz']}))
    print(args.profile,counts,len(rejected),'controls PASS',flush=True)
if __name__=='__main__':main()
