"""Temporarily restore the original wet typo to challenge the continuity test."""
import subprocess
from run import ROOT,RUST,OUT,ENV,sha,save

def main():
 path=RUST/'src/tropo.rs';original=path.read_bytes();digest=sha(path)
 good=b'f.mul_add(c[j], (1.0 - f) * c[i])';bad=b'f.mul_add(b[j], (1.0 - f) * c[i])'
 assert original.count(good)==1
 dest=OUT/'wet_original_mutation';dest.mkdir(parents=True,exist_ok=True)
 cmd=['cargo','test','--locked','--offline','--lib','terrestrial_tests::wet_mapping_is_continuous_at_latitude_knots','--','--exact','--nocapture']
 try:
  path.write_bytes(original.replace(good,bad));mutant=sha(path)
  p=subprocess.run(cmd,cwd=RUST,env=ENV,capture_output=True,text=True)
  (dest/'original-expression.log').write_text(p.stdout+p.stderr)
  assert p.returncode==101 and 'test result: FAILED' in p.stdout,(p.stdout,p.stderr)
 finally:path.write_bytes(original)
 assert sha(path)==digest
 p2=subprocess.run(cmd,cwd=RUST,env=ENV,capture_output=True,text=True)
 (dest/'corrected-expression.log').write_text(p2.stdout+p2.stderr)
 assert p2.returncode==0,(p2.stdout,p2.stderr)
 save(dest/'receipt.json',dict(status='PASS',command=cmd,source_sha256=digest,mutant_sha256=mutant,original_expression_exit=p.returncode,corrected_exit=p2.returncode,logs={n:sha(dest/n) for n in ['original-expression.log','corrected-expression.log']}))
 print('Original wet expression fails independent knot continuity; corrected expression passes.',flush=True)
if __name__=='__main__':main()
