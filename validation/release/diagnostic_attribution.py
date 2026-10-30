"""0107 metadata projection: validate real Rust emit sites, retain every body byte.

Only diagnostic streams are eligible. This is not a generic text scrubber;
callers preserve raw products and run their existing numerical/order comparisons.
"""
import hashlib
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
