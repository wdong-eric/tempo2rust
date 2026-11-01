"""0107 metadata projection: validate real Rust emit sites, retain every body byte.

Only diagnostic streams are eligible. This is not a generic text scrubber;
callers preserve raw products and run their existing numerical/order comparisons.
"""
import hashlib
import argparse
import json
import re
from pathlib import Path

PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
SITE=re.compile(r'\b(?:rust_log|rust_timing|main_log|source_log|log_loaded)\s*\(\s*/\* TEMPO2 ([\w.\-]+):([\d,]+) \*/')
VERSION=re.compile(r'\.version\(\s*"([^"\n]+)",\s*"([^"\n]+)"\s*\)')
PREFIX=re.compile(rb'(?m)^((?:\x1b\[[0-9;]*m)*)\[([^\]\n]+\.(?:rs|C|c|h)):(\d+)\]')
VERSION_ROW=re.compile(rb'(?m)^\[TEMPO2-RUST VERSION:\] \[([^\]\n]+\.rs):(\d+)\] \[TEMPO2 counterpart: ([^\]\n]+)::([^\]\n]+)\]\n')
NATIVE_VERSION=re.compile(rb'(?m)^\[TEMPO2 VERSION:\] \[([^\]\n]+)\] \[([^\]\n]*)\] \[([^\]\n]+)\]\n')
INTERNAL_PREFIX=re.compile(rb'(?m)^(?:\x1b\[[0-9;]*m)*\[(?:TEMPO2:|TEMPO2 SOURCE VERSION:\])')


class Attribution:
    def __init__(self, candidate):
        self.candidate=Path(candidate).resolve();self.sites={};self.versions={};self.sources={}
        for path in sorted((self.candidate/'rust/src').rglob('*.rs')):
            if path.name.endswith('_tests.rs'):continue
            source=path.read_text().split("#[cfg(test)]\nmod tests {",1)[0];self.sources[str(path.relative_to(self.candidate))]=hashlib.sha256(path.read_bytes()).hexdigest()
            for match in SITE.finditer(source):
                key=(path.name,source.count('\n',0,match.start())+1)
                assert key not in self.sites,key
                self.sites[key]=(match[1],tuple(sorted(map(int,match[2].split(',')))))
            for match in VERSION.finditer(source):
                key=(path.name,source.count('\n',0,match.start())+1)
                assert key not in self.versions,key
                self.versions[key]=(match[1],match[2])
            # Historical generators may still be used only as historical tools.
            # Reintroducing their native-labelled Rust calls is a failing audit.
            assert not re.search(r'\.log\(\s*[^,]+,\s*"[^"\n]+\.(?:C|c|h)"',source),path
            assert not re.search(r'\.timing\(\s*"[^"\n]+\.(?:C|c|h)"',source),path
        # Several native branches share one Rust emitter. Bind that named set;
        # do not infer a native line from message wording or numeric values.
        groups=[{(file,line) for line in lines} for file,lines in self.sites.values()]
        merged=[]
        for group in groups:
            overlaps=[old for old in merged if old & group]
            for old in overlaps:group|=old;merged.remove(old)
            merged.append(group)
        self.native={point:b'[TEMPO2:'+next(iter(group))[0].encode()+b':'+
                     ','.join(str(n) for _,n in sorted(group)).encode()+b']'
                     for group in merged for point in group}
        cargo=(self.candidate/'rust/Cargo.toml').read_text()
        version=re.search(r'(?m)^version\s*=\s*"([^"]+)"',cargo)[1]
        self.identity=f'[TEMPO2-RUST VERSION:] tempo2-rust {version} (TEMPO2 source {PIN}; Rug128)\n'.encode()
        self.header=b'[TEMPO2 SOURCE VERSION:] '+PIN.encode()+b'\n'

    def stream(self, data, kind):
        """Validate raw diagnostic bytes, then project exactly once for comparison."""
        assert kind in ['native','rust']
        assert not INTERNAL_PREFIX.search(data),'internal comparison label in raw diagnostic input'
        def location(match):
            label=match[2].decode();file=Path(label).name;line=int(match[3])
            if kind=='rust':
                if file.endswith('.rs'):
                    assert label==file,'Rust emit label must be a basename: '+label
                    assert (file,line) in self.sites,('unknown Rust emit site',file,line)
                    native_file,lines=self.sites[file,line]
                    return match[1]+self.native[native_file,lines[0]]
                assert (file,line) not in self.native,('native-labelled Rust diagnostic',file,line)
                return match[0] # explicit provider/caller metadata outside Rust-owned emitters
            return match[1]+self.native.get((file,line),match[0][len(match[1]):])
        data=PREFIX.sub(location,data)
        if kind=='rust':
            def version(match):
                file,line=match[1].decode(),int(match[2]);counterpart=(match[3].decode(),match[4].decode())
                assert (file,line) in self.versions,('unknown Rust version site',file,line)
                assert self.versions[file,line]==counterpart,('wrong version counterpart',file,line,counterpart)
                return f'[TEMPO2 SOURCE VERSION:] {counterpart[0]}::{counterpart[1]}\n'.encode()
            data=VERSION_ROW.sub(version,data)
            assert data.count(self.identity)<=1,'duplicate Rust build identity'
            data=data.replace(self.identity,self.header)
            assert b'[TEMPO2-RUST VERSION:]' not in data,'invalid Rust version identity'
            assert b'[TEMPO2 VERSION:]' not in data,'native build presented as Rust version'
        else:
            def version(match):
                file,function,value=(part.decode().strip() for part in match.groups())
                if file=='tempo2.h':
                    assert value=='2026.09.1',('unexpected pinned native version',value)
                    return self.header
                candidates={pair for pair in self.versions.values() if pair[0][:20]==file and pair[1][:20]==function}
                assert len(candidates)==1,('unknown or ambiguous native version counterpart',file,function)
                file,function=candidates.pop()
                return f'[TEMPO2 SOURCE VERSION:] {file}::{function}\n'.encode()
            data=NATIVE_VERSION.sub(version,data)
        return data

    def structured(self, value, kind, key=''):
        """Project only designated diagnostic fields, preserving schemas and data."""
        if isinstance(value,dict):
            if ('source_channel' in value or 'source_stderr' in value) and isinstance(value.get('bytes'),list):
                return {**value,'bytes':list(self.stream(bytes(value['bytes']),kind))}
            return {name:self.structured(item,kind,name) for name,item in value.items()}
        if isinstance(value,list):return [self.structured(item,kind,key) for item in value]
        if isinstance(value,str) and key in ['stdout','stderr']:
            return self.stream(value.encode('latin1'),kind).decode('latin1')
        return value

    def manifest(self):
        return {'policy':'0107; validated named attribution only; bodies/order/numerics retained',
                'sources':self.sources,'sites':[{'rust_file':file,'rust_line':line,'native_file':native_file,'native_lines':lines}
                    for (file,line),(native_file,lines) in sorted(self.sites.items())],
                'versions':[{'rust_file':file,'rust_line':line,'native_counterpart':counterpart}
                    for (file,line),counterpart in sorted(self.versions.items())]}


def compare_pairs(pairs, mapping):
    """Use one validated adapter for the ignored Rust corpus harnesses too."""
    differences=[]
    for pair in pairs:
        try:
            native=mapping.stream(pair['native'].encode('latin1'),'native')
            rust=mapping.stream(pair['rust'].encode('latin1'),'rust')
        except AssertionError as error:
            differences.append({**pair,'attribution_error':str(error)})
        else:
            if native!=rust:differences.append(pair)
    return differences


# 0113: TKlog.C::_TKchklog caps the formatted C string before removing bytes
# from ESC through the next 'm'. This deliberately is not an ANSI grammar.
def redirected_log(formatted):
    capped=formatted.split(b'\0',1)[0][:359]
    output=bytearray();incode=False
    for byte in capped:
        if byte==27:incode=True
        if not incode:output.append(byte)
        if byte==ord('m'):incode=False
    return bytes(output)


def diagnostic_rendering(prefix, message, severity):
    """TKlog.h WHEREWARN/WHEREERR; message is complete C backing storage."""
    assert b'\0' not in prefix and b'\0' not in message
    assert len(message)<359,'backing message may already be truncated'
    if severity=='warning':
        formatted=b'\x1b[0m\x1b[1m'+prefix+b' \x1b[0m\x1b[0;35mWarning: \x1b[0m'+message+b'\n'
    else:
        assert severity=='error'
        formatted=b'\x1b[0m\x1b[1;31m'+prefix+b' ERROR:\x1b[0m'+message+b'\n'
    return formatted,redirected_log(formatted)


# Exact pinned source definitions qualified by the independent 0112/0113 proof.
# A different formatter or emitter needs its own source review, not a fallback.
RENDERING_SOURCES={
    'TKlog.C':'696904f67674dcfa51a485b0943d10bd39358daeb14649fe4c5d728d667f1d90',
    'TKlog.h':'977dc204af66a0e61fe3b6288c630e3a3343ebc9dc08d5e1f2e535b5ea518e3c',
    'observatory.C':'fabfaabc78b75652590c5a81c1c2bb05f806550f99dbd7dd3f1ab43d056bf2c3',
    'probe.cc':'96273636438eacdeb2fc8b5311ebf9c9102930720e039e80766b89ac42fa26b6',
}


def compare_terrestrial(mapping, case, expected, actual, base, destination, compare, project):
    """Strict W comparison, then the bounded 0113 proof from independent records.

    The runner owns its existing freeze, complete catalogue and execution gates.
    This additional path binds the saved raw products to those compared records;
    failures are returned for the runner to save and raise without losing evidence.
    """
    import gzip
    base=Path(base);destination=Path(destination)
    report={'contract':'0113 redirected W warning/error rendering','status':'failure','stage':'attribution',
            'strict_difference':None,'verifier_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'inputs':{}}
    def read(path):
        path=Path(path);data=path.read_bytes()
        report['inputs'][str(path)]=hashlib.sha256(data).hexdigest()
        return data
    def digest(data):return hashlib.sha256(data).hexdigest()
    def packed(value):return json.dumps(value,separators=(',',':'),ensure_ascii=True,allow_nan=False).encode()
    try:
        left=mapping.structured(project(expected),'native');right=mapping.structured(project(actual),'rust')
        try:compare(left,right,case['id'])
        except AssertionError as error:report['strict_difference']=str(error)
        else:
            # Equal text alone could be forged by truncating Rust to the native
            # prefix. Check source consistency for this qualified stored history
            # too; only a real strict difference can gain the additional proof.
            transitions=[];previous=(0,0)
            for event in expected['events']:
                if event['phase']=='begin':continue
                counts=(event['state']['warnings'],event['state']['errors'])
                if counts!=previous:transitions.append((event['operation'],event['phase'],counts))
                previous=counts
            qualified=[('alias','end',(1,0)),('read_alias','exit',(0,1)),('init_obs','exit',(0,1))]
            if len(transitions)!=1 or transitions[0] not in qualified:
                report.update(status='pass',stage='complete',result='strict attribution comparison')
                return report
        # Preserve the original ordered schema, types and all non-stdout gates.
        report['stage']='non-rendering'
        compare(list(left),list(right),'case schema')
        for field in left:
            if field!='stdout':compare(left[field],right[field],case['id']+'.'+field)
        for value in [expected,actual]:
            compare([value['id'],value['classification'],value['actions'],value['status']],
                    [case['id'],case['classification'],case['actions'],case['status']],'case identity')
        report['stage']='evidence-integrity'
        contract=json.loads(read(base/'corrected_native/contract.json'));profile=contract['profile']
        assert profile['pin']==PIN and profile['coverage'] is False
        oracle_bytes=read(base/'corrected_native/reference.json.gz')
        assert digest(oracle_bytes)==contract['corrected_sha256']
        oracle=json.loads(gzip.decompress(oracle_bytes))
        originals=[row for row in oracle if row['id']==case['id']];assert len(originals)==1
        compare(originals[0],expected,'independent native record')
        receipts=json.loads(gzip.decompress(read(base/'corrected_native/receipts.json.gz')))
        native_receipts=[row for row in receipts if row['id']==case['id']];assert len(native_receipts)==1
        nr=native_receipts[0];rr=json.loads(read(destination/'receipt.json'))
        assert nr['case_sha256']==digest(packed(case)),'native case identity'
        compare(nr['launch_before'],nr['launch_after'],'native launch identity')
        assert nr['launch_before']['executable_sha256']==profile['executable_sha256'],'native executed image identity'
        assert nr['launch_before']['dependency_count']==len(profile['dependencies'])
        sources={}
        for name,bound in RENDERING_SOURCES.items():
            paths=[Path(path) for path in profile['dependencies'] if Path(path).name==name]
            assert len(paths)==1,('ambiguous native source',name)
            path=paths[0];assert profile['dependencies'][str(path)]==bound==digest(read(path)),name
            sources[name]=path
        source=sources['observatory.C'].parent;producer=source.parent.parent
        assert all(sources[name].parent==source for name in ['TKlog.C','TKlog.h'])
        assert profile['flags']==['-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(source)],'unqualified logger compilation'
        # The native source set and caller establish redirected stdout, event
        # markers and the two real warning/error sites. Rust identities bind the
        # independently recorded logger, emitter and caller to this candidate.
        for name in ['rust/src/clock_diagnostics.rs','rust/src/observatory.rs','rust/src/terrestrial_tests.rs']:
            assert digest(read(mapping.candidate/name))==rr['identity']['sources'][name],name
        launch=json.loads(read(destination/'launch.json'))
        compare(launch['identity'],rr['identity'],'Rust launch identity')
        assert launch['status']==rr['test_process_status']==0
        raw_input=read(destination/'input.json');spec=json.loads(raw_input)
        assert digest(raw_input)==rr['input_sha256']==launch['input_sha256'],'Rust input identity'
        workparent=producer/'xxxx';work=workparent/case['id']
        actions=[[v.replace('{case}',str(work)) for v in action] for action in case['actions']]
        compare(actions,spec['actions'],'concrete action identity')
        commands=('\n'.join('\t'.join('x'+v.encode().hex() for v in action) for action in actions)+'\n').encode()
        assert digest(commands)==nr['input_sha256'],'native concrete input identity'
        resources={name:digest(value.encode()) for name,value in case['files'].items()}
        # Resource maps bind names/bytes, not filesystem enumeration order.
        assert resources==rr['resources_before'],'Rust initial resources'
        assert {name:{'size':len(value.encode()),'sha256':resources[name]} for name,value in case['files'].items()}==nr['resources_before'],'native initial resources'
        compare(rr['resources_before'],launch['resources_before'],'Rust launch resources')
        assert {name:row['sha256'] for name,row in nr['resources_after'].items() if name not in ['actions.tsv','events.jsonl']}==rr['resources_after'],'resource effects'
        outcome=read(destination/'streams.outcome').decode()
        assert outcome==rr['library_outcome'],'Rust outcome identity'
        assert actual['status']=={'success':0,'source_exit':1}[outcome]
        nraw={'events':gzip.decompress(read(base/'corrected_native/raw'/(case['id']+'.jsonl.gz'))),
              **{channel:read(base/'corrected_native/raw'/(case['id']+'.'+channel)) for channel in ['stdout','stderr']}}
        rraw={'events':read(destination/'events.jsonl'),
              **{channel:read(destination/('streams.'+channel)) for channel in ['stdout','stderr']}}
        for key,data in nraw.items():assert digest(data)==nr['raw_sha256'][key],('native raw identity',key)
        for key,data in rraw.items():assert digest(data)==rr['raw']['events.jsonl' if key=='events' else 'streams.'+key],('Rust raw identity',key)
        raw_events={kind:[json.loads(line) for line in raw['events'].splitlines()] for kind,raw in [('native',nraw),('rust',rraw)]}
        compare(raw_events['rust'].pop(0),{'phase':'launch','nonce':spec['nonce'],'input_sha256':digest(raw_input)},'Rust challenge')
        native_u=producer.parents[2]/'validation/terrestrial_resources'
        def normalize(data,kind):
            source_root=str(source) if kind=='native' else spec['source_directory'].removesuffix('/')
            return data.replace(source_root.encode(),b'{source}').replace(str(workparent).encode(),b'{cases}').replace(str(native_u).encode(),b'{terrestrial}')
        for kind,raw,value in [('native',nraw,expected),('rust',rraw,actual)]:
            compare(json.loads(normalize(packed(raw_events[kind]),kind)),value['events'],kind+' raw events')
            for channel in ['stdout','stderr']:
                compare(normalize(raw[channel],kind).decode('latin1'),value[channel],kind+' raw '+channel)
        report['stage']='event-association'
        event_sequence=[]
        for i,action in enumerate(case['actions']):
            event_sequence+=[[i,'begin',action[0]],[i,'exit' if case['status'] and i==len(case['actions'])-1 else 'end',action[0]]]
        compare(event_sequence,[[e['event'],e['phase'],e['operation']] for e in expected['events']],'action/event association')
        # Identify one complete stored message by its counter/storage transition.
        # No case names or stdout delimiters select the emission.
        emissions=[];previous={'warnings':0,'errors':0,'warning_log':[],'error_log':[]}
        for index,event in enumerate(expected['events']):
            if event['phase']=='begin':continue
            state={key:event['state'][key] for key in previous}
            if state!=previous:
                assert previous=={'warnings':0,'errors':0,'warning_log':[],'error_log':[]},'ambiguous diagnostic history'
                if state['warnings']==1 and state['errors']==0 and state['error_log']==[]:
                    severity='warning';storage='warning_log';line=272
                    assert (event['operation'],event['phase'])==('alias','end')
                else:
                    assert state['errors']==1 and state['warnings']==0 and state['warning_log']==[],'unsupported warning/error counter transition'
                    severity='error';storage='error_log';line=163
                    assert event['operation'] in ['read_alias','init_obs'] and event['phase']=='exit'
                assert len(state[storage])==1,'ambiguous storage slot'
                emissions.append((index,severity,storage,line))
            previous=state
        assert len(emissions)==1,'requires one unambiguous diagnostic emission'
        index,severity,storage,line=emissions[0]
        report['stage']='backing-message'
        messages={kind:events[index]['state'][storage][0].encode('latin1') for kind,events in raw_events.items()}
        # Raw names in this qualified scope agree; do not infer lost native text
        # from Rust stdout or extend pathname projection to unknown cases.
        assert messages['native']==messages['rust'],'complete backing messages differ'
        event=raw_events['native'][index];action=actions[event['event']]
        if severity=='warning':
            resolved=bytes.fromhex(event['result']['output']['bytes'])
            assert b'\0' in resolved,'incomplete alias output'
            message=b"Assuming site '"+action[1].encode().split(b'\0',1)[0]+b"' means '"+resolved.split(b'\0',1)[0]+b"'"
        else:
            if event['operation']=='read_alias':filename=action[1]
            else:
                # The qualified init_obs path uses the default root and alias
                # filename; an override also emits an extra log and needs review.
                preceding=actions[:event['event']]
                assert all(a[0]!='root_key' and not (a[0] in ['env','unset'] and a[1]=='TEMPO2_ALIAS') for a in preceding)
                roots=[a[2] for a in preceding if a[:2]==['env','TEMPO2']]
                assert len(roots)==1
                filename=roots[0]+'/observatory/aliases'
            message=b"Cannot read '"+filename.encode().split(b'\0',1)[0]+b"'. Possibly $TEMPO2 is not set, or you have not updated the contents."
        assert messages['native']==message,'backing message does not match emitter arguments'
        report['stage']='emitter-association'
        sites=[key for key,value in mapping.sites.items() if value==('observatory.C',(line,))]
        assert len(sites)==1,'ambiguous Rust emitter'
        file,rust_line=sites[0]
        assert file=='observatory.rs'
        marker=f'/* TEMPO2 observatory.C:{line} */ b\'{severity[0]}\''
        assert marker in (mapping.candidate/'rust/src'/file).read_text()
        prefixes={'native':f'[{sources["observatory.C"]}:{line}]'.encode(),'rust':f'[{file}:{rust_line}]'.encode()}
        compare(mapping.stream(prefixes['native'],'native'),mapping.stream(prefixes['rust'],'rust'),'emitter correspondence')
        report['stage']='rendering'
        renders={};outside={}
        for kind,raw in [('native',nraw),('rust',rraw)]:
            formatted,emission=diagnostic_rendering(prefixes[kind],messages[kind],severity)
            before=bytearray();after=bytearray();target=before
            for i,event in enumerate(raw_events[kind]):
                if i==index:target=after
                if event['phase'] in ['begin','end']:
                    target.extend(f'@{event["phase"].upper()} {event["event"]} {event["operation"]}\n'.encode())
            assert raw['stdout']==before+emission+after,(kind,'unexpected rendered or surrounding byte')
            outside[kind]=(bytes(before),bytes(after))
            renders[kind]={'prefix':prefixes[kind].decode(),'backing_message_hex':messages[kind].hex(),
                'formatted_hex':formatted.hex(),'expected_hex':emission.hex(),'observed_hex':raw['stdout'][len(before):len(before)+len(emission)].hex(),
                'expected_sha256':digest(emission),'observed_sha256':digest(raw['stdout'][len(before):len(before)+len(emission)]),
                'emission_offset':len(before),'emission_bytes':len(emission),
                'formatted_tail_after_cap_hex':formatted[359:].hex(),'stdout_hex':raw['stdout'].hex(),'stderr_hex':raw['stderr'].hex()}
        assert outside['native']==outside['rust'],'bytes outside emission differ'
        report.update(status='pass',stage='complete',result='verified source-attribution rendering' if report['strict_difference'] else 'strict attribution and exact source rendering',
            event_index=index,event=expected['events'][index]['event'],operation=expected['events'][index]['operation'],
            severity=severity,channel='stdout',storage=storage,slot=0,renderings=renders,
            native_executable_sha256=profile['executable_sha256'],rust_executable_sha256=rr['identity']['executable_sha256'],
            all_non_rendering_fields_exact=True,all_outside_stdout_bytes_exact=True)
    except (AssertionError,KeyError,ValueError,OSError) as error:
        report['error']=str(error)
    return report


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['candidate','pairs','out']:p.add_argument('--'+name,type=Path,required=True)
    a=p.parse_args();mapping=Attribution(a.candidate)
    differences=compare_pairs(json.loads(a.pairs.read_text()),mapping)
    a.out.write_text(json.dumps(differences,indent=2)+'\n')
    a.out.with_suffix('.receipt.json').write_text(json.dumps({'status':'failure' if differences else 'pass',
        'pairs':hashlib.sha256(a.pairs.read_bytes()).hexdigest(),
        'adapter':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        'differences':len(differences),'attribution':mapping.manifest()},indent=2)+'\n')
    print(len(differences),'diagnostic differences after validated attribution')
    raise SystemExit(bool(differences))
