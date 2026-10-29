#!/usr/bin/env python3
"""Replay AH's approved UT1 producer against the complete X caller histories."""
import argparse
import ast
import copy
import importlib.util
import json
from pathlib import Path
import sys
from common import ROOT, Run, sha


def main(args):
    evidence=Run(args.out);native=args.native.resolve()
    sys.path.insert(0,str(native))
    spec=importlib.util.spec_from_file_location('x_runner',native/'run.py')
    x=importlib.util.module_from_spec(spec);spec.loader.exec_module(x)
    import profile
    # Reuse the accepted pure transformation without executing AH's build setup.
    path=ROOT/'validation/core_output/native_build.py';source=path.read_text()
    node=next(n for n in ast.parse(source).body if isinstance(n,ast.FunctionDef) and n.name=='observer_corrected')
    function='\n'.join(source.splitlines()[node.lineno-1:node.end_lineno])
    namespace={};exec(compile(function,str(path),'exec'),namespace)
    raw=profile.blob('get_obsCoord.C').decode();corrected=namespace['observer_corrected'](raw)
    mutation=('get_obsCoord.C',raw,corrected)
    ordinary,ordinary_profile=profile.compile_probe('fixed000',mutation=mutation)
    observer,observer_profile=profile.compile_probe('fixed001',mutation=mutation,observer=True)
    original=x.read_gzip(native/'outputs/observer_snapshots.json.gz')
    catalog=x.frozen_cases();observed=[];plain=[];changes=[];receipts=[]
    def unchanged(before,after,path):
        assert type(before) is type(after),(path,'type')
        if isinstance(before,dict):
            assert list(before)==list(after),(path,'ordered keys')
            for key in before:
                if key=='correctionUT1' and before[key]!=after[key]:
                    changes.append(dict(path=path+'.'+key,original=before[key],corrected=after[key]))
                else:unchanged(before[key],after[key],path+'.'+key)
        elif isinstance(before,list):
            assert len(before)==len(after),(path,'length')
            for i,(a,b) in enumerate(zip(before,after,strict=True)):unchanged(a,b,path+f'[{i}]')
        else:assert before==after,(path,before,after)
    for case,before in zip(catalog,original,strict=True):
        actual,receipt=x.execute(case,observer,observer_profile,evidence.out/'observer_raw')
        normal,normal_receipt=x.execute(case,ordinary,ordinary_profile,evidence.out/'ordinary_raw')
        projected=copy.deepcopy(actual);projected['events']=[e for e in projected['events'] if e['phase']!='exit']
        for event in projected['events']:
            if 'private' in event:event['private']=None
        x.compare(normal,projected,case['id']+'.corrected_observer_projection')
        unchanged(before,actual,case['id'])
        observed.append(actual);plain.append(normal);receipts.append({'ordinary':normal_receipt,'observer':receipt})
    assert changes
    x.write_gzip(evidence.out/'reference.json.gz',plain)
    x.write_gzip(evidence.out/'observer_snapshots.json.gz',observed)
    x.write_gzip(evidence.out/'launches.json.gz',receipts)
    manifest={'original':sha(native/'outputs/observer_snapshots.json.gz'),'ordinary_profile':ordinary_profile,
              'observer_profile':observer_profile,'authority':'decisions/0094-ah-handover-and-ai-release.md',
              'transform':{'file':str(path),'sha256':sha(path),'function':function},'changes':changes,
              'cases':len(catalog),'ordinary':sha(evidence.out/'reference.json.gz'),'observer':sha(evidence.out/'observer_snapshots.json.gz')}
    (evidence.out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
    evidence.save({'status':'pass','manifest':sha(evidence.out/'manifest.json'),'cases':len(catalog),
                   'changed_fields':len(changes),'scope':'Only accepted correctionUT1 producer fields differ; all other state, private caches, diagnostics and outcomes exact. Corrected observer independently equals corrected ordinary execution.',
                   'producer':sha(__file__)})
    print(len(catalog),'cases;',len(changes),'UT1 field changes; all other state exact')


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--native',type=Path,required=True);parser.add_argument('--out',type=Path,required=True)
    main(parser.parse_args())
