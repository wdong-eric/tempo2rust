#!/usr/bin/env python3
"""0111 grouped native observer/terrestrial contraction evidence; no Rust edits."""
import argparse
import difflib
import json
from pathlib import Path
import platform
import sys
from common import ROOT, Run, sha


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','observer','terrestrial']:p.add_argument('--'+name,type=Path,required=True)
    a=p.parse_args();run=Run(a.out);inputs={};groups={}
    for group,directory,names in [('observer',a.observer,['get_obsCoord','tai2ut1']),
            ('terrestrial',a.terrestrial,['observatory','eop','tropo'])]:
        directory=directory.resolve();profile_path=directory/'build/ordinary/profile.json'
        profile=json.loads(profile_path.read_text());inputs[str(profile_path)]=sha(profile_path)
        assert profile['mutation'] is None and not profile['coverage']
        compiler=Path(profile['commands'][0][0]);flags=profile['flags']
        products={}
        for name in names:
            source=directory/'build/ordinary/source'/(name+'.C')
            # U includes the primary sources in probe.cc; X compiles them apart.
            unit=name if group=='observer' else 'probe'
            obj=directory/'build/ordinary'/(unit+'.o')
            if group=='terrestrial':assert '#include "'+name+'.C"' in (directory/'probe.cc').read_text()
            inputs[str(source)]=sha(source);inputs[str(obj)]=sha(obj)
            dump=run.out/(group+'-'+unit+'.object.s')
            if not dump.exists():
                command=['otool','-tvV',obj] if platform.system()=='Darwin' else ['objdump','-drwC',obj]
                dump.write_bytes(run.run(command,group+'-'+unit+'-disassembly'))
            assemblies={}
            for mode,extra in [('source',[]),('separate',['-ffp-contract=off'])]:
                output=run.out/(name+'-'+mode+'.s')
                run.run([compiler,*flags,*extra,'-g','-fverbose-asm','-S',source,'-o',output],name+'-'+mode)
                assemblies[mode]={'path':str(output),'sha256':sha(output)}
            products[name]={'source':sha(source),'actual_object':sha(obj),'actual_translation_unit':unit,
                'object_disassembly':sha(dump),'supplementary_source_assembly':assemblies}
        groups[group]={'compiler':str(compiler),'compiler_sha256':sha(compiler),
            'compiler_version':profile['compiler_version'],'flags':flags,
            'native_executable':profile['executable_sha256'],'products':products}
    # Reuse W's source-extracted, bounded scalar experiment verbatim. Only paths,
    # compiler selection and the old macOS-only expectation of differences vary.
    recipe=ROOT/'validation/terrestrial_resources_port/arithmetic.py';source=recipe.read_text()
    replacements={
        'from freeze import ROOT,HERE':'ROOT=Path('+repr(str(a.terrestrial.resolve().parents[1]))+');HERE=Path('+repr(str(run.out))+')',
        "CC='/opt/homebrew/bin/g++-16'":'CC='+repr(groups['terrestrial']['compiler']),
        ';assert differences':'; # Zero differences is an observed result on an uncontracted compiler profile.',
    }
    adapted=source
    for old,new in replacements.items():
        assert adapted.count(old)==1,old
        adapted=adapted.replace(old,new)
    path=run.out/'bounded.py';path.write_text(adapted)
    (run.out/'bounded.patch').write_text(''.join(difflib.unified_diff(source.splitlines(True),adapted.splitlines(True),fromfile='historical/arithmetic.py',tofile='AL/bounded.py')))
    run.run([sys.executable,path],'bounded-terrestrial')
    bounded=run.out/'build/arithmetic/receipt.json'
    run.save({'status':'pass','groups':groups,'inputs':inputs,
        'bounded_recipe':sha(recipe),'adapted_recipe':sha(path),'bounded_receipt':sha(bounded),
        'bounded_differences':json.loads(bounded.read_text())['differences'],
        'scope':'Original native compiler profile versus contraction-off diagnostic, source assembly and bounded scalar witnesses. Not a global fusion policy or permission to edit unverified Rust sites.'})
