#!/usr/bin/env python3
"""TEMPO2 counterpart: tests/testEndToEnd.C::checkIdeal, unchanged 1 ns test.
Rebuild only the test driver to relocate its compile-time DATDIR; reuse A2 library.
"""
import json
import os
import re
import shutil
import subprocess
from decimal import Decimal, getcontext
from pathlib import Path
from prepare_fixtures import sha, save
from run_reference import ROOT, PROJECT, A2, BUILD, RUNTIME, COMPILER, normalize, resources, verify_build


def main():
    getcontext().prec=90
    identity=verify_build();work=BUILD/'upstream';work.mkdir(exist_ok=True)
    source=PROJECT/'tempo2/tests/testEndToEnd.C'
    assert source.read_bytes()==(A2/'build/source/tests/testEndToEnd.C').read_bytes()
    data=A2/'build/source/tests/test_data'
    for name in ['test3.par','test3.tim']:
        assert (data/name).read_bytes()==(PROJECT/'tempo2/tests/test_data'/name).read_bytes()
        assert (ROOT/'fixtures'/('original_'+name)).read_bytes()==(data/name).read_bytes()
        shutil.copyfile(ROOT/'fixtures'/('original_'+name),work/name)
    shutil.copyfile(source,work/'testEndToEnd.C')
    gtest=A2/'build/source/tests/gtest-1.7.0'
    flags=[COMPILER,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(A2/'build/source'),'-I'+str(gtest),'-I'+str(gtest/'include'),'-DTEST_DELTA=1e-9l','-DDATDIR="'+str(work)+'"','-pthread']
    # gtest is rebuilt here: no unverified auxiliary object is borrowed from A2.
    commands=[flags+['-c',str(gtest/'src/gtest-all.cc'),'-o',str(work/'gtest.o')],
              flags+[str(work/'testEndToEnd.C'),str(work/'gtest.o'),str(A2/'build/libtempo2-reference.a'),'-lquadmath','-lgfortran','-o',str(work/'testEndToEnd')]]
    for command in commands:
        p=subprocess.run(command,cwd=work,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True)
        assert p.returncode==0,p.stdout
    trace=work/'resources.tsv';trace.unlink(missing_ok=True)
    command=[str(work/'testEndToEnd'),'--gtest_filter=testEndToEnd.checkIdeal']
    p=subprocess.run(command,cwd=work,env={**os.environ,'TEMPO2':str(RUNTIME),'REFERENCE_RESOURCE_LOG':str(trace),'LC_ALL':'C'},stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,timeout=60)
    # Elapsed test-run milliseconds are not scientific evidence and vary by load.
    log=re.sub(r'\(\d+ ms', '(ELAPSED ms',normalize(p.stdout))
    historical=json.loads((A2/'checks/ddh.json').read_text())
    vals=list(map(Decimal,historical['all_residuals_s']))
    result={'purpose':'upstream ideal-test behavior; independent of future port agreement','threshold_s':'1e-9','returncode':p.returncode,
            'source_sha256':sha(source),'binary_sha256':sha(work/'testEndToEnd'),'identity':identity,
            'inputs':{name:sha(work/name) for name in ['test3.par','test3.tim']},'resources':resources(trace),
            'build_commands':[[normalize(a) for a in c] for c in commands],
            'test_command':[normalize(a) for a in command],'log':log,
            'historical_a2_localization':{'artifact_sha256':sha(A2/'checks/ddh.json'),'n_toas':len(vals),'n_outside_threshold':sum(abs(x)>=Decimal('1e-9') for x in vals),'first_failure_index':next(i for i,x in enumerate(vals) if abs(x)>=Decimal('1e-9')),'max_abs_residual_s':str(max(map(abs,vals))),'recomputed_here':False}}
    assert p.returncode==1 and '1.13759e-09' in log and '1e-09' in log,log
    save(ROOT/'upstream_ddh.json',result)
    print('Unchanged DDH checkIdeal failed at original 1 ns threshold, as expected.')

if __name__=='__main__':main()
