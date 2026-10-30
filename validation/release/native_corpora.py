#!/usr/bin/env python3
"""Rebuild authored early native corpora in fresh staging on the executing host."""
import argparse
import ast
import difflib
import json
import os
from pathlib import Path
import platform
import re
import shutil
import sys
from common import ROOT, Run, sha

STAGES=['jpl','clocks','parameters','numeric_toolkit','tim_io','preprocessing','par_specialized',
        'pulsar_lifecycle','time_ephemeris','terrestrial_resources','observer_coordinates',
        'planetary_ephemeris','propagation']
HISTORY={
    'jpl':['ddc07b20fafc21fe92f2ecc1dd24c59db9df4142'],
    'par_specialized':['38d62af9c183a0bb3294638999ca63cb431fc046','ad6fbf692751b39c67713fd5a446c675a51db665'],
    'observer_coordinates':['64cab7fc47f72d054a5d3af61441f20f069d9962'],
    'planetary_ephemeris':['7946aeee6e7e105fc276b311f1c36a4288767d0d'],
    'propagation':['b8d6263c1a29231ef45d1a03a7488dce0ed495f3','bd290cc163ad706d8044d5deb2a637c9ed7803e0'],
}
TELESCOPE=['TEL_CLK_OFFS','TEL_DX','TEL_DY','TEL_DZ','TEL_VX','TEL_VY','TEL_VZ']


def bounded_telescope_inputs(source):
    """Apply decision 0072 to new Q inputs, preserving all other fixtures."""
    for family in TELESCOPE:
        old="('"+family+"',1000,"
        assert source.count(old)==1,family
        source=source.replace(old,"('"+family+"',500,")
    return source


def stage_history(run, root, history, source=ROOT):
    """Retain only pinned shallow roots, with no detached metadata writer."""
    if history:run.run(['git','init',root],'history-init')
    for pin in history:
        run.run(['git','-C',source,'cat-file','-e',pin+'^{commit}'],'history-present-'+pin,expected=None)
        repository=source if run.commands[-1]['status']==0 else run.run(['git','-C',source,'remote','get-url','origin'],'history-origin-'+pin).decode().strip()
        # Keep shallow metadata under one writer across sequential fetches (0104).
        # Explicit refs retain earlier roots instead of relying on FETCH_HEAD.
        run.run(['git','-C',root,'fetch','--no-auto-maintenance','--no-tags','--depth=1',
                 repository,pin+':refs/al-history/'+pin],'history-fetch-'+pin,
                env=dict(os.environ,GIT_TRACE2_EVENT=str(run.out/('history-trace-'+pin+'.jsonl'))))
        actual=run.run(['git','-C',root,'rev-parse','refs/al-history/'+pin],'history-verify-'+pin).decode().strip()
        assert actual==pin


def build(args):
    run=Run(args.out);root=run.out/'staging';root.mkdir()
    shutil.copyfile(__file__,run.out/'producer.py')
    native=args.native_repository.resolve();a2=args.a2.resolve()
    receipt=json.loads((a2/'receipt.json').read_text())
    assert sha(a2/'source/config.h')==receipt['source']['config.h']
    assert sha(a2/'libtempo2-reference.a')==receipt['library']
    # The bootstrap repository may be bare. Fixture constructors need the raw
    # pinned working files as well as Git blob access; never use the A2 overlays.
    run.run(['git','init',root/'tempo2'],'native-init')
    run.run(['git','-C',root/'tempo2','fetch','--no-tags','--depth=1',native,receipt['source_pin']],'native-fetch')
    run.run(['git','-C',root/'tempo2','checkout','--detach','FETCH_HEAD'],'native-checkout')
    reference=root/'validation/reference/build';reference.mkdir(parents=True)
    (reference/'source').symlink_to(a2/'source',target_is_directory=True)
    (reference/'libtempo2-reference.a').symlink_to(a2/'libtempo2-reference.a')
    compiler=shutil.which(os.environ.get('NATIVE_CXX','g++'))
    assert compiler,'Selected NATIVE_CXX is unavailable'
    cc=shutil.which(os.environ.get('NATIVE_CC','gcc'));fc=shutil.which(os.environ.get('FC','gfortran'))
    gcov=shutil.which(os.environ.get('NATIVE_GCOV','gcov'))
    changes=[];sources={};stages={}
    # These snapshots contain the original JPL defect witness and Q's accepted
    # parser projection. Fetch just their pinned commits, never all history.
    history=sorted({pin for stage in args.stages for pin in HISTORY.get(stage,[])})
    stage_history(run,root,history)
    def copy_sources(stage):
        directory=root/'validation'/stage;directory.mkdir(exist_ok=True)
        names=run.run(['git','-C',ROOT,'ls-files','-z','--','validation/'+stage],stage+'-sources').decode().split('\0')
        for name in filter(None,names):
            src=ROOT/name;dest=root/name;dest.parent.mkdir(parents=True,exist_ok=True)
            shutil.copyfile(src,dest);sources[name]=sha(src)
        return directory

    def write_adapter(path,updated):
        original=path.read_text();path.write_text(updated)
        changes.extend(difflib.unified_diff(original.splitlines(True),updated.splitlines(True),fromfile='original/'+str(path.relative_to(root)),tofile='portable/'+str(path.relative_to(root))))
        (run.out/'platform-adaptations.patch').write_text(''.join(changes))

    def replace_function(path,name,body):
        original=path.read_text();node=next(n for n in ast.parse(original).body if isinstance(n,ast.FunctionDef) and n.name==name)
        lines=original.splitlines(True)
        write_adapter(path,''.join(lines[:node.lineno-1])+body+'\n'+''.join(lines[node.end_lineno:]))

    def adapt(path):
        # Only build/platform identities change. Fixtures, event validation and
        # scientific probes keep their authored bytes and complete case sets.
        original=path.read_text();updated=original
        updated=updated.replace("shutil.which('g++-16')",repr(compiler))
        updated=updated.replace("'/opt/homebrew/bin/g++-16'",repr(compiler))
        updated=updated.replace('shutil.which("gcc-16")',repr(cc))
        updated=updated.replace("'/opt/homebrew/bin/gcc-16'",repr(cc))
        updated=updated.replace("'/opt/homebrew/bin/gfortran-16'",repr(fc))
        updated=updated.replace("'/opt/homebrew/bin/gcov-16'",repr(gcov)).replace("shutil.which('gcov-16')",repr(gcov))
        updated=updated.replace('470c61b68a61504558e76ef772639e46b565a1e4500115d6e3fb6a71286dd69e',sha(a2/'source/config.h'))
        updated=updated.replace('313507c5063cb6a8156a0e5fd0f752e2baa59a77c78b5b32822d8efeddd2dbf5',sha(a2/'libtempo2-reference.a'))
        if platform.system()=='Linux':
            updated=updated.replace("'-Wl,-map,'","'-Wl,-Map,'")
            updated=updated.replace("'-dynamiclib'","'-shared','-fPIC'")
            updated=updated.replace("['otool','-L',str(exe)],text=True).splitlines()[1:]","['ldd',str(exe)],text=True).splitlines()")
            updated=updated.replace("['otool','-L',str(target)],text=True).splitlines()[1:]","['ldd',str(target)],text=True).splitlines()")
            updated=updated.replace(r"\bT _",r"\bT ")
            updated=updated.replace("['sw_vers']","['uname','-a']")
            updated=updated.replace("subprocess.check_output(['xcrun','-f',program],text=True).strip()","shutil.which(program)")
            updated=updated.replace("subprocess.check_output(['xcrun','-f',prog],text=True).strip()","shutil.which(prog)")
            updated=updated.replace("'xcrun_'+","'path_'+")
            updated=updated.replace(r"\n_fortran_mod\n",r" U fortran_mod\n")
            for old,new in [('libquadmath.0.dylib','libquadmath.so.0'),('libstdc++.6.dylib','libstdc++.so.6'),('libgcc_s.1.1.dylib','libgcc_s.so.1'),('libgfortran.5.dylib','libgfortran.so.5')]:
                updated=updated.replace(old,new)
        write_adapter(path,updated)

    if 'parameters' in args.stages:
        binary=copy_sources('binary')
        run.run([sys.executable,binary/'prepare_fixtures.py'],'binary-fixtures',cwd=root)
    typed=any(s in args.stages for s in ['pulsar_lifecycle','terrestrial_resources','observer_coordinates','planetary_ephemeris','propagation'])
    if typed:
        numeric=copy_sources('numeric_toolkit');adapt(numeric/'profile.py')
        run.run([sys.executable,numeric/'inventory.py'],'numeric-inventory',cwd=root)
        lifecycle=copy_sources('pulsar_lifecycle')
        run.run([sys.executable,'-c','from inventory import build,save,HERE; save(HERE/"inventory.json",build())'],'lifecycle-types',cwd=lifecycle)
    if 'jpl' in args.stages:
        coverage=copy_sources('coverage');adapt(coverage/'reproduce_native.py')
        run.run([sys.executable,coverage/'reproduce_native.py'],'jpl-prior-replay',cwd=root)
    elif 'planetary_ephemeris' in args.stages:copy_sources('jpl')
    for stage in args.stages:
        directory=copy_sources(stage);path=directory/('run.py' if stage=='jpl' else 'profile.py');adapt(path)
        if args.coverage and stage!='jpl':
            assert gcov,'Selected NATIVE_GCOV is unavailable'
            adapt(directory/'coverage.py')
        if stage=='clocks':adapt(directory/'run.py')
        if stage=='par_specialized':
            fixtures=directory/'fixtures.py'
            write_adapter(fixtures,bounded_telescope_inputs(fixtures.read_text()))
            if args.coverage:
                # Q's full scalar history exceeded the old 90-second process
                # allowance under gcov. This changes only the execution deadline.
                runner=directory/'run.py';text=runner.read_text()
                assert text.count('timeout=90')==1
                write_adapter(runner,text.replace('timeout=90','timeout=300'))
            # Historical projection/configuration still come from their pinned
            # commits. Preservation now names AL's actual isolated inputs.
            protected=directory/'protected.py'
            replace_function(protected,'snapshot',"def snapshot():\n    paths=json.loads((HERE/'al-inputs.json').read_text())\n    return {p:sha((ROOT/p).read_bytes()) for p in paths}\n")
            (directory/'al-inputs.json').write_text(json.dumps(list(sources))+'\n')
            run.run([sys.executable,protected,'--record'],stage+'-freeze',cwd=root)
        if stage in ['terrestrial_resources','observer_coordinates','planetary_ephemeris','propagation']:
            if stage=='propagation':
                supplemental=directory/'supplemental.py';text=supplemental.read_text()
                assert text.count("compile_probe('supplemental')")==1
                # TKlog truncates before path normalization. The supplemental
                # source directory must match ordinary/observer's eight bytes.
                write_adapter(supplemental,text.replace("compile_probe('supplemental')","compile_probe('supplem0')"))
            inventory=directory/'inventory.py';adapt(inventory)
            updated=inventory.read_text().replace("('tempo2','validation/reference')","('tempo2',)").replace("['tempo2','validation/reference']","['tempo2']")
            write_adapter(inventory,updated)
            if stage!='propagation':
                replace_function(inventory,'protected_paths',"def protected_paths():\n    # AL guards current staged authored inputs; A2 is receipt-bound by the parent.\n    return [ROOT/p for p in json.loads((HERE/'al-inputs.json').read_text())]\n")
            (directory/'al-inputs.json').write_text(json.dumps(list(sources))+'\n')
            run.run([sys.executable,directory/'prepare.py' if stage=='propagation' else inventory],stage+'-freeze',cwd=root)
        if stage=='pulsar_lifecycle':
            # The old preservation gate names a different independent Git repo.
            # AL instead verifies its A2 receipt above and guards every staged
            # authored input below. Native checkout cleanliness is still checked.
            runner=directory/'run.py';original=runner.read_text()
            updated=original.replace("['tempo2','validation/reference']","['tempo2']")
            assert updated!=original;write_adapter(runner,updated)
            run.run([sys.executable,'-c','from inventory import build,save,HERE; save(HERE/"inventory.json",build())'],stage+'-inventory',cwd=directory)
            for generator in ['fixtures.py','generate.py']:
                run.run([sys.executable,directory/generator],stage+'-'+generator,cwd=root)
            protected={str(p.relative_to(root)):sha(p) for p in root.joinpath('validation').rglob('*') if p.is_file() and str(p.relative_to(root)) in sources}
            (directory/'protected_hashes.json').write_text(json.dumps(protected,indent=2)+'\n')
        if stage=='preprocessing':
            header=run.run(['git','-C',native,'show',receipt['source_pin']+':tempo2.h'],'p-types-header').decode()
            header=re.sub(r'/\*.*?\*/|//[^\n]*','',header,flags=re.S)
            types={kind:[declaration.strip()+';' for declaration in header.split('typedef struct '+kind+' {',1)[1].split('} '+kind+';',1)[0].split(';') if declaration.strip()] for kind in ['observation','pulsar']}
            (directory/'TYPES.json').write_text(json.dumps(types,indent=2)+'\n')
        run.run([sys.executable,directory/'run.py','--record'],stage+'-record',cwd=root,expected=None)
        stages[stage]={'exit':run.commands[-1]['status'],'record_exit':run.commands[-1]['status'],'directory':str(directory),
                       'profile_adapter':sha(path),'runner':sha(directory/'run.py')}
        # The port checks require private/exit histories and AB's additional
        # branch cases. These observers separately prove their ordinary projection.
        supplements=([['supplemental.py','--record']] if stage=='propagation' else [])
        if stage in ['observer_coordinates','planetary_ephemeris','propagation']:
            supplements.append(['observer_audit.py'])
        if args.coverage and stage!='jpl':supplements.append(['coverage.py'])
        if args.coverage and stage=='preprocessing':supplements.append(['classify_coverage.py'])
        if stages[stage]['exit']==0:
            for script,*flags in supplements:
                run.run([sys.executable,directory/script,*flags],stage+'-'+script,cwd=root,expected=None)
                stages[stage][script]=run.commands[-1]['status']
                if run.commands[-1]['status']:stages[stage]['exit']=run.commands[-1]['status']
        run.save({'status':'running','stages':stages,'sources':sources})
        print(stage,stages[stage]['exit'],flush=True)
    assert sha(a2/'source/config.h')==receipt['source']['config.h']
    assert sha(a2/'libtempo2-reference.a')==receipt['library']
    (run.out/'platform-adaptations.patch').write_text(''.join(changes))
    failed=[name for name,row in stages.items() if row['exit']]
    run.save({'status':'failure' if failed else 'pass','stages':stages,'failed':failed,
              'sources':sources,'a2_receipt':sha(a2/'receipt.json'),'compiler':compiler,
              'adaptations':sha(run.out/'platform-adaptations.patch'),'producer':sha(run.out/'producer.py'),
              'scope':'Fresh complete authored native corpora; Rust comparison is a separate gate'})
    return bool(failed)


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','a2','native-repository']:p.add_argument('--'+name,type=Path,required=True)
    p.add_argument('--stages',nargs='+',choices=STAGES,default=STAGES)
    p.add_argument('--coverage',action='store_true',help='Execute separate gcov profiles; never equate counters with numerical agreement')
    raise SystemExit(build(p.parse_args()))
