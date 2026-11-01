#!/usr/bin/env python3
"""Replay affected AA controls against the corrected fresh corpus, without old whitespace artifacts."""
import argparse
import difflib
import json
from pathlib import Path
import sys
from common import ROOT, Run, sha


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--out',type=Path,required=True);p.add_argument('--corpus',type=Path,required=True)
    a=p.parse_args();run=Run(a.out);receipt=a.corpus.resolve()/'receipt.json'
    corpus=json.loads(receipt.read_text());root=Path(corpus['workspace'])
    port=root/'validation/observer_coordinates_port';output=root/'rust/verification/observer_coordinates_corrected'
    direct=output/'release/direct.json';assert json.loads(direct.read_text())['status']=='PASS'
    original=ROOT/'validation/observer_coordinates_port/controls.py';source=original.read_text()
    old="HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[1];X=ROOT/'validation/observer_coordinates'"
    assert source.count(old)==1
    routed=source.replace(old,"HERE=Path("+repr(str(port))+");ROOT=HERE.parents[1];X=ROOT/'validation/observer_coordinates'")
    old="a=module('aa_run_controls',HERE/'run.py')";assert routed.count(old)==1
    routed=routed.replace(old,old+'\na.OUT=Path('+repr(str(output))+')')
    for old,new in [
        ("dest=a.OUT/args.profile/'controls'", "dest=Path("+repr(str(run.out/'products'))+")"),
        ("staged=HERE/'build/control-routing.py'", "staged=Path("+repr(str(run.out/'native-control-routing.py'))+")")]:
        assert routed.count(old)==1
        routed=routed.replace(old,new)
    # Retain all 35 type/identity controls, 19 independent numeric checks and
    # fifteen compiled native mutation comparisons. Whitespace code is unchanged
    # by 0117 and its complete old central artifact is not a bootstrap dependency.
    end=routed.index(' # Existing central memmove experiment')
    routed=routed[:end]+''' exe=Path(direct['identity']['executable']);ident=a.identity(exe)
 assert ident==direct['identity'],'corrected observer executable/source identity'
 a.save(dest/'receipt.json',dict(status='PASS',identity=ident,controls=rows,
 supporting_checks=support['checks'],retained_mutation_witnesses=witnesses,
 direct_sha256=a.sha(a.OUT/args.profile/'direct.json'),
 scope='Affected 0117 controls; unchanged whitespace buffer campaign excluded'))
 print('PASS 35 controls, 19 supporting checks, 15 native mutation witnesses')
if __name__=='__main__':main()
'''
    path=run.out/'controls.py';path.write_text(routed)
    patch=run.out/'routing.patch';patch.write_text(''.join(difflib.unified_diff(source.splitlines(True),routed.splitlines(True),fromfile='historical/controls.py',tofile='AL/controls.py')))
    run.run([sys.executable,path,'--profile','release'],'controls',cwd=root)
    product=run.out/'products/receipt.json'
    run.save({'status':'pass','scope':'35 controls, 19 independent checks, 15 actual native mutation comparisons; unchanged whitespace excluded',
        'inputs':{str(x):sha(x) for x in [receipt,direct,original,
            root/'validation/observer_coordinates/outputs/controls.json',
            root/'validation/observer_coordinates/outputs/mutations.json.gz']},
        'routed_source':sha(path),'routing':sha(patch),'product':sha(product),'producer':sha(__file__)})
