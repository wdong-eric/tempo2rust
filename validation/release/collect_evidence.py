#!/usr/bin/env python3
"""Select rerunnable evidence; omit source trees, caches, libraries and runtime data."""
import argparse
import json
from pathlib import Path
import shutil
from common import sha

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--input',type=Path,required=True);p.add_argument('--out',type=Path,required=True)
    a=p.parse_args()
    if not a.input.is_dir():p.error('input directory does not exist: '+str(a.input))
    a.out.mkdir(parents=True,exist_ok=False);files={}
    excluded={'source','src','target','test-target','runtime','runtime-historical','prefix','upstream.git','.git','cmake','gmp-cache'}
    for path in sorted(a.input.rglob('*')):
        relative=path.relative_to(a.input)
        if not path.is_file() or path.is_symlink():continue
        # The unchanged AE runners write their numerical evidence inside the
        # isolated candidate export. Retain those results, not its source tree.
        fit_evidence=any('/candidate/source/validation/general_fitting/build/'+name+'/'
                         in '/'+relative.as_posix() for name in ['public','families'])
        if excluded.intersection(relative.parts) and not fit_evidence:continue
        if any(part.startswith('install-') and not part.endswith('-build') for part in relative.parts):continue
        if (path.suffix not in {'.json','.jsonl','.tsv','.stdout','.stderr','.log','.txt','.patch','.s','.diff'}
                and not path.name.endswith(('.json.gz','.jsonl.gz'))
                and path.name not in {'stdout','stderr','producer.py','polyco_new.dat','newpolyco.dat',
                                     't2pred.dat','t1.dat','cheby.dat','exchanged.dat'}):continue
        target=a.out/relative;target.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(path,target)
        files[str(relative)]={'sha256':sha(target),'bytes':target.stat().st_size}
    (a.out/'manifest.json').write_text(json.dumps({'source':str(a.input.resolve()),'files':files},indent=2)+'\n')
    print(len(files),'evidence files; raw binaries, dependencies and runtime trees excluded')
