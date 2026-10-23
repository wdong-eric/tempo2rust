"""Mutation controls use the same normal comparator and launch/identity gates."""
import copy,json,subprocess,sys,tempfile
from pathlib import Path
from unittest.mock import patch
import run as port

def run(raw,native,identity,exe):
    results=[]
    def reject(name,fn):
        try:fn()
        except (AssertionError,FileNotFoundError):results.append(dict(name=name,status='rejected'));return
        raise AssertionError('control accepted: '+name)
    port.validate(raw,native);results.append(dict(name='positive exact comparison',status='accepted'))
    mutations={
        'missing case':lambda p:p['cases'].pop(),
        'reordered cases':lambda p:p['cases'].reverse(),
        'missing action':lambda p:p['cases'][0]['actions'].pop(),
        'missing event':lambda p:p['cases'][0]['events'].pop(),
        'reordered fields via cell indices':lambda p:p['registry'].reverse(),
        'missing field':lambda p:p['initialized_bytes'][0]['cells'][0].pop('paramSet'),
        'wrong enum':lambda p:p['registry'][2].update(enum_index=12),
        'wrong logical extent':lambda p:p['initialized_bytes'][2].update(aSize=12),
        'wrong backing extent':lambda p:p['initialized_bytes'][2]['cells'].pop(),
        'changed initialized value':lambda p:p['initialized_bytes'][0]['cells'][0]['val'].update(value='1'),
        'changed signed flag':lambda p:p['initialized_bytes'][0]['cells'][0].update(fitFlag=-2),
        'changed blank label':lambda p:p['initialized_bytes'][0]['cells'][1].update(label_bytes='78'+p['initialized_bytes'][0]['cells'][1]['label_bytes'][2:]),
        'invented prefit default':lambda p:p['initialized_bytes'][0]['cells'][0].update(prefit={'class':'finite','value':'0'}),
        'lost copy tail':lambda p:p['cases'][7]['events'][1]['result']['destination_after']['cells'][3]['val'].update(value='0'),
        'lost copy link':lambda p:p['cases'][8]['events'][1]['result']['destination_after']['linkTo'].__setitem__(4,None),
        'lost strcpy suffix':lambda p:p['supplemental'][1]['destination_after']['cells'][0].update(label_bytes='7800'+'00'*62),
        'changed boundary class':lambda p:p['boundaries'][0]['error'].update(kind='unwritten_value'),
        'rolled back copy prefix':lambda p:p['boundaries'][6].update(after=copy.deepcopy(p['boundaries'][6]['before'])),
        'changed boundary caller values':lambda p:[p['boundaries'][6][state]['cells'][2]['val'].update(value='123') for state in ['before','after']],
        'changed boundary caller links':lambda p:[p['boundaries'][6][state]['linkTo'].__setitem__(4,123) for state in ['before','after']],
        'aSize written before boundary':lambda p:p['boundaries'][6]['after'].update(aSize=3),
    }
    for name,mutate in mutations.items():
        changed=copy.deepcopy(raw);mutate(changed);reject(name,lambda:port.validate(changed,native))
    for key in ['compiler_sha256','cargo_sha256','sources','executable_sha256']:
        changed=copy.deepcopy(identity);changed[key]='wrong';reject('wrong '+key,lambda:port.validate_identity(identity,changed))
    with tempfile.TemporaryDirectory(prefix='parameter-controlsO-',dir='/private/tmp') as td:
        fake=Path(td)/'probe';fake.write_text('#!/bin/sh\nexit 0\n');fake.chmod(0o755)
        reject('forwarding/wrong executable digest',lambda:port.execute(fake,identity,port.load(port.HERE/'case_projection.json')))
        reject('no-op launch cannot reuse output',lambda:port.execute(fake,port.identity(fake),[]))
        fake.write_text('''#!/usr/bin/env python3
import os,json
with open(os.environ['PARAMETER_OUTPUT'],'w') as f:json.dump({'token':'stale','input_sha256':'stale'},f)
print('test parameter_storage_tests::probe ... ok')
print('1 passed; 0 failed')
''')
        reject('stale launch challenge',lambda:port.execute(fake,port.identity(fake),[]))
        out=Path(td)/'failed-run';out.mkdir();receipt=out/'debug_direct.json';receipt.write_text('{"status":"PASS"}')
        failed=subprocess.CompletedProcess([],1,'','injected build failure')
        with patch.object(port,'OUT',out),patch.object(sys,'argv',['run','--profile','debug']),patch.object(port.subprocess,'run',return_value=failed):
            reject('failed run deletes previous success receipt',port.main)
        assert not receipt.exists()
    return results
