#!/usr/bin/env python3
"""Assignment I exact H projection and Rust execution; never writes H evidence."""
import argparse
import copy
import hashlib
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
    counts=dict(finite=0,nonfinite=0,discrete=0,events=0)
    for name,events in reference.items():
        rules=projection['scenarios'][name]
        assert len(actual[name])==len(events)==len(rules),f'{name}: event count'
        for i in range(len(events)):
            a,b,rule=actual[name][i],events[i],rules[i]
            assert sorted(b)==rule['reference_fields'],f'{name}/{i}: reference fields'
            assert sorted(a)==rule['exact_fields'],f'{name}/{i}: Rust fields {set(a)^set(rule["exact_fields"])}'
            assert a['op']==b['op']==rule['op'],f'{name}/{i}: event operation'
            compare_value(a,{k:b[k] for k in rule['exact_fields']},f'{name}/{i}',counts)
            counts['events']+=1
    return counts

def execute(exe,case,inputs,name):
    for src,dst in case.get('setup',[]): shutil.copyfile(inputs/src,inputs/dst)
    commands=inputs.parent/'commands.txt'; output=inputs.parent/'observed.json'
    commands.write_text('\n'.join(case['commands'])+'\n')
    p=subprocess.run([str(exe),'--exact','jpleph::tests::probe','--ignored','--nocapture'],env=dict(ENV,JPL_COMMANDS=str(commands),JPL_OUTPUT=str(output)),capture_output=True,text=True)
    assert p.returncode==0,(name,p.stdout,p.stderr)
    return load(output)

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--profile',choices=['debug','release'],default='debug');args=parser.parse_args()
    OUT.mkdir(parents=True,exist_ok=True)
    h=module('h_fixture_generator',H/'run.py'); h.BUILD=HERE/'build'/args.profile;h.INPUTS=h.BUILD/'inputs'
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
    actual={name:execute(exe,case,h.INPUTS,name) for name,case in cases.items()}
    save(OUT/f'{args.profile}_raw.json',actual)
    reference=load(H/'outputs/reference.json');projection=load(HERE/'projection.json')
    assert sha(H/'outputs/reference.json')==projection['reference_sha256']
    counts=compare(actual,reference,projection)
    save(OUT/f'{args.profile}_comparison.json',dict(status='PASS',platform=platform.platform(),machine=platform.machine(),counts=counts,finite_maximum_difference=0,executable_sha256=sha(exe)))
    print(args.profile,counts,flush=True)

if __name__=='__main__':main()
