#!/usr/bin/env python3
"""Readable registry, complete branch dispositions and coverage cross references."""
import json
from profile import HERE,ROOT,save,sha,blob
from inventory import inventory


def main():
    inv=inventory();save(HERE/'inventory.json',inv)
    results=json.loads((HERE/'observations.json').read_text())
    natural=next(r for r in results if r['id']=='registry')['events'][3]['result']
    names={r['index']:r['name'] for r in inv['parameter_enum']}
    registry=[]
    for i in range(2000):
        name=names.get(i,'unnamed_'+str(i));prefix=name+'.';size=natural[prefix+'aSize']
        cells=[]
        for j in range(size):
            p=prefix+str(j)+'.'
            cells.append(dict(index=j,**{k:natural[p+k] for k in ['val','err','fitFlag','paramSet','label','shortlabel']}))
        registry.append(dict(enum_index=i,name=name,aSize=size,cells=cells,
                             nLinkTo=natural[prefix+'nLinkTo'],nLinkFrom=natural[prefix+'nLinkFrom'],
                             unwritten=['prefit','prefitErr','linkTo entries beyond nLinkTo','linkFrom entries beyond nLinkFrom']))
    save(HERE/'registry.json',registry)
    lines=['# Parameter registry','', 'Raw canonical initialization: 166 enum entries (162 physical parameters, `param_LAST`, three virtual values), 2,000 allocated slots, 9,616 parameter cells. `registry.json` records every allocated slot/index and label, including empty labels. `observations.json` retains the independent allocate-only snapshots for both unused argument values.','',
           '`longdouble` is GCC `__float128` (113 significand bits); `val`, `err`, `prefit`, `prefitErr` are arrays of that type. `aSize`, fit/set flags and link indices/counts are `int`; labels are allocated character strings. `prefit` and `prefitErr` are **unwritten** by canonical initialization. Inactive link entries and the observations/covariance payload are not read as goldens.','',
           '| Enum index | Name | Array size | First label | Last label |','| --- | --- | --- | --- | --- |']
    for r in registry[:166]:lines.append(f"| {r['enum_index']} | `{r['name']}` | {r['aSize']} | {r['cells'][0]['label']} | {r['cells'][-1]['label']} |")
    lines+=['','Slots 166–1999 each have size one and empty labels; see the complete JSON. Natural `val`, `err`, `fitFlag`, `paramSet`, and link counts are zero in this build. Both `param_daop` and `param_iperharm` are labeled `IPERHARM`: the former source label was overwritten. This is preserved.','',
            'The initializer leaves `ne_sw_ifuncN` **and `nchromx`** unwritten. Explicit caller markers 117 and 119 survive initialization. They are controlled caller values, not defaults. The later controlled parser experiments use `ne_sw_ifuncN=117`, `nchromx=0`; neither is inferred from malloc.']
    (HERE/'REGISTRY.md').write_text('\n'.join(lines)+'\n')
    linecounts={};functions=[]
    if (HERE/'line_coverage.json').exists():
        coverage=json.loads((HERE/'line_coverage.json').read_text())
        for unit in coverage['units'].values():
            for f in unit['data']['files']:
                if f['file'].endswith('/readParfile.C'):linecounts={l['line_number']:l for l in f['lines']}
                if any(f['file'].endswith('/'+n) for n in ['readParfile.C','initialise.C']):functions+=f['functions']
    tokens={}
    for r in results:
        for e in r['events']:
            if e['phase']=='end' and e['op']=='stream':
                for token in e['result']:tokens.setdefault(token['keyword'],[]).append(r['id'])
    rows=[]
    for b in inv['dispatch']:
        row=dict(b)
        row['line_counts']={str(n):linecounts[n]['count'] for n in range(b['body_line']+1,b['end_line']) if n in linecounts}
        row['candidate_cases_by_spelling']=sorted({c for key,cs in tokens.items() if any(key.lower()==k.lower() for k in b['keywords']) for c in cs})
        row['remaining']='Not exhaustive: malformed/overflow indices, incomplete scans and specialized record semantics require separate disposition.'
        if b['readValue_calls']:
            row['disposition']='Bounded scalar/index path; exact observations and source line counts. This does not close every condition outcome.'
        elif b['id'] in ['D000','D001','D002','D003','D005','D006','D007','D008','D010','D011','D012','D013','D014','D015','D016','D017','D018','D019','D022','D023','D025','D030','D031','D033','D087','D088','D254','D334','D335','D336','D337','D338']:
            row['disposition']='Bounded common control/comment/unknown path; ordered token consumption observed.'
        else:
            row['disposition']='Deferred specialized branch; inventory only.'
            if 285<=b['line']<=461:row['remaining']='Jump selectors, FDJUMP formats, partial scans, counts, capacity and fit flags.'
            elif b['line']<1431:row['remaining']='Clock-chain integration (J), sampled wave/DM/SW/quad tables, constraints, units, ordering, capacity and incomplete scans.'
            elif b['line']<1677:row['remaining']='Compound GW/orbital/telescope/interpolation records, all parsed fields and downstream model meaning.'
            else:row['remaining']='Noise selectors/events, sample counts, packed settings, unit conversions and incomplete scans.'
        if b['id']=='D029':row['remaining']='FD1/FD9 and ignored FD13 tested. FD10–12 violate fd capacity despite the f-array guard; FDJUMP nested branches deferred.'
        rows.append(row)
    save(HERE/'coverage.json',dict(branches=rows,functions=inv['functions'],gcov_functions=functions,
         cases=[dict(id=r['id'],actions=len(r['actions']),events=len(r['events']),returncode=r['returncode']) for r in results],
         limits='candidate_cases_by_spelling is a navigation aid, not proof of branch selection. line_counts are aggregate GCC counts; condition false/true edges remain in line_coverage.json. Scalar snapshots prove selected parameter/index changes. No full branch/path closure claimed.'))
    table=['# Ordered dispatch and remaining parser work','',f"All {len(rows)} top-level branches of `readParfile.C::checkLine`, in source order. `inventory.json` also retains every nested if condition, scan and source expression. These are dispositions, not a claim of complete parser validation. Full actual GCC counts are in `line_coverage.json`; the case/action matrix is `cases.json`.",'',
           '| ID | Source line | Test / keyword spelling | Disposition and remaining work |','| --- | --- | --- | --- |']
    for b in rows:
        label=', '.join(b['keywords']) or b['condition']
        table.append(f"| {b['id']} | {b['line']} | `{label.replace('|','&#124;')}` | {b['disposition']} {b['remaining']} |")
    table+=['','Keyword matching details and deterministic shadowing are documented in `FINDINGS.md`. Specialized branches remain open even if a scalar switch into that feature was tested. TIM, preprocessing, sorting, fitting and timing evaluation are entirely outside this assignment.']
    (HERE/'DISPATCH.md').write_text('\n'.join(table)+'\n')
    gcases=json.loads((ROOT/'validation/binary/cases.json').read_text());chosen={}
    for c in gcases:
        if c['model'] not in chosen:
            path=ROOT/'validation/binary/fixtures'/c['par'];chosen[c['model']]=dict(path=str(path.relative_to(ROOT)),sha256=sha(path.read_bytes()))
    save(HERE/'input_hashes.json',dict(binary_fixtures=chosen,
         support_evidence={str(p.relative_to(ROOT)):sha(p.read_bytes()) for p in [ROOT/'validation/coverage/REPORT.md',ROOT/'validation/coverage/BEHAVIOR_DIFFERENCES.md',ROOT/'validation/coverage/inventory.json',ROOT/'decisions/0020-parameter-par-reference-stage.md',ROOT/'decisions/0008-preservation-policy-and-stage-plan.md',ROOT/'handoffs/11-parameter-par-reference.md']},
         runtime_resources='None opened by bounded cases. TEMPO2 path is explicit synthetic build/runtime; EPHEM constructs strings only. Real clock dependencies linked; chain graph/cache coverage remains J.'))
    print(len(registry),'slots;',sum(r['aSize'] for r in registry),'cells;',len(rows),'dispatch dispositions')


if __name__=='__main__':main()
