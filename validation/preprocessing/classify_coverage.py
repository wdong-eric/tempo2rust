#!/usr/bin/env python3
"""Attach explicit dispositions to every unhit measured edge, without altering counts."""
import gzip,hashlib,json,subprocess,sys
from pathlib import Path
HERE=Path(__file__).resolve().parent

def sha(b):return hashlib.sha256(b).hexdigest()
# GCC emits cleanup/exception successors at these strcasecmp calls even when
# gcov's JSON throw flag is false. The preserved -fdump-tree-cfg-lineno output
# shows call blocks entering resx cleanup blocks, separately from comparison edges.
CALL_EDGES={
 'preProcess.C':{200:[1],259:[1],266:[1],401:[1],402:[1],435:[1],436:[1],722:[1],750:[1],752:[1],759:[1],761:[1],766:[1],818:[1],821:[1],823:[1],826:[1],829:[1],832:[1],835:[1],862:[5],950:[1],957:[1],959:[1]},
 'preProcessSimple.C':{271:[1,5],276:[1],281:[1],288:[1],290:[1],296:[1],299:[1],336:[1,5],341:[1],346:[1],353:[1],355:[1],361:[1]}}

def disposition(file,line,i,b):
    if file=='preProcess.C' and (line in [490,506,507,508] or 528<=line<=531):
        return 'source-inspected/unrun', 'Plugin search exhaustion or missing-entry diagnostic: zero-path module and post-consumption/post-dlclose dlerror pointer are not portable initialized %s inputs. See inventory unrun boundaries.'
    if b['throw'] or i in CALL_EDGES.get(file,{}).get(line,[]):
        return 'compiler exception control/unrun', 'No C++ exception or forced resource failure injected. For untagged strcasecmp cleanup successors, see the preserved source-line CFG and its resx blocks.'
    if (file,line,i)==('preProcessSimple.C',503,1):
        return 'structurally unreachable', 'Assignment of -1 is always true; changing it to comparison is a tested mutant, not a source path.'
    if (file,line,i)==('preProcess.C',756,1):
        return 'structurally unreachable', 'found is assigned only 0 or 1; this else follows found==0.'
    if file=='preProcessSimple.C' and (line,i) in [(281,3),(346,3)]:
        return 'structurally unreachable', 'The enclosing test established MJD or FREQ and the preceding MJD branch failed; stable local string must be FREQ.'
    if (file,line,i)==('preProcess.C',499,1):
        return 'structurally unreachable', 'Successful throwing new[] returns a non-null str; allocation failure does not reach delete[].'
    if file=='preProcess.C' and line==290:
        return 'defined prefix not exercised; unsafe continuation withheld', 'No-DM branch can be reached before an earlier defined exit, but continuation to stage2 reads uninitialized ndm. Cases use declared -setdm when reaching stage2.'
    if file=='preProcess.C' and line==171:
        return 'defined but unexercised', 'This warning call site did not overflow the 30 stored diagnostic slots. M covers logger storage separately; it is not P execution coverage.'
    if file=='preProcess.C' and line==886:
        return 'defined but unexercised', 'The SIMUL only forward search did not encounter an already-deleted j row after matching both its flag ID and second value. Other deletion guards and loop-order mutations were exercised; this exact guard outcome remains a gap.'
    return 'defined but unexercised', 'This exact measured compiler edge was not hit by the bounded corpus; its source line and edge index remain an explicit coverage gap. No unreachable or undefined claim is inferred.'

def main():
    c=json.loads((HERE/'coverage.json').read_text());inv=json.loads((HERE/'inventory.json').read_text());p=c['profile'];src=Path(next(a[2:] for a in p['command'] if a.startswith('-I')))
    missing=[];cfg={}
    for file,entry in c['files'].items():
        source=(src/file).read_text().splitlines()
        target=HERE/'build'/('classification-'+Path(file).stem+'.o')
        cmd=[p['command'][0],'-O0','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(src),'-fdump-tree-cfg-lineno','-c',str(src/file),'-o',str(target)]
        subprocess.run(cmd,capture_output=True,text=True,check=True)
        dumps=list((HERE/'build').glob(target.stem+'.C.*cfg'));assert len(dumps)==1
        data=dumps[0].read_bytes();cfg[file]=dict(command=cmd,sha256=sha(data),text=data.decode())
        for n,line in entry['lines'].items():
            for i,b in enumerate(line['branches']):
                if b['count']:continue
                kind,reason=disposition(file,int(n),i,b)
                missing.append(dict(file=file,line=int(n),branch=i,classification=kind,reason=reason,source=source[int(n)-1].strip(),gcov_throw=b['throw'],gcov_fallthrough=b['fallthrough']))
    missing.sort(key=lambda x:(x['file'],x['line'],x['branch']))
    counts={kind:sum(x['classification']==kind for x in missing) for kind in sorted({x['classification'] for x in missing})}
    source_conditions=[]
    for s in inv['conditions']:
        d=dict(s);d['measured_line']=c['files'][s['file']]['lines'].get(str(s['line']));source_conditions.append(d)
    out=dict(coverage_sha256=sha((HERE/'coverage.json').read_bytes()),script_sha256=sha(Path(__file__).read_bytes()),python=sys.version,unhit_edges=missing,classification_counts=counts,source_conditions=source_conditions)
    (HERE/'coverage_classification.json').write_text(json.dumps(out,indent=2)+'\n')
    (HERE/'coverage_cfg.json.gz').write_bytes(gzip.compress(json.dumps(cfg,separators=(',',':')).encode(),mtime=0))
    md='# Measured coverage and limits\n\nSeparate native O0/gcov execution, with O2 caller/exporter. Counts include compiler exception/cleanup edges. The numerical reference uses O2 throughout. See coverage.json for the actual build/runner/runtime identity and per-case counters.\n\n| Unit | Functions | Lines | Compiler edges |\n|---|---:|---:|---:|\n'
    for f,s in c['summary'].items():md+=f"| {f} | {s['functions_hit']}/{s['functions_total']} | {s['lines_hit']}/{s['lines_total']} | {s['branches_hit']}/{s['branches_total']} |\n"
    diffs=[o['case'] for o in c['observations'] if o['comparison']!='exact']
    md+='\nInstrumented comparisons: '+str(len(c['observations'])-len(diffs))+' exact, '+str(len(diffs))+' different. Any differences are retained in full in coverage.json.\n\n'
    md+='Every unhit edge is recorded individually in coverage_classification.json, with source text, location, index and disposition. coverage_cfg.json.gz preserves fresh compiler CFG dumps supporting the untagged strcasecmp cleanup edges; these are distinct from source comparison outcomes. Source conditional-to-line attachment is not an assertion that one source condition equals one compiler edge.\n\n'
    for kind,count in counts.items():md+=f'- {kind}: {count} edges.\n'
    md+='\nThe seven unhit primary executable lines are plugin failure diagnostics withheld under the documented dlerror/null boundary. The source-literal-disabled binary epoch block has no executable coverage requirement and remains in the static inventory.\n\nDefined but unexercised edges remain unfinished coverage; accepted M logger evidence does not turn them into executed P branches. These bounded witnesses are not exhaustive input/combinatorial coverage, another-platform evidence, sanitizer results or scientific accuracy validation.\n'
    (HERE/'COVERAGE.md').write_text(md)
    print(json.dumps(counts,indent=2))
if __name__=='__main__':main()
