"""Read-only AL publication review: delivered identities and focused checks."""
from pathlib import Path
import hashlib
import json
import os
import re
import subprocess
import tempfile
import time

ROOT = Path('/Users/Eric.Dong/Downloads/tempo2rust')
REV = 'c14b1ee6fb703484036345c00453ba27c66c19df'
SHALLOW = Path(Path('/tmp/tempo2-al-shallow-review-current').read_text().strip())
SOURCE = SHALLOW/'checkout'
BUILD = ROOT/'validation/release/build'
OUT = Path(tempfile.mkdtemp(prefix='tempo2-al-central-review-'))
Path('/tmp/tempo2-al-central-review-current').write_text(str(OUT)+'\n')
(OUT/'executed-script.py').write_bytes(Path(__file__).read_bytes())

def sha(path):
    h=hashlib.sha256()
    with Path(path).open('rb') as f:
        for block in iter(lambda:f.read(1024*1024), b''):h.update(block)
    return h.hexdigest()

commands=[]
def run(label,args,expected=0,cwd=OUT):
    args=list(map(str,args));start=time.monotonic()
    with (OUT/(label+'.stdout')).open('wb') as stdout,(OUT/(label+'.stderr')).open('wb') as stderr:
        r=subprocess.run(args,cwd=cwd,stdout=stdout,stderr=stderr)
    commands.append({'label':label,'argv':args,'exit':r.returncode,'seconds':time.monotonic()-start,
                     'stdout':sha(OUT/(label+'.stdout')),'stderr':sha(OUT/(label+'.stderr'))})
    (OUT/'commands.json').write_text(json.dumps(commands,indent=2)+'\n')
    assert r.returncode==expected,label
    print(label,r.returncode,flush=True)

print(OUT,flush=True)
report=(SOURCE/'rust/RELEASE_QUALIFICATION_REPORT.md').read_text()
receipts={}
for name,expected in re.findall(r'\| `([^`]+\.json)` \| `([0-9a-f]{64})` \|',report):
    path=BUILD/name
    actual=sha(path);assert actual==expected,name
    receipts[name]=actual
assert len(receipts)>=25
manifest=BUILD/'selected-evidence-v1/manifest.json'
selected=json.loads(manifest.read_text())
for name,record in selected['files'].items():
    path=manifest.parent/name
    assert path.stat().st_size==record['bytes'] and sha(path)==record['sha256'],name
aggregate=json.loads((BUILD/'integrated-v1/receipt.json').read_text())
assert aggregate['candidate']=='ca49531f5eb0b21362f585887f7278c269d4f7be'
assert len(aggregate['stages'])==58 and not aggregate['blocked']
assert set(aggregate['failed'])=={'m2-comparison','fits','boundaries','ddh','predictors','output'}
assert sum(v['exit']==0 for v in aggregate['stages'].values())==52
assert aggregate['status']=='failure'

git_files={}
for line in subprocess.check_output(['git','-C',SOURCE,'ls-tree','-r',REV,'rust','validation/release','.github/workflows'],text=True).splitlines():
    meta,name=line.split('\t',1);mode,kind,oid=meta.split();assert kind=='blob',name
    data=(SOURCE/name).read_bytes()
    assert hashlib.sha1(b'blob '+str(len(data)).encode()+b'\0'+data).hexdigest()==oid,name
    git_files[name]=sha(SOURCE/name)
assert not subprocess.check_output(['git','-C',ROOT,'diff','--name-only','e014f02',REV,'--','rust/src'],text=True).strip()
pins={}
for name,expected in [('tempo2','fda6ad4a9753ad7dc5d291406f2f310a9161423f'),
                      ('validation/reference','7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572')]:
    actual=subprocess.check_output(['git','-C',ROOT/name,'rev-parse','HEAD'],text=True).strip()
    status=subprocess.check_output(['git','-C',ROOT/name,'status','--porcelain'],text=True)
    assert actual==expected,name
    # Native construction exports Git blobs. Finder metadata is not an input;
    # retain and report it without deleting user files or calling the tree clean.
    status_rows=status.splitlines()
    assert all(row.startswith('?? ') and Path(row[3:]).name=='.DS_Store' for row in status_rows),(name,status)
    pins[name]={'head':actual,'status':status,'tracked_changes':False}
run('comparator-controls',['python3','-B','-m','unittest','discover','-s',SOURCE/'validation/release','-p','test_*.py'])
run('package-audit',['python3','-B',SOURCE/'validation/release/audit.py',
    '--candidate',BUILD/'integrated-v1/candidate/source',
    '--prefix',BUILD/'integrated-v1/install-default',
    '--prefix',BUILD/'integrated-v1/install-combined','--out',OUT/'package-audit'])

# Check the two clock identities and their complete recorded runtime membership.
a2=BUILD/'integrated-v1/a2'
native=json.loads((a2/'receipt.json').read_text())
for name,expected in native['runtime'].items():assert sha(a2/'runtime'/name)==expected,name
assert not (a2/'runtime/clock/gps2utc.clk').exists()
assert sha(a2/'runtime-historical/clock/gps2utc.clk')=='30727fdfaaec0f1775543804210d057ffe62076594bb8884d8f8ccafafb676be'
historical={str(p.relative_to(a2/'runtime-historical')) for p in (a2/'runtime-historical').rglob('*') if p.is_file()}
assert historical==set(native['runtime'])|{'clock/gps2utc.clk'}
for name,expected in native['runtime'].items():assert sha(a2/'runtime-historical'/name)==expected,name
ddh=json.loads((BUILD/'integrated-v1/ddh/receipt.json').read_text())
ddh_summary=[]
for record in ddh['records']:
    ddh_summary.append({'clock':record['clock'],'fidelity_failures':record['fidelity_failures'],
        'ideal':{stage:{engine:{'maximum_s':v['maximum_s'],'failing_rows':len(v['failing_rows'])} for engine,v in values.items()} for stage,values in record['ideal'].items()}})
package=json.loads((OUT/'package-audit/receipt.json').read_text())
summary={'candidate_reviewed':REV,'aggregate_candidate':aggregate['candidate'],
    'report_receipts':receipts,'selected_evidence_files':len(selected['files']),
    'selected_evidence_bytes':sum(v['bytes'] for v in selected['files'].values()),
    'selected_manifest':sha(manifest),'frozen_git_files':git_files,'pins':pins,
    'aggregate_stages':len(aggregate['stages']),'passing_stages':52,'failed_stages':aggregate['failed'],
    'package_checks':package['checked_files'],'package_mismatches':package['mismatches'],
    'runtime_files':len(native['runtime']),'ddh':ddh_summary,
    'shallow_checkout_finding':str(SHALLOW/'summary.json'),
    'scope':'Identity audit, existing-result checks and focused comparator controls; not Linux execution or new scientific acceptance',
    'script':sha(__file__),'commands':sha(OUT/'commands.json')}
(OUT/'review.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps({k:v for k,v in summary.items() if k not in ['frozen_git_files','report_receipts','ddh']},indent=2),flush=True)
