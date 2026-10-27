"""Independently verify AJ's delivered hashes and committed source binding."""
from pathlib import Path
import hashlib
import json
import re
import subprocess

ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
OUT = Path(Path('/tmp/tempo2-aj-committed-review-current').read_text().strip())
BUILD = ROOT / 'validation/library_interfaces/build'
PRODUCTION = '3e41497d5a86e627b0ce994d4882e4b2e77d3bf7'
VALIDATION = '31c729079da44afe7155e42707edeac20d66bd2b'

def sha(p):
    h = hashlib.sha256()
    with Path(p).open('rb') as f:
        for block in iter(lambda: f.read(1048576), b''):
            h.update(block)
    return h.hexdigest()

checked = {}
def check(path, expected):
    path = Path(path).resolve()
    digest = sha(path)
    assert digest == expected, (path, expected, digest)
    checked[str(path)] = digest

report = ROOT / 'rust/LIBRARY_INTERFACES_REPORT.md'
audit_path = BUILD / 'final-audit/audit.json'
audit = json.loads(audit_path.read_text())
resolved = subprocess.check_output(['git','rev-parse',audit['production_commit']],cwd=ROOT,text=True).strip()
assert resolved == PRODUCTION and audit['failures'] == []
assert audit['unique_files'] == len(audit['files']) == 2444
assert sha(audit_path) in report.read_text()
for path,digest in audit['files'].items():
    check(path,digest)
for path,digest in audit['receipts'].items():
    check(path,digest)
    assert digest in report.read_text(),path
for name,digest in audit['loaded_evidence'].items():
    check(audit_path.parent/name,digest)
check(ROOT/'validation/library_interfaces/audit_delivery.py',audit['script'])

def blobs(rev, path):
    rows = subprocess.check_output(['git','ls-tree','-r',rev,path],cwd=ROOT,text=True)
    return {line.split('\t')[1]:line.split()[2] for line in rows.splitlines()}

production = blobs(PRODUCTION,'rust')
validation = blobs(VALIDATION,'validation/library_interfaces')
def check_blob(path, name, expected):
    data = Path(path).read_bytes()
    assert hashlib.sha1(b'blob '+str(len(data)).encode()+b'\0'+data).hexdigest() == expected,name

for name,oid in production.items():
    check_blob(ROOT/name,name,oid)
for name,oid in validation.items():
    check_blob(ROOT/name,name,oid)
replay = json.loads((OUT/'replay-identity.json').read_text())
for name,digest in replay['observer_sources'].items():
    check(OUT/'workspace'/name,digest)
    check_blob(OUT/'workspace'/name,name,validation[name])

prefix_counts = {}
for profile in ['debug','release']:
    installed = BUILD/'profiles-final-v1'/(profile+'-prefix')/'share/tempo2-rust/rust'
    count = 0
    for path in installed.rglob('*'):
        if path.is_file():
            name='rust/'+str(path.relative_to(installed))
            check_blob(path,name,production[name]);count += 1
    prefix_counts[profile]=count

for profile in ['debug','release']:
    log=(BUILD/'final-checks'/(profile+'.stdout')).read_text()
    assert 'test result: ok. 91 passed; 0 failed; 36 ignored;' in log,profile
checks=json.loads((BUILD/'final-checks/receipt.json').read_text())
assert all(c['status']==0 for c in checks['commands'])
assert checks['mapped_units']==93
for name in ['final-abi-debug-v4','final-abi-release-v3']:
    d=json.loads((BUILD/name/'receipt.json').read_text())
    assert len(d['symbols'])==64 and len(d['profiles'])==4 and len(d['rejected_controls'])==12
    assert all(p['layout_rows']==44 and p['value_state_rows']==68 and p['storage_rows']==121 and
               p['low_count_rows']==48 and p['boundary_rows']==26 and p['phase_diagnostic_lines']==270 for p in d['profiles'])
for name,n in [('final-rust-debug',29),('final-rust-release',29),('final-rust-combined',65)]:
    d=json.loads((BUILD/name/'receipt.json').read_text())
    assert len(d['commands'])-2==n and all(c['status']==0 for c in d['commands'])
pairs=json.loads((BUILD/'final-installed-backend-pairs/receipt.json').read_text())
assert len(pairs['records'])==59 and sum(r['comparison']['checks'] for r in pairs['records'])==41961
assert pairs['failures']==0 and pairs['changed_artifacts']==[]
pin_records={}
for repo,pin in [('tempo2','fda6ad4a9753ad7dc5d291406f2f310a9161423f'),('validation/reference','7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572')]:
    assert subprocess.check_output(['git','-C',ROOT/repo,'rev-parse','HEAD'],text=True).strip()==pin
    assert not subprocess.check_output(['git','-C',ROOT/repo,'status','--porcelain'])
    pin_records[repo]=pin
result=dict(production=PRODUCTION,validation=VALIDATION,report_sha256=sha(report),
            owner_audit_sha256=sha(audit_path),verified_files=checked,verified_count=len(checked),
            committed_production_files=len(production),committed_validation_files=len(validation),
            central_frozen_observers=len(replay['observer_sources']),installed_source_files=prefix_counts,
            source_pins=pin_records,script_sha256=sha(__file__),failures=[],
            scope='Read-only identity, source binding and claimed-count audit of named delivered campaigns; independent execution is recorded separately.')
(OUT/'delivery-audit.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({k:result[k] for k in ['verified_count','committed_production_files','committed_validation_files','central_frozen_observers','installed_source_files','failures']}))
