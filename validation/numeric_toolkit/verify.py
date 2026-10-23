#!/usr/bin/env python3
"""Fresh ordinary replay plus exact-comparator, identity, freshness and source mutations."""
import copy
import json
import math
import re
import subprocess
from profile import HERE,BUILD,ENV,compile_probe,save,sha
from run import compare,identity_gate,execution_identity,execute,validate,regenerate
from inventory import inventory


def rejected(call):
    try:call()
    except (AssertionError,FileNotFoundError) as error:return str(error)
    raise AssertionError('Control unexpectedly accepted')


def semantic(row):
    r=copy.deepcopy(row)
    for e in r['events']:e.pop('stdout_position',None)
    for stream in ['stdout','stderr']:
        r[stream]=re.sub(r'\[\{source\}/[^\]:]+:\d+\]', '[source-location]', r[stream])
    return r


def first_result(row):return next(e['result'] for e in row['events'] if e['phase']=='end')


def controls(cs,rows,exe,profile):
    bycase={c['id']:c for c in cs};byrow={r['id']:r for r in rows};checks=[]
    def artifact(name,case,change):
        expected=byrow[case];actual=copy.deepcopy(expected);change(actual)
        checks.append(dict(kind='artifact',name=name,rejection=rejected(lambda:compare(expected,actual))))
    checks.append(dict(kind='positive',name='complete baseline exact replay',cases=len(rows),status='PASS'))
    for name,mutation in [
        ('missing_case',lambda x:x.pop()),('extra_case',lambda x:x.append(x[-1])),
        ('case_order',lambda x:x.reverse())]:
        x=copy.deepcopy(rows);mutation(x);checks.append(dict(kind='artifact',name=name,rejection=rejected(lambda:compare(rows,x))))
    artifact('missing_event','vector_0',lambda r:r['events'].pop())
    artifact('extra_event','vector_0',lambda r:r['events'].append(r['events'][-1]))
    artifact('event_order','vector_0',lambda r:r['events'].reverse())
    artifact('missing_result_field','vector_0',lambda r:first_result(r).pop('dot'))
    artifact('extra_result_field','vector_0',lambda r:first_result(r).update(extra=0))
    artifact('component_length','vector_0',lambda r:first_result(r)['before'].pop())
    artifact('numeric_bits','vector_0',lambda r:first_result(r)['before'][0].update(bytes_le='00'*8))
    artifact('nonfinite_class','parse_28',lambda r:first_result(r)['value'].update(**{'class':'finite'}))
    artifact('format_return','format_sprintf_4',lambda r:first_result(r).update(**{'return':99}))
    artifact('format_buffer','format_sprintf_4',lambda r:first_result(r).update(buffer_bytes='00'))
    artifact('copy_retained_state','copypsr_0',lambda r:first_result(r)['fields'][0].update(retained=True))
    artifact('copy_extent','copypsr_0',lambda r:first_result(r)['observations'].pop())
    artifact('parameter_link','copyparam_3',lambda r:first_result(r)['destination_after']['linkTo'].__setitem__(0,99))
    artifact('warning_count','diagnostic_capacity',lambda r:r['events'][1]['result'].update(warnings=999))
    artifact('diagnostic_text','diagnostic_capacity',lambda r:r.update(stdout=r['stdout'].replace('DUP1','DUP2')))
    artifact('exit_status','link_repeat_exit',lambda r:r.update(returncode=0))
    artifact('exit_stage','link_repeat_exit',lambda r:r['events'].clear())
    artifact('action_input','parse_00',lambda r:r['actions'][0].__setitem__(1,'different'))
    for key in ['compiler_sha256','sources','runtime','runner_sources','generated','macros_sha256','compiler_backend','header_dependencies','preprocessed_normalized_sha256']:
        wrong=copy.deepcopy(profile)
        if isinstance(wrong['stable'][key],dict):
            sub=next(iter(wrong['stable'][key]));wrong['stable'][key][sub]='deliberately wrong identity'
        else:wrong['stable'][key]='deliberately wrong identity'
        checks.append(dict(kind='identity',name=key,rejection=rejected(lambda:identity_gate(profile,wrong,exe))))
    wrong=copy.deepcopy(profile);wrong['stable']['runner_sources']['probe.cc']='wrong harness'
    checks.append(dict(kind='identity',name='harness',rejection=rejected(lambda:identity_gate(profile,wrong,exe))))
    # Same path forwarding wrapper cannot inherit the fresh executable's digest.
    original=exe.read_bytes()
    try:
        exe.write_text('#!/bin/sh\nexit 0\n');exe.chmod(0o755)
        checks.append(dict(kind='identity',name='same_path_replaced_executable',rejection=rejected(lambda:identity_gate(profile,profile,exe))))
    finally:exe.write_bytes(original);exe.chmod(0o755)
    execution_identity(exe,profile)
    # Truthfully bind control executable bytes; test the separate fresh-output gate.
    for name,body in [('no_output','return 0;'),('stale_output','FILE*f=fopen(argv[2],"w");fputs("{\\\"phase\\\":\\\"launch\\\",\\\"nonce\\\":\\\"stale\\\"}\\n",f);fclose(f);return 0;')]:
        src=BUILD/(name+'.cc');target=BUILD/name;src.write_text('#include <stdio.h>\nint main(int argc,char**argv){'+body+'}\n')
        subprocess.run([profile['command'][0],str(src),'-o',str(target)],check=True,capture_output=True,env=ENV)
        candidate=copy.deepcopy(profile);candidate['executable_path']=str(target);candidate['executable_sha256']=sha(target.read_bytes())
        # Existing good output at the same per-case path must be removed before launch.
        result=execute(bycase['parse_00'],exe,profile);validate(bycase['parse_00'],result)
        checks.append(dict(kind='freshness',name=name,executable_sha256=candidate['executable_sha256'],rejection=rejected(lambda:execute(bycase['parse_00'],target,candidate))))
    return checks


MUTATIONS=[
 ('parse_precision','TKlongdouble.C','ld = strtoflt128(str,NULL);','ld = (__float128)strtod(str,NULL);','parse_41'),
 ('display_precision','TKlongdouble.C','"%Qg", ld','"%.8Qg", ld','parse_50'),
 ('format_return','TKlongdouble.C','int ret_status = 0;','int ret_status = 7;','format_sprintf_4'),
 ('format_value','TKlongdouble.C','quadmath_snprintf(o,BUFSIZE-(o-buf),qfmt, ld);','quadmath_snprintf(o,BUFSIZE-(o-buf),qfmt, ld + 1);','format_sprintf_3'),
 ('hms_conversion','tempo2Util.C','turn = hr/24.;','turn = hr/12.;','coord_hms_05'),
 ('dms_wrap','tempo2Util.C','if(turn >= 1.0)turn = turn - 1.0;','if(turn >= 1.0)turn = turn - 2.0;','coord_dms_14'),
 ('hms_carry','tempo2Util.C','(sec*10000. +0.5)/10000','(sec*10000000. +0.5)/10000000','turn_hms_18'),
 ('dms_carry','tempo2Util.C','(sec*1000. +0.5)/1000','(sec*100000. +0.5)/100000','turn_dms_18'),
 ('mod_quotient','tempo2Util.C','ret = a - (int)(a/p)*p;\n    return ret;\n}\ndouble','ret = a - floorq(a/p)*p;\n    return ret;\n}\ndouble','mod128_01'),
 ('nint_tie','tempo2Util.C','i=(int)(x+0.5);','i=(int)(x+0.25);','nint_05'),
 ('nlong_tie','tempo2Util.C','i=(long)(x-0.5);','i=(long)(x-0.25);','nlong_03'),
 ('dot_order','tempo2Util.C','v1[0]*v2[0]+v1[1]*v2[1]+v1[2]*v2[2]','v1[0]*v2[0]+(v1[1]*v2[1]+v1[2]*v2[2])','vector_1'),
 ('sum_order','tempo2Util.C','res[0] = v1[0] + v2[0];\n    res[1] = v1[1] + v2[1];\n    res[2] = v1[2] + v2[2];','res[2] = v1[2] + v2[2];\n    res[1] = v1[1] + v2[1];\n    res[0] = v1[0] + v2[0];','vector_3'),
 ('rotation_sign','tempo2Util.C','x[2] = -se*tmpy+ce*tmpz;','x[2] = se*tmpy+ce*tmpz;','rotation_1_forward'),
 ('copyparam_extent','tempo2Util.C','for (i=0;i<p1.aSize;i++)','for (i=0;i<p1.aSize-1;i++)','copyparam_3'),
 ('copyparam_links','tempo2Util.C','p2->aSize = p1.aSize;','p2->aSize = p1.aSize; p2->nLinkTo = 0;','copyparam_3'),
 ('copypsr_fd_extent','tempo2Util.C','for (i=0; i<=p[p2].nfdJumps; i++)','for (i=0; i<=p[p1].nfdJumps; i++)','copypsr_0'),
 ('copypsr_observation','tempo2Util.C','for (i=0;i<p[p1].nobs;i++)','for (i=0;i<p[p1].nobs-1;i++)','copypsr_0'),
 ('link_semantics','tempo2Util.C','return 1.0/psr->param[param_px].val[0];','return 2.0/psr->param[param_px].val[0];','link_formula_3'),
 ('diagnostic_suppression','tempo2Util.C','else if (!warnedAboutMultiple)','else if (true)','diagnostic_capacity'),
 ('diagnostic_storage','TKlog.h','TK_MAX_ERRORS 30','TK_MAX_ERRORS 29','diagnostic_error_storage'),
]


def diagnostics(rows):
    result=[]
    for r in rows:
        if not r['id'].startswith('rotation_'):continue
        v=first_result(r)
        if any(x['class']!='finite' for group in ['before','rotated','roundtrip'] for x in v[group]):continue
        b=[float(x['decimal']) for x in v['before']];rot=[float(x['decimal']) for x in v['rotated']];rt=[float(x['decimal']) for x in v['roundtrip']]
        scale=max(max(abs(x) for x in b),1.0)
        result.append(dict(id=r['id'],relative_norm_error=abs(math.hypot(*rot)-math.hypot(*b))/scale,
                           scaled_roundtrip_max_error=max(abs(x-y) for x,y in zip(b,rt))/scale))
    return dict(kind='independent binary64 norm/roundtrip diagnostics, not source comparator tolerances or port budgets',cases=result,
                max_relative_norm_error=max(x['relative_norm_error'] for x in result),max_scaled_roundtrip_error=max(x['scaled_roundtrip_max_error'] for x in result))


def main():
    (HERE/'verification.json').unlink(missing_ok=True)
    cs,rows,exe,p=regenerate();compare(json.loads((HERE/'inventory.json').read_text()),inventory(),'inventory')
    checks=controls(cs,rows,exe,p);mutants=[];bycase={c['id']:c for c in cs};byrow={r['id']:r for r in rows}
    # Same selected record must also match from an independent unmodified build path.
    pos,pp=compile_probe('positive');positive=execute(bycase['vector_1'],pos,pp);validate(bycase['vector_1'],positive)
    compare(semantic(byrow['vector_1']),semantic(positive),'positive_rebuild')
    for name,file,old,new,case in MUTATIONS:
        mx,mp=compile_probe('mut_'+name,(file,old,new));mr=execute(bycase[case],mx,mp);validate(bycase[case],mr)
        difference=rejected(lambda:compare(semantic(byrow[case]),semantic(mr),'behavior'))
        # Changes to path, executable hash or source-location text cannot satisfy this check.
        mutants.append(dict(name=name,case=case,difference=difference,profile=mp,execution=execution_identity(mx,mp),observation=mr))
        print('Rejected behavior mutant:',name,flush=True)
    save(HERE/'verification.json',dict(status='PASS',checks=checks,source_mutations=mutants,positive_build=pp,diagnostics=diagnostics(rows),
          baseline_execution=execution_identity(exe,p),policy='Full default rebuild first; goldens untouched. Mutant comparisons discard only event byte offsets and source file/line labels. A value/state/return/stream difference is required.'))
    print('PASS:',len(checks),'controls;',len(mutants),'compiled behavior mutants')


if __name__=='__main__':main()
