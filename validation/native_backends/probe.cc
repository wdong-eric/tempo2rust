#include "config.h"
#include "tempo2.h"
#include "TKmatrix.h"
#include "TKfit.h"
#include "T2accel.h"
#include "t2fit.h"
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <vector>
#include <array>
#include <string>
extern int accel_uinv(double*,int);
static FILE*out;
extern void (*ai_matrix_observer)(int,int,double**);
static std::vector<double> scales;
static std::vector<std::vector<double>> design;
static void observe_matrix(int nr,int nc,double**a){scales.assign(nc,0);design.assign(nr,std::vector<double>(nc));for(int i=0;i<nr;i++)for(int j=0;j<nc;j++){scales[j]=fmax(scales[j],fabs(a[i][j]));design[i][j]=a[i][j];}}

static void v(std::string key,const char*kind,longdouble x){char b[128];quadmath_snprintf(b,sizeof b,"%.36Qg",x);fprintf(out,"%s\t%s\t%s\n",key.c_str(),kind,b);}
static void matrix(const char*key,double**m,int r,int c){for(int i=0;i<r;i++)for(int j=0;j<c;j++)v(std::string(key)+"/"+std::to_string(i)+"/"+std::to_string(j),"matrix",m[i][j]);}
static void snapshot(pulsar*ps,int n,std::string stage,bool timing){
 for(int p=0;p<n;p++)for(int i=0;i<ps[p].nobs;i++){
  auto&o=ps[p].obsn[i];std::string key=stage+"/"+std::to_string(p)+"/"+std::to_string(i);
  if(!timing || ps[p].useCalceph)v(key+"/tt","time",o.correctionTT_calcEph);
  double*vv[]={o.earth_ssb,o.sun_ssb,o.jupiter_earth,o.saturn_earth,o.uranus_earth,o.neptune_earth,o.venus_earth,o.earthMoonBary_ssb,o.earthMoonBary_earth};
  for(int k=0;k<9;k++)for(int j=0;j<6;j++)v(key+"/v/"+std::to_string(k)+"/"+std::to_string(j),j<3?"position":"velocity",vv[k][j]);
  if(!timing)for(int k=0;k<9;k++)for(int j=0;j<6;j++){
   v(key+"/planet/"+std::to_string(k)+"/"+std::to_string(j),j<3?"position":"velocity",o.planet_ssb[k][j]);
   v(key+"/next/"+std::to_string(k)+"/"+std::to_string(j),j<3?"position":"velocity",o.planet_ssb_tmr[k][j]);
   v(key+"/diff/"+std::to_string(k)+"/"+std::to_string(j),j<3?"position":"velocity",o.planet_ssb_derv[k][j]);
  }
  if(timing){v(key+"/bat","mjd",o.bat);v(key+"/phase","phase",o.phase);v(key+"/residual","time",o.residual);}
 }
}
int main(int argc,char**argv){
 if(argc<3)return 2;ai_matrix_observer=observe_matrix;out=fopen(argv[2],"w");if(!out)return 3;std::string mode=argv[1];
 if(mode=="kernels"){
#ifdef HAVE_BLAS
  double**a=malloc_blas(2,3),**b=malloc_blas(3,2),**c=malloc_blas(2,2);double av[]={1,2,3,-1,0,4},bv[]={2,-1,0,3,5,1},x[]={2,-1,3},y[2];memcpy(a[0],av,sizeof av);memcpy(b[0],bv,sizeof bv);
  accel_multMatrix(a[0],b[0],2,3,2,c[0]);matrix("product",c,2,2);accel_multMatrixVec(a[0],x,2,3,y);for(int i=0;i<2;i++)v("vector/"+std::to_string(i),"matrix",y[i]);
  free_blas(a);free_blas(b);free_blas(c);
#endif
#ifdef HAVE_LAPACK
  double**m=malloc_blas(3,3);double orig[]={4,2,1,2,5,2,1,2,6};memcpy(m[0],orig,sizeof orig);v("cholfac/status","exact",accel_cholfac(m[0],3));matrix("cholfac",m,3,3);
  memcpy(m[0],orig,sizeof orig);v("uinv/status","exact",accel_uinv(m[0],3));matrix("uinv",m,3,3);
  double**q=malloc_blas(4,2),**cov=malloc_blas(2,2);double qv[]={1,-1,1,0,1,1,0,1},rhs[]={1,2,4,3},param[2];memcpy(q[0],qv,sizeof qv);
  v("qr/chi","matrix",accel_lsq_qr(q,rhs,param,4,2,cov,1));matrix("qr/a",q,4,2);matrix("qr/c",cov,2,2);for(int i=0;i<4;i++)v("qr/rhs/"+std::to_string(i),"matrix",rhs[i]);
  for(int i=0;i<2;i++)v("qr/p/"+std::to_string(i),"matrix",param[i]);free_blas(q);free_blas(cov);free_blas(m);
#endif
 }else if(mode=="failures"){
#ifdef HAVE_LAPACK
  double**m=malloc_blas(2,2);m[0][0]=-1;m[0][1]=m[1][0]=0;m[1][1]=1;
  v("cholfac/failure","exact",accel_cholfac(m[0],2));
  m[0][0]=4;m[0][1]=m[1][0]=2;m[1][1]=5;v("cholfac/recovery","exact",accel_cholfac(m[0],2));free_blas(m);
  double**q=malloc_blas(3,2);double rhs[]={1,2,4};for(int i=0;i<3;i++){q[i][0]=1;q[i][1]=0;}
  v("qr/failure","exact",accel_lsq_qr(q,rhs,NULL,3,2,NULL,0));
  for(int i=0;i<3;i++){q[i][0]=1;q[i][1]=i-1;rhs[i]=i==2?4:i+1;}
  v("qr/recovery","matrix",accel_lsq_qr(q,rhs,NULL,3,2,NULL,0));free_blas(q);
#endif
 }else if(mode=="failed-position"){
  if(argc!=4)return 2;MAX_PSR=1;MAX_OBSN=1;auto*ps=new pulsar[1];initialise(ps,1);auto&s=ps[0];strcpy(s.JPL_EPHEMERIS,argv[3]);s.nobs=1;auto&o=s.obsn[0];o.sat=999999;o.correctionTT_Teph=0;o.nclock_correction=0;
  double*vv[]={o.earth_ssb,o.sun_ssb,o.jupiter_earth,o.saturn_earth,o.uranus_earth,o.neptune_earth,o.venus_earth,o.earthMoonBary_ssb,o.earthMoonBary_earth};
  for(auto*v:vv)for(int j=0;j<6;j++)v[j]=123.;
  for(int k=0;k<9;k++)for(int j=0;j<6;j++){o.planet_ssb[k][j]=0;o.planet_ssb_tmr[k][j]=0;}
  readEphemeris_calceph(ps,1);v("ignored_failure/earth","position",o.earth_ssb[0]);destroyOne(ps);delete[]ps;
 }else if(mode=="solver"){
  if(argc!=6)return 2;int constraint=atoi(argv[3]),scale=atoi(argv[4]),mask=atoi(argv[5]);useT2accel=2;
  double**a=malloc_blas(3,2),**c=malloc_blas(1,2),**cov=malloc_blas(2,2);double y[]={1,2,4},target[]={3},p[2],e[2];for(int i=0;i<3;i++){a[i][0]=1;a[i][1]=i-1;}c[0][0]=0;c[0][1]=1;
  double chi=TKrobustDefConstrainedLeastSquares(y,y,a,a,c,3,2,constraint,1e-27,scale,mask&1?p:NULL,mask&2?e:NULL,mask&4?cov:NULL,0,target);v("chi","matrix",chi);
  for(int i=0;i<2;i++){if(mask&1)v("p/"+std::to_string(i),"matrix",p[i]);if(mask&2)v("e/"+std::to_string(i),"matrix",e[i]);}if(mask&4)matrix("cov",cov,2,2);free_blas(a);free_blas(c);free_blas(cov);
 }else if(mode=="calceph"){
  if(argc!=6)return 2;MAX_PSR=2;MAX_OBSN=4;auto*ps=new pulsar[2];initialise(ps,1);
  for(int p=0;p<2;p++){
   auto&s=ps[p];s.nobs=p+2;s.units=atoi(argv[4]);s.eclCoord=atoi(argv[5]);strcpy(s.JPL_EPHEMERIS,argv[3]);
   s.param[param_dphaseplanet].paramSet[4]=1;s.param[param_dmassplanet].paramSet[5]=1;s.param[param_dmassplanet].val[5]=1.0/1048576;
   for(int i=0;i<s.nobs;i++){auto&o=s.obsn[i];o.sat=58000.125Q+p*20+i/8.Q;o.nclock_correction=0;o.correctionTT_Teph=0;
    for(int k=0;k<9;k++)for(int j=0;j<6;j++){o.planet_ssb[k][j]=(k+1)*0.125+j;o.planet_ssb_tmr[k][j]=o.planet_ssb[k][j]+0.25;}
   }
  }
  for(int pass=0;pass<2;pass++){tt2tb_calceph(ps,2);readEphemeris_calceph(ps,2);snapshot(ps,2,"direct"+std::to_string(pass),false);}for(int p=0;p<2;p++)destroyOne(ps+p);delete[]ps;
 }else if(mode=="pipeline"){
  if(argc<7 || (argc-5)%2)return 2;useT2accel=atoi(argv[3]);const char*cov=!strcmp(argv[4],"-")?NULL:argv[4];int n=(argc-5)/2;MAX_PSR=n;MAX_OBSN=512;auto*ps=new pulsar[n];initialise(ps,1);
  std::vector<std::array<char,MAX_FILELEN>> pars(n),tims(n);for(int p=0;p<n;p++){strcpy(pars[p].data(),argv[5+2*p]);strcpy(tims[p].data(),argv[6+2*p]);}
  readParfile(ps,reinterpret_cast<char(*)[MAX_FILELEN]>(pars.data()),reinterpret_cast<char(*)[MAX_FILELEN]>(tims.data()),n);readTimfile(ps,reinterpret_cast<char(*)[MAX_FILELEN]>(tims.data()),n);char*args[]={argv[0]};preProcess(ps,n,1,args);
  formBatsAll(ps,n);formResiduals(ps,n,1);snapshot(ps,n,"pre",true);t2Fit(ps,n,cov);
  for(int p=0;p<n;p++){auto&s=ps[p];std::string k="fit/"+std::to_string(p);v(k+"/chi","chi",s.fitChisq);v(k+"/dof","exact",s.fitNfree);int nf=s.fitinfo.output.totalNfit;v(k+"/n","exact",nf);for(int j=0;j<nf;j++){v(k+"/scale/"+std::to_string(j),"matrix",scales.at(j));v(k+"/p/"+std::to_string(j),"fit",s.fitinfo.output.parameterEstimates[j]);v(k+"/e/"+std::to_string(j),"error",s.fitinfo.output.errorEstimates[j]);for(int l=0;l<nf;l++)v(k+"/c/"+std::to_string(j)+"/"+std::to_string(l),"cov",s.covar[j][l]);}}
  formBatsAll(ps,n);formResiduals(ps,n,1);snapshot(ps,n,"post",true);for(int p=0;p<n;p++)destroyOne(ps+p);delete[]ps;
 }else return 2;
 fclose(out);return 0;
}
