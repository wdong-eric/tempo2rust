"""Uninstrumented source timing profile; worker proof belongs to native.py."""
from pathlib import Path
import hashlib,json,subprocess,sys
ROOT=Path(__file__).resolve().parents[2]
base=Path(sys.argv[1]).resolve();out=Path(sys.argv[2]).resolve();out.mkdir(parents=True,exist_ok=False)
identity=json.loads((base/'identity.json').read_text());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
for p,h in identity['dependencies'].items():assert sha(p)==h,p
deps=[Path(p) for p in identity['dependencies'] if Path(p).name not in ['tempo2.C.o','libtempo2-reference.a']]
main=next(Path(p) for p in identity['dependencies'] if Path(p).name=='tempo2.C.o')
archive=next(Path(p) for p in identity['dependencies'] if Path(p).name=='libtempo2-reference.a')
calc=(base/'original-calculate_bclt.C').read_text().replace('if (psr[p].correctTroposphere==0) psr[0].obsn[i].troposphericDelay=0;', 'if (psr[p].correctTroposphere==0) psr[p].obsn[i].troposphericDelay=0; // AC selected-pulsar correction.')
(out/'calculate_bclt.C').write_text(calc)
for name in ['dm_delays.C','pipeline.cc']:(out/name).write_bytes((base/name).read_bytes())
(out/'sw_delay.C').write_bytes((base/'original-sw_delay.C').read_bytes())
assert 'ak_visit' not in calc and 'ak_end' not in calc
cxx='/opt/homebrew/bin/g++-16';flags=['-O2','-std=gnu++17','-DHAVE_CONFIG_H','-I'+str(ROOT/'validation/reference/build/source')];commands=[]
def run(label,cmd):
 cmd=list(map(str,cmd));r=subprocess.run(cmd,cwd=out,capture_output=True,timeout=180);(out/(label+'.log')).write_bytes(r.stdout+r.stderr);commands.append(dict(label=label,command=cmd,exit=r.returncode));assert r.returncode==0,(label,r.stderr[-2000:])
for name in ['pipeline','sw_delay']:run(name,[cxx,*flags,'-c',out/(name+('.cc' if name=='pipeline' else '.C')),'-o',out/(name+'.o')])
for mode,extra in [('serial',[]),('parallel',['-fopenmp','-DHAVE_OPENMP'])]:
 objects=[]
 for name in ['calculate_bclt','dm_delays']:
  obj=out/(mode+'-'+name+'.o');run(mode+'-'+name,[cxx,*flags,*extra,'-c',out/(name+'.C'),'-o',obj]);objects.append(obj)
 for target,entry in [('pipeline',out/'pipeline.o'),('tempo2',main)]:
  exe=out/(mode+'-'+target);run(mode+'-'+target,[cxx,*extra,entry,*objects,out/'sw_delay.o',*deps,archive,'-lquadmath','-lgfortran','-Wl,-map,'+str(exe)+'.map','-o',exe]);run(mode+'-'+target+'-linkage',['otool','-L',exe])
 run(mode+'-symbols',['nm','-u',out/(mode+'-calculate_bclt.o')])
 symbols=(out/(mode+'-symbols.log')).read_text();assert ('GOMP_parallel' in symbols)==(mode=='parallel');assert 'ak_' not in symbols
(out/'identity.json').write_text(json.dumps(dict(calibration_identity={str(base/'identity.json'):sha(base/'identity.json')},dependencies=identity['dependencies'],commands=commands,source={str(Path(__file__)):sha(__file__)},artifacts={p.name:sha(p) for p in out.iterdir() if p.is_file()}),indent=2)+'\n')
print('Uninstrumented native serial/OpenMP profiles built')
