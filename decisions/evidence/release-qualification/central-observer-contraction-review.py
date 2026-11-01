#!/usr/bin/env python3
"""Bound saved Linux LMST outputs to the source and explicit Rust contraction trees."""
import argparse
from fractions import Fraction
import gzip
import hashlib
import json
import math
from pathlib import Path
import re
import shutil
import struct
import subprocess
import zipfile

p=argparse.ArgumentParser(description=__doc__)
p.add_argument('--root',type=Path,required=True)
p.add_argument('--frozen',type=Path,required=True)
p.add_argument('--archive',type=Path,required=True)
p.add_argument('--out',type=Path,required=True)
a=p.parse_args();a.out.mkdir(parents=True,exist_ok=False)
shutil.copyfile(__file__,a.out/'executed-review.py')
sha=lambda b:hashlib.sha256(b).hexdigest()
bits=lambda x:struct.pack('>d',x).hex()
def fused(x,y,z):
    result=math.fma(x,y,z)
    assert bits(result)==bits(float(Fraction(x)*Fraction(y)+Fraction(z)))
    return result

def lmst(mjd,olong,use_fused):
    n=math.trunc(mjd);fmjd=mjd-n;tu0=(float(n-51545)+0.5)/3.6525e4
    dtu=fmjd/3.6525e4;tu=tu0+dtu
    aa=24110.54841;bb=8640184.812866;cc=0.093104;dd=-6.2e-6
    seconds_per_jc=86400.0*36525.0
    bprime=1.0+bb/seconds_per_jc;cprime=2.0*cc/seconds_per_jc;dprime=3.0*dd/seconds_per_jc
    if use_fused:
        gmst0=fused(tu0,fused(tu0,fused(tu0,dd,cc),bb),aa)/86400.0
        sdd=fused(tu,fused(tu,dprime,cprime),bprime)
        term=fused(tu0,tu0,fused(tu,tu,tu*tu0))
        gst=gmst0+dtu*fused(term,dd,fused(tu+tu0,cc,seconds_per_jc+bb))/86400.0
    else:
        gmst0=(aa+tu0*(bb+tu0*(cc+tu0*dd)))/86400.0
        sdd=bprime+tu*(cprime+tu*dprime)
        gst=gmst0+dtu*(seconds_per_jc+bb+cc*(tu+tu0)+dd*(tu*tu+tu*tu0+tu0*tu0))/86400.0
    xlst=gst-olong/360.0
    # Source fortran_mod receives an exactly represented binary64 value and 1.
    # On these bounded finite inputs its wide subtraction is exact.
    xlst-=math.trunc(xlst)
    if xlst<0:xlst+=1.0
    return [bits(xlst),bits(sdd)]

inputs={}
with zipfile.ZipFile(a.archive) as z:
    manifest_bytes=z.read('manifest.json')
    assert sha(manifest_bytes)=='6484c6b3238d807478740c4a8f20ae3770c0a99ae0b7481bdb72307ca140dd15'
    manifest=json.loads(manifest_bytes)['files']
    def read(name):
        b=z.read(name);assert sha(b)==manifest[name]['sha256'] and len(b)==manifest[name]['bytes']
        inputs[name]=manifest[name];return b
    pre='linux/native-corpora/staging/validation/observer_coordinates/'
    native_profile=json.loads(read(pre+'build/ordinary/profile.json'))
    assert native_profile['pin']=='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
    assert native_profile['machine']=='x86_64' and native_profile['compiler_version'].startswith('g++-13 ')
    assert native_profile['observer'] is False and native_profile['mutation'] is None
    native_source=subprocess.check_output(['git','-C',str(a.root/'tempo2'),'show',native_profile['pin']+':get_obsCoord.C'])
    source_map=json.loads(read(pre+'outputs/frozen/source_map.json'))
    assert sha(native_source)==source_map['hashes']['get_obsCoord.C']
    mapped=next(r for r in source_map['routines'] if r['name']=='lmst')
    assert mapped['source']==native_source.decode().splitlines()[mapped['start']-1:mapped['end']]
    source_audit=json.loads(read('linux/source-audit/receipt.json'))
    rust_source=(a.frozen/'rust/src/get_obs_coord.rs').read_bytes()
    assert sha(rust_source)==source_audit['rust_source']['get_obs_coord.rs']
    cases=json.loads(read(pre+'outputs/frozen/cases.json'))
    results=[]
    for caseid in ['c008_lmst_dates','c009_lmst_wrap']:
        fixture=next(r for r in cases if r['id']==caseid)
        observed={}
        for variant in ['record_raw','observer_raw']:
            records=[json.loads(line) for line in gzip.decompress(read(pre+f'outputs/{variant}/{caseid}.jsonl.gz')).splitlines()]
            observed[variant]=[r['result']['outputs'] for r in records if r.get('phase')=='end' and r.get('operation')=='lmst']
        assert observed['record_raw']==observed['observer_raw']
        rustpre='linux/corpus/wxxxxxxxxxxxxxx/validation/observer_coordinates_port/outputs/release/'+caseid+'/'
        rust_records=[json.loads(line) for line in read(rustpre+'events.jsonl').splitlines()]
        rust=[r['result']['outputs'] for r in rust_records if r.get('phase')=='end' and r.get('operation')=='lmst']
        actions=[r for r in fixture['actions'] if r[0]=='lmst']
        assert len(actions)==len(rust)==len(observed['record_raw'])
        for action,expected,actual in zip(actions,observed['record_raw'],rust):
            assert all(r['class']=='finite' for r in expected+actual)
            native_bits=[r['bits'] for r in expected];rust_bits=[r['bits'] for r in actual]
            separate=lmst(float(action[1]),float(action[2]),False)
            contracted=lmst(float(action[1]),float(action[2]),True)
            assert native_bits==separate,(action,native_bits,separate)
            assert rust_bits==contracted,(action,rust_bits,contracted)
            results.append({'case':caseid,'mjd':action[1],'longitude':action[2],
                'native':native_bits,'rust':rust_bits,'separate':separate,'fused':contracted})
    # This is a lexical navigation inventory, not authorization to alter all sites.
    inventory={}
    for path in sorted((a.frozen/'rust/src').glob('*.rs')):
        hits=[{'line':i,'text':line.strip()} for i,line in enumerate(path.read_text().splitlines(),1) if '.mul_add(' in line or '.source_mul_add(' in line]
        if hits:inventory[path.name]=hits
summary={'scope':'Saved LMST finite results and source-expression witnesses; no production edit or new Linux execution',
    'run':41069548581,'candidate':'a35104a611abb5c1d5c48dc4d1866f239ccd3ab9',
    'source':'get_obsCoord.C:496-531','native_source_sha256':sha(native_source),'rust_source_sha256':sha(rust_source),
    'native_executable':native_profile['executable_sha256'],'compiler':native_profile['compiler_version'].splitlines()[0],'flags':native_profile['flags'],
    'query_count':len(results),'scalar_outputs':len(results)*2,
    'differing_queries':sum(r['native']!=r['rust'] for r in results),
    'differing_scalars':sum(x!=y for r in results for x,y in zip(r['native'],r['rust'])),
    'separate_matches_all_native':True,'fused_matches_all_rust':True,'native_observer_neutral_for_results':True,
    'inventory_files':len(inventory),'inventory_lines':sum(map(len,inventory.values())),
    'inputs':inputs,'recipe_sha256':sha(Path(__file__).read_bytes())}
assert summary['query_count']==45 and summary['differing_scalars']==10
(a.out/'receipt.json').write_text(json.dumps(summary,indent=2)+'\n')
(a.out/'lmst-results.json').write_text(json.dumps(results,indent=2)+'\n')
(a.out/'contraction-navigation.json').write_text(json.dumps(inventory,indent=2)+'\n')
print(json.dumps({k:v for k,v in summary.items() if k!='inputs'},indent=2))
print('receipt_sha256',sha((a.out/'receipt.json').read_bytes()))
