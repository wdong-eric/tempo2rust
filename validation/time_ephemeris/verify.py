"""Normal comparison/identity controls, independent checks, compiled behavioral witnesses."""
import argparse
import copy
import json
from pathlib import Path
import struct
import subprocess
from profile import HERE, BUILD, ENV, sha, save, compile_probe, check_live
from fixtures import catalog, membership
from inventory import inventory
from run import OUT, REF, unpack, pack, projected, compare, launch

def value(x):return struct.unpack('<d',bytes.fromhex(x['bits']))[0]

def independent():
    checks=[]
    # Closed-form polynomials, not a second copy of the recurrence.
    for n in [0,1,2,3,4,18]:
        raw=unpack(REF/('interp_count_'+str(n)+'.json.gz'))
        events=[e for e in projected(raw)['events'] if e['phase']=='end']
        for t,event in zip([0,.25,.5,.5,1],events):
            x=2*t-1
            expect=0 if n==0 else 2 if n==1 else 2+3*x if n==2 else 2+3*x+5*(2*x*x-1)
            deriv=0 if n<2 else 1.5 if n==2 else (3+20*x)/2
            got=event['state']['output']
            assert value(got[0])==expect and value(got[1])==deriv,(n,t,got[:2])
            checks.append(dict(case=raw['id'],t=t,value=expect,derivative=deriv))
    # At these dyadic dates FB count=3 is a quadratic in the selected block.
    raw=unpack(REF/'fb_count_3.json.gz');ends=[e for e in projected(raw)['events'] if e['phase']=='end' and e['op']=='fb']
    for i,t in [(0,50000),(2,50000.5),(4,50002),(5,50004),(6,50016)]:
        # Record is selected from the half-day part; exact endpoints use next block.
        jda=int(t+2400000.5)+(.5 if (t+2400000.5)%1>=.5 else -.5)
        nr=int((jda-2450000.5)/4)+2
        x=2*((t-50000-(nr-2)*4)/4)-1
        expect=(nr-1)*(1+2*x+3*(2*x*x-1))/1024
        assert value(ends[i]['state']['output'][0])==expect
        checks.append(dict(case=raw['id'],mjd=t,value=expect))
    # Big/little IFTE arithmetic, cache cells and outputs must agree, aside from swap flag.
    a=projected(unpack(REF/'ifte_big.json.gz'));b=projected(unpack(REF/'ifte_little.json.gz'))
    for x,y in zip(a['events'],b['events']):
        if 'state' not in x:continue
        for k in ['ifte.buf','ifte.pc','ifte.vc','ifte.twot','ifte.ipt0','ifte.ipt1','output']:
            assert x['state'][k]==y['state'][k],k
    checks.append(dict(case='ifte_big/little',check='exact matching arithmetic and retained arrays'))
    return checks

MUTATIONS=[
 ('ifte_record_cache','ifteph.C','if (ifte.swap_endian) IFTswapDoubles(ifte.buf, ncoeff);',
  'if (ifte.swap_endian) IFTswapDoubles(ifte.buf, ncoeff); ifte.irec=irec;','ifte_reinit'),
 ('ifte_reinit_clear','ifteph.C','ifte.iinfo.np = 2;',
  'ifte.iinfo.np = 2; ifte.iinfo.twot=0; memset(ifte.iinfo.pc+2,0,16*sizeof(double)); memset(ifte.iinfo.vc+2,0,16*sizeof(double));','ifte_reinit'),
 ('ifte_subinterval','ifteph.C','l = (int)(temp - dt1);','l = 0;','interp_components_2'),
 ('ifte_derivative_scale','ifteph.C','vfac=(dna+dna)/t[1];','vfac=(dna+dna);','interp_count_3'),
 ('fb_halfday','tt2tdb.C','if (tdt - (int)tdt >= 0.5)','if (tdt - (int)tdt >= 0.75)','fb_cache'),
 ('fb_file_cache','tt2tdb.C','if (nr!=tdbnrl)','if (true)','fb_cache'),
 ('fb_nr1','tt2tdb.C','j<nr-1;j++','j<nr;j++','fb_nr1_cold'),
 ('bypass_retention','tt2tdb.C','psr[p].obsn[i].correctionTT_TB = 0.0;',
  '{ psr[p].obsn[i].correctionTT_TB = 0.0; psr[p].obsn[i].correctionTT_Teph=0; psr[p].obsn[i].einsteinRate=0; }','tt_bypass_PKS'),
 ('calceph_pulsar','tt2tdb.C','deltaT = psr[0].obsn[i].correctionTT_calcEph;',
  'deltaT = psr[p].obsn[i].correctionTT_calcEph;','tt_multi_calc_1'),
 ('obstermdot_scaling','tt2tdb.C',
  'obsTerm /= (double) (IFTE_K*IFTE_K); // both were in SI \n                    else\n                        obsTerm /= (double)IFTE_K;',
  'obsTermDot /= (double) (IFTE_K*IFTE_K); // both were in SI \n                    else\n                        obsTermDot /= (double)IFTE_K;','tt_e1_u1'),
 ('deltaT_retention','tt2tdb.C','if (psr[p].timeEphemeris == IF99_TIMEEPH)',
  'deltaT=0; if (psr[p].timeEphemeris == IF99_TIMEEPH)','tt_delta_retention'),
 ('record_swap','read_fortran.h','return ival;','if (swapByte==1) IFTswapInt(&ival); return ival;','fortran_record_big'),
 ('wrapper_alias','ifteph.C','vE[i] = res[i];\n        vEDot[i] = res[i+3];',
  'vE[i] = res[i];\n    }\n    for (i=0;i<3;i++) { vEDot[i] = res[i+3];','ifte_big'),
]

def controls(exe,profile):
    results=[]
    def reject(name,fn):
        try:fn()
        except (AssertionError,ValueError,KeyError) as error:
            results.append(dict(name=name,rejected=True,reason=str(error)[:300]));return
        raise AssertionError('control accepted: '+name)
    case=next(c for c in catalog() if c['id']=='ifte_reinit')
    frozen=projected(unpack(REF/'ifte_reinit.json.gz'))
    current=projected(launch(exe,profile,case));compare(frozen,current)
    results.append(dict(name='fresh positive launch',passed=True))
    edits={
      'missing case':lambda x:x.pop('id'),
      'missing event':lambda x:x['events'].pop(3),
      'action order':lambda x:x['events'].reverse(),
      'missing field':lambda x:x['events'][1]['state'].pop('ifte.pc'),
      'field order':lambda x:x['events'][1].update(state=dict(reversed(list(x['events'][1]['state'].items())))),
      'retained tail':lambda x:x['events'][1]['state']['output'][-1].update(bits='0000000000000000'),
      'knownness':lambda x:x['events'][1]['state'].update({'ifte.title':{'unwritten':True}}),
      'cache':lambda x:x['events'][5]['state'].update({'ifte.irec':2}),
      'owner lifetime':lambda x:x['events'][7]['state'].update({'old0.owner':'released-by-caller'}),
      'signed zero':lambda x:x['events'][1]['state']['ifte.twot'].update(bits='0000000000000080'),
      'numeric class':lambda x:x['events'][1]['state']['ifte.twot'].update({'class':'NaN'}),
      'stdout':lambda x:x.update(stdout='20'),
      'stderr':lambda x:x.update(stderr='20'),
      'status':lambda x:x.update(status=1),
      'resource file':lambda x:x['final_files'].update(eph='0'*64),
    }
    for name,edit in edits.items():
        changed=copy.deepcopy(frozen);edit(changed)
        reject(name,lambda:compare(frozen,changed))
    bad=unpack(REF/'ifte_reinit.json.gz');bad['nonce']='wrong';reject('launch challenge',lambda:projected(bad))
    for name,edit in [
       ('executable identity',lambda p:p.update(executable_sha256='0'*64)),
       ('compiler identity',lambda p:p['stable'].update(compiler_sha256='0'*64)),
       ('runtime identity',lambda p:p['stable']['runtime']['libquadmath.0.dylib'].update(sha256='0'*64)),
       ('raw source identity',lambda p:p['stable']['sources'].update({'ifteph.C':'0'*64})),
       ('checkout identity',lambda p:p['stable']['checkout_sources'].update({'tt2tdb.C':'0'*64})),
       ('runner identity',lambda p:p['stable']['runner_sources'].update({'run.py':'0'*64})),
       ('config identity',lambda p:p['stable']['generated'].update({'config.h':'0'*64})),
       ('header identity',lambda p:p['stable']['header_dependencies'].update({next(iter(p['stable']['header_dependencies'])):'0'*64})),
    ]:
        p=copy.deepcopy(profile);edit(p);reject(name,lambda:check_live(exe,p))
    actual=BUILD/'changed-executable';actual.write_bytes(exe.read_bytes()+b'control')
    reject('actual executable changed',lambda:check_live(actual,profile));actual.unlink()
    meta=json.loads((REF/'oracle.json').read_text())['inputs'];m=copy.deepcopy(meta);m['cases'].pop()
    reject('case membership',lambda:compare(meta,m));m=copy.deepcopy(meta);m['cases'][0]['actions'][0][0]='other'
    reject('input action identity',lambda:compare(meta,m))
    m=copy.deepcopy(meta);m['cases'][-1]['files'][next(iter(m['cases'][-1]['files']))]['sha256']='0'*64
    reject('input resource identity',lambda:compare(meta,m))
    receipt=OUT/'last_execution.json';receipt.write_text('{"stale_success":true}')
    p=subprocess.run(['python3',str(HERE/'run.py'),'--no-build'],capture_output=True,env=ENV)
    assert p.returncode!=0 and not receipt.exists(),'unsupported shortcut/stale receipt'
    results.append(dict(name='CLI no-build rejected and stale receipt removed',status=p.returncode,stderr=p.stderr.decode()))
    return results

def mutations():
    cases={c['id']:c for c in catalog()};results=[]
    for name,file,old,new,case in MUTATIONS:
        exe,profile=compile_probe('mutation-'+name,(file,old,new))
        raw=launch(exe,profile,cases[case]);expected=projected(unpack(REF/(case+'.json.gz')));actual=projected(raw)
        try:compare(expected,actual)
        except AssertionError as e:witness=str(e)
        else:raise AssertionError('no behavioral witness '+name)
        assert expected['events'][0]==actual['events'][0],'profile-only difference'
        path=OUT/'mutations'/(name+'.json.gz');pack(path,raw)
        results.append(dict(name=name,case=case,profile=profile,witness=witness,path=str(path),sha256=sha(path.read_bytes())))
        print('mutation witnessed',name,witness[:150],flush=True)
    return results

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--mutations',action='store_true');args=parser.parse_args()
    frozen=json.loads((REF/'oracle.json').read_text())
    compare(frozen['inputs'],dict(cases=membership(catalog()),inventory=inventory()))
    exe,profile=compile_probe();compare(frozen['profile'],profile['stable'])
    report=dict(profile=profile,controls=controls(exe,profile),independent_polynomials=independent())
    if args.mutations:report['mutations']=mutations()
    save(OUT/'verification.json',report)
    print('PASS controls',len(report['controls']),'independent',len(report['independent_polynomials']),'mutations',len(report.get('mutations',[])))
