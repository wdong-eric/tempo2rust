import hashlib,importlib.util,json,shutil,struct,subprocess,sys,tempfile
from pathlib import Path
sys.dont_write_bytecode=True
root=Path(__file__).resolve().parents[3]
spec=importlib.util.spec_from_file_location('h',root/'validation/jpl/run.py');h=importlib.util.module_from_spec(spec);spec.loader.exec_module(h)
result={'scope':'Rust-only safe-header probe; source undefined branch classified by source inspection, not executed natively','source_commit':'fda6ad4a9753ad7dc5d291406f2f310a9161423f','rust_sha256':{},'cases':{}}
with tempfile.TemporaryDirectory(prefix='tempo2-jpl-legacy-',dir='/tmp') as d:
 d=Path(d)
 for n in ['stdio.rs','jpleph.rs']:
  b=(root/'rust/src'/n).read_bytes();(d/n).write_bytes(b);result['rust_sha256'][n]=hashlib.sha256(b).hexdigest()
 (d/'main.rs').write_text('''#![allow(dead_code)]\nmod stdio;mod jpleph;fn main(){let a=std::env::args().nth(1).unwrap();match jpleph::Ephemeris::init(std::fs::File::open(a),None,None){Ok(Some(e))=>println!("handle version={} ncoeff={} extended={:?}",e.ephemeris_version,e.ncoeff,&e.ipt[13..]),Ok(None)=>println!("no_handle {}",jpleph::jpl_init_error_code()),Err(x)=>println!("boundary {}",x.0)}}''')
 cmd=[shutil.which('rustc'),'--edition=2024','-O',str(d/'main.rs'),'-o',str(d/'probe')]
 p=subprocess.run(cmd,capture_output=True,text=True);assert p.returncode==0,p.stderr
 base,_=h.synthetic('<',version=405,count=6)
 for name in ['normal','unwritten_extended_comparison']:
  b=bytearray(base)
  if name!='normal':struct.pack_into('<3I',b,2844,0xffffffff,0,0)
  f=d/(name+'.bin');f.write_bytes(b)
  p=subprocess.run([d/'probe',f],capture_output=True,text=True);assert p.returncode==0,p.stderr
  result['cases'][name]={'sha256':hashlib.sha256(b).hexdigest(),'rust':p.stdout.strip()}
Path('/tmp/tempo2-jpl-legacy-header-after-review.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
