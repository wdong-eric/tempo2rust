"""Bounded source-derived remove_white correction; never execute an overlapping strcpy."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
PIN = 'fda6ad4a9753ad7dc5d291406f2f310a9161423f'
SOURCE_SHA = '40184b982564c13a4c02865d04045139c078edbd08a9ec40fc25c2265c7bf1d3'
CC = Path('/opt/homebrew/bin/g++-16').resolve()
ENV = {**os.environ, 'LC_ALL': 'C', 'LANG': 'C'}
sha = lambda b: hashlib.sha256(b).hexdigest()

def git(*args):
    return subprocess.check_output(['git', '-C', str(ROOT/'tempo2'), *args])

assert git('rev-parse', 'HEAD').decode().strip() == PIN
before = git('status', '--porcelain')
assert before == b''
raw = git('show', PIN+':get_obsCoord.C')
assert sha(raw) == SOURCE_SHA
start = raw.index(b'void remove_white(char *str)\n{')
end = raw.index(b'\n}', start)+2
body = raw[start:end]
old = b'strcpy(str+i,str+i+1);'
new = b'memmove(str+i,str+i+1,strlen(str+i+1)+1);'
assert body.count(old) == 3
fixed = body.replace(old, new)
# Minimal raw-function extraction: only libc string operations are dependencies.
preamble = b'#include <string.h>\n'
(HERE/'build').mkdir(exist_ok=True)
out = Path(tempfile.mkdtemp(prefix='central-', dir=HERE/'build'))
driver = out/'driver.cc'
driver.write_text(r'''
#include <stdio.h>
#include <string.h>
void remove_white(char *);
int main(int argc,char **argv) {
    if(argc!=2 || strlen(argv[1])%2 || strlen(argv[1])>126) return 2;
    unsigned char storage[66]; memset(storage,0xa5,sizeof(storage));
    unsigned char *buf=storage+1;
    size_t n=strlen(argv[1])/2;
    for(size_t i=0;i<n;i++) { unsigned x; if(sscanf(argv[1]+2*i,"%2x",&x)!=1) return 3; buf[i]=(unsigned char)x; }
    buf[n]=0;
    remove_white((char *)buf);
    for(size_t i=0;i<sizeof(storage);i++) printf("%02x",storage[i]);
    puts("");
}
''')
# The first seven are exactly X's no-shift inputs; original is run only on these.
unchanged = [b'', b'x', b'a b', b'a\tb', b'a b ', b'a\tb\t', b'a\rb']
cases = [(s,s) for s in unchanged] + [
    (b' \t  fred',b'fred'),
    (b'fred    blogs\t\tjoe',b'fred blogs\tjoe'),
    (b'a\t \t b',b'a\tb'),
    (b'a \t \t',b'a '),
    (b'\n \t\na',b'a'),
    (b'a\nb',b'ab'),
    (b'a \n\tb',b'a b'),
    (b' \ta\rb\n',b'a\rb'),
    (b'\xff  \xfe\n',b'\xff \xfe'),
    (b' \t \n',b''),
    (b'\n',b''),
    (b'a b\x00 \n suffix',b'a b'),
]
profiles, records = {}, {}
for label, function in [('original',body),('corrected',fixed)]:
    source=out/(label+'.cc'); source.write_bytes(preamble+function+b'\n')
    exe=out/label
    command=[str(CC),'-O2','-std=gnu++11',str(source),str(driver),'-o',str(exe)]
    built=subprocess.run(command,capture_output=True,env=ENV)
    (out/(label+'.compile.log')).write_bytes(built.stdout+built.stderr)
    assert built.returncode==0, built.stderr
    identities={str(p):sha(p.read_bytes()) for p in (CC,source,driver,exe)}
    profiles[label]={'command':command,'identities':identities,
                     'dynamic_libraries':subprocess.check_output(['otool','-L',str(exe)],text=True)}
    records[label]=[]
    for data, expected in (cases if label=='corrected' else cases[:len(unchanged)]):
        before_buffer=bytes([0xa5])+data+b'\x00'+bytes([0xa5])*(64-len(data))
        assert len(before_buffer)==66
        runs=[]
        for _ in range(2):
            assert all(sha(Path(p).read_bytes())==h for p,h in identities.items())
            run=subprocess.run([str(exe),data.hex()],capture_output=True,env=ENV)
            assert run.returncode==0 and run.stderr==b''
            runs.append(run.stdout)
        assert runs[0]==runs[1]
        observed=bytes.fromhex(runs[0].decode().strip())
        assert len(observed)==66 and observed[0]==0xa5 and observed[-1]==0xa5
        assert observed[1:].split(b'\x00',1)[0]==expected, (data,observed,expected)
        # Shifts include the old NUL, but never touch bytes following it.
        nul=before_buffer.index(0)
        assert observed[nul+1:]==before_buffer[nul+1:]
        if label=='original': assert observed==before_buffer
        records[label].append({'input_hex':data.hex(),'expected_prefix_hex':expected.hex(),
                               'buffer_before_hex':before_buffer.hex(),'buffer_after_hex':observed.hex()})
    assert all(sha(Path(p).read_bytes())==h for p,h in identities.items())
assert records['corrected'][:len(unchanged)]==records['original']
# For source input "  x", the first copy includes a space, x and the NUL:
# destination [0,3), source [1,4); their nonempty intersection proves invalid strcpy.
copy_length=len(b'  x'[1:])+1
assert copy_length==3 and set(range(copy_length)) & set(range(1,copy_length+1))
assert git('status','--porcelain')==before
assert git('show',PIN+':get_obsCoord.C')==raw
receipt={'status':'PASS','pin':PIN,'source_sha256':SOURCE_SHA,
         'raw_function_sha256':sha(body),'corrected_function_sha256':sha(fixed),
         'script_sha256':sha(Path(__file__).read_bytes()),'profiles':profiles,'records':records,
         'original_cases':len(unchanged),'corrected_cases':len(cases),'runs_each':2,
         'patch':{'old':old.decode(),'new':new.decode(),'sites':[464,469,474]},
         'compiler_version':subprocess.check_output([str(CC),'--version'],text=True),
         'host':subprocess.check_output(['sw_vers'],text=True),
         'scope':'Routine-only corrected native experiment; overlapping original never executed; no full X replay or Rust acceptance.'}
r=out/'receipt.json'; r.write_text(json.dumps(receipt,indent=2)+'\n')
print('PASS',len(unchanged),'original,',len(cases),'corrected; each twice')
print(out)
print('receipt_sha256',sha(r.read_bytes()))
