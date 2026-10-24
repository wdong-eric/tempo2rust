"""Y's fresh Rust executions and exact projections of complete V membership."""
import argparse,gzip,hashlib,json,os,secrets,shutil,subprocess,sys,time
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];RUST=ROOT/'rust';V=ROOT/'validation/time_ephemeris';OUT=HERE/'outputs'
sys.path.insert(0,str(V))
import fixtures
import profile as native_profile
import run as vrun
# When this is __main__, importing run above finds V (first sys.path entry).
ENV=dict(os.environ,CARGO_HOME=str(RUST/'.cargo-home'),GMP_MPFR_SYS_CACHE=str(RUST/'target/gmp-cache'),LC_ALL='C',LANG='C',RUST_TEST_THREADS='1')
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def save(p,x):p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(x,indent=2)+'\n')
def unpack(p):return json.loads(gzip.decompress(p.read_bytes()))
def pack(p,x):p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(gzip.compress(json.dumps(x,separators=(',',':')).encode(),mtime=0))
def inputs(case):
 work=HERE/'work/x'/case['id']
 assert len(str(work))==len(str(V/'build/cases'/case['id'])),'source filename storage/path-length contract'
 if work.exists():shutil.rmtree(work)
 work.mkdir(parents=True)
 for n,b in case['files'].items():p=work/n;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(b)
 actions=[[str(work/str(v)[1:]) if str(v).startswith('@') else str(v) for v in row] for row in case['actions']]
 return work,actions

def native_launch(exe,profile,case):
 native_profile.check_live(exe,profile)
 work,actions=inputs(case);inp=work/'actions.txt';inp.write_text('\n'.join('\t'.join('x'+a.encode().hex() for a in row) for row in actions)+'\n')
 ih=sha(inp);nonce=secrets.token_hex(16);events=work/'events.jsonl';cmd=[str(exe),str(inp),str(events),nonce]
 env=dict(native_profile.ENV,TEMPO2=str(work/'unset-root'))
 result=subprocess.run(cmd,capture_output=True,env=env,timeout=60)
 raw=dict(id=case['id'],nonce=nonce,command=cmd,input_sha256=ih,input_text=inp.read_text(),resource_sha256={n:hashlib.sha256(b).hexdigest() for n,b in case['files'].items()},executable_sha256=sha(exe),events=events.read_text(),stdout=result.stdout.hex(),stderr=result.stderr.hex(),status=result.returncode,final_files={n:sha(work/n) for n in case['files']})
 assert sha(inp)==ih;native_profile.check_live(exe,profile)
 assert raw['status']==case['status'];return raw

def identity(exe=None):
 paths=[*RUST.joinpath('src').glob('*.rs'),*RUST.joinpath('tests').rglob('*.rs'),*RUST.joinpath('examples').glob('*.rs'),*RUST.joinpath('native').glob('*'),*HERE.glob('*.py'),*HERE.glob('*.md'),RUST/'Cargo.toml',RUST/'Cargo.lock',RUST/'build.rs']
 compiler=Path(subprocess.check_output(['rustup','which','rustc'],text=True).strip())
 return dict(sources={str(p.relative_to(ROOT)):sha(p) for p in sorted(paths) if p.is_file()},compiler=str(compiler),compiler_sha256=sha(compiler),**({'executable':str(exe),'executable_sha256':sha(exe)} if exe else {}))
def build(profile,dest=None):
 cmd=['cargo','test','--lib','--locked','--offline','--no-run','--message-format=json']+(['--release'] if profile=='release' else [])
 r=subprocess.run(cmd,cwd=RUST,env=ENV,capture_output=True,text=True);save((dest or OUT/profile)/'build.json',dict(command=cmd,status=r.returncode,stdout=r.stdout,stderr=r.stderr));assert r.returncode==0,r.stderr
 return Path(next(x['executable'] for line in r.stdout.splitlines() if line.startswith('{') and (x:=json.loads(line)).get('executable') and x['target']['kind']==['lib']))
def execute(case,exe,ident,dest):
 assert identity(exe)==ident,'execution identity'
 work,actions=inputs(case);dest.mkdir(parents=True,exist_ok=True);inp=dest/'input.json';nonce=secrets.token_hex(16);save(inp,dict(actions=actions,nonce=nonce));ih=sha(inp);event=dest/'events.jsonl';event.unlink(missing_ok=True)
 cmd=[str(exe),'time_ephemeris_tests::time_ephemeris_probe','--exact','--ignored','--nocapture']
 r=subprocess.run(cmd,cwd=RUST,env=dict(ENV,TEMPO2_Y_INPUT=str(inp),TEMPO2_Y_OUTPUT=str(event)),capture_output=True,text=True,timeout=60)
 save(dest/'launch.json',dict(command=cmd,status=r.returncode,stdout=r.stdout,stderr=r.stderr,identity=ident,input_sha256=ih,resources_before={n:hashlib.sha256(b).hexdigest() for n,b in case['files'].items()}))
 assert r.returncode==0,r.stdout+r.stderr
 events=[json.loads(l) for l in event.read_text().splitlines()];launch=events[0]
 assert launch['nonce']==nonce and launch['input_sha256']==ih and sha(inp)==ih,'launch challenge/input'
 outcome=Path(str(event)+'.outcome').read_text();status={'success':0,'source_exit':1,'boundary':2}[outcome]
 payload=dict(id=case['id'],events=events,stdout=Path(str(event)+'.stdout').read_bytes().hex(),stderr=Path(str(event)+'.stderr').read_bytes().hex(),status=status,final_files={n:sha(work/n) for n in case['files']})
 assert identity(exe)==ident;save(dest/'result.json',payload);return payload

def native_payload(raw):
 events=[json.loads(l) for l in raw['events'].splitlines()];assert events[0]['nonce']==raw['nonce']
 return {k:raw[k] for k in ['id','stdout','stderr','status','final_files']}|dict(events=events)

def project(payload):
 x=json.loads(json.dumps(payload));launch=x['events'][0]
 x['events'][0]={'phase':'launch','constants':launch['constants']}
 def visit(v):
  if isinstance(v,dict):
   # Redundant text formatting is projected away, not a numerical comparison:
   # typed IEEE bits retain finite values/signs, and every nonfinite bit pattern.
   if v.get('type')=='binary64':v.pop('hex',None)
   if v.get('type')=='binary128':v.pop('value',None)
   for z in v.values():visit(z)
  elif isinstance(v,list):
   for z in v:visit(z)
 visit(x)
 return x

def ownership_disposition(expected):
 x=json.loads(json.dumps(expected))
 case=x['id']
 if case in ('ifte_short','ifte_truncate','fb_before','fb_after'):
  event=x['events'][-1];assert event['phase']=='exit'
  name='ifte' if case.startswith('ifte') else 'fortran'
  state=event['state']
  for field in ('position','eof','error'):
   value=state.pop(name+'.'+field);assert type(value)is int
  assert state[name+'.owner'] in ('live','live-inside-FB')
  state[name+'.owner']='released-on-error'
 return x

def compare(a,b,path='$'):
 assert type(a)is type(b),(path,'type',type(a).__name__,type(b).__name__)
 if isinstance(a,dict):
  assert set(a)==set(b),(path,'membership',sorted(set(a)-set(b)),sorted(set(b)-set(a)))
  if path.endswith('.state'):assert list(a)==list(b),(path,'ordered fields')
  for k in a:compare(a[k],b[k],path+'.'+k)
 elif isinstance(a,list):
  assert len(a)==len(b),(path,'length',len(a),len(b))
  for i,(v,w)in enumerate(zip(a,b)):compare(v,w,path+f'[{i}]')
 else:assert a==b,(path,str(a)[:180],str(b)[:180])

def main():
 # Remove the selected profile's stale success before argument validation/build.
 pre=argparse.ArgumentParser(add_help=False);pre.add_argument('--profile',choices=['debug','release']);known,_=pre.parse_known_args()
 if known.profile:
  old=OUT/known.profile/'direct.json'
  if old.exists():save(OUT/known.profile/'history'/(sha(old)+'.json'),json.loads(old.read_text()));old.unlink()
  (OUT/known.profile/'controls.json').unlink(missing_ok=True)
 p=argparse.ArgumentParser();p.add_argument('--profile',choices=['debug','release'],required=True);p.add_argument('--case');p.add_argument('--continue',dest='keep',action='store_true');a=p.parse_args()
 from freeze import check
 protected=check()
 receipt=json.loads((OUT/'native/receipt.json').read_text());native_profile.check_live(Path(receipt['profile']['executable_path']),receipt['profile'])
 native_records={r['id']:r for r in receipt['records']}
 cases=fixtures.catalog()
 frozen=json.loads((V/'outputs/reference-corrected/oracle.json').read_text())
 compare(frozen['inputs']['cases'],fixtures.membership(cases),'accepted V membership')
 dest=OUT/a.profile/'runs'/secrets.token_hex(8);dest.mkdir(parents=True)
 exe=build(a.profile,dest);ident=identity(exe);failures=[];records=[]
 for case in cases:
  if a.case and a.case!=case['id']:continue
  expected_path=OUT/'native/equalpaths'/(case['id']+'.json.gz')
  assert sha(expected_path)==native_records[case['id']]['sha256'],'corrected native record identity'
  expected=ownership_disposition(project(native_payload(unpack(expected_path))))
  actual=project(execute(case,exe,ident,dest/case['id']))
  try:compare(expected,actual,case['id'])
  except AssertionError as e:
   failures.append(dict(id=case['id'],difference=str(e)));print('FAIL',case['id'],str(e),flush=True)
   if not a.keep:raise
  else:print('PASS',a.profile,case['id'],flush=True)
  records.append(dict(id=case['id'],events=len(actual['events']),status=actual['status'],result_sha256=sha(dest/case['id']/'result.json')))
 assert identity(exe)==ident and check()==protected
 report=dict(identity=ident,cases=records,failures=failures,complete=not a.case,run_directory=str(dest),native_receipt_sha256=sha(OUT/'native/receipt.json'),protected=protected)
 save(dest/'direct.json',report)
 if failures:raise SystemExit(1)
 if not a.case:
  import importlib.util
  spec=importlib.util.spec_from_file_location('y_controls',HERE/'controls.py');c=importlib.util.module_from_spec(spec);spec.loader.exec_module(c)
  c.checks(sys.modules[__name__],a.profile,exe,ident,cases,dest)
 save(OUT/a.profile/'direct.json',report)
if __name__=='__main__':main()
