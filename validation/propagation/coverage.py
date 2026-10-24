"""Measured O2/gcov source reachability with an exact ordinary-profile comparison."""
import gzip
import json
import subprocess
from pathlib import Path
from inventory import HERE, PRIMARY, sha, save, check
from profile import BUILD, ENV, compile_probe, actual_identity
from fixtures import cases, UNRUN
from combined import load
from controls import differences as field_differences
from run import OUTPUT, execute, validate, read_gzip, write_gzip, compare, frozen_cases

def disposition(file,line,throw=False):
    if throw:return 'fault-injection-unverified','compiler exception/unwind edge; allocation/I/O fault injection excluded'
    if file=='ifunc.C' and line in (8,42,43,53,54):
        return 'terminal-counter-unavailable','basis/NaN assertion cases execute terminal paths but abort drops gcov counters; no per-edge claim'
    if file=='sw_delay.C' and line==119:
        return 'native-invalid-unrun','eclCoord outside exact 0/1 leaves solarWindModel automatic coordinates uninitialized'
    if file=='sw_delay.C' and line in (588,604):
        return 'outside-caller-domain-unverified','invalid month diagnostic is not produced by supported solar dates; exotic integer MJD inputs unverified'
    if file=='sw_delay.C' and line in (415,435,444,445,455):
        return 'defined-outcome-unverified','unhit mcl2 clamp/modulo/rotation boundary; explicit finite geometry gap'
    if file=='refphs.C' and line==12:
        return 'defined-outcome-unverified','TKlog storage saturation specifically at the phase-capacity diagnostic is unverified'
    return 'defined-outcome-unverified','unhit source/compiler outcome retained as an explicit gap'

def main():
    destination=OUTPUT/'coverage.json';destination.unlink(missing_ok=True);check();preservation_before={}
    oracle=OUTPUT/'reference.json.gz';oracle_hash=sha(oracle.read_bytes())
    catalog,combined_results=load();ordinary={r['id']:r for r in combined_results}
    compare([c['id'] for c in catalog],list(ordinary),'oracle membership')
    exe,profile=compile_probe('coverage',coverage=True)
    for p in exe.parent.glob('*.gcda'):p.unlink()
    agreements=[];differences=[];receipts=[]
    for c in catalog:
        r,receipt=execute(c,exe,profile,OUTPUT/'coverage_raw');validate(c,r);receipts.append(receipt)
        try:compare(ordinary[c['id']],r,c['id']);agreements.append(c['id'])
        except AssertionError as e:differences.append(dict(case=c['id'],difference=str(e),fields=field_differences(ordinary[c['id']],r)))
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
        fn=next(x for x in f['functions'] if x['demangled_name'].split('(')[0]==routine['name'] and x['start_line']==routine['start'])
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
        if Path(f['file']).name in ('tabulatedfunction.C','clkcorr.C','TKlog.C','tempo2Util.C','TKlongdouble.C','global_data.C','tropo.C','observatory.C'):
            dependencies.append(dict(file=f['file'],functions=[x for x in f['functions'] if x['execution_count']]))
    write_gzip(OUTPUT/'coverage_receipts.json.gz',receipts)
    source_decisions=[]
    for routine in inv['routines']:
        for d in routine['decisions']:
            source_decisions.append(dict(file=routine['file'],routine=routine['name'],**d,
                measured_lines=[l for l in lines if l['file']==routine['file'] and l['line_number']==d['line']]))
    check();preservation_after={}
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
    # Exact disagreements remain exposed; coverage never replaces the O2 oracle.
    assert not preservation_before and not preservation_after, 'native coverage completed; preservation gate remains unresolved'

if __name__=='__main__':main()
