#!/usr/bin/env python3
"""Assignment I exact H projection and Rust execution; never writes H evidence."""
import argparse
import copy
import hashlib
import fnmatch
from fractions import Fraction
import importlib.util
import json
import os
from pathlib import Path
import platform
import shutil
import struct
import subprocess
import sys
sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
RUST = ROOT/'rust'
H = ROOT/'validation/jpl'
OUT = RUST/'verification/jpl'
BUILD = HERE/'build'  # Importing regressions may redirect only output/build storage.
ENV = dict(os.environ,CARGO_HOME=str(RUST/'.cargo-home'),GMP_MPFR_SYS_CACHE=str(RUST/'target/gmp-cache'))

def load(path): return json.loads(path.read_text())
def save(path,value):
    path.parent.mkdir(parents=True,exist_ok=True)
    path.write_text(json.dumps(value,indent=2,sort_keys=True)+'\n')
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
def module(name,path):
    spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m

def compare_value(a,b,path,counts):
    assert type(a)==type(b),f'{path}: type {type(a)} != {type(b)}'
    if isinstance(b,dict):
        assert set(a)==set(b),f'{path}: fields {set(a)^set(b)}'
        if 'class' in b:
            assert a['class']==b['class'],f'{path}: class {a} != {b}'
            if b['class']=='finite':
                for v in (a,b): assert struct.pack('>d',float(v['decimal'])).hex()==v['bits'],f'{path}: decimal roundtrip'
                assert a['bits']==b['bits'],f'{path}: bits {a} != {b}'
                counts['finite']+=1
            else:
                assert a['decimal']==b['decimal'],f'{path}: class spelling'
                counts['nonfinite']+=1
            return
        for k in b: compare_value(a[k],b[k],path+'.'+k,counts)
    elif isinstance(b,list):
        assert len(a)==len(b),f'{path}: component count {len(a)} != {len(b)}'
        for i in range(len(b)): compare_value(a[i],b[i],f'{path}[{i}]',counts)
    else:
        assert a==b,f'{path}: {str(a)[:140]} != {str(b)[:140]}'
        counts['discrete']+=1

def compare(actual,reference,projection):
    assert set(actual)==set(reference)==set(projection['scenarios']),'scenario set'
    if 'scenario_count' in projection:
        assert len(reference)==projection['scenario_count']==55,'declared scenario count'
        assert sum(map(len,reference.values()))==projection['event_count']==1443,'declared reference event count'
        assert [e['op'] for e in reference['title_zero']]==['startup','open','control','state','close'],'title reference order'
    abi={'struct_size','double_align','pvsun_offset','macro_offset','au_offset','iinfo_offset','ifile_offset'}
    counts=dict(finite=0,nonfinite=0,discrete=0,reference_events=0,exact_events=0,boundary_events=0,unreachable_events=0,executed_events=0)
    for name,events in reference.items():
        rules=projection['scenarios'][name]
        assert len(events)==len(rules),f'{name}: reference event count'
        expected_actual=2 if name=='title_zero' else len(events)
        assert len(actual[name])==expected_actual,f'{name}: Rust event count'
        counts['executed_events']+=len(actual[name])
        for i in range(len(events)):
            b,rule=events[i],rules[i]
            fields={'op','reference_fields','exact_fields','classifications'}
            if name=='title_zero':fields |= {'reference_event','reference_event_sha256'}
            if name=='title_zero' and i:fields |= {'mode','reason'}
            assert set(rule)==fields,f'{name}/{i}: projection fields'
            assert sorted(b)==rule['reference_fields'],f'{name}/{i}: reference fields'
            assert b['op']==rule['op'],f'{name}/{i}: reference operation'
            expected_mode=('boundary' if i==1 else 'unreachable') if name=='title_zero' and i else 'exact'
            assert rule.get('mode','exact')==expected_mode,f'{name}/{i}: unapproved classification'
            assert rule.get('classifications',{})=={k:'reference_C_ABI_only' for k in sorted(set(b)&abi)},f'{name}/{i}: ABI classifications'
            if name=='title_zero':
                assert rule['reference_event']==b.get('event'),f'{name}/{i}: reference identity'
                digest=hashlib.sha256(json.dumps(b,sort_keys=True,separators=(',',':')).encode()).hexdigest()
                assert rule['reference_event_sha256']==digest,f'{name}/{i}: reference contents'
            counts['reference_events']+=1
            if expected_mode=='unreachable':
                assert rule['exact_fields']==[] and rule['reason']=='Decision 0019: no handle after the title_conversion boundary'
                counts['unreachable_events']+=1
                continue
            a=actual[name][i]
            if expected_mode=='boundary':
                assert rule['exact_fields']==[] and rule['reason']=='Decision 0019: atoi whitespace scan leaves the 84-byte title'
                expected=dict(event=0,op='open',boundary='title_conversion',init_error=0,io_calls=[1,2,0],last_fread_size_requested_returned=[204,1,1])
                compare_value(a,expected,f'{name}/{i}',counts)
                counts['boundary_events']+=1
                continue
            assert rule['exact_fields']==sorted(set(b)-abi),f'{name}/{i}: exact projection fields'
            assert sorted(a)==rule['exact_fields'],f'{name}/{i}: Rust fields {set(a)^set(rule["exact_fields"])}'
            assert a['op']==b['op'],f'{name}/{i}: event operation'
            compare_value(a,{k:b[k] for k in rule['exact_fields']},f'{name}/{i}',counts)
            counts['exact_events']+=1
    return counts

def execute(exe,case,inputs,name):
    for src,dst in case.get('setup',[]): shutil.copyfile(inputs/src,inputs/dst)
    commands=inputs.parent/'commands.txt'; output=inputs.parent/'observed.json'
    commands.write_text('\n'.join(case['commands'])+'\n')
    output.unlink(missing_ok=True)
    p=subprocess.run([str(exe),'--exact','jpleph::tests::probe','--ignored','--nocapture'],env=dict(ENV,JPL_COMMANDS=str(commands),JPL_OUTPUT=str(output)),capture_output=True,text=True)
    assert p.returncode==0,(name,p.stdout,p.stderr)
    assert 'test jpleph::tests::probe ... ok' in p.stdout and '1 passed; 0 failed' in p.stdout, (name,'intended probe did not run',p.stdout)
    assert output.is_file(), (name,'missing fresh probe output')
    return load(output)

def isolated_compare(observed,specs,rules):
    assert set(observed)==set(specs)==set(rules),'isolated case set'
    results={}
    for name,spec in specs.items():
        events=observed[name];rule=rules[name]
        assert len(events)==len(spec['commands'])+1,(name,'isolated event coverage')
        assert events[0]['op']=='startup'
        for i,command in enumerate(spec['commands']):
            assert events[i+1]['op']==command.split()[0] and events[i+1]['event']==i,(name,'isolated order')
        assert all('boundary' not in e for e in events[:-1]),(name,'earlier boundary')
        final=events[-1]
        if rule['result']=='boundary':
            assert final.get('boundary')==rule['boundary'],(name,'boundary disposition',final)
            assert set(final)==({'event','op','boundary','io_calls','last_fread_size_requested_returned'} | ({'init_error'} if final['op']=='open' else set())),(name,'fabricated boundary output')
        else:
            assert 'boundary' not in final,(name,'unexpected boundary')
            if rule['result']=='safe_decoded_getter':
                compare_value(final['value'],events[1]['header_doubles'][rule['header_index']],name,dict(finite=0,nonfinite=0,discrete=0))
            else:assert final['status']==rule['status'],(name,'safe status')
        results[name]=dict(reference_failure_kind=spec['failure_kind'],reference_last_completed_op=spec['last_completed_op'],rust_last_completed_op=final['op'],disposition=rule,status='PASS')
    return results

def analytic(actual,cases):
    polynomial=module('h_polynomial',H/'analyze.py').chebyshev
    errors=[0.0]*3;count=0
    for command,event in zip(cases['helper_interpolation']['commands'],actual['helper_interpolation'][1:],strict=True):
        if not command.startswith('interp '):continue
        _,fraction,step,ncf,ncm,na,velocity=command.split()
        fraction,step=Fraction(fraction),Fraction(step);ncf,ncm,na,velocity=map(int,(ncf,ncm,na,velocity))
        if not step:continue
        temp=fraction*na;sub=int(temp);tc=2*(temp-sub)-1
        if sub==na:sub-=1;tc=Fraction(1)
        for d in range(velocity):
            for component in range(ncm):
                expected=Fraction(0)
                for k in range(ncf):
                    i=ncf*(sub*ncm+component)+k
                    expected+=Fraction((-1 if i%7==0 else 1)*(i+1),16)*polynomial(k,tc,d)
                expected*=Fraction(na*2,step)**d
                observed=float(event['out'][d*ncm+component]['decimal'])
                errors[d]=max(errors[d],abs(observed-float(expected)));count+=1
    assert count==576 and errors==[0.0]*3,(count,errors)
    return dict(components=count,max_absolute_difference_position_velocity_acceleration=errors,method='H exact rational explicit polynomial and analytic derivatives')

def manifests():
    checks=load(H/'outputs/checks.json');pin=checks['source_commit']
    for name,digest in checks['source_sha256'].items():
        blob=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',f'{pin}:{name}'])
        assert hashlib.sha256(blob).hexdigest()==digest==sha(ROOT/'tempo2'/name)
    for name,digest in checks['harness_sha256'].items():assert sha(H/name)==digest,name
    for name,digest in load(RUST/'native/upstream_manifest.json')['files'].items():assert sha(RUST/'native'/name)==digest,name
    files=[p for base in [RUST/'src',RUST/'tests',RUST/'examples',RUST/'native'] for p in base.rglob('*') if p.is_file() and '__pycache__' not in p.parts]
    files += [RUST/n for n in ['Cargo.toml','Cargo.lock','build.rs','jpl_source_manifest.json']]
    files += [p for p in HERE.iterdir() if p.is_file()]
    files += [ROOT/'decisions'/n for n in ['0017-jpl-rust-contract-and-i-release.md','0019-jpl-title-boundary-projection.md']]
    files += [OUT/'title_scan_source_evidence.json']
    files += [p for p in H.rglob('*') if p.is_file() and 'build' not in p.parts and '__pycache__' not in p.parts]
    return dict(executed_git_head=subprocess.check_output(['git','-C',str(ROOT),'rev-parse','HEAD'],text=True).strip(),source_commit=pin,sha256={str(p.relative_to(ROOT)):sha(p) for p in sorted(files)})

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--profile',choices=['debug','release'],default='debug');args=parser.parse_args()
    OUT.mkdir(parents=True,exist_ok=True)
    executed=manifests()
    h=module('h_fixture_generator',H/'run.py'); h.BUILD=BUILD/args.profile;h.INPUTS=h.BUILD/'inputs'
    h.INPUTS.mkdir(parents=True,exist_ok=True)
    manifest=h.inputs();assert manifest==load(H/'inputs.json'),'H input identity'
    cases=h.scenarios(manifest)
    portable=json.loads(json.dumps(cases).replace(str(h.INPUTS),'$INPUTS').replace(str(h.REAL),'$REAL'))
    assert portable==load(H/'cases.json'),'H command identity'
    cmd=['cargo','test','--lib','--no-run','--locked','--offline','--message-format=json']+(['--release'] if args.profile=='release' else [])
    p=subprocess.run(cmd,cwd=RUST,env=ENV,capture_output=True,text=True)
    (OUT/f'{args.profile}_build.log').write_text(p.stderr)
    assert p.returncode==0,p.stderr
    built=[json.loads(s) for s in p.stdout.splitlines() if s.startswith('{')]
    exe=Path(next(x['executable'] for x in built if x.get('executable') and x['target']['kind']==['lib']))
    ownership={}
    for test in ['stdio::tests::descriptor_transfer_partial_items_and_failure_cleanup','jpleph::tests::title_scan_boundaries_close_stream_and_allow_reinitialization']:
        command=[str(exe),'--exact',test,'--ignored','--test-threads=1']
        p=subprocess.run(command,env=ENV,capture_output=True,text=True)
        assert p.returncode==0 and f'test {test} ... ok' in p.stdout and '1 passed; 0 failed' in p.stdout,(test,p.stdout,p.stderr)
        ownership[test]=dict(command=command,returncode=p.returncode,stdout=p.stdout,stderr=p.stderr)
    save(OUT/f'{args.profile}_ownership.json',ownership)
    actual={name:execute(exe,case,h.INPUTS,name) for name,case in cases.items()}
    save(OUT/f'{args.profile}_raw.json',actual)
    reference=load(H/'outputs/reference.json');projection=load(HERE/'projection.json')
    assert sha(H/'outputs/reference.json')==projection['reference_sha256']
    counts=compare(actual,reference,projection)
    specs=load(H/'outputs/isolated.json')['cases'];rules=load(HERE/'isolated_projection.json')
    isolated={name:execute(exe,{'commands':[c.replace(str(H/'build/inputs'),str(h.INPUTS)) for c in spec['commands']]},h.INPUTS,name) for name,spec in specs.items()}
    save(OUT/f'{args.profile}_isolated_raw.json',isolated)
    save(OUT/f'{args.profile}_isolated_comparison.json',isolated_compare(isolated,specs,rules))
    save(OUT/f'{args.profile}_analytic.json',analytic(actual,portable))
    module('supplemental',HERE/'supplemental.py').run(h,exe,args.profile)
    coverage=load(H/'coverage.json');mapping={}
    for name,spec in coverage['functions'].items():
        ordinary=sorted({case for pattern in spec['cases'] for case in cases if fnmatch.fnmatchcase(case,pattern)})
        assert ordinary and all(case in isolated for case in spec['isolated']),name
        mapping[name]=dict(source=spec['source'],ordinary=ordinary,isolated=spec['isolated'])
    assert len(mapping)==12
    save(OUT/f'{args.profile}_coverage.json',dict(functions=mapping,scenarios=len(cases),reference_events=counts['reference_events'],exact_events=counts['exact_events'],boundary_events=counts['boundary_events'],unreachable_events=counts['unreachable_events'],isolated=len(isolated)))
    assert executed['sha256']==manifests()['sha256'],'inputs changed during execution'
    save(OUT/f'{args.profile}_source_hashes.json',executed)
    save(OUT/f'{args.profile}_comparison.json',dict(status='PASS',platform=platform.platform(),machine=platform.machine(),counts=counts,finite_maximum_difference=0,executable_sha256=sha(exe)))
    print(args.profile,counts,flush=True)

if __name__=='__main__':main()
