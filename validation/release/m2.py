#!/usr/bin/env python3
"""Reconstruct the authored M2 catalogue and native inputs inside a clean export."""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import shutil
from common import ROOT, Run, sha


def prepare(out,a2,candidate):
    run=Run(out);out=run.out;dest=candidate/'validation/fitting'
    assert not (candidate/'.git').exists() and (candidate.parent/'receipt.json').exists()
    source=ROOT/'validation/fitting/generate_reference.py'
    spec=importlib.util.spec_from_file_location('al_m2_generator',source)
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
    m.ROOT=out/'catalogue';m.ROOT.mkdir();m.INPUT=m.ROOT/'inputs';m.INPUT.mkdir()
    m.BUILD=out;m.REF=out/'reference';m.REF.mkdir();m.RUNTIME=a2/'runtime'
    m.A2=out/'a2';(m.A2/'fixtures').mkdir(parents=True)
    (m.A2/'fixtures/original').symlink_to(a2/'source/tests/test_data',target_is_directory=True)
    cxx=os.environ.get('NATIVE_CXX','g++')
    run.run([cxx,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(a2/'source'),
        '-I'+str(ROOT/'validation/release/a2'),ROOT/'validation/fitting/export_reference.cc',
        a2/'libtempo2-reference.a','-lquadmath','-lgfortran','-o',out/'export_reference'],'build')
    m.create_inputs()
    # These are recipe-generated inputs, not downloaded/reused golden outputs.
    # Keep committed fixtures as the public workload; report any regeneration drift.
    regenerated={p.name:sha(p) for p in m.INPUT.iterdir() if p.is_file()}
    differences=[name for name,digest in regenerated.items() if (dest/'inputs'/name).exists() and sha(dest/'inputs'/name)!=digest]
    shutil.copyfile(m.ROOT/'cases.json',dest/'cases.json')
    reference=dest/'reference';reference.mkdir(exist_ok=True)
    assert not list(reference.glob('*.json')), 'Refuse to replace an existing native campaign'
    cases=json.loads((dest/'cases.json').read_text())
    env=dict(os.environ,TEMPO2=str(a2/'runtime'),LC_ALL='C',REFERENCE_RESOURCE_LOG=str(out/'resources.tsv'))
    for row in cases['solver']:
        run.run([out/'export_reference','solver',dest/'inputs'/row['input'],reference/(row['name']+'.json')],
                row['name'],env=env,expected=None if row['expected']=='boundary' else 0)
    run.run([out/'export_reference','callbacks','unused',reference/'callbacks.json'],'callbacks',env=env)
    for row in cases['timing']:
        run.run([out/'export_reference','fit',dest/'inputs'/row['par'],dest/'inputs'/row['tim'],
            int(row['rescale']),reference/(row['name']+'.json')],row['name'],env=env)
    run.save({'status':'pass','scope':'M2 native preparation; numerical comparison separate',
        'regenerated_input_differences':differences,'regenerated_inputs':regenerated,
        'commands_from_catalogue':m.COMMANDS,'generator':sha(source),'a2_receipt':sha(a2/'receipt.json'),
        'outputs':{p.name:sha(p) for p in reference.iterdir() if p.is_file()}})


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','a2','candidate']:p.add_argument('--'+name,type=Path,required=True)
    a=p.parse_args();prepare(a.out,a.a2.resolve(),a.candidate.resolve())
