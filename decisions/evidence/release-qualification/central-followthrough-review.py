from pathlib import Path
import hashlib,json,re,tarfile
root=Path('/Users/Eric.Dong/Downloads/tempo2rust')
out=Path('/tmp/tempo2-al-next-review-current').read_text().strip();out=Path(out)
build=root/'validation/release/build'
def digest(path):
 with Path(path).open('rb') as stream:return hashlib.file_digest(stream,'sha256').hexdigest()
report={'scope':'Independent retained-evidence verification only; no new engine execution or acceptance','report_receipts':{},'archives':{},'focused':{}}
archived_files={}
packet=build/'followup-evidence-v1';index=json.loads((packet/'manifest.json').read_text())
for number,(name,entry) in enumerate(index.items(),1):
 archive=packet/entry['archive'];assert digest(archive)==entry['archive_sha256'],name
 assert archive.stat().st_size==entry['archive_bytes'],name
 actual={};manifest=None
 with tarfile.open(archive,mode='r|gz') as bundle:
  for member in bundle:
   assert member.isfile(),(name,member.name,member.type)
   stream=bundle.extractfile(member)
   if member.name=='manifest.json':
    data=stream.read();assert hashlib.sha256(data).hexdigest()==entry['manifest_sha256'],name
    manifest=json.loads(data)
   else:
    assert member.name not in actual,(name,member.name)
    actual[member.name]={'bytes':member.size,'sha256':hashlib.file_digest(stream,'sha256').hexdigest()}
 assert manifest and actual==manifest['files'],name
 assert len(actual)==entry['files'],name
 assert sum(x['bytes'] for x in actual.values())==entry['uncompressed_bytes'],name
 archived_files.update({str(Path(entry['source'])/relative):row['sha256'] for relative,row in actual.items()})
 report['archives'][name]={'files':len(actual),'bytes':entry['uncompressed_bytes'],'archive_bytes':entry['archive_bytes'],'sha256':entry['archive_sha256']}
 if number%5==0:print('Verified archive',number,'of',len(index),flush=True)
packet=build/'linux-followthrough-evidence-v1';index=json.loads((packet/'manifest.json').read_text())
for entry in index['runs']:
 selected=packet/entry['selected'];manifest=selected/'manifest.json'
 assert digest(manifest)==entry['manifest_sha256'],entry['selected']
 files=json.loads(manifest.read_text())['files']
 assert len(files)==entry['files'] and sum(x['bytes'] for x in files.values())==entry['bytes']
 for relative,record in files.items():
  path=selected/relative
  assert path.stat().st_size==record['bytes'] and digest(path)==record['sha256'],(entry['selected'],relative)
 archived_files.update({str(Path(entry['source'])/relative):row['sha256'] for relative,row in files.items()})
 report['focused'][entry['selected']]={'files':len(files),'bytes':entry['bytes']}
for name,expected in index['recipes'].items():assert digest(packet/'recipes'/name)==expected,name
assert digest(packet/'producer.py')==index['producer_sha256']
for name,marker in [('RELEASE_QUALIFICATION_REPORT.md','## Follow-through retained identities'),('RELEASE_LINUX_FOLLOWUP.md','## Bound local evidence')]:
 text=(root/'rust'/name).read_text().split(marker,1)[1]
 for relative,expected in re.findall(r'\| `([^`]+\.json)` \| `([a-f0-9]{64})` \|',text):
  path=build/relative
  actual=digest(path) if path.is_file() else archived_files[str(path)]
  assert actual==expected,(relative,actual,expected)
  report['report_receipts'][relative]={'sha256':actual,'verified_at':'original path' if path.is_file() else 'verified selected archive'}
report['totals']={'archive_selections':len(report['archives']),'archive_files':sum(x['files'] for x in report['archives'].values()),'archive_bytes':sum(x['archive_bytes'] for x in report['archives'].values()),'focused_files':sum(x['files'] for x in report['focused'].values()),'focused_bytes':sum(x['bytes'] for x in report['focused'].values()),'report_receipts':len(report['report_receipts'])}
report['recipe_sha256']=digest(__file__)
(out/'evidence-review.json').write_text(json.dumps(report,indent=2)+'\n')
(out/'executed-evidence-review.py').write_bytes(Path(__file__).read_bytes())
print(json.dumps(report['totals']),flush=True)
print('Review SHA-256:',digest(out/'evidence-review.json'),flush=True)
