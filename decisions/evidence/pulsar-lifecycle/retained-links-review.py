"""Central diagnostic for retained embedded link arrays; never replaces S's oracle."""
import copy,hashlib,json,sys,tempfile
from pathlib import Path
root=Path('/Users/Eric.Dong/Downloads/tempo2rust')
sys.path.insert(0,str(root/'validation/pulsar_lifecycle'))
import profile as profile_module
import run
from fixtures import cases
out=Path(tempfile.mkdtemp(prefix='tempo2-pulsar-links-central-',dir='/private/tmp'))
profile_module.BUILD=out
run.BUILD=out
files=[p for p in (root/'validation/pulsar_lifecycle').rglob('*') if p.is_file() and 'build' not in p.parts and '__pycache__' not in p.parts]
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
before={str(p.relative_to(root)):digest(p) for p in files}
run.protected()
mut=('initialise.C','psr->param[i].nLinkTo   = 0;', 'psr->param[i].nLinkTo   = 0; memset(psr->param[i].linkTo, 0, sizeof psr->param[i].linkTo); memset(psr->param[i].linkFrom, 0, sizeof psr->param[i].linkFrom);')
receipt={'status':'ACCEPTANCE_BLOCKER_CONFIRMED','output':str(out),'cases':[],'profiles':{},'mutation':mut}
for name,change in [('ordinary',None),('reset_retained_links',mut)]:
 exe,p=profile_module.compile_probe(name,mutation=change)
 receipt['profiles'][name]=p
 for cid in ['allocate_1','seeded_reinitialise']:
  c=next(c for c in cases() if c['id']==cid)
  expected=run.reference(cid)
  actual=run.execute(c,exe,p,out/'raw'/name)
  run.compare(c['returncode'],actual['returncode'],cid+'.status')
  run.compare(expected,actual,cid)
  print(name,cid,'exactly matches delivered projection',flush=True)
  receipt['cases'].append({'build':name,'case':cid,'result':'exactly matches delivered projection','events':len(actual['events']),'field_records':run.validate(actual),'execution':run.execution_identity(exe,p)})
  del actual,expected
run.protected()
after={str(p.relative_to(root)):digest(p) for p in files}
assert before==after,'S evidence changed'
receipt['preserved_files']=len(before)
receipt['input_hashes']=before
profile_module.save(out/'central-review.json',receipt)
Path('/tmp/tempo2-pulsar-links-review-output').write_text(str(out)+'\n')
print('CONFIRMED: zeroing retained link arrays is invisible to the delivered exact state comparison',flush=True)
print('OUTPUT',out,flush=True)
