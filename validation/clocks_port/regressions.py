#!/usr/bin/env python3
"""Redirect existing I/M1/M2 runners, preserving all projections and budgets."""
import importlib.util,json,os,shutil,subprocess,sys
from pathlib import Path
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent;ROOT=HERE.parent.parent;RUST=ROOT/'rust';OUT=RUST/'verification/clocks/glob_correction'
def module(name,path):
    spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec);sys.modules[name]=m;spec.loader.exec_module(m);return m

def main():
    stage=sys.argv[1]
    port=module('run',ROOT/'validation/jpl_port/run.py');port.OUT=OUT/'jpl';port.BUILD=HERE/'build/jpl'
    port.OUT.mkdir(parents=True,exist_ok=True)
    shutil.copyfile(RUST/'verification/jpl/title_scan_source_evidence.json',port.OUT/'title_scan_source_evidence.json')
    os.environ.update(port.ENV)
    if stage=='jpl':
        sys.argv=[str(ROOT/'validation/jpl_port/run.py'),'--profile',sys.argv[2]];port.main();return
    if stage in ('m1a','m1b','m2'):
        runner=module('existing_'+stage,RUST/('verify.py' if stage=='m1a' else 'verify_'+stage+'.py'))
        runner.OUTPUT=OUT/stage
        sys.argv=[stage]+(['--runtime',str(ROOT/'tempo2/T2runtime')] if stage=='m1b' else [])
        runner.main();return
    raise ValueError(stage)
if __name__=='__main__':main()
