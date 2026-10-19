#!/usr/bin/env python3
"""Snapshot A's completed exports; retain decimal strings without conversion."""
import hashlib
import json
from pathlib import Path

root = Path(__file__).resolve().parent
source = root.parent / 'reference'
manifest_bytes=(source/'build_manifest.json').read_bytes()
manifest=json.loads(manifest_bytes)
assert manifest['source_commit']=='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
assert manifest['precision_backend']=='__float128'
destination=root/'results/reference_inputs'
destination.mkdir(exist_ok=True)
metadata={key:manifest[key] for key in ('source_commit','canonical_status','platform','machine',
          'precision_backend','compiler_versions','instrumentation_sha256')}
metadata['build_manifest_sha256']=hashlib.sha256(manifest_bytes).hexdigest()
metadata['files']={}
for name in ['checks/precision.json','export_reference.cc','REPORT.md']:
    data=(source/name).read_bytes()
    target=destination/Path(name).name
    target.write_bytes(data)
    metadata['files'][name]={'source':str(source/name),'snapshot':str(target.relative_to(root)),
                             'sha256':hashlib.sha256(data).hexdigest()}
for fixture,target_name in [('barycentric','test1.tim-standard.tsv'),
                            ('barycentric_4ns','test1.tim-subtractBATs.tsv'),
                            ('terrestrial','test2.tim-standard.tsv'),
                            ('long_span','long_span.tsv')]:
    name=f'outputs/{fixture}.json'
    data=(source/name).read_bytes()
    record=json.loads(data)
    assert record['status']=='completed' and record['returncode']==0
    assert record['source_commit']==manifest['source_commit']
    assert record['build_manifest_sha256']==metadata['build_manifest_sha256']
    (destination/f'{fixture}.json').write_bytes(data)
    lines=[]
    for key,value in record['configuration'].items():
        lines.append(f'config\t{key}\t{value}')
    for key,p in record['parameters'].items():
        lines.append(f'param\t{key}\t{p["set"]}\t{p["fit_flag"]}\t{p["value"]}\t{p["upstream_label"]}')
    for stage,observations in record['stages'].items():
        for index,fields in enumerate(observations):
            for key,value in fields.items():
                assert '\t' not in str(value) and '\n' not in str(value)
                lines.append(f'obs\t{stage}\t{index}\t{key}\t{value}')
    target=destination/target_name
    target.write_text('\n'.join(lines)+'\n')
    metadata['files'][name]={'source':str(source/name),'snapshot':f'results/reference_inputs/{fixture}.json',
       'sha256':hashlib.sha256(data).hexdigest(),'derived_tsv':str(target.relative_to(root)),
       'derived_tsv_sha256':hashlib.sha256(target.read_bytes()).hexdigest(),'availability':record['availability']}
metadata['scope']='A completed exports; terrestrial remains provisional. Re-evaluate exported bbat through phase only, not observatory corrections.'
(destination/'provenance.json').write_text(json.dumps(metadata,indent=2)+'\n')
print('Captured four completed A exports, preserving their availability status.')
