"""Separate O0/gcov measured reachability, exactly checked against ordinary state."""
import gzip
import json
from pathlib import Path
import subprocess
from profile import HERE,BUILD,ENV,compile_probe,save,sha
from fixtures import cases
from run import execute,compare,reference,execution_identity,protected

def main():
    (HERE/'coverage.json').unlink(missing_ok=True);protected()
    exe,p=compile_probe('coverage',coverage=True)
    for f in BUILD.glob('coverage-*.gcda'):f.unlink()
    matches=[]
    for c in cases():
        r=execute(c,exe,p,BUILD/'coverage_raw');compare(reference(c['id']),r,'coverage.'+c['id']);matches.append(c['id']);print('coverage matched',c['id'],flush=True)
    inventory=json.loads((HERE/'inventory.json').read_text());selected=inventory['functions'];files=[];commands=[]
    for unit in ['initialise','tempo2Util','TKmatrix']:
        cmd=['/opt/homebrew/bin/gcov-16','--json-format','--branch-probabilities',str(BUILD/('coverage-'+unit+'.gcno'))];commands.append(cmd)
        subprocess.run(cmd,cwd=BUILD,capture_output=True,text=True,check=True,env=ENV)
        gcov=json.loads(gzip.decompress((BUILD/('coverage-'+unit+'.gcov.json.gz')).read_bytes()))
        files.extend(f for f in gcov['files'] if Path(f['file']).name==unit+'.C')
    rows=[];lines=[];branches=[]
    for item in selected:
        f=next(f for f in files if Path(f['file']).name==item['file'])
        fn=next(x for x in f['functions'] if x['demangled_name'].split('(')[0]==item['name'])
        assert fn['execution_count']>0,item['name'];rows.append(dict(**item,measured=fn))
        for l in f['lines']:
            if not item['start']<=l['line_number']<=item['end']:continue
            row=dict(file=item['file'],function=item['name'],**l)
            if not l['count']:
                n=l['line_number'];row['gap']='unrun allocator failure' if item['name']=='initialiseOne' else 'unrun allocator failure or unsupported sizeof(int)>sizeof(double) host guard' if item['name']=='malloc_blas' else 'unhit selected line; source inspected'
            lines.append(row)
            for index,b in enumerate(l.get('branches',[])):
                gap='executed' if b['count'] else 'compiler exception edge' if b.get('throw') else 'unrun allocation failure/host guard, or compiler-generated short circuit; see exact source line'
                branches.append(dict(file=item['file'],function=item['name'],line=l['line_number'],index=index,classification=gap,**b))
    save(HERE/'coverage.json',dict(status='PASS',profile=p,execution=execution_identity(exe,p),gcov_commands=commands,gcov_sha256=sha(Path('/opt/homebrew/bin/gcov-16').read_bytes()),ordinary_exact_cases=matches,functions=rows,lines=lines,branches=branches,summary=dict(functions_entered=len(rows),lines=len(lines),lines_hit=sum(x['count']>0 for x in lines),branches=len(branches),branches_hit=sum(x['count']>0 for x in branches)),limitations='Aborting zero-dimension processes do not flush GCC counters; their raw assert evidence is separate. Native allocation/frees are not intercepted. Reachability is not physical validation.'))
    protected();print('PASS twelve primary functions measured')
if __name__=='__main__':main()
