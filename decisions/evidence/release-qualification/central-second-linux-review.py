#!/usr/bin/env python3
"""Verify and summarize the second AL Linux artifact without changing any gate."""
import argparse
from collections import Counter
from decimal import Decimal
import hashlib
import json
from pathlib import Path
import re

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('packet', type=Path)
parser.add_argument('--out', type=Path, required=True)
a = parser.parse_args()
packet = a.packet.resolve()
evidence = packet / 'artifact'
linux = evidence / 'linux'


def sha(path):
    with path.open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()


def read(relative):
    return json.loads((linux / relative).read_text())


run = json.loads((packet / 'run.json').read_text())
assert run['id'] == 41056090173
assert run['head_sha'] == '4accd26e2d51b3dc805d9a1ae0a2be7195e171ea'
assert run['status'] == 'completed' and run['conclusion'] == 'failure'
artifacts = json.loads((packet / 'artifacts.json').read_text())['artifacts']
assert len(artifacts) == 1 and artifacts[0]['id'] == 12979305748
artifact = artifacts[0]
assert artifact['workflow_run']['head_sha'] == run['head_sha']
assert artifact['digest'] == 'sha256:' + sha(packet / 'artifact.zip')
assert artifact['size_in_bytes'] == (packet / 'artifact.zip').stat().st_size
files = json.loads((evidence / 'manifest.json').read_text())['files']
for name, identity in files.items():
    path = (evidence / name).resolve()
    assert path.is_relative_to(evidence) and path.is_file(), name
    assert path.stat().st_size == identity['bytes'] and sha(path) == identity['sha256'], name
assert {str(p.relative_to(evidence)) for p in evidence.rglob('*') if p.is_file()} == set(files) | {'manifest.json'}
matrix = read('receipt.json')
assert matrix['candidate'] == run['head_sha']
stages = matrix['stages']
failed = [k for k,v in stages.items() if v['exit'] not in (0,None)]
blocked = [k for k,v in stages.items() if v['exit'] is None]
assert failed == matrix['failed'] and blocked == matrix['blocked']
names = ['provider-abi','rust-suite','c-default','c-combined','c-concurrency-default',
         'c-concurrency-combined','lomb-diagnostic','event-diagnostic','native-corpora',
         'workers','performance','m2-comparison','fits','boundaries','ddh','timing',
         'predictors','output','package-audit','source-audit','evidence-closure']
receipts = {name:read(name+'/receipt.json') for name in names}
events = []
for row in receipts['event-diagnostic']['records']:
    record = {k:v for k,v in row.items() if k not in ['measurements','rust_errors']}
    record['rust_errors'] = row.get('rust_errors',[])
    record['maximum_residual_difference_s'] = str(max(
        (abs(Decimal(m['difference'])) for m in row.get('measurements',[])
         if m['field'].endswith(('residual_s','residualtn_s'))), default=Decimal(0)))
    events.append(record)
caches = {}
for name in ['c-concurrency-default','c-concurrency-combined']:
    texts = [(linux/name/f).read_bytes() for f in ['asan-execute.stdout','asan-cache-cleanup-execute.stdout']]
    blocks = [{(m[0],m[1]):m[2] for m in re.findall(rb'AK_BEGIN (\d+) (\d+)\n(.*?)AK_END \1 \2\n', text, re.S)} for text in texts]
    common = blocks[0].keys() & blocks[1].keys()
    caches[name] = {'control':receipts[name]['cache_control'],
        'stdout_bytes':[len(t) for t in texts],
        'begin_markers':[t.count(b'AK_BEGIN') for t in texts],
        'end_markers':[t.count(b'AK_END') for t in texts],
        'complete_blocks':[len(b) for b in blocks],
        'common_complete_blocks_equal':all(blocks[0][key] == blocks[1][key] for key in common),
        'failed_commands':[(c['label'],c['exit']) for c in receipts[name]['records'] if c['exit']]}
predictors = receipts['predictors']['cases']
worker_qualification = read('workers/qualification/receipt.json')
worker_performance = read('workers/performance/receipt.json')
worker_controls = read('workers/timed-control/receipt.json')
source = receipts['source-audit']
assert source['changed_rust_bodies'] == ['rust/src/fitting_tests.rs','rust/src/predictor_tests.rs']
report = {
    'scope':'Retained second Linux evidence review; no engine rerun or milestone acceptance',
    'run':{k:run[k] for k in ['id','head_sha','status','conclusion','html_url','created_at','updated_at']},
    'archive_sha256':sha(packet/'artifact.zip'), 'manifest_sha256':sha(evidence/'manifest.json'),
    'verified_files':len(files),'verified_bytes':sum(v['bytes'] for v in files.values()),
    'stage_counts':dict(Counter('blocked' if r['exit'] is None else 'pass' if r['exit']==0 else 'fail' for r in stages.values())),
    'failed':failed,'blocked':blocked,
    'native_corpus_failure':receipts['native-corpora']['failed_command'],
    'native_corpus_error':(linux/'native-corpora/history-fetch-7946aeee6e7e105fc276b311f1c36a4288767d0d.stderr').read_text(),
    'native_corpora_executed':len(receipts['native-corpora'].get('stages',{})),
    'provider_measurement':receipts['provider-abi']['measurement'],
    'c_interfaces':{name:{'long_double_profile':receipts[name]['long_double_profile'],
        'symbols':len(receipts[name]['symbols']),'profiles':len(receipts[name]['profiles']),
        'rejected_controls':len(receipts[name]['rejected_controls'])} for name in ['c-default','c-combined']},
    'rust_suite':receipts['rust-suite']['results'],
    'lomb_counterfactual':receipts['lomb-diagnostic']['comparisons'],
    'event_invariant_failures':receipts['event-diagnostic']['invariant_failures'],
    'events':events,'cache_diagnostics':caches,
    'm2':{'records':len(receipts['m2-comparison']['records']),
        'failures':receipts['m2-comparison']['failures'],
        'failed_records':[{k:r[k] for k in ['name','failures']} for r in receipts['m2-comparison']['records'] if r['failures']]},
    'fits_failures':receipts['fits']['failures'],'boundary_failures':receipts['boundaries']['failures'],
    'ddh':{'historical_ideal_failures':receipts['ddh']['historical_ideal_failures'],
        'fidelity':{r['clock']:r['fidelity_failures'] for r in receipts['ddh']['records']}},
    'predictors':{'cases':len(predictors),'checks':sum(r['checks'] for r in predictors),
        'failures':sum(len(r['failures']) for r in predictors),
        'by_field':dict(Counter('serialization' if f['field'].startswith('serialized') else 'log10rms' if 'log10rms' in f['field'] else f['field'] for r in predictors for f in r['failures']))},
    'output':receipts['output']['summary'],
    'workers':{'processes':len(worker_qualification['records']),
        'comparisons':len(worker_qualification['results']),'failures':worker_qualification['failures'],
        'checks':sum(r['checks'] for r in worker_qualification['results']),
        'timing_samples':len(worker_performance['records']),'product_controls':len(worker_controls['records'])},
    'package':{k:receipts['package-audit'][k] for k in ['checked_files','mismatches']},
    'source':{'unchanged':len(source['unchanged_rust_bodies']),
        'changed':source['changed_rust_bodies'],'added':source['added_rust_bodies'],'removed':source['removed_rust_bodies']},
    'closure_native_records':len(receipts['evidence-closure']['native']),
    'job_log_sha256':sha(packet/'job.complete.raw.log'),
    'receipt_sha256':{name:sha(linux/name/'receipt.json') for name in names},
    'recipe_sha256':sha(Path(__file__)),
}
with a.out.open('x') as f:
    f.write(json.dumps(report,indent=2)+'\n')
print(json.dumps({k:report[k] for k in ['verified_files','verified_bytes','stage_counts','workers','source']}))
print('review_sha256',sha(a.out))
