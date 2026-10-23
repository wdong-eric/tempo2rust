"""Independent state/stream witnesses; byte decoding does not alter the oracle."""
import gzip,json,re,struct
from profile import HERE,blob,save,sha
from inventory import clean
from run import read_observations,compare

def raw(x):
    b=bytearray(x['size'])
    for start,h in x['nonzero']:b[start:start+len(h)//2]=bytes.fromhex(h)
    return bytes(b)
def number(state,name,fmt='i',index=0):return struct.unpack_from('<'+fmt,raw(state[name]),index*struct.calcsize(fmt))[0]
def states(r,initial):
    state=initial.copy();result=[]
    for e in r['events']:
        if e['phase'] in ['end','exit_state']:
            for k,v in e['changes'].items():
                if v['after'] is None:state.pop(k,None)
                else:state[k]=v['after']
            result.append((e,state.copy()))
    return result

def main():
    rs={r['id']:r for r in read_observations()};initial=json.loads(gzip.decompress((HERE/'initial_state.json.gz').read_bytes()));checks=[]
    def check(name,truth,detail):
        assert truth,(name,detail);checks.append(dict(name=name,detail=detail,passed=True))
    def final(id):return states(rs[id],initial)[-1][1]
    s=final('jump_retention');check('ordinary JUMP retains SAT flag and failed numeric value',number(s,'p0.jumpSAT','b',1)==1 and number(s,'p0.jumpVal','d',1)==2.0,'SATJUMP, reset count, JUMP BAD: SAT[1]=1, value[1]=2')
    check('jump error arrays retain explicit nonzero caller values',number(s,'p0.jumpValErr','d',1)==8 and number(s,'p0.fdjumpValErr','d',1999)==2008,'Error destinations are not scanned by the selected jump records.')
    check('one-based last jump/FD slots',number(s,'p0.nJumps')==1999 and number(s,'p0.nfdJumps')==1999,'Index 1999 written; index 2000 never attempted.')
    s=final('dm_histories');check('DM reset versus independent solar counter',(number(s,'p0.dmoffsDMnum'),number(s,'p0.dmoffsCMnum'),number(s,'p0.ne_sw_ifuncN'))==(2,2,5),'DMMODEL resets DM/CM counts only.')
    check('DM error discarded and CM missing error zero',number(s,'p0.dmoffsDM_error','d',0)==0 and number(s,'p0.dmoffsCM_error','d',1)==0 and number(s,'p0.dmoffsDM_weight','d',0)==1,'Parsed error is discarded for DMOFF/_DM; _CM local error starts zero.')
    s=final('constraint_DMMODEL_DM1');check('constraint independent-if accumulation',number(s,'p0.nconstraints')==22 and number(s,'p0.constraints',index=0)==1 and number(s,'p0.constraints',index=1)==0,'Each DMMODEL_DM1 appends 1 individual + 4 common + 6 annual terms, including repeats.')
    for cid,last in [('constraint_IFUNC_COV',10),('constraint_PARAM',0)]:
        x=final(cid)['p0.constraint_special[0]'];b=raw(x['initialized_prefix']);check(cid+' initialized allocation prefix',b[-1]==last and x['allocated']==len(b)+1 and x['unwritten_tail']==1,dict(prefix_hex=b.hex(),allocated=x['allocated'],unwritten_tail=1))
    s=final('qifunc_precedence');check('lowercase substring selected by earlier IFUNC',number(s,'p0.ifuncN')==1 and number(s,'p0.quad_ifuncN_p')==9,'aaaaa1qifunc_p9 gives valid fixed-offset index 1 in earlier IFUNC branch.')
    s=final('substring_precedence');check('lowercase wave_cos enters sine branch',number(s,'p0.wave_sine','d',1)==8 and number(s,'p0.wave_cos','d',1)==number(initial,'p0.wave_cos','d',1),'wave_sin2 updates sine only; token-specific stream evidence also records wave_cos1.')
    s=states(rs['shape_retention'],initial)[0][1]
    check('shapelet warning preserves coefficient tail',number(s,'p0.TNShapeletEvCoef','d',0)==2.5 and number(s,'p0.TNShapeletEvCoef','d',1)==number(initial,'p0.TNShapeletEvCoef','d',1),'Failed second scan leaves coefficients 1..9 caller-initialized.')
    check('missing fit flag retained',number(s,'p0.TNShapeletEvFitFlag',index=0)==number(initial,'p0.TNShapeletEvFitFlag',index=0),'BAD is not an integer; old flag survives.')
    s=final('shape_invalid_scale');check('scale exit precedes event count',number(s,'p0.nTNShapeletEvents')==0 and number(s,'p0.TNShapeletEvCoef','d',1)==2,'Normal source exit 1 with defined prefix captured by atexit.')
    r=rs['clock_history'];vals=[e['result'] for e in r['events'] if e['phase']=='end' and e['op']=='route'];check('clock chain repeat/cross-pulsar history',[v['count'] for v in vals]==[2,2,2] and vals[0]==vals[1]==vals[2],vals)
    for reader in ['full','simple','global']:
        s=final('reader_'+reader);check(reader+' append/reset history',number(s,'p0.nfdJumps')==4 and number(s,'p0.nJumps')==(1 if reader=='full' else 4),dict(nJumps=number(s,'p0.nJumps'),nfdJumps=number(s,'p0.nfdJumps'),nconstraints=number(s,'p0.nconstraints')))
    for r in rs.values():
        initial_event=r['events'][0];check_digest=sha(json.dumps(initial,sort_keys=True,separators=(',',':')).encode());assert initial_event['state_sha256']==check_digest
    # Reconstitute exact raw JSONL, including the content-addressed shared initial line.
    manifests=[]
    for kind in ['reference','replay']:
        p=HERE/(kind+'.raw.json.gz')
        if not p.exists():continue
        manifest=json.loads(gzip.decompress(p.read_bytes()));assert [x['id'] for x in manifest['cases']]==list(rs)
        for item in manifest['cases']:
            artifact=HERE/item['path'];assert sha(artifact.read_bytes())==item['sha256'];x=json.loads(gzip.decompress(artifact.read_bytes()));il=x['initial_line'];line=gzip.decompress((HERE/il['path']).read_bytes()).decode();assert sha(line.encode())==il['sha256']
            lines=x['events_jsonl'].splitlines(keepends=True);lines.insert(il['index'],line);assert sha(''.join(lines).encode())==x['receipt']['events_sha256']
            assert sha(bytes.fromhex(x['stdout_hex']))==x['receipt']['stdout_sha256'];assert sha(bytes.fromhex(x['stderr_hex']))==x['receipt']['stderr_sha256'];assert json.loads(lines[0])['nonce']==x['receipt']['nonce']
        manifests.append(kind)
    save(HERE/'analysis.json',dict(checks=checks,raw_archives_reconstructed_and_verified=manifests,cases=len(rs),events=sum(len(r['events']) for r in rs.values())))
    print('PASS',len(checks),'semantic witnesses and raw reconstruction')
if __name__=='__main__':main()
