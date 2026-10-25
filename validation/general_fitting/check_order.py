"""Reproduce the source caller-order mismatch without changing its helpers."""
from pathlib import Path
import hashlib, json, subprocess

ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
BUILD=HERE/'build/order';BUILD.mkdir(parents=True,exist_ok=True)
REF=ROOT/'validation/reference/build';native=BUILD/'native'
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
cmd=['/opt/homebrew/bin/g++-16','-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(REF/'source'),str(HERE/'order_probe.cc'),str(REF/'libtempo2-reference.a'),'-lquadmath','-lgfortran','-o',str(native)]
p=subprocess.run(cmd,capture_output=True);assert p.returncode==0,p.stderr.decode()
p=subprocess.run([str(native)],capture_output=True);assert p.returncode==0,p.stderr.decode()
(BUILD/'stdout').write_bytes(p.stdout);(BUILD/'stderr').write_bytes(p.stderr)
rows=[l for l in p.stdout.decode().splitlines()if l.startswith('ORDER ')]
assert rows==['ORDER 0 CORRECTED 0 MISMATCHES 3','ORDER 0 CORRECTED 1 MISMATCHES 0','ORDER 1 CORRECTED 0 MISMATCHES 0','ORDER 1 CORRECTED 1 MISMATCHES 0']
(BUILD/'receipt.json').write_text(json.dumps({'pin':'fda6ad4a9753ad7dc5d291406f2f310a9161423f','builder_sha256':sha(__file__),'probe_sha256':sha(HERE/'order_probe.cc'),'archive_sha256':sha(REF/'libtempo2-reference.a'),'executable_sha256':sha(native),'stdout_sha256':sha(BUILD/'stdout'),'command':cmd,'results':rows},indent=2)+'\n')
print('three original unsorted pairings fail; corrected order and sorted neighbors pass')
