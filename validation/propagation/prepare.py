"""One-shot pre-execution source and authored-case freeze; never replaces evidence."""
from inventory import HERE,freeze,save
from fixtures import cases,validate_inputs
if __name__=='__main__':
 freeze();catalog=cases()
 assert len({c['id'] for c in catalog})==len(catalog)
 for c in catalog:validate_inputs(c)
 save(HERE/'outputs/frozen/cases.json',catalog)
 print('Frozen',len(catalog),'authored cases before execution')
