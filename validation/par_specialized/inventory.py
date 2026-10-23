"""Source-derived released membership, nested paths, declarations and caller boundaries."""
import json,re
from functools import lru_cache
from profile import ROOT,HERE,blob,sha,save,project_blob,RELEASE

def clean(s):
    return re.sub(r'"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\'|//[^\n]*|/\*[\s\S]*?\*/',lambda m:re.sub(r'[^\n]',' ',m[0]) if m[0].startswith('/') else m[0],s)

@lru_cache(None)
def released():
    manifest=json.loads(project_blob('decisions/evidence/par-specialized/released-branches.json',RELEASE))
    for n,h in manifest['accepted_k_artifacts_sha256'].items():assert sha(project_blob(n,manifest['accepted_k_commit']))==h,n
    k=json.loads(project_blob('validation/parameters/inventory.json',manifest['accepted_k_commit']))
    ids={b['id'] for b in manifest['deferred_top_level_branches']}|{'D029'}
    return manifest,k,[b for b in k['dispatch'] if b['id'] in ids]

@lru_cache(None)
def projection():
    _,k,branches=released();lines=clean(blob('readParfile.C').decode()).splitlines()
    text='\n'.join('\n'.join(lines[b['line']-1:b['end_line']-1]) for b in branches)
    fields=set(re.findall(r'psr->(\w+)',text))
    init=clean(blob('initialise.C').decode()).split('void initialiseOne')[1].split('void allocateMemory')[0]
    fields.update(re.findall(r'psr->(\w+)\s*=',init));fields.update(re.findall(r'strcpy\(psr->(\w+)\s*,',init))
    fields.update('clock ephemeris JPL_EPHEMERIS rajStrPre rajStrPost decjStrPre decjStrPost phaseJump phaseJumpDir phaseJumpID jumpSAT jumpScaled jumpValErr fdjumpValErr'.split())
    header=clean(blob('tempo2.h').decode()).split('typedef struct pulsar {')[1].split('} pulsar;')[0]
    # Retained uncertainty arrays and companion cells are part of the projection.
    fields.update(re.findall(r'\b((?:wave_|quad_|telD|gwsrc_|TNShapelet)[A-Za-z0-9_]*)\s*(?=\[|[,;])',header))
    fields-=set('param obsn covar ToAextraCovar constraint_special'.split())
    return sorted(fields)

def inventory():
    manifest,k,branches=released();raw=blob('readParfile.C').decode();lines=raw.splitlines();header=clean(blob('tempo2.h').decode())
    decls={}
    for dec in header.split('typedef struct pulsar {')[1].split('} pulsar;')[0].split(';'):
        for f in projection():
            if re.search(r'\b'+f+r'\b',dec):decls[f]=' '.join(dec.split())+';'
    assert set(decls)==set(projection()),set(projection())-set(decls)
    rows=[]
    for b in branches:
        body='\n'.join(lines[b['line']-1:b['end_line']-1])
        nested=[d for d in k['nested_decisions'] if b['line']<=d['line']<b['end_line']]
        rows.append(dict(**b,source=body,raw_segment_sha256=sha(body.encode()),nested=nested,fields=sorted(set(re.findall(r'psr->(\w+)',clean(body)))),remaining='All unmeasured outcomes remain untested-defined unless an explicit undefined rule applies; measured line/branch outcomes are reconciled in coverage ledger.'))
    helpers=blob('t2fit_nestlike.h').decode().split('double t2FitFunc')[0]
    return dict(release=manifest,k_top_level=len(k['dispatch']),k_nested=len(k['nested_decisions']),selected=rows,
        helpers=helpers,helper_names=re.findall(r'static inline \w+ (\w+)\(',helpers),
        projected_fields=decls,initialization=k['initializer_assignments'],
        capacities=[l.strip() for l in blob('tempo2.h').decode().splitlines() if re.match(r'#define MAX_',l)],
        caller_map=[dict(name=n,source='readParfile.C',lines=r,contract=c) for n,r,c in [
        ('checkLine','173-2384','Explicit initialized pulsar, scratch parameters; one real stream held across ordered token calls.'),
        ('readSimpleParfile','72-139','Real FILE; initialized pulsar and explicit epochs/prefit; no count reset; checkAllSet; native scratch leak retained.'),
        ('readParfile','2534-2767','Real files for both pulsars; resets ordinary jumps/phase jumps/links, retains FD and append counters; defaults and finalization.'),
        ('readParfileGlobal','141-171','tpar[0] for every pulsar; no full reset/finalization; no ecliptic scratch-dependent record permitted.')]],
        undefined_rules=UNRUN,
        immutable_membership='All tracked files under validation/parameters, clocks, numeric_toolkit, tim_io, precision, binary, jpl, coverage; both nested Git repositories. P preprocessing and O parameter_storage_port excluded.')

UNRUN=[
 dict(family='jump/FD jump',source='readParfile.C:288-453',rule='Empty/missing remainder or selector; MJD/FREQ exactly four tokens; NAME/TEL/flag and ordinary SCALED exactly three: uninitialized local strings. One-based count >= MAX_JUMPS-1 before append: invalid destination.'),
 dict(family='FD context',source='readParfile.C:350-420',rule='FD numeric <=0 or fval-1 outside param_fd.aSize despite param_f guard; integer overflow; missing scale token. Index-zero FD-jump is defined nonconsumption.'),
 dict(family='sampled',source='readParfile.C:1088-1149,1563-1680',rule='Failed numeric index scans; indices outside actual capacity except explicitly prechecked upper ORBIFUNC/IFUNC and lower DY/VY/DZ exits. Ordinary lowercase qifunc_p1 and wave1 have undefined index parsing; only crafted spellings with valid fixed-offset indices execute.'),
 dict(family='DM/CM/SW',source='readParfile.C:1159-1245',rule='Missing mjd or value reads uninitialized local; missing discarded err in DMOFF/_DM is not read and may be defined. Uninitialized natural solar counter and counts outside 0..MAX_IFUNC-1 unrun; overflow check after write is unreachable safely.'),
 dict(family='constraints',source='readParfile.C:1257-1429',rule='Missing cname, failed special fgets, append beyond constraints/MAX_PARAMS, allocation failure and reading past initialized IFUNC_COV prefix unrun. No strlen of returned special pointer.'),
 dict(family='noise',source='readParfile.C:1693-2020',rule='Failed local subtraction-order/packed-string scans, PHASE missing local string and all append destinations outside their capacities unrun. Shapelet scans target initialized destinations and are safe within capacity.'),
 dict(family='strings',source='readParfile.C selected fscanf/sscanf/strcpy',rule='All unbounded %s input bounded below actual local and destination capacity; no oversized strings, overflowing integer conversion, failed allocation/I/O.'),
 dict(family='global reader',source='readParfile.C:147-166',rule='Ecliptic records touch uninitialized parameter scratch; unrun.'),
 dict(family='diagnostic ABI',source='TKlog.C; decision 0030',rule='Logger color-removal pointer arithmetic retains accepted host-observation qualification; no claim of portable diagnostic execution. Native routines unmodified.')]
if __name__=='__main__':
    x=inventory();save(HERE/'inventory.json',x);print(len(x['selected']),len(x['projected_fields']),len(x['helper_names']))
