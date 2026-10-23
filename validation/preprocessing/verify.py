#!/usr/bin/env python3
"""Normal-gate negative controls and compiled disposable source sensitivity."""
import copy,json,shlex,subprocess,sys
from pathlib import Path
from profile import HERE,BUILD,sha,save,compile_probe
from run import compare,execute,validate,check_identity,execution_identity,runtime,load_results

def mutations():
    from profile import blob
    pre=blob('preProcess.C').decode().replace('\r\n','\n')
    simple=blob('preProcessSimple.C').decode()
    epoch=blob('updateEpoch.C').decode()
    ordered=simple[simple.index('        // Correctly taking note'):simple.index('        /* Check filtering */')]
    phase='        psr->obsn[i].sat += ((psr->obsn[i].phaseOffset/psr->param[param_f].val[0])/SECDAY); '
    assert phase in ordered
    tel=simple[simple.index('                if (strcasecmp(selectedSite, obsSite)!=0)'):simple.index('            if (strcasecmp(str1,"SCALED")')+len('            if (strcasecmp(str1,"SCALED")')]
    nstart=pre.index('                    //\t   Check equad');nmid=pre.index('                    //\t   Check efac',nstart);nend=pre.index('\n                }\n                err *=',nmid)
    noise=pre[nstart:nend]
    ec=epoch[:epoch.index('void updateEpoch(pulsar*')]
    fj='if (strcmp(psr->obsn[i].flagVal[k],val[l])==0)\n                        psr->obsn[i].jump[(psr->obsn[i].obsNjump)++]=psr->nJumps;'
    reverse='else\n    {\n        for (p=0;p<npsr;p++)\n        {\n            for (i=0;i<psr[p].nobs;i++)'
    return [
      ('phase_offset_order','stage2_order','preProcessSimple.C',ordered,ordered.replace(phase,'')+phase+'\n'),
      ('tel_inequality','jump_ordinary_3','preProcessSimple.C',tel,tel.replace('!=0','==0')),
      ('phase_assignment','phase_assignment','preProcessSimple.C','if ((psr->phaseJumpID[i] = -1))','if ((psr->phaseJumpID[i] == -1))'),
      ('fjump_value_only','fjump_value_only','preProcessSimple.C',fj,fj.replace('if (strcmp','if (strcmp(psr->obsn[i].flagID[k],psr->fjumpID)==0 && strcmp')),
      ('uncertainty_order','noise_order','preProcess.C',noise,pre[nmid:nend]+pre[nstart:nmid]),
      ('select_boundary','select_PASS_MJD_False','preProcess.C','(double)psr[p].obsn[i].sat < v1 ||\n                                 (double)psr[p].obsn[i].sat > v2','(double)psr[p].obsn[i].sat <= v1 ||\n                                 (double)psr[p].obsn[i].sat > v2'),
      ('simultaneous_order','simul_order_chain','preProcess.C',reverse,reverse.replace('i=0;i<psr[p].nobs;i++','i=psr[p].nobs-1;i>=0;i--')),
      ('simultaneous_carried','simul_only_carried','preProcess.C','                        for (k=0;k<psr[p].obsn[i].nFlags;k++)\n                        {\n                            if (strcmp(psr[p].obsn[i].flagID[k],s1)==0 &&\n                                    strcmp(psr[p].obsn[i].flagVal[k],s3)==0','                        found1=found2=0;\n                        for (k=0;k<psr[p].obsn[i].nFlags;k++)\n                        {\n                            if (strcmp(psr[p].obsn[i].flagID[k],s1)==0 &&\n                                    strcmp(psr[p].obsn[i].flagVal[k],s3)==0'),
      ('mjd_prefit_index','mjd_1_1.0000000155051979176','units.C','(p->prefit[0] - IFTE_MJD0)','(p->prefit[arr] - IFTE_MJD0)'),
      ('constant_promotion','units_2_1_@','ifteph.h','#define IFTE_KM1 1.55051979176e-8','#define IFTE_KM1 1.55051979176e-8Q'),
      ('epoch_pulsar_zero','epoch_cross_1','updateEpoch.C',ec,ec.replace('psr->param','psr[p].param')),
      ('epoch_dt_narrowing','epoch_precision','updateEpoch.C','double dt = (nMJD','longdouble dt = (nMJD'),
      ('epoch_prefit','epoch_motion_15','updateEpoch.C','psr[p].param[param_f].prefit[1] = psr[p].param[param_f].val[1];','psr[p].param[param_f].prefit[1] = psr[p].param[param_f].val[2];'),
      ('nofit_fd_zero','wrapper_options','preProcess.C','for (i=1;i<=psr[p].nfdJumps;i++)','for (i=0;i<=psr[p].nfdJumps;i++)'),
      ('link_enum_order','wrapper_link_chain','preProcess.C','for (i=0;i<MAX_PARAMS;i++){\n            if(psr[p].param[i].nLinkTo>0)','for (i=MAX_PARAMS-1;i>=0;i--){\n            if(psr[p].param[i].nLinkTo>0)'),
      ('dm_index_zero','dm_direct_0','preProcessSimple.C','if ((int)(psr->obsn[i].sat-startdmmjd+0.5) > 0)','if ((int)(psr->obsn[i].sat-startdmmjd+0.5) >= 0)'),
    ]

def rejected(fn):
    try:fn()
    except (AssertionError,FileNotFoundError) as e:return str(e)
    raise AssertionError('negative control was accepted')

def main():
    cs={c['id']:c for c in json.loads((HERE/'cases.json').read_text())};gold={r['id']:r for r in load_results()}
    expected=json.loads((HERE/'profile.json').read_text());exe,p=compile_probe();p['stable']['runtime_fixtures']=runtime()
    check_identity(expected,p,exe)
    positive=execute(cs['simple_repeat'],exe,p);validate(cs['simple_repeat'],positive);compare(gold['simple_repeat'],positive)
    controls={'fresh_build':'accepted with complete simple_repeat comparison'}
    for key in ['compiler_sha256','sources','runtime','runner_sources','compiler_backend','header_dependencies']:
        changed=copy.deepcopy(p)
        if isinstance(changed['stable'][key],dict):changed['stable'][key]['CONTROL']='changed'
        else:changed['stable'][key]='changed'
        controls[key]=rejected(lambda:check_identity(expected,changed,exe))
    saved=exe.read_bytes();forward=BUILD/'forward_original';forward.write_bytes(saved);forward.chmod(0o755)
    try:
        exe.write_text('#!/bin/sh\nexec '+shlex.quote(str(forward))+' "$@"\n');exe.chmod(0o755)
        controls['same_path_forwarding_wrapper']=rejected(lambda:execute(cs['simple_repeat'],exe,p))
    finally:exe.write_bytes(saved);exe.chmod(0o755)
    # Alter actual source/runtime declarations through the same normal identity gate.
    src=Path(next(x[2:] for x in p['command'] if x.startswith('-I')))
    for file in [src/'preProcess.C',HERE/'probe.cc',HERE/'run.py',BUILD/'runtime'/'observatory'/'aliases']:
        old=file.read_bytes()
        try:
            file.write_bytes(old+b'\n')
            controls['actual_'+file.name]=rejected(lambda:check_identity(expected,p,exe))
        finally:file.write_bytes(old)
    neg={}
    for name,change in [
        ('missing_case',lambda x:x.pop()),
        ('reordered_cases',lambda x:x.reverse()),
        ('missing_event',lambda x:x[0]['events'].pop(1)),
        ('missing_field',lambda x:x[0]['events'][2]['changes'].pop(next(iter(x[0]['events'][2]['changes'])))),
        ('changed_class',lambda x:x[0].update(classification='host')),
        ('stdout_order',lambda x:x[0].update(stdout='CHANGED')),
        ('returncode',lambda x:x[0].update(returncode=1))]:
        a=list(gold.values());b=copy.deepcopy(a);change(b);neg[name]=rejected(lambda:compare(a,b))
    # Benign synthetic launch control: receipt from a preceding real run cannot
    # satisfy a fresh nonce. The injected executable's identity is declared here.
    oldevents=BUILD/'cases'/'simple_repeat'/'events.jsonl';stale=BUILD/'stale.jsonl';stale.write_bytes(oldevents.read_bytes())
    wrapper=BUILD/'stale_runner';wrapper.write_text('#!/bin/sh\ncp '+shlex.quote(str(stale))+' "$2"\n');wrapper.chmod(0o755)
    wp=copy.deepcopy(p);wp['executable_path']=str(wrapper);wp['executable_sha256']=sha(wrapper.read_bytes())
    neg['stale_launch']=rejected(lambda:execute(cs['simple_repeat'],wrapper,wp))
    wrapper.write_text('#!/bin/sh\nexit 0\n');wp['executable_sha256']=sha(wrapper.read_bytes())
    neg['missing_launch']=rejected(lambda:execute(cs['simple_repeat'],wrapper,wp))
    # Exercise main's failure path and stale-success removal, with protected gold restored.
    frozen=(HERE/'profile.json').read_bytes()
    try:
        altered=copy.deepcopy(expected);altered['stable']['compiler_sha256']='tampered'
        save(HERE/'profile.json',altered);save(HERE/'regeneration.json',{'stale_success':True})
        proc=subprocess.run([sys.executable,str(HERE/'run.py')],capture_output=True,text=True)
        assert proc.returncode!=0 and not (HERE/'regeneration.json').exists()
        controls['normal_cli_failure_removes_receipt']=dict(returncode=proc.returncode,stdout=proc.stdout,stderr=proc.stderr)
    finally:(HERE/'profile.json').write_bytes(frozen)
    proc=subprocess.run([sys.executable,str(HERE/'run.py'),'--no-build'],capture_output=True,text=True)
    assert proc.returncode==2
    controls['no_build_option_absent']=dict(returncode=proc.returncode,stderr=proc.stderr)
    mutations=[]
    for name,cid,file,old,new in mutations():
        mx,mp=compile_probe('mut_'+name,(file,old,new));mp['stable']['runtime_fixtures']=runtime()
        identity_rejection=rejected(lambda:check_identity(expected,mp,mx))
        r=execute(cs[cid],mx,mp)
        # Only source-visible state/data/files/status can prove sensitivity.
        observable=lambda x:{k:x[k] for k in ['events','outputs','returncode']}
        difference=rejected(lambda:compare(observable(gold[cid]),observable(r),'observable'))
        mutations.append(dict(name=name,case=cid,mutation=[file,old,new],identity_rejection=identity_rejection,observable_difference=difference,profile=mp,result=r))
        print('PASS compiled mutation',name,flush=True)
    execution_identity(exe,p,True)
    save(HERE/'verification.json',dict(profile=p,execution_identity=execution_identity(exe,p,True),identity_controls=controls,comparison_controls=neg,compiled_mutations=mutations))
    print('PASS integrity and sensitivity')
if __name__=='__main__':main()
