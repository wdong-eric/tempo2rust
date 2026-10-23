#!/usr/bin/env python3
"""Separate instrumented execution; per-case gcov mapping, never inferred coverage."""
import gzip,json,subprocess
from pathlib import Path
from profile import HERE,BUILD,PRIMARY,compile_probe,save
from run import execute,validate,compare,execution_identity,runtime,read_observations

def main():
    cs=json.loads((HERE/'cases.json').read_text());gold=read_observations()
    (HERE/'coverage.json').unlink(missing_ok=True)
    for f in BUILD.glob('coverage-*.gcda'):f.unlink()
    exe,p=compile_probe('coverage',coverage=True);p['stable']['runtime_fixtures']=runtime()
    files={};agreements=[]
    for c,g in zip(cs,gold):
        for f in BUILD.glob('coverage-*.gcda'):f.unlink()
        r=execute(c,exe,p);validate(c,r)
        try:compare(g,r);agreements.append(dict(case=c['id'],comparison='exact'))
        except AssertionError as e:agreements.append(dict(case=c['id'],comparison='instrumentation differs',difference=str(e),instrumented_observation=r))
        for unit in ['readParfile.C','initialise.C','probe.cc']:
            stem=Path(unit).stem;gcno=BUILD/('coverage-'+stem+'.gcno')
            cmd=['/opt/homebrew/bin/gcov-16','--json-format','--branch-probabilities','--branch-counts',str(gcno)]
            proc=subprocess.run(cmd,cwd=BUILD,capture_output=True,text=True,check=True)
            data=json.loads(gzip.decompress((BUILD/('coverage-'+stem+'.gcov.json.gz')).read_bytes()))
            for f in data['files']:
                name=Path(f['file']).name
                if name not in PRIMARY:continue
                out=files.setdefault(name,dict(lines={},functions={}))
                for line in f['lines']:
                    n=str(line['line_number']);o=out['lines'].setdefault(n,dict(count=0,cases=[],branches=[]));o['count']+=line['count']
                    if line['count']:o['cases'].append(c['id'])
                    for i,b in enumerate(line.get('branches',[])):
                        while len(o['branches'])<=i:o['branches'].append(dict(count=0,cases=[],throw=b.get('throw',False),fallthrough=b.get('fallthrough',False)))
                        ob=o['branches'][i];ob['count']+=b['count']
                        if b['count']:ob['cases'].append(c['id'])
                for fun in f['functions']:
                    fn=fun['demangled_name'];o=out['functions'].setdefault(fn,dict(start=fun['start_line'],end=fun['end_line'],calls=0,cases=[]));o['calls']+=fun['execution_count']
                    if fun['execution_count']:o['cases'].append(c['id'])
        if len(agreements)%10==0:print('coverage',len(agreements),len(cs),flush=True)
    summary={}
    for name,f in files.items():
        ls=list(f['lines'].values());bs=[b for l in ls for b in l['branches']];fs=list(f['functions'].values())
        summary[name]=dict(lines_hit=sum(l['count']>0 for l in ls),lines_total=len(ls),branches_hit=sum(b['count']>0 for b in bs),branches_total=len(bs),functions_hit=sum(f['calls']>0 for f in fs),functions_total=len(fs),unexecuted_lines=[int(n) for n,l in f['lines'].items() if not l['count']])
    save(HERE/'coverage.json',dict(profile=p,execution_identity=execution_identity(exe,p,True),method='Fresh GCC -O0 --coverage process per declared case; gcda removed per case; gcov16 JSON, exceptional compiler branches included. Separate from O2 native reference.',observations=agreements,files=files,summary=summary))
    inv=json.loads((HERE/'inventory.json').read_text());ledger=[];lines=files['readParfile.C']['lines']
    for row in inv['selected']:
        ls={n:x for n,x in lines.items() if row['line']<=int(n)<row['end_line']}
        decisions=[]
        for dec in row['nested']:
            measured=lines.get(str(dec['line']),dict(count=0,cases=[],branches=[]))
            boundaries={1181:'True requires prior invalid DM append; unrun undefined.',1185:'True requires prior invalid CM append; unrun undefined.',1201:'True requires prior invalid DM append; unrun undefined.',1218:'True requires prior invalid CM append; unrun undefined.',1235:'True requires prior invalid solar append; unrun undefined.',410:'False impossible inside the already matched FD prefix branch.',1095:'Lowercase wave alternative shadowed by earlier sine branch.',1104:'Lowercase wave alternative shadowed by earlier sine branch.',1120:'Lowercase wavedm alternative shadowed by earlier sine branch and invalid fixed-offset scan.',1636:'Ordinary lowercase qifunc_p spelling is shadowed by earlier IFUNC and has undefined index scan.',1649:'Ordinary lowercase qifunc_c spelling is shadowed by earlier IFUNC and has undefined index scan.'}
            decisions.append(dict(**dec,measured=measured,source_boundary=boundaries.get(dec['line'],'Remaining safe semantic variants are untested-defined; exceptional compiler edges are not source decisions.'),remaining='Unhit compiler outcomes require source interpretation: exception edges, ordered shadowing and undefined guards are not required executable paths; all other unhit outcomes remain untested-defined. See UNRUN.md.'))
        ledger.append(dict(id=row['id'],condition=row['condition'],lines_hit=sum(x['count']>0 for x in ls.values()),lines_total=len(ls),unhit_lines=[int(n) for n,x in ls.items() if not x['count']],nested=decisions))
    save(HERE/'branch_coverage.json',dict(selected=ledger,case_membership=[c['id'] for c in cs],method='Per-case real gcov counts; lexical source decisions are separate from compiler branch outcomes.'))
    print(json.dumps(summary,indent=2))
if __name__=='__main__':main()
