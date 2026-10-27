#!/usr/bin/env python3
"""Affected serial crate checks and source-map closure, in fresh owned evidence."""
from pathlib import Path
import argparse, hashlib, json, os, re, subprocess
p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True);a=p.parse_args()
root=Path(__file__).resolve().parents[2];out=a.out.resolve();out.mkdir(parents=True,exist_ok=False);records=[]
commands=[
 ('format',['cargo','fmt','--manifest-path',root/'rust/Cargo.toml','--check']),
 ('clippy',['cargo','clippy','--offline','--locked','--manifest-path',root/'rust/Cargo.toml','--all-targets','--','-D','warnings']),
 ('debug',['cargo','test','--offline','--locked','--manifest-path',root/'rust/Cargo.toml','--lib','--','--test-threads=1']),
 ('release',['cargo','test','--offline','--locked','--manifest-path',root/'rust/Cargo.toml','--lib','--release','--','--test-threads=1']),
 ('rustc',['rustc','-Vv']),('cargo',['cargo','-V']),('clang',['clang','--version']),('gcc',['/opt/homebrew/bin/gcc-16','--version']),('os',['sw_vers']),('arch',['uname','-m']),('sdk',['xcrun','--show-sdk-version'])]
for name,cmd in commands:
 cmd=list(map(str,cmd));r=subprocess.run(cmd,cwd=root,capture_output=True)
 (out/(name+'.stdout')).write_bytes(r.stdout);(out/(name+'.stderr')).write_bytes(r.stderr);records.append({'command':cmd,'status':r.returncode});assert r.returncode==0,(name,r.stderr.decode(errors='replace'));print(name,'passed',flush=True)
inventory=set(re.findall(r'^\| `([^`]+\.[cC])` \|',(root/'validation/coverage/SOURCE_INVENTORY.md').read_text(),re.M))
mapped=set(re.findall(r'`([^`]+\.[cC])`',(root/'rust/LIBRARY_INTERFACES_SOURCE_MAP.md').read_text()))
assert not inventory-mapped,sorted(inventory-mapped)
raw=subprocess.check_output(['git','-C',root/'tempo2','show','fda6ad4:tempo2pred.h'])
assert raw==(root/'rust/include/tempo2pred.h').read_bytes()
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
inputs=[root/'rust/Cargo.toml',root/'rust/Cargo.lock',root/'rust/build.rs']
for folder in ['src','native','include']:inputs.extend(p for p in (root/'rust'/folder).rglob('*') if p.is_file())
(out/'receipt.json').write_text(json.dumps({'commands':records,'mapped_units':len(inventory),'source':{str(p):sha(p) for p in inputs},'script':sha(Path(__file__)),'evidence':{p.name:sha(p) for p in out.iterdir() if p.is_file()}},indent=2)+'\n')
