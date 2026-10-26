"""Frozen AF identical-input components; outputs must go to a fresh directory."""
from pathlib import Path
import argparse,subprocess,json,hashlib
from decimal import Decimal,getcontext
getcontext().prec=70
D=Decimal
EPS=D(2)**-52

from compare import read_tsv, Checks, phase_bound, hz_bound

def main():
 p=argparse.ArgumentParser();p.add_argument('--native',type=Path,required=True);p.add_argument('--rust',type=Path,required=True);p.add_argument('--out',type=Path,required=True);p.add_argument('--case',default='polynomial',choices=['polynomial','constant','linear']);a=p.parse_args();a.out.mkdir(parents=True,exist_ok=False)
 subprocess.run([str(a.native.resolve()),'component',str((a.out/'native.tsv').resolve()),a.case],check=True,stdout=(a.out/'native.log').open('w'),stderr=subprocess.STDOUT)
 subprocess.run([str(a.rust.resolve()),a.case],check=True,stdout=(a.out/'rust.json').open('w'),stderr=(a.out/'rust.log').open('w'))
 native=read_tsv(a.out/'native.tsv');rust=json.loads((a.out/'rust.json').read_text());checks=Checks()
 for key in ['coeff','derivative','phase','frequency']:
  count=48 if key in ('coeff','derivative') else 13
  checks.exact(key+'.native_count',sum(k==key for k,i in native),count)
  checks.exact(key+'.rust_count',len(rust[key]),count)
  for i,v in enumerate(rust[key]):
   bound=16*EPS*8*6*D('5.875')*(128 if key=='derivative' else 1) if key in ('coeff','derivative') else (hz_bound(D('5.875')) if key=='frequency' else phase_bound(D('5.875')))
   checks.number(f'{key}[{i}]',v,native[key,i],bound)
 for label,field in [('rms','rms_turn'),('mav','mav_turn')]:checks.number(label,rust['test'][field],native[label,0],phase_bound(D('5.875')))
 fail=checks.failures;checked=checks.count
 receipt={'checked':checked,'failures':fail,'native':hashlib.sha256(a.native.read_bytes()).hexdigest(),'rust':hashlib.sha256(a.rust.read_bytes()).hexdigest(),'contract':hashlib.sha256((Path(__file__).parent/'CONTRACT.md').read_bytes()).hexdigest(),'native_summary':{k:str(native[k,0]) for k in ['rms','mav']},'rust_summary':rust['test']}
 (a.out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps(receipt,indent=2));return bool(fail)
if __name__=='__main__':raise SystemExit(main())
