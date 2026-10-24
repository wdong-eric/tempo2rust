"""Run accepted R from T-owned staging; only ROOT and OUT assignments differ."""
import difflib,subprocess,sys
from pathlib import Path
from run import ROOT,OUT,ENV,sha,save,protected

ACCEPTED={
 'run.py':'7d6424654a666d3852255bd9264ba0fbb88ba3c6e57f7bf717da4087a6b10529',
 'native.py':'d10a35686b2a6cf0629b413d52027cf5608f7b91a326ee1fb51cb61f0680d290',
 'verify.py':'e3e56140a9b06635e9d3fe07873e33c0a1bd87e6c1171b614e901aede9c29866',
}

def main():
 original=ROOT/'validation/observation_storage_port';stage=OUT/'routing/observation_storage_port'
 receipt=OUT/'routing/receipt.json';receipt.unlink(missing_ok=True)
 for name,digest in ACCEPTED.items():assert sha(original/name)==digest,'accepted R runner must be restored: '+name
 frozen=protected()
 files={p.name:p.read_bytes() for p in original.iterdir() if p.is_file()}
 source_hashes={str((original/name).relative_to(ROOT)):sha(original/name) for name in sorted(files)}
 raw=files['run.py'].decode()
 changes={"ROOT=HERE.parents[1]":f"ROOT=Path({str(ROOT)!r})", "OUT=RUST/'verification/observation_storage'":f"OUT=Path({str(OUT/'R')!r})"}
 routed=raw
 for before,after in changes.items():
  assert routed.count(before)==1,before
  routed=routed.replace(before,after)
 restored=routed
 for before,after in changes.items():restored=restored.replace(after,before)
 assert restored==raw,'routing changed more than the two path assignments'
 stage.mkdir(parents=True,exist_ok=True)
 assert {p.name for p in stage.iterdir() if p.is_file()} in (set(),set(files)),'unexpected staged membership'
 for name,data in files.items():(stage/name).write_bytes(routed.encode() if name=='run.py' else data)
 staged_hashes={str((stage/name).relative_to(ROOT)):sha(stage/name) for name in sorted(files)}
 for name in files:
  if name!='run.py':assert (stage/name).read_bytes()==files[name],name
 diff=OUT/'routing/run.diff'
 diff.write_text(''.join(difflib.unified_diff(raw.splitlines(True),routed.splitlines(True),fromfile=str((original/'run.py').relative_to(ROOT)),tofile=str((stage/'run.py').relative_to(ROOT)))))
 save(OUT/'routing/identity.json',dict(decision='0050',accepted_revision='04918d5',original=source_hashes,staged=staged_hashes,diff_sha256=sha(diff),runner_sha256=sha(Path(__file__))))
 command=[sys.executable,str(stage/'verify.py')]
 result=subprocess.run(command,cwd=ROOT,env=dict(ENV,PYTHONDONTWRITEBYTECODE='1'),capture_output=True,text=True)
 (OUT/'routing/verify.log').write_text(result.stdout+result.stderr)
 print(result.stdout,end='');print(result.stderr,end='',file=sys.stderr)
 assert source_hashes=={p:sha(ROOT/p) for p in source_hashes},'accepted R inputs changed'
 assert staged_hashes=={p:sha(ROOT/p) for p in staged_hashes},'staged R inputs changed'
 assert set(files)=={p.name for p in original.iterdir() if p.is_file()}
 assert set(files)=={p.name for p in stage.iterdir() if p.is_file()}
 assert protected()==frozen
 assert result.returncode==0,('R/O regressions',result.returncode)
 save(receipt,dict(status='PASS',decision='0050',original_runner_sha256=ACCEPTED['run.py'],staged_runner_sha256=sha(stage/'run.py'),identity_sha256=sha(OUT/'routing/identity.json'),diff_sha256=sha(diff),command=command,returncode=result.returncode,log_sha256=sha(OUT/'routing/verify.log'),regression_receipt_sha256=sha(OUT/'R/delivery_checks.json'),protected_files=frozen))

if __name__=='__main__':main()
