"""Measured O2/gcov source reachability with an exact ordinary-profile comparison."""
import gzip
import json
import subprocess
from pathlib import Path
from inventory import HERE, PRIMARY, sha, save, preservation_status
from profile import BUILD, ENV, compile_probe, actual_identity
from fixtures import cases, UNRUN
from run import OUTPUT, execute, validate, read_gzip, write_gzip, compare, frozen_cases

def disposition(file,line,throw=False):
    if throw:return 'host-dependent','compiler exception/unwind edge; allocation/I/O exceptions not injected'
    if file=='eop.C':
        if line in (159,160):return 'native-invalid','upper endpoint reaches samp[size]; intentionally unrun'
        if line in (102,109):return 'inactive','format is assigned only 0 or 1; false continuation unreachable'
    if file=='observatory.C':
        if line in (296,297):return 'inactive','successful lookup_observatory_alias initializes before this test'
        if 125<=line<=139 or 321<=line<=366:return 'inactive','literal #if 0'
    if file=='tropo.C':
        if line in (273,274,276,277):return 'host-dependent','glob allocation/read failure not injected'
        if line in (282,283):return 'native-invalid','nonzero-warning no-match continuation reaches null host gl_pathv'
        if 381<=line<=398:return 'inactive','literal #if 0'
    return 'still-defined-unverified','unhit source/compiler outcome; exact source line and branch index retained, no full-path claim'

def main():
    destination=OUTPUT/'coverage.json';destination.unlink(missing_ok=True);preservation_before=preservation_status()
    oracle=OUTPUT/'reference.json.gz';oracle_hash=sha(oracle.read_bytes())
    ordinary={r['id']:r for r in read_gzip(oracle)}
    catalog=frozen_cases();compare([c['id'] for c in catalog],list(ordinary),'oracle membership')
    exe,profile=compile_probe('coverage',coverage=True)
    for p in exe.parent.glob('*.gcda'):p.unlink()
    agreements=[];differences=[];receipts=[]
    for c in catalog:
        r,receipt=execute(c,exe,profile,OUTPUT/'coverage_raw');validate(c,r);receipts.append(receipt)
        try:compare(ordinary[c['id']],r,c['id']);agreements.append(c['id'])
        except AssertionError as e:differences.append(dict(case=c['id'],difference=str(e)))
    files=[];commands=[]
    for gcno in sorted(exe.parent.glob('*.gcno')):
        cmd=['/opt/homebrew/bin/gcov-16','--json-format','--branch-probabilities',str(gcno)]
        r=subprocess.run(cmd,cwd=exe.parent,capture_output=True,text=True,env=ENV);assert r.returncode==0,r.stderr
        commands.append(cmd)
        data=json.loads(gzip.decompress(gcno.with_suffix('.gcov.json.gz').read_bytes()))
        files.extend(data['files'])
    inv=json.loads((OUTPUT/'frozen/source_map.json').read_text());routines=[];lines=[];branches=[]
    for routine in inv['routines']:
        f=next(f for f in files if Path(f['file']).name==routine['file'])
        fn=next(x for x in f['functions'] if x['demangled_name'].split('(')[0]==routine['name'])
        routines.append(dict(file=routine['file'],name=routine['name'],measured=fn))
        for item in f['lines']:
            n=item['line_number']
            if not routine['start']<=n<=routine['end']:continue
            row=dict(file=routine['file'],function=routine['name'],**item)
            if not item['count']:row['classification'],row['reason']=disposition(routine['file'],n)
            lines.append(row)
            for index,b in enumerate(item.get('branches',[])):
                classification,reason=('executed','measured count') if b['count'] else disposition(routine['file'],n,b.get('throw',False))
                branches.append(dict(file=routine['file'],function=routine['name'],line=n,index=index,classification=classification,reason=reason,**b))
    dependencies=[]
    for f in files:
        if Path(f['file']).name in ('tabulatedfunction.C','clkcorr.C','TKlog.C','tempo2Util.C','TKlongdouble.C','global_data.C'):
            dependencies.append(dict(file=f['file'],functions=[x for x in f['functions'] if x['execution_count']]))
    write_gzip(OUTPUT/'coverage_receipts.json.gz',receipts)
    source_decisions=[]
    for routine in inv['routines']:
        for d in routine['decisions']:
            source_decisions.append(dict(file=routine['file'],routine=routine['name'],**d,
                measured_lines=[l for l in lines if l['file']==routine['file'] and l['line_number']==d['line']]))
    preservation_after=preservation_status()
    result=dict(status='PROFILE_DIFFERENCE' if differences else 'PRESERVATION_DRIFT' if preservation_before or preservation_after else 'PASS',
        native_status='PROFILE_DIFFERENCE' if differences else 'PASS',preservation_before=preservation_before,preservation_after=preservation_after,profile=profile,
        actual_identity=actual_identity(exe,profile),gcov_commands=commands,
        gcov_sha256=sha(Path('/opt/homebrew/bin/gcov-16').read_bytes()),
        ordinary_exact_cases=agreements,ordinary_differences=differences,
        routines=routines,lines=lines,branches=branches,source_decisions=source_decisions,
        selected_dependencies=dependencies,unrun=UNRUN,
        summary=dict(routines=len(routines),routines_entered=sum(r['measured']['execution_count']>0 for r in routines),
            lines=len(lines),lines_hit=sum(l['count']>0 for l in lines),
            branches=len(branches),branches_hit=sum(b['count']>0 for b in branches)))
    save(destination,result);assert sha(oracle.read_bytes())==oracle_hash
    print(result['status'],result['summary'],len(agreements),'exact cases;',differences[:3])
    assert not differences, 'coverage profile differs; no projection authorized'
    assert not preservation_before and not preservation_after, 'native coverage completed; preservation gate remains unresolved'

if __name__=='__main__':main()
