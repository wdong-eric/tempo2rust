#!/usr/bin/env python3
"""Challenge 0127 expectations through the unchanged complete P Rust comparator."""
import argparse
import copy
import json
import os
from pathlib import Path
import shutil
from common import Run, sha
from discrete_contract import p_state


def controls(args):
    run=Run(args.out);corpus=args.corpus.resolve()
    case=next(c for c in json.loads((corpus/'cases.json').read_text()) if c['id']=='simul_123')
    initial=json.loads((corpus/'initial.json').read_text())
    _,states=p_state.reconstruct(case,initial)
    state=next(s for e,s in states if e.get('action')==6)
    records=[]
    for name in ['positive','wrong-decision','missing-state','extra-state','observation-order',
                 'class-sign','unrelated-state','repeat','threshold','diagnostics']:
        work=run.out/name;work.mkdir();value=copy.deepcopy(case);value['workdir']=str(work)
        event=next(e for e in value['events'] if e.get('action')==6 and e['phase']=='end')
        changes=event['changes']
        if name=='wrong-decision':changes['p0.obs0.deleted']['after']=0
        elif name=='missing-state':changes['p0.obs0.freq']={'before':state['p0.obs0.freq'],'after':None}
        elif name=='extra-state':changes['unexpected']={'before':None,'after':0}
        elif name=='observation-order':
            for i,j in [(0,1),(1,0)]:
                key=f'p0.obs{i}.whole_record'
                changes[key]={'before':state[key],'after':state[f'p0.obs{j}.whole_record']}
        elif name=='class-sign':
            key='p0.obs0.toaErr';negative={**state[key],'bits_le':'00000000000000c0'}
            changes[key]={'before':state[key],'after':negative}
        elif name=='unrelated-state':
            key='p1.obs0.freq';changes[key]={'before':state[key],
                'after':{'type':'binary64','class':'finite','bits_le':'0000000000000000'}}
        elif name=='repeat':
            event=next(e for e in value['events'] if e.get('action')==7 and e['phase']=='end')
            event['changes']['p0.obs0.deleted']={'before':1,'after':0}
        elif name=='threshold':
            for action in value['actions'][-2:]:action[1]+=' 0'
        elif name=='diagnostics':value['stdout']='unexpected diagnostic\n'
        for filename in ['initial.json','floating-layout.json']:shutil.copyfile(corpus/filename,work/filename)
        for filename,raw in value['files'].items():(work/filename).write_bytes(bytes.fromhex(raw))
        (work/'cases.json').write_text(json.dumps([value])+'\n')
        command=['cargo','test','--manifest-path',args.candidate.resolve()/'rust/Cargo.toml',
                 '--offline','--locked',*(['--release'] if args.release else []),'--lib',
                 'preprocessing_corpus_tests::p_corpus','--','--ignored','--exact','--test-threads=1']
        run.run(command,name,env={**os.environ,'AC_PREPROCESS_CORPUS':str(work)},expected=None)
        status=run.commands[-1]['status']
        differences=json.loads((work/'differences.json').read_text())
        diagnostics=json.loads((work/'diagnostic-differences.json').read_text())
        assert status==(0 if name=='positive' else 101),(name,status)
        assert bool(differences or diagnostics)==(name!='positive'),name
        records.append({'control':name,'exit':status,'state_differences':len(differences),
                        'diagnostic_differences':len(diagnostics)})
    run.save({'status':'pass','records':records,'inputs':{str(p):sha(p) for p in
        [corpus/'cases.json',corpus/'initial.json',args.candidate.resolve()/'rust/src/preprocessing_corpus_tests.rs',Path(__file__)]},
        'scope':'One positive and nine corruptions through the actual complete state/diagnostic comparator; no production mutation'})


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    for name in ['corpus','candidate','out']:p.add_argument('--'+name,type=Path,required=True)
    p.add_argument('--release',action='store_true');controls(p.parse_args())
