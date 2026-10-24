"""Read-only pointer/call observer; full ordinary projection audited separately."""
import copy
from inventory import check,save,sha
from profile import compile_probe,resources,actual_identity
from run import OUTPUT,execute,validate,frozen_cases,read_gzip,write_gzip,compare

def projection(result):
    out=copy.deepcopy(result);out['events']=[e for e in out['events'] if e['phase']!='exit']
    for e in out['events']:e.update(private=None,calls=None,open_handles=None)
    return out

def main():
    check();oracle=OUTPUT/'reference.json.gz';old=sha(oracle.read_bytes());catalog=frozen_cases();baseline=read_gzip(oracle)
    rm=resources();exe,profile=compile_probe('observed',observer=True)
    assert len(str(exe.parent/'source'))==len(str(exe.parent.parent/'ordinary/source'))
    results=[];receipts=[]
    for c,ordinary in zip(catalog,baseline):
        r,receipt=execute(c,exe,profile,OUTPUT/'observer_raw',rm);validate(c,r,True)
        compare(ordinary,projection(r),c['id']+'.ordinary_projection')
        again,rr=execute(c,exe,profile,OUTPUT/'observer_replay_raw',rm);validate(c,again,True);compare(r,again,c['id']+'.observer_replay')
        for e in r['events']:
            if e['phase']=='end' and e['operation']=='driver':assert e['open_handles']==0
            if e['private']['random']:
                assert all(q=={'class':'finite','little_endian_bytes':'0'*32} for q in e['private']['random']['r'][97:])
        results.append(r);receipts.append(dict(first=receipt,replay=rr));print('PASS observer',c['id'],flush=True)
    write_gzip(OUTPUT/'observed.json.gz',results);write_gzip(OUTPUT/'observer_receipts.json.gz',receipts)
    check();actual_identity(exe,profile);assert sha(oracle.read_bytes())==old
    save(OUTPUT/'observer_audit.json',dict(status='PASS',cases=len(results),ordinary_projection_exact=len(results),observer_repeat_exact=len(results),profile=profile,observed_sha256=sha((OUTPUT/'observed.json.gz').read_bytes()),oracle_sha256=old))
if __name__=='__main__':main()
