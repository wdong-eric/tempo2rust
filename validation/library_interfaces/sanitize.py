#!/usr/bin/env python3
"""Instrument the installed C boundary; Rust/MPFR are not sanitizer-instrumented."""
from pathlib import Path
import argparse, hashlib, json, os, subprocess
p=argparse.ArgumentParser();p.add_argument('--prefix',type=Path,required=True);p.add_argument('--out',type=Path,required=True);a=p.parse_args()
prefix=a.prefix.resolve();out=a.out.resolve();out.mkdir(parents=True,exist_ok=False)
here=Path(__file__).resolve().parent;commands=[]
install=json.loads((prefix.parent/(prefix.name+'-build')/'install.json').read_text())
flags=['-fsanitize=address,undefined','-fno-omit-frame-pointer','-O1','-g','-std=c11','-I'+str(prefix/'include')]
def run(args,name,cwd=None):
 args=list(map(str,args));r=subprocess.run(args,cwd=cwd,capture_output=True,env={**os.environ,'UBSAN_OPTIONS':'halt_on_error=1:print_stacktrace=1'})
 commands.append({'command':args,'status':r.returncode});(out/(name+'.stdout')).write_bytes(r.stdout);(out/(name+'.stderr')).write_bytes(r.stderr)
 assert r.returncode==0,(name,r.stderr.decode(errors='replace'))
 assert b'ERROR: AddressSanitizer' not in r.stderr and b'runtime error:' not in r.stderr
run(['clang',*flags,'-c',prefix/'share/tempo2-rust/rust/native/predictor.c','-o',out/'predictor.o'],'boundary-build')
for name in ['consumer','storage','low_count','boundary']:
 run(['clang',*flags,here/(name+'.c'),out/'predictor.o',prefix/'lib/libtempo2pred_rust.a',*install['native_static_libs'],'-o',out/name],name+'-build')
 wd=out/(name+'-run');wd.mkdir();run([out/name,*([wd/'storage.tsv'] if name=='storage' else [])],name,cwd=wd)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
(out/'receipt.json').write_text(json.dumps({'scope':'C boundary ASan/UBSan only; uninstrumented Rust/provider dependencies','commands':commands,'script':sha(Path(__file__)),'source':{str(here/(n+'.c')):sha(here/(n+'.c')) for n in ['consumer','storage','low_count','boundary']},'artifacts':{str(p):sha(p) for p in [prefix/'lib/libtempo2pred_rust.a',prefix/'share/tempo2-rust/rust/native/predictor.c']},'evidence':{str(p.relative_to(out)):sha(p) for p in sorted(out.rglob('*')) if p.is_file()}},indent=2)+'\n')
print('Four installed C boundary clients passed ASan/UBSan')
