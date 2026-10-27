"""Build external callers, wrapping existing AJ/AF/AI consumers in AK scopes."""
from pathlib import Path
import hashlib,json,os,subprocess,sys,shutil
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
out=Path(sys.argv[1]).resolve();out.mkdir(parents=True,exist_ok=False)
crate=out/'client';bins=crate/'src/bin';bins.mkdir(parents=True)
manifest='''[package]
name="ak-external-callers"
version="0.1.0"
edition="2024"
[features]
default=[]
calceph=["tempo2-rust/calceph"]
blas=["tempo2-rust/blas"]
lapack=["blas","tempo2-rust/lapack"]
[dependencies]
'''+f'tempo2-rust={{path={json.dumps(str(ROOT/"rust"))}}}\n'+'''rug = { version = "=1.30.0", default-features = false, features = ["float", "integer", "std"] }
serde = { version = "=1.0.229", features = ["derive"] }
serde_json = "=1.0.151"
sha2 = "=0.10.9"
'''
(crate/'Cargo.toml').write_text(manifest)
shutil.copytree(ROOT/"rust/examples/analysis_support",bins/"analysis_support")
source={}
for name in ['cli.rs','concurrent.rs']:
 p=HERE/name;shutil.copy2(p,bins/name);source[str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
for p in [ROOT/'rust/examples'/n for n in ['parallel_execution.rs','predictor_workflows.rs','native_backends.rs','toolkit.rs']]+[ROOT/'validation/library_interfaces/core.rs']:
 raw=p.read_text();source[str(p)]=hashlib.sha256(raw.encode()).hexdigest()
 text='\n'.join(line for line in raw.splitlines() if not line.startswith('//!'))
 assert text.count('fn main()')==1
 text=text.replace('fn main()', 'fn source_main()',1)
 text+='''
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
 let workers = std::env::var("AK_WORKERS").unwrap_or_else(|_| "1".into()).parse()?;
 tempo2_rust::execution::with_workflow(|| tempo2_rust::execution::with_observation_workers(workers, source_main))
}
'''
 (bins/p.name).write_text(text)
commands=[]
for profile in ['debug','release']:
 cmd=['cargo','build','--offline','--manifest-path',str(crate/'Cargo.toml'),'--bins']+(['--release'] if profile=='release' else [])
 with (out/(profile+'-build.log')).open('wb') as log:r=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT,timeout=600)
 commands.append(dict(command=cmd,exit=r.returncode));assert r.returncode==0,(out,profile)
print(out,flush=True)
(out/'receipt.json').write_text(json.dumps(dict(source=source,commands=commands,manifest=manifest),indent=2)+'\n')
