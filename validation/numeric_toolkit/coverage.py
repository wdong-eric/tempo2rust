#!/usr/bin/env python3
"""O0/gcov coverage on the same bounded callers; not an O2 numerical reference."""
import gzip
import json
import subprocess
from pathlib import Path
from profile import HERE,BUILD,ENV,compile_probe,save,sha
from fixtures import cases
from run import execute,validate,execution_identity


def main():
    (HERE/'coverage.json').unlink(missing_ok=True)
    exe,p=compile_probe('coverage',coverage=True)
    functions={};lines={};branches={};commands=[]
    units=['probe','TKlongdouble','tempo2Util','TKlog','global_data']
    for i,c in enumerate(cases()):
        for f in BUILD.glob('coverage-*.gcda'):f.unlink()
        r=execute(c,exe,p);validate(c,r)
        for unit in units:
            cmd=['/opt/homebrew/bin/gcov-16','--json-format','--branch-probabilities',str(BUILD/('coverage-'+unit+'.gcno'))]
            if i==0:commands.append(cmd)
            proc=subprocess.run(cmd,cwd=BUILD,capture_output=True,text=True,check=True,env=ENV)
            data=json.loads(gzip.decompress((BUILD/('coverage-'+unit+'.gcov.json.gz')).read_bytes()))
            for file in data['files']:
                n=Path(file['file']).name
                if n not in ['TKlongdouble.C','tempo2Util.C','TKlongdouble.h','TKlog.C']:continue
                for f in file['functions']:
                    key=n+':'+f['name']
                    row=functions.setdefault(key,dict(file=n,name=f['demangled_name'],line=f['start_line'],end_line=f['end_line'],cases={}))
                    if f['execution_count']:row['cases'][c['id']]=row['cases'].get(c['id'],0)+f['execution_count']
                for line in file['lines']:
                    ln=line['line_number'];key=n+':'+str(ln)
                    row=lines.setdefault(key,dict(file=n,line=ln,count=0,cases=[]));row['count']+=line['count']
                    if line['count'] and c['id'] not in row['cases']:row['cases'].append(c['id'])
                    for j,b in enumerate(line.get('branches',[])):
                        k=key+':'+str(j);br=branches.setdefault(k,dict(file=n,line=ln,index=j,count=0,cases=[],throw=b.get('throw',False),fallthrough=b.get('fallthrough',False)))
                        br['count']+=b['count']
                        if b['count'] and c['id'] not in br['cases']:br['cases'].append(c['id'])
        if i%50==0:print('coverage',i+1,'/',len(cases()),flush=True)
    primary=[f for f in functions.values() if f['file'] in ['TKlongdouble.C','tempo2Util.C']]
    assert len(primary)==24,(len(primary),primary)
    assert all(f['cases'] for f in primary)
    summary={}
    for file in sorted({r['file'] for r in lines.values()}):
        ls=[r for r in lines.values() if r['file']==file];bs=[r for r in branches.values() if r['file']==file];fs=[r for r in functions.values() if r['file']==file]
        summary[file]=dict(lines=len(ls),lines_executed=sum(r['count']>0 for r in ls),unexecuted_lines=[r['line'] for r in ls if not r['count']],
            branches=len(bs),branches_executed=sum(r['count']>0 for r in bs),functions=len(fs),functions_executed=sum(bool(r['cases']) for r in fs))
    save(HERE/'coverage.json',dict(status='PASS',profile=p,execution=execution_identity(exe,p),gcov_commands=commands,gcov_sha256=sha(Path('/opt/homebrew/bin/gcov-16').read_bytes()),
        summary=summary,functions=list(functions.values()),lines=list(lines.values()),branches=list(branches.values()),
        scope='O0 instrumented reachability only; full exact reference replay uses O2. Exception/compiler-generated branches are included, not claimed as all scientific decisions. Unsafe/unselected cases never run.'))
    print('PASS: 24/24 primary functions executed;',summary)


if __name__=='__main__':main()
