#!/usr/bin/env python3
"""Isolated execution/provenance for TEMPO2 binary models and formResiduals.
No standalone TEMPO2 counterpart: orchestrates export_binary.cc and A2 resource trace.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import subprocess
import tempfile
from prepare_fixtures import MODELS, save, sha

ROOT=Path(__file__).resolve().parent
PROJECT=ROOT.parents[1]
A2=PROJECT/'validation/reference'
BUILD=ROOT/'build'
RUNTIME=A2/'build/source/T2runtime'
SOURCE_PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
A2_PIN='7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572'
COMPILER='/opt/homebrew/bin/g++-16'


def normalize(text):
    return text.replace(str(RUNTIME),'$RUNTIME').replace(str(A2/'build/source'),'$A2_SOURCE').replace(str(ROOT),'$BINARY').replace(str(A2),'$A2')


def verify_build():
    for repo,pin in [(PROJECT/'tempo2',SOURCE_PIN),(A2,A2_PIN)]:
        assert subprocess.check_output(['git','-C',str(repo),'rev-parse','HEAD'],text=True).strip()==pin
        assert not subprocess.check_output(['git','-C',str(repo),'status','--porcelain'],text=True).strip()
    manifest=json.loads((A2/'build_manifest.json').read_text())
    assert sha(A2/'build/libtempo2-reference.a')==manifest['static_library_sha256']
    for filename,digest in manifest['linked_dependency_hashes'].items():assert sha(Path(filename))==digest
    source_files=[m+'model.C' for m in MODELS]+['formResiduals.C','t2fit_stdFitFuncs.C','readParfile.C','initialise.C','formBats.C','readTimfile.C','calculate_bclt.C','readEphemeris.C','vectorPulsar.C','tempo2.h','TKlongdouble.float128.h','ifunc.C']
    # Reconstruct the accepted changes in our own scratch tree, never in either reference repo.
    patchfiles=['initializer-correction.patch','instrumentation.patch']
    changed=set()
    for filename in patchfiles:
        changed.update(re.findall(r'^--- (?:a/)?([^\t\n ]+)',(A2/filename).read_text(),re.M))
    with tempfile.TemporaryDirectory(dir=BUILD,prefix='verify-source-') as tmp:
        tmp=Path(tmp)
        for name in changed:
            (tmp/name).parent.mkdir(parents=True,exist_ok=True)
            (tmp/name).write_bytes(subprocess.check_output(['git','-C',str(PROJECT/'tempo2'),'show',SOURCE_PIN+':'+name]))
        for filename in patchfiles:
            cmd=['patch','--batch','-p1' if filename.startswith('initializer') else '-p0','-i',str(A2/filename)]
            subprocess.run(cmd,cwd=tmp,check=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
        source_hashes={}
        for name in sorted(set(source_files)|changed):
            canonical=PROJECT/'tempo2'/name
            raw=subprocess.check_output(['git','-C',str(PROJECT/'tempo2'),'show',SOURCE_PIN+':'+name])
            expected=(tmp/name).read_bytes() if name in changed else raw
            assert re.sub(rb'\$Id:[^$]*\$',b'$Id$',canonical.read_bytes())==raw,name
            built=A2/'build/source'/name
            assert (re.sub(rb'\$Id:[^$]*\$',b'$Id$',built.read_bytes()) if name.endswith('.h') else built.read_bytes())==expected,name
            source_hashes[name]={'canonical_checkout_sha256':sha(canonical),'pinned_blob_sha256':hashlib.sha256(raw).hexdigest(),'a2_sha256':sha(built),'approved_change':name in changed}
    assert (A2/'build/source/TKlongdouble.h').read_bytes()==(A2/'build/source/TKlongdouble.float128.h').read_bytes()
    return {'source_commit':SOURCE_PIN,'a2_revision':A2_PIN,'a2_manifest_sha256':sha(A2/'build_manifest.json'),
            'library_sha256':manifest['static_library_sha256'],'source_hashes':source_hashes,
            'initializer_correction_sha256':sha(A2/'initializer-correction.patch'),
            'instrumentation_sha256':sha(A2/'instrumentation.patch'),'linked_dependencies':manifest['linked_dependency_hashes'],
            'precision':manifest['precision_probe']}


def resources(trace):
    events=[];files={}
    if trace.exists():
        for line in trace.read_text().splitlines():
            stage,name,mode,opened,error,caller=line.split('\t')
            path=Path(name).resolve();key=normalize(str(path))
            events.append({'stage':stage,'path':key,'mode':mode,'opened':opened=='1','errno':int(error),'caller':Path(caller).name})
            if opened=='1' and mode.startswith('r') and key not in files:files[key]={'sha256':sha(path),'bytes':path.stat().st_size}
    return {'events':events,'files':files}


def run_case(case,outdir,identity):
    name=case['name'];inputs=ROOT/'fixtures'
    paths=[inputs/case[k] for k in ['par','tim','requests','labels']]
    with tempfile.TemporaryDirectory(prefix=name+'-',dir=BUILD) as tmp:
        work=Path(tmp);raw=work/'result.jsonl';trace=work/'resources.tsv'
        cmd=[str(BUILD/'export_binary'),*map(str,paths),str(raw)]
        env={**os.environ,'TEMPO2':str(RUNTIME),'REFERENCE_RESOURCE_LOG':str(trace),'LC_ALL':'C'}
        try:
            proc=subprocess.run(cmd,cwd=work,env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,timeout=20)
            code=proc.returncode;log=proc.stdout;outcome='returned' if code==0 else 'source_exit' if code>0 else 'source_signal'
        except subprocess.TimeoutExpired as e:
            code=None;log=(e.stdout or b'').decode(errors='replace');outcome='timeout'
        records=[json.loads(line) for line in raw.read_text().splitlines()] if raw.exists() else []
        if code==0:
            assert len(records)>=2 and records[0]['record']=='parsed' and records[1]['record']=='prepared',name
            requests=[line.split() for line in paths[2].read_text().splitlines()]
            assert sum(r['record']=='result' for r in records)==sum(int(r[4]) for r in requests),name
        result={'schema_version':1,'case':case,'process':{'outcome':outcome,'returncode':code,'signal':-code if code is not None and code<0 else None},
                'identity':identity,'inputs':{p.name:sha(p) for p in paths},'resources':resources(trace),'records':records,
                'diagnostics':normalize(log)}
        save(outdir/(name+'.json'),result)
        (outdir/(name+'.log')).write_text(normalize(log))
        classes=sorted({r.get('class','withheld') for r in records if r['record']=='result' and 'source_semantics' in r})
        print(name,outcome,','.join(classes),flush=True)


def main():
    parser=argparse.ArgumentParser();parser.add_argument('--output',type=Path,default=ROOT/'outputs');parser.add_argument('--case',action='append');args=parser.parse_args()
    BUILD.mkdir(exist_ok=True);args.output.mkdir(parents=True,exist_ok=True)
    identity=verify_build()
    assert {p.name:sha(p) for p in sorted((ROOT/'fixtures').iterdir()) if p.is_file()}==json.loads((ROOT/'input_hashes.json').read_text())
    cmd=[COMPILER,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(A2/'build/source'),'-I'+str(A2),str(ROOT/'export_binary.cc'),str(A2/'build/libtempo2-reference.a'),'-lquadmath','-lgfortran','-o',str(BUILD/'export_binary')]
    build=subprocess.run(cmd,cwd=BUILD,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True)
    (BUILD/'compile.log').write_text(build.stdout);assert build.returncode==0,build.stdout
    identity.update(exporter_sha256=sha(ROOT/'export_binary.cc'),executable_sha256=sha(BUILD/'export_binary'))
    cases=json.loads((ROOT/'cases.json').read_text())
    for case in cases:
        if args.case is None or case['name'] in args.case:run_case(case,args.output,identity)
    if args.case is None:
        manifest={'identity':identity,'platform':platform.platform(),'compiler':subprocess.check_output([COMPILER,'--version'],text=True),
                  'build_command':[normalize(x) for x in cmd],'cases_sha256':sha(ROOT/'cases.json'),'input_hashes_sha256':sha(ROOT/'input_hashes.json'),
                  'inventory_sha256':sha(ROOT/'source_inventory.json'),'outputs':{p.name:sha(p) for p in sorted(args.output.glob('*.json')) if p.name!='manifest.json'}}
        save(args.output/'manifest.json',manifest)

if __name__=='__main__':main()
