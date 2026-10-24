"""Read both immutable original-source corpora for complete observer/coverage audits."""
from run import OUTPUT,frozen_cases,read_gzip,compare
from supplemental import DEST,frozen_cases as supplemental_cases

def load():
 catalog=frozen_cases()+supplemental_cases()
 results=read_gzip(OUTPUT/'reference.json.gz')+read_gzip(DEST/'reference.json.gz')
 compare([c['id'] for c in catalog],[r['id'] for r in results],'combined oracle membership')
 assert len({c['id'] for c in catalog})==len(catalog)
 return catalog,results
