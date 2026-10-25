#!/usr/bin/env python3
"""Defined original/corrected neighbors for current-pulsar ownership fixes."""
import difflib
import json
import os
import subprocess
import native_timing as n

B = n.HERE/'build/cross-pulsar'
B.mkdir(parents=True, exist_ok=True)
identity = json.loads((n.BUILD/'identity.json').read_text())
source = n.ac.blob('formResiduals.C').decode()
# Isolate ownership: the unrelated unused newmean read has no producer and no use.
a = source.index('        double newmean = 0;')
b = source.index('        // deal with the REFPHS', a)
original = source[:a]+source[b:]
(B/'original.C').write_text(original)
(B/'original.patch').write_text(''.join(difflib.unified_diff(source.splitlines(True), original.splitlines(True))))
# Original p>0 averages are unwritten: never turn those bytes into an oracle.
harness = (n.HERE/'native_timing.cc').read_text().replace('if(i>=0 && active[p].Average', 'if(p==0 && i>=0 && active[p].Average')
(B/'probe.cc').write_text(harness)
commands = []
cmd = [n.ac.CC, '-O2', '-std=gnu++11', '-DHAVE_CONFIG_H', '-I'+str(n.ac.REF/'source'), '-c', str(B/'original.C'), '-o', str(B/'original.o')]
subprocess.run(cmd, check=True, capture_output=True)
commands.append(cmd)
for variant in ['original', 'corrected']:
    objects = [p for p in identity['objects'] if not p.endswith('formResiduals.C.o')]
    objects.insert(0, str(B/'original.o' if variant == 'original' else n.BUILD/'formResiduals.C.o'))
    cmd = [n.ac.CC, '-O2', '-std=gnu++11', '-DHAVE_CONFIG_H', '-I'+str(n.ac.REF/'source'), str(B/'probe.cc'), *objects, str(n.ac.REF/'libtempo2-reference.a'), '-lquadmath', '-lgfortran', '-o', str(B/variant)]
    subprocess.run(cmd, check=True, capture_output=True)
    commands.append(cmd)

f = n.HERE/'fixtures'
cases = {'reference': [('cut-sat.par','cut-sat.tim'),('cut-second.par','cut-bat.tim')],
         'shapelet': [('shapelet.par','shapelet.tim'),('shapelet-second.par','shapelet.tim')],
         'averaging': [('averaging.par','averaging.tim'),('averaging-second.par','averaging.tim')]}
results = {}
for name, pairs in cases.items():
    for selection in ['both', 'first', 'second']:
        chosen = pairs if selection == 'both' else [pairs[selection == 'second']]
        for variant in ['original', 'corrected']:
            work = B/f'{name}-{selection}-{variant}'
            work.mkdir(exist_ok=True)
            cmd = [str(B/variant),str(work/'native.tsv'),'0',str(len(chosen)),*[str(f/p) for pair in chosen for p in pair]]
            r = subprocess.run(cmd, cwd=work, env={**os.environ, 'TEMPO2':str(n.ac.REF/'source/T2runtime'),'LC_ALL':'C'}, capture_output=True, check=True)
            (work/'stdout').write_bytes(r.stdout)
            (work/'stderr').write_bytes(r.stderr)
            data = dict((tuple(line.split('\t',3)[:3]),line.split('\t',3)[3]) for line in (work/'native.tsv').read_text().splitlines())
            avg = (work/'avg.dat').read_bytes() if (work/'avg.dat').exists() else None
            results[name,selection,variant] = data,avg
            commands.append(cmd)
    for selection in ['first','second']:
        assert results[name,selection,'original'] == results[name,selection,'corrected'], (name, selection)
    original, oa = results[name,'both','original']
    corrected, ca = results[name,'both','corrected']
    assert {k:v for k,v in original.items() if k[0]=='0'} == {k:v for k,v in corrected.items() if k[0]=='0'}, name
    if name == 'averaging':
        assert oa != ca
        assert ca == results[name,'second','corrected'][1]
    else:
        assert any(original[k] != corrected[k] for k in original if k[0]=='1' and k[2]=='residual_s'), name
        single = results[name,'second','corrected'][0]
        assert {k[1:]:v for k,v in corrected.items() if k[0]=='1'} == {k[1:]:v for k,v in single.items() if k[2] not in ['avg_bat','avg_residual','avg_uncertainty']}, name
    print(name, 'distinct second pulsar corrected; first and both single-pulsar neighbors unchanged')
(B/'identity.json').write_text(json.dumps({'commands':commands,'native_identity_sha256':n.sha(n.BUILD/'identity.json'),'source_patch_sha256':n.sha(B/'original.patch'),'harness_sha256':n.sha(B/'probe.cc'),'executables':{v:n.sha(B/v) for v in ['original','corrected']}}, indent=2)+'\n')
