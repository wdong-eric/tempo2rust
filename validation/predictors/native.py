"""AF native build in owned staging; accepted AD/A2 objects remain read-only."""
from pathlib import Path
import subprocess,hashlib,json,difflib
ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
PIN='fda6ad4a9753ad7dc5d291406f2f310a9161423f'
REF=ROOT/'validation/reference/build'
CC='/opt/homebrew/bin/gcc-16';CXX='/opt/homebrew/bin/g++-16'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def build(out):
 out=Path(out);out.mkdir(parents=True,exist_ok=False);src=out/'source';src.mkdir()
 ad=ROOT/'validation/timing_models/build/native';identity=json.loads((ad/'identity.json').read_text())
 dependencies=[p for p in ad.glob('*.C.o') if p.name!='initialise.C.o']+[p for p in (ad/'ac').glob('*.C.o') if p.name!='initialise.C.o']
 for p in dependencies:assert sha(p)==identity['objects'][str(p)],p
 archive=REF/'libtempo2-reference.a';assert sha(archive)==json.loads((ROOT/'validation/reference/build_manifest.json').read_text())['static_library_sha256']
 changes={
 'initialise.C':[('for(int j=0;j<3;j++)o.siteVel[j]=0;', 'for(int j=0;j<3;j++)o.siteVel[j]=0;\n        for(int j=0;j<6;j++){o.earthMoonBary_ssb[j]=0;o.earthMoonBary_earth[j]=0;o.observatory_earth[j]=0;o.earth_ssb[j]=0;}')],
 'cheby2d.c': [('ld_fprintf(f,','fprintf(f,'),('if (nx < 0 && ny < 0)','if (nx < 2 || ny < 1)'),('    assert (dcheby->nx == cheby->nx);','    assert (cheby->nx >= 2);\n    assert (dcheby->nx == cheby->nx);')],
 'tempo2pred.c':[('    T2Predictor pred;','    T2Predictor pred;\n    T2Predictor_Init(&pred);')],
 't1polyco.c':[('    for (ic=0; ic < t1p->ncoeff; ic++){','    if (t1p->ncoeff < 2 || t1p->ncoeff > 32) return -1;\n    for (ic=0; ic < t1p->ncoeff; ic++){')],
 'cheby2d_int.C':[('    psr->nobs = nx*ny+1;','    psr->nobs = nx*ny+1;\n    for(int i=0;i<psr->nobs;i++){auto& o=psr->obsn[i];o.obsNjump=0;o.obsNfdjump=0;o.toaErr=0;o.origErr=0;o.equad=0;}')],
 'polyco.C':[('    char date[12];','    char date[100];'),('    psr->nobs = (ntoas*(*nsets))+1;','    psr->nobs = (ntoas*(*nsets))+1;\n    if(psr->nobs>800 || ncoeff<2 || ncoeff>31) {fprintf(stderr,"AF bounds boundary\\n");exit(2);}\n    for(int i=0;i<psr->nobs;i++){auto& o=psr->obsn[i];o.obsNjump=0;o.obsNfdjump=0;o.origErr=0;o.equad=0;if(!strcmp(sitename,"@")||!strcasecmp(sitename,"bat")){o.tdis1=0;o.tdis2=0;}}'),('if (i>800)','if (i+1>=800)')]
 }
 commands=[];patch='';objects=[];source_hashes={}
 for name,edits in changes.items():
  if name=='initialise.C':
   original=(ad/'source'/name).read_text()
   assert sha(ad/'source'/name)==identity['sources'][str(ad/'source'/name)]
  else:original=subprocess.check_output(['git','-C',str(ROOT/'tempo2'),'show',PIN+':'+name]).decode()
  corrected=original
  for a,b in edits:
   assert a in corrected,(name,a);corrected=corrected.replace(a,b)
  if name=='t1polyco.c':
   old='    t1p->binary_phase = T1P_grabLongDouble(line, 63, 7);\n    t1p->binary_frequency = T1P_grabLongDouble(line, 70, 9);'
   new='    if (strspn(line+63," \\n\\r") == strlen(line+63)) {t1p->binary_phase=0;t1p->binary_frequency=0;} else {\n'+old+'\n    }\n    if(t1p->ncoeff<2 || t1p->ncoeff>32)return -1;'
   assert old in corrected;corrected=corrected.replace(old,new)
   old='    t1p->doppler = T1P_grabLongDouble(line, 73, 6);\n    t1p->log10rms = T1P_grabLongDouble(line, 79, 7);'
   new='    int width=6,point=73;while(line[point]==\' \')point++;if(line[point]==\'+\'||line[point]==\'-\')point++;while(line[point]>=\'0\'&&line[point]<=\'9\')point++;if(line[point]==\'.\' && line[point+1]>=\'0\' && line[point+1]<=\'9\' && line[point+2]>=\'0\' && line[point+2]<=\'9\' && line[point+3]>=\'0\' && line[point+3]<=\'9\' && point+4-73>width)width=point+4-73;\n    t1p->doppler = T1P_grabLongDouble(line,73,width);\n    t1p->log10rms = T1P_grabLongDouble(line,73+width,7);'
   assert old in corrected;corrected=corrected.replace(old,new)
  if name=='tempo2pred.c':
   marker='int T2Predictor_GetPlan_Ext(char *filename,'
   declaration='extern void af_plan_interval(int,const T2Predictor*,long double,long double);\n'
   assert marker in corrected;corrected=corrected.replace(marker,declaration+marker)
   marker='        lastphase = phase;'
   assert marker in corrected;corrected=corrected.replace(marker,'        af_plan_interval(iseg,&pred,lastmjd,mjd);\n'+marker)
  if name=='polyco.C':
   decl='void af_polyco(pulsar*,int,int,longdouble*,longdouble,longdouble,double,double,double,const char*,int,double,const char*,longdouble,longdouble);\n'
   corrected=corrected.replace('void tzFit(pulsar *psr',decl+'void tzFit(pulsar *psr',1)
   marker='        fprintf(fout,"%21.6f ",(double)dm);'
   hook='        af_polyco(psr,iref,ncoeff,coeff,rphase,dm,*doppler,*rms,*utc,sitename,tspan,obsFreq,date,psr->param[param_pb].paramSet[0]?phi0:0,psr->param[param_pb].paramSet[0]?phifac:0);\n'
   assert marker in corrected;corrected=corrected.replace(marker,hook+marker)
  p=src/name;p.write_text(corrected);source_hashes[name]={'original':hashlib.sha256(original.encode()).hexdigest(),'corrected':sha(p)}
  patch+=''.join(difflib.unified_diff(original.splitlines(True),corrected.splitlines(True),fromfile='original/'+name,tofile='corrected/'+name))
  obj=out/(name+'.o');cmd=[CXX if name.endswith('.C') else CC,'-O2','-DHAVE_CONFIG_H','-I'+str(REF/'source'),'-c',str(p),'-o',str(obj)]
  subprocess.run(cmd,check=True,stdout=(out/(name+'.log')).open('w'),stderr=subprocess.STDOUT);commands.append(cmd);objects.append(obj)
 (out/'corrections.patch').write_text(patch)
 exe=out/'predictor';cmd=[CXX,'-O2','-ffp-contract=off','-DHAVE_CONFIG_H','-I'+str(REF/'source'),str(HERE/'native.cc'),*map(str,objects+dependencies),str(archive),'-lquadmath','-lgfortran','-o',str(exe)]
 subprocess.run(cmd,check=True,stdout=(out/'link.log').open('w'),stderr=subprocess.STDOUT);commands.append(cmd)
 (out/'identity.json').write_text(json.dumps({'pin':PIN,'sources':source_hashes,'objects':{str(p):sha(p) for p in objects+dependencies},'archive':sha(archive),'commands':commands,'executable':sha(exe),'harness':sha(HERE/'native.cc'),'builder':sha(Path(__file__)),'patch':sha(out/'corrections.patch')},indent=2)+'\n')
 return exe
if __name__=='__main__':
 import sys
 print(build(Path(sys.argv[1]).resolve()))
