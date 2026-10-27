"""Build disposable enabled/disabled native oracles from pinned source + AH dependencies."""
from pathlib import Path
import argparse, difflib, hashlib, json, subprocess, sys
ROOT=Path(__file__).resolve().parents[2]; HERE=Path(__file__).resolve().parent
PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f'; REF=ROOT/'validation/reference/build'
CXX='/opt/homebrew/bin/g++-16'
sys.path.insert(0,str(ROOT/'validation/general_fitting'))
from fitting_corrections import corrected

def sha(p): return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def raw(name): return subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':'+name],text=True)

def build(out, profile, provider):
    out=out.resolve(); out.mkdir(parents=True,exist_ok=False); source=out/'source';source.mkdir()
    base=ROOT/'validation/core_output/build/native-v9/identity.json';identity=json.loads(base.read_text())
    deps={}
    for name,digest in identity['objects'].items():
        p=Path(name);assert sha(p)==digest,p;deps[p.name]=p
    archive=REF/'libtempo2-reference.a';assert sha(archive)==identity['archive']
    blas=profile in ('blas','lapack','combined','qr-default');lapack=profile in ('lapack','combined','qr-default');calceph=profile in ('calceph','combined','qr-default')
    config=(REF/'source/config.h').read_text()+'\n#undef HAVE_BLAS\n#undef HAVE_LAPACK\n#undef HAVE_CALCEPH\n#undef QR_DEFAULT\n#undef F77_FUNC\n#define F77_FUNC(name,NAME) name ## _\n'
    for macro,active in [('HAVE_BLAS',blas),('HAVE_LAPACK',lapack),('HAVE_CALCEPH',calceph),('QR_DEFAULT',profile=='qr-default')]:
        if active:config+=f'#define {macro} 1\n'
    (source/'config.h').write_text(config);(source/'T2accel.h').write_text(raw('T2accel.h'))
    commands=[];sources={};patch=''
    def run(cmd,label):
        r=subprocess.run(list(map(str,cmd)),capture_output=True);(out/(label+'.log')).write_bytes(r.stdout+r.stderr);commands.append(list(map(str,cmd)));assert r.returncode==0,(label,r.stderr.decode()[-3000:])
    def compile(name,text,original=None):
        nonlocal patch
        p=source/name;p.write_text(text)
        if original is not None: patch+=''.join(difflib.unified_diff(original.splitlines(True),text.splitlines(True),fromfile='pinned/'+name,tofile='compiled/'+name))
        sources[name]={'original':hashlib.sha256((original or text).encode()).hexdigest(),'compiled':sha(p)}
        obj=out/(name+'.o');run([CXX,'-O2','-DHAVE_CONFIG_H','-I'+str(source),'-I'+str(REF/'source'),'-I'+str(provider/'include'),'-c',p,'-o',obj],name);return obj
    for name in ['TKfit.C','TKmatrix.C','T2accel.C','cholesky.C','readEphemeris_calceph.C','tt2tdb.C','global.C']:
        original=raw(name);text=corrected(name,original)
        if name=='TKfit.C':
            text='extern void (*ai_matrix_observer)(int,int,double**);\n'+text
            text=text.replace('    double chisq = 0;','    if(ai_matrix_observer)ai_matrix_observer(ndata,nparams,designMatrix);\n    double chisq = 0;',1)
            start=text.index('        double **augmented_DM=NULL;');end=text.index('        for (i=0;i<ndata;i++){\n            for (j=0;',start)
            text=text[:start]+'''        // 0093: independent matrix and real RHS regardless of parameter output.
        double **augmented_DM=malloc_blas(ndata+nconstraints,nparams);
        for (i=0;i<ndata;i++) augmented_white_data[i]=white_data[i];
'''+text[end:]
        if name=='T2accel.C':
            start=text.index('#define F77_dgemm');end=text.index('\n\nextern "C" {\n    extern void F77_dpotf2',start)
            declarations=text[start:end];text=text[:start]+text[end:]
            pos=text.index('#ifdef HAVE_LAPACK');text=text[:pos]+'#ifdef HAVE_BLAS\n'+declarations+'\n#endif\n'+text[pos:]
        if name=='tt2tdb.C':text=text.replace('deltaT = psr[0].obsn[i].correctionTT_calcEph;','deltaT = psr[p].obsn[i].correctionTT_calcEph; // Y accepted current-pulsar correction.')
        if name=='readEphemeris_calceph.C':
            # Preserve supplied inactive arrays; poison-initialized probe gives
            # an original defined witness. Normal initialized native inputs are
            # left as source reads; never infer a golden from those fields.
            pass
        deps[name+'.o']=compile(name,text,original)
    deps['ai_hooks.o']=compile('ai_hooks.cc','void (*ai_matrix_observer)(int,int,double**)=nullptr;\n')
    def link(name,main):
        libs=[str(provider/'lib/libcalceph.a')] if calceph else []
        if blas:libs+=['-framework','Accelerate']
        exe=out/name;run([CXX,main,*deps.values(),archive,*libs,'-lquadmath','-lgfortran','-o',exe],name+'-link');return exe
    link('tempo2',compile('tempo2.C',raw('tempo2.C'),raw('tempo2.C')))
    if (HERE/'probe.cc').exists():link('probe',compile('probe.cc',(HERE/'probe.cc').read_text()))
    (out/'patch.diff').write_text(patch)
    record={'pin':PIN,'profile':profile,'config':sha(source/'config.h'),'source':sources,'objects':{str(p):sha(p) for p in deps.values()},'archive':sha(archive),'AH_identity':sha(base),'provider':str(provider),'commands':commands,'builder':sha(__file__),'patch':sha(out/'patch.diff'),'executables':{p.name:sha(p) for p in [out/'tempo2',out/'probe'] if p.exists()}}
    if calceph:record['calceph']={str(p):sha(p) for p in [provider/'include/calceph.h',provider/'lib/libcalceph.a']}
    (out/'identity.json').write_text(json.dumps(record,indent=2)+'\n')
    return out
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('out',type=Path);p.add_argument('--profile',choices=['disabled','calceph','blas','lapack','combined','qr-default'],default='combined');p.add_argument('--provider',type=Path,default=HERE/'build/provider');a=p.parse_args();print(build(a.out,a.profile,a.provider.resolve()))
