"""Separately measured O0/gcov reachability and exact differences from O2."""
import gzip
import json
from pathlib import Path
import shutil
import subprocess
from profile import HERE, BUILD, ENV, sha, save, compile_probe
from fixtures import catalog
from inventory import ROUTINES
from run import OUT, REF, unpack, pack, projected, launch

def differences(a,b,path='root'):
    if type(a)!=type(b):return [dict(path=path,expected=a,observed=b)]
    if isinstance(a,dict):
        if list(a)!=list(b):return [dict(path=path,expected_keys=list(a),observed_keys=list(b))]
        return [d for k in a for d in differences(a[k],b[k],path+'.'+k)]
    if isinstance(a,list):
        if len(a)!=len(b):return [dict(path=path,expected_length=len(a),observed_length=len(b))]
        return [d for i,(x,y) in enumerate(zip(a,b)) for d in differences(x,y,path+'['+str(i)+']')]
    return [] if a==b else [dict(path=path,expected=a,observed=b)]

def disposition(file,line,branch):
    if file=='read_fortran.h' and line in [46,49]:
        return 'host-dependent', 'natural big-endian host arm; seeded swap controls do not change host detection'
    if file=='ifteph.C' and line==259:
        return 'inactive', 'cache-hit skip unreachable after successful init: source never updates irec'
    if file=='ifteph.C' and line in [261,263,264]:
        return 'still-defined/unverified', 'live-stream fseek failure not induced with regular files'
    if file=='tt2tdb.C' and line in [182,183,184] and not branch.get('throw'):
        return 'host-dependent', 'debugFlag remains zero; unentered logger path retains accepted M host/ABI limits'
    if branch.get('throw'):
        return 'still-defined/unverified', 'compiler exception edge; dependency/fault path not forced'
    return 'still-defined/unverified', 'unhit compiler edge; inspect retained gcov source attribution; no completeness claim'

def gcov():
    dest=BUILD/'gcov';dest.mkdir(exist_ok=True)
    command=[shutil.which('gcov-16'),'-j','-b','-c',str(BUILD/'coverage-probe.gcno')]
    p=subprocess.run(command,cwd=dest,capture_output=True,text=True,env=ENV)
    assert p.returncode==0,p.stdout+p.stderr
    files=list(dest.glob('*.gcov.json.gz'));assert len(files)==1,files
    data=unpack(files[0]);return data,p.stdout+p.stderr

def main():
    exe,profile=compile_probe('coverage',coverage=True)
    for path in BUILD.glob('coverage-*.gcda'):path.unlink()
    records=[];previous={};diffs=[]
    for i,case in enumerate(catalog()):
        raw=launch(exe,profile,case);path=OUT/'coverage'/(case['id']+'.json.gz');pack(path,raw)
        dif=differences(projected(unpack(REF/(case['id']+'.json.gz'))),projected(raw));diffs.append(dict(case=case['id'],differences=dif))
        data,log=gcov();calls={}
        for f in data['files']:
            name=Path(f['file']).name
            if name not in ROUTINES:continue
            for fn in f['functions']:
                key=name+':'+fn['name'];count=fn['execution_count'];delta=count-previous.get(key,0)
                if delta:calls[key]=delta
                previous[key]=count
        records.append(dict(case=case['id'],path=str(path),sha256=sha(path.read_bytes()),calls=calls,differences=len(dif)))
        if i%10==0:print('coverage',i+1,'/',len(catalog()),case['id'],'differences',len(dif),flush=True)
    selected=[];gaps=[];summary=[]
    for f in data['files']:
        name=Path(f['file']).name
        if name not in ROUTINES:continue
        selected.append(f)
        for routine,a,b in ROUTINES[name]:
            functions=[v for v in f['functions'] if a<=v['start_line']<=b]
            assert len(functions)==1,(name,routine,functions)
            fn=functions[0];assert fn['execution_count']>0,(name,routine)
            lines=[v for v in f['lines'] if a<=v['line_number']<=b]
            branches=[br for v in lines for br in v.get('branches',[])]
            summary.append(dict(file=name,routine=routine,calls=fn['execution_count'],lines=len(lines),hit_lines=sum(v['count']>0 for v in lines),edges=len(branches),hit_edges=sum(v['count']>0 for v in branches)))
            for v in lines:
                if v['count']==0:
                    cls,why=disposition(name,v['line_number'],{})
                    gaps.append(dict(file=name,routine=routine,line=v['line_number'],kind='line',classification=cls,reason=why))
                for j,branch in enumerate(v.get('branches',[])):
                    if branch['count']==0:
                        cls,why=disposition(name,v['line_number'],branch)
                        gaps.append(dict(file=name,routine=routine,line=v['line_number'],kind='edge',index=j,branch=branch,classification=cls,reason=why))
    assert len(summary)==29
    report=dict(profile=profile,gcov_version=subprocess.check_output([shutil.which('gcov-16'),'--version'],text=True),
                cases=records,summary=summary,gaps=gaps,differences=diffs,raw_selected=selected,gcov_log=log)
    save(OUT/'coverage.json',report)
    lines=['# Measured coverage','', 'O0/gcov is separate from the ordinary O2 oracle. See outputs/coverage.json for complete counters, per-case calls, actual executable and every difference.','', '| Routine | Calls | Lines | Edges |','|---|---:|---:|---:|']
    for r in summary:lines.append(f"| {r['routine']} | {r['calls']} | {r['hit_lines']}/{r['lines']} | {r['hit_edges']}/{r['edges']} |")
    lines+=['','## Every unhit line or compiler edge','','| Source:line | Kind | Classification | Reason |','|---|---|---|---|']
    for g in gaps:lines.append(f"| {g['file']}:{g['line']} | {g['kind']} {g.get('index','')} | {g['classification']} | {g['reason']} |")
    lines+=['',f"Cases with O0/O2 differences: {sum(bool(d['differences']) for d in diffs)}. The raw differences are retained, without a new tolerance.", '', 'Native-invalid paths remain unrun as enumerated in SOURCE_MAP.md; coverage of an error branch does not validate malformed memory or invalid arithmetic inputs. Literal-disabled source remains inspection only. The negative-step cases reach the otherwise-unhit cache-skip arm with valid initialized backing.']
    (HERE/'COVERAGE.md').write_text('\n'.join(lines)+'\n')
    print('PASS coverage',len(summary),'routines;',len(gaps),'classified gaps;',sum(bool(d['differences']) for d in diffs),'differing cases')
if __name__=='__main__':main()
