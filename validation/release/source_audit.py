#!/usr/bin/env python3
"""Bind the responsibility map and current bodies; lexical indexes are not coverage."""
import argparse
import csv
import json
from pathlib import Path
import re
import subprocess
from common import ROOT, PIN, Run, identities, sha

BASELINE='6acaf53d2e13291eb6d0b0988faf7dee7d33a635'

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--out',type=Path,required=True);p.add_argument('--native-repository',type=Path,required=True)
    a=p.parse_args();run=Run(a.out)
    make=run.run(['git','-C',a.native_repository,'show',PIN+':Makefile.am'],'makefile').decode()
    fit=re.search(r'^FITFUNCS=(.*)$',make,re.M)[1]
    names=re.search(r'^libtempo2_la_SOURCES = (.*)$',make,re.M)[1].replace('$(headers)','enum_str.C').replace('$(FITFUNCS)',fit).split()
    # The standalone predictor library also owns t2toolkit_global.C.
    assert 't2toolkit_global.C' in make
    names=sorted(set(names)|{'t2toolkit_global.C'})
    rows=list(csv.DictReader((ROOT/'validation/release/source_correspondence.tsv').open(),delimiter='\t'))
    assert len(names)==93 and len(rows)==93 and {r['source'] for r in rows}==set(names)
    changed=[];unchanged={};lexical=[];unsafe=[];departures=[]
    for path in sorted((ROOT/'rust/src').rglob('*.rs')):
        rel=str(path.relative_to(ROOT))
        previous=run.run(['git','-C',ROOT,'show',BASELINE+':'+rel],'baseline-'+path.name,expected=None)
        if previous==path.read_bytes():unchanged[rel]=sha(path)
        else:changed.append(rel)
        for number,line in enumerate(path.read_text().splitlines(),1):
            item={'file':rel,'line':number,'text':line.strip()}
            if re.search(r'\b(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?fn\s+|^pub (?:mod|use) ',line):lexical.append(item)
            if re.search(r'\bunsafe\b',line):unsafe.append(item)
            if re.search(r'0053|fda6ad4|source (?:bug|defect)|Source (?:bug|defect)',line):departures.append(item)
    sofa=list((ROOT/'rust/native/sofa').glob('*.for'))
    assert len(sofa)==36
    sofa_records={}
    for path in sofa:
        original=run.run(['git','-C',a.native_repository,'show',PIN+':sofa/'+path.name],'sofa-'+path.name)
        assert original==path.read_bytes(),path
        sofa_records[path.name]=sha(path)
    data={'status':'pass','baseline':BASELINE,'native_pin':PIN,'responsibilities':rows,
          'unchanged_rust_bodies':unchanged,'changed_rust_bodies':changed,'sofa':sofa_records,
          'function_and_export_navigation':lexical,'unsafe_navigation':unsafe,'departure_navigation':departures,
          'meaning':'Identity/presence audit and lexical navigation, not exhaustive branch or numerical acceptance',
          'maps':{str(p.relative_to(ROOT)):sha(p) for p in (ROOT/'rust').glob('*SOURCE_MAP.md')},
          'rust_source':identities(ROOT/'rust/src')}
    run.save(data)
    print(len(names),'library units;',len(unchanged),'unchanged Rust body files;',len(changed),'changed;',len(unsafe),'unsafe lexical sites;',len(sofa),'identical SOFA routines')
