#!/usr/bin/env python3
"""Review verified third-run Linux evidence without changing sources or gates."""
import argparse
import ast
from datetime import datetime, timezone
from decimal import Decimal
import hashlib
import json
from pathlib import Path
import re
import shutil
import types
import zipfile

p=argparse.ArgumentParser(description=__doc__)
p.add_argument('--root',type=Path,required=True)
p.add_argument('--packet',type=Path,required=True)
p.add_argument('--frozen',type=Path,required=True)
p.add_argument('--out',type=Path,required=True)
a=p.parse_args();a.out.mkdir(parents=True,exist_ok=False)
shutil.copyfile(__file__,a.out/'executed-review.py')
sha=lambda b:hashlib.sha256(b).hexdigest()
collection_path=a.packet/'verified-archive/collection-receipt.json'
collection=json.loads(collection_path.read_bytes())
assert collection['run']==41069548581
assert collection['candidate']=='a35104a611abb5c1d5c48dc4d1866f239ccd3ab9'
assert collection['archive_sha256']=='cdb2fca1cf14f4bd5c9c025c4aa2f9185bdf5c289a60e1d4a4456bff4831cf0a'
assert (a.packet/'artifact.zip').stat().st_size==collection['archive_bytes']
# The preceding verifier already hashed every archive member. Recheck the
# manifest and each selected member read here; do not repeat the 6.6 GB audit.
with zipfile.ZipFile(a.packet/'artifact.zip') as z:
    manifest_bytes=z.read('manifest.json')
    assert sha(manifest_bytes)==collection['manifest_sha256']
    files=json.loads(manifest_bytes)['files'];selected={}
    def read(name):
        data=z.read(name);identity=files[name]
        assert len(data)==identity['bytes'] and sha(data)==identity['sha256'],name
        selected[name]=identity
        return data
    def record(name):return json.loads(read(name))
    matrix=record('linux/receipt.json')
    assert matrix['candidate']==collection['candidate']
    counts={str(code):sum(v['exit']==code for v in matrix['stages'].values()) for code in [0,1,None]}
    assert counts=={'0':54,'1':13,'None':0} and len(matrix['stages'])==67
    audit=record('linux/source-audit/receipt.json')
    for name,digest in audit['rust_source'].items():
        assert sha((a.frozen/'rust/src'/name).read_bytes())==digest,name
    assert len(audit['rust_source'])==157
    cache={}
    guard_tree=ast.parse((a.frozen/'validation/release/cache_lifetime.py').read_text())
    guard=next(ast.literal_eval(node.value) for node in guard_tree.body if isinstance(node,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='GUARD' for t in node.targets))
    original=(a.frozen/'rust/src/predictor_ffi.rs').read_text()
    marker='fn finish(f: impl FnOnce() -> Result<(), String>) -> c_int {'
    old='match catch_unwind(AssertUnwindSafe(f)) {'
    assert original.count(marker)==original.count(old)==1
    guarded=original.replace(marker,guard+marker).replace(old,'match catch_unwind(AssertUnwindSafe(|| {\n        FOREIGN_THREAD_CACHE.with(|_| {});\n        f()\n    })) {')
    for mode in ['default','combined']:
        prefix=f'linux/c-concurrency-{mode}/';receipt=record(prefix+'receipt.json')
        identity=receipt['ownership_control']['identity']
        assert identity['changed']==['src/predictor_ffi.rs']
        assert identity['source']['src/predictor_ffi.rs']==sha(original.encode())
        assert identity['guard_source']==sha(guarded.encode())
        assert identity['patch']==sha(read(prefix+'library-owned-cache.patch'))
        failed=[r['label'] for r in receipt['records'] if r['exit']!=0]
        assert failed==['asan-execute']
        transcripts=[];products=[]
        for label,folder in [('asan-execute','asan'),('asan-cache-cleanup-execute','asan-cache-cleanup'),('asan-library-owned-execute','asan-library-owned')]:
            stdout=read(prefix+label+'.stdout');stderr=read(prefix+label+'.stderr')
            rows=re.findall(rb'AK_BEGIN (\d+) (\d+)\n(.*?)AK_END \1 \2\n',stdout,re.S)
            assert len(rows)==12 and stdout.count(b'AK_BEGIN')==stdout.count(b'AK_END')==12
            groups={(i,j):body for i,j,body in rows};assert len(groups)==12
            transcripts.append(groups)
            saved={Path(n).name:sha(read(n)) for n in files if n.startswith(prefix+folder+'/') and n.endswith('.tsv')}
            assert len(saved)==12;products.append(saved)
            if folder=='asan':assert b'SUMMARY: AddressSanitizer: 544 byte(s) leaked in 16 allocation(s).' in stderr
            else:assert not any(t in stderr for t in [b'ERROR: AddressSanitizer',b'runtime error:',b'WARNING: ThreadSanitizer'])
        assert transcripts[0]==transcripts[1]==transcripts[2]
        assert products[0]==products[1]==products[2]
        cache[mode]={'complete_workflows_each':12,'storage_files_each':12,'equal':True,
            'original_leak_bytes':544,'original_leak_allocations':16,
            'guard_source':identity['guard_source'],'patch':identity['patch'],
            'scope':receipt['scope']}
    m2=record('linux/m2-comparison/receipt.json')
    comparator=(a.frozen/'validation/fitting/compare.py').read_bytes()
    assert sha(comparator)==m2['comparator']
    module=types.ModuleType('central_m2_counterfactual')
    exec(compile(comparator,'frozen validation/fitting/compare.py','exec'),module.__dict__)
    def replay():
        rows=[]
        for entry in m2['records']:
            label=entry['name']
            if label=='callbacks':
                result=module.compare_callbacks(record('linux/m2-inputs/signed-reference/callbacks.json'),record('linux/m2-comparison/rust/callbacks.json'))
            elif '-shared-' in label:
                name,index=label.rsplit('-shared-',1)
                ref=record(f'linux/m2-inputs/signed-reference/{name}.json')
                actual=record(f'linux/m2-comparison/rust/shared_{name}.json')[int(index)]
                row=ref['cycles'][int(index)] if 'cycles' in ref else ref
                result=module.compare_shared(row,actual,ref['mode'])
            else:
                assert label.endswith('-timing');name=label[:-7]
                ref=record(f'linux/m2-inputs/signed-reference/{name}.json')
                actual=record(f'linux/m2-comparison/rust/{name}.json')
                epoch=next(line.split()[1] for line in actual['input']['par']['text'].splitlines() if line.startswith('PEPOCH '))
                result=module.compare_timing(ref,actual,epoch)
            rows.append({'name':label,'failures':result.errors})
        return rows
    unchanged=replay()
    assert unchanged==[{'name':r['name'],'failures':r['failures']} for r in m2['records']]
    assert sum(len(r['failures']) for r in unchanged)==15
    module.NONFINITE.add('-NaN')
    adapted=replay()
    assert sum(len(r['failures']) for r in adapted)==4
    controls=[]
    for expected,actual,accepted in [('-NaN','-NaN',True),('NaN','NaN',True),('-NaN','NaN',False),('NaN','-NaN',False),('-NaN','0',False),('-NaN','+Infinity',False),('-NaN','nan',False)]:
        c=module.Comparison();c.number(expected,actual,Decimal(0),'control','control')
        assert (not c.errors)==accepted
        controls.append({'expected':expected,'actual':actual,'accepted':accepted})
    (a.out/'m2-counterfactual.json').write_text(json.dumps({'scope':'In-memory NONFINITE set extension only; no committed comparator or gate changes','original':unchanged,'adapted':adapted,'controls':controls},indent=2)+'\n')
    pre='linux/native-corpora/staging/validation/tim_io/build/cases/deletion_boundary/'
    actions=[[bytes.fromhex(v[1:]).decode() for v in line.split('\t')] for line in read(pre+'actions.tsv').decode().splitlines()]
    events=[json.loads(line) for line in read(pre+'events.jsonl').splitlines()]
    native_header=(a.frozen/'tempo2/tempo2.h')
    if not native_header.exists():native_header=a.root/'tempo2/tempo2.h'
    assert 'char deleteFileName[100];' in native_header.read_text()
    assert 'if(k=="deleteFileName")strcpy(p.deleteFileName,v.c_str());' in (a.frozen/'validation/tim_io/probe.cc').read_text()
    assert actions[0][:3]==['set','0','deleteFileName'] and len(actions[0][3].encode())==152
    assert events[-1]['phase']=='begin' and events[-1]['action']==0 and events[-1]['op']=='set'
    assert record(pre+'receipt.json')['returncode']==-6
    assert b'*** buffer overflow detected ***' in read('linux/native-corpora/tim_io-record.stderr')
    corpus=record('linux/corpus/receipt.json')
    timing=record('linux/timing/receipt.json')
    predictors=record('linux/predictors/receipt.json')
    receipts={label:record(f'linux/{label}/receipt.json') for label in ['provider-abi','package-audit','fits','boundaries','ddh','event-diagnostic','output']}
    summary={'scope':'Central review of selected immutable third Linux products; no milestone acceptance',
        'candidate':matrix['candidate'],'run':collection['run'],'matrix':{'stages':67,'passed':54,'failed':matrix['failed'],'blocked':matrix['blocked']},
        'source_files_bound':157,'cache':cache,'m2':{'original_failures':15,'signed_nonfinite_counterfactual_failures':4,'negative_and_positive_controls':len(controls)},
        'tim_io':{'path_bytes':152,'destination_bytes':100,'last_event':{k:events[-1][k] for k in ['phase','action','op']},'exit':-6,'localization':'Fixture caller strcpy overflows before readTimfile; confirm corrected short-path execution on Linux'},
        'corpus':{k:corpus[k] for k in ['failed','blocked','source_mismatches']},
        'timing':{'cases':timing['cases'],'failures':timing['failures']},
        'predictors':{'cases':len(predictors['cases']),'checks':sum(r['checks'] for r in predictors['cases']),'failures':sum(len(r['failures']) for r in predictors['cases'])},
        'fits_failures':receipts['fits']['failures'],'boundary_failures':receipts['boundaries']['failures'],
        'ddh':{k:receipts['ddh'][k] for k in ['fidelity_failures','historical_ideal_failures']},
        'package_checked_files':receipts['package-audit']['checked_files'],
        'archive_collection_receipt_sha256':sha(collection_path.read_bytes()),'selected_files_checked':len(selected),
        'recipe_sha256':sha(Path(__file__).read_bytes()),'completed_utc':datetime.now(timezone.utc).isoformat()}
(a.out/'selected-inputs.json').write_text(json.dumps(selected,indent=2)+'\n')
(a.out/'receipt.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(summary,indent=2))
print('receipt_sha256',sha((a.out/'receipt.json').read_bytes()))
