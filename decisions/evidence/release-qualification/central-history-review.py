"""Central offline review of AL's first 0104 correction and retained evidence."""
from pathlib import Path
import hashlib
import json
import subprocess

ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
REVISION = 'cecee60899611f52ba41975b082d4951e0c66195'
PACKET = ROOT / 'validation/release/build/second-followthrough-history-fixed-v1'
OUT = Path('/var/folders/0f/w1j_z2b96f53kqjcj06c8yrw0000gn/T/tempo2-central-history-cecee60-xuyonfrm')
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()

receipt = json.loads((PACKET / 'receipt.json').read_text())
failed = ROOT / 'decisions/evidence/release-qualification/build/linux-run-41056090173-review-20261030/artifact/linux/native-corpora/receipt.json'
blocked = json.loads((PACKET / 'failed-preparation/receipt.json').read_text())
assert receipt['status'] == 'pass' and receipt['native_corpora_executed'] == 0
assert sha(failed) == receipt['native_failed_receipt'] == blocked['native_receipt']
assert blocked['status'] == 'failure' and blocked['stages'] == {}
assert len(blocked['blocked']) == receipt['unavailable_comparisons'] == 14
assert blocked['native_preparation']['failed_command'] == json.loads(failed.read_text())['failed_command']
committed = subprocess.check_output(['git', 'show', REVISION + ':validation/release/corpus_compare.py'], cwd=ROOT)
assert committed == (PACKET / 'failed-preparation/producer.py').read_bytes()

trees = {}
fetches = [c for c in receipt['commands'] if c['label'].startswith('history-fetch-')]
assert len(fetches) == len(receipt['history_pins']) == 7
for pin in receipt['history_pins']:
    command = next(c for c in fetches if c['label'] == 'history-fetch-' + pin)
    assert command['status'] == 0 and '--no-auto-maintenance' in command['argv']
    assert '--depth=1' in command['argv'] and command['argv'][-1] == pin + ':refs/al-history/' + pin
    assert command['argv'][-2] == 'https://github.com/wdong-eric/tempo2rust'
    original = subprocess.check_output(['git', 'rev-parse', pin + '^{tree}'], cwd=ROOT).decode().strip()
    staged = subprocess.check_output(['git', 'rev-parse', 'refs/al-history/' + pin + '^{tree}'], cwd=PACKET / 'staging').decode().strip()
    assert original == staged == (PACKET / ('actual-tree-' + pin + '.stdout')).read_text().strip()
    trace = [json.loads(line) for line in (PACKET / ('history-trace-' + pin + '.jsonl')).read_text().splitlines()]
    maintenance = [event for event in trace if event.get('event') == 'child_start' and any(x in event.get('argv', []) for x in ['maintenance', 'gc', 'repack'])]
    assert not maintenance, (pin, maintenance)
    trees[pin] = original
assert set((PACKET / 'staging/.git/shallow').read_text().splitlines()) == set(trees)
assert subprocess.check_output(['git', 'rev-list', '--all', '--count'], cwd=PACKET / 'staging').strip() == b'7'

files = [PACKET / 'receipt.json', failed, PACKET / 'failed-preparation/receipt.json', PACKET / 'failed-preparation/producer.py', OUT / 'receipt.json']
files += list(PACKET.glob('*.stdout')) + list(PACKET.glob('*.stderr')) + list(PACKET.glob('*.jsonl'))
result = {'revision': REVISION, 'scope': 'Offline history and failure-publication review; no native corpus or Linux qualification executed.',
          'unit_test_exit': json.loads((OUT / 'receipt.json').read_text())['exit'],
          'seven_exact_shallow_trees': trees, 'unavailable_comparisons': len(blocked['blocked']),
          'source': sha(__file__), 'files': {str(p): sha(p) for p in files}}
assert result['unit_test_exit'] == 0
target = OUT / 'central-evidence-review.json'
assert not target.exists()
target.write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps({'review': str(target), 'sha256': sha(target), 'verified_files': len(result['files']), 'trees': len(trees), 'unit_tests': 2}))
