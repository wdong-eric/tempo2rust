"""Fresh exact replay, normal-gate controls and disposable defined compiled mutations."""
import base64
import copy
import gzip
import json
from pathlib import Path
import shutil
from profile import HERE,BUILD,compile_probe,save,sha,runners
from fixtures import cases
from run import regenerate,execute,compare,identity_gate,execution_identity,reference,validate,protected

MUTATIONS=[
 ('initial_nits','initialise.C','psr->nits=1;','psr->nits=2;','natural_1'),
 ('omitted_control_write','initialise.C','psr->nits=1;','psr->nits=1; psr->ne_sw_ifuncN=0;','seeded_reinitialise'),
 ('name_suffix','tempo2Util.C','strcpy(p[p2].name,p[p1].name);','memcpy(p[p2].name,p[p1].name,sizeof p[p2].name);','copy_partial'),
 ('matrix_columns','TKmatrix.C','*dimM = cols;','*dimM = cols>1 ? cols-1 : cols;','matrix_2_3'),
 ('matrix_stride','TKmatrix.C','uinv[i]=memory+cols*i;','uinv[i]=memory+(cols-1)*i;','matrix_2_3'),
 ('observation_order','tempo2Util.C','for (i=0;i<p[p1].nobs;i++)','for (i=p[p1].nobs-1;i>=0;i--)','copy_forward_overlap'),
 ('inclusive_jumps','tempo2Util.C','i<=p[p2].nJumps','i<p[p2].nJumps','copy_zero'),
 ('destination_fd_count','tempo2Util.C','i<=p[p2].nfdJumps','i<=p[p1].nfdJumps','copy_partial'),
 ('omitted_fd_count_copy','tempo2Util.C','p[p2].dm_series_type = p[p1].dm_series_type;','p[p2].dm_series_type = p[p1].dm_series_type; p[p2].nfdJumps = p[p1].nfdJumps;','copy_partial'),
 ('zero_covariance','TKmatrix.C','memory+=2; // the first two bytes are for the dimensions.','memory[2]=1; memory+=2; // the first two bytes are for the dimensions.','matrix_2_3'),
]

def rejected(fn):
    try:fn()
    except (AssertionError,FileNotFoundError) as e:return str(e)
    raise AssertionError('negative control was accepted')

def raw_check(c):
    raw=json.loads(gzip.decompress((HERE/'raw'/(c['id']+'.json.gz')).read_bytes()))
    events=[json.loads(s) for s in base64.b64decode(raw['events_base64']).splitlines()]
    assert events.pop(0)==dict(phase='launch',nonce=raw['nonce'])
    ref=reference(c['id'])
    # Raw source-path/address presentation and stdout offsets remain in the raw bundle.
    compare(raw['returncode'],ref['returncode'],'raw.status')
    for e,r in zip(events,ref['events']):
        compare(e['state'],r['state'],'raw.state');compare(e['phase'],r['phase'],'raw.phase')
    compare(len(events),len(ref['events']),'raw.event_count')
    return validate(ref)

def main():
    (HERE/'verification.json').unlink(missing_ok=True)
    import run
    receipt=HERE/'regeneration.json';receipt.write_text('{"status":"stale-success-control"}')
    build=run.compile_probe
    def fail_build():raise RuntimeError('declared failing build control')
    run.compile_probe=fail_build
    try:
        try:run.regenerate()
        except RuntimeError as error:assert str(error)=='declared failing build control'
        else:raise AssertionError('failing build unexpectedly succeeded')
        assert not receipt.exists(),'stale receipt survived failed normal attempt'
    finally:run.compile_probe=build
    cs,summary,exe,p=regenerate();controls=[dict(name='stale_success_receipt',rejection='failed normal attempt removed preceding success receipt')]
    def control(name,fn):controls.append(dict(name=name,rejection=rejected(fn)))
    r=reference('matrix_2_3')
    for name,edit in [
      ('missing_event',lambda x:x['events'].pop()),
      ('reordered_event',lambda x:x['events'].reverse()),
      ('extra_event',lambda x:x['events'].append(x['events'][0])),
      ('missing_field',lambda x:x['events'][0]['state'].pop()),
      ('extra_field',lambda x:x['events'][0]['state'].append({'path':'extra','state':'unwritten'})),
      ('wrong_status',lambda x:x.update(returncode=1)),
      ('changed_stream',lambda x:x.update(stderr='unexpected')),
      ('changed_runs',lambda x:x['events'][2]['state'][0]['runs'][0].__setitem__(1,'0'*16)),
      ('changed_run_count',lambda x:x['events'][2]['state'][0]['runs'][0].__setitem__(0,2)),
    ]:
        x=copy.deepcopy(r);edit(x);control(name,lambda x=x:compare(r,x))
    copycase=reference('copy_shared');x=copy.deepcopy(copycase)
    f=next(f for f in x['events'][1]['state'] if f['path']=='destination.obsn');f['symbol']='foreign@0'
    control('alias_owner',lambda:compare(copycase,x));del copycase,x
    for key in ['compiler_sha256','compiler_backend','sources','generated','header_dependencies','preprocessed_normalized_sha256','runtime','macros_sha256','runner_sources']:
        bad=copy.deepcopy(p);bad['stable'][key]='wrong'
        control('identity_'+key,lambda bad=bad:identity_gate(p,bad,exe))
    replacement=BUILD/'replacement';shutil.copyfile(exe,replacement);replacement.chmod(0o755)
    bound=copy.deepcopy(p);bound['executable_path']=str(replacement)
    replacement.write_text('#!/bin/sh\nexit 0\n');replacement.chmod(0o755)
    control('same_path_executable_replacement',lambda:execution_identity(replacement,bound))
    # Bind the actual replacement identity to challenge the separate fresh-output gate.
    bound['executable_sha256']=sha(replacement.read_bytes())
    c=next(c for c in cs if c['id']=='matrix_2_3')
    control('no_output_with_stale_case_directory',lambda:execute(c,replacement,bound))
    replacement.write_text('#!/bin/sh\nprintf \'{"phase":"launch","nonce":"stale"}\\n\' > "$1"\n');replacement.chmod(0o755);bound['executable_sha256']=sha(replacement.read_bytes())
    control('stale_challenge',lambda:execute(c,replacement,bound))
    mutants=[]
    for name,file,old,new,case in MUTATIONS:
        mx,mp=compile_probe('mutant_'+name,mutation=(file,old,new));c=next(c for c in cs if c['id']==case)
        result=execute(c,mx,mp,BUILD/'mutation_raw'/name);expected=reference(case)
        compare(c['returncode'],result['returncode'],name+'.defined_status')
        # Values and states, independently of source hashes, diagnostic filenames and byte offsets.
        a=[e['state'] for e in expected['events']];b=[e['state'] for e in result['events']]
        witness=rejected(lambda:compare(a,b,'state'))
        dest=HERE/'mutations';dest.mkdir(exist_ok=True)
        (dest/(name+'.json.gz')).write_bytes(gzip.compress(json.dumps(result,separators=(',',':')).encode(),mtime=0))
        # Retain raw mutant streams as well as the complete comparison projection.
        shutil.copyfile(BUILD/'mutation_raw'/name/(case+'.json.gz'),dest/(name+'.raw.json.gz'))
        mutants.append(dict(name=name,case=case,mutation=[file,old,new],witness=witness,profile=mp,execution=execution_identity(mx,mp)))
        print('mutation rejected:',name,witness[:180],flush=True)
    raw_records=sum(raw_check(c) for c in cs);protected()
    save(HERE/'verification.json',dict(status='PASS',controls=controls,mutants=mutants,raw_validated_records=raw_records,ordinary_execution=execution_identity(exe,p),scope='All compiled mutations are defined for their declared initialized bounded witnesses. No native undefined trial.'))
    print('PASS',len(controls),'controls and',len(mutants),'compiled mutations')
if __name__=='__main__':main()
