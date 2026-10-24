"""Fresh challenged Rust replay of immutable U; ordinary verification never records."""
import argparse,gzip,hashlib,json,os,secrets,subprocess,sys
from pathlib import Path
from freeze import ROOT,HERE,check
OUT=ROOT/'rust/verification/terrestrial_resources';RUST=ROOT/'rust';U=ROOT/'validation/terrestrial_resources'
ENV=dict(os.environ,CARGO_HOME=str(RUST/'.cargo-home'),GMP_MPFR_SYS_CACHE=str(RUST/'target/gmp-cache'),LC_ALL='C',RUST_TEST_THREADS='1')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def save(p,x):p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(x,indent=2)+'\n')
def identity(exe=None):
 paths=sorted([*RUST.joinpath('src').glob('*.rs'),*RUST.joinpath('tests').rglob('*.rs'),*HERE.glob('*.py'),*HERE.glob('*.md'),RUST/'Cargo.toml',RUST/'Cargo.lock',RUST/'build.rs',*RUST.joinpath('native').glob('*')])
 compiler=Path(subprocess.check_output(['rustup','which','rustc'],text=True).strip())
 return dict(sources={str(p.relative_to(ROOT)):sha(p) for p in paths if p.is_file()},compiler=str(compiler),compiler_sha256=sha(compiler),**({'executable':str(exe),'executable_sha256':sha(exe)} if exe else {}))
def build(profile):
 cmd=['cargo','test','--lib','--locked','--offline','--no-run','--message-format=json']+(['--release'] if profile=='release' else [])
 p=subprocess.run(cmd,cwd=RUST,env=ENV,capture_output=True,text=True);save(OUT/profile/'build.json',dict(command=cmd,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr));assert p.returncode==0,p.stdout+p.stderr
 return Path(next(x['executable'] for l in p.stdout.splitlines() if l.startswith('{') and (x:=json.loads(l)).get('executable') and x['target']['kind']==['lib']))
def compare(a,b,path='$'):
 assert type(a) is type(b),(path,'type',type(a),type(b))
 if isinstance(a,dict):
  assert list(a)==list(b),(path,'ordered membership',list(a),list(b))
  for k in a:compare(a[k],b[k],path+'.'+k)
 elif isinstance(a,list):
  assert len(a)==len(b),(path,'length',len(a),len(b))
  for i,(x,y) in enumerate(zip(a,b)):compare(x,y,path+f'[{i}]')
 else:assert a==b,(path,str(a)[:200],str(b)[:200])
def project(x):
 # Only explicitly frozen native ABI profile cells; all state/types stay exact.
 x=json.loads(json.dumps(x))
 for e in x['events']:
  if e['phase']=='end' and e['operation']=='profile':
   for k in ['longdouble_size','longdouble_digits','observation_size','pulsar_size']:e['result'].pop(k)
 return x

def validate_launch(value,nonce,ih):
 compare(value,dict(phase='launch',nonce=nonce,input_sha256=ih),'launch')

def execute(case,exe,ident,dest):
 assert identity(exe)==ident,'stale source/compiler/executable'
 dest.mkdir(parents=True,exist_ok=True)
 # Match U's path byte lengths because TKlog truncates before path projection.
 original=U/'build/cases'/case['id']
 work=HERE/'build'/('x'*len('cases'))/case['id']
 # W directory name is five bytes longer; use a short symlink-free directory at
 # the workspace level of the W build root and pad to the precise original size.
 parent=OUT
 total=len(str(original))-len(case['id'])-1
 workparent=parent/('x'*(total-len(str(parent))-1))
 assert len(str(workparent))==total
 work=workparent/case['id']
 import shutil
 if work.exists():shutil.rmtree(work)
 work.mkdir(parents=True)
 for name,value in case['files'].items():
  p=work/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(value)
 resources={str(p.relative_to(work)):sha(p) for p in sorted(work.rglob('*')) if p.is_file()}
 actions=[[v.replace('{case}',str(work)) for v in a] for a in case['actions']]
 source=str(U/'build/ordinary/source')+'/'
 nonce=secrets.token_hex(16);inp=dest/'input.json';events=dest/'events.jsonl';events.unlink(missing_ok=True)
 save(inp,dict(actions=actions,nonce=nonce,source_directory=source));ih=sha(inp)
 cmd=[str(exe),'terrestrial_tests::terrestrial_probe','--exact','--ignored','--nocapture']
 p=subprocess.run(cmd,cwd=RUST,env=dict(ENV,TEMPO2_W_INPUT=str(inp),TEMPO2_W_OUTPUT=str(events),TEMPO2_W_STREAMS=str(dest/'streams')),capture_output=True,text=True,timeout=120)
 save(dest/'launch.json',dict(command=cmd,status=p.returncode,stdout=p.stdout,stderr=p.stderr,identity=ident,input_sha256=ih,resources_before=resources))
 assert p.returncode==0,(case['id'],p.stdout,p.stderr)
 assert identity(exe)==ident and sha(inp)==ih,'execution identity changed'
 def normalize(s):return s.replace(source[:-1],'{source}').replace(str(workparent),'{cases}').replace(str(U),'{terrestrial}')
 lines=events.read_text().splitlines();launch=json.loads(lines.pop(0));validate_launch(launch,nonce,ih)
 outcome=(dest/'streams.outcome').read_text();status=0 if outcome=='success' else 1 if outcome=='source_exit' else 2
 payload=dict(id=case['id'],classification=case['classification'],actions=case['actions'],status=status,events=[json.loads(normalize(l)) for l in lines],stdout=normalize((dest/'streams.stdout').read_bytes().decode('latin1')),stderr=normalize((dest/'streams.stderr').read_bytes().decode('latin1')))
 save(dest/'result.json',payload)
 save(dest/'receipt.json',dict(input_sha256=ih,library_outcome=outcome,test_process_status=p.returncode,resources_before=resources,resources_after={str(p.relative_to(work)):sha(p) for p in sorted(work.rglob('*')) if p.is_file()},raw={p.name:sha(p) for p in [events,dest/'streams.stdout',dest/'streams.stderr']},identity=ident))
 return payload

def main():
 p=argparse.ArgumentParser();p.add_argument('--profile',choices=['debug','release'],required=True);p.add_argument('--case');p.add_argument('--continue',dest='keep',action='store_true');p.add_argument('--development',action='store_true');a=p.parse_args()
 preservation_failure=None
 try:before=check()
 except AssertionError as e:
  if not a.development:raise
  preservation_failure=str(e);before=None;print('PRESERVATION_DRIFT',preservation_failure,flush=True)
 frozen=json.loads((HERE/'build/freeze.json').read_text());catalog=frozen['cases']
 oracle_path=OUT/'corrected_native/reference.json.gz'
 assert sha(oracle_path)=='6948f57785698ec65704652443f79146a26a92da6c4869a14d3b1e3b6eb0386e'
 assert sha(OUT/'corrected_native/contract.json')=='c4273b5fa79d3d2a748575a96d749a7ab90f60385995dc2339cb225906abf97a'
 corrected=json.loads((OUT/'corrected_native/contract.json').read_text())
 assert sha(U/'outputs/reference.json.gz')==corrected['original_sha256']
 for path,digest in corrected['profile']['dependencies'].items():assert sha(Path(path))==digest,('corrected native identity',path)
 assert sha(OUT/'fixed00/probe')==corrected['profile']['executable_sha256']
 oracle=json.loads(gzip.decompress(oracle_path.read_bytes()));compare([c['id'] for c in catalog],[c['id'] for c in oracle],'case membership')
 (OUT/a.profile/'direct.json').unlink(missing_ok=True)
 exe=build(a.profile);ident=identity(exe);results=[];failures=[]
 for case,expected in zip(catalog,oracle):
  if a.case and case['id']!=a.case:continue
  actual=execute(case,exe,ident,OUT/a.profile/case['id'])
  try:compare(project(expected),project(actual),case['id'])
  except AssertionError as e:
   failures.append(dict(id=case['id'],difference=str(e)));print('FAIL',case['id'],str(e),flush=True)
   if not a.keep:raise
  else:print('PASS',a.profile,case['id'],flush=True)
  results.append(dict(id=case['id'],events=len(actual['events']),status=actual['status']))
 if not a.case and not failures:
  import controls
  controls.checks(a.profile,exe,ident,catalog)
 if before is not None:assert check()==before
 save(OUT/a.profile/'direct.json',dict(status='PRESERVATION_DRIFT' if preservation_failure else 'FAIL' if failures else 'PASS',preservation_failure=preservation_failure,identity=ident,cases=results,failures=failures,protected=before,contract='W NMF_wet coefficient correction under 0053',oracle_sha256=sha(oracle_path)))
 assert not failures,failures
if __name__=='__main__':main()
