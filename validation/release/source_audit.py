#!/usr/bin/env python3
"""Bind the responsibility map and current bodies; lexical indexes are not coverage."""
import argparse
import csv
import json
from pathlib import Path
import re
from common import ROOT, PIN, Run, identities, sha

BASELINE='6acaf53d2e13291eb6d0b0988faf7dee7d33a635'


def compare_bodies(run):
    previous=run.run(['git','-C',ROOT,'ls-tree','-r','--name-only',BASELINE,'--','rust/src'],
                     'baseline-tree',expected=None)
    if run.commands[-1]['status'] != 0:
        run.save({'status':'failure','baseline':BASELINE,'baseline_status':'unavailable',
                  'failed_command':'baseline-tree'})
        raise SystemExit('Required source-audit baseline is unavailable; fetch '+BASELINE)
    baseline={name for name in previous.decode().splitlines() if name.endswith('.rs')}
    current={str(path.relative_to(ROOT)):path for path in (ROOT/'rust/src').rglob('*.rs')}
    added=sorted(current.keys()-baseline);removed=sorted(baseline-current.keys())
    changed=added+removed;unchanged={}
    for name in sorted(current.keys() & baseline):
        original=run.run(['git','-C',ROOT,'show',BASELINE+':'+name],
                         'baseline-'+name.removeprefix('rust/src/').replace('/','-'))
        if original==current[name].read_bytes():unchanged[name]=sha(current[name])
        else:changed.append(name)
    return {'baseline':BASELINE,'baseline_status':'available','unchanged_rust_bodies':unchanged,
            'changed_rust_bodies':sorted(changed),'added_rust_bodies':added,'removed_rust_bodies':removed}

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--out',type=Path,required=True);p.add_argument('--native-repository',type=Path,required=True)
    a=p.parse_args();run=Run(a.out)
    bodies=compare_bodies(run)
    make=run.run(['git','-C',a.native_repository,'show',PIN+':Makefile.am'],'makefile').decode()
    fit=re.search(r'^FITFUNCS=(.*)$',make,re.M)[1]
    names=re.search(r'^libtempo2_la_SOURCES = (.*)$',make,re.M)[1].replace('$(headers)','enum_str.C').replace('$(FITFUNCS)',fit).split()
    # The standalone predictor library also owns t2toolkit_global.C.
    assert 't2toolkit_global.C' in make
    names=sorted(set(names)|{'t2toolkit_global.C'})
    rows=list(csv.DictReader((ROOT/'validation/release/source_correspondence.tsv').open(),delimiter='\t'))
    assert len(names)==93 and len(rows)==93 and {r['source'] for r in rows}==set(names)
    lexical=[];unsafe=[];departures=[]
    for path in sorted((ROOT/'rust/src').rglob('*.rs')):
        rel=str(path.relative_to(ROOT))
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
    data={'status':'pass',**bodies,'native_pin':PIN,'responsibilities':rows,'sofa':sofa_records,
          'function_and_export_navigation':lexical,'unsafe_navigation':unsafe,'departure_navigation':departures,
          'meaning':'Identity/presence audit and lexical navigation, not exhaustive branch or numerical acceptance',
          'maps':{str(p.relative_to(ROOT)):sha(p) for p in (ROOT/'rust').glob('*SOURCE_MAP.md')},
          'rust_source':identities(ROOT/'rust/src')}
    run.save(data)
    print(len(names),'library units;',len(bodies['unchanged_rust_bodies']),'unchanged Rust body files;',len(bodies['changed_rust_bodies']),'changed;',len(unsafe),'unsafe lexical sites;',len(sofa),'identical SOFA routines')
