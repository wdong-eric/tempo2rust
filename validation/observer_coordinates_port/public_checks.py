"""Run the accepted M1/M2/public recovery gate once, redirecting its outputs only."""
import hashlib,importlib.util,json,sys
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];RUST=ROOT/'rust';OUT=HERE/'build/public'
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
s=importlib.util.spec_from_file_location('aa_accepted_m2',RUST/'verify_m2.py');m=importlib.util.module_from_spec(s);s.loader.exec_module(m);m.OUTPUT=OUT
before={str(p.relative_to(ROOT)):sha(p) for p in sorted((RUST/'src').glob('*.rs'))}
status=0
try:m.main()
except SystemExit as e:status=e.code
assert status==1,'expected retained M2 failure exit'
failures=json.loads((OUT/'open_gate_failures.json').read_text());assert failures==json.loads((RUST/'verification/m2/open_gate_failures.json').read_text()) and len(failures)==4,failures
assert before=={n:sha(ROOT/n) for n in before}
(OUT/'aa-receipt.json').write_text(json.dumps(dict(status='PASS affected regressions; four historical M2 gate failures remain',sources=before,runner_sha256=sha(RUST/'verify_m2.py'),M1_runner_sha256=sha(RUST/'verify_m1b.py'),routing_sha256=sha(__file__),failures=failures,commands_sha256=sha(OUT/'commands.json'),results_sha256=sha(OUT/'numerical_errors.json')),indent=2)+'\n')
print('PASS public checks; exactly four unchanged M2 findings')
