"""O2/gcov primary coverage; every unhit line/edge has an explicit disposition."""
import gzip
import json
import subprocess
from pathlib import Path
from inventory import HERE,sha,save,check
from profile import compile_probe,resources,ENV,actual_identity
from run import OUTPUT,frozen_cases,read_gzip,write_gzip,execute,validate,compare
from controls import differences

def disposition(line,throw=False):
    if throw:return 'fault-unrun','compiler unwind/exception edge; no allocation fault injection'
    if line==543:return 'unreachable-defined-domain','0 <= ix3 < 243000 implies 0 <= j <= 96; no deliberate static corruption'
    if line==561:return 'defined-unsampled','rsq==0 outcome not forced; measured nonzero rejection and acceptance histories retained'
    return 'defined-unsampled','unhit source/compiler outcome; no full branch claim'

def main():
    check();catalog=frozen_cases();base=read_gzip(OUTPUT/'reference.json.gz');old=sha((OUTPUT/'reference.json.gz').read_bytes());rm=resources();exe,p=compile_probe('coverage',coverage=True)
    for f in exe.parent.glob('*.gcda'):f.unlink()
    exact=[];diffs=[];receipts=[]
    for c,expected in zip(catalog,base):
        actual,receipt=execute(c,exe,p,OUTPUT/'coverage_raw',rm);validate(c,actual);receipts.append(receipt)
        ds=differences(expected,actual)
        if ds:diffs.append(dict(case=c['id'],fields=ds))
        else:exact.append(c['id'])
    cmd=['/opt/homebrew/bin/gcov-16','--json-format','--branch-probabilities',str(exe.parent/'readEphemeris.gcno')]
    r=subprocess.run(cmd,cwd=exe.parent,capture_output=True,text=True,env=ENV);assert r.returncode==0,r.stderr
    data=json.loads(gzip.decompress((exe.parent/'readEphemeris.gcov.json.gz').read_bytes()))
    f=next(f for f in data['files'] if Path(f['file']).name=='readEphemeris.C');lines=[];edges=[]
    for l in f['lines']:
        row=dict(l)
        if not l['count']:row['classification'],row['reason']=disposition(l['line_number'])
        lines.append(row)
        for i,b in enumerate(l.get('branches',[])):
            cl,reason=('executed','measured count') if b['count'] else disposition(l['line_number'],b.get('throw',False))
            edges.append(dict(line=l['line_number'],index=i,classification=cl,reason=reason,**b))
    write_gzip(OUTPUT/'coverage_receipts.json.gz',receipts)
    check();actual_identity(exe,p);assert sha((OUTPUT/'reference.json.gz').read_bytes())==old
    save(OUTPUT/'coverage.json',dict(status='PROFILE_DIFFERENCE' if diffs else 'PASS',profile=p,command=cmd,gcov_sha256=sha(Path(cmd[0]).read_bytes()),ordinary_exact_cases=exact,ordinary_differences=diffs,routines=f['functions'],lines=lines,branches=edges,summary=dict(routines=len(f['functions']),routines_entered=sum(x['execution_count']>0 for x in f['functions']),lines=len(lines),lines_hit=sum(x['count']>0 for x in lines),branches=len(edges),branches_hit=sum(x['count']>0 for x in edges))))
    print('Coverage:',len(exact),'exact,',len(diffs),'profile differences;',len(lines),'lines,',len(edges),'edges')
if __name__=='__main__':main()
