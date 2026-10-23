#!/usr/bin/env python3
"""Normal-gate negative controls and compiled disposable source sensitivity."""
import copy,json,shlex,subprocess,sys
from pathlib import Path
from profile import HERE,BUILD,sha,save,compile_probe
from run import compare,execute,validate,check_identity,execution_identity,runtime

MUTATIONS=[
('selector','selector_head_last','readTimfile.C','format = 2;','format = 0;'),
('state_retention','direct_repeat','readTimfile.C','static int infoNum=-1;','int infoNum=-1;'),
('directive_order','order_without_eset','readTimfile.C','psr->obsn[nObs].toaErr *= (efac);','psr->obsn[nObs].toaErr += (efac);'),
('jump_history','include_history','readTimfile.C','(*jumpVal)++;','(*jumpVal)+=2;'),
('fdjump_bug','flags','readTimfile.C','psr->obsn[nObs].fdjump[0] = *jumpVal;','psr->obsn[nObs].fdjump[0] = *fdjumpVal;'),
('writer_inversion','writer_inversions','readTimfile.C','oldsat -= psr->tOffset[k]/SECDAY;','oldsat += psr->tOffset[k]/SECDAY;'),
('writer_retained','writer_legacy','readTimfile.C',"psr[0].obsn[i].fname[22]='\\0';","psr[0].obsn[i].fname[21]='\\0';"),
('sort_latch','sort_append','sortToAs.C','if (!psr->sorted){','if (true){'),
('comparison','sort_reverse','sortToAs.C','return -1;\n    else if (elem1->sat > elem2->sat)\n        return 1;','return 1;\n    else if (elem1->sat > elem2->sat)\n        return -1;'),
('jbo_endian','jbo_records','readJBO_bat.C','raw[0]=d;','raw[0]=a;'),
('date_year','date_1230101','readJBO_bat.C','+ 1900;','+ 1901;'),
('direct_io','io_sequence_1','read_fortran2.h','return ival;','return ival+1;'),
('character_order','io_sequence_1','read_fortran2.h','str[i] = fgetc(c_fileptr2);','str[i] = fgetc(c_fileptr2);')
]
# Character routine has two matching assignments; choose the full branch below.
MUTATIONS[-1]=('character_order','io_sequence_1','read_fortran2.h','for (i=len-1;i>=0;i--)','for (i=0;i<len;i++)')

def rejected(fn):
    try:fn()
    except (AssertionError,FileNotFoundError) as e:return str(e)
    raise AssertionError('negative control was accepted')

def main():
    cs={c['id']:c for c in json.loads((HERE/'cases.json').read_text())};gold={r['id']:r for r in json.loads((HERE/'observations.json').read_text())}
    expected=json.loads((HERE/'profile.json').read_text());exe,p=compile_probe();p['stable']['runtime_fixtures']=runtime()
    check_identity(expected,p,exe)
    positive=execute(cs['modern_basic'],exe,p);validate(cs['modern_basic'],positive);compare(gold['modern_basic'],positive)
    controls={'fresh_build':'accepted with complete modern_basic comparison'}
    for key in ['compiler_sha256','sources','runtime','runner_sources','compiler_backend','header_dependencies']:
        changed=copy.deepcopy(p)
        if isinstance(changed['stable'][key],dict):changed['stable'][key]['CONTROL']='changed'
        else:changed['stable'][key]='changed'
        controls[key]=rejected(lambda:check_identity(expected,changed,exe))
    saved=exe.read_bytes();forward=BUILD/'forward_original';forward.write_bytes(saved);forward.chmod(0o755)
    try:
        exe.write_text('#!/bin/sh\nexec '+shlex.quote(str(forward))+' "$@"\n');exe.chmod(0o755)
        controls['same_path_forwarding_wrapper']=rejected(lambda:execute(cs['modern_basic'],exe,p))
    finally:exe.write_bytes(saved);exe.chmod(0o755)
    # Alter actual source/runtime declarations through the same normal identity gate.
    src=Path(next(x[2:] for x in p['command'] if x.startswith('-I')))
    for file in [src/'readTimfile.C',HERE/'probe.cc',HERE/'run.py',BUILD/'runtime'/'observatory'/'aliases']:
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
        ('missing_field',lambda x:x[0]['events'][0]['state'].pop('jump')),
        ('changed_class',lambda x:x[0].update(classification='host')),
        ('stdout_order',lambda x:x[0].update(stdout='CHANGED')),
        ('returncode',lambda x:x[0].update(returncode=1))]:
        a=list(gold.values());b=copy.deepcopy(a);change(b);neg[name]=rejected(lambda:compare(a,b))
    a=gold['writer_tempo2'];b=copy.deepcopy(a);b['outputs']['out.tim']=None
    neg['missing_file']=rejected(lambda:validate(cs['writer_tempo2'],b))
    # Benign synthetic launch control: receipt from a preceding real run cannot
    # satisfy a fresh nonce. The injected executable's identity is declared here.
    oldevents=BUILD/'cases'/'modern_basic'/'events.jsonl';stale=BUILD/'stale.jsonl';stale.write_bytes(oldevents.read_bytes())
    wrapper=BUILD/'stale_runner';wrapper.write_text('#!/bin/sh\ncp '+shlex.quote(str(stale))+' "$2"\n');wrapper.chmod(0o755)
    wp=copy.deepcopy(p);wp['executable_path']=str(wrapper);wp['executable_sha256']=sha(wrapper.read_bytes())
    neg['stale_launch']=rejected(lambda:execute(cs['modern_basic'],wrapper,wp))
    wrapper.write_text('#!/bin/sh\nexit 0\n');wp['executable_sha256']=sha(wrapper.read_bytes())
    neg['missing_launch']=rejected(lambda:execute(cs['modern_basic'],wrapper,wp))
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
    for name,cid,file,old,new in MUTATIONS:
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
