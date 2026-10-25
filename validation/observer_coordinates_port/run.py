"""AA Rust comparisons against accepted X recordings; never rerun or mutate X."""
import argparse,gzip,hashlib,json,os,secrets,shutil,subprocess,sys
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];RUST=ROOT/'rust';X=ROOT/'validation/observer_coordinates';OUT=HERE/'outputs'
sys.path.insert(0,str(X))
import fixtures
from run import compare as exact_compare
ENV=dict(os.environ,CARGO_HOME=str(RUST/'.cargo-home'),GMP_MPFR_SYS_CACHE=str(RUST/'target/gmp-cache'),LC_ALL='C',LANG='C',RUST_TEST_THREADS='1')
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def save(p,x):p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(x,indent=2)+'\n')
def unpack(p):return json.loads(gzip.decompress(p.read_bytes()))
def identity(exe=None):
 compiler=Path(subprocess.check_output(['rustup','which','rustc'],text=True).strip())
 paths=[*RUST.joinpath('src').glob('*.rs'),*RUST.joinpath('native/sofa').glob('*.for'),HERE/'run.py',HERE/'CONTRACT.md',RUST/'build.rs',RUST/'Cargo.lock',RUST/'Cargo.toml']
 return dict(sources={str(p.relative_to(ROOT)):sha(p) for p in sorted(paths)},compiler=str(compiler),compiler_sha256=sha(compiler),**({'executable':str(exe),'executable_sha256':sha(exe)} if exe else {}))
def build(profile):
 cmd=['cargo','test','--lib','--locked','--offline','--no-run','--message-format=json']+(['--release'] if profile=='release' else [])
 r=subprocess.run(cmd,cwd=RUST,env=ENV,capture_output=True,text=True);save(OUT/profile/'build.json',dict(command=cmd,status=r.returncode,stdout=r.stdout,stderr=r.stderr));assert r.returncode==0,r.stderr
 built=Path(next(x['executable'] for l in r.stdout.splitlines() if l.startswith('{') and (x:=json.loads(l)).get('executable') and x['target']['kind']==['lib']))
 retained=HERE/'build/binaries'/profile/sha(built);retained.parent.mkdir(parents=True,exist_ok=True)
 if not retained.exists():shutil.copy2(built,retained)
 assert sha(retained)==sha(built)
 return retained
def inputs(case):
 work=HERE/'work/x'/case['id']
 assert len(str(work))==len(str(X/'build/cases'/case['id']))
 if work.exists():shutil.rmtree(work)
 work.mkdir(parents=True)
 for n,b in case['files'].items():p=work/n;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(b)
 return work,[[v.replace('{case}',str(work)) for v in a] for a in case['actions']]
def execute(case,exe,ident,dest):
 assert identity(exe)==ident,'actual execution identity'
 work,actions=inputs(case);dest.mkdir(parents=True,exist_ok=True)
 inp=dest/'input.json';nonce=secrets.token_hex(16);save(inp,dict(actions=actions,nonce=nonce));ih=sha(inp);event=dest/'events.jsonl';event.unlink(missing_ok=True)
 cmd=[str(exe),'observer_coordinates_tests::observer_probe','--exact','--ignored','--nocapture']
 resources={n:sha(work/n) for n in case['files']}
 r=subprocess.run(cmd,cwd=RUST,env=dict(ENV,TEMPO2_AA_INPUT=str(inp),TEMPO2_AA_OUTPUT=str(event)),capture_output=True,text=True,timeout=120)
 save(dest/'launch.json',dict(command=cmd,status=r.returncode,stdout=r.stdout,stderr=r.stderr,input_sha256=ih,resources_before=resources,executable_sha256=sha(exe)))
 assert r.returncode==0,r.stdout+r.stderr
 launch=json.loads(Path(str(event)+'.launch').read_text());assert launch==dict(nonce=nonce,input_sha256=ih) and sha(inp)==ih
 def normalize(s):return s.replace(str(X/'build/ordinary/source'),'{source}').replace(str(HERE/'work/x'),'{cases}').replace(str(X),'{observer}')
 payload=dict(id=case['id'],classification=case.get('classification','defined-host-profile'),actions=case['actions'],status={'success':0,'source_exit':1,'boundary':2}[Path(str(event)+'.outcome').read_text()],events=[json.loads(normalize(l)) for l in event.read_text().splitlines()],stdout=normalize(Path(str(event)+'.stdout').read_bytes().decode('latin1')),stderr=normalize(Path(str(event)+'.stderr').read_bytes().decode('latin1')))
 assert identity(exe)==ident
 save(dest/'result.json',payload);return payload

def main():
 p=argparse.ArgumentParser();p.add_argument('--profile',choices=['debug','release'],required=True);p.add_argument('--case');a=p.parse_args()
 destination=OUT/a.profile;destination.mkdir(parents=True,exist_ok=True)
 receipt=destination/'direct.json'
 if receipt.exists():
  history=destination/'history'/sha(receipt);history.mkdir(parents=True,exist_ok=True);shutil.copy2(receipt,history/'direct.json')
 receipt.unlink(missing_ok=True)
 catalog=fixtures.cases();assert len(catalog)==133
 expected=unpack(X/'outputs/observer_snapshots.json.gz');ordinary=unpack(X/'outputs/reference.json.gz')
 exact_compare(catalog,json.loads((X/'outputs/frozen/cases.json').read_text()),'case membership')
 exe=build(a.profile);ident=identity(exe);rows=[]
 for c,e,o in zip(catalog,expected,ordinary):
  if a.case and a.case not in c['id']:continue
  actual=execute(c,exe,ident,destination/c['id'])
  try:
   exact_compare(e,actual,c['id']+'.private')
   projection=json.loads(json.dumps(actual));projection['events']=[v for v in projection['events'] if v['phase']!='exit']
   for v in projection['events']:
    if 'private' in v:v['private']=None
   exact_compare(o,projection,c['id']+'.ordinary');error=None
  except AssertionError as failure:error=str(failure)
  rows.append(dict(id=c['id'],status='FAIL' if error else 'PASS',error=error,events=len(actual['events']),result_sha256=sha(destination/c['id']/'result.json')))
  print(rows[-1]['status'],c['id'],error or '',flush=True)
 save(receipt,dict(status='FAIL' if any(r['error'] for r in rows) else 'PASS',profile=a.profile,identity=ident,oracle={n:sha(X/'outputs'/n) for n in ['reference.json.gz','observer_snapshots.json.gz','manifest.json']},cases=rows,scope='all' if not a.case else a.case))
 print('RESULT',len(rows),sum(r['error'] is None for r in rows),flush=True)
if __name__=='__main__':main()
