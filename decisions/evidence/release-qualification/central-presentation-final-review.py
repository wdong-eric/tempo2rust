#!/usr/bin/env python3
"""Bind the final presentation delivery and independently reject its old mutations."""
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import shutil
import subprocess
import sys
import tarfile

ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
OUT=Path('/tmp/tempo2-central-presentation-final-review-20261101')
DELIVERY=ROOT/'validation/release/build/presentation-review-v1'
PACKET=DELIVERY/'selected-evidence'
CANDIDATE='b093586d728f5f9beca81d49f45cff9609225964'
PRODUCTION='801bc94139cca90d6072234b09f5a1bcec5e4b05'
OUT.mkdir()
shutil.copyfile(__file__,OUT/'executed-recipe.py')


def sha(path):
    with path.open('rb') as stream:return hashlib.file_digest(stream,'sha256').hexdigest()


def read(path):return json.loads(path.read_text())


delivery=read(DELIVERY/'receipt.json')
binding=read(PACKET/'binding.json')
assert delivery['candidate']==binding['candidate']==CANDIDATE
assert delivery['production']==binding['production']==PRODUCTION
archive=Path(delivery['archive'])
assert sha(archive)==delivery['archive_sha256'] and archive.stat().st_size==delivery['archive_bytes']
manifest=read(PACKET/'manifest.json')
assert len(manifest)==delivery['selected_files']==1802
assert sha(PACKET/'manifest.json')==delivery['manifest_sha256']
assert sha(PACKET/'binding.json')==delivery['binding_sha256']
with tarfile.open(archive) as bundle:
    members=bundle.getmembers()
    assert len(members)==delivery['archive_members']==1803
    assert all(member.isfile() for member in members)
    manifest_member=min((m for m in members if PurePosixPath(m.name).name=='manifest.json'),key=lambda m:len(PurePosixPath(m.name).parts))
    prefix=PurePosixPath(manifest_member.name).parent
    names=set()
    for member in members:
        name=str(PurePosixPath(member.name).relative_to(prefix))
        assert name not in names and '..' not in PurePosixPath(name).parts
        names.add(name)
        expected=delivery['manifest_sha256'] if name=='manifest.json' else manifest[name]
        with bundle.extractfile(member) as stream:assert hashlib.file_digest(stream,'sha256').hexdigest()==expected,name
        assert sha(PACKET/name)==expected,name
    assert names==set(manifest)|{'manifest.json'}
assert {str(p.relative_to(PACKET)) for p in PACKET.rglob('*') if p.is_file()}==set(manifest)|{'manifest.json'}

git_archive=OUT/'candidate.tar'
with git_archive.open('xb') as stream:
    subprocess.run(['git','-C',str(ROOT),'archive','--format=tar',CANDIDATE],stdout=stream,check=True)
source=OUT/'source';source.mkdir()
with tarfile.open(git_archive) as bundle:bundle.extractall(source,filter='data')
for name,digest in {**binding['production_sources'],**binding['recipe_and_document_hashes']}.items():
    assert sha(source/name)==digest,name
subprocess.run(['git','-C',str(ROOT),'diff','--exit-code',PRODUCTION,CANDIDATE,'--',
                'rust/src','rust/Cargo.toml','rust/Cargo.lock','rust/build.rs','rust/native','rust/data','rust/install.py'],check=True)
sys.path.insert(0,str(source/'validation/release'))
from diagnostic_attribution import Attribution
from presentation import normalized
mapping=Attribution(source)
assert sha(source/'validation/release/diagnostic_attribution.py')==binding['projection']
mutations=PACKET/'central-original-false-acceptance'
rejected=[]
for name in ['mutated.stdout','version-controls/build-identity.stdout','version-controls/version-site.stdout']:
    try:normalized((mutations/name).read_bytes(),'rust',mapping)
    except AssertionError as error:
        assert 'internal comparison label' in str(error)
        rejected.append(name)
    else:raise AssertionError(('false acceptance remains',name))
assert normalized((mutations/'correct.stdout').read_bytes(),'rust',mapping)==normalized((mutations/'before.stdout').read_bytes(),'native',mapping)

runs=PACKET/'runs'
installed=runs/'presentation-installed-v2'
record=read(installed/'receipt.json')
assert record['projection']==binding['projection'] and record['status']=='pass'
assert all(record['checks'].values()) and len(record['checks'])==18 and len(record['commands'])==20
assert all(row['status']==row['expected'] for row in record['commands'])
replayed={}
for case in ['report','versions','debug']:
    replayed[case]=normalized((installed/(case+'-before.stdout')).read_bytes(),'native',mapping)==normalized((installed/(case+'-after.stdout')).read_bytes(),'rust',mapping)
for case in ['evaluation','fit','bats']:
    replayed[case]=mapping.structured(read(installed/(case+'-before.json')),'native')==mapping.structured(read(installed/(case+'-after.json')),'rust')
    replayed[case+'-stderr']=normalized((installed/(case+'-before.stderr')).read_bytes(),'native',mapping)==normalized((installed/(case+'-after.stderr')).read_bytes(),'rust',mapping)
assert all(replayed.values()),replayed
final=read(runs/'presentation-final-checks-v2/receipt.json')
for name,digest in final['logs'].items():assert sha(runs/'presentation-v1'/name)==digest,name
for name,digest in final['receipts'].items():assert sha(runs/name/'receipt.json')==digest,name
assert '98 passed; 1 failed; 37 ignored' in (runs/'presentation-v1/unit-final.stdout').read_text()
assert '1 passed; 0 failed' in (runs/'presentation-v1/m2-unit-final.stdout').read_text()
assert 'Ran 22 tests' in (runs/'presentation-v1/harness-final.stderr').read_text()
assert sha(runs/'presentation-v1/successor-output.rs')==sha(source/'rust/src/core_output_source.rs')
timing=read(runs/'presentation-timing-v2/receipt.json')
assert timing['status']=='pass' and timing['cases']==6 and not timing['failures']
boundaries=read(runs/'presentation-boundaries-v2/receipt.json')
assert boundaries['status']=='pass' and boundaries['rows']==176
cmd=[sys.executable,'-B','-m','unittest','discover','-s',str(source/'validation/release'),'-p','test_diagnostic_attribution.py','-v']
test=subprocess.run(cmd,capture_output=True,env=dict(os.environ,PYTHONDONTWRITEBYTECODE='1'))
(OUT/'controls.stdout').write_bytes(test.stdout);(OUT/'controls.stderr').write_bytes(test.stderr)
assert test.returncode==0 and b'Ran 5 tests' in test.stderr
report={'scope':'Final macOS presentation delivery review; no Linux or scientific milestone acceptance',
    'candidate':CANDIDATE,'production':PRODUCTION,'checker':delivery['checker'],
    'archive_sha256':sha(archive),'verified_archive_members':len(names),'verified_selected_files':len(manifest),
    'verified_bytes':sum((PACKET/name).stat().st_size for name in manifest),
    'production_sources':len(binding['production_sources']),'bound_recipes_and_documents':len(binding['recipe_and_document_hashes']),
    'rejected_original_mutations':rejected,'fresh_control_tests':5,'replayed_comparisons':replayed,
    'installed_commands':20,'installed_checks':18,'timing_cases':6,'boundary_rows':176,
    'clean_export_tests':'98 passed; missing generated catalogue then prepared and affected test passed',
    'projection_sha256':binding['projection'],'manifest_sha256':sha(PACKET/'manifest.json'),
    'delivery_receipt_sha256':sha(DELIVERY/'receipt.json'),'final_checks_sha256':sha(runs/'presentation-final-checks-v2/receipt.json'),
    'recipe_sha256':sha(OUT/'executed-recipe.py')}
(OUT/'review.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
print('review_sha256',sha(OUT/'review.json'))
