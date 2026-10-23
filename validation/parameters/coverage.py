#!/usr/bin/env python3
"""Separate GCC coverage build: compare observations first, retain source line counts."""
import concurrent.futures
import gzip
import json
import subprocess
from profile import HERE,BUILD,prepare,compile_probe,save,sha
from run import execute,validate,compare,check_execution_identity


def main():
    cases=json.loads((HERE/'cases.json').read_text());gold=json.loads((HERE/'observations.json').read_text())
    for p in BUILD.glob('coverage-*.gcda'):p.unlink()
    exe,profile=compile_probe(prepare(),'coverage',coverage=True)
    def one(c):
        r=execute(c,exe,profile);validate(c,r);return r
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
        for i,r in enumerate(pool.map(one,cases)):
            compare(gold[i],r,'coverage.'+r['id'])
            if i%50==0:print(i+1,len(cases),flush=True)
    units={}
    for source in ['initialise','readParfile','TKlongdouble','tempo2Util']:
        command=['/opt/homebrew/bin/gcov-16','--json-format','--branch-probabilities','--branch-counts',str(BUILD/('coverage-'+source+'.gcno'))]
        p=subprocess.run(command,cwd=BUILD,capture_output=True,text=True,check=True)
        path=BUILD/('coverage-'+source+'.gcov.json.gz')
        data=json.loads(gzip.decompress(path.read_bytes()))
        units[source]=dict(command=command,stdout=p.stdout,stderr=p.stderr,data=data)
    save(HERE/'line_coverage.json',dict(profile=profile,execution_identity=check_execution_identity(exe,profile),units=units,ordinary_observation_comparison='exact for every case and field',cases=len(cases)))
    print('PASS: instrumented outputs agree; coverage exported')


if __name__=='__main__':main()
