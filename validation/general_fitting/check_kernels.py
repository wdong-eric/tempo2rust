"""Covariance helper comparison and guarded FFT initialization witness (0053)."""
from pathlib import Path
from decimal import Decimal as D, getcontext
import argparse, difflib, hashlib, json, math, subprocess
from fitting_corrections import corrected

ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent
BUILD=HERE/'build/kernels';REF=ROOT/'validation/reference/build'
PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
getcontext().prec=70
KINDS=['power','beta','qp','redpink','sqexp','matern15','matern25','dmparam','dm','mask']

def read_native(text):
    values={}
    for row in text.splitlines():
        if not row.startswith('VALUE '):continue
        _,key,i,j,v=row.split();index=(key,int(i),int(j))
        assert index not in values,index
        values[index]=D(v)
    assert values
    return values

def compare(reference, actual):
    flat={}
    for name,v in actual.items():
        if isinstance(v,list):
            for i,row in enumerate(v):
                if isinstance(row,list):
                    for j,value in enumerate(row):flat[name,i,j]=D(value)
                else:flat[name,i,-1]=D(row)
        else:flat[name,-1,-1]=D(v)
    assert flat.keys()==reference.keys(),(flat.keys()-reference.keys(),reference.keys()-flat.keys())
    failures=[]
    for (name,i,j),ref in reference.items():
        got=flat[name,i,j];group=[abs(v) for (k,_,c),v in reference.items() if k==name and (j<0 or c==j)]
        scale=max(group)
        if name in ['covariance','positive']:
            scale=(reference[name,i,i]*reference[name,j,j]).sqrt()
        elif name=='determinant':scale=max(D(1),abs(ref))
        exact=ref==0 or name in ['masked','unmasked','sigma']
        gate=D(0) if exact else D('5e-14')*scale
        classes=ref.is_finite() and got.is_finite()
        signed_zero=ref!=0 or got!=0 or ref.is_signed()==got.is_signed()
        if not classes or not signed_zero or abs(ref-got)>gate:
            failures.append({'field':[name,i,j],'reference':str(ref),'rust':str(got),'gate':str(gate)})
    return failures

def run(profile):
    BUILD.mkdir(parents=True,exist_ok=True)
    base=['/opt/homebrew/bin/g++-16','-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(REF/'source')]
    commands=[];sources={};boundaries=[];records=[]
    for version in ['original','corrected']:
        folder=BUILD/version;folder.mkdir(exist_ok=True);objects=[]
        for name in ['cholesky.C','choleskyRoutines.C']:
            old=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':'+name]).decode()
            source=corrected(name,old) if version=='corrected' else old
            (folder/(name+'.patch')).write_text(''.join(difflib.unified_diff(old.splitlines(True),source.splitlines(True),fromfile='original/'+name,tofile=version+'/'+name)))
            if name=='choleskyRoutines.C':
                # Track initialization, without loading the omitted source value.
                token='    p_i=(double*)malloc(sizeof(double)*npts);'
                assert source.count(token)==3
                source=source.replace(token,token+'\n    bool ae_dc_initialized=false;')
                source=source.replace('    p_i[0]=0;', '    p_i[0]=0; ae_dc_initialized=true;')
                token='    TK_fft(1,npts,p_r,p_i);'
                assert source.count(token)==3
                source=source.replace(token,'    extern void ae_dc_guard(bool);ae_dc_guard(ae_dc_initialized);\n'+token)
            else:
                token='void cholesky_covarFunc2matrix(double** m, double* covarFunc, int ndays,double *resx,double *resy,double *rese,int np, int nc){'
                assert source.count(token)==1
                source=source.replace(token,token+'\n    extern void ae_kernel(double*,int);ae_kernel(covarFunc,ndays);')
            path=folder/name;path.write_text(source);obj=folder/(name+'.o');objects.append(obj)
            cmd=base+['-c',str(path),'-o',str(obj)];p=subprocess.run(cmd,capture_output=True);assert p.returncode==0,p.stderr.decode();commands.append(cmd);sources[str(path)]=sha(path)
        exe=folder/'native';cmd=base+[str(HERE/'kernels_probe.cc'),*map(str,objects),str(REF/'libtempo2-reference.a'),'-lquadmath','-lgfortran','-o',str(exe)]
        p=subprocess.run(cmd,capture_output=True);assert p.returncode==0,p.stderr.decode();commands.append(cmd)
        for kind in KINDS if version=='corrected' else KINDS[:4]:
            p=subprocess.run([str(exe),kind],capture_output=True,timeout=30)
            path=folder/(kind+'.txt');path.write_bytes(p.stdout+p.stderr)
            assert p.returncode==(86 if version=='original' else 0),(kind,p.returncode,p.stderr.decode())
            if version=='original':
                assert 'DC 0' in p.stdout.decode() and 'VALUE ' not in p.stdout.decode()
                boundaries.append({'kind':kind,'exit':p.returncode,'output_sha256':sha(path),'executable_sha256':sha(exe)})
                continue
            rust=ROOT/'rust/target'/profile/'examples/fitting_kernels'
            raw=subprocess.check_output([str(rust),kind],timeout=30);got=json.loads(raw)
            rp=folder/(kind+'-'+profile+'.json');rp.write_bytes(raw)
            reference=read_native(p.stdout.decode());failures=compare(reference,got)
            records.append({'kind':kind,'fields':len(reference),'failures':failures,'native_output_sha256':sha(path),'rust_output_sha256':sha(rp)})
    # A missing export and a changed structural zero must not pass this checker.
    controls={('x',0,-1):D(0)}
    try:compare(controls,{})
    except AssertionError:pass
    else:raise AssertionError('missing field passed')
    assert compare(controls,{'x':['1e-30']}) and compare(controls,{'x':['-0']})
    receipt={'pin':PIN,'profile':profile,'commands':commands,'sources':sources,'builder_sha256':sha(__file__),
        'probe_sha256':sha(HERE/'kernels_probe.cc'),'corrections_sha256':sha(HERE/'fitting_corrections.py'),
        'archive_sha256':sha(REF/'libtempo2-reference.a'),'rust_sha256':sha(rust),'native_sha256':sha(BUILD/'corrected/native'),
        'original_boundaries':boundaries,'records':records,'rejection_controls':['missing field','changed zero','signed zero']}
    (BUILD/(profile+'.json')).write_text(json.dumps(receipt,indent=2)+'\n')
    print('kernel failures',sum(len(r['failures'])for r in records))
    for r in records:
        if r['failures']:print(r['kind'],r['failures'][:3])

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--profile',choices=['debug','release'],default='debug');run(p.parse_args().profile)
