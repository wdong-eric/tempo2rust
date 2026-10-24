"""Small analytical and source-contract checks; never replacement golden values."""
import math
import struct
from inventory import save,sha,preservation_status
from run import OUTPUT,read_gzip,frozen_cases,compare

def value(x):return struct.unpack('>d',bytes.fromhex(x['bits']))[0]
def main():
    dest=OUTPUT/'independent.json';dest.unlink(missing_ok=True);before=preservation_status()
    catalog=frozen_cases();results=read_gzip(OUTPUT/'reference.json.gz');compare([c['id'] for c in catalog],[r['id'] for r in results])
    by={c['label']:r for c,r in zip(catalog,results)};checks=[]
    def ends(label,op):return [e for e in by[label]['events'] if e['phase']=='end' and e['operation']==op]
    v=ends('vector_ecl_0','vector')[0]['caller']['pulsars'][0]
    assert [value(x) for x in v['position']]==[1,0,0]
    checks.append(dict(name='cardinal_position',max_absolute_difference=0))
    for e in ends('precession','precession'):
        a=catalog[[c['label'] for c in catalog].index('precession')]['actions'][e['event']]
        if a[1:]!=['51544.5','0','0']:continue
        m=[[value(x) for x in row] for row in e['result']['matrix']]
        assert m==[[1,0,0],[0,1,0],[0,0,1]];checks.append(dict(name='J2000_zero_nutation_identity',max_absolute_difference=0))
    for selector in (-3,0,1,2,3,4,5):
        e=ends('ang_selector_'+str(selector),'ang')[4]
        expected=(1/360)*(15 if selector>2 else 1)*(2*math.pi if selector in (1,3) else 1)
        d=abs(value(e['result']['value'])-expected);assert d<1e-15
        checks.append(dict(name='packed_one_degree_selector_'+str(selector),absolute_difference=d,diagnostic_bound=1e-15))
    for e in ends('iau_direct','iau'):
        r=e['result'];pn=math.sqrt(sum(value(x)**2 for x in r['position']));tn=math.sqrt(sum(value(x)**2 for x in r['input_position']))/299792458
        zn=math.sqrt(sum(value(x)**2 for x in r['zenith']));zin=math.sqrt(sum(value(x)**2 for x in r['input_zenith']))
        assert abs(pn-tn)<1e-15 and abs(zn-zin)<1e-12
        checks.append(dict(name='SOFA_rotation_norms_'+str(e['event']),position_difference_light_seconds=pn-tn,zenith_difference=zn-zin,diagnostic_bounds=[1e-15,1e-12]))
    vals=[e['result']['value'] for e in ends('ut1_cache','ut1')]
    for x in vals[1:]:compare(vals[0],x)
    checks.append(dict(name='UT1_file_root_change_retains_cache',exact=True))
    first=ends('ut1_interpolation','ut1')[0]['result']['value'];assert value(first)==-310000*1e-4
    checks.append(dict(name='UT1_TAI_sign_and_lower_endpoint',exact=True))
    original=ends('imag_shared','lookup')[0]['result']['coordinates'];later=ends('imag_shared','lookup')[1]['result']['coordinates']
    compare(original[3:],later[3:]);assert original[:3]!=later[:3]
    checks.append(dict(name='IMAG_xyz_changes_geodetic_retained',exact=True))
    cache=ends('iau_eop_cache','iau');r=cache[0]['result']
    for e in cache[1:]:compare(r,e['result'])
    checks.append(dict(name='EOP_file_root_change_retains_cache',exact=True))
    x=ends('satellite_spelling_2','driver')[0]['caller']['pulsars'];assert x[0]['troposphere']==0 and x[1]['troposphere']==1
    checks.append(dict(name='STL_BAT_pulsar_local_side_effect',exact=True))
    after=preservation_status();save(dest,dict(status='PASS' if not before and not after else 'PRESERVATION_DRIFT',preservation_before=before,preservation_after=after,checks=checks,oracle_sha256=sha((OUTPUT/'reference.json.gz').read_bytes())))
    assert not before and not after
    print('PASS',len(checks),'independent supporting checks')

if __name__=='__main__':main()
