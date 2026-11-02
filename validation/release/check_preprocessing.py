#!/usr/bin/env python3
"""AL P replay: retain raw comparisons and separately verify bounded 0127 targets."""
import argparse,gzip,hashlib,json,os,re,shutil,subprocess,sys
from pathlib import Path
from discrete_contract import simul_targets, resolve_simul
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];P=ROOT/'validation/preprocessing';BUILD=HERE/'build/preprocessing'
def sha(b):return hashlib.sha256(b).hexdigest()
def main():
    ap=argparse.ArgumentParser();ap.add_argument('--release',action='store_true');ap.add_argument('--case',action='append');ap.add_argument('--generate',action='store_true');ap.add_argument('--candidate',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);ap.add_argument('--reference-root',type=Path,required=True);a=ap.parse_args()
    global ROOT,P,BUILD
    ROOT=a.candidate.resolve();P=a.reference_root.resolve();BUILD=a.out.resolve()
    assert not a.generate, 'Release qualification does not regenerate production fields'
    assert not BUILD.exists(), 'Use a fresh evidence directory'
    runtime=ROOT/'validation/preprocessing/build/runtime'
    runtime.parent.mkdir(parents=True,exist_ok=True)
    if not runtime.exists():runtime.symlink_to(P/'build/runtime',target_is_directory=True)
    assert runtime.resolve()==(P/'build/runtime').resolve()
    BUILD.mkdir(parents=True,exist_ok=True)
    profile=json.loads((P/'profile.json').read_text());inc=next(x[2:] for x in profile['command'] if x.startswith('-I'))
    initial=json.loads(gzip.decompress((P/'initial.json.gz').read_bytes()))
    if a.generate:
        fields=sorted(k.removeprefix('p0.') for k in initial['state'] if k.startswith('p0.') and not k.startswith(('p0.param.','p0.obs')) and k!='p0.caller_capacity')
        source='// Generated from the accepted P complete pulsar-field membership.\nfn fixed(p:&mut SourcePulsar,root:&str,s:&mut Map,load:bool){\n'
        source+=''.join('field(&mut p.'+f+',&format!("{root}'+f+'"),s,load);\n' for f in fields)+'}\n'
        (ROOT/'rust/src/preprocessing_corpus_fields.rs').write_text(source);return
    layout=initial['abi']['layout'];ranges={}
    widths={'longdouble':16,'double':8,'float':4}
    types=json.loads((P/'TYPES.json').read_text())
    for kind in ['observation','pulsar']:
        for full,span in layout.items():
            if not full.startswith(kind+'.'):continue
            name=full.split('.',1)[1]
            for declaration in types[kind]:
                match=re.match(r'(longdouble|double|float)\s+',declaration)
                if match and re.search(r'\b'+re.escape(name)+r'\b',declaration):
                    ranges[full]=[span['offset'],span['bytes'],widths[match[1]]];break
    (BUILD/'floating-layout.json').write_text(json.dumps(ranges)+'\n')
    (BUILD/'initial.json').write_text(json.dumps(initial)+'\n')
    cases=json.loads(gzip.decompress((P/'observations.json.gz').read_bytes()));selected=[];dispositions=[]
    for c in cases:
        if a.case and c['id'] not in a.case:continue
        if any(x[0]=='plugins' for x in c['actions']):
            dispositions.append({'case':c['id'],'disposition':'native external callback/plugin ABI outside AC release; Rust explicitly rejects source selection plugin requests'})
            continue
        work=BUILD/c['id'];work.mkdir(exist_ok=True)
        for name,raw in c['files'].items():
            target=work/name;target.parent.mkdir(exist_ok=True,parents=True);target.write_bytes(bytes.fromhex(raw))
        # Retain the original native source directory in diagnostics. Resource files
        # are the accepted read-only P caller fixtures; authored inputs use AC paths.
        c=json.loads(json.dumps(c).replace('{case}',str(work)).replace('{build}',str(P/'build')).replace('{source}',inc))
        c['workdir']=str(work);selected.append(c)
    (BUILD/'cases.json').write_text(json.dumps(selected)+'\n')
    env={**os.environ,'AC_PREPROCESS_CORPUS':str(BUILD)}
    cmd=['cargo','test','--manifest-path',str(ROOT/'rust/Cargo.toml'),'--locked','--offline',*(['--release'] if a.release else []),'--lib','preprocessing_corpus_tests::p_corpus','--','--ignored','--exact','--nocapture','--test-threads=1']
    run=subprocess.run(cmd,env=env)
    (BUILD/('receipt-'+('release' if a.release else 'debug')+'.json')).write_text(json.dumps({'reference_sha256':sha((P/'observations.json.gz').read_bytes()),'initial_sha256':sha((P/'initial.json.gz').read_bytes()),'profile_sha256':sha((P/'profile.json').read_bytes()),'types_sha256':sha((P/'TYPES.json').read_bytes()),'runner_sha256':sha(Path(__file__).read_bytes()),'cases':len(selected),'dispositions':dispositions,'returncode':run.returncode},indent=2)+'\n')
    # Native histories and their failed comparison remain intact. Only the
    # separately derived 0127 histories use a second expected-state file.
    targets,deltas,proofs=simul_targets([c for c in selected if c['id'].startswith('simul')],initial)
    if not proofs:raise SystemExit(run.returncode)
    contract=BUILD/'rug128-contract';contract.mkdir()
    for name in ['initial.json','floating-layout.json']:shutil.copyfile(BUILD/name,contract/name)
    for case in targets:
        work=contract/case['id'];work.mkdir();case['workdir']=str(work)
        for name,raw in case['files'].items():(work/name).write_bytes(bytes.fromhex(raw))
    (contract/'cases.json').write_text(json.dumps(targets)+'\n')
    (contract/'expectations.json').write_text(json.dumps({'authority':'0127','proofs':proofs,'original_differences':deltas},indent=2)+'\n')
    target_run=subprocess.run(cmd,env={**env,'AC_PREPROCESS_CORPUS':str(contract)})
    failures=json.loads((BUILD/'differences.json').read_text())
    remaining=resolve_simul(failures,deltas)
    diagnostics=json.loads((BUILD/'diagnostic-differences.json').read_text())
    controls=None
    if target_run.returncode==0 and any(c['id']=='simul_123' for c in targets):
        controls=subprocess.run([sys.executable,HERE/'discrete_contract_controls.py',
            '--corpus',contract,'--candidate',ROOT,'--out',BUILD/'contract-controls',
            *(['--release'] if a.release else [])],env=env).returncode
    failed=bool(remaining or diagnostics or target_run.returncode or controls)
    (BUILD/'receipt.json').write_text(json.dumps({'status':'failure' if failed else 'pass',
        'authority':'0127','original_returncode':run.returncode,'original_differences':len(failures),
        'contract_returncode':target_run.returncode,'contract_histories':len(proofs),
        'contract_controls_returncode':controls,
        'checked_simul_histories':len(targets),'resolved_original_records':len(deltas),
        'remaining_failures':remaining,'diagnostic_failures':diagnostics,
        'inputs':{str(p):sha(p.read_bytes()) for p in [P/'observations.json.gz',P/'initial.json.gz',
            ROOT/'rust/src/selection.rs',ROOT/'rust/src/preprocessing_corpus_tests.rs',
            BUILD/'differences.json',contract/'cases.json',contract/'expectations.json',
            Path(__file__),HERE/'discrete_contract.py']},
        'scope':'Original P comparison retained; complete state/diagnostics replayed against independently derived bounded Rug128 targets; other gates unchanged'},indent=2)+'\n')
    raise SystemExit(failed)
if __name__=='__main__':main()
