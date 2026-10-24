from pathlib import Path
import decimal, fractions, hashlib, importlib.util, json, os, re, shutil, subprocess, sys, tempfile, uuid
sys.dont_write_bytecode=True
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
HERE=ROOT/'validation/numeric_toolkit'
OUT=Path(tempfile.mkdtemp(prefix='tempo2-numeric-boundaries-'))
print('OUT',OUT,flush=True)
sha=lambda b:hashlib.sha256(b).hexdigest()
def save(path,value):path.write_text(json.dumps(value,indent=2,sort_keys=True)+'\n')
def protected():
 return {str(p.relative_to(ROOT)):sha(p.read_bytes()) for p in HERE.rglob('*') if p.is_file() and 'build' not in p.relative_to(HERE).parts and '__pycache__' not in p.parts}
frozen=protected()
spec=importlib.util.spec_from_file_location('numeric_profile',HERE/'profile.py');profile=importlib.util.module_from_spec(spec);spec.loader.exec_module(profile)
profile.BUILD=OUT/'native';profile.BUILD.mkdir()
exe,identity=profile.compile_probe('boundary_probe')
save(OUT/'native-profile.json',identity)
original=json.loads((HERE/'profile.json').read_text())
old_macros=(HERE/'build/probe-macros.txt').read_bytes()
new_macros=(profile.BUILD/'boundary_probe-macros.txt').read_bytes()
assert sorted(old_macros.splitlines())==sorted(new_macros.splitlines()),'native macro definitions changed'
old_stable=dict(original['stable']);new_stable=dict(identity['stable'])
old_stable.pop('macros_sha256');new_stable.pop('macros_sha256')
assert old_stable==new_stable,'ordinary native profile changed beyond macro emission order'
assert sha(exe.read_bytes())==identity['executable_sha256']
print('native built; stable profile matches M except raw macro order; complete macro definitions match',flush=True)
rows=[
 ('half_below','nlong',['0x1.ffffffffffffffffffffffffffffp-2']),
 ('negative_half_inside','nlong',['-0x1.ffffffffffffffffffffffffffffp-2']),
 ('half','nlong',['0.5']),('negative_half','nlong',['-0.5']),
 ('half_above','nlong',['0x1.0000000000000000000000000001p-1']),
 ('one_half','nlong',['1.5']),('negative_one_half','nlong',['-1.5']),
 ('integer_limit','nlong',['9223372036854775807']),
 ('mod_integer','mod128',['5.5','2']),('mod_negative_exact','mod128',['-4','2']),
 ('mod_negative_zero_divisor','mod128',['-0','-2']),
 ('mod_decimal','mod128',['0.3','0.1']),('mod_rounded_quotient','mod128',['1','0.1']),
 ('mod_negative_quotient','mod128',['-1','0.1']),
 ('mod_infinite_divisor','mod128',['1','inf']),('mod_zero_infinite_divisor','mod128',['0','inf'])]
actions=[['profile']];indexes=[]
for name,op,values in rows:
 starts=[]
 for val in values:starts.append(len(actions));actions.append(['parse',val])
 result=len(actions);actions.append([op,*values]);indexes.append((starts,result))
ap=OUT/'actions.tsv';ep=OUT/'native-events.jsonl';nonce=uuid.uuid4().hex
ap.write_text('\n'.join('\t'.join('x'+v.encode().hex() for v in a) for a in actions)+'\n')
with (OUT/'native-stdout.txt').open('w') as out,(OUT/'native-stderr.txt').open('w') as err:
 result=subprocess.run([str(exe),str(ap),str(ep),nonce],stdout=out,stderr=err,env=profile.ENV)
assert result.returncode==0,result.returncode
events=[json.loads(s) for s in ep.read_text().splitlines()]
assert events[0]=={'phase':'launch','nonce':nonce}
ended={e['action']:e['result'] for e in events if e['phase']=='end'}
assert len(ended)==len(actions)
assert ended[0]['measured_longdouble_mant']==113
assert sha(exe.read_bytes())==identity['executable_sha256']
def exact(v):
 if v['class']=='NaN':return 'NaN'
 if v['class']=='+Infinity':return 'inf'
 if v['class']=='-Infinity':return '-inf'
 assert v['type']=='binary128'
 bits=int.from_bytes(bytes.fromhex(v['bytes_le']),'little');negative=bool(bits>>127)
 exp=(bits>>112)&32767;mant=bits&((1<<112)-1)
 if exp:mant+=1<<112
 if mant==0:return '-0' if negative else '0'
 exp=(exp-16383 if exp else 1-16383)-112
 f=fractions.Fraction((-1 if negative else 1)*mant)*(fractions.Fraction(2)**exp)
 with decimal.localcontext() as ctx:
  ctx.prec=600
  return str(decimal.Decimal(f.numerator)/decimal.Decimal(f.denominator))
inputs=[];native=[]
for (name,op,values),(starts,index) in zip(rows,indexes):
 vals=[exact(ended[i]['value']) for i in starts]
 inputs.append('\t'.join([name,op,*vals]))
 native.append({'id':name,'op':op,'original_tokens':values,'exact_native_inputs':vals,'result':ended[index]})
(OUT/'rug-inputs.tsv').write_text('\n'.join(inputs)+'\n');save(OUT/'native-results.json',native)
base='04918d5'
code=subprocess.check_output(['git','show',base+':rust/src/tempo2_util.rs'],cwd=ROOT,text=True)
selected=[]
for name in ['fortran_mod','fortran_nlong']:
 m=re.search(r'pub\(crate\) fn '+name+r'\([\s\S]+?\n\}',code)
 assert m,name
 selected.append(m.group())
# These functions are extracted without modification from accepted R.
rust='use rug::{Float,float::{Round,Special}};\n'+'\n'.join(selected)+r'''
fn parse(precision:u32,s:&str)->Float {
 match s {"inf"=>Float::with_val(precision,Special::Infinity),"-inf"=>-Float::with_val(precision,Special::Infinity),"NaN"=>Float::with_val(precision,Special::Nan),_=>Float::with_val(precision,Float::parse(s).unwrap())}
}
fn emit(id:&str,p:u32,variant:&str,v:Float) {
 let class=if v.is_nan(){"NaN"}else if v.is_infinite(){"Infinity"}else{"finite"};
 let sign=v.is_sign_negative();
 let (n,e)=v.to_integer_exp().map(|(n,e)|(n.to_string(),if n==0 {0} else {e})).unwrap_or(("0".to_string(),0));
 println!("{}\t{}\t{}\t{}\t{}\t{}\t{}",id,p,variant,class,sign,n,e);
}
fn main() {
 let input=std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
 for line in input.lines(){let a:Vec<_>=line.split('\t').collect();
  for precision in [113,128]{let x=parse(precision,a[2]);
   if a[1]=="nlong" {emit(a[0],precision,"source_order",fortran_nlong(&x));}
   else {let y=parse(precision,a[3]);
    let q=Float::with_val(precision,&x/&y).to_integer_round(Round::Zero).unwrap().0;
    assert!(q.to_i32().is_some());
    let product=Float::with_val(precision,q)*&y;
    emit(a[0],precision,"source_order",x.clone()-product);
    emit(a[0],precision,"rug_remainder",x.clone()%&y);
    if precision==128 {emit(a[0],precision,"accepted_partial_helper",fortran_mod(x,y.to_f64()));}
   }
  }
 }
}
'''
rustfile=OUT/'rug-probe.rs';rustfile.write_text(rust)
deps=ROOT/'rust/target/debug/deps';rlibs=list(deps.glob('librug-*.rlib'));assert len(rlibs)==1
link_dependencies={str(p):sha(p.read_bytes()) for prefix in ['rug','gmp_mpfr_sys','az','libm','libc'] for p in deps.glob('lib'+prefix+'-*.rlib')}
rlib=rlibs[0];rlib_sha=sha(rlib.read_bytes());rustc=Path(subprocess.check_output(['rustup','which','rustc'],text=True).strip());rex=OUT/'rug-probe'
cmd=[str(rustc),'--crate-name','numeric_boundary_probe','--edition=2024',str(rustfile),'--extern','rug='+str(rlib),'-L','dependency='+str(deps),'-o',str(rex)]
r=subprocess.run(cmd,text=True,capture_output=True);(OUT/'rust-compile.log').write_text(r.stdout+r.stderr);assert r.returncode==0,r.stderr
r=subprocess.run([str(rex),str(OUT/'rug-inputs.tsv')],text=True,capture_output=True);assert r.returncode==0,r.stderr
(OUT/'rug-results.tsv').write_text(r.stdout)
results=[]
for line in r.stdout.splitlines():
 name,p,v,kind,sign,n,e=line.split('\t');results.append(dict(id=name,precision=int(p),variant=v,classification=kind,negative=sign=='true',integer=n,exponent=int(e)))
def normalized(v):
 if isinstance(v,int):return ('finite',v<0,fractions.Fraction(v))
 if 'bytes_le' in v:
  val=exact(v)
  return ('finite',val.startswith('-'),fractions.Fraction(val))
 return ('NaN',False,None) if v['class']=='NaN' else ('Infinity',v['class'].startswith('-'),None)
def normalized_rug(v):
 if v['classification']!='finite':return (v['classification'],False if v['classification']=='NaN' else v['negative'],None)
 n=int(v['integer'])
 return ('finite',v['negative'],fractions.Fraction(0) if n==0 else fractions.Fraction(n)*fractions.Fraction(2)**v['exponent'])
comparison=[]
for n in native:
 for r in results:
  if n['id']==r['id']:
   match=normalized(n['result'])==normalized_rug(r)
   comparison.append(dict(id=n['id'],precision=r['precision'],variant=r['variant'],matches_native=match,native_result=n['result'],rug=r))
assert all(x['matches_native'] for x in comparison if x['precision']==113 and x['variant']=='source_order')
assert protected()==frozen,'accepted M evidence changed'
assert sha(rlib.read_bytes())==rlib_sha,'Rug library changed during probe'
assert all(sha(Path(p).read_bytes())==h for p,h in link_dependencies.items()),'Rust dependency changed during probe'
assert identity['stable']['runner_sources']==profile.runners(),'M runners changed'
receipt=dict(rust_link_dependency_files=link_dependencies,native_measured_profile=ended[0],macro_projection=dict(reason='Complete identical macro definitions emitted in a different order',accepted_raw_sha256=sha(old_macros),fresh_raw_sha256=sha(new_macros),sorted_definitions_sha256=sha(b'\n'.join(sorted(old_macros.splitlines())))),scope='Disposable native and Rug arithmetic probe; no production edits or new gate',output=str(OUT),cases=len(rows),native_profile_sha256=sha((OUT/'native-profile.json').read_bytes()),native_executable_sha256=sha(exe.read_bytes()),accepted_rust_revision=base,accepted_source_sha256=sha(code.encode()),extracted_helpers_sha256=sha('\n'.join(selected).encode()),rust_probe_sha256=sha(rustfile.read_bytes()),rust_executable_sha256=sha(rex.read_bytes()),rustc_sha256=sha(rustc.read_bytes()),rustc_version=subprocess.check_output([str(rustc),'-vV'],text=True),rug_rlib_path=str(rlib),rug_rlib_sha256=rlib_sha,compile_command=cmd,protected_files=len(frozen),protected_sha256=sha(json.dumps(frozen,sort_keys=True).encode()),comparison=comparison)
save(OUT/'review.json',receipt)
print('PASS',len(rows),'native cases; all 113-bit source-order results match; accepted M files preserved',len(frozen),flush=True)
for x in comparison:
 if not x['matches_native']:
  r=x['rug'];print('DIFFERENCE',x['id'],x['precision'],x['variant'],'native=',x['native_result'],'rug=',r,flush=True)
print('RECEIPT',OUT/'review.json',sha((OUT/'review.json').read_bytes()),flush=True)
