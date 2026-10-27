"""Verify retained final artifacts and write a compact handoff index."""
import json, subprocess
from pathlib import Path
from check import HERE, ROOT, sha

def verify(mapping):
    for name,digest in mapping.items():assert sha(name)==digest,name

if __name__=='__main__':
    base=HERE/'build/profiles-final-v3';index={'profiles':{},'artifacts':{}}
    top=json.loads((base/'receipt.json').read_text());assert all(r['exit']==0 for r in top['commands'])
    for folder in sorted(p for p in base.iterdir() if p.is_dir()):
        record={};native=json.loads((folder/'native/identity.json').read_text())
        verify(native['objects']);verify({str(folder/'native'/p):h for p,h in native['executables'].items()})
        for name,v in native['source'].items():assert sha(folder/'native/source'/name)==v['compiled']
        for group in ['paired','cli','banner']:
            receipt=folder/group/'receipt.json'
            if not receipt.exists():
                assert group=='paired' and folder.name=='qr-default-debug' or group=='cli' and folder.name in ['blas-debug','lapack-debug','calceph-debug'],receipt
                continue
            r=json.loads(receipt.read_text());verify(r['files'])
            if group=='paired':
                verify(r['sources_and_inputs']);assert r['failures']==0 and not r['changed_artifacts']
                assert all(c['exit']==0 for x in r['records'] for c in x['commands'])
                actual={str(p) for x in r['records'] for p in (folder/group/x['case']).iterdir() if p.is_file()}
                assert actual==set(r['files'])
                record.update(cases=len(r['records']),cells=sum(x['comparison']['checks'] for x in r['records']),binary64_cells_equal=sum(x['comparison']['binary64_cells_equal'] for x in r['records']))
            elif group=='cli':verify(r['identity']);record['cli_commands']=len(r['records'])
            else:
                for x in r['records']:assert sha(x['executable'])==x['sha256']
                record['banner_commands']=len(r['records'])
            index['artifacts'][str(receipt)]=sha(receipt)
        index['artifacts'][str(folder/'native/identity.json')]=sha(folder/'native/identity.json')
        index['profiles'][folder.name]=record
    for path in ['provider-identity.json','final-checks-v3/receipt.json','public-boundaries-final-v3/receipt.json']:
        p=HERE/'build'/path;r=json.loads(p.read_text())
        if 'files' in r:verify(r['files'])
        if path.startswith('final-checks'):
            for x in r['records']:assert x['exit']==0 and sha(p.parent/(x['label']+'.log'))==x['log_sha256']
        index['artifacts'][str(p)]=sha(p)
    for p in [HERE/'build/comparator-controls-final.log',base/'receipt.json',HERE/'build/calceph-original-failed-position.tsv',HERE/'build/calceph-original-failed-position.stderr',HERE/'build/accel-assembly.txt']:
        index['artifacts'][str(p)]=sha(p)
    index['implementation_commits']=[subprocess.check_output(['git','rev-parse','4c04811'],cwd=ROOT,text=True).strip(),subprocess.check_output(['git','log','-1','--format=%H','--','rust/src/tkfit.rs'],cwd=ROOT,text=True).strip()]
    index['pins']={name:subprocess.check_output(['git','-C',str(ROOT/name),'rev-parse','HEAD'],text=True).strip() for name in ['tempo2','validation/reference']}
    index['authored']={str(p):sha(p) for p in HERE.iterdir() if p.is_file()}
    index['authored'].update({str(p):sha(p) for p in [ROOT/'rust/examples/native_backends.rs',ROOT/'rust/tests/native_backends.rs']})
    (HERE/'build/delivery-identity.json').write_text(json.dumps(index,indent=2)+'\n')
    print(json.dumps(index['profiles'],indent=2))
