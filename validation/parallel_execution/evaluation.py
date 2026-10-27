"""Serial standalone C predictor evaluation, including the unchanged Rust ABI bridge."""
from pathlib import Path
import datetime,hashlib,json,os,re,shlex,subprocess,sys
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
out=Path(sys.argv[1]).resolve();out.mkdir(parents=True,exist_ok=False);predictor=Path(sys.argv[2]).resolve()
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();records=[]
def run(label,cmd):
 cmd=list(map(str,cmd));started=datetime.datetime.now(datetime.timezone.utc).isoformat();r=subprocess.run(cmd,cwd=out,capture_output=True,timeout=120,env=dict(os.environ,LC_ALL='C'))
 (out/(label+'.stdout')).write_bytes(r.stdout);(out/(label+'.stderr')).write_bytes(r.stderr);records.append(dict(label=label,command=cmd,started_utc=started,exit=r.returncode));assert r.returncode==0,(label,r.stderr[-1800:]);return r.stdout
archive=ROOT/'validation/reference/build/libtempo2-reference.a';rust=ROOT/'rust/target/release/libtempo2_rust.a';shim=HERE/'build/c-concurrency-v2/plain/predictor.o'
run('compile',['clang','-std=c11','-O2','-I'+str(ROOT/'rust/include'),'-c',HERE/'evaluation.c','-o',out/'client.o'])
run('native-link',['/opt/homebrew/bin/g++-16',out/'client.o',archive,'-lquadmath','-lgfortran','-o',out/'native'])
libs=shlex.split(re.findall('native-static-libs: (.*)',(HERE/'build/native-static-libs.log').read_text())[-1])
run('rust-link',['clang',out/'client.o',shim,rust,*libs,'-o',out/'rust'])
values={};samples=[]
for sample in range(3):
 for engine in ['native','rust']:
  label=f'{engine}-{sample}';usage=out/(label+'.usage');raw=run(label,['/usr/bin/time','-l','-o',usage,out/engine,predictor,10000]).decode()
  m=re.fullmatch(r'calls (\d+) checksum (\S+) evaluate_s ([\d.]+)\n',raw);assert m,raw
  values[engine]=m[2];samples.append(dict(engine=engine,sample=sample,calls=int(m[1]),checksum=m[2],evaluate_s=float(m[3]),usage=usage.read_text()))
assert values['native']==values['rust'],values
(out/'receipt.json').write_text(json.dumps(dict(records=records,samples=samples,inputs={str(p):sha(p) for p in [Path(__file__),HERE/'evaluation.c',archive,rust,shim,predictor]},files={p.name:sha(p) for p in out.iterdir() if p.is_file()}),indent=2)+'\n')
print('Six standalone predictor evaluation measurements; exact aggregate checksums')
