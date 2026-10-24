"""Decision 0042 lifetime checks and exact comparison with the preserved delivery."""
import gzip
import json
import struct
from profile import HERE,BUILD,save,sha
from fixtures import cases
from run import compare,reference,protected

BASELINE_MANIFEST = '14115f05255c62f6935f8d7405f3b92903be686024b1c5683860be0b4b6fe985'
LINK_PHASES = {'allocate_-3':{'reallocated':11},'allocate_0':{'reallocated':11},
               'allocate_1':{'reallocated':11},'allocate_5':{'reallocated':11},
               'seeded_reinitialise':{'reinitialised':11,'initialised_again':22}}

def check_lifetimes(result):
    """Current embedded links survive native allocation; fresh prefit buffers do not."""
    checked=0
    for event in result['events']:
        phase=event['phase'];fields={f['path']:f for f in event['state']}
        value=LINK_PHASES.get(result['id'],{}).get(phase)
        if value is not None:
            for i in range(2000):
                prefix=f'p.param[{i}]'
                for name in ['linkTo','linkFrom']:
                    path=prefix+'.'+name
                    compare(dict(path=path,unit=4,count=5,runs=[[5,value.to_bytes(4,'little',signed=True).hex()]]),fields[path],path)
                    checked+=1
                for name in ['prefit','prefitErr']:
                    path=prefix+'.'+name+'.data';compare(dict(path=path,state='unwritten'),fields[path],path)
                for name in ['nLinkTo','nLinkFrom']:
                    path=prefix+'.'+name;compare(dict(path=path,unit=4,count=1,runs=[[1,'00000000']]),fields[path],path)
        # A natural first allocation must never read its unseeded embedded arrays.
        if result['id'].startswith('allocate_') and phase=='allocated':
            for i in range(2000):
                for name in ['linkTo','linkFrom']:
                    path=f'p.param[{i}].'+name;compare(dict(path=path,state='unwritten'),fields[path],path)
        if result['id']=='seeded_reinitialise' and phase=='caller_seeded':
            path='p.ToAextraCovar';compare(dict(path=path,symbol='first.extraCovar'),fields[path],path)
            for name,values in [('rows',[1]),('cols',[2]),('row_offsets',[0])]:
                path='p.ToAextraCovar.data.'+name
                compare(dict(path=path,unit=8,count=len(values),runs=[[1,v.to_bytes(8,'little').hex()] for v in values]),fields[path],path)
            path='p.ToAextraCovar.data.cells'
            compare(dict(path=path,unit=8,count=2,runs=[[1,struct.pack('<d',v).hex()] for v in [11.,12.]]),fields[path],path)
    return checked

def main():
    protected();baseline=BUILD/'pre0042';manifest_bytes=(baseline/'manifest.json').read_bytes()
    compare(BASELINE_MANIFEST,sha(manifest_bytes),'baseline manifest')
    manifest=json.loads(manifest_bytes)
    for name,digest in manifest['files'].items():compare(digest,sha((baseline/name).read_bytes()),'preserved.'+name)
    changed=[];known=0;added=0
    for case in cases():
        old=json.loads(gzip.decompress((baseline/'expected'/(case['id']+'.json.gz')).read_bytes()))
        new=reference(case['id']);known+=check_lifetimes(new)
        compare({k:v for k,v in old.items() if k!='events'},{k:v for k,v in new.items() if k!='events'},case['id'])
        compare(len(old['events']),len(new['events']),case['id']+'.events')
        for a,b in zip(old['events'],new['events']):
            compare({k:v for k,v in a.items() if k!='state'},{k:v for k,v in b.items() if k!='state'},case['id']+'.event')
            old_fields={f['path']:f for f in a['state']};new_fields={f['path']:f for f in b['state']}
            changed_paths=[];added_paths=[]
            allow_links=a['phase'] in LINK_PHASES.get(case['id'],{})
            for path,field in new_fields.items():
                if allow_links and path.startswith('p.param[') and path.endswith(('.linkTo','.linkFrom')):
                    compare(dict(path=path,state='unwritten'),old_fields[path],path+'.old')
                    changed_paths.append(path)
                elif case['id']=='seeded_reinitialise' and a['phase']=='caller_seeded' and path.startswith('p.ToAextraCovar.data.'):
                    assert path not in old_fields;added_paths.append(path)
                else:compare(old_fields[path],field,case['id']+'.'+a['phase']+'.'+path)
            compare(list(old_fields),[p for p in new_fields if p not in added_paths],case['id']+'.ordered_field_membership')
            if changed_paths or added_paths:changed.append(dict(case=case['id'],phase=a['phase'],known_links=len(changed_paths),added=added_paths))
            added+=len(added_paths)
    compare(24000,known,'known link records');compare(4,added,'extra covariance records')
    # Scientific inputs, source classifications and the protected membership stay frozen.
    for name in ['cases.json','inventory.json','protected_hashes.json','fixtures.py','inventory.py']:
        compare(manifest['files'][name],sha((HERE/name).read_bytes()),'unchanged.'+name)
    protected()
    save(HERE/'correction.json',dict(status='PASS',baseline_manifest_sha256=BASELINE_MANIFEST,preserved_files=len(manifest['files']),cases=len(cases()),known_link_records=known,added_records=added,changes=changed,all_other_projected_values_order_streams_status_unchanged=True))
    print('PASS: 24,000 retained-link records and four initial extra-covariance records; all other projected state unchanged')
if __name__=='__main__':main()
