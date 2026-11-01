#!/usr/bin/env python3
"""Classify saved corpus differences and test one source-bound scalar lead."""
from collections import Counter
from datetime import datetime,timezone
import hashlib,json,math,struct,subprocess,sys,zipfile
from pathlib import Path
ROOT=Path('/Users/Eric.Dong/Downloads/tempo2rust')
PACKET=ROOT/'decisions/evidence/release-qualification/build/linux-run-41294531402-review-20261101'
OUT=Path(sys.argv[1]);OUT.mkdir(parents=True,exist_ok=False)
PIN='fecbc18af95196a16db2046306099b6345a50976'
sha=lambda b:hashlib.sha256(b).hexdigest()
assert sys.float_info.mant_dig==53
collection=json.loads((PACKET/'collection-receipt.json').read_bytes())
assert collection['candidate']==PIN
members={}
with zipfile.ZipFile(PACKET/'artifact.zip') as z:
    manifest_bytes=z.read('manifest.json');assert sha(manifest_bytes)==collection['manifest_sha256']
    manifest=json.loads(manifest_bytes)['files']
    def read(name):
        b=z.read(name);assert sha(b)==manifest[name]['sha256'];members[name]=sha(b);return json.loads(b)
    p=read('linux/corpus/preprocessing/differences.json')
    assert len(p)==160 and read('linux/corpus/preprocessing/transport-differences.json')==[]
    families=Counter()
    for row in p:
        case=row['tag'].split()[0]
        family=next((name for name in ['units','simul','epoch'] if case.startswith(name)),case)
        families[family]+=1
    deleted=[r for r in p if r['field'].endswith('.deleted')]
    assert len(deleted)==16 and all(r['native']==0 and r['rust']==1 for r in deleted)
    prefix='linux/corpus/wxxxxxxxxxxxxxx/validation/input_to_bat/build/planetary/'
    differences=read(prefix+'differences.json');assert len(differences)==3
    assert all('c076_absent_nutation:' in r['path'] and r['path'].endswith('.earth_ssb[1].bits') for r in differences)
    assert len({(r['native'],r['rust']) for r in differences})==1
    case=next(c for c in read(prefix+'cases.json') if c['id']=='c076_absent_nutation')
    offset=next(a for a in case['actions'] if a[0]=='offset')
    assert offset==['offset','0','1','2','50000','3','50020','7','50030','9','50040','11']
    begin=next(e for e in case['events'] if e['event']==12 and e['phase']=='begin')
    sat=begin['caller']['observations'][0]['sat']
    assert sat=={'class':'finite','little_endian_bytes':'000000000000000000000080b4860e40'}
    # This fixed binary128 input is exactly 50010.25, also the pinned probe's seed.
    t0,v0,t1,v1=map(float,offset[4:8]);arrival=50010.25
    m=(v0-v1)/(t0-t1)
    c0=v0-m*t0;c1=math.fma(-m,t0,v0)
    original=m*arrival+c0;fused=math.fma(m,arrival,c1)
    row=differences[0]
    native=struct.unpack('>d',bytes.fromhex(row['native']))[0]
    rust=struct.unpack('>d',bytes.fromhex(row['rust']))[0]
    assert original-fused==native-rust
    source_bindings={}
    for repository,revision,name,needles in [
        (ROOT,PIN,'rust/src/read_ephemeris.rs',[b'let c = (-m).mul_add(t0, v0);',b'offset = m.mul_add(sat, c);']),
        (ROOT,PIN,'validation/planetary_ephemeris/probe.cc',[b'v.sat=parse_longdouble("50010.25")']),
        (ROOT/'tempo2','fda6ad4a9753ad7dc5d291406f2f310a9161423f','readEphemeris.C',[b'ival = m*(double)psr[p].obsn[i].sat+c;'])]:
        data=subprocess.check_output(['git','-C',str(repository),'show',revision+':'+name])
        assert all(needle in data for needle in needles)
        source_bindings[revision+':'+name]=sha(data)
    result={'scope':'Read-only saved-evidence triage and scalar counterfactual; no engine/compiler or acceptance change',
      'candidate':PIN,'archive_sha256':collection['archive_sha256'],'members':members,'sources':source_bindings,
      'preprocessing':{'rows':len(p),'families':dict(families),'field_counts':dict(Counter(r['field'] for r in p)),
                       'deleted_rows':deleted,'transport_differences':0},
      'planetary':{'differences':differences,'source_actions':offset,'sat':sat,'sat_exact':'50010.25',
        'm':m.hex(),'separate_intercept':c0.hex(),'fused_intercept':c1.hex(),
        'separate_offset':original.hex(),'fused_offset':fused.hex(),
        'scalar_delta':(original-fused).hex(),'observed_delta':(native-rust).hex(),
        'exact_delta_agreement':True,'observed_ulp_difference':int(row['native'],16)-int(row['rust'],16),
        'limit':'Explains the observed shift at the scalar level. Actual Linux readEphemeris object/source-profile and neutral before/after engine checks remain required before correction/acceptance.'},
      'python':sys.version,'recipe_sha256':sha(Path(__file__).read_bytes()),'completed_utc':datetime.now(timezone.utc).isoformat()}
    (OUT/'receipt.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({'families':dict(families),'deleted_rows':len(deleted),'planetary_delta':result['planetary']['scalar_delta'],'receipt_sha256':sha((OUT/'receipt.json').read_bytes())},indent=2))
