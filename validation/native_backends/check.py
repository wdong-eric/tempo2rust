"""Fresh matched-provider execution with quantity gates and artifact integrity checks."""
from pathlib import Path
from decimal import Decimal as D, getcontext
import argparse, hashlib, json, os, shutil, subprocess, sys
getcontext().prec=70
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
RUNTIME=ROOT/'validation/reference/build/source/T2runtime'
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def read(p):
    rows={}
    for line in Path(p).read_text().splitlines():
        key,kind,x=line.split('\t');assert key not in rows,key;rows[key]=(kind,D(x))
    return rows

def matrix_scale_group(key):
    # The observer names a scalar, vector (.../i), or matrix (.../i/j).
    # Strip only numerical indices: qr/rhs and qr/p are distinct vectors,
    # qr/chi is its own scalar, and each matrix column has its own scale.
    parts=key.split('/');indices=[]
    while parts and parts[-1].isdigit():indices.insert(0,parts.pop())
    assert len(indices)<=2,key
    return ('/'.join(parts),len(indices),indices[-1] if len(indices)==2 else None)

def compare(native,rust):
    a=read(native);b=read(rust);fail=[];bits=0
    if a.keys()!=b.keys():fail.append({'membership':{'missing':sorted(a.keys()-b.keys()),'extra':sorted(b.keys()-a.keys())}})
    elif list(a)!=list(b):fail.append({'order':'quantity row order differs'})
    for key,(kind,x) in a.items():
        if key not in b:continue
        k,y=b[key]
        if k!=kind:fail.append({'key':key,'kind':[kind,k]});continue
        if x.is_nan() or y.is_nan() or x.is_infinite() or y.is_infinite():
            if (x.is_nan(),x.is_infinite(),x.is_signed())!=(y.is_nan(),y.is_infinite(),y.is_signed()):fail.append({'key':key,'class':[str(x),str(y)]})
            continue
        if x==0 or y==0:
            if x!=y or x.is_signed()!=y.is_signed():fail.append({'key':key,'zero':[str(x),str(y)]})
            else:bits+=1
            continue
        if x.is_signed()!=y.is_signed():
            fail.append({'key':key,'sign':[str(x),str(y)]});continue
        if float(x).hex()==float(y).hex():bits+=1
        if kind=='exact':tol=D(0)
        elif kind in ('time','position'):tol=D('1e-12')
        elif kind=='velocity':tol=D('1e-15')
        elif kind=='mjd':tol=D('1e-12')/D(86400)
        elif kind=='phase':tol=D('1e-10')
        elif kind=='chi':tol=D('1e-12')+D('1e-6')*abs(x)
        elif kind in ('fit','error'):
            owner,index=key.rsplit('/',2)[0],key.rsplit('/',1)[1];scale=a[owner+'/scale/'+index][1]
            tol=D('1e-11')/scale + (D('1e-10') if kind=='fit' else D('1e-6'))*abs(x)
        elif kind=='cov':
            owner,i,j=key.rsplit('/',3)[0],key.split('/')[-2],key.split('/')[-1]
            tol=D('1e-10')*(abs(a[owner+'/c/'+i+'/'+i][1]*a[owner+'/c/'+j+'/'+j][1])).sqrt()
        elif kind=='matrix':
            # Direct column/vector scales, not mixed-unit pipeline magnitudes.
            group=matrix_scale_group(key)
            cells=[abs(z) for q,(t,z) in a.items() if t=='matrix' and matrix_scale_group(q)==group]
            tol=D('5e-14')*max(cells or [abs(x)])
        else:raise ValueError(kind)
        if abs(x-y)>tol:fail.append({'key':key,'native':str(x),'rust':str(y),'difference':str(abs(x-y)),'gate':str(tol)})
    return {'checks':len(a),'binary64_cells_equal':bits,'failures':fail}

def inputs(folder,eph,calceph):
    folder.mkdir()
    original=(ROOT/'validation/fitting/inputs/f0.par').read_text()
    s=original.replace('PEPOCH 54000','PEPOCH 58000').replace('POSEPOCH 54000','POSEPOCH 58000').replace('DMEPOCH 54000','DMEPOCH 58000')
    if calceph:s+='EPH_FILE '+str(eph)+'\n'
    variants={'calceph':s,'tdb':s.replace('UNITS TCB','UNITS TDB'),'ecliptic':s.replace('RAJ 01:00:00.00000\nDECJ -00:00:00.00000','ELONG 15.0\nELAT -5.0'),'robust':s+'ROBUST H\n','prior':s+'CONSTRAIN PARAM F0 10.12345678902 0.01\n','global0':s.replace('F0 10.12345678902 1','F0 10.12345678902 2'),'global1':s.replace('F0 10.12345678902 1','F0 10.12345678902 2').replace('PSRJ J0000-0000','PSRJ J0000-0001')}
    for k,v in variants.items():(folder/(k+'.par')).write_text(v)
    for name,start,n in [('pks',58000,12),('pks2',58002,10)]:
        (folder/(name+'.tim')).write_text('FORMAT 1\n'+'\n'.join(f'toa{i} {1000+100*(i%4)} {start+i*0.27:.12f} {1+i%4} pks' for i in range(n))+'\n')
    return {str(p):sha(p) for p in folder.iterdir()}

def run(out,native,rust,calceph,blas,lapack,eph):
    out=out.resolve();out.mkdir(parents=True,exist_ok=False);records=[];files={};env={**os.environ,'TEMPO2':str(RUNTIME)}
    # Cargo profiles reuse target paths; compare immutable per-run executables.
    binaries=out/'bin';binaries.mkdir()
    for name,src in [('native',native),('rust',rust)]:shutil.copy2(src,binaries/name)
    native=binaries/'native';rust=binaries/'rust'
    frozen={str(p):sha(p) for p in [native,rust,eph,HERE/'CONTRACT.md',Path(__file__)]}
    frozen.update({str(p):sha(p) for p in [*ROOT.glob('rust/src/**/*.rs'),ROOT/'rust/build.rs',ROOT/'rust/Cargo.toml',ROOT/'rust/native/backends.c',ROOT/'rust/examples/native_backends.rs']})
    def case(name,mode,args):
        folder=out/name;folder.mkdir();rec={'case':name,'commands':[]};records.append(rec)
        for engine,exe in [('native',native),('rust',rust)]:
            dest=folder/(engine+'.tsv');cmd=[str(exe),mode,str(dest),*map(str,args)]
            r=subprocess.run(cmd,cwd=ROOT,env=env,capture_output=True,timeout=90);rec['commands'].append({'argv':cmd,'exit':r.returncode})
            (folder/(engine+'.stdout')).write_bytes(r.stdout);(folder/(engine+'.stderr')).write_bytes(r.stderr)
        if all(r['exit']==0 for r in rec['commands']):rec['comparison']=compare(folder/'native.tsv',folder/'rust.tsv')
        else:rec['execution_failure']=True
        for p in folder.iterdir():files[str(p)]=sha(p)
        print(name, len(rec.get('comparison',{}).get('failures',[])) if 'comparison'in rec else 'execution failure',flush=True)
    case('kernels','kernels',[])
    if lapack:
        case('failures','failures',[])
        for c in [0,1]:
            for scale in [0,1]:
                for mask in range(8):case(f'solver-{c}-{scale}-{mask}','solver',[c,scale,mask])
    if calceph:
        for units in [0,1]:
            for ecl in [0,1]:case(f'calceph-{units}-{ecl}','calceph',[eph,units,ecl])
    frozen.update(inputs(out/'inputs',eph,calceph))
    for mode in [0,1,2]:
        for par in ['calceph','tdb','ecliptic','prior','robust']:
            case(f'pipeline-{par}-{mode}','pipeline',[mode,'-',out/'inputs'/f'{par}.par',out/'inputs/pks.tim'])
        case(f'pipeline-global-{mode}','pipeline',[mode,'-',out/'inputs/global0.par',out/'inputs/pks.tim',out/'inputs/global1.par',out/'inputs/pks2.tim'])
        case(f'pipeline-covariance-{mode}','pipeline',[mode,ROOT/'validation/general_fitting/inputs/covariance.model',out/'inputs/calceph.par',out/'inputs/pks.tim'])
    changed=[p for p,h in {**frozen,**files}.items() if not Path(p).exists() or sha(p)!=h]
    receipt={'sources_and_inputs':frozen,'files':files,'records':records,'changed_artifacts':changed,'failures':sum(len(r.get('comparison',{}).get('failures',[]))+bool(r.get('execution_failure')) for r in records)}
    (out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');return receipt
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('out',type=Path);p.add_argument('--native',type=Path,required=True);p.add_argument('--rust',type=Path,required=True);p.add_argument('--profile',choices=['disabled','calceph','blas','lapack','combined'],default='combined');p.add_argument('--ephemeris',type=Path,default=HERE/'build/inpop17a_TDB_m100_p100_tt.dat');a=p.parse_args();r=run(a.out,a.native.resolve(),a.rust.resolve(),a.profile in ('calceph','combined'),a.profile in ('blas','lapack','combined'),a.profile in ('lapack','combined'),a.ephemeris.resolve());print('FAILURES',r['failures']);sys.exit(bool(r['failures'] or r['changed_artifacts']))
