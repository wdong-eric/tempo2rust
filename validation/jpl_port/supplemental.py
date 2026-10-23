"""I-owned canonical probes beyond H: C-locale atoi whitespace and bounds."""
import json
import shutil
import subprocess
import run as port


def run(h,exe,profile):
    source=h.BUILD/'source';source.mkdir(exist_ok=True)
    manifest=port.load(port.RUST/'jpl_source_manifest.json')
    for name,digest in manifest['files'].items():
        data=subprocess.check_output(['git','-C',str(port.ROOT/'tempo2'),'show',f"{manifest['source_commit']}:{name}"])
        assert port.hashlib.sha256(data).hexdigest()==digest
        (source/name).write_bytes(data)
    cc=shutil.which('gcc-16');assert cc
    native=h.BUILD/'supplemental-canonical'
    command=[cc,'-O2','-std=c11','-I',str(source),str(port.H/'probe.c'),'-lm','-o',str(native)]
    compiled=subprocess.run(command,capture_output=True,text=True);assert compiled.returncode==0,compiled.stderr
    supplemental_inputs=h.BUILD/'supplemental_inputs';supplemental_inputs.mkdir(exist_ok=True)
    base,_=h.synthetic('<');cases={};inputs={};canonical={};rust={}
    prefixes=dict(ordinary=b'430/',vertical_tab=b'\x0b430/',other_c_whitespace=b' \t\n\r\x0c430/',all_c_whitespace=b' \t\n\r\x0b\x0c430/',negative_version=b'-430/',positive_version=b'+430/',terminated_no_digits=b'x',sign_then_whitespace=b'+ ')
    for name,prefix in prefixes.items():
        data=bytearray(base);data[26:84]=b' '*58;data[26:26+len(prefix)]=prefix
        path=supplemental_inputs/('supplemental_'+name+'.bin');path.write_bytes(data)
        inputs[name]=dict(sha256=port.sha(path),title_prefix_hex=prefix.hex())
        cases[name]=dict(commands=[f'open {path} 0','close'],setup=[])
        canonical[name]=h.execute(native,cases[name]);rust[name]=port.execute(exe,cases[name],h.INPUTS,name)
    abi=set(port.load(port.HERE/'projection.json')['scenarios']['zero_au'][0]['classifications'])
    rules=dict(scenarios={name:[dict(op=e['op'],reference_fields=sorted(e),exact_fields=sorted(set(e)-abi),classifications={k:'reference_C_ABI_only' for k in sorted(set(e)&abi)}) for e in events] for name,events in canonical.items()})
    counts=port.compare(rust,canonical,rules)
    assert canonical['vertical_tab'][1]['version']==430 and canonical['vertical_tab'][1]['recsize']==6600
    port.save(port.OUT/f'{profile}_supplemental_reference.json',canonical)
    port.save(port.OUT/f'{profile}_supplemental_raw.json',rust)
    port.save(port.OUT/f'{profile}_supplemental_comparison.json',dict(status='PASS',counts=counts,inputs=inputs,compiler=subprocess.check_output([cc,'--version'],text=True).splitlines()[0],command=command,canonical_probe_sha256=port.sha(native),source_hashes=manifest['files']))

    # Unlike numeric controls these cases have no source-defined atoi value.
    clang=shutil.which('clang');assert clang
    sanitized=h.BUILD/'title-sanitized'
    command=[clang,'-O1','-g','-std=c11','-fsanitize=address','-fno-sanitize-recover=all','-I',str(source),str(port.H/'probe.c'),'-lm','-o',str(sanitized)]
    p=subprocess.run(command,capture_output=True,text=True);assert p.returncode==0,p.stderr
    evidence={}
    for name,title in [('all_whitespace',b' '*58),('last_sign',b' '*57+b'+'),('last_minus',b' '*57+b'-')]:
        data=bytearray(base);data[26:84]=title
        path=supplemental_inputs/('supplemental_'+name+'.bin');path.write_bytes(data)
        lines=[f'open {path} 0']
        p=subprocess.run([str(sanitized)],input='\n'.join(lines)+'\n',capture_output=True,text=True)
        summary=[line for line in p.stderr.splitlines() if 'SUMMARY:' in line or 'ERROR:' in line]
        evidence[name]=dict(sha256=port.sha(path),commands=lines,returncode=p.returncode,source_diagnostic=summary,source_stdout=p.stdout)
        assert p.returncode and 'stack-buffer-overflow' in p.stderr,(name,p.returncode,p.stderr)
        observed=port.execute(exe,dict(commands=lines),h.INPUTS,name)
        evidence[name]['rust']=observed
        # The scenario projection records these as operation-local boundaries.
        assert observed[-1].get('boundary')=='title_conversion',(name,observed)
    port.save(port.OUT/f'{profile}_title_boundaries.json',dict(command=command,compiler=subprocess.check_output([clang,'--version'],text=True).splitlines()[0],executable_sha256=port.sha(sanitized),source_hashes=manifest['files'],cases=evidence))

    # Extend only H's observation harness: the included pinned JPL source remains
    # byte-identical. A 12-slot nut destination accommodates direct acceleration.
    harness=(port.H/'probe.c').read_text()
    replacements={
        'else if (!strcmp(op,"state")) {':'else if (!strcmp(op,"state_wide")) {',
        'double et, pv[16][6], nut[6]; int bary,list[14];':'double et, pv[16][6], nut[12]; int bary,list[14];',
        'fill(&pv[0][0],96); fill(nut,6);':'fill(&pv[0][0],96); fill(nut,12);',
        'numbers(nut,6); snapshot(e,controlled);':'numbers(nut,12); snapshot(e,controlled);',
    }
    for old,new in replacements.items():
        assert harness.count(old)==1,old
        harness=harness.replace(old,new)
    harness_path=h.BUILD/'demand_probe.c';harness_path.write_text(harness)
    native=h.BUILD/'demand-canonical'
    command=[cc,'-O2','-std=c11','-I',str(source),str(harness_path),'-lm','-o',str(native)]
    p=subprocess.run(command,capture_output=True,text=True);assert p.returncode==0,p.stderr
    cases={};canonical={};rust={}
    for index in [10,11,12,13]:
        for demand in [-2147483648,-1,0,1,2,3,4,2147483647]:
            name=f'list_{index}_demand_{demand}';entries=[0]*14;entries[index]=demand
            commands=[f'open {h.INPUTS/"extended_le.bin"} 0','control','state_wide 2450003.5 1 '+' '.join(map(str,entries)),'close']
            cases[name]=dict(commands=commands,setup=[])
            canonical[name]=h.execute(native,cases[name]);rust[name]=port.execute(exe,cases[name],h.INPUTS,name)
    rules=dict(scenarios={name:[dict(op=e['op'],reference_fields=sorted(e),exact_fields=sorted(set(e)-abi),classifications={k:'reference_C_ABI_only' for k in sorted(set(e)&abi)}) for e in events] for name,events in canonical.items()})
    counts=port.compare(rust,canonical,rules)
    for index in [10,11,12,13]:
        for demand in [4,2147483647]:assert rust[f'list_{index}_demand_{demand}'][3]['nut']==rust[f'list_{index}_demand_2'][3]['nut']
        for demand in [-1,-2147483648]:assert rust[f'list_{index}_demand_{demand}'][3]['nut']==rust[f'list_{index}_demand_1'][3]['nut']
    port.save(port.OUT/f'{profile}_demand_reference.json',canonical)
    port.save(port.OUT/f'{profile}_demand_raw.json',rust)
    port.save(port.OUT/f'{profile}_demand_comparison.json',dict(status='PASS',counts=counts,command=command,harness_sha256=port.sha(harness_path),harness_changes=replacements,source_hashes=manifest['files'],input_sha256=port.sha(h.INPUTS/'extended_le.bin'),scenarios={name:case['commands'] for name,case in cases.items()},scope='controlled same-host full-state observations, not portable C values'))
