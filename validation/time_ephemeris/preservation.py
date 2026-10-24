"""Exact release-anchored preservation; decision 0050 removes the provisional 0048 exception."""
import argparse
import json
from pathlib import Path
import subprocess
import sys
from profile import HERE, ROOT, sha, save, ENV
OUT=HERE/'outputs'
MANIFEST=OUT/'preservation-frozen.json'
RECEIPT=OUT/'preservation-final.json'
RELEASE='8e56df0'
ROUTED='validation/observation_storage_port/run.py'
BEFORE='7d6424654a666d3852255bd9264ba0fbb88ba3c6e57f7bf717da4087a6b10529'
DIRECTORIES=['binary','clocks','clocks_port','coverage','fitting','jpl','jpl_port',
 'numeric_toolkit','observation_storage_port','par_specialized','parameter_storage_port',
 'parameters','precision','preprocessing','pulsar_lifecycle','reference','tim_io']
SKIP={'.git','build','target','.cargo-home','__pycache__'}

def paths():
    members=[]
    for name in DIRECTORIES:
        for p in (ROOT/'validation'/name).rglob('*'):
            if p.is_file() and not any(part in SKIP for part in p.relative_to(ROOT).parts) and p.name!='.DS_Store':members.append(p)
    members += [ROOT/'validation/reference/build/source/config.h',ROOT/'validation/reference/build/libtempo2-reference.a']
    for name in ['reference','reference-initial']:
        members += [p for p in (OUT/name).rglob('*') if p.is_file()]
    return sorted(str(p.relative_to(ROOT)) for p in members)

def actual():return {n:sha((ROOT/n).read_bytes()) for n in paths()}

def enforce(frozen,current):
    assert set(frozen)==set(current),'protected membership changed'
    transitions=[]
    for n,h in frozen.items():
        if current[n]==h:continue
        raise AssertionError('protected drift '+n+' expected '+h+' actual '+current[n])
    return transitions

def check(manifest):
    # Preserve the original manifest and checker identity as historical evidence.
    # This authored pin binds the corrected exact checker required by decision 0050.
    assert sha(Path(__file__).read_bytes())==(HERE/'preservation-checker.sha256').read_text().strip(),'preservation checker changed'
    transitions=enforce(manifest['files'],actual())
    for directory,pin in manifest['repositories'].items():
        head=subprocess.check_output(['git','-C',str(ROOT/directory),'rev-parse','HEAD'],text=True).strip()
        status=subprocess.check_output(['git','-C',str(ROOT/directory),'status','--porcelain'],text=True)
        assert head==pin and not status,(directory,head,status)
    return transitions

def freeze():
    assert not MANIFEST.exists(),'preservation freeze already exists; never overwrite it'
    files=actual();tracked=0
    for n in files:
        p=subprocess.run(['git','-C',str(ROOT),'show',RELEASE+':'+n],capture_output=True)
        if p.returncode==0:
            files[n]=sha(p.stdout);tracked+=1
    assert files[ROUTED]==BEFORE,'R release identity'
    repositories={directory:subprocess.check_output(['git','-C',str(ROOT/directory),'rev-parse','HEAD'],text=True).strip() for directory in ['tempo2','validation/reference']}
    assert repositories['tempo2']=='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
    assert repositories['validation/reference']=='7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572'
    data=dict(release=RELEASE,files=files,release_tracked_members=tracked,repositories=repositories,
              checker_sha256=sha(Path(__file__).read_bytes()),
              scope='Added after decision 0048. Tracked members anchored to release 8e56df0; pre-existing local generated members frozen at this explicit check, not retrospectively certified. Original V oracles remain unchanged.')
    transitions=check(data);save(MANIFEST,data)
    print('Frozen',len(files),'protected files;',tracked,'release-tracked; transitions',len(transitions),flush=True)

def controls(manifest):
    current=actual();enforce(manifest['files'],current);results=[]
    for name,path in [('further R runner drift',ROUTED),('unrelated protected drift','validation/numeric_toolkit/probe.cc')]:
        changed=dict(current);changed[path]='0'*64
        try:enforce(manifest['files'],changed)
        except AssertionError as e:results.append(dict(control=name,rejected=True,reason=str(e)))
        else:raise AssertionError('accepted '+name)
    changed=dict(current);changed.pop(ROUTED)
    try:enforce(manifest['files'],changed)
    except AssertionError as e:results.append(dict(control='R membership removal',rejected=True,reason=str(e)))
    else:raise AssertionError('accepted membership removal')
    return results

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--freeze',action='store_true');parser.add_argument('--check-only',action='store_true');args=parser.parse_args()
    if args.freeze:freeze();sys.exit(0)
    RECEIPT.unlink(missing_ok=True)
    manifest=json.loads(MANIFEST.read_text());before=check(manifest)
    if not args.check_only:
        # Ordinary replay never records. Controls remove the stale execution receipt,
        # so the final complete replay follows them.
        for script in ['verify.py','run.py']:
            subprocess.run([sys.executable,str(HERE/script)],env=ENV,check=True)
    after=check(manifest)
    save(RECEIPT,dict(passed=True,check_only=args.check_only,members=len(manifest['files']),
         checker_sha256=sha(Path(__file__).read_bytes()),manifest_sha256=sha(MANIFEST.read_bytes()),
         transitions_before=before,transitions_after=after,controls=controls(manifest),
         execution_receipt_sha256=sha((OUT/'last_execution.json').read_bytes()) if (OUT/'last_execution.json').exists() else None))
    print('PASS preservation',len(manifest['files']),'members;',len(after),'exceptions; 3 rejecting controls')
