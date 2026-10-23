"""Complete ordered primary dispatch inventory; dispositions are not runtime coverage."""
import re
from profile import HERE, blob, uncomment, enum_names, sha, save


def inventory():
    functions = []
    for filename in ['initialise.C', 'readParfile.C']:
        raw = blob(filename).decode()
        s = uncomment(raw)
        matches = list(re.finditer(r'^(?:static )?(?:void|int)\s+(\w+)\s*\([^;{}]*\)\s*\{', s, re.M))
        for m in matches:
            functions.append(dict(file=filename, name=m[1], line=s[:m.start()].count('\n')+1))
    assert len(functions) == 15
    s = uncomment(blob('readParfile.C').decode())
    start = s.index('void checkLine(', s.index('void readParfileGlobal'))
    end = s.index('static void checkBinaryParameterConsistency', start)
    segment = s[start:end]
    tokens = list(re.finditer(r'"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\'|\b\w+\b|[^\s]', segment))
    depth = 0
    branches = []
    decisions = []
    for i,t in enumerate(tokens):
        if t[0]=='{': depth+=1
        elif t[0]=='}': depth-=1
        elif t[0]=='if':
            paren = 0
            for j in range(i+1,len(tokens)):
                if tokens[j][0]=='(': paren+=1
                elif tokens[j][0]==')':
                    paren-=1
                    if paren==0: break
            a,b=t.start(),tokens[j].end()
            decisions.append(dict(line=s[:start+a].count('\n')+1,brace_depth=depth,condition=' '.join(segment[a:b].split())))
            if depth==1:
                branches.append(dict(id=f'D{len(branches):03d}',line=s[:start+a].count('\n')+1,
                                     offset=a, condition=' '.join(segment[a:b].split()), condition_end=b))
    # Explicit final else is a dispatch disposition too.
    tail = segment.rfind('else\n')
    branches.append(dict(id=f'D{len(branches):03d}',line=s[:start+tail].count('\n')+1,
                         offset=tail, condition='else (unknown token)', condition_end=tail+4))
    for i,b in enumerate(branches):
        stop = branches[i+1]['offset'] if i+1<len(branches) else len(segment)
        body=segment[b['condition_end']:stop]
        b['end_line']=s[:start+stop].count('\n')+1
        b['body_line']=s[:start+b['condition_end']].count('\n')+1
        b['parameters']=list(dict.fromkeys(re.findall(r'\bparam_\w+',body)))
        b['keywords']=re.findall(r'(?:strcasecmp|strncasecmp|strstr)\s*\(\s*str\s*,\s*"([^"]+)"', b['condition'])
        b['matching'] = [n for n in ['strcasecmp','strncasecmp','strstr','str['] if n in b['condition']]
        b['readValue_calls']=[' '.join(x.split()) for x in re.findall(r'readValue\s*\([^;]+;',body)]
        b['scans']=[' '.join(x.split()) for x in re.findall(r'(?:sscanf|fscanf|fgets)\s*\([^;]+;',body)]
        b['disposition']='shared scalar/index candidate' if b['readValue_calls'] else 'deferred specialized/control branch; see coverage.json'
        b['source_sha256']=sha(body.encode())
        del b['offset'], b['condition_end']
    out=dict(source_revision='fda6ad4a9753ad7dc5d291406f2f310a9161423f',functions=functions,
             parameter_enum=[dict(index=i,name=n,kind='physical' if i<enum_names().index('param_LAST') else 'marker/virtual') for i,n in enumerate(enum_names())],
             dispatch=branches, nested_decisions=decisions,
             parameter_fields=uncomment(blob('tempo2.h').decode()).split('typedef struct parameter {')[1].split('} parameter;')[0].strip(),
             initializer_assignments=[dict(line=i+1,source=l.strip()) for i,l in enumerate(uncomment(blob('initialise.C').decode()).splitlines()) if 'psr->' in l],
             limits='Lexical ordered top-level chain plus complete nested condition/scan source anchors. Runtime coverage is separately measured, not inferred from catalog membership.')
    # Include all nested decisions for specialized branches, without pretending they are top-level aliases.
    out['all_keyword_tests']=[dict(line=segment[:m.start()].count('\n')+1,test=m[0]) for m in re.finditer(r'(?:strcasecmp|strncasecmp|strcmp|strstr)\s*\([^\n;{}]+',s[start:end])]
    for row in out['all_keyword_tests']: row['line']+=s[:start].count('\n')
    return out


if __name__=='__main__':
    out=inventory()
    save(HERE/'inventory.json',out)
    print(f"{len(out['parameter_enum'])} enum entries, {len(out['functions'])} functions, {len(out['dispatch'])} ordered dispatch branches")
