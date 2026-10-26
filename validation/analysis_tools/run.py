"""AG replay into a fresh directory, strict numerical/shape gates and identities."""
from pathlib import Path
import argparse,hashlib,json,subprocess,sys,struct,math
from compare import compare,read
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
def run(profile,native,original,dest):
 dest=Path(dest).resolve();dest.mkdir(parents=True,exist_ok=False);native=Path(native).resolve();original=Path(original).resolve();examples=ROOT/'rust/target'/profile/'examples';records=[];commands=[]
 def call(name,args,cwd=None):
  args=list(map(str,args));r=subprocess.run(args,cwd=cwd or ROOT,capture_output=True,timeout=240);(dest/(name+'.stdout')).write_bytes(r.stdout);(dest/(name+'.stderr')).write_bytes(r.stderr);commands.append({'name':name,'args':args,'cwd':str(cwd or ROOT),'status':r.returncode});assert r.returncode==0,(name,r.returncode,r.stderr[-1500:]);return r
 def pair(name,mode,example,nargs=(),rargs=()):
  a=dest/(name+'-native.tsv');b=dest/(name+'-rust.tsv');r=call(name+'-native',[native,mode,a,*nargs]);s=call(name+'-rust',[examples/example,*rargs]);b.write_bytes(s.stdout)
  if mode=='components-gw':
   warnings=r.stdout.decode().count('Warning: distribution computed from harmonic file has negative probabilities');a.write_text(a.read_text()+f'population.warnings\texact\t{warnings}\n')
  result=compare(a,b);records.append({'case':name,**result});return a,b
 pair('toolkit','toolkit','toolkit')
 for c in ['spectrum','sigma','gw']:pair('components-'+c,'components-'+c,'analysis_components',rargs=['--case',c])
 par=ROOT/'validation/general_fitting/inputs/robust-H.par';tim=ROOT/'validation/general_fitting/inputs/multi-frequency.tim';runtime=ROOT/'validation/reference/build/source/T2runtime';common=['--par',par,'--tim',tim,'--runtime',runtime]
 for est,pre in [(1,0),(2,0),(3,0),(4,1),(5,0)]:pair(f'actual-spectrum-{est}','spectrum','spectrum',[par,tim,runtime,est,1,pre,-1,.01],common+['--estimator',est,'--prewhite',pre])
 for sampling in [0,1]:pair(f'actual-gw-{sampling}','gw','gw_simulation',[par,tim,runtime,dest/f'native-{sampling}',sampling],common+['--loglin',sampling,'--output',dest/f'rust-{sampling}'])
 base=ROOT/'validation/timing_models/fixtures/library'
 for control in ['eccentric.par','eccentric-coalescence.par']:
  pair(control,'eccentric','analysis_eccentric',[base/'base.par',base/'base.tim',runtime,base/control],['--par',base/'base.par','--tim',base/'base.tim','--runtime',runtime,'--control',base/control])
 # Actual native scalar classes, both native -> Rust -> native and independently
 # generated Rust populations -> native. The parser must retain every prefix bit.
 call('wire-generate',[native,'wire-generate',dest/'generate.tsv',dest/'special'])
 for source in ['special','rust-0','rust-1']:
  a=dest/(source+'-wire-native.tsv');b=dest/(source+'-wire-rust.tsv')
  call(source+'-wire-native',[native,'wire',a,dest/source,dest/(source+'-native-copy')]);r=call(source+'-wire-rust',[examples/'analysis_wire','--input',dest/source,'--output',dest/(source+'-rust-copy')]);b.write_bytes(r.stdout);records.append({'case':source+'-wire',**compare(a,b)})
  for kind in ['tensor','general']:
   expected=(dest/f'{source}.{kind}.bin').read_bytes()
   assert expected==(dest/f'{source}-rust-copy.{kind}.bin').read_bytes()==(dest/f'{source}-native-copy.{kind}.bin').read_bytes()
  call(source+'-wire-back',[native,'wire',dest/(source+'-back.tsv'),dest/(source+'-rust-copy'),dest/(source+'-back-copy')]);assert read(a)==read(dest/(source+'-back.tsv'))
 # Preserve raw and corrected demonstrated defects. Never execute undefined
 # one-row covariance backing or uninitialized exact-alignment imaginary values.
 for name,exe in [('original',original),('corrected',native)]:
  call('bugs-'+name,[exe,'bugs',dest/('bugs-'+name+'.tsv')])
  for sentinel in [0,17]:
   cwd=dest/f'diagnostic-{name}-{sentinel}';cwd.mkdir();call(cwd.name,[exe,'diagnostic',cwd/'trace.tsv',sentinel],cwd)
 cwd=dest/'diagnostic-rust';cwd.mkdir();r=call('diagnostic-rust',[examples/'analysis_diagnostics','--output',cwd]);(cwd/'trace.tsv').write_bytes(r.stdout)
 for sentinel in [0,17]:
  a=dest/f'diagnostic-corrected-{sentinel}';records.append({'case':'diagnostic-'+str(sentinel),**compare(a/'trace.tsv',cwd/'trace.tsv')})
  files={p.name for p in cwd.iterdir()}-{'trace.tsv'};assert files==({p.name for p in a.iterdir()}-{'trace.tsv'})
  for f in files:assert (cwd/f).read_bytes()==(a/f).read_bytes(),f
 assert (dest/'diagnostic-original-0/prefit.res').read_bytes()!=(dest/'diagnostic-original-17/prefit.res').read_bytes()
 raw=read(dest/'bugs-original.tsv');fixed=read(dest/'bugs-corrected.tsv')
 assert abs(float(fixed['elimination.cos'][1])-3)<1e-14 and abs(float(raw['elimination.cos'][1])-3)>.1
 for i in range(3):assert fixed[f'sort.reversed.{i}']==fixed[f'sort.direct.{i}']
 assert raw['sort.reversed.0']!=raw['sort.direct.0']
 assert raw['linear.0.plus'][1]=='1' and raw['linear.7.cross'][1]=='7'
 assert all(float(v)==0 for k,(u,v) in fixed.items() if k.startswith('linear.'))
 # Exact-width min/max rejection, plus required membership/unit/state/family gates.
 controls=[]
 def reject(name,reference,rows):
  p=dest/(name+'.tsv');p.write_text(''.join(f'{k}\t{u}\t{v}\n'for k,(u,v)in rows.items()));r=compare(reference,p);assert r['failures'],name;controls.append({'name':name,'failures':r['failures']})
 ref=dest/'toolkit-native.tsv';baseline=read(dest/'toolkit-rust.tsv')
 for key in ['stat.min','stat.max','float.min','float.max']:
  rows=dict(baseline);u,v=rows[key];x=float(v)
  if key.startswith('float.'):
   bits=struct.unpack('I',struct.pack('f',x))[0];x=struct.unpack('f',struct.pack('I',bits+(1 if x>=0 else -1)))[0]
  else:x=math.nextafter(x,math.inf)
  rows[key]=(u,repr(x));reject('mutant-'+key,ref,rows)
 for key in ['rng.25.draws','state.0','sort.x.0']:
  rows=dict(baseline);u,v=rows[key];rows[key]=(u,str(int(v)+1));reject('mutant-'+key,ref,rows)
 for key,change in [('driver.4.1.0.count','count'),('driver.2.1.0.x.0','bin'),('driver.4.1.0.y.0','unit')]:
  ref=dest/'components-spectrum-native.tsv';rows=read(dest/'components-spectrum-rust.tsv');u,v=rows[key];rows[key]=(u,str(float(v)+1))if change!='unit' else('s',v);reject('mutant-'+change,ref,rows)
 ref=dest/'components-gw-native.tsv';rows=read(dest/'components-gw-rust.tsv');rows['population.0.0.9']=('amplitude','1');reject('mutant-polarization',ref,rows)
 rows=read(dest/'components-gw-rust.tsv');del rows['population.0.0.8'];reject('mutant-membership',ref,rows)
 receipt={'profile':profile,'records':records,'negative_controls':controls,'commands':commands,'source_revision':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT).decode().strip(),'native':sha(native),'original_native':sha(original),'rust_executables':{p.name:sha(p)for p in examples.iterdir()if p.is_file()and p.name in ['toolkit','spectrum','gw_simulation','analysis_components','analysis_eccentric','analysis_diagnostics','analysis_wire']},'authored':{str(p.relative_to(ROOT)):sha(p)for p in HERE.iterdir()if p.is_file()},'inputs':{str(p.relative_to(ROOT)):sha(p)for p in [par,tim,base/'base.par',base/'base.tim',base/'eccentric.par',base/'eccentric-coalescence.par']},'artifacts':{str(p.relative_to(dest)):sha(p)for p in dest.rglob('*')if p.is_file()}}
 (dest/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');failures=sum(len(r['failures'])for r in records);print(profile,sum(r['checks']for r in records),'checks',failures,'failures',len(controls),'rejection controls');return bool(failures)
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--profile',choices=['debug','release'],required=True);p.add_argument('--native',required=True);p.add_argument('--original',required=True);p.add_argument('--output',required=True);a=p.parse_args();sys.exit(run(a.profile,a.native,a.original,a.output))
