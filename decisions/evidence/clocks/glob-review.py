import hashlib,json,os,shutil,subprocess,sys,tempfile
from pathlib import Path
root=Path(__file__).resolve().parents[3]
revision=sys.argv[1] if len(sys.argv)>1 else '189fed3526ce7a3b1a8c6725d72d041251306ccc'
with tempfile.TemporaryDirectory(prefix='tempo2-glob-review-',dir='/tmp') as td:
 t=Path(td).resolve(); tree=t/'tree'; tree.mkdir()
 for name in ['a','b','x','^','!','-','.', '.a','ab','a b','back\\slash','colon:']:
  p=tree/name;p.mkdir(exist_ok=True);(p/'a.clk').write_text('');(p/'.hidden.clk').write_text('')
 pin='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
 blob=subprocess.check_output(['git','-C',str(root/'tempo2'),'show',pin+':clkcorr.C']).decode()
 body=blob[blob.index('static std::vector<std::string> globClockFiles('):blob.index('static void ensureClockCorrectionsInitialized(int dispWarnings)')]
 (t/'native.cc').write_text('#include <glob.h>\n#include <cstdio>\n#include <cstdlib>\n#include <string>\n#include <vector>\n#include <algorithm>\n'+body+'\nint main(int argc,char**argv){for(const auto &p:globClockFiles(std::string(argv[1])+"/*.clk"))std::puts(p.c_str());}\n')
 for n in ['clock_glob.rs','clock_text.rs']:(t/n).write_bytes(subprocess.check_output(['git','show',revision+':rust/src/'+n],cwd=root))
 (t/'rust.rs').write_text('#[path="'+str(t/'clock_text.rs')+'"] mod clock_text;\n#[path="'+str(t/'clock_glob.rs')+'"] mod clock_glob;\nfn main(){let p=std::env::args_os().nth(1).unwrap();for f in clock_glob::files(std::path::Path::new(&p),b"clk"){println!("{}",f.display());}}\n')
 gcc=shutil.which('g++-16');assert gcc
 subprocess.run([gcc,'-O2','-std=gnu++11','-DHAVE_CONFIG_H',str(t/'native.cc'),'-o',str(t/'native')],check=True,capture_output=True)
 subprocess.run(['rustc','--edition=2024',str(t/'rust.rs'),'-o',str(t/'rust')],check=True,capture_output=True)
 rows=[]
 for suffix in ['a','*','?','[ab]','[!x]','[^x]','[[:alpha:]]','[=a=]','[[=a=]]','[[.a.]]','\\.a','.*','[.]a','back\\\\slash','a/','a//','./a','a/../b']:
  path=str(tree)+'/'+suffix
  c=subprocess.check_output([str(t/'native'),path],text=True,env=dict(os.environ,LC_ALL='C')).splitlines()
  r=subprocess.check_output([str(t/'rust'),path],text=True,env=dict(os.environ,LC_ALL='C')).splitlines()
  rows.append(dict(pattern=suffix,native=[s.replace(str(t),'{tmp}') for s in c],rust=[s.replace(str(t),'{tmp}') for s in r],equal=c==r))
 result={'source_pin':pin,'canonical_function_sha256':hashlib.sha256(body.encode()).hexdigest(),'compiler':subprocess.check_output([gcc,'--version'],text=True).splitlines()[0],'compiler_sha256':hashlib.sha256(Path(gcc).read_bytes()).hexdigest(),'rust_source_sha256':{n:hashlib.sha256((t/n).read_bytes()).hexdigest() for n in ['clock_glob.rs','clock_text.rs']},'scope':'Read-only raw canonical globClockFiles slice versus unchanged Rust modules, bounded regular temporary paths; no other native clock call or withheld experiment.','reviewed_commit':subprocess.check_output(['git','rev-parse',revision],cwd=root,text=True).strip(),'checks':rows}
 Path('/tmp/tempo2-clock-glob-review.json').write_text(json.dumps(result,indent=2)+'\n')
 print(json.dumps({'cases':len(rows),'mismatches':[r for r in rows if not r['equal']]},indent=2))
