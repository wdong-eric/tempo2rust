#!/usr/bin/env python3
"""Integrity checks and disposable compiled source mutations, never canonical edits."""
import copy
import json
from pathlib import Path
import shlex
import subprocess
import sys
from profile import HERE, BUILD, sha, save, prepare, compile_probe
from run import compare, execute, validate, check_identity, check_execution_identity

MUTATIONS = [
 ('dispatch', 'alias_DM', 'readParfile.C', '&(psr->param[param_dm]),0);', '&(psr->param[param_cm]),0);'),
 ('index', 'index_FB', 'readParfile.C', '&(psr->param[param_fb]),fbval);', '&(psr->param[param_fb]),0);'),
 ('duplicate', 'value_forms', 'readParfile.C', 'parameter->err[arr]     = 0.0;', 'parameter->err[arr]     = 0.0; parameter->prefitErr[arr] = 0.0;'),
 ('uncertainty', 'coordinate_duplicate', 'readParfile.C', 'parameter->prefitErr[arr]/=(12.0*60.0*60.0/M_PI);', 'parameter->prefitErr[arr]/=(24.0*60.0*60.0/M_PI);'),
 ('link_direction', 'link_DSHK_after', 'tempo2Util.C', 'return 1.0/psr->param[param_px].val[0];', 'return psr->param[param_px].val[0];'),
 ('finalization', 'final_defaults', 'readParfile.C', 'copyParam(psr->param[param_pepoch],&(psr->param[param_posepoch]));', '/* mutation: omit position epoch copy */'),
 ('wrapper_reset', 'repeat_full', 'readParfile.C', 'psr[p].nJumps=0;', 'psr[p].nJumps=1;'),
 ('failure_stage', 'final_no_pepoch', 'readParfile.C', 'printf("ERROR [PAR1]: Have not set a period epoch in %s\\n",filename);\n        exit(1);', 'printf("ERROR [PAR1]: Have not set a period epoch in %s\\n",filename);\n        return;'),
]


def must_reject(a,b,name):
    try:compare(a,b)
    except AssertionError as e:return dict(name=name,rejected=True,diagnostic=str(e))
    raise AssertionError('Comparator accepted mutation '+name)


def reject_identity(name,expected,actual,exe,diagnostic):
    try:check_identity(expected,actual,exe)
    except AssertionError as e:
        assert diagnostic in str(e),str(e)
        return dict(name=name,rejected=True,diagnostic=str(e))
    raise AssertionError('Identity gate accepted '+name)


def main():
    (HERE/'verification.json').unlink(missing_ok=True)
    cs=json.loads((HERE/'cases.json').read_text());rs=json.loads((HERE/'observations.json').read_text())
    assert len(cs)==len(rs) and len({c['id'] for c in cs})==len(cs)
    for c,r in zip(cs,rs):validate(c,r)
    cm={c['id']:c for c in cs};rm={r['id']:r for r in rs}
    src=prepare();exe,profile=compile_probe(src)
    expected_profile=json.loads((HERE/'profile.json').read_text())
    identity_checks=[dict(name='fresh build accepted',accepted=True,
                         execution_identity=check_identity(expected_profile,profile,exe))]
    positive=execute(cm['numeric'],exe,profile);validate(cm['numeric'],positive)
    compare(rm['numeric'],positive,'identity.positive.numeric')
    for name,keys in [('wrong compiler',['compiler_sha256']),
                      ('wrong source',['sources','readParfile.C']),
                      ('wrong runner',['runner_sources','run.py']),
                      ('wrong harness',['runner_sources','probe.cc'])]:
        changed=copy.deepcopy(profile);parent=changed
        for key in keys[:-1]:parent=parent[key]
        parent[keys[-1]]='0'*64
        diagnostic=('execution.' if keys[0]=='runner_sources' else 'profile.')+'.'.join(keys)
        identity_checks.append(reject_identity(name,expected_profile,changed,exe,diagnostic))
    # Same-path harmless forwarding wrapper: numerical equivalence must not
    # authorize attributing its launch to the freshly compiled native hash.
    native=BUILD/'identity_native';native.unlink(missing_ok=True);exe.rename(native)
    try:
        exe.write_text('#!/bin/sh\nexec '+shlex.quote(str(native))+' "$@"\n');exe.chmod(0o700)
        control=reject_identity('forwarding wrapper',expected_profile,profile,exe,'execution.executable_sha256')
        control.update(actual_wrapper_sha256=sha(exe.read_bytes()),native_sha256=sha(native.read_bytes()))
        identity_checks.append(control)
    finally:
        exe.unlink(missing_ok=True);native.rename(exe)
    check_identity(expected_profile,profile,exe)
    receipt=HERE/'regeneration.json'
    before=(sha(receipt.read_bytes()),receipt.stat().st_mtime_ns) if receipt.exists() else None
    command=[sys.executable,str(HERE/'run.py'),'--no-build']
    rejected=subprocess.run(command,capture_output=True,text=True,timeout=20)
    assert rejected.returncode==2 and 'unrecognized arguments: --no-build' in rejected.stderr
    after=(sha(receipt.read_bytes()),receipt.stat().st_mtime_ns) if receipt.exists() else None
    compare(before,after,'removed shortcut receipt')
    identity_checks.append(dict(name='removed --no-build cannot publish',rejected=True,command=command,
                                returncode=rejected.returncode,stdout=rejected.stdout,stderr=rejected.stderr,
                                receipt_before=before,receipt_after=after))
    # Reuse measured coverage only after checking all native inputs and flags.
    measured=json.loads((HERE/'line_coverage.json').read_text())['profile']
    coverage_keys=['source_revision','compiler','compiler_sha256','platform','machine','locale',
                   'sources','generated','numeric_header_sha256','globals_data_sha256','probe_sha256',
                   'runtime_hashes','support_policy','mutation']
    for key in coverage_keys:compare(measured[key],profile[key],'coverage_reuse.'+key)
    commands=[]
    for p in [measured,profile]:
        commands.append([s for s in p['command'][:-2] if s!='--coverage' and not s.startswith('-Wl,-map,')])
    compare(commands[0],commands[1],'coverage_reuse.native_command')
    coverage_reuse=dict(line_coverage_sha256=sha((HERE/'line_coverage.json').read_bytes()),
                        native_identity_fields_checked=coverage_keys,native_command_checked=commands[0],
                        reason='Only Python identity gating/receipts changed; native sources, harness, configuration, compiler and flags unchanged.')
    checks=[]
    for name,kind in [('case deletion','case'),('extra case','extra'),('case reorder','order'),('event deletion','event'),('field deletion','field'),('component deletion','component'),('class edit','class'),('action edit','action')]:
        b=copy.deepcopy(rs)
        if kind=='case':b.pop()
        elif kind=='extra':b.append(copy.deepcopy(b[0]))
        elif kind=='order':b[0],b[1]=b[1],b[0]
        elif kind=='event':b[0]['events'].pop()
        elif kind=='field':del b[0]['returncode']
        elif kind=='component':b[0]['events'][3]['result'].pop(next(iter(b[0]['events'][3]['result'])))
        elif kind=='class':next(e for e in b if e['id']=='numeric')['events'][1]['result']['class']='NaN'
        elif kind=='action':b[0]['actions'][0][2]='99'
        checks.append(must_reject(rs,b,name))
    # Actual launch challenge: execute a successful no-output program against a
    # previously populated case directory. Directory deletion must remove stale data.
    noop=Path('/usr/bin/true');noop_profile=copy.deepcopy(profile)
    noop_profile.update(command=[str(noop)],executable_sha256=sha(noop.read_bytes()))
    try:execute(cm['numeric'],noop,noop_profile)
    except (FileNotFoundError,AssertionError) as e:checks.append(dict(name='stale output / no probe launch',rejected=True,diagnostic=type(e).__name__))
    else:raise AssertionError('Accepted no-op executable')
    alloc=[next(e['result'] for e in rm['allocate_'+str(i)]['events'] if e['phase']=='end') for i in [0,1]]
    compare(*alloc,'unused realloc argument')
    init=[rm[n]['events'][1]['result'] for n in ['registry','init_full']]
    a,b=copy.deepcopy(init);a['natural_controls'].pop('noWarnings');b['natural_controls'].pop('noWarnings');compare(a,b,'unused fullSetup')
    evidence=[]
    for name,case,file,old,new in MUTATIONS:
        exe,profile=compile_probe(src,'mutation_'+name,(file,old,new))
        observed=execute(cm[case],exe,profile)
        rejection=must_reject(rm[case],observed,name)
        evidence.append(dict(name=name,case=case,profile=profile,
                             execution_identity=check_execution_identity(exe,profile),
                             rejection=rejection,observed=observed))
        print('Detected source mutation:',name,flush=True)
    save(HERE/'verification.json',dict(identity_checks=identity_checks,coverage_reuse=coverage_reuse,
         comparator_checks=checks,source_mutations=evidence,
         initialization_equivalence=['allocateMemory realloc=0/1','initialiseOne fullSetup=0/1 except explicit noWarnings input'],
         cases_sha256=sha((HERE/'cases.json').read_bytes()),observations_sha256=sha((HERE/'observations.json').read_bytes())))
    print('PASS:',len(identity_checks),'identity controls, membership/freshness gates and',len(evidence),'compiled source mutations')


if __name__=='__main__':main()
