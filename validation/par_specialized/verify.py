"""Normal identity/launch/comparator controls and compiled semantic sensitivity."""
import copy,gzip,json,shlex,subprocess,sys
from pathlib import Path
from profile import HERE,BUILD,sha,save,compile_probe
from run import compare,execute,validate,check_identity,execution_identity,runtime,read_observations
from protected import snapshot

MUTATIONS=[
('precedence','substring_precedence','readParfile.C','else if (strstr(str,"WAVE_SIN")!=NULL || strstr(str,"wave")!=NULL)','else if (strstr(str,"WAVE_SIN")!=NULL)'),
('count_reset','dm_histories','readParfile.C','psr->dmoffsCMnum=0;','psr->dmoffsCMnum=1;'),
('reader_reset','reader_full','readParfile.C','psr[p].nJumps=0;','/* mutation: retain ordinary count */'),
('jump_retention','jump_retention','readParfile.C','psr->nJumps++;','psr->nJumps++; psr->jumpSAT[psr->nJumps]=0;'),
('ifunc_weight','samples_IFUNC','readParfile.C','psr->ifunc_weights[number-1]=1.0;','psr->ifunc_weights[number-1]=2.0;'),
('ifunc_error','samples_IFUNC','readParfile.C','&psr->ifuncE[number-1]);','&psr->ifuncV[number-1]);'),
('constraint_accumulation','constraint_DMMODEL_DM1','readParfile.C','psr->constraints[psr->nconstraints++] = constraint_dmmodel_dm1;','/* mutation: lose individual DM1 term */'),
('constraint_terminator','constraint_IFUNC_COV','readParfile.C',"psr->constraint_special[psr->nconstraints][strlen(txt)]='\\n';","psr->constraint_special[psr->nconstraints][strlen(txt)]='\\0';"),
('subtract_packing','initial_and_helpers','t2fit_nestlike.h','packed &= ~(0xFFULL << 8);','packed &= ~(0xFFFFULL << 8);'),
('rn_conversion','rn_conversion','readParfile.C','log10(2.0*M_PI*pow(3.0,0.5)/(86400.0*365.25*1e6)*psr->TNRedAmp)','log10(2.0*M_PI*pow(3.0,0.5)/(86400.0*365.25*1e3)*psr->TNRedAmp)'),
('shape_zeroing','shape_retention','readParfile.C','logwarn("Couldn\'t read Shape coef for shape %d coef=%d; setting coefs to zero.",nTNEv,icoef);','logwarn("Couldn\'t read Shape coef for shape %d coef=%d; setting coefs to zero.",nTNEv,icoef); for(int j=icoef;j<psr->TNShapeletEvN[nTNEv];j++)psr->TNShapeletEvCoef[nTNEv][j]=0;'),
('shape_consumption','shape_count_3','readParfile.C','icoef < psr->TNShapeletEvN[nTNEv];','icoef < psr->TNShapeletEvN[nTNEv]-1;'),
('offset_flag','average_offsets','readParfile.C',"if (str[k]=='-') /* Have flag */","if (str[k]=='@') /* mutation */"),
('fd_pint','fd_order','readParfile.C','psr->fdjump_pint_format = 1; // FDxJUMP (PINT) format\n                char rest[1024];','psr->fdjump_pint_format = 0; // mutation\n                char rest[1024];')]

def rejected(fn):
    try:fn()
    except (AssertionError,FileNotFoundError) as e:return str(e)
    raise AssertionError('negative control accepted')

def main():
    (HERE/'verification.json').unlink(missing_ok=True)
    cs={c['id']:c for c in json.loads((HERE/'cases.json').read_text())};gold={r['id']:r for r in read_observations()}
    frozen=json.loads((HERE/'profile.json').read_text());exe,p=compile_probe('verify_probe');p['stable']['runtime_fixtures']=runtime();check_identity(frozen,p,exe)
    positive=execute(cs['initial_and_helpers'],exe,p);validate(cs['initial_and_helpers'],positive);compare(gold['initial_and_helpers'],positive)
    controls={'positive':'complete fresh initialized state and helpers exact'}
    for key in ['compiler_sha256','sources','runtime','runner_sources','compiler_backend','header_dependencies','macros_sorted_sha256']:
        changed=copy.deepcopy(p)
        if isinstance(changed['stable'][key],dict):changed['stable'][key]['CONTROL']='changed'
        else:changed['stable'][key]='changed'
        controls[key]=rejected(lambda:check_identity(frozen,changed,exe))
    original=exe.read_bytes();forward=BUILD/'forward_original';forward.write_bytes(original);forward.chmod(0o755)
    try:
        exe.write_text('#!/bin/sh\nexec '+shlex.quote(str(forward))+' "$@"\n');exe.chmod(0o755)
        controls['same_path_forwarder']=rejected(lambda:execute(cs['initial_and_helpers'],exe,p))
    finally:exe.write_bytes(original);exe.chmod(0o755)
    src=Path(next(x[2:] for x in p['command'] if x.startswith('-I')))
    for file in [src/'readParfile.C',src/'config.h',HERE/'probe.cc',HERE/'run.py',BUILD/'runtime'/'clock'/'a.clk']:
        old=file.read_bytes()
        try:
            file.write_bytes(old+b'\n');controls['actual_'+file.name]=rejected(lambda:check_identity(frozen,p,exe))
        finally:file.write_bytes(old)
    neg={};base=list(gold.values())
    for name,change in [('missing_case',lambda x:x.pop()),('reordered_cases',lambda x:x.reverse()),('missing_event',lambda x:x[0]['events'].pop(1)),('initial_membership',lambda x:x[0]['events'][0].pop('state_sha256')),('state_byte',lambda x:x[1]['events'][-2]['changes'].update(unexpected_byte=1)),('classification',lambda x:x[0].update(classification='other')),('stdout',lambda x:x[0].update(stdout='other')),('returncode',lambda x:x[0].update(returncode=1))]:
        alt=copy.deepcopy(base);change(alt);neg[name]=rejected(lambda:compare(base,alt));del alt
    # The normal validator checks all direct token begin/end memberships too.
    altered=copy.deepcopy(gold['jump_retention']);altered['events'].pop(next(i for i,e in enumerate(altered['events']) if e['phase']=='dispatch_end'))
    neg['incomplete_dispatch']=rejected(lambda:validate(cs['jump_retention'],altered))
    stale=BUILD/'stale.jsonl';stale.write_bytes((BUILD/'cases'/'initial_and_helpers'/'events.jsonl').read_bytes())
    wrapper=BUILD/'stale_runner';wrapper.write_text('#!/bin/sh\ncp '+shlex.quote(str(stale))+' "$2"\n');wrapper.chmod(0o755)
    wp=copy.deepcopy(p);wp['executable_path']=str(wrapper);wp['executable_sha256']=sha(wrapper.read_bytes())
    neg['stale_launch']=rejected(lambda:execute(cs['initial_and_helpers'],wrapper,wp))
    wrapper.write_text('#!/bin/sh\nexit 0\n');wp['executable_sha256']=sha(wrapper.read_bytes())
    neg['missing_launch']=rejected(lambda:execute(cs['initial_and_helpers'],wrapper,wp))
    old=(HERE/'profile.json').read_bytes()
    try:
        altered=copy.deepcopy(frozen);altered['stable']['compiler_sha256']='CONTROL'
        save(HERE/'profile.json',altered);save(HERE/'regeneration.json',{'stale_success':True})
        proc=subprocess.run([sys.executable,str(HERE/'run.py')],capture_output=True,text=True)
        assert proc.returncode!=0 and not (HERE/'regeneration.json').exists() and not (HERE/'replay.raw.json.gz').exists()
        controls['normal_failure_clears_success']=dict(returncode=proc.returncode,stdout=proc.stdout,stderr=proc.stderr)
    finally:(HERE/'profile.json').write_bytes(old)
    proc=subprocess.run([sys.executable,str(HERE/'run.py'),'--no-build'],capture_output=True,text=True);assert proc.returncode==2
    controls['no_build_absent']=dict(returncode=proc.returncode,stderr=proc.stderr)
    mutations=[]
    for name,cid,file,old,new in MUTATIONS:
        mx,mp=compile_probe('mut_'+name,(file,old,new));mp['stable']['runtime_fixtures']=runtime()
        identity_rejection=rejected(lambda:check_identity(frozen,mp,mx))
        r=execute(cs[cid],mx,mp)
        observable=lambda x:{k:x[k] for k in ['events','returncode']}
        difference=rejected(lambda:compare(observable(gold[cid]),observable(r),'native_observable'))
        target=HERE/'mutations'/(name+'.json.gz');target.parent.mkdir(exist_ok=True)
        target.write_bytes(gzip.compress(json.dumps(dict(result=r,profile=mp),separators=(',',':')).encode(),mtime=0))
        mutations.append(dict(name=name,case=cid,mutation=[file,old,new],identity_rejection=identity_rejection,observable_difference=difference,artifact=str(target.relative_to(HERE)),sha256=sha(target.read_bytes()),executed_binary=mp['executable_sha256'],execution_identity=execution_identity(mx,mp,True)))
        print('PASS compiled',name,flush=True)
    compare(json.loads((HERE/'protected.json').read_text()),snapshot(),'protected')
    save(HERE/'verification.json',dict(profile=p,execution_identity=execution_identity(exe,p,True),identity_controls=controls,comparison_controls=neg,compiled_mutations=mutations))
    print('PASS identity, comparator, launch and compiled semantic controls')
if __name__=='__main__':main()
