"""Frozen source/type/write/ownership inventory. Does not execute native code."""
import hashlib
import json
from pathlib import Path
import re
import subprocess

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
PIN = 'fda6ad4a9753ad7dc5d291406f2f310a9161423f'
KINDS = ['clock_correction','observation','storePrecision','FitOutput','FitInfo','parameter','pulsar']
ROUTINES = {'initialise.C':['initialise','initialiseOne','allocateMemory','destroyOne','destroyMemory'],
 'tempo2Util.C':['copyPSR'], 'TKmatrix.C':['malloc_blas','free_blas','get_blas_rows','get_blas_cols','malloc_uinv','free_uinv']}

def blob(n): return subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':'+n])
def sha(b): return hashlib.sha256(b).hexdigest()
def save(p,x): p.write_text(json.dumps(x,indent=2,sort_keys=True)+'\n')
def uncomment(s):
    return re.sub(r'"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\'|//[^\n]*|/\*[\s\S]*?\*/',lambda m:re.sub(r'[^\n]',' ',m[0]) if m[0].startswith('/') else m[0],s)
def structures():
    text=uncomment(blob('tempo2.h').decode());out={}
    for kind in KINDS:
        m=re.search(r'typedef struct\s+'+(r'\{' if kind=='clock_correction' else kind+r'\s*\{')+r'(.*?)\}\s*'+kind+r';',text,re.S);assert m,kind
        rows=[]
        for d in m[1].split(';'):
            if not d.strip(): continue
            f=re.fullmatch(r'\s*((?:enum\s+)?\w+(?:\s+long)?)\s*(\**\s*.+?)\s*',d,re.S);assert f,d
            for part in f[2].split(','):
                p=re.fullmatch(r'\s*(\**)\s*(\w+)((?:\s*\[[^\]]+\])*)\s*',part);assert p,part
                rows.append(dict(name=p[2],type=f[1],pointers=len(p[1]),extents=re.findall(r'\[([^\]]+)\]',p[3]),declaration=f[1]+' '+part.strip(),line=text[:m.start(1)+m[1].index(d)].count('\n')+1))
        out[kind]=rows
    assert len(out['pulsar'])==349 and len(out['observation'])==95
    accepted=json.loads((ROOT/'validation/numeric_toolkit/inventory.json').read_text())['caller_fields']
    for k in ['pulsar','observation']: assert [r['name'] for r in out[k]]==accepted[k]
    return out

def source_functions():
    out=[]
    for file,names in ROUTINES.items():
        s=uncomment(blob(file).decode());lines=s.splitlines()
        for n in names:
            m=re.search(r'\b'+n+r'\s*\([^;{}]*\)\s*\{',s);assert m,n
            end=m.end();depth=1
            while depth:
                depth+=(s[end]=='{')-(s[end]=='}');end+=1
            a=s[:m.start()].count('\n')+1;b=s[:end].count('\n')+1
            out.append(dict(file=file,name=n,start=a,end=b,source='\n'.join(blob(file).decode().splitlines()[a-1:b]),branches=[dict(line=i+1,text=l.strip()) for i,l in enumerate(lines) if a<=i+1<=b and re.search(r'\b(if|else|for|while|assert)\b',l)]))
    return out

def build():
    types=structures();funcs=source_functions();init=next(x for x in funcs if x['name']=='initialiseOne');copy=next(x for x in funcs if x['name']=='copyPSR')
    si=uncomment(init['source']);sc=uncomment(copy['source'])
    for r in types['pulsar']:
        n=r['name'];anchors=[]
        for offset,line in enumerate(si.splitlines()):
            if re.search(r'psr->'+n+r'\b',line):anchors.append(dict(line=init['start']+offset,text=line.strip()))
        r['initializer_anchors']=anchors
        if n=='param':state='source allocation: all aSize/nLink counts, val/err/fitFlag/paramSet and full calloc label buffers; link arrays/prefit/prefitErr unwritten'
        elif n in ['obsn','covar']:state='source allocates; observation backing unwritten' if n=='obsn' else 'source malloc_blas MAX_FIT squared; all calloc element bytes zero'
        elif anchors:state='source writes string prefix through NUL; suffix unwritten' if r['type']=='char' and r['extents'] and n not in ['jumpSAT','jumpScaled'] else 'source writes complete scalar/array'
        else:state='unwritten by initializer'
        r['natural_state']=state
        r['copy_anchors']=[dict(line=copy['start']+i,text=l.strip()) for i,l in enumerate(sc.splitlines()) if re.search(r'\.'+n+r'\b',l)]
        r['copy_disposition']='partial source-order assignment; see anchors and declared counts' if r['copy_anchors'] and n not in ['nfdJumps','obsn'] else 'pointer retained; selected backing assigned forward' if n=='obsn' else 'retained'
    return dict(pin=PIN,types=types,functions=funcs,constants=re.findall(r'^#define\s+\w+[^\n]*',uncomment(blob('tempo2.h').decode()),re.M),enums=re.findall(r'enum\s+\w+\s*\{[^}]*\}',uncomment(blob('tempo2.h').decode()),re.S),callbacks=re.findall(r'typedef[^;]*\(\*[^;]*;',uncomment(blob('tempo2.h').decode())),source_sha256={n:sha(blob(n)) for n in ['tempo2.h',*ROUTINES,'TKmatrix.h']},ownership={
 'initialiseOne':'new observation malloc, real 10000x10000 covariance, 2000 parameter owners; repeated call overwrites pointers without freeing. Save aliases first; release old owners separately after observation.',
 'allocateMemory':'2000 slots, 9616 cells; independent 8 buffers plus 2 calloc labels per cell. realloc ignored; old allocations remain live.',
 'destroyOne':'if obsn free; free_blas covar; each of 2000 constraint_special pointers independently; destroyMemory. Never inspect pointers after destruction.',
 'destroyMemory':'slot order; all labels/shortlabels per current aSize then val err prefit prefitErr fitFlag paramSet label shortlabel.',
 'copyPSR':'distinct pulsar objects; source strings terminated; ordinary jump bound copied from source and inclusive; FD bound retained destination and inclusive. Shared observation owner handled once by harness; interior buffers detached before destruction.',
 'nested':'FitInfo/FitOutput/storePrec/tzrobs untouched; callback identities valid fixture functions, never invoked. Opaque pointer targets fixture-owned.'},classifications={
 'defined':'initialized typed caller storage, bounded counts, valid live ownership; fullSetup and realloc signed arguments ignored',
 'host_abi':'matrix dimension metadata via size_t in malloc storage; debug varargs %d with size_t, TKlog before-buffer formation; recorded as host diagnostic profile only',
 'assert_termination':'zero matrix rows or columns; prefix captured, no returned pointer read',
 'undefined_unrun':['unwritten or freed reads','same-pulsar or overlapping strcpy','unterminated string','capacity overflow','null/interior/foreign/double free','size multiplication overflow','invalid callback invocation'],
 'untested_defined':['resource exhaustion and OS allocator failure','other platform or numeric backend','full timing/fitting/plugin consumers'],
 'inactive':'commented observation zeroing, intended fullSetup conditional and commented copy allocation are not executed defaults'})

if __name__=='__main__':
    save(HERE/'inventory.json',build())
    paths=subprocess.check_output(['git','ls-files','validation'],cwd=ROOT,text=True).splitlines()
    excluded=('validation/pulsar_lifecycle/','validation/preprocessing/','validation/par_specialized/','validation/observation_storage_port/')
    mutable={'validation/parameter_storage_port/run.py','validation/parameter_storage_port/native.py','validation/parameter_storage_port/verify.py'}
    save(HERE/'protected_hashes.json',{p:sha((ROOT/p).read_bytes()) for p in paths if not p.startswith(excluded) and p not in mutable})
    print('Frozen 349 pulsar fields, 95 observation fields, seven nested types and twelve routines')
