#!/usr/bin/env python3
"""Read-only witness for the three retained W full-line-cap differences."""
import argparse
import ast
import gzip
import json
from pathlib import Path
import re
from common import Run, sha
from diagnostic_attribution import Attribution


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--workspace',type=Path,required=True);p.add_argument('--out',type=Path,required=True)
    a=p.parse_args();root=a.workspace.resolve();run=Run(a.out);mapping=Attribution(root)
    source=root/'validation/terrestrial_resources_port/run.py';text=source.read_text()
    # Reuse W's exact schema/ABI projection and comparison, without running its
    # CLI or changing its failed receipt. Compare every field independently.
    functions=[node for node in ast.parse(text).body if isinstance(node,ast.FunctionDef) and node.name in ['project','compare']]
    assert len(functions)==2
    scope={'json':json};exec(compile(ast.Module(body=functions,type_ignores=[]),str(source),'exec'),scope)
    base=root/'rust/verification/terrestrial_resources';oracle=base/'corrected_native/reference.json.gz'
    native=json.loads(gzip.decompress(oracle.read_bytes()));records=[];inputs={str(source):sha(source),str(oracle):sha(oracle)}
    for path in [base/'fixed00/source/TKlog.h',base/'fixed00/source/TKlog.C',root/'rust/src/clock_diagnostics.rs',
                 Path(__file__).with_name('diagnostic_attribution.py'),Path(__file__).with_name('common.py')]:
        inputs[str(path)]=sha(path)
    prefix=re.compile(rb'(?m)^\[[^\]\n]*observatory\.(?:C|rs):\d+\]')
    projected_prefix=re.compile(rb'(?m)^\[TEMPO2:observatory.C:\d+\]')
    def emission(data,pattern):
        matches=list(pattern.finditer(data));assert len(matches)==1
        match=matches[0];ends=[len(data)]
        for token in [b'@END ',b'\n']:
            pos=data.find(token,match.end())
            if pos>=0:ends.append(pos+int(token==b'\n'))
        end=min(ends)
        return data[match.start():match.end()],data[match.end():end],data[:match.start()]+data[end:]
    def formatted(prefix,message,warning):
        # Exact TKlog.h WHEREWARN/WHEREERR sequences, also used by the Rust
        # renderer. TKlog.C caps these bytes before removing complete SGR codes.
        if warning:
            return b'\x1b[0m\x1b[1m'+prefix+b' \x1b[0m\x1b[0;35mWarning: \x1b[0m'+message+b'\n'
        return b'\x1b[0m\x1b[1;31m'+prefix+b' ERROR:\x1b[0m'+message+b'\n'
    for case,event_index,storage in [('alias_bounded_tokens',9,'warning_log'),
            ('observatory_exit_read_alias',7,'error_log'),('observatory_missing_alias',9,'error_log')]:
        actual_path=base/'release'/case/'result.json';actual=json.loads(actual_path.read_text())
        expected=next(row for row in native if row['id']==case)
        left=mapping.structured(scope['project'](expected),'native');right=mapping.structured(scope['project'](actual),'rust')
        assert list(left)==list(right)
        comparisons={}
        for field in left:
            try:scope['compare'](left[field],right[field],case+'.'+field)
            except AssertionError as error:comparisons[field]={'equal':False,'difference':str(error)}
            else:comparisons[field]={'equal':True}
        assert [field for field,row in comparisons.items() if not row['equal']]==['stdout']
        native_raw=base/'corrected_native/raw'/(case+'.stdout');rust_raw=base/'release'/case/'streams.stdout'
        np,nb,nother=emission(native_raw.read_bytes(),prefix);rp,rb,rother=emission(rust_raw.read_bytes(),prefix)
        assert nother==rother,'non-diagnostic stdout changed'
        # Reconstruct each side from its independent stored message, never from
        # rendered stdout. Rust's raw event stream has one initial launch row.
        native_events=base/'corrected_native/raw'/(case+'.jsonl.gz')
        rust_events=base/'release'/case/'events.jsonl'
        nevents=[json.loads(line) for line in gzip.decompress(native_events.read_bytes()).splitlines()]
        revents=[json.loads(line) for line in rust_events.read_bytes().splitlines()]
        assert revents[0]['phase']=='launch'
        messages={'native':nevents[event_index]['state'][storage][0].encode('latin1'),
                  'rust':revents[event_index+1]['state'][storage][0].encode('latin1')}
        assert messages['native']==messages['rust']
        for kind,projected in [('native',left),('rust',right)]:
            # The existing W pathname projection, bound to this observed root.
            assert messages[kind].replace(str(base/'xxxx').encode(),b'{cases}')==projected['events'][event_index]['state'][storage][0].encode('latin1')
        # The two error messages contain the same observed filename on both
        # sides. Keep that identity explicit; this witness substitutes no data.
        paths=None
        if case!='alias_bounded_tokens':
            pattern=rb"Cannot read '([^']+)'"
            npath=re.search(pattern,nb)[1];rpath=re.search(pattern,rb)[1]
            assert npath==rpath
            paths={'native':npath.decode(),'rust':rpath.decode(),'exactly_equal':True}
        warning=case=='alias_bounded_tokens';rendered={}
        for kind,label,body in [('native',np,nb),('rust',rp,rb)]:
            wire=formatted(label,messages[kind],warning);capped=wire[:359]
            visible=re.sub(rb'\x1b\[[0-9;]*m',b'',capped)
            assert b'\x1b' not in visible and visible==label+body
            rendered[kind]={'full_bytes':len(wire),'capped_bytes':len(capped),
                'full_visible_bytes':len(re.sub(rb'\x1b\[[0-9;]*m',b'',wire)),
                'ansi_bytes_removed':len(capped)-len(visible),'full_hex':wire.hex(),
                'visible_emission_exactly_reproduced':True}
        assert rendered['native']['full_visible_bytes']>len(np+nb) and rb.startswith(nb)
        _,nbody,_=emission(left['stdout'].encode('latin1'),projected_prefix)
        _,rbody,_=emission(right['stdout'].encode('latin1'),projected_prefix)
        assert rbody.startswith(nbody)
        records.append({'case':case,'all_fields':comparisons,'native_prefix':np.decode(),'rust_prefix':rp.decode(),
            'native_prefix_bytes':len(np),'rust_prefix_bytes':len(rp),
            'native_emission_bytes':len(np+nb),'rust_emission_bytes':len(rp+rb),
            'native_full_visible_emission_witness_bytes':rendered['native']['full_visible_bytes'],
            'formatted_before_ansi_removal':rendered,
            'independent_backing_messages':{'field':f'events[{event_index}].state.{storage}[0]',
                'native_raw_event_index':event_index,'rust_raw_event_index':event_index+1,
                'raw_messages':{kind:value.decode('latin1') for kind,value in messages.items()},
                'exactly_equal':True,'established_path_projection_matches':True},
            'observed_filename_binding':paths,'visible_common_body':nbody.decode('latin1'),
            'remaining_rust_tail':rbody[len(nbody):].decode('latin1'),
            'raw_common_body':nb.decode('latin1'),'raw_remaining_rust_tail':rb[len(nb):].decode('latin1'),
            'all_other_stdout_bytes_exact':True})
        for path in [actual_path,native_raw,rust_raw,native_events,rust_events]:inputs[str(path)]=sha(path)
    (run.out/'producer.py').write_bytes(Path(__file__).read_bytes())
    run.save({'status':'pass','scope':'Presentation explanation only; all three full diagnostic comparison failures remain unchanged.',
        'records':records,'inputs':inputs,'attribution':mapping.manifest(),'recipe':sha(__file__)})
    print('Three native emissions exactly reproduced by the 359-byte formatted cap followed by ANSI removal; every other projected field and stdout byte agrees.')
