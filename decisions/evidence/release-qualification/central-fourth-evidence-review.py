#!/usr/bin/env python3
"""Verify AL's fourth-run evidence packet without relabeling retained failures."""
import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import zipfile

p=argparse.ArgumentParser(description=__doc__)
p.add_argument('--root',type=Path,required=True)
p.add_argument('--packet',type=Path,required=True)
p.add_argument('--out',type=Path,required=True)
a=p.parse_args();a.out.mkdir(parents=True,exist_ok=False)
shutil.copyfile(__file__,a.out/'executed-review.py')
def sha(path):
    h=hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda:stream.read(1024*1024),b''):h.update(block)
    return h.hexdigest()
ledger=json.loads((a.packet/'ledger.json').read_text())
producer=a.packet/'producer.py'
assert sha(producer)==ledger['recipe']
recipe_commit='b95879c84279467fbff77aab4c2c5c76f565c099'
assert producer.read_bytes()==subprocess.check_output(['git','-C',str(a.root),'show',recipe_commit+':validation/release/third_followthrough.py'])
# The producer's recorded Git HEAD was 91309fd while its final evidence-group
# selection edit was uncommitted. Its executed bytes, not that HEAD alone, are
# the identity; b95879c commits those exact producer bytes.
assert ledger['source_revision']=='91309fd84cd54b1275b1aee3f6c12bbfd4dc38f6'
linux=a.packet/'original-linux-receipt.json'
assert sha(linux)==ledger['original_linux_receipt']['sha256']
with zipfile.ZipFile(ledger['original_linux_archive']['path']) as bundle:
    raw=bundle.read('manifest.json')
    assert hashlib.sha256(raw).hexdigest()=='6484c6b3238d807478740c4a8f20ae3770c0a99ae0b7481bdb72307ca140dd15'
    manifest=json.loads(raw)['files']
    assert manifest['linux/receipt.json']==ledger['original_linux_receipt']
    assert bundle.read('linux/receipt.json')==linux.read_bytes()
original=json.loads(linux.read_text());assert original['stages']==ledger['original_linux_stages']
assert len(original['stages'])==67
assert sum(v['exit']==0 for v in original['stages'].values())==54
count=total=0;groups={}
assert len(ledger['focused_evidence'])==37
for name,binding in ledger['focused_evidence'].items():
    directory=a.packet/name;manifest_path=directory/'manifest.json'
    assert sha(manifest_path)==binding['manifest'],name
    recorded=json.loads(manifest_path.read_text());source=Path(recorded['source'])
    assert source.resolve()==Path(binding['original_directory']).resolve()
    files=recorded['files']
    assert {str(p.relative_to(directory)) for p in directory.rglob('*') if p.is_file()}==set(files)|{'manifest.json'},name
    for relative,expected in files.items():
        saved=directory/relative;live=source/relative
        assert saved.stat().st_size==live.stat().st_size==expected['bytes'],(name,relative)
        assert sha(saved)==expected['sha256']==sha(live),(name,relative)
        count+=1;total+=expected['bytes']
    if 'receipt' in binding:
        assert sha(directory/'receipt.json')==binding['receipt']
        receipt=json.loads((directory/'receipt.json').read_text())
        assert receipt.get('status')==binding['status']
        if 'stages' in binding:assert receipt['stages']==binding['stages']
    groups[name]={'manifest_sha256':binding['manifest'],'verified_files':len(files),
                  'recorded_status':binding.get('status'),'stages':binding.get('stages')}
    print(name,len(files),'verified',flush=True)
assert count==47887 and total==7139104046
output=json.loads((a.packet/'third-output-v2/manifest.json').read_text())['files']
assert any(n.endswith('literal-polyco_new.dat') for n in output)
assert any(n.endswith('literal-newpolyco.dat') for n in output)
summary={'scope':'Independent retained-file and frozen-ledger review; no new scientific execution or qualification acceptance',
    'packet':str(a.packet.resolve()),'ledger_sha256':sha(a.packet/'ledger.json'),
    'producer_sha256':sha(producer),'producer_committed_identity':recipe_commit,
    'collection_git_head':ledger['source_revision'],'original_linux_stage_count':67,
    'original_linux_passed':54,'original_linux_failed':13,'groups':groups,
    'verified_files':count,'verified_bytes':total,'original_and_copied_bytes_match':True,
    'prefixed_predictor_products_retained':True,'recipe_sha256':sha(Path(__file__)),
    'completed_utc':datetime.now(timezone.utc).isoformat()}
(a.out/'receipt.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps({k:v for k,v in summary.items() if k!='groups'},indent=2))
print('receipt_sha256',sha(a.out/'receipt.json'))
