#!/usr/bin/env python3
"""Measured function/case membership on raw GCC binary128 coverage build.

O0/gcov execution counts are reachability evidence, not the O2 numeric oracle.
Only permitted ordinary/exit cases run; withheld cases are never executed.
"""
import gzip
import json
import subprocess
from pathlib import Path
from profile import BUILD, HERE, ENV, prepare, save, sha
from fixtures import make_cases
from run import compile_probe, execute, validate_case


def gcov(unit):
    cmd=['/opt/homebrew/bin/gcov-16','--json-format','--branch-probabilities',str(BUILD/('coverage-'+unit+'.gcno'))]
    proc=subprocess.run(cmd,cwd=BUILD,text=True,capture_output=True,check=True,env=ENV)
    (BUILD/('coverage-'+unit+'.log')).write_text(proc.stdout+proc.stderr)
    return json.loads(gzip.decompress((BUILD/('coverage-'+unit+'.gcov.json.gz')).read_bytes()))


def main():
    src,cc,flags,_=prepare()
    exe,profile=compile_probe(src,cc,[f for f in flags if f!='-O2']+['-O0','--coverage'],'coverage')
    for p in BUILD.glob('coverage-*.gcda'):p.unlink()
    rows={};lines={};executed=[]
    for case in make_cases():
        if case['classification'] in ('undefined','bounded'):continue
        (BUILD/'coverage-probe.gcda').unlink(missing_ok=True)
        result=execute(case,exe);validate_case(case,result);executed.append(case['id'])
        data=gcov('probe')
        for f in data['files']:
            name=Path(f['file']).name
            if name not in ('tabulatedfunction.C','clkcorr.C','toa2utc.C'):continue
            for fun in f['functions']:
                key=name+':'+fun['name']
                if key not in rows:rows[key]=dict(source=name,line=fun['start_line'],end_line=fun['end_line'],name=fun['demangled_name'],case_counts={})
                if fun['execution_count']:rows[key]['case_counts'][case['id']]=fun['execution_count']
            counts=lines.setdefault(name,{})
            for line in f['lines']:counts[line['line_number']]=counts.get(line['line_number'],0)+line['count']
    functions=sorted(rows.values(),key=lambda x:(x['source'],x['line'],x['name']))
    assert len(functions)==32,len(functions) # 4 table + 4 public + 18 helpers + 3 constructors + 2 lambdas + toa2utc
    assert all(f['case_counts'] for f in functions),[f['name'] for f in functions if not f['case_counts']]
    dependencies=[]
    for unit in ('observatory','TKlog','TKlongdouble','tempo2Util','global_data'):
        for f in gcov(unit)['files']:
            if Path(f['file']).name!=unit+'.C':continue
            dependencies += [dict(source=unit+'.C',name=fn['demangled_name'],line=fn['start_line'],calls=fn['execution_count']) for fn in f['functions']]
    result=dict(status='PASS',scope='Function execution membership only; counts include read-only harness snapshots and public/helper calls. Unexecuted source lines are listed; no claim of full branch/path coverage.',
                build=profile,cases=executed,functions=functions,dependencies=dependencies,
                lines={name:dict(executable=len(counts),executed=sum(n>0 for n in counts.values()),unexecuted=[i for i,n in sorted(counts.items()) if not n]) for name,counts in lines.items()})
    save(HERE/'outputs/coverage.json',result)
    print('PASS: 32/32 primary function/constructor/lambda entries executed with per-case counts;',len(executed),'permitted cases')


if __name__=='__main__':main()
