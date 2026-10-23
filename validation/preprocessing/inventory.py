"""Complete raw primary routine/condition inventory; no native execution."""
import re
from profile import blob, PRIMARY, HERE, save, sha


def uncomment(s):
    return re.sub(r'"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\'|//[^\n]*|/\*[\s\S]*?\*/',
        lambda m: re.sub(r'[^\n]', ' ', m[0]) if m[0].startswith('/') else m[0],s)


def enum_names():
    return re.findall(r'\bparam_\w+',re.search(r'enum label\s*\{(.*?)\}',uncomment(blob('tempo2.h').decode()),re.S)[1])


def projected_fields():
    text='\n'.join(uncomment(blob(n).decode()) for n in PRIMARY)
    names=set(re.findall(r'\bpsr(?:\[[^\]]+\])?(?:->|\.)(\w+)',text))
    names -= {'obsn','param'}
    # Initialized retained partners of all selected arrays and fields; source global PAR inputs.
    names.update('nWhite_dm jumpVal jumpValErr jumpScaled jumpSAT fdjumpVal fdjumpValErr fdjumpIdx fdjump_log fdjump_pint_format phaseJumpDir sorted binaryModel nCompanion nTNECORR clockFromOverride ephemeris JPL_EPHEMERIS rajStrPre rajStrPost decjStrPre decjStrPost'.split())
    return sorted(names)

ROLES={
'preProcess':'ordered argv + enum-order links + name/jumps + white file + DM-series + nofit/fit + simulate/set + stage1 + epoch + forbidden fits + DM file + T2/TN/TOBS + modify + stage2 + plugin/select + stage3, then global PAR',
'useSelectFile':'nonempty bounded file, feof repeats last stored line unless tokenized; exact PASS/REJECT/PROCESS, insensitive LOGIC; mutations across all pulsars',
'logicFlag':'five assigned tokens; EXIST can use four but fixtures supply five; numeric threshold must parse when comparison consumes it; first matching undeleted flag',
'processFlag':'six assigned tokens; independent sequential mode tests on undeleted matching flag rows',
'processSimultaneous':'six/seven valid tokens or early return; strict threshold; nested i/j mutation; carried found flags in only mode',
'readWhiteNoiseModelFile':'bounded complete raw tokens; JITTER then START_TIME_JITTER/TOAERR_ZERO in file order; all rows including deleted',
'preProcessSimple':'one initialized pulsar; stage1(0,-1), stage2(0,0,null,0), stage3; no wrapper ndm read',
'preProcessSimple1':'nWhite used by both scans; sentinel -1; EPHVER overrides tempo1; final stored observation for last; units100 active',
'preProcessSimple2':'weighted zero-error exit before row writes; vector/phase/offset/filter/pass/jumps/FD/-to/DM order; append counts',
'preProcessSimple3':'FJUMP discovery undeleted/id-aware then value-only application including deleted; ordinary then FD; phase ID assignment -1 and double search from index1',
'scale_param':'valid index; val/err/prefit/prefitErr in order regardless of flags',
'xform_mjd':'valid index; binary64 IFTE_MJD0 promoted; prefit[0] input even for arr>0',
'transform_units':'from/to integer tests; sequential repeated factors, DM/CM loops, BPJ index0 before later elements; exact @ TZR gate; units retained',
'updateEpoch_str':'psr[0] selection controls, psr[p] rows; SAT/BAT boundary flags1/2; sentinel -1 and bounded int narrowing; numeric-prefix parser',
'updateEpoch':'dt to double; F0/F1/F2 and conditional F3 sequence, selected prefit, RA then DEC then RA2 then DEC2, DM0; FITWAVES diagnostics; binary block inactive'}

UNRUN=[
 dict(id='wrapper_no_setdm',source='preProcess.C:48,480',reason='ndm uninitialized argument value; simple2 short circuit does not define caller read'),
 dict(id='missing_empty_dm',source='preProcess.C:294-318',reason='unchecked fopen/feof; empty file reads unwritten line; >10000 rows unchecked'),
 dict(id='tnsecorr_no_tobs',source='preProcess.C:420-443',reason='tobsval unwritten if no matching flag; invalid atof becomes defined zero but is not metadata default'),
 dict(id='empty_select',source='preProcess.C:586-590',reason='first failed fgets leaves line unwritten'),
 dict(id='invalid_scans',source='preProcess.C;preProcessSimple.C',reason='missing argv/value/select/noise tokens, malformed numeric comparison, dangling tOffset flag or pass value before ID can consume unwritten/null data'),
 dict(id='invalid_capacity',source='preProcessSimple.C:275-403,427-493',reason='unchecked jump/FD append, invalid counts, flag strings >15 for FJUMP, unsafe output capacity; bounded last-valid capacity exits allowed'),
 dict(id='empty_last',source='preProcessSimple.C:123',reason='nobs=0 and last>0 indexes -1'),
 dict(id='invalid_int_conversion',source='preProcessSimple.C:385-394;updateEpoch.C:58-62;preProcess.C:254',reason='nonfinite/out-of-int-range DM index, epoch selection or NITS conversion'),
 dict(id='plugin_zero_paths',source='preProcess.C:485-510',reason='module uninitialized when no paths'),
 dict(id='plugin_missing_entry',source='preProcess.C:526-531',reason='dlsym failure followed by dlclose then dlerror; diagnostic pointer lifetime/null behavior host-dependent, withheld'),
 dict(id='plugin_all_missing',source='preProcess.C:497-513',reason='dlerror cleared in loop then printf %s with null; no portable result, withheld'),
 dict(id='global_ecliptic_or_guard',source='readParfile.C:137,151',reason='global reader elong/elat scratch uninitialized; fixtures use only fully assigned scalar/common records'),
 dict(id='allocation_io_exhaustion',source='dependencies',reason='not forced; separate environmental/resource failure coverage'),
]


def inventory():
    out={'pin':'fda6ad4a9753ad7dc5d291406f2f310a9161423f','routines':[], 'conditions':[], 'preprocessor':[], 'unrun':UNRUN,'projected_pulsar_fields':projected_fields(),'parameter_enum':enum_names()}
    for fn in PRIMARY:
        raw=blob(fn);s=raw.decode();clean=uncomment(s)
        funcs=list(re.finditer(r'\bvoid\s+(\w+)\s*\([^;]*?\)\s*\{',clean,re.S))
        for m in funcs:
            start=m.end()-1;depth=1;end=start+1
            # Strings cannot introduce scope delimiters here; mask them for brace counting.
            masked=re.sub(r'"(?:\\.|[^"\\])*"',lambda x:' '*len(x[0]),clean)
            while depth:
                depth+=(masked[end]=='{')-(masked[end]=='}');end+=1
            body=clean[m.start():end];name=m[1];lo=s.count('\n',0,m.start())+1;hi=s.count('\n',0,end)+1
            out['routines'].append(dict(file=fn,name=name,start=lo,end=hi,raw_sha256=sha(raw),contract=ROLES[name],fields=sorted(set(re.findall(r'(?:->|\.)(\w+)',body))),parameter_refs=sorted(set(re.findall(r'\bparam_\w+',body))),calls=sorted(set(re.findall(r'\b(\w+)\s*\(',body)) - {'if','for','while','sizeof',name})))
            for cond in re.finditer(r'\b(if|for|while|switch)\s*\(',body):
                a=cond.end();b=a;d=1
                while d:
                    d+=(body[b]=='(')-(body[b]==')');b+=1
                line=lo+body.count('\n',0,cond.start());inactive=fn=='updateEpoch.C' and line>=161
                out['conditions'].append(dict(file=fn,routine=name,line=line,kind=cond[1],expression=body[a:b-1].strip(),classification='literal-disabled binary block' if inactive else 'active; bounded fixtures or explicit unrun boundary; measured dispositions in coverage.json'))
        out['preprocessor'] += [dict(file=fn,line=i,text=l) for i,l in enumerate(s.splitlines(),1) if l.lstrip().startswith('#')]
    assert len(out['routines'])==15
    return out

if __name__=='__main__':save(HERE/'inventory.json',inventory())
