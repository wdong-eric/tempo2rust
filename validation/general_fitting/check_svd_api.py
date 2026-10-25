"""Direct source TKsvd output and extended-RHS comparison; no sorted modes."""
from pathlib import Path
from decimal import Decimal as D, getcontext
import argparse, hashlib, json, subprocess
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
BUILD=HERE/'build/svd-api';REF=ROOT/'validation/reference/build'
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
getcontext().prec=80

def run(profile):
    BUILD.mkdir(parents=True,exist_ok=True);native=BUILD/'native'
    cmd=['/opt/homebrew/bin/g++-16','-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(REF/'source'),str(HERE/'svd_probe.cc'),str(REF/'libtempo2-reference.a'),'-lquadmath','-lgfortran','-o',str(native)]
    subprocess.run(cmd,capture_output=True,check=True)
    rust=ROOT/'rust/target'/profile/'examples/fitting_svd'
    a=subprocess.check_output([str(native)]);b=subprocess.check_output([str(rust)])
    out=BUILD/profile;out.mkdir(exist_ok=True);(out/'native.txt').write_bytes(a);(out/'rust.json').write_bytes(b)
    ref=[(key,int(i),int(j),D(value))for key,i,j,value in (line.split()for line in a.decode().splitlines())]
    got=json.loads(b);failures=[]
    for key,i,j,value in ref:
        actual=got
        for field in key.split('.'):actual=actual[field]
        if i>=0:actual=actual[i]
        if j>=0:actual=actual[j]
        actual=D(actual)
        scale=max(abs(v)for k,_,c,v in ref if k==key and c==j) if key not in ['pythag','decomposition.singular_values'] else abs(value)
        relative=D('1e-10'if key in ['solution','dropped_solution','decomposition.singular_values']else'5e-14')
        gate=relative*scale if value else D(0)
        if abs(actual-value)>gate or (value==actual==0 and value.is_signed()!=actual.is_signed()):
            failures.append({'quantity':[key,i,j],'native':str(value),'rust':str(actual),'gate':str(gate)})
    assert len(ref)==69
    (out/'receipt.json').write_text(json.dumps({'profile':profile,'compile':cmd,'builder_sha256':sha(__file__),'probe_sha256':sha(HERE/'svd_probe.cc'),'native_sha256':sha(native),'rust_sha256':sha(rust),'quantities':len(ref),'failures':failures},indent=2)+'\n')
    print('standalone SVD quantities',len(ref),'failures',failures)

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--profile',choices=['debug','release'],default='debug');run(p.parse_args().profile)
