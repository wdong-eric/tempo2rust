"""Show the read-only exit observer adds only its declared final state event."""
from inventory import HERE, sha, save, preservation_status
from profile import BUILD, compile_probe
from fixtures import cases
from run import OUTPUT, read_gzip, write_gzip, execute, compare, frozen_cases

def main():
    dest=OUTPUT/'observer_audit.json';dest.unlink(missing_ok=True)
    preservation_before=preservation_status();oracle=OUTPUT/'reference.json.gz';oracle_hash=sha(oracle.read_bytes())
    ordinary={r['id']:r for r in read_gzip(oracle)}
    catalog=frozen_cases();compare([c['id'] for c in catalog],list(ordinary),'oracle membership')
    caller=BUILD/'observer/probe.cc';caller.parent.mkdir(parents=True,exist_ok=True)
    source=(HERE/'probe.cc').read_text();assert source.count('atexit(observeExit);')==1
    caller.write_text(source.replace('atexit(observeExit);','(void)0;'))
    exe,profile=compile_probe('observer',caller=caller)
    receipts=[];matches=[]
    for c in catalog:
        actual,receipt=execute(c,exe,profile,OUTPUT/'observer_audit_raw')
        expected=ordinary[c['id']]
        expected={**expected,'events':[e for e in expected['events'] if e['phase']!='exit']}
        compare(expected,actual,c['id']);receipts.append(receipt);matches.append(c['id'])
    # These are actual initialized prefixes observed by the ordinary callback.
    for i in range(1,5):
        final=ordinary[f'observatory_partial_after_valid_{i}']['events'][-1]
        assert final['phase']=='exit' and len(final['state']['observatories'])==1
    final=ordinary['eop_bad_after_valid']['events'][-1]
    assert final['phase']=='exit' and len(final['caller_eop_samples'])==1
    final=ordinary['meteorology_partial_initialization_exit']['events'][-1]
    assert final['phase']=='exit' and len(final['state']['wet'])==3 and final['state']['pressure']==[]
    assert final['state']['meteorology_initialized']==0
    write_gzip(OUTPUT/'observer_audit_receipts.json.gz',receipts)
    preservation_after=preservation_status();assert sha(oracle.read_bytes())==oracle_hash
    status='PRESERVATION_DRIFT' if preservation_before or preservation_after else 'PASS'
    save(dest,dict(status=status,native_status='PASS',profile=profile,matches=matches,
        preservation_before=preservation_before,preservation_after=preservation_after,
        observed_prefix_assertions=6,oracle_sha256=oracle_hash))
    print('PASS exit observer agreement',len(matches),'cases and six initialized-prefix assertions')
    assert status=='PASS','observer audit completed; preservation gate remains unresolved'

if __name__=='__main__':main()
