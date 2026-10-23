"""Source-derived complete definition/condition/caller inventory, no execution claims."""
import re,subprocess
from profile import blob,sha,PRIMARY,ROOT,PIN,HERE,save,fields

def uncomment(s):
    return re.sub(r'"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\'|//[^\n]*|/\*[\s\S]*?\*/',lambda m:re.sub(r'[^\n]',' ',m[0]) if m[0].startswith('/') else m[0],s)

def inventory():
    out={}
    for f in PRIMARY:
        raw=blob(f)
        expected={'readTimfile.C':'01463be0d78e75078cdbba7ae7fcd578e5d7db08fbec476d4f289488803a1ea2','sortToAs.C':'59dea2a1e05373edb049725379c90be41939618a261b6601e5026acaac82ec49','readJBO_bat.C':'0c7046c8f5db8e19829dfe70bab90c6fcc44ae16c5476c6fb321eb17b393b956','read_fortran2.h':'c42706c8c960dfc4b9693e5f3f33b03549f40ee1e40371c33f0628c49e722fca'}
        assert sha(raw)==expected[f]
        s=uncomment(raw.decode());routines=[]
        for m in re.finditer(r'^(?:void|int|float|double)\s+(\w+)\s*\(([^)]*)\)\s*\{',s,re.M):
            depth=1;i=m.end()
            while depth:
                if s[i]=='{':depth+=1
                if s[i]=='}':depth-=1
                i+=1
            start=s[:m.start()].count('\n')+1;end=s[:i].count('\n')+1
            body=s[m.start():i]
            routines.append(dict(name=m[1],arguments=m[2],start=start,end=end,
                conditions=[dict(line=start+body[:x.start()].count('\n'),text=x[0]) for x in re.finditer(r'\b(?:if|else if|for|while|switch)\s*\([^\n]*|\bcase\s+[^:]+:|\belse\b',body)],
                accesses=sorted(set(re.findall(r'(?:psr(?:\[[^\]]*\])?->|psr\[[^\]]*\]\.)(\w+)',body)))))
        out[f]=dict(sha256=sha(raw),lines=len(s.splitlines()),routines=routines,preprocessor=[dict(line=i+1,text=l) for i,l in enumerate(s.splitlines()) if l.lstrip().startswith('#')])
    assert sum(len(v['routines']) for v in out.values())==19
    names=sorted({r['name'] for v in out.values() for r in v['routines']})
    callers=subprocess.run(['git','-C',str(ROOT/'tempo2'),'grep','-n','-E',r'('+ '|'.join(names)+r')[[:space:]]*\(',PIN,'--','*.C','*.h'],capture_output=True,text=True,check=True).stdout.splitlines()
    return dict(pin=PIN,files=out,routine_count=19,caller_candidates=callers,caller_method='Raw pinned git grep; definitions/prototypes/comments included, no implied execution',observation_fields=fields('observation'),initializer_source_sha256=sha(blob('initialise.C')),classifications_document_sha256=sha((HERE/'SOURCE_MAP.md').read_bytes()))

if __name__=='__main__':save(HERE/'inventory.json',inventory())
