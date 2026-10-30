#!/usr/bin/env python3
"""Reconstruct the authored M2 catalogue and native inputs inside a clean export."""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import shutil
import difflib
from common import ROOT, Run, sha


def observer_differences(raw, corrected, path=''):
    """Only the historical observer's discarded negative NaN sign may differ."""
    if isinstance(raw,dict):
        assert raw.keys()==corrected.keys(),path
        return sum(observer_differences(v,corrected[k],path+'/'+k) for k,v in raw.items())
    if isinstance(raw,list):
        assert len(raw)==len(corrected),path
        return sum(observer_differences(a,b,path+'/'+str(i)) for i,(a,b) in enumerate(zip(raw,corrected,strict=True)))
    assert raw==corrected or (raw=='NaN' and corrected=='-NaN'),(path,raw,corrected)
    return int(raw!=corrected)


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
    def campaign(exe,directory,prefix):
        for row in cases['solver']:
            run.run([exe,'solver',dest/'inputs'/row['input'],directory/(row['name']+'.json')],
                    prefix+row['name'],env=env,expected=None if row['expected']=='boundary' else 0)
        run.run([exe,'callbacks','unused',directory/'callbacks.json'],prefix+'callbacks',env=env)
        for row in cases['timing']:
            run.run([exe,'fit',dest/'inputs'/row['par'],dest/'inputs'/row['tim'],
                int(row['rescale']),directory/(row['name']+'.json')],prefix+row['name'],env=env)
    campaign(out/'export_reference',reference,'')
    # Decision 0104: successor observer, never edit the historical exporter or
    # replace its raw outputs. Preserve the sign present in the actual value.
    exporter=ROOT/'validation/fitting/export_reference.cc';original=exporter.read_text()
    old='if (isnanq(x)) return "\\\"NaN\\\"";'
    assert original.count(old)==1
    corrected=original.replace(old,'if (isnanq(x)) return signbitq(x) ? "\\\"-NaN\\\"" : "\\\"NaN\\\"";')
    observer=out/'signed_nan_exporter.cc';observer.write_text(corrected)
    (out/'observer.patch').write_text(''.join(difflib.unified_diff(original.splitlines(True),corrected.splitlines(True),fromfile=str(exporter),tofile='AL/signed_nan_exporter.cc')))
    command=run.commands[0]['argv'].copy()
    command[command.index(str(exporter))]=str(observer);command[-1]=str(out/'signed_nan_exporter')
    run.run(command,'build-signed-observer')
    successor=out/'signed-reference';successor.mkdir()
    campaign(out/'signed_nan_exporter',successor,'signed-')
    changes={p.name:observer_differences(json.loads(p.read_text()),json.loads((successor/p.name).read_text()))
             for p in reference.glob('*.json') if p.stat().st_size}
    control=out/'observer_control.cc'
    control.write_text('#define main historical_main\n#include "signed_nan_exporter.cc"\n#undef main\n'
        'int main() { longdouble n=nanq(""); longdouble v[]={n,-n,0.0Q,-0.0Q,HUGE_VALQ,-HUGE_VALQ,1.25Q,-1.25Q}; '
        'for(auto x:v) puts(number(x).c_str()); }\n')
    command[command.index(str(observer))]=str(control);command[-1]=str(out/'observer_control')
    run.run(command,'build-observer-control')
    values=run.run([out/'observer_control'],'observer-control').decode().splitlines()
    assert values==['"NaN"','"-NaN"','"0"','"-0"','"+Infinity"','"-Infinity"','"1.25"','"-1.25"'],values
    run.save({'status':'pass','scope':'M2 native preparation; numerical comparison separate',
        'regenerated_input_differences':differences,'regenerated_inputs':regenerated,
        'commands_from_catalogue':m.COMMANDS,'generator':sha(source),'a2_receipt':sha(a2/'receipt.json'),
        'outputs':{p.name:sha(p) for p in reference.iterdir() if p.is_file()},
        'signed_observer':{'source':sha(observer),'historical_source':sha(exporter),
            'patch':sha(out/'observer.patch'),'sign_changes':changes,'controls':values,
            'outputs':{p.name:sha(p) for p in successor.iterdir() if p.is_file()}}})


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','a2','candidate']:p.add_argument('--'+name,type=Path,required=True)
    a=p.parse_args();prepare(a.out,a.a2.resolve(),a.candidate.resolve())
