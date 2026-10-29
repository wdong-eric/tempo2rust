#!/usr/bin/env python3
"""Separately witness accepted initialization and reserved-slot copy corrections."""
import argparse
import copy
import gzip
import importlib.util
import json
from pathlib import Path
import re
import sys
from common import ROOT, Run, sha


def main(args):
    evidence=Run(args.out);native=args.native.resolve()
    sys.path.insert(0,str(native))
    spec=importlib.util.spec_from_file_location('s_runner',native/'run.py')
    s=importlib.util.module_from_spec(spec);spec.loader.exec_module(s)
    import profile as native_profile
    string_capacity=int(re.search(rb'^#define\s+MAX_STRLEN\s+(\d+)',native_profile.blob('tempo2.h'),re.M)[1])
    correction=ROOT/'validation/general_fitting/fitting_corrections.py'
    spec=importlib.util.spec_from_file_location('ae_corrections',correction)
    ae=importlib.util.module_from_spec(spec);spec.loader.exec_module(ae)
    prepare=native_profile.prepare
    def corrected_sources(name='source',mutation=None):
        source=prepare(name,mutation)
        path=source/'tempo2Util.C';raw=path.read_text()
        path.write_text(ae.corrected('tempo2Util.C',raw))
        return source
    native_profile.prepare=corrected_sources
    old='    psr->ne_sw  = NE_SW_DEFAULT;'
    new=old+' psr->ne_sw_ifuncN=0; psr->nWhite_dm=0; psr->nchromx=0; psr->AverageDMResiduals=0;'
    mutation=('initialise.C',old,new)
    # Same prefix length and source line count retain TKlog's truncation/locations.
    exe,profile=s.compile_probe('fixed',mutation=mutation)
    records=[];changes=[]
    expected=evidence.out/'expected';expected.mkdir()
    for case in s.cases():
        original=s.reference(case['id'])
        actual=s.execute(case,exe,profile,evidence.out/'raw')
        checked=copy.deepcopy(actual)
        assert len(original['events'])==len(checked['events'])
        for before,after in zip(original['events'],checked['events'],strict=True):
            assert before['phase']==after['phase'] and len(before['state'])==len(after['state'])
            for i,(a,b) in enumerate(zip(before['state'],after['state'],strict=True)):
                assert a['path']==b['path']
                if a==b:continue
                field=a['path'].rsplit('.',1)[-1]
                if field in {'ne_sw_ifuncN','nWhite_dm','nchromx','AverageDMResiduals'}:
                    assert b==dict(a,runs=[[1,'00000000']]),(case['id'],a,b)
                else:
                    assert field in {'jumpVal','jumpValErr','fitJump','jumpStr',
                                     'fdjumpVal','fdjumpValErr','fitfdJump','fdjumpStr'},(case['id'],after['phase'],a,b)
                    assert {k:v for k,v in a.items() if k!='runs'}=={k:v for k,v in b.items() if k!='runs'}
                    left=[v for n,v in a['runs'] for _ in range(n)]
                    right=[v for n,v in b['runs'] for _ in range(n)]
                    # Strings project their entire fixed-size slot as bytes.
                    reserved=string_capacity if field in {'jumpStr','fdjumpStr'} else 1
                    assert left[reserved:]==right[reserved:],(case['id'],a['path'],'nonreserved state changed')
                changes.append(dict(case=case['id'],phase=after['phase'],path=a['path'],original=a,corrected=b))
                after['state'][i]=a
        s.compare(original,checked,case['id']+'.all_other_state')
        path=expected/(case['id']+'.json.gz')
        path.write_bytes(gzip.compress(json.dumps(actual,separators=(',',':')).encode(),mtime=0))
        records.append(dict(case=case['id'],original_sha256=sha(native/'expected'/(case['id']+'.json.gz')),corrected_sha256=sha(path)))
    assert {'ne_sw_ifuncN','nWhite_dm','nchromx','AverageDMResiduals'} <= {c['path'].rsplit('.',1)[-1] for c in changes}
    evidence.save({'status':'pass','profile':profile,'mutation':mutation,'records':records,'changes':changes,
                   'copy_transform':{'file':str(correction),'sha256':sha(correction),'function':"corrected('tempo2Util.C', source)"},
                   'scope':'Native proof only: four accepted initialization writes and reserved slot-zero copy corrections; every other complete state/diagnostic field exact. Raw S evidence remains unchanged.',
                   'authority':['decisions/0068-general-input-to-bat-prepared-contract.md',
                                'rust/INPUT_TO_BAT_REPORT.md','decisions/0079-ad-handover-and-ae-release.md',
                                'rust/GENERAL_FITTING_REPORT.md'],
                   'producer':sha(__file__)})
    print(len(records),'cases;',len(changes),'changed approved records; all other state exact')


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--native',type=Path,required=True);parser.add_argument('--out',type=Path,required=True)
    main(parser.parse_args())
