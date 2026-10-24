"""One-shot pre-experiment source/caller/field/case/profile freeze."""
import hashlib
import json
import re
import subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent
ROOT=HERE.parent.parent
PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
RELEASE='7946aeee6e7e105fc276b311f1c36a4288767d0d'
PRIMARY={'readEphemeris.C':['readEphemeris','random2','gasdev']}
FILES=['readEphemeris.C','jpleph.c','jpleph.h','jpl_int.h','ifteph.h','tempo2.h','clkcorr.C',
       'tabulatedfunction.C','tabulatedfunction.h','observatory.C','TKlog.C','TKlog.h',
       'TKlongdouble.C','TKlongdouble.float128.h','tempo2Util.C','global.C']
PROFILE=dict(cxx='/opt/homebrew/bin/g++-16',cc='/opt/homebrew/bin/gcc-16',
             cxx_flags=['-O2','-std=gnu++11','-DHAVE_CONFIG_H'],c_flags=['-O2','-std=c11'],
             binary128='TKlongdouble.float128.h',config_sha256='470c61b68a61504558e76ef772639e46b565a1e4500115d6e3fb6a71286dd69e',
             global_slices=['before void extra_delays','from void CVSdisplayVersion to EOF'],
             primary_own_translation_unit=True,dependency_patches=[],locale='C',link=['-lquadmath'])
def sha(b):return hashlib.sha256(b).hexdigest()
def blob(n):return subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':'+n])
def save(p,x):
    p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(x,indent=2,allow_nan=False)+'\n')
def uncomment(s):
    return re.sub(r'"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\'|//[^\n]*|/\*[\s\S]*?\*/',lambda m:re.sub(r'[^\n]',' ',m[0]) if m[0].startswith('/') else m[0],s)
def source_map():
    from fixtures import VECTORS,PLANETS
    raw=blob('readEphemeris.C').decode();text=uncomment(raw)
    assert sha(raw.encode())=='822678396155955e8f55f307b373d1666b4def988d0b21e0762bf6fd5e53ce3a'
    text=re.sub(r'#if 0\b.*?#endif',lambda m:re.sub(r'[^\n]',' ',m[0]),text,flags=re.S)
    routines=[]
    for name in PRIMARY['readEphemeris.C']:
        m=re.search(r'\b'+name+r'\s*\([^;{}]*\)\s*\{',text);assert m
        end=m.end();depth=1
        while depth:depth+=(text[end]=='{')-(text[end]=='}');end+=1
        lo=text[:m.start()].count('\n')+1;hi=text[:end].count('\n')+1
        routines.append(dict(file='readEphemeris.C',name=name,start=lo,end=hi,body_sha256=sha(raw.splitlines(keepends=True)[lo-1:hi].__str__().encode()),
          source=raw.splitlines()[lo-1:hi],decisions=[dict(line=i+1,text=l.strip()) for i,l in enumerate(text.splitlines()) if lo<=i+1<=hi and re.search(r'\b(if|else|for|while)\b|\?',l)]))
    paths=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'ls-tree','-r','--name-only',PIN],text=True).splitlines()
    callers=[]
    for path in paths:
      if path.endswith(('.C','.h','.c','.cpp')):
        for i,line in enumerate(uncomment(blob(path).decode(errors='replace')).splitlines()):
          hits=[n for n in PRIMARY['readEphemeris.C'] if re.search(r'\b'+n+r'\s*\(',line)]
          if hits:callers.append(dict(file=path,line=i+1,routines=hits,text=line.strip()))
    types=json.loads((ROOT/'validation/pulsar_lifecycle/inventory.json').read_text())['types']
    return dict(pin=PIN,release=RELEASE,hashes={f:sha(blob(f)) for f in FILES},routines=routines,callers=callers,
        preprocessor=[dict(line=i+1,text=l) for i,l in enumerate(raw.splitlines()) if l.lstrip().startswith('#')],
        inactive=['literal #if 0 Shapiro experiment','commented planetary phase Earth adjustment'],
        types=types,read_fields=sorted(set(re.findall(r'psr\[p\]\.([A-Za-z_0-9]+)',text))),
        observation_fields=sorted(set(re.findall(r'obsn\[i\]\.([A-Za-z_0-9]+)',text))),
        changed_fields=list(VECTORS+PLANETS),private_fields=dict(random2=['r[100]','result','iff','ix1','ix2','ix3'],gasdev=['iset','gset']),
        rust_context={f:sha(subprocess.check_output(['git','-C',str(ROOT),'show',RELEASE+':'+f])) for f in ('rust/src/read_ephemeris.rs','rust/src/form_bats.rs')})
def protected_paths():
    # X's accepted preservation membership already covers older references. Exclude
    # every evolving central/W/Y path; add X's now accepted evidence explicitly.
    old=json.loads((ROOT/'validation/observer_coordinates/outputs/frozen/protected.json').read_text())
    paths={ROOT/p for p in old if not p.startswith(('rust/','decisions/','validation/terrestrial_resources_port/','validation/time_ephemeris_port/'))}
    x=ROOT/'validation/observer_coordinates'
    paths.update(p for p in x.rglob('*') if p.is_file() and not any(q in ('build','__pycache__') for q in p.relative_to(x).parts))
    return sorted(paths)
def repositories():
    return {n:dict(head=subprocess.check_output(['git','-C',str(ROOT/n),'rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','-C',str(ROOT/n),'status','--porcelain'],text=True)) for n in ('tempo2','validation/reference')}
def freeze():
    from fixtures import cases,UNRUN,RESOURCES
    out=HERE/'outputs/frozen';assert not out.exists(),'one-shot freeze; preserve prior evidence'
    save(out/'source_map.json',source_map());save(out/'cases.json',cases());save(out/'unrun.json',UNRUN)
    save(out/'profile.json',PROFILE);save(out/'resources.json',RESOURCES)
    save(out/'protected.json',{str(p.relative_to(ROOT)):sha(p.read_bytes()) for p in protected_paths()})
    r=repositories();assert all(v['status']=='' for v in r.values());save(out/'repositories.json',r)
    save(out/'contract.json',{'sha256':sha((HERE/'CONTRACT.md').read_bytes())})
    print('Frozen 3 routines, complete types/branches/callers, profile and',len(cases()),'cases')
def check():
    from fixtures import cases,UNRUN,RESOURCES
    p=HERE/'outputs/frozen'
    for name,current in [('source_map',source_map()),('cases',cases()),('unrun',UNRUN),('profile',PROFILE),('resources',RESOURCES),('repositories',repositories()),('contract',{'sha256':sha((HERE/'CONTRACT.md').read_bytes())})]:
        assert json.loads((p/(name+'.json')).read_text())==current,'frozen '+name+' changed'
    for n,h in json.loads((p/'protected.json').read_text()).items():assert sha((ROOT/n).read_bytes())==h,'accepted evidence changed: '+n
if __name__=='__main__':freeze()
