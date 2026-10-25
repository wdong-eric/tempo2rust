#!/usr/bin/env python3
"""0053 JBO aliasing correction: source-identical neighbors, separate evidence."""
import hashlib,json,os,re,subprocess,sys,uuid
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];N=ROOT/'validation/tim_io';BUILD=Path(__file__).resolve().parent/'build/jbo-corrected'
def sha(data):return hashlib.sha256(data).hexdigest()
def main():
    BUILD.mkdir(parents=True,exist_ok=True)
    sys.path.insert(0,str(N));import run as n
    profile=json.loads((N/'profile.json').read_text());identity=n.execution_identity(Path(profile['executable_path']),profile,True)
    source_dir=Path(next(x[2:] for x in profile['command'] if x.startswith('-I')))
    original=source_dir/'readJBO_bat.C';raw=original.read_text();fixed=raw
    changes=[('char raw_record[25*8];','double words[25];\n    char *raw_record = (char*)words;'),('doubles = (double*)raw_record;','doubles = words;'),('''    swap4(raw);
    swap4(raw+4);
    int* i = (int*)raw;
    int a = i[0];
    int b = i[1];
    i[1] = a;
    i[0] = b;''','''    // AC 0053: int* access to double storage violates strict aliasing.
    // Reverse its object representation through char accesses instead.
    for (int i=0; i<4; ++i) {
        char t=raw[i]; raw[i]=raw[7-i]; raw[7-i]=t;
    }''')]
    # Preserve original diagnostic source coordinates after the one extra line.
    for before,after in changes:
        assert fixed.count(before)==1,before
        fixed=fixed.replace(before,after+ ('\n#line '+str(raw[:raw.index(before)+len(before)].count('\n')+2)+' "'+str(original)+'"' if '\n' in after and before.startswith('char ') else ''))
    source=BUILD/'readJBO_bat.C';source.write_text('#line 1 "'+str(original)+'"\n'+fixed)
    exe=BUILD/'probe';cmd=[str(source) if x==str(original) else str(exe) if x==profile['executable_path'] else '-Wl,-map,'+str(BUILD/'probe.map') if x.startswith('-Wl,-map,') else x for x in profile['command']]
    proc=subprocess.run(cmd,capture_output=True);(BUILD/'compile.log').write_bytes(proc.stdout+proc.stderr);assert proc.returncode==0,proc.stderr.decode()
    runs=[]
    for p in sorted((ROOT/'validation/input_to_bat/build/n').glob('[0-9]*/reference.json')):
        ref=json.loads(p.read_text())
        if not ref['id'].startswith('jbo'):continue
        work=p.parent;script=work/'actions.tsv';nonce=uuid.uuid4().hex;events_path=BUILD/(ref['id']+'.jsonl');cmd_run=[str(exe),str(script),str(events_path),nonce]
        before_hash=sha(script.read_bytes());proc=subprocess.run(cmd_run,env={**os.environ,'LC_ALL':'C','LANG':'C','TEMPO2':str(N/'build/runtime')},capture_output=True,timeout=90)
        events=[json.loads(x) for x in events_path.read_text().splitlines()];assert events.pop(0)=={'phase':'launch','nonce':nonce}
        assert events==ref['events'],ref['id']+' state'
        assert proc.returncode==ref['returncode'] and proc.stderr.hex()==ref['stderr_hex']
        old=bytes.fromhex(ref['stdout_hex'])
        only_words=rb'\['+re.escape(str(original).encode())+rb':108\][^\[]*'
        assert re.sub(only_words,b'',old)==re.sub(only_words,b'',proc.stdout),(ref['id'],'change outside word diagnostics')
        output={'stdout_hex':proc.stdout.hex(),'stderr_hex':proc.stderr.hex()};out=BUILD/(ref['id']+'-streams.json');out.write_text(json.dumps(output)+'\n')
        assert sha(script.read_bytes())==before_hash
        runs.append({'id':ref['id'],'command':cmd_run,'nonce':nonce,'returncode':proc.returncode,'events_sha256':sha(events_path.read_bytes()),'actions_sha256':before_hash,'original_reference_sha256':sha(p.read_bytes()),'streams_sha256':sha(out.read_bytes()),'only_changed_diagnostics':'readJBO_bat.C:108','changed':old!=proc.stdout})
    n.execution_identity(Path(profile['executable_path']),profile)
    (BUILD/'receipt.json').write_text(json.dumps({'original_identity':identity,'original_source_sha256':sha(original.read_bytes()),'corrected_source_sha256':sha(source.read_bytes()),'executable_sha256':sha(exe.read_bytes()),'compile_command':cmd,'compile_log_sha256':sha((BUILD/'compile.log').read_bytes()),'runs':runs},indent=2)+'\n')
    print('JBO corrected native:',len(runs),'complete unchanged state histories; only word diagnostics differ')
if __name__=='__main__':main()
