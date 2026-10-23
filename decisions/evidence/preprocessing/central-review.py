#!/usr/bin/env python3
"""Independent replay of delivered P, preserving all sealed artifacts.

Run at the original fixture paths: source stores some caller paths as bytes.
Only ignored build products/case workspaces are written. No golden/profile,
source caller, comparator, tolerance, or scientific state is modified.
"""
import collections, copy, gzip, importlib.util, json, os, shlex, subprocess, sys, tempfile
from pathlib import Path
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
HERE=ROOT/'validation/preprocessing'
DELIVERY='2038c41'
OUT=Path(os.environ.get('TEMPO2_PREPROCESSING_CENTRAL_OUT','/tmp/tempo2-preprocessing-central-review.json'))
sys.path.insert(0,str(HERE))
import profile as native
import run
import verify
from fixtures import cases
from inventory import inventory
sha=native.sha
load=lambda p: json.loads(gzip.decompress(p.read_bytes()) if p.suffix=='.gz' else p.read_bytes())
def committed(name):return subprocess.check_output(['git','-C',str(ROOT),'show',DELIVERY+':validation/preprocessing/'+name])
def rejected(fn):
    try:fn()
    except (AssertionError,FileNotFoundError) as e:return str(e)
    raise AssertionError('negative control accepted')

def main():
    OUT.unlink(missing_ok=True)
    seal=load(HERE/'delivery.json')
    artifact_hashes=dict(seal['artifact_sha256'])
    artifact_hashes['delivery.json']=sha((HERE/'delivery.json').read_bytes())
    for name,h in artifact_hashes.items():
        assert sha((HERE/name).read_bytes())==h,name
        assert sha(committed(name))==h,('commit',name)
    for name,h in seal['protected_artifact_sha256'].items():assert sha((ROOT/name).read_bytes())==h,name
    nested={}
    for name,expected in seal['nested_repositories'].items():
        cmd=['git','-C',str(ROOT/name)]
        actual=dict(head=subprocess.check_output(cmd+['rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(cmd+['status','--porcelain'],text=True).strip())
        assert actual==expected,(name,actual)
        nested[name]=actual
    print('PASS committed artifacts, protected inputs and nested repositories',flush=True)
    frozen=load(HERE/'profile.json'); regeneration=load(HERE/'regeneration.json'); delivered=load(HERE/'verification.json.gz'); coverage=load(HERE/'coverage.json')
    run.compare(seal['source_pin'],native.PIN)
    for name,h in seal['raw_primary_and_constant_sha256'].items():assert sha(native.blob(name))==h,name
    profiles={'reference':frozen,'replay':regeneration['executed_profile'],'verification':delivered['profile'],'coverage':coverage['profile']}
    for m in delivered['compiled_mutations']:profiles['mutation.'+m['name']]=m['profile']
    identities={k:run.execution_identity(Path(p['executable_path']),p,True) for k,p in profiles.items()}
    run.compare(seal['current_executed_identities'],identities,'delivered.executed')
    print('PASS all delivered executed identities',len(profiles),flush=True)
    gold=run.load_results();by_id={r['id']:r for r in gold};cs=cases()
    run.compare(load(HERE/'cases.json'),cs,'cases');run.compare(load(HERE/'inventory.json'),inventory(),'inventory')
    run.compare([r['id'] for r in gold],[c['id'] for c in cs],'membership')
    spec=importlib.util.spec_from_file_location('p_inspect',HERE/'inspect.py');inspection=importlib.util.module_from_spec(spec);spec.loader.exec_module(inspection)
    initial=load(HERE/'initial.json.gz')
    for r in gold:
        state,_=inspection.reconstruct(r,initial)
        for v in state.values():
            if isinstance(v,dict) and set(v)=={'size','nonzero'}:inspection.unpack(v)
    print('PASS complete delivered delta chains',len(gold),flush=True)
    mutations={}
    expected_mutations={m[0]:m for m in verify.source_mutations()}
    assert set(expected_mutations)=={m['name'] for m in delivered['compiled_mutations']}
    for m in delivered['compiled_mutations']:
        spec=expected_mutations[m['name']]
        run.compare(list(spec[2:]),m['mutation'],'mutation.specification')
        run.compare(m['mutation'],m['profile']['mutation'],'mutation.profile')
        assert m['case']==spec[1]
        a,b=by_id[m['case']],m['result']
        select=lambda x:{'events':x['events'][1:],'outputs':x['outputs'],'returncode':x['returncode']}
        mutations[m['name']]=rejected(lambda:run.compare(select(a),select(b)))
    print('PASS 16 delivered post-initial mutation differences',flush=True)
    exe,p=native.compile_probe('central_probe');p['stable']['runtime_fixtures']=run.runtime()
    # All stable fields must match, including unchanged plugin binary identities.
    identity=run.check_identity(frozen,p,exe)
    print('PASS fresh native build; exact complete stable profile',p['executable_sha256'],flush=True)
    receipts=[]
    for i,(c,expected) in enumerate(zip(cs,gold)):
        actual=run.execute(c,exe,p);run.validate(c,actual);run.compare(expected,actual,'fresh.'+c['id'])
        work=native.BUILD/'cases'/c['id'];receipt=load(work/'receipt.json')
        receipts.append(dict(case=c['id'],classification=c['classification'],actions=len(c['actions']),events=len(actual['events']),complete_exact_comparison=True,receipt=receipt,initial_state_sha256=actual['events'][0]['state_sha256']))
        if i%25==0:print('PASS fresh cases',i+1,'/',len(cs),c['id'],flush=True)
    run.execution_identity(exe,p,True)
    assert len({r['receipt']['nonce'] for r in receipts})==len(cs)
    print('PASS complete fresh replay',len(cs),flush=True)
    # Independent controls use the real normal gate and comparator. All mutated
    # physical files below belong to this central build or a temporary directory.
    controls={'positive_fresh_build':identity}
    for key in ['compiler_sha256','sources','runtime','runner_sources','compiler_backend','header_dependencies','plugins','runtime_fixtures','macros_sorted_sha256','preprocessed_normalized_sha256']:
        changed=copy.deepcopy(p)
        if isinstance(changed['stable'][key],dict):changed['stable'][key]['CONTROL']='changed'
        else:changed['stable'][key]='changed'
        controls['profile_'+key]=rejected(lambda:run.check_identity(frozen,changed,exe))
    saved=exe.read_bytes()
    try:
        exe.write_text('#!/bin/sh\nexit 0\n');exe.chmod(0o755)
        controls['actual_same_path_executable']=rejected(lambda:run.execute(cs[0],exe,p))
    finally:exe.write_bytes(saved);exe.chmod(0o755)
    src=Path(next(x[2:] for x in p['command'] if x.startswith('-I')))
    f=src/'preProcess.C';saved=f.read_bytes()
    try:
        f.write_bytes(saved+b'\n');controls['actual_source_bytes']=rejected(lambda:run.check_identity(frozen,p,exe))
    finally:f.write_bytes(saved)
    for name,change in [
        ('missing_case',lambda x:x.pop()),('reordered_cases',lambda x:x.reverse()),
        ('missing_event',lambda x:x[0]['events'].pop(1)),
        ('missing_field',lambda x:x[0]['events'][2]['changes'].pop(next(iter(x[0]['events'][2]['changes'])))),
        ('changed_class',lambda x:x[0].update(classification='host')),('stdout',lambda x:x[0].update(stdout='changed')),
        ('returncode',lambda x:x[0].update(returncode=1))]:
        # Two small cases suffice to exercise membership/order and field changes.
        a=gold[:2];b=copy.deepcopy(a);change(b);controls['comparison_'+name]=rejected(lambda:run.compare(a,b))
    c=next(c for c in cs if c['id']=='simple_repeat')
    with tempfile.TemporaryDirectory(prefix='tempo2-p-central-control-') as directory:
        d=Path(directory);stale=d/'events.jsonl';stale.write_bytes((native.BUILD/'cases'/c['id']/'events.jsonl').read_bytes())
        wrapper=d/'runner';wrapper.write_text('#!/bin/sh\ncp '+shlex.quote(str(stale))+' "$2"\n');wrapper.chmod(0o755)
        wp=copy.deepcopy(p);wp['executable_path']=str(wrapper);wp['executable_sha256']=sha(wrapper.read_bytes())
        controls['stale_launch_nonce']=rejected(lambda:run.execute(c,wrapper,wp))
        wrapper.write_text('#!/bin/sh\nexit 0\n');wp['executable_sha256']=sha(wrapper.read_bytes())
        controls['missing_launch']=rejected(lambda:run.execute(c,wrapper,wp))
    # Restore the last control's case workspace with a successful fresh real run.
    actual=run.execute(c,exe,p);run.validate(c,actual);run.compare(by_id[c['id']],actual)
    run.execution_identity(exe,p,True)
    for name,h in artifact_hashes.items():assert sha((HERE/name).read_bytes())==h,('preserved',name)
    for name,h in seal['protected_artifact_sha256'].items():assert sha((ROOT/name).read_bytes())==h,('protected',name)
    for k,dp in profiles.items():run.compare(identities[k],run.execution_identity(Path(dp['executable_path']),dp,True),'preserved.executed.'+k)
    result=dict(delivery_commit=DELIVERY,artifact_sha256=artifact_hashes,protected_artifact_sha256=seal['protected_artifact_sha256'],nested_repositories=nested,delivered_executed_identities=identities,
        cases=len(cs),actions=sum(r['actions'] for r in receipts),events=sum(r['events'] for r in receipts),classifications=dict(collections.Counter(r['classification'] for r in receipts)),
        all_delivered_delta_chains_valid=True,complete_stable_profile_exact=True,fresh_profile=p,fresh_execution_identity=identity,fresh_receipts=receipts,
        controls=controls,control_count=len(controls),delivered_post_initial_mutation_differences=mutations,
        compiled_mutations_independently_recompiled=False,coverage_independently_repeated=False,
        preserved_all_sealed_artifacts_and_delivered_executed_identities=True,
        scope='Fresh unchanged source compilation and exact 302-case replay; 22 controls including positive identity. Delivered mutations and coverage inspected by artifacts and executed identities, not rerun. Only ignored build/case workspaces rewritten at original caller paths; no path/value normalization beyond delivered run.py.')
    native.save(OUT,result)
    print('PASS central P review;',len(controls),'controls;',len(cs),'cases;',result['actions'],'actions;',result['events'],'events;',OUT,flush=True)
if __name__=='__main__':main()
