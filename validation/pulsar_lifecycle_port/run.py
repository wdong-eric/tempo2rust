"""Exact challenged S replay. Outputs stay under T; accepted evidence is read-only."""
import argparse,re,gzip,hashlib,json,os,secrets,subprocess,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent;OUT=ROOT/'rust/verification/pulsar_lifecycle';RUST=ROOT/'rust'
ENV=dict(os.environ,CARGO_HOME=str(RUST/'.cargo-home'),GMP_MPFR_SYS_CACHE=str(RUST/'target/gmp-cache'),LC_ALL='C',RUST_TEST_THREADS='1')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def save(p,v):p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(v,indent=2)+'\n')
def identity(exe=None):
 paths=[*RUST.joinpath('src').glob('*.rs'),*RUST.joinpath('tests').rglob('*.rs'),*HERE.glob('*.py'),*HERE.glob('*.md'),*HERE.glob('*.cc'),RUST/'Cargo.toml',RUST/'Cargo.lock',RUST/'build.rs']
 compiler=Path(subprocess.check_output(['rustup','which','rustc'],text=True).strip());cargo=Path(subprocess.check_output(['rustup','which','cargo'],text=True).strip())
 return dict(compiler=str(compiler),compiler_sha256=sha(compiler),cargo=str(cargo),cargo_sha256=sha(cargo),sources={str(p.relative_to(ROOT)):sha(p) for p in sorted(paths)},**({'executable':str(exe),'executable_sha256':sha(exe)} if exe else {}))
def protected():
 frozen=json.loads((OUT/'freeze/protected.json').read_text())
 changed=[p for p,h in frozen.items() if sha(ROOT/p)!=h];assert not changed,changed
 return len(frozen)
def build(profile):
 cmd=['cargo','test','--lib','--locked','--offline','--no-run','--message-format=json']+(['--release'] if profile=='release' else [])
 p=subprocess.run(cmd,cwd=RUST,env=ENV,capture_output=True,text=True);save(OUT/profile/'build.json',dict(command=cmd,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr));assert p.returncode==0,p.stdout+p.stderr
 exe=Path(next(x['executable'] for l in p.stdout.splitlines() if l.startswith('{') and (x:=json.loads(l)).get('executable') and x.get('target',{}).get('kind')==['lib']))
 return exe
# Only source ABI and host logger presentation are outside typed state equality.
ABI_FIELDS={'longdouble_mantissa','pulsar_bytes','observation_bytes','size_t_bytes'}
def diagnostic_projection(s):
 s=re.sub(r'\[(?:\{source\}/)?(TKmatrix.C|initialise.C):',r'[{source}/\1:',s)
 rules={184:'owned allocation identity',192:'typed matrix metadata',194:'dense allocation range',203:'data owner release',208:'metadata owner release'}
 for line,body in rules.items():s=re.sub(r'(\[\{source\}/TKmatrix.C:'+str(line)+r'\]) [^\n]*',r'\1 '+body,s)
 return s
def project(e):return dict(phase=e['phase'],state=[r for r in e['state'] if e['phase']!='profile' or r['path'] not in ABI_FIELDS],stdout=diagnostic_projection(e.get('stdout','')),stderr=e.get('stderr',''))
def project_expected(expected):
 out=[];prior=0
 for e in expected['events']:
  current=e['stdout_position'];out.append(project(e|dict(stdout=expected['stdout'][prior:current])));prior=current
 assert prior==len(expected['stdout'])
 return out
def compare(a,b,path='$'):
 if a==b:return
 assert type(a)==type(b),(path,'type',type(a),type(b))
 if isinstance(a,dict):
  assert a.keys()==b.keys(),(path,'membership',a.keys()^b.keys())
  for k in a:compare(a[k],b[k],path+'.'+k)
 elif isinstance(a,list):
  assert len(a)==len(b),(path,'length',len(a),len(b))
  for i,(x,y) in enumerate(zip(a,b)):compare(x,y,path+f'[{i}]')
 else:raise AssertionError((path,str(a)[:150],str(b)[:150]))
def validate_launch(value,token,ih):
 assert value==dict(phase='launch',nonce=token,input_sha256=ih),'launch challenge/input identity'

def execute(case,exe,ident,dest):
 assert identity(exe)==ident,'stale source or executable'
 dest.mkdir(parents=True,exist_ok=True);inp=dest/'input.json';out=dest/'events.jsonl';out.unlink(missing_ok=True)
 token=secrets.token_hex(16);save(inp,dict(args=case['args'],nonce=token));ih=sha(inp)
 cmd=[str(exe),'pulsar_lifecycle_tests::lifecycle_probe','--exact','--ignored','--nocapture']
 start=time.monotonic();p=subprocess.run(cmd,cwd=RUST,env=dict(ENV,TEMPO2_T_INPUT=str(inp),TEMPO2_T_OUTPUT=str(out),TEMPO2_T_DIAGNOSTICS=str(dest/'diagnostics.json')),capture_output=True,text=True,timeout=300)
 save(dest/'launch.json',dict(command=cmd,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr,input_sha256=ih,elapsed_s=time.monotonic()-start,identity=ident))
 assert p.returncode==0,(case['id'],p.stdout,p.stderr)
 assert identity(exe)==ident and sha(inp)==ih,'execution identity changed'
 with out.open() as f:
  validate_launch(json.loads(next(f)),token,ih)
  actual=[json.loads(l) for l in f]
 expected=json.loads(gzip.decompress((ROOT/'validation/pulsar_lifecycle/expected'/(case['id']+'.json.gz')).read_bytes()))
 compare([project(e) for e in actual],project_expected(expected),case['id'])
 # Lossless local compression; retain every record and all 100 million zero cells.
 compressed=out.with_suffix('.jsonl.gz');compressed.write_bytes(gzip.compress(out.read_bytes(),mtime=0));out.unlink()
 return dict(id=case['id'],events=len(actual),records=sum(len(e['state']) for e in actual),expected_sha256=sha(ROOT/'validation/pulsar_lifecycle/expected'/(case['id']+'.json.gz')),output_sha256=sha(compressed),source_status=case['returncode'],rust_status=p.returncode)
def main():
 p=argparse.ArgumentParser();p.add_argument('--profile',choices=['debug','release'],required=True);p.add_argument('--case');a=p.parse_args()
 cases=json.loads((OUT/'freeze/membership.json').read_text())['S'];cases=[c for c in cases if not a.case or c['id']==a.case]
 assert cases
 before=protected();exe=build(a.profile);ident=identity(exe);results=[]
 for c in cases:
  results.append(execute(c,exe,ident,OUT/a.profile/c['id']));print(a.profile,c['id'],'PASS',flush=True)
 assert protected()==before
 controls=[]
 if not a.case:
  import controls as checks
  controls=checks.run(a.profile,exe,ident)
 save(OUT/a.profile/('direct.json' if not a.case else a.case+'_direct.json'),dict(status='PASS',identity=ident,cases=results,controls=controls,protected=before,source_projection=sorted(ABI_FIELDS),toolchain=subprocess.check_output(['rustc','-vV'],text=True)))
if __name__=='__main__':main()
