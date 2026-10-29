#!/usr/bin/env python3
"""Join requirement contracts to measured bodies/edges without inferring acceptance."""
import argparse
import csv
import json
from pathlib import Path
import platform
from common import ROOT, Run, sha

GATES={
    'parameter-input':['parameters','par_specialized'], 'tim-input':['tim_io'],
    'preprocessing':['preprocessing'],
    'owned-storage':['parameter_storage','parameter-storage-unwritten','observation_storage','pulsar_lifecycle','lifecycle-corrected'],
    'numeric-utilities':['boundaries','external-default'], 'clock-chain':['clocks'], 'jpl':['jpl'],
    'time-ephemeris':['time_ephemeris'], 'terrestrial':['terrestrial_resources'],
    'observer':['observer_coordinates','observer-corrected'],
    'planetary':['planetary_ephemeris','planetary-random'],
    'propagation':['propagation','propagation-corrected','propagation-helpers'],
    'global-timing':['timing','workers'], 'binary-models':['timing','ddh','external-default'],
    'fit-orchestration':['fits','m2-comparison'], 'fit-callbacks':['fits','external-default'],
    'constraints':['fits','external-default'], 'solvers':['fits','m2-comparison','paired-combined'],
    'correlated-noise':['fits','external-default'], 'toolkit':['external-default','rust-suite'],
    'spectra':['external-default','contraction'], 'gravitational-waves':['external-default','timing'],
    'predictor-leaves':['c-default','c-combined','c-concurrency-default','c-concurrency-combined'],
    'predictor-generation':['predictors','external-default'],
    'providers':['provider-abi',*['paired-'+p for p in ['disabled','blas','lapack','calceph','combined','qr-default']]],
    'core-output':['output','external-default'],
}
STATIC={'tai2tt.C':'Entire legacy body is #if 0; active clkcorr chain owns the operation',
        'utc2tai.C':'Entire legacy body is #if 0; active clkcorr chain owns the operation',
        'storePrecision.C':'recordPrecision has no active body at the pin; explicit Rust no-op',
        'enum_str.C':'Generated constant namespace: canonical names and sentinels, checked by installed consumer',
        't2toolkit_global.C':'Native global data definitions, represented by the owned ToolkitRng state'}

def measured_rows(data, stage):
    """Retain actual gcov records, including unhit and compiler-generated edges."""
    functions=[];branches=[]
    def gcov_file(file):
        name=Path(file['file']).name
        functions.extend(dict(file=name,name=f['demangled_name'],measurement=f) for f in file['functions'])
        for line in file['lines']:
            for index,branch in enumerate(line.get('branches',[])):
                branches.append(dict(file=name,line=line['line_number'],index=index,**branch))
    if 'units' in data:
        for unit in data['units'].values():
            for file in unit['data']['files']:gcov_file(file)
    elif isinstance(data.get('files'),dict):
        for name,file in data['files'].items():
            functions.extend(dict(file=name,name=n,measurement=f) for n,f in file['functions'].items())
            for line,entry in file['lines'].items():
                branches.extend(dict(file=name,line=int(line),index=i,**b) for i,b in enumerate(entry['branches']))
    elif 'raw_selected' in data:
        # V retains the measured function/line objects and an explicit gap list.
        for entry in data['summary']:
            functions.append(dict(file=entry['file'],name=entry['routine'],measurement=entry))
        gaps={(g['file'],g['line'],g['index']):g for g in data['gaps'] if g['kind']=='edge'}
        for file in data['raw_selected']:
            name=Path(file['file']).name
            for line in file['lines']:
                for index,branch in enumerate(line.get('branches',[])):
                    row=dict(file=name,line=line['line_number'],index=index,**branch)
                    if gap:=gaps.get((name,line['line_number'],index)):
                        row.update(classification=gap['classification'],reason=gap['reason'])
                    branches.append(row)
    else:
        for row in data.get('functions',data.get('routines',[])):
            name=row.get('name',row.get('demangled_name'))
            file=row.get('file',row.get('source','readEphemeris.C' if stage=='planetary_ephemeris' else None))
            assert file and name,(stage,row)
            functions.append(dict(file=Path(file).name,name=name,measurement=row))
        for row in data.get('branches',[]):
            branches.append(dict(row,file=row.get('file','readEphemeris.C' if stage=='planetary_ephemeris' else None)))
    return functions,branches


def main(args):
    run=Run(args.out)
    requirements=list(csv.DictReader((ROOT/'validation/release/requirements.tsv').open(),delimiter='\t'))
    correspondence=list(csv.DictReader((ROOT/'validation/release/source_correspondence.tsv').open(),delimiter='\t'))
    sources={r['source'] for r in correspondence}
    assert {s.strip() for r in requirements for s in r['native_sources'].split(';')}==sources
    audit=json.loads(args.audit.read_text())
    assert audit['status']=='pass' and audit['baseline_status']=='available'
    # Reuse applies to identical scientific bodies, not automatically to another
    # compiler, ABI or platform. Changed bodies explicitly invalidate this reuse.
    changed=audit['changed_rust_bodies']
    proofs={};functions=[];branches=[];native={};headers=[];header_branches=[]
    for selection in args.native:
        stage,value=selection.split('=',1);directory=Path(value).resolve()
        receipt=directory/'receipt.json';record=json.loads(receipt.read_text())
        assert stage not in native,'Select one explicit native run per stage'
        native[stage]={'receipt':str(receipt),'sha256':sha(receipt),'result':record['stages'][stage]}
        base=directory/'staging/validation'/stage
        candidates=[base/'coverage.json',base/'outputs/coverage.json',base/'line_coverage.json']
        for path in candidates:
            if not path.exists():continue
            data=json.loads(path.read_text());key=str(path)
            proofs[key]={'sha256':sha(path),'stage':stage,'kind':'declared-case inventory' if stage=='jpl' else 'measured native reachability'}
            if stage=='jpl':
                for name,row in data['functions'].items():
                    functions.append(dict(file='jpleph.c',name=name,declared=row,evidence=key))
            else:
                fs,bs=measured_rows(data,stage)
                functions.extend(dict(f,evidence=key) for f in fs if f['file'] in sources)
                headers.extend(dict(f,evidence=key) for f in fs if f['file'] in ['read_fortran.h','read_fortran2.h','t2fit_nestlike.h','TKlongdouble.h'])
                branches.extend(dict(b,evidence=key) for b in bs if b['file'] in sources)
                header_branches.extend(dict(b,evidence=key) for b in bs if b['file'] in ['read_fortran.h','read_fortran2.h','t2fit_nestlike.h','TKlongdouble.h'])
        for name in ['inventory.json','outputs/frozen/source_map.json','branch_coverage.json',
                     'coverage_classification.json','UNRUN.md','SOURCE_MAP.md']:
            path=base/name
            if path.exists():proofs[str(path)]={'sha256':sha(path),'stage':stage,'kind':'source decisions and dispositions'}
    comparisons={}
    if args.matrix:
        matrix=json.loads(args.matrix.read_text())
        comparisons.update({stage:dict(row,receipt=str(args.matrix.resolve()),sha256=sha(args.matrix))
                            for stage,row in matrix['stages'].items()})
    for selection in args.comparison:
        stage,value=selection.split('=',1);receipt=Path(value)
        record=json.loads(receipt.read_text())
        assert stage not in comparisons,'Select one explicit comparator result per stage: '+stage
        if stage in record.get('stages',{}):row={'exit':record['stages'][stage]}
        else:row={'blocked':record['blocked'][stage]}
        comparisons[stage]={**row,'receipt':str(receipt.resolve()),'sha256':sha(receipt)}
    for selection in args.gate:
        name,value=selection.split('=',1);path=Path(value).resolve();record=json.loads(path.read_text())
        assert name not in comparisons,name
        comparisons[name]={'status':record.get('status'),'exit':record.get('returncode'),
                           'receipt':str(path),'sha256':sha(path)}
    maps={}
    for path in sorted([*ROOT.joinpath('rust').glob('*SOURCE_MAP.md'),*ROOT.joinpath('rust').glob('*_API.md'),
                        ROOT/'rust/TIMING_MODELS_REPORT.md',ROOT/'rust/INPUT_TO_BAT_REPORT.md']):
        # Preserve the authored routine/branch rows verbatim with their locations.
        # Their source-inspected and representative-only limits are not upgraded.
        maps[str(path.relative_to(ROOT))]={'sha256':sha(path),'rows':[
            {'line':i,'text':line} for i,line in enumerate(path.read_text().splitlines(),1) if line.startswith('|')]}
    rows=[]
    assert set(GATES)=={r['requirement'] for r in requirements}
    for requirement in requirements:
        selected={s.strip() for s in requirement['native_sources'].split(';')}
        fs=[f for f in functions if f['file'] in selected];bs=[b for b in branches if b['file'] in selected]
        authored=[dict(map=name,sha256=entry['sha256'],**row)
                  for name,entry in maps.items() for row in entry['rows']
                  if any(source in row['text'] for source in selected)]
        missing=sorted(selected-{f['file'] for f in fs if 'measurement' in f})
        unhit=[b for b in bs if b.get('count',0)==0]
        rows.append(dict(requirement,functions=fs,branches=bs,authored_correspondence=authored,source_authorities={
            r['source']:r['body_authority'] for r in correspondence if r['source'] in selected},
            numerical_evidence={name:comparisons.get(name,{'status':'not supplied'}) for name in GATES[requirement['requirement']]},
            static_dispositions={s:STATIC[s] for s in selected if s in STATIC},
            gaps={'no_fresh_measured_function_rows':missing,'unhit_or_gap_records':len(unhit),
                  'branch_acceptance':'Open: combine per-edge disposition with independent numerical witnesses; an entered body or passing family does not prove every branch'},
            reuse='Invalidated by changed bodies: '+','.join(changed) if changed else
                  'Scientific Rust bodies unchanged at the audited baseline; authored correspondence may be reused. Host execution, numerical gates and unhit branches remain separate.'))
    run.save({'status':'pass','qualification':'incomplete','requirements':rows,'native':native,
              'comparisons':comparisons,'proofs':proofs,'authored_maps':maps,
              'header_functions':headers,'header_branches':header_branches,
              'audit':{'path':str(args.audit.resolve()),'sha256':sha(args.audit),'changed_bodies':changed},
              'contracts':sha(ROOT/'validation/release/requirements.tsv'),
              'meaning':'Successful accounting, not scientific acceptance or exhaustive branch closure. All failed, unhit, source-inspected and platform-specific obligations remain visible.',
              'remaining':['Unhit defined branches and bodies without fresh per-function evidence',
                           'Numerical comparison failures retained in their exact receipts',
                           *(['Actual selected Linux compiler/ABI/provider and worker execution'] if platform.system()!='Linux' else []),
                           'Central acceptance of corrections and scientific differences']})
    summary=['# AL requirement evidence ledger','',
             'This joins actual native counters to authored function/branch contracts. It does not turn reachability into numerical agreement or close remaining scientific gates.','',
             '| Requirement | Function records | Edge/gap records | Authored correspondence rows | Sources without fresh function records |',
             '|---|---:|---:|---:|---|']
    for row in rows:summary.append('| '+row['requirement']+' | '+str(len(row['functions']))+' | '+str(len(row['branches']))+' | '+str(len(row['authored_correspondence']))+' | '+', '.join(row['gaps']['no_fresh_measured_function_rows'])+' |')
    summary+=['','The receipt retains every function/edge record, its evidence path and hash, the existing source dispositions, and every authored map row with its line number. Missing records are explicit gaps, including inactive/no-op sources that need static disposition. No percentage is used as an acceptance threshold.']
    (run.out/'ledger.md').write_text('\n'.join(summary)+'\n')
    print(len(rows),'requirements; evidence ledger written; qualification incomplete')


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out',type=Path,required=True);parser.add_argument('--audit',type=Path,required=True)
    parser.add_argument('--native',action='append',default=[],metavar='STAGE=RUN_DIRECTORY')
    parser.add_argument('--comparison',action='append',default=[],metavar='STAGE=RECEIPT')
    parser.add_argument('--matrix',type=Path)
    parser.add_argument('--gate',action='append',default=[],metavar='NAME=RECEIPT')
    main(parser.parse_args())
