#!/usr/bin/env python3
"""Read-only checks of P's committed scope and coverage accounting."""
import collections,gzip,hashlib,json,subprocess
from pathlib import Path
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust');HERE=ROOT/'validation/preprocessing'
sha=lambda b:hashlib.sha256(b).hexdigest()
load=lambda p:json.loads(gzip.decompress(p.read_bytes()) if p.suffix=='.gz' else p.read_bytes())
c=load(HERE/'coverage.json');a=load(HERE/'coverage_classification.json');seal=load(HERE/'delivery.json');cases=load(HERE/'cases.json')
assert a['coverage_sha256']==sha((HERE/'coverage.json').read_bytes())
assert a['script_sha256']==sha((HERE/'classify_coverage.py').read_bytes())
assert [x['case'] for x in c['observations']]==[x['id'] for x in cases]
assert all(x['comparison']=='exact' for x in c['observations'])
unhit=[];summaries={};functions={}
for file,f in c['files'].items():
    source=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',seal['source_pin']+':'+file]).decode().splitlines()
    branches=[b for line in f['lines'].values() for b in line['branches']]
    summary=dict(functions_total=len(f['functions']),functions_hit=sum(v['calls']>0 for v in f['functions'].values()),lines_total=len(f['lines']),lines_hit=sum(l['count']>0 for l in f['lines'].values()),branches_total=len(branches),branches_hit=sum(b['count']>0 for b in branches),unexecuted_lines=sorted(int(n) for n,l in f['lines'].items() if not l['count']))
    assert summary==c['summary'][file],file;summaries[file]=summary
    functions[file]=list(f['functions'])
    for n,line in f['lines'].items():
        for i,b in enumerate(line['branches']):
            if not b['count']:unhit.append((file,int(n),i,source[int(n)-1].strip(),b['throw'],b['fallthrough']))
assert sorted(unhit)==sorted((e['file'],e['line'],e['branch'],e['source'],e['gcov_throw'],e['gcov_fallthrough']) for e in a['unhit_edges'])
assert dict(collections.Counter(e['classification'] for e in a['unhit_edges']))==a['classification_counts']
commit_files={}
for commit in ['a4e7177','fced9cb','2038c41']:
    files=subprocess.check_output(['git','-C',str(ROOT),'diff-tree','--no-commit-id','--name-only','-r',commit],text=True).splitlines()
    assert files and all(f.startswith('validation/preprocessing/') for f in files),(commit,files)
    commit_files[commit]=files
cfg=load(HERE/'coverage_cfg.json.gz')
for name,r in cfg.items():assert sha(r['text'].encode())==r['sha256'],name
result=dict(delivery_commit='2038c41',commit_files=commit_files,coverage_summary=summaries,primary_functions=functions,all_147_unhit_edges_accounted=True,coverage_classifications=a['classification_counts'],open_defined_coverage=[e for e in a['unhit_edges'] if e['classification'].startswith('defined')],coverage_cfg_sha256={n:v['sha256'] for n,v in cfg.items()},instrumented_case_membership_exact=True,instrumented_cases=302,artifact_count=len(seal['artifact_sha256'])+1,protected_artifact_count=len(seal['protected_artifact_sha256']))
p=Path('/tmp/tempo2-preprocessing-acceptance-checks.json');p.write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
print('PASS committed scope, coverage totals, all missing-edge dispositions and CFG identities:',p)
