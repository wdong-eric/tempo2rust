#!/usr/bin/env python3
"""Run existing corpus gates against a copied candidate and fresh native evidence."""
import argparse
import ast
import difflib
import json
import os
from pathlib import Path
import platform
import shutil
import sys
import tempfile
from common import ROOT, Run, sha, identities

PORTS=['jpl_port','clocks_port','parameter_storage_port','observation_storage_port',
       'pulsar_lifecycle_port','time_ephemeris_port','terrestrial_resources_port',
       'observer_coordinates_port','input_to_bat']
STAGES=['jpl','clocks','parameters','parameter_storage','observation_storage','tim_io',
        'preprocessing','par_specialized','pulsar_lifecycle','time_ephemeris',
        'terrestrial_resources','observer_coordinates','planetary_ephemeris','propagation']
DEPENDENCIES={
    'parameter_storage':['parameters','numeric_toolkit'],
    'observation_storage':['parameters','numeric_toolkit'],
}


def execute(args):
    run=Run(args.out);reference=args.reference_root.resolve();candidate=args.candidate.resolve()
    shutil.copyfile(__file__,run.out/'producer.py')
    native_receipt=reference.parent/'receipt.json'
    native_stages=json.loads(native_receipt.read_text())['stages']
    blocked={}
    for stage in args.stages:
        for dependency in DEPENDENCIES.get(stage,[stage]):
            row=native_stages.get(dependency,{})
            if row.get('record_exit',row.get('exit'))!=0:
                blocked.setdefault(stage,[]).append(dependency+' ordinary native corpus')
        required=(['observer_audit.py'] if stage in ['observer_coordinates','planetary_ephemeris','propagation'] else [])
        if stage=='propagation':required+=['supplemental.py']
        for proof in required:
            if native_stages.get(stage,{}).get(proof)!=0:blocked.setdefault(stage,[]).append(proof)
    args.stages=[stage for stage in args.stages if stage not in blocked]
    if not args.stages:
        run.save({'status':'failure','blocked':blocked,'native_receipt':sha(native_receipt)})
        return 1
    assert (reference/'tempo2/.git').exists(),'Use an AL native corpus working snapshot'
    # TKlog truncates before path projection. Preserve the native root's byte
    # length while keeping every successor runner/output in this new directory.
    base=run.out/'w';padding=len(str(reference))-len(str(base))
    assert padding>=0,'Choose a shorter --out path to preserve native pathname lengths'
    root=Path(str(base)+'x'*padding)
    excluded=shutil.ignore_patterns('.git','__pycache__','rust','tempo2','decisions',
                                    *PORTS,'fitting','general_fitting','timing_models')
    def staged_inputs(directory,names):
        # Native launch scratch stays in its original, retained producer run.
        # Each successor recreates its own concrete caller files when needed.
        return excluded(directory,names)|({'cases'} if Path(directory).name=='build' else set())
    shutil.copytree(reference,root,symlinks=True,ignore=staged_inputs)
    (root/'tempo2').symlink_to(reference/'tempo2',target_is_directory=True)
    rust=root/'rust'
    shutil.copytree(candidate/'rust',rust)
    source_ids=identities(rust)
    # A short pathname is essential for N's native 100-byte deleteFileName.
    # The alias points into retained output; it does not copy or truncate inputs.
    alias=Path(tempfile.mkdtemp(prefix='al-corpus-',dir='/tmp'))/'root';alias.symlink_to(root,target_is_directory=True)
    changes=[];scripts={}
    cxx=shutil.which(os.environ.get('NATIVE_CXX','g++'));assert cxx
    cc=shutil.which(os.environ.get('NATIVE_CC','gcc'));assert cc
    def write(path,text):
        before=path.read_text();path.write_text(text)
        changes.extend(difflib.unified_diff(before.splitlines(True),text.splitlines(True),fromfile='historical/'+str(path.relative_to(root)),tofile='AL/'+str(path.relative_to(root))))
        (run.out/'adaptations.patch').write_text(''.join(changes))
    def function(path,name,body):
        text=path.read_text();node=next(n for n in ast.parse(text).body if isinstance(n,ast.FunctionDef) and n.name==name)
        lines=text.splitlines(True);write(path,''.join(lines[:node.lineno-1])+body+'\n'+''.join(lines[node.end_lineno:]))
    for name in [*PORTS,'fitting','general_fitting','timing_models']:
        dest=root/'validation'/name
        assert not dest.exists(),dest
        shutil.copytree(candidate/'validation'/name,dest)
        for path in dest.glob('*.py'):
            scripts[str(path.relative_to(root))]=sha(path)
            text=path.read_text()
            for old in ["str(RUST/'.cargo-home')","str(ROOT/'rust/.cargo-home')","str(ROOT / 'rust/.cargo-home')"]:
                text=text.replace(old,repr(os.environ.get('CARGO_HOME',str(Path.home()/'.cargo'))))
            text=text.replace("shutil.which('g++-16')",repr(cxx)).replace("'/opt/homebrew/bin/g++-16'",repr(cxx))
            text=text.replace("shutil.which('gcc-16')",repr(cc))
            text=text.replace("dir='/private/tmp'","dir='/tmp'")
            for variable in ['x', 'json.loads(line)']:
                text=text.replace(variable+"['target']['kind']==['lib']",variable+".get('profile',{}).get('test')")
            text=text.replace("x.get('target',{}).get('kind')==['lib']","x.get('profile',{}).get('test')")
            if platform.system()=='Linux':
                text=text.replace("'-Wl,-map,'","'-Wl,-Map,'")
                for a,b in [('libquadmath.0.dylib','libquadmath.so.0'),('libstdc++.6.dylib','libstdc++.so.6'),('libgcc_s.1.1.dylib','libgcc_s.so.1')]:text=text.replace(a,b)
            if name=='input_to_bat' and path.name in ['check_tim_corpus.py','corrected_jbo.py']:
                for expression in ['HERE.parent.parent','HERE.parents[1]','Path(__file__).resolve().parents[2]']:
                    text=text.replace(expression,repr(root).replace('PosixPath','Path'))
                text=text.replace('Path(__file__).resolve().parent',repr(alias/'validation/input_to_bat').replace('PosixPath','Path'))
            write(path,text)
    # K's logger truncates before normalization. Both engines must read the same
    # native case paths; substituting shorter AC paths changes the actual message.
    path=root/'validation/input_to_bat/check_parameters.py';text=path.read_text()
    begin=text.index("        for name,data in case['files'].items():")
    end=text.index('        selected.append(case)',begin)
    text=text[:begin]+text[end:]
    begin=text.index('    text=json.dumps(selected).replace(')
    end=text.index("    (BUILD/'cases.json')",begin)
    text=text[:begin]+'''    for case in selected:
        work=K/'build/cases'/(case['id']+'-'+hashlib.sha256(case['id'].encode()).hexdigest()[:10])
        case.update(json.loads(json.dumps(case).replace('{build}',str(BUILD)).replace('{case}',str(work))))
'''+text[end:]
    write(path,text)
    shutil.copytree(candidate/'decisions',root/'decisions')
    # Frozen legacy projections retain every field and disposition. Only their
    # oracle identity is rebound to the explicitly selected, freshly built corpus.
    for stage in ['jpl','clocks']:
        if stage not in args.stages:continue
        path=root/'validation'/f'{stage}_port/projection.json';projection=json.loads(path.read_text())
        projection['reference_sha256']=sha(root/'validation'/stage/'outputs/reference.json')
        write(path,json.dumps(projection,indent=2)+'\n')
    if 'par_specialized' in args.stages:
        path=root/'validation/input_to_bat/telescope_boundaries.py'
        write(path,path.read_text().replace("family+'1000'","family+'500'"))
    if 'propagation' in args.stages:
        path=root/'validation/input_to_bat/corrected_propagation.py';text=path.read_text()
        # The copied profile identifies actual retained objects at the native
        # producer path. Link replacements must address those exact objects.
        text=text.replace('def main():\n', 'def main():\n    global NATIVE\n',1)
        marker="    profile = json.loads((NATIVE / 'profile.json').read_text())"
        assert text.count(marker)==1
        text=text.replace(marker,marker+"\n    link = profile['commands'][-1]\n    NATIVE = Path(link[link.index('-o') + 1]).parent")
        old="json.loads((AB / 'outputs/record_identity.json').read_text())['actual_executable_sha256']"
        assert text.count(old)==1
        text=text.replace(old,"json.loads((AB / 'outputs/last_execution.json').read_text())['profile']['executable_sha256']")
        write(path,text)
        path=root/'validation/input_to_bat/check_propagation.py';text=path.read_text()
        old="        work = build / case['id']"
        assert text.count(old)==1
        write(path,text.replace(old,"        work = (build if args.corrected else AB / 'build/cases') / case['id']")
              .replace('work.mkdir(exist_ok=True)','work.mkdir(parents=True,exist_ok=True)'))
    if 'jpl' in args.stages:
        path=root/'validation/jpl_port/run.py'
        write(path,path.read_text().replace("OUT = RUST/'verification/jpl'","OUT = RUST/'verification/jpl_al'"))
        function(path,'manifests',"""def manifests():
    checks=load(H/'outputs/checks.json');pin=checks['source_commit']
    for name,digest in checks['source_sha256'].items():
        blob=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',f'{pin}:{name}'])
        assert hashlib.sha256(blob).hexdigest()==digest==sha(ROOT/'tempo2'/name)
    for name,digest in checks['harness_sha256'].items():assert sha(H/name)==digest,name
    paths=[p for d in [RUST/'src',RUST/'native',H,HERE] for p in d.rglob('*') if p.is_file() and not any(q in p.parts for q in ['build','__pycache__'])]
    return dict(sha256={str(p.relative_to(ROOT)):sha(p) for p in paths})
""")
    if 'parameter_storage' in args.stages:
        path=root/'validation/parameters/catalog.py';text=path.read_text()
        marker="    lines=['# Parameter registry'"
        assert text.count(marker)==1
        # Reuse K's complete 2,000-slot registry builder. The remaining legacy
        # catalog publishes old assignment reports and unrelated input hashes.
        write(path,text[:text.index(marker)]+"\nif __name__=='__main__':main()\n")
        # AE changed these two undefined-source reads to ownership copies.
        # Retain all six actual copy boundaries, and run AE's separate test of
        # initialized/unwritten copies below (GENERAL_FITTING_REPORT.md:205).
        path=root/'validation/parameter_storage_port/run.py';text=path.read_text()
        old=",'prefit':('prefit','unwritten_value'),'prefitErr':('prefitErr','unwritten_value')"
        assert text.count(old)==1
        write(path,text.replace(old,'').replace('boundaries=8,',"boundaries=len(raw['boundaries']),"))
        path=root/'validation/parameter_storage_port/controls.py';text=path.read_text()
        assert "p['boundaries'][6]" in text
        # The same prefix/caller/reentry mutations now target label_capacity,
        # which still stops after the complete first-cell copy.
        write(path,text.replace("p['boundaries'][6]","p['boundaries'][4]"))
    if 'observation_storage' in args.stages:
        # Retain the raw-header validation, 95-field map and four M projections.
        # AL replaces only the obsolete pre-implementation ownership snapshot.
        path=root/'validation/observation_storage_port/freeze.py';text=path.read_text()
        begin=text.index('protected=json.loads(');end=text.index('cs=json.loads(',begin)
        text=text[:begin]+text[end:]
        text=text.replace("len(protected),'protected files;'", "0,'legacy ownership files;'" )
        write(path,text)
        path=root/'validation/observation_storage_port/run.py'
        write(path,path.read_text().replace("['tempo2','validation/reference']","['tempo2']"))
        run.run([sys.executable,root/'validation/observation_storage_port/freeze.py'],'observation-fields',cwd=root)
    if 'terrestrial_resources' in args.stages:
        path=root/'validation/terrestrial_resources_port/run.py';text=path.read_text()
        text=text.replace(" assert sha(oracle_path)=='6948f57785698ec65704652443f79146a26a92da6c4869a14d3b1e3b6eb0386e'",
                          " assert sha(oracle_path)==json.loads((HERE/'build/al-corrected.json').read_text())['oracle']")
        text=text.replace(" assert sha(OUT/'corrected_native/contract.json')=='c4273b5fa79d3d2a748575a96d749a7ab90f60385995dc2339cb225906abf97a'",
                          " assert sha(OUT/'corrected_native/contract.json')==json.loads((HERE/'build/al-corrected.json').read_text())['contract']")
        write(path,text)
    if 'time_ephemeris' in args.stages:
        path=root/'validation/time_ephemeris_port/controls.py'
        write(path,path.read_text().replace('verification-mutations-final.json','verification.json'))
        if platform.system()=='Linux':
            path=root/'validation/time_ephemeris/verify.py'
            write(path,path.read_text().replace('libquadmath.0.dylib','libquadmath.so.0'))
    if 'pulsar_lifecycle' in args.stages:
        path=root/'validation/pulsar_lifecycle_port/run.py';text=path.read_text()
        text=text.replace("OUT=ROOT/'rust/verification/pulsar_lifecycle'", "OUT=Path(os.environ.get('AL_LIFECYCLE_OUTPUT',str(ROOT/'rust/verification/pulsar_lifecycle')))")
        text=text.replace("ROOT/'validation/pulsar_lifecycle/expected'", "Path(os.environ.get('AL_LIFECYCLE_EXPECTED',str(ROOT/'validation/pulsar_lifecycle/expected')))")
        # Both modes share the same immutable preservation and membership freeze.
        text=text.replace("OUT/'freeze/", "ROOT/'rust/verification/pulsar_lifecycle/freeze/")
        text=text.replace('ident=identity(exe);results=[]','ident=identity(exe);results=[];failures=[]')
        old="  results.append(execute(c,exe,ident,OUT/a.profile/c['id']));print(a.profile,c['id'],'PASS',flush=True)"
        assert text.count(old)==1
        text=text.replace(old,"""  try:results.append(execute(c,exe,ident,OUT/a.profile/c['id']))
  except AssertionError as error:
   failures.append(dict(id=c['id'],difference=str(error)));print(a.profile,c['id'],'FAIL',str(error),flush=True)
  else:print(a.profile,c['id'],'PASS',flush=True)""")
        text=text.replace("dict(status='PASS',identity=ident,cases=results", "dict(status='FAIL' if failures else 'PASS',failures=failures,identity=ident,cases=results")
        text=text.replace("if __name__=='__main__':main()", " if failures:raise SystemExit(1)\nif __name__=='__main__':main()")
        write(path,text)
    if 'observer_coordinates' in args.stages:
        path=root/'validation/observer_coordinates_port/run.py';text=path.read_text()
        text=text.replace("destination=OUT/a.profile", "destination=Path(os.environ.get('AL_OBSERVER_OUTPUT',str(OUT)))/a.profile")
        text=text.replace("X/'outputs/observer_snapshots.json.gz'", "Path(os.environ.get('AL_OBSERVER_ORACLE',str(X/'outputs')))/'observer_snapshots.json.gz'")
        text=text.replace("X/'outputs/reference.json.gz'", "Path(os.environ.get('AL_OBSERVER_ORACLE',str(X/'outputs')))/'reference.json.gz'")
        text=text.replace("sha(X/'outputs'/n)", "sha(Path(os.environ.get('AL_OBSERVER_ORACLE',str(X/'outputs')))/n)")
        old="if __name__=='__main__':main()"
        assert text.count(old)==1
        # AA records failed comparisons but historically returned zero. Preserve
        # its complete rows and make the enclosing qualification gate fail too.
        write(path,text.replace(old," if any(r['error'] for r in rows):raise SystemExit(1)\n"+old))
    # New preservation manifests bind this candidate and these staged recipes.
    # They do not pretend to reproduce historical ownership snapshots.
    protected={str(p.relative_to(root)):sha(p) for p in rust.rglob('*') if p.is_file()}
    protected.update({p:sha(root/p) for p in scripts})
    for name in ['parameter_storage_port','observation_storage_port']:
        (root/'validation'/name/'protected_hashes.json').write_text(json.dumps(protected)+'\n')
    if 'pulsar_lifecycle' in args.stages:
        freeze=rust/'verification/pulsar_lifecycle/freeze';freeze.mkdir(parents=True)
        (freeze/'protected.json').write_text(json.dumps(protected)+'\n')
        (freeze/'membership.json').write_text(json.dumps({'S':json.loads((root/'validation/pulsar_lifecycle/cases.json').read_text())})+'\n')
    for name in ['time_ephemeris_port','terrestrial_resources_port']:
        path=root/'validation'/name/'build/freeze.json';path.parent.mkdir(parents=True)
        freeze={'protected':protected,'repositories':{}}
        if name=='terrestrial_resources_port' and 'terrestrial_resources' in args.stages:
            freeze['cases']=json.loads((root/'validation/terrestrial_resources/outputs/frozen/cases.json').read_text())
        path.write_text(json.dumps(freeze)+'\n')
    statuses={};proofs={};env=dict(os.environ,CARGO_TARGET_DIR=str(args.target.resolve()),LC_ALL='C',LANG='C')
    def call(label,command,extra_env=None):
        run.run(command,label,cwd=root,env=env if extra_env is None else dict(env,**extra_env),expected=None)
        statuses[label]=run.commands[-1]['status'];print(label,statuses[label],flush=True)
        run.save({'status':'running','stages':statuses})
        return statuses[label]==0
    def script(port,name,*flags):return [sys.executable,root/'validation'/port/name,*flags]
    profile=args.profile;release=['--release'] if profile=='release' else []
    for stage in args.stages:
        if stage=='preprocessing':
            call(stage,[sys.executable,ROOT/'validation/release/check_preprocessing.py',*release,'--candidate',root,'--reference-root',root/'validation/preprocessing','--out',run.out/'preprocessing'])
        elif stage in ['parameters','tim_io','par_specialized','planetary_ephemeris','propagation']:
            filename={'parameters':'check_parameters.py','tim_io':'check_tim_corpus.py','par_specialized':'check_par.py','planetary_ephemeris':'check_planetary.py','propagation':'check_propagation.py'}[stage]
            if stage=='parameters' and not call('parameters-native-matched',script('parameters','run.py','--record')):continue
            if stage=='par_specialized' and not call('par-native-matched',script('par_specialized','run.py','--record')):continue
            call(stage,script('input_to_bat',filename,*release))
            if stage=='planetary_ephemeris':call('planetary-random',script('input_to_bat','check_random.py',*release))
            if stage=='propagation':
                call('propagation-helpers',script('input_to_bat','check_helpers.py'))
                if call('propagation-corrected-native',script('input_to_bat','corrected_propagation.py')):
                    call('propagation-corrected',script('input_to_bat',filename,*release,'--corrected'))
        else:
            port=stage+'_port'
            if stage=='parameter_storage':
                if not call('parameter-registry',script('parameters','catalog.py')):continue
            if stage=='time_ephemeris':
                oracle=root/'validation/time_ephemeris/outputs/reference-corrected'
                oracle.rename(oracle.with_name('reference-inherited'))
                if not call('time-native-matched',script('time_ephemeris','run.py','--record')):continue
                if not call('time-native-controls',script('time_ephemeris','verify.py','--mutations')):continue
                if not call('time-corrected-native',script(port,'native.py')):continue
            if stage=='terrestrial_resources':
                if not call('terrestrial-native-controls',script('terrestrial_resources','controls.py')):continue
                (rust/'verification/terrestrial_resources').mkdir(parents=True,exist_ok=True)
                if not call('terrestrial-corrected-native',script(port,'corrected_native.py')):continue
                corrected=rust/'verification/terrestrial_resources/corrected_native'
                proofs['terrestrial_corrected']={'oracle':sha(corrected/'reference.json.gz'),'contract':sha(corrected/'contract.json')}
                (root/'validation'/port/'build/al-corrected.json').write_text(json.dumps(proofs['terrestrial_corrected'])+'\n')
            flags=['--profile',profile]
            if stage in ['time_ephemeris','terrestrial_resources']:flags+=['--continue']
            call(stage,script(port,'run.py',*flags))
            if stage=='parameter_storage':
                passed=call('parameter-storage-unwritten',['cargo','test','--manifest-path',rust/'Cargo.toml',
                     '--locked','--offline',*release,'--lib',
                     'general_fitting_tests::copying_unused_prefit_cells_preserves_unwritten_state','--','--exact'])
                if passed:assert '1 passed; 0 failed' in (run.out/'parameter-storage-unwritten.stdout').read_text()
            if stage=='pulsar_lifecycle':
                corrected=run.out/'lifecycle-corrected-native'
                if call('lifecycle-corrected-native',[sys.executable,ROOT/'validation/release/lifecycle_correction.py',
                        '--native',root/'validation/pulsar_lifecycle','--out',corrected]):
                    call('lifecycle-corrected',script(port,'run.py',*flags),{
                        'AL_LIFECYCLE_EXPECTED':str(corrected/'expected'),
                        'AL_LIFECYCLE_OUTPUT':str(rust/'verification/pulsar_lifecycle_corrected')})
            if stage=='observer_coordinates':
                corrected=run.out/'observer-corrected-native'
                if call('observer-corrected-native',[sys.executable,ROOT/'validation/release/observer_correction.py',
                        '--native',root/'validation/observer_coordinates','--out',corrected]):
                    call('observer-corrected',script(port,'run.py',*flags),{
                        'AL_OBSERVER_ORACLE':str(corrected),
                        'AL_OBSERVER_OUTPUT':str(rust/'verification/observer_coordinates_corrected')})
    # Public source bytes are checked even when a comparison fails.
    source_mismatches=[name for name,digest in source_ids.items() if sha(rust/name)!=digest]
    failed=[name for name,code in statuses.items() if code]
    run.save({'status':'failure' if failed or blocked or source_mismatches else 'pass',
              'stages':statuses,'failed':failed,'blocked':blocked,'source_mismatches':source_mismatches,'profile':profile,
              'candidate_sources':source_ids,'authored_comparators':scripts,'short_path_alias':str(alias),
              'native_receipt':sha(reference.parent/'receipt.json'),'corrected_proofs':proofs,
              'native_root':str(reference),'workspace':str(root),'adaptations':sha(run.out/'adaptations.patch'),'producer':sha(run.out/'producer.py')})
    return bool(failed or blocked or source_mismatches)


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','candidate','reference-root','target']:p.add_argument('--'+name,type=Path,required=True)
    p.add_argument('--profile',choices=['debug','release'],default='release')
    p.add_argument('--stages',nargs='+',choices=STAGES,default=STAGES)
    raise SystemExit(execute(p.parse_args()))
