"""Lossless raw/projection validation and complete action-to-action field deltas."""
import base64
import gzip
import hashlib
import json
from pathlib import Path
from fixtures import cases
from profile import HERE,save,sha
from run import compare,reference,validate


def main():
    raw=[];transitions=0
    dest=HERE/'deltas';dest.mkdir(exist_ok=True)
    for c in cases():
        deltas=[]
        r=reference(c['id']);validate(r)
        bundle=json.loads(gzip.decompress((HERE/'raw'/(c['id']+'.json.gz')).read_bytes()))
        events=[json.loads(s) for s in base64.b64decode(bundle['events_base64']).splitlines()]
        compare(events.pop(0),dict(phase='launch',nonce=bundle['nonce']))
        compare(len(events),len(r['events']))
        for a,b in zip(events,r['events']):compare(a['state'],b['state'],'raw.state');compare(a['phase'],b['phase'],'raw.phase')
        for i in range(1,len(r['events'])):
            before=r['events'][i-1];after=r['events'][i];a={x['path']:x for x in before['state']};b={x['path']:x for x in after['state']}
            # Every changed, removed or added record carries its complete values.
            changes=[dict(path=k,before=a.get(k),after=b.get(k)) for k in sorted(a.keys()|b.keys()) if a.get(k)!=b.get(k)]
            deltas.append(dict(case=c['id'],before_phase=before['phase'],after_phase=after['phase'],before_index=i-1,after_index=i,changes=changes))
        (dest/(c['id']+'.json.gz')).write_bytes(gzip.compress(json.dumps(deltas,separators=(',',':')).encode(),mtime=0))
        transitions+=len(deltas)
        raw.append(dict(case=c['id'],raw_sha256=sha((HERE/'raw'/(c['id']+'.json.gz')).read_bytes()),expected_sha256=sha((HERE/'expected'/(c['id']+'.json.gz')).read_bytes()),records=validate(r)))
    save(HERE/'raw_validation.json',dict(status='PASS',cases=raw,transitions=transitions,encoding='Ordered native field records, exact primitive bytes in maximal equal-unit runs; symbolic pointers; structs recursively omit padding. RLE counts/width/hex/canonical adjacency validated. No unwritten range is decoded.'))
if __name__=='__main__':main()
