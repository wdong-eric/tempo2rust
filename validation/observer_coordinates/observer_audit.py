"""Equivalence of the private-state observer and raw ordinary translation units."""
import copy
from inventory import sha,save,preservation_status
from profile import compile_probe,actual_identity
from run import OUTPUT,execute,read_gzip,write_gzip,compare,frozen_cases

def project(result):
    r=copy.deepcopy(result)
    r['events']=[e for e in r['events'] if e['phase']!='exit']
    for e in r['events']:
        if 'private' in e:e['private']=None
    return r

def main():
    destination=OUTPUT/'observer_audit.json';destination.unlink(missing_ok=True)
    before=preservation_status();catalog=frozen_cases();ordinary=read_gzip(OUTPUT/'reference.json.gz');refhash=sha((OUTPUT/'reference.json.gz').read_bytes())
    compare([c['id'] for c in catalog],[r['id'] for r in ordinary])
    exe,p=compile_probe('observer',observer=True);receipts=[];observed=[]
    for c,expected in zip(catalog,ordinary):
        actual,receipt=execute(c,exe,p,OUTPUT/'observer_raw');again,replay=execute(c,exe,p,OUTPUT/'observer_replay_raw')
        compare(actual,again,c['id']+'.observer_replay');compare(expected,project(actual),c['id']+'.ordinary_equivalence')
        if c['status']:
            e=actual['events'][-1];assert e['phase']=='exit' and e['operation']==c['actions'][-1][0]
        receipts.append(dict(first=receipt,replay=replay));observed.append(actual);print('PASS observer',c['id'],flush=True)
    by_label={c['label']:r for c,r in zip(catalog,observed)}
    last=lambda label:by_label[label]['events'][-1]
    zero=last('ut1_zero_termination')['private']
    assert zero['count']==3 and zero['count2']==13 and zero['entry'][13:18]==[0,999,888,777,666]
    limit=last('ut1_row_limit')['private'];assert limit['count']==1000 and limit['count2']==6000 and limit['read']==0
    missing=last('ut1_missing')['private'];assert missing['count']==0 and missing['read']==0
    fail=last('satellite_no_epoch_x_velocity')['caller']['observations'][0]
    assert all(x['bits']=='0000000000000000' for x in fail['velocity'])
    assert fail['position'][0]['bits']=='408f400000000000'
    tai=last('tai_failure_prefix')['caller']['observations']
    assert tai[0]['correctionUT1']['bits']=='0000000000000000' and tai[1]['correctionUT1']['bits']=='405ec00000000000'
    shared=last('imag_defined_failure_prefix')['state']['observatories'][2]['coordinates'][0]
    import struct
    assert struct.unpack('>d',bytes.fromhex(shared['bits']))[0]==7123456
    private=[e['private'] for e in by_label['ut1_cache']['events'] if e['phase']=='end' and e['operation']=='ut1']
    for v in private[1:]:compare(private[0],v,'retained complete UT1 backing')
    for r in observed:
        for e in r['events']:
            if e.get('private') is not None:
                assert len(e['private']['mjdVal'])==1000 and len(e['private']['entry'])==10000
    after=preservation_status();assert sha((OUTPUT/'reference.json.gz').read_bytes())==refhash
    write_gzip(OUTPUT/'observer_snapshots.json.gz',observed);write_gzip(OUTPUT/'observer_receipts.json.gz',receipts)
    save(destination,dict(status='PASS' if not before and not after else 'PRESERVATION_DRIFT',profile=p,actual_identity=actual_identity(exe,p),preservation_before=before,preservation_after=after,matches=len(observed),fresh_replays=len(observed),prefix_and_retention_assertions=7,snapshot_sha256=sha((OUTPUT/'observer_snapshots.json.gz').read_bytes()),ordinary_sha256=refhash))
    assert not before and not after
    print('PASS observer equivalence, complete private state and failure prefixes')

if __name__=='__main__':main()
