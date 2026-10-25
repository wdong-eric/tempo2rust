"""AE-owned native integration. Rebuild accepted AC/AD corrections without changing them."""
from pathlib import Path
import sys,importlib.util,subprocess,hashlib,json,difflib
from fitting_corrections import corrected
ROOT=Path(__file__).resolve().parents[2];HERE=Path(__file__).resolve().parent;BUILD=HERE/'build/native'
sys.path.insert(0,str(ROOT/'validation/timing_models'))
spec=importlib.util.spec_from_file_location('ad_native',ROOT/'validation/timing_models/native_timing.py');ad=importlib.util.module_from_spec(spec);spec.loader.exec_module(ad)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def build():
 ad.BUILD=BUILD/'ad';ad.build()
 src=BUILD/'source';src.mkdir(parents=True,exist_ok=True);objects=[];commands=[];patch=''
 for name in ['TKfit.C','t2fit.C','t2fit_nestlike.C','t2fit_stdFitFuncs.C','t2fit_fitwaves.C','constraints_covar.C','choleskyRoutines.C']:
  old=ad.ac.blob(name).decode();new=corrected(name,old)
  if name=='TKfit.C':
   new=new.replace('        wmax = TKfindMax_Ld(w,nparams);','        extern void ae_svd(int,int,int,const longdouble*,double);\n        ae_svd(ndata,nparams,nconstraints,w,tol);\n        wmax = TKfindMax_Ld(w,nparams);')
  if name=='t2fit.C':
   declaration='extern void ae_prepared(int,pulsar*,const double*,const double*,const double*,const int*,int,double**,double**,double*,double**);\nextern int ae_solver_owner;\nextern void ae_update(pulsar*,int,int,int,const char*);\n'
   new=new.replace('void t2Fit(pulsar *psr,unsigned int npsr, const char *covarFuncFile){',declaration+'void t2Fit(pulsar *psr,unsigned int npsr, const char *covarFuncFile){')
   new=new.replace('        free_blas(cholesky_L);','        ae_prepared(ipsr,psr+ipsr,psr_x,psr_y,psr_e,psr_toaidx,psr_ndata,designMatrix,white_designMatrix,psr_white_y,constraintsMatrix);\n        free_blas(cholesky_L);',1)
   new=new.replace('            chisq = TKrobustDefConstrainedLeastSquares(', '            ae_solver_owner=ipsr;\n            chisq = TKrobustDefConstrainedLeastSquares(')
   new=new.replace('        chisq = TKrobustDefConstrainedLeastSquares(y,', '        ae_solver_owner=-1;\n        chisq = TKrobustDefConstrainedLeastSquares(y,')
   new=new.replace('    t2fit_postfit(psr,npsr);','    ae_solver_owner=-2;\n    t2fit_postfit(psr,npsr);')
   call='        func(psr,ipsr,param,fitinfo->paramCounters[ifit],val[ifit],error[ifit]);'
   new=new.replace(call,'        ae_update(psr,ipsr,param,fitinfo->paramCounters[ifit],"before_update");\n'+call+'\n        ae_update(psr,ipsr,param,fitinfo->paramCounters[ifit],"after_update");')
  path=src/name;path.write_text(new);obj=BUILD/(name+'.o')
  patch+=''.join(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='original/'+name,tofile='corrected-observed/'+name))
  cmd=[ad.ac.CC,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(ad.ac.REF/'source'),'-c',str(path),'-o',str(obj)];r=subprocess.run(cmd,capture_output=True);(BUILD/(name+'.log')).write_bytes(r.stdout+r.stderr);assert r.returncode==0,r.stderr.decode();commands.append(cmd);objects.append(obj)
 (BUILD/'corrections.patch').write_text(patch)
 objects+=sorted((ad.BUILD).glob('*.C.o'))+[p for p in sorted((ad.BUILD/'ac').glob('*.C.o')) if p.name!='initialise.C.o']
 exe=BUILD/'fit';cmd=[ad.ac.CC,'-O2','-std=gnu++11','-DHAVE_CONFIG_H','-I'+str(ad.ac.REF/'source'),str(HERE/'native_fit.cc'),*map(str,objects),str(ad.ac.REF/'libtempo2-reference.a'),'-lquadmath','-lgfortran','-o',str(exe)]
 r=subprocess.run(cmd,capture_output=True);(BUILD/'link.log').write_bytes(r.stdout+r.stderr);assert r.returncode==0,r.stderr.decode();commands.append(cmd)
 (BUILD/'identity.json').write_text(json.dumps({'pin':ad.ac.PIN,'builder_sha256':sha(Path(__file__)),'harness_sha256':sha(HERE/'native_fit.cc'),'ad_identity_sha256':sha(ad.BUILD/'identity.json'),'archive_sha256':sha(ad.ac.REF/'libtempo2-reference.a'),'objects':{str(p):sha(p)for p in objects},'commands':commands,'executable_sha256':sha(exe),'patch_sha256':sha(BUILD/'corrections.patch')},indent=2)+'\n');return exe
if __name__=='__main__':print(build())
