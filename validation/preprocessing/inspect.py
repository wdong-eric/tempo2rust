#!/usr/bin/env python3
"""Reconstruct complete states and raw streams; never rewrites reference evidence."""
import argparse,decimal,gzip,hashlib,json,struct
from pathlib import Path
HERE=Path(__file__).resolve().parent

def load(p):return json.loads(gzip.decompress(p.read_bytes()) if p.suffix=='.gz' else p.read_bytes())
def sha(b):return hashlib.sha256(b).hexdigest()
def unpack(v):
    b=bytearray(v['size']);end=0
    for start,hexstr in v['nonzero']:
        data=bytes.fromhex(hexstr);assert start>=end and start+len(data)<=len(b)
        assert all(data);b[start:start+len(data)]=data;end=start+len(data)
    return bytes(b)

def number(b):
    # Exact binary128 bit decomposition; decimal is a view, bits remain the oracle.
    n=int.from_bytes(b,'little');sgn=-1 if n>>127 else 1;e=(n>>112)&32767;frac=n&((1<<112)-1)
    if e==32767:return 'NaN' if frac else '-Infinity' if sgn<0 else '+Infinity'
    if e==0 and frac==0:return '-0' if sgn<0 else '0'
    with decimal.localcontext() as c:
        c.prec=40
        coefficient=sgn*(frac+(1<<112 if e else 0));power=(e if e else 1)-16383-112
        return str(decimal.Decimal(coefficient << power) if power>=0 else decimal.Decimal(coefficient)/decimal.Decimal(1 << -power))

def values(state,p,key,typ):
    b=unpack(state[f'p{p}.{key}'])
    if typ=='q':return [number(b[i:i+16]) for i in range(0,len(b),16)]
    return list(struct.unpack('<'+typ*(len(b)//struct.calcsize(typ)),b))

def param(state,p,name,j=0,field='val'):
    inv=load(HERE/'inventory.json');i=inv['parameter_enum'].index('param_'+name)
    sizes=values(state,p,'param.aSize','i');base=sum(sizes[:i])+j
    return values(state,p,'param.'+field,'i' if field in ['fitFlag','paramSet'] else 'q')[base]

def reconstruct(record,initial):
    assert sha(json.dumps(initial,sort_keys=True,separators=(',',':')).encode())==record['events'][0]['state_sha256']
    state=initial['state'].copy();ends=[]
    for e in record['events'][1:]:
        for k,v in e.get('changes',{}).items():
            assert state.get(k)==v['before'],(record['id'],e,k,'broken before-image')
            if v['after'] is None:state.pop(k)
            else:state[k]=v['after']
        if e['phase'] in ['end','exit_state']:ends.append((e,state.copy()))
    return state,ends

def raw_check(path,records):
    archive=load(path);assert [x['id'] for x in archive['cases']]==[x['id'] for x in records]
    for c,r in zip(archive['cases'],records):
        lines=c['events_without_initial'].splitlines(keepends=True)
        original=archive['initial_events'][c['initial_event_ref']]
        assert sha(original.encode())==c['initial_event_ref'];lines.insert(1,original)
        raw=''.join(lines).encode();receipt=c['receipt']
        assert sha(raw)==receipt['events_sha256']
        ev=[json.loads(x) for x in lines];assert ev[0]=={'phase':'launch','nonce':receipt['nonce']}
        ev=ev[1:];initial=ev[0];ev[0]={'phase':'initial','state_sha256':sha(json.dumps(initial,sort_keys=True,separators=(',',':')).encode())}
        # State/nonce bytes are path-independent. Only declared diagnostics are normalized.
        assert ev==r['events'],c['id']
        for stream in ['stdout','stderr']:
            data=bytes.fromhex(c[stream+'_hex']);assert sha(data)==receipt[stream+'_sha256']
            text=data.decode('latin1');p=archive['profile'];src=next(x[2:] for x in p['command'] if x.startswith('-I'))
            text=text.replace(src,'{source}').replace(str(HERE/'build'),'{build}').replace(str(HERE),'{preprocessing}')
            assert text==r[stream],(c['id'],stream)
        assert receipt['returncode']==r['returncode']
        assert receipt['executable_sha256']==archive['profile']['executable_sha256']
    return dict(cases=len(records),raw_sha256=sha(path.read_bytes()),lossless_reconstruction=True)

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--case');a=ap.parse_args()
    assert number(((16383 << 112)|(1 << 127)).to_bytes(16,'little'))=='-1'
    rs=load(HERE/'observations.json.gz');initial=load(HERE/'initial.json.gz');states={}
    for r in rs:
        s,_=reconstruct(r,initial);states[r['id']]=s
        # Validate sparse representations, not just the selected fields used below.
        for v in s.values():
            if isinstance(v,dict) and set(v)=={'size','nonzero'}:unpack(v)
    if a.case:
        r=next(x for x in rs if x['id']==a.case)
        print(json.dumps(dict(record=r,final_state=states[a.case]),indent=2));return
    checks=[raw_check(HERE/'reference.raw.json.gz',rs)]
    if (HERE/'replay.raw.json.gz').exists():checks.append(raw_check(HERE/'replay.raw.json.gz',rs))
    findings={}
    for cid in ['wrapper_link_chain','wrapper_overwritten_label','epoch_cross_1','epoch_cross_2','wrapper_epoch_multi','epoch_motion_15','units_2_1_@','simul_only_carried','simul_order_chain','wrapper_multi_select_order','plugin_ok']:
        s=states[cid];findings[cid]={}
        for p in [0,1]:
            findings[cid]['p'+str(p)]={
                'F0':param(s,p,'f'),'F1':param(s,p,'f',1),'F2':param(s,p,'f',2),'F2_prefit':param(s,p,'f',2,'prefit'),
                'PEPOCH':param(s,p,'pepoch'),'SINI':param(s,p,'sini'),'KIN':param(s,p,'kin'),
                'DAOP':param(s,p,'daop'),'IPERHARM':param(s,p,'iperharm'),
                'deleted':[s[f'p{p}.obs{i}.deleted'] for i in range(4)],'nits':values(s,p,'nits','i')}
    out=dict(cases=len(rs),actions=sum(len(r['actions']) for r in rs),events=sum(len(r['events']) for r in rs),raw_archives=checks,all_delta_chains_valid=True,initial_identity=sha(json.dumps(initial,sort_keys=True,separators=(',',':')).encode()),findings=findings)
    (HERE/'inspection.json').write_text(json.dumps(out,indent=2)+'\n');print('PASS exact raw reconstruction and complete delta chains',len(rs))
if __name__=='__main__':main()
