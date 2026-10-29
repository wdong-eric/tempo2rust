#!/usr/bin/env python3
"""Portable successor to AK's complete worker calibration and matched timings."""
import argparse
import difflib
import json
import os
from pathlib import Path
import platform
import shutil
import sys
from common import ROOT, Run, sha


def execute(args):
    run=Run(args.out);out=run.out;core=args.core.resolve();a2=args.a2.resolve()
    shutil.copyfile(__file__,out/'producer.py')
    cxx=shutil.which(os.environ.get('NATIVE_CXX','g++'));assert cxx
    identity=json.loads((core/'receipt.json').read_text())
    assert identity['profile']=='disabled' and identity['a2_receipt']==sha(a2/'receipt.json')
    assert identity['a2_archive']==sha(a2/'libtempo2-reference.a')
    for name,digest in identity['objects'].items():assert sha(core/name)==digest,name
    scripts=out/'scripts';scripts.mkdir();patches=[];sources={}
    shutil.copyfile(ROOT/'validation/release/process_usage.py',scripts/'process_usage.py')
    sources['process_usage.py']={'executed':sha(scripts/'process_usage.py')}
    def stage(name,replacements=()):
        original=(ROOT/'validation/parallel_execution'/name).read_text();updated=original
        for before,after in [('Path(__file__).resolve().parents[2]',repr(ROOT)),
                             ("ROOT/'validation/reference/build/source/T2runtime'",repr(a2/'runtime')),
                             ("'/opt/homebrew/bin/g++-16'",repr(cxx)),*replacements]:
            updated=updated.replace(before,str(after))
        if platform.system()=='Linux':
            updated=updated.replace("'-Wl,-map,'","'-Wl,-Map,'")
            updated=updated.replace("['otool','-L',exe]","['ldd',exe]")
            updated=updated.replace("['/usr/bin/time','-l'","['/usr/bin/time','-v'")
        if name=='performance.py':
            before="m=re.search(r'([\\d.]+) real\\s+([\\d.]+) user\\s+([\\d.]+) sys',raw);rss=int(re.search(r'(\\d+)\\s+maximum resident set size',raw)[1])"
            assert before in updated
            updated='from process_usage import parse_usage\n'+updated.replace(before,'cpu,rss=parse_usage(raw,'+repr(platform.system())+')')
            updated=updated.replace('process_cpu_s=float(m[2])+float(m[3])','process_cpu_s=cpu')
        # repr(Path) uses PosixPath, which these historical recipes do not import.
        updated=updated.replace('PosixPath(', 'Path(')
        path=scripts/name;path.write_text(updated)
        patches.extend(difflib.unified_diff(original.splitlines(True),updated.splitlines(True),fromfile='AK/'+name,tofile='AL/'+name))
        sources[name]={'original':sha(ROOT/'validation/parallel_execution'/name),'executed':sha(path)}
        return path
    native=stage('native.py',[
        ("ROOT/'tempo2'",repr(args.native_repository.resolve())),
        ("REF=ROOT/'validation/reference/build'",'REF='+repr(a2)),
        ("receipt=ROOT/'validation/native_backends/build/profiles-final-v3/disabled-release/native/identity.json'",'receipt='+repr(core/'receipt.json')),
        ("for name,digest in identity['objects'].items():", "for name,digest in identity['objects'].items():\n  name=str(Path("+repr(str(core))+')/name)'),
        ("identity['archive']","identity['a2_archive']"),
        ("main=next(Path(c[1]) for c in identity['commands'] if len(c)>2 and c[-1].endswith('/tempo2') and c[1].endswith('tempo2.C.o'))",
         "main=out/'tempo2.C.o';run('main',[CXX,*flags,'-c',"+repr(str(core/'source/tempo2.C'))+",'-o',main])")])
    # Keep the authored worker hook and fixture transformations byte-identical.
    shutil.copyfile(ROOT/'validation/parallel_execution/native_workers.cc',scripts/'native_workers.cc')
    run.run([sys.executable,native,out/'calibration'],'native-calibration')
    timed=stage('native_timed.py',[("ROOT/'validation/reference/build/source'",repr(a2/'source'))])
    run.run([sys.executable,timed,out/'calibration',out/'timed'],'native-timed')
    crate=out/'consumer';bins=crate/'src/bin';bins.mkdir(parents=True)
    installed=args.prefix.resolve()/'share/tempo2-rust/rust'
    (crate/'Cargo.toml').write_text('[package]\nname="al-workers"\nversion="0.1.0"\nedition="2024"\n[dependencies]\ntempo2-rust={path='+json.dumps(str(installed))+'}\nserde_json="=1.0.151"\n')
    for name,source in [('cli',ROOT/'validation/parallel_execution/cli.rs'),('parallel_execution',ROOT/'rust/examples/parallel_execution.rs')]:
        shutil.copyfile(source,bins/(name+'.rs'));sources[name]={'source':str(source),'sha256':sha(source)}
    run.run(['cargo','generate-lockfile','--offline','--manifest-path',crate/'Cargo.toml'],'consumer-lock')
    run.run(['cargo','build','--offline','--locked','--release','--manifest-path',crate/'Cargo.toml','--target-dir',crate/'target'],'consumer-build')
    client=crate/'target/release'
    qualification=stage('qualify.py')
    run.run([sys.executable,qualification,out/'qualification','--native',out/'calibration','--rust',client/'parallel_execution'],'qualification',expected=None)
    performance=stage('performance.py')
    run.run([sys.executable,performance,out/'performance','--native',out/'timed','--cli',client/'cli','--rust',client/'parallel_execution','--inputs',out/'qualification/inputs'],'performance',expected=None)
    control=stage('timed_control.py')
    run.run([sys.executable,control,out/'performance',out/'calibration',out/'timed-control'],'timed-control',expected=None)
    (out/'platform-adaptations.patch').write_text(''.join(patches))
    failed=[c['label'] for c in run.commands if c['status']]
    run.save({'status':'failure' if failed else 'pass','failed':failed,'sources':sources,
              'core_receipt':sha(core/'receipt.json'),'a2_receipt':sha(a2/'receipt.json'),
              'installed_manifest':sha(installed/'Cargo.toml'),'lockfile':sha(crate/'Cargo.lock'),
              'adaptations':sha(out/'platform-adaptations.patch'),'runner':sha(out/'producer.py'),
              'scope':'Complete AK eleven-case calibration and six-workload, three-sample 1/2/4-worker timings; no optimization or performance gate'})
    return bool(failed)


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','core','a2','prefix','native-repository']:p.add_argument('--'+name,type=Path,required=True)
    raise SystemExit(execute(p.parse_args()))
