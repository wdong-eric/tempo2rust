#!/usr/bin/env python3
"""R exact typed observation comparison with identities checked around launches."""
import argparse,copy,gzip,hashlib,importlib.util,json,os,platform,secrets,subprocess,sys,tempfile
from fractions import Fraction
from pathlib import Path
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];RUST=ROOT/'rust';OUT=Path(os.environ.get('TEMPO2_OBSERVATION_VERIFICATION_ROOT', RUST/'verification/observation_storage'))
ENV=dict(os.environ,CARGO_HOME=str(RUST/'.cargo-home'),GMP_MPFR_SYS_CACHE=str(RUST/'target/gmp-cache'),LC_ALL='C')
FIELDS=json.loads((HERE/'fields.json').read_text())['fields']
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def load(p):return json.loads(p.read_text())
def save(p,v):p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(v,indent=2,sort_keys=True)+'\n')
spec=importlib.util.spec_from_file_location('o_compare',ROOT/'validation/parameter_storage_port/run.py');o=importlib.util.module_from_spec(spec);spec.loader.exec_module(o)
compare=o.compare

def number_bytes(b):
 bits=int.from_bytes(b,'little');fracbits,expbits,bias={8:(52,11,1023),16:(112,15,16383)}[len(b)]
 sign=bool(bits>>(fracbits+expbits));exponent=(bits>>fracbits)&((1<<expbits)-1);mantissa=bits&((1<<fracbits)-1)
 if exponent==(1<<expbits)-1:return dict(**{'class':'NaN' if mantissa else '-Infinity' if sign else '+Infinity'})
 val=Fraction((1<<fracbits)+mantissa if exponent else mantissa)*Fraction(2)**((exponent or 1)-bias-fracbits)
 if sign:val=-val
 return {'class':'finite','exact':str(val),'negative':sign}

def norm(v):
 if isinstance(v,dict):
  if 'class' in v:
   if v['class']!='finite':
    assert set(v)=={'class'} and v['class'] in ['NaN','+Infinity','-Infinity'];return v
   if 'bytes_le' in v:
    assert set(v)=={'class','bytes_le','bits'} and len(v['bytes_le'])*4==v['bits'];return number_bytes(bytes.fromhex(v['bytes_le']))
   assert set(v)=={'class','integer','exponent','negative'}
   assert type(v['negative']) is bool
   n=Fraction(int(v['integer']));n=n*Fraction(2)**v['exponent'] if n else n;assert not n or (n<0)==v['negative']
   return {'class':'finite','exact':str(n),'negative':v['negative']}
  return {k:norm(x) for k,x in v.items()}
 if isinstance(v,list):return [norm(x) for x in v]
 return v

def m_expected():
 def field(f,key):
  b=bytearray(f['bytes'])
  for offset,h in f[key]:b[offset:offset+len(bytes.fromhex(h))]=bytes.fromhex(h)
  spec=next(x for x in FIELDS if x['name']==f['field']);typ=spec['source_type'];dims=spec['extents']
  size={'longdouble':16,'double':8,'int':4,'long long':8,'char':1,'clock_correction':40}[typ]
  def cell(data):
   if typ in ['longdouble','double']:return number_bytes(data)
   if typ=='char':return data[0]
   if typ=='long long':return str(int.from_bytes(data,'little',signed=True))
   if typ=='int':return int.from_bytes(data,'little',signed=True)
   assert len(data)==40;return dict(correction=number_bytes(data[:8]),corrects_to=list(data[8:]))
  def nested(data,ds):
   if not ds:return cell(data)
   step=len(data)//ds[0];assert step*ds[0]==len(data)
   return [nested(data[i*step:(i+1)*step],ds[1:]) for i in range(ds[0])]
  assert len(b)==size*__import__('math').prod(dims)
  return dict(name=spec['name'],type=typ,dimensions=dims,value=nested(b,dims))
 cases=[]
 for c in load(HERE/'m_projection.json'):
  assert c['source_unchanged'] is True
  source=[[field(f,'source') for f in obs] for obs in c['observations']]
  cases.append(dict(id=c['id'],source_before=source,source_after=source,destination_before=[[field(f,'before') for f in obs] for obs in c['observations']],destination_after=[[field(f,'after') for f in obs] for obs in c['observations']]))
 return cases

def blank():
 def value(f):
  v={'correction':None,'corrects_to':[None]*32} if f['source_type']=='clock_correction' else None
  for d in reversed(f['extents']):v=[copy.deepcopy(v) for _ in range(d)]
  return v
 return [dict(name=f['name'],type=f['source_type'],dimensions=f['extents'],value=value(f)) for f in FIELDS]
def get(s,name):return next(x for x in s if x['name']==name)['value']
def put(s,name,v):next(x for x in s if x['name']==name)['value']=v

def expected_boundaries():
 num=lambda v:{'class':'finite','exact':str(Fraction(v)),'negative':Fraction(v)<0}
 out=[];s=blank();get(s,'fname')[3]=122
 out.append(dict(case='isolated_byte',before=copy.deepcopy(s),after=copy.deepcopy(s),error='observation_unwritten_fname[0]'))
 get(s,'fname')[:3]=[97,0,None];b=copy.deepcopy(s);get(s,'fname')[3]=120
 out.append(dict(case='through_nul_and_holes',before=b,after=s))
 out.append(dict(case='write_capacity',after=[97,98,99],error='observation_capacity_fname[3]'))
 out.append(dict(case='missing_nul',after=[97,98,99],error='observation_missing_terminator_fname'))
 s=blank();put(s,'nclock_correction',0);out.append(dict(case='identity_without_sat_site',after=copy.deepcopy(s)))
 put(s,'nclock_correction',-7);out.append(dict(case='negative_count',after=copy.deepcopy(s)))
 put(s,'nclock_correction',1);out.append(dict(case='unwritten_active_correction',after=copy.deepcopy(s),error='observation_unwritten_correctionsTT[0].correction'))
 put(s,'nclock_correction',31)
 for c in get(s,'correctionsTT'):c['correction']=num(0)
 out.append(dict(case='correction_capacity',after=s,error='clock_correction_storage'))
 s=blank();put(s,'clockCorr',2);out.append(dict(case='disabled_clock_no_jumps',after=s))
 for mode in ['hole','no_nul','early_nul','jump_capacity','jump_unwritten','negative_jumps']:
  s=blank();put(s,'clockCorr',0);put(s,'obsNjump',1);put(s,'nclock_correction',0);get(s,'jump')[0]=1
  c=get(s,'correctionsTT');c[0]['correction']=num('9.5');c[0]['corrects_to'][0]=97;c[1]['corrects_to'][31]=222
  if mode=='no_nul':c[0]['corrects_to'][:31]=[120]*31
  elif mode=='early_nul':c[0]['corrects_to'][1]=0
  elif mode=='jump_capacity':put(s,'obsNjump',41);put(s,'jump',[0]*40)
  elif mode=='jump_unwritten':get(s,'jump')[0]=None
  elif mode=='negative_jumps':put(s,'obsNjump',-7)
  before=copy.deepcopy(s);error=None
  if mode in ['hole','no_nul','early_nul']:
   put(s,'nclock_correction',1);c[1]['correction']=num('9.5')
   if mode=='hole':c[1]['corrects_to'][0]=97;error='observation_unwritten_correctionsTT[0].corrects_to[1]'
   else:
    c[1]['corrects_to'][:31]=[120]*31 if mode=='no_nul' else [97]+[0]*30
    c[0]['correction']=num('2.5');c[0]['corrects_to'][:9]=list(b'SATJMP x\0')
  elif mode=='jump_capacity':error='clock_jump_storage'
  elif mode=='jump_unwritten':error='observation_unwritten_jump[0]'
  out.append(dict(case=mode,before=before,after=s,error=error,terminal=error is not None))
 return out

def protected():
 frozen=load(HERE/'protected_hashes.json');assert {p:sha(ROOT/p) for p in frozen}==frozen,'protected evidence changed'
 for repo in ['tempo2','validation/reference']:assert subprocess.check_output(['git','-C',str(ROOT/repo),'status','--porcelain'])==b''
 return frozen

def identity(exe=None):
 paths=[*RUST.joinpath('src').glob('*.rs'),*RUST.joinpath('examples').glob('*.rs'),*RUST.joinpath('tests').rglob('*.rs'),*HERE.glob('*.py'),*HERE.glob('*.json'),HERE/'probe.cc',RUST/'Cargo.toml',RUST/'Cargo.lock',RUST/'build.rs',ROOT/'validation/parameter_storage_port/run.py']
 rustc=Path(subprocess.check_output(['rustup','which','rustc'],text=True).strip());cargo=Path(subprocess.check_output(['rustup','which','cargo'],text=True).strip())
 return dict(**({'executable_sha256':sha(exe)} if exe else {}),compiler_sha256=sha(rustc),cargo_sha256=sha(cargo),toolchain=subprocess.check_output(['rustc','-vV'],text=True),sources={str(p.relative_to(ROOT)):sha(p) for p in sorted(paths)})
def validate_identity(a,b):assert a==b,'execution identity mismatch'
def execute(exe,expected,plan):
 validate_identity(expected,identity(exe))
 with tempfile.TemporaryDirectory(prefix='observationR-',dir='/private/tmp') as td:
  inp=Path(td)/'input.json';out=Path(td)/'output.json';save(inp,plan);token=secrets.token_hex(16)
  cmd=[str(exe),'--exact','observation_storage_tests::probe','--ignored','--nocapture']
  p=subprocess.run(cmd,cwd=RUST,env=dict(ENV,OBSERVATION_INPUT=str(inp),OBSERVATION_OUTPUT=str(out),OBSERVATION_TOKEN=token),capture_output=True,text=True,timeout=60)
  assert p.returncode==0 and '1 passed; 0 failed' in p.stdout and 'test observation_storage_tests::probe ... ok' in p.stdout,(p.returncode,p.stdout,p.stderr)
  raw=load(out);check_launch(raw,token,sha(inp));validate_identity(expected,identity(exe))
  return raw,dict(command=cmd,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr,token=token,input_sha256=sha(inp),output_sha256=sha(out))
def check_launch(raw,token,inp):assert raw['token']==token and raw['input_sha256']==inp,'stale launch'

def validate(raw,native):
 assert set(raw)=={'token','input_sha256','cases','m_cases','unwritten','boundaries'}
 assert set(native)=={'token','abi','signed_types','cases'}
 abi=[dict(name=f['name'],type=f['source_type'],dimensions=f['extents'],cell_bytes={'longdouble':16,'double':8,'int':4,'long long':8,'char':1,'clock_correction':40}[f['source_type']]) for f in FIELDS]
 counts=compare(native['abi'],abi);compare(native['signed_types'],{'int':True,'long long':True},counts=counts)
 compare(norm(raw['cases']),norm(native['cases']),counts=counts)
 compare(norm(raw['m_cases']),m_expected(),counts=counts)
 compare(raw['unwritten'],blank(),counts=counts)
 compare(norm(raw['boundaries']),expected_boundaries(),counts=counts)
 # Independently fixed ordered identities; native caller cannot silently drop a case.
 assert [c['case'] for c in native['cases']]==[0,1,2]
 for c in native['cases']:
  assert c['independent'] is True
  for key in ['source_before','source_after','destination_before','destination_after']:
   assert [{k:f[k] for k in ['name','type','dimensions']} for f in c[key]]==[{k:f[k] for k in ['name','type','dimensions']} for f in blank()]
 return counts

def main():
 parser=argparse.ArgumentParser();parser.add_argument('--profile',choices=['debug','release'],required=True);args=parser.parse_args();OUT.mkdir(parents=True,exist_ok=True)
 receipt=OUT/(args.profile+'_direct.json');receipt.unlink(missing_ok=True);frozen=protected();start=identity()
 cmd=['cargo','test','--lib','--locked','--offline','--no-run','--message-format=json']+(['--release'] if args.profile=='release' else [])
 p=subprocess.run(cmd,cwd=RUST,env=ENV,capture_output=True,text=True);save(OUT/(args.profile+'_build.json'),dict(command=cmd,returncode=p.returncode,stderr=p.stderr));assert p.returncode==0,p.stderr
 artifacts=[json.loads(l) for l in p.stdout.splitlines() if l.startswith('{')];exe=Path(next(x['executable'] for x in artifacts if x.get('executable') and x.get('target',{}).get('kind')==['lib']))
 ident=identity(exe);validate_identity(start,{k:v for k,v in ident.items() if k!='executable_sha256'})
 import native
 nexe,nid=native.build(args.profile);native.validate(nid);token=secrets.token_hex(16)
 with tempfile.TemporaryDirectory(prefix='observation-nativeR-',dir='/private/tmp') as td:
  dest=Path(td)/'output.json';p=subprocess.run([str(nexe),str(dest),token],env=ENV,capture_output=True,text=True,timeout=60);assert p.returncode==0,p.stderr
  nr=load(dest);assert nr['token']==token;native.validate(nid)
 raw,launch=execute(exe,ident,dict(schema=1,cases=[0,1,2],m_cases=[f'copypsr_{i}' for i in range(4)]));counts=validate(raw,nr)
 import controls
 results=controls.run(raw,nr,ident,exe,nid)
 validate_identity(ident,identity(exe));native.validate(nid);assert frozen==protected()
 names=[]
 for name,data in [(args.profile+'_raw.json.gz',raw),(args.profile+'_native.json.gz',nr)]:
  (OUT/name).write_bytes(gzip.compress(json.dumps(data,separators=(',',':')).encode(),mtime=0));names.append(name)
 save(receipt,dict(status='PASS',profile=args.profile,counts=counts,fields=95,cases=3,m_cases=4,boundaries=len(raw['boundaries']),controls=results,identity=ident,executable=str(exe),launch=launch,native=nid,host=platform.platform(),artifacts={n:sha(OUT/n) for n in names}))
 print(args.profile,counts,len(results),'controls PASS',flush=True)
if __name__=='__main__':main()
