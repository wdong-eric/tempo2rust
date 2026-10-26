"""Fresh native reader-only witness of the source Doppler-width correction."""
from pathlib import Path
import hashlib,json,shutil,subprocess,tempfile
root=Path('/Users/Eric.Dong/Downloads/tempo2rust');build=root/'validation/predictors/build'
out=Path(tempfile.mkdtemp(prefix='tempo2-af-reader-review-'))
Path('/tmp/tempo2-af-reader-review-current').write_text(str(out)+'\n')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
identities={}
for label,stage in [('before','native-delivery-v2'),('after','native-corrected')]:
 source=build/stage;d=json.loads((source/'identity.json').read_text());identities[label]=d
 assert sha(source/'predictor')==d['executable']
 shutil.copy2(source/'predictor',out/label);shutil.copy2(source/'identity.json',out/(label+'-identity.json'))
 shutil.copy2(source/'source/t1polyco.c',out/(label+'-t1polyco.c'))
 shutil.copy2(source/'corrections.patch',out/(label+'-corrections.patch'))
a,b=identities['before'],identities['after']
assert [name for name in a['sources'] if a['sources'][name]!=b['sources'].get(name)]==['t1polyco.c']
assert a['harness']==b['harness'] and a['archive']==b['archive'] and a['pin']==b['pin']
records=[]
for case in ['isolated-bary-polyco','isolated-pks-polyco']:
 d=out/case;d.mkdir();src=build/'public-debug-delivery'/case/'rust/polyco_new.dat';file=d/'polyco_new.dat';shutil.copy2(src,file)
 outputs={}
 for label in ['before','after']:
  cmd=[str(out/label),'inspect',str(d/(label+'.tsv')),str(file)]
  with (d/(label+'.stdout')).open('w') as so,(d/(label+'.stderr')).open('w') as se:
   p=subprocess.run(cmd,stdout=so,stderr=se)
  assert p.returncode==0,(case,label,p.returncode)
  outputs[label]={(f[0],int(f[1])):f[2] for f in (line.split('\t') for line in (d/(label+'.tsv')).read_text().splitlines())}
 changes=[{'field':k[0],'index':k[1],'before':v,'after':outputs['after'][k]} for k,v in outputs['before'].items() if outputs['after'][k]!=v]
 assert outputs['before'].keys()==outputs['after'].keys()
 if case=='isolated-bary-polyco':
  assert {(r['field'],r['index']) for r in changes}=={(field,i) for field in ['doppler','log10rms'] for i in [0,1]}
  for i,rms in [(0,-16.517),(1,-16.564)]:
   assert float(outputs['before']['doppler',i])==-9999
   assert float(outputs['before']['log10rms',i])==990
   assert float(outputs['after']['doppler',i])==-9999.99
   assert float(outputs['after']['log10rms',i])==rms
 else:assert not changes
 records.append({'case':case,'input_sha256':sha(file),'changed':changes,'other_fields_exact':True})
shutil.copy2(__file__,out/'central_af_polyco_reader_review.py')
receipt={'scope':'Fresh execution of existing immutable native reader executables against the same original file bytes; no regeneration, timing-engine run or Rust rerun.',
 'source_delta':['t1polyco.c'],'same_harness_archive_and_pin':True,'executables':{k:sha(out/k) for k in ['before','after']},'records':records,
 'files':{str(p.relative_to(out)):sha(p) for p in sorted(out.rglob('*')) if p.is_file()}}
(out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps({'evidence':str(out),'receipt_sha256':sha(out/'receipt.json'),'records':records},indent=2))
