#!/usr/bin/env python3
"""Actual native objects and bounded contraction witnesses; no Rust arithmetic edits."""
import argparse
import difflib
import json
import os
from pathlib import Path
import platform
import sys
from common import ROOT, Run, sha


def capture_group(run, group, directory, names, supplementary=False):
    directory=directory.resolve();profile_path=directory/'build/ordinary/profile.json'
    profile=json.loads(profile_path.read_text());inputs={str(profile_path):sha(profile_path)}
    assert profile['mutation'] is None and not profile['coverage'] and not profile.get('observer',False)
    compiler=Path(profile['commands'][0][0]);flags=profile['flags'];products={}
    assert sha(compiler)==profile['dependencies'][str(compiler.resolve())]
    exe=directory/'build/ordinary/probe';assert sha(exe)==profile['executable_sha256']
    inputs[str(compiler)]=sha(compiler);inputs[str(exe)]=sha(exe)
    for name in names:
        source=directory/'build/ordinary/source'/(name+'.C')
        assert sha(source)==profile['dependencies'][str(source)]
        # U includes primary sources in probe.cc; the other groups compile apart.
        unit='probe' if group=='terrestrial' else name
        obj=directory/'build/ordinary'/(unit+'.o')
        if group=='terrestrial':assert '#include "'+name+'.C"' in (directory/'probe.cc').read_text()
        command=next(c for c in profile['commands'] if '-c' in c and c[-1]==str(obj))
        translation_unit=Path(command[command.index('-c')+1])
        assert sha(translation_unit)==profile['dependencies'][str(translation_unit)]
        if 'objects' in profile:assert sha(obj)==profile['objects'][obj.name]
        for path in [source,obj,translation_unit]:inputs[str(path)]=sha(path)
        dump=run.out/(group+'-'+unit+'.object.s')
        if not dump.exists():
            disassemble=['otool','-tvV',obj] if platform.system()=='Darwin' else ['objdump','-drwC',obj]
            dump.write_bytes(run.run(disassemble,group+'-'+unit+'-disassembly'))
        assemblies={}
        if supplementary:
            for mode,extra in [('source',[]),('separate',['-ffp-contract=off'])]:
                output=run.out/(name+'-'+mode+'.s')
                run.run([compiler,*flags,*extra,'-g','-fverbose-asm','-S',source,'-o',output],name+'-'+mode)
                assemblies[mode]={'path':str(output),'sha256':sha(output)}
        products[name]={'source':sha(source),'actual_object':sha(obj),'actual_translation_unit':unit,
            'translation_unit_sha256':sha(translation_unit),'compile_command':command,
            'object_hash_in_original_profile':profile.get('objects',{}).get(obj.name),
            'object_disassembly':sha(dump),'object_disassembly_path':str(dump),
            'supplementary_source_assembly':assemblies}
    return {'compiler':str(compiler),'compiler_sha256':sha(compiler),
        'compiler_version':profile.get('compiler_version',profile.get('compiler')),'flags':flags,
        'native_profile':str(profile_path),'native_profile_sha256':sha(profile_path),
        'native_executable':profile['executable_sha256'],'link_command':profile['commands'][-1],
        'products':products},inputs


def observed_ephemeris(text):
    declaration='void al_ephemeris_offset(int,int,int,int,double,double,double,double,double,double,double,double);\n'
    observed=text;expressions=[]
    for axis,letter in enumerate('XYZ'):
        start=text.index('m = (psr[p].telD'+letter+'_v[k]')
        end=text.index('break;',start)
        body=text[start:end];assert body.count('ival = m*(double)psr[p].obsn[i].sat+c;')==1
        expression=body
        for array in ['t','v']:
            for suffix,index in [('[k+1]',1),('[k]',0)]:
                expression=expression.replace('psr[p].telD'+letter+'_'+array+suffix,array+str(index))
        expression=expression.replace('psr[p].obsn[i].sat','sat')
        expressions.append(expression)
        hook=('al_ephemeris_offset('+str(axis)+',p,i,k,psr[p].telD'+letter+'_t[k],'
            'psr[p].telD'+letter+'_t[k+1],psr[p].telD'+letter+'_v[k],psr[p].telD'+letter+'_v[k+1],'
            '(double)psr[p].obsn[i].sat,m,c,ival);\n                        ')
        assert observed.count(body)==1
        observed=observed.replace(body,body+hook)
    assert len({''.join(e.split()) for e in expressions})==1,'axis expressions differ'
    return declaration+observed,expressions[0]


def check_offsets(path):
    rows=[]
    for line in path.read_text().splitlines():
        fields=line.split('\t');assert len(fields)==15
        assert fields[9:12]==fields[12:15],('source/replay interpolation differs',fields)
        rows.append(dict(zip(['axis','pulsar','observation','interval','t0','t1','v0','v1','sat',
            'source_m','source_c','source_offset','replay_m','replay_c','replay_offset'],fields)))
    assert rows,'offset observer did not execute'
    return rows


def ephemeris_witness(run, directory):
    directory=directory.resolve();ordinary=directory/'build/ordinary'
    profile_path=ordinary/'profile.json';profile=json.loads(profile_path.read_text())
    source=ordinary/'source/readEphemeris.C';probe=ROOT/'validation/release/ephemeris_offsets.cc'
    inputs={str(p):sha(p) for p in [profile_path,source,probe]}
    assert inputs[str(source)]==profile['dependencies'][str(source)]
    work=run.out/'ephemeris-offsets';work.mkdir()
    text=source.read_text();observed,expression=observed_ephemeris(text)
    observed_path=work/'readEphemeris.C';observed_path.write_text(observed)
    (work/'source.patch').write_text(''.join(difflib.unified_diff(text.splitlines(True),observed.splitlines(True),fromfile=str(source),tofile=str(observed_path))))
    include=work/'ephemeris_interpolation.inc';include.write_text(expression)
    original_obj=ordinary/'readEphemeris.o';obj=work/'readEphemeris.o'
    command=next(c for c in profile['commands'] if '-c' in c and c[-1]==str(original_obj))
    command=list(command);command[command.index('-c')+1]=str(observed_path);command[-1]=str(obj)
    command[command.index('-MF')+1]=str(work/'readEphemeris.d')
    run.run(command,'ephemeris-observed-build')
    dependencies=(work/'readEphemeris.d').read_text().replace('\\\n','').split(':',1)[1].split()
    for name in dependencies:
        path=Path(name)
        if path!=observed_path:
            assert sha(path)==profile['dependencies'][name]
            inputs[name]=sha(path)
    callback=work/'offsets.o';compiler=profile['commands'][0][0]
    run.run([compiler,*profile['flags'],'-I'+str(work),'-c',probe,'-o',callback],'ephemeris-offset-callback-build')
    exe=work/'probe';link=list(profile['commands'][-1]);link.insert(1,str(callback))
    link=[str(obj) if x==str(original_obj) else x for x in link];link[-1]=str(exe)
    link=[('-Wl,-map,' if platform.system()=='Darwin' else '-Wl,-Map,')+str(work/'probe.map')
          if x.startswith(('-Wl,-map,','-Wl,-Map,')) else x for x in link]
    for x in profile['commands'][-1]:
        if x.endswith('.o'):
            path=Path(x);assert sha(path)==profile['objects'][path.name];inputs[x]=sha(path)
    run.run(link,'ephemeris-observed-link')
    native=ordinary/'probe';assert sha(native)==profile['executable_sha256'];inputs[str(native)]=sha(native)
    resources=directory/'build/resources';manifest=directory/'build/resources.json'
    resource_hashes=json.loads(manifest.read_text());inputs[str(manifest)]=sha(manifest)
    records=[]
    # Existing Y cancellation lead, each axis's repeated/boundary history, and
    # first-matching-interval overlap. No new corpus or resource generation.
    for case,expected_rows in [('c076_absent_nutation',2),('c054_offset_0_4',8),
            ('c060_offset_1_4',8),('c066_offset_2_4',8),('c055_offset_overlap_0',2)]:
        actions=directory/'build/cases'/case/'actions.tsv';inputs[str(actions)]=sha(actions)
        data=actions.read_bytes();(work/(case+'.actions.tsv')).write_bytes(data)
        for line in data.decode().splitlines():
            fields=[bytes.fromhex(x[1:]).decode() for x in line.split('\t')]
            if fields[0]=='pulsar':
                path=resources/(fields[5]+'.bin')
                assert sha(path)==resource_hashes[fields[5]]['sha256'];inputs[str(path)]=sha(path)
        products={};trace=work/(case+'.trace.tsv')
        for label,binary in [('original',native),('observed',exe)]:
            events=work/(case+'.'+label+'.jsonl')
            command=[binary,actions,events,resources]
            name='offset-'+case+'-'+label
            run.run(command,name,env=dict(os.environ,LC_ALL='C',LANG='C',AL_EPHEMERIS_OFFSETS=str(trace)))
            products[label]={'events':sha(events),'stdout':sha(run.out/(name+'.stdout')),'stderr':sha(run.out/(name+'.stderr'))}
        assert products['original']==products['observed'],('ephemeris observer is not neutral',case)
        rows=check_offsets(trace);assert len(rows)==expected_rows,(case,len(rows))
        records.append({'case':case,'input_sha256':sha(actions),'observer_neutral':True,
            'products':products,'trace_sha256':sha(trace),'rows':rows})
    for path,digest in inputs.items():assert sha(path)==digest,('witness input changed',path)
    return {'cases':records,'inputs':inputs,'observed_source':sha(observed_path),
        'observed_object':sha(obj),'callback_object':sha(callback),'observed_executable':sha(exe),
        'source_expression':expression,'source_expression_sha256':sha(include),
        'scope':'Neutrality and exact same-profile expression replay; original object disassembly is separate. No production correction inferred.'}


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['out','observer','terrestrial']:p.add_argument('--'+name,type=Path,required=True)
    # The historical focused 0117 recipe only stages observer/terrestrial inputs.
    for name in ['planetary','propagation']:p.add_argument('--'+name,type=Path)
    a=p.parse_args();run=Run(a.out);inputs={};groups={}
    selected=[('observer',a.observer,['get_obsCoord','tai2ut1'],True),
              ('terrestrial',a.terrestrial,['observatory','eop','tropo'],True)]
    if a.planetary:selected.append(('planetary',a.planetary,['readEphemeris'],False))
    if a.propagation:selected.append(('propagation',a.propagation,
        ['shapelet','sw_delay','calculate_bclt','shapiro_delay','dm_delays','secularMotion'],False))
    for group,directory,names,supplementary in selected:
        groups[group],bound=capture_group(run,group,directory,names,supplementary)
        inputs.update(bound)
    offsets=ephemeris_witness(run,a.planetary) if a.planetary else None
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
    for input_path,digest in inputs.items():assert sha(input_path)==digest,('native input changed',input_path)
    run.save({'status':'pass','groups':groups,'inputs':inputs,
        'bounded_recipe':sha(recipe),'adapted_recipe':sha(path),'bounded_receipt':sha(bounded),
        'bounded_differences':json.loads(bounded.read_text())['differences'],'ephemeris_offsets':offsets,
        'scope':'Original native compiler profile versus contraction-off diagnostic, source assembly and bounded scalar witnesses. Not a global fusion policy or permission to edit unverified Rust sites.'})
