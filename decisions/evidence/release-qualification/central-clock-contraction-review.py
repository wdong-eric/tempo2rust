#!/usr/bin/env python3
"""Bind the Linux duplicate-knot result to separate and fused binary64 arithmetic."""
import argparse
from fractions import Fraction
import hashlib
import json
import math
from pathlib import Path
import shutil
import struct
import zipfile

p=argparse.ArgumentParser(description=__doc__)
p.add_argument('--root',type=Path,required=True)
p.add_argument('--archive',type=Path,required=True)
p.add_argument('--out',type=Path,required=True)
a=p.parse_args();a.out.mkdir(parents=True,exist_ok=False)
shutil.copyfile(__file__,a.out/'executed-review.py')
sha=lambda b:hashlib.sha256(b).hexdigest()
bits=lambda x:struct.pack('>d',x).hex()
inputs={}
with zipfile.ZipFile(a.archive) as z:
    manifest_raw=z.read('manifest.json')
    assert sha(manifest_raw)=='6484c6b3238d807478740c4a8f20ae3770c0a99ae0b7481bdb72307ca140dd15'
    manifest=json.loads(manifest_raw)['files']
    def read(name):
        raw=z.read(name);assert sha(raw)==manifest[name]['sha256'] and len(raw)==manifest[name]['bytes']
        inputs[name]=manifest[name];return raw
    pre='linux/native-corpora/staging/validation/clocks/'
    refs=json.loads(read(pre+'outputs/reference.json'))['cases']
    ref=next(r for r in refs if r['id']=='table_internal_duplicate')
    fixture=next(r for r in json.loads(read(pre+'cases.json')) if r['id']==ref['id'])
    native=json.loads(read(pre+'outputs/manifest.json'))
    profile=json.loads(read(pre+'build/profile.json'))
    log=read('linux/corpus/clocks.stdout').decode()
    assert ref['returncode']==0 and not ref['timeout']
    assert native['source']['source_revision']=='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
    assert profile['machine']=='x86_64' and profile['compiler'].startswith('g++-13 ')
    source=(a.root/'tempo2/tabulatedfunction.C').read_bytes()
    assert sha(source)==native['source']['sources']['tabulatedfunction.C']['sha256']
    rows=[tuple(map(float,line.split())) for line in fixture['files']['sample.clk'].splitlines() if not line.startswith('#')]
    records=[]
    for event in ref['events']:
        if event['phase']!='end' or event['operation']!='value':continue
        x=float(event['result']['query']['decimal']);lo=0;hi=len(rows)-1
        while hi>lo+1:
            mid=(lo+hi)//2
            if rows[mid][0]>x:hi=mid
            else:lo=mid
        x0,y0=rows[hi-1];x1,y1=rows[hi]
        ratio=(x-x0)/(x1-x0);difference=y1-y0
        product=ratio*difference
        separate=product+y0
        exact=Fraction(ratio)*Fraction(difference)+Fraction(y0)
        fused=float(exact)
        assert bits(fused)==bits(math.fma(ratio,difference,y0))
        expected=event['result']['value']['bits']
        assert expected==bits(separate)
        records.append({'query':repr(x),'bracket':[hi-1,hi],'ratio_bits':bits(ratio),
            'native_bits':expected,'separate_bits':bits(separate),'fused_bits':bits(fused)})
    mismatch=[r for r in records if r['native_bits']!=r['fused_bits']]
    assert len(records)==5 and len(mismatch)==1
    assert mismatch[0]['query']=='1.9999999999999998'
    failure=next(s for s in log.splitlines() if s.startswith('table_internal_duplicate FAIL'))
    assert "'bits': '4033ffffffffffff'" in failure and "'bits': '4034000000000000'" in failure
    mac_path=a.root/'decisions/evidence/clocks/interpolation-contraction.json'
    mac=json.loads(mac_path.read_bytes())
    assert mac['source_blob_sha256']==sha(source)
    assert mac['reference_result']['value']['bits']==mismatch[0]['fused_bits']
    assert any('fmadd' in s for s in mac['instruction_sequence'])
receipt={'scope':'One source expression and five saved finite queries; no new Linux executable or Rust correction executed',
    'run':41069548581,'candidate':'a35104a611abb5c1d5c48dc4d1866f239ccd3ab9',
    'source':'tabulatedfunction.C:163-164','source_sha256':sha(source),
    'native_executable':native['native']['executable_sha256'],'compiler':profile['compiler'],
    'flags':profile['flags'],'records':records,'rust_failure':failure,
    'accepted_mac_evidence_sha256':sha(mac_path.read_bytes()),'inputs':inputs,
    'arithmetic_witness':'Exact rational final product-plus-addition rounded once agrees with math.fma; separate binary64 operations match all five saved Linux outputs',
    'recipe_sha256':sha(Path(__file__).read_bytes())}
(a.out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps({k:v for k,v in receipt.items() if k!='inputs'},indent=2))
print('receipt_sha256',sha((a.out/'receipt.json').read_bytes()))
