// Direct source helper calls; the source observer guards uninitialized FFT DC.
#include "tempo2.h"
#include "TKmatrix.h"
#include "cholesky.h"
#include "choleskyRoutines.h"
#include <cstdio>
#include <cstring>
#include <cstdlib>
void cholesky_sqexp_kernel(double**,double,double,double*,double*,double*,int,int);
void cholesky_matern15_kernel(double**,double,double,double*,double*,double*,int,int);
void cholesky_matern25_kernel(double**,double,double,double*,double*,double*,int,int);
void cholesky_powerlawQPModel(double**,double,double,double,double,double,double,double,double*,double*,double*,int,int);
void cholesky_powerlawRedPinkModel(double**,double,double,double,double,double,double*,double*,double*,int,int);
static bool captured=false;
static double samples[14];
void ae_kernel(double*c,int n){if(!captured){for(int i=0;i<=n;i++){samples[i]=c[i];printf("VALUE kernel %d -1 %.17g\n",i,c[i]);}captured=true;}}
void ae_dc_guard(bool initialized){printf("DC %d\n",initialized);if(!initialized)exit(86);}
void dump(const char*name,double**m,int n){for(int i=0;i<n;i++)for(int j=0;j<n;j++)printf("VALUE %s %d %d %.17g\n",name,i,j,m[i][j]);}
void vec(const char*name,double*v,int n){for(int i=0;i<n;i++)printf("VALUE %s %d -1 %.17g\n",name,i,v[i]);}
int main(int argc,char**argv){
 if(argc!=2)return 2;const char*kind=argv[1];
 if(strcmp(kind,"mask")==0){
  auto*ps=new pulsar();ps->obsn=new observation[7]();int ip[7]={0,1,2,3,4,5,6};double sat[7]={0,1,2,4,6,7,8};
  for(int i=0;i<7;i++)ps->obsn[i].sat=sat[i];
  auto m=malloc_blas(8,8),mm=malloc_blas(8,8),unmasked=malloc_blas(8,8);
  for(int i=0;i<8;i++)for(int j=0;j<8;j++)mm[i][j]=10*i+j+1;
  addCovar(m,mm,NULL,NULL,NULL,8,1,ip,ps,1,7);addCovar(unmasked,mm,NULL,NULL,NULL,8,1,NULL,ps,1,7);
  dump("masked",m,8);dump("unmasked",unmasked,8);return 0;
 }
 double x[6]={0,.25,2.75,5.5,12.5,0},y[6]={.1,.25,-.25,.5,-.5,1},e[6]={.125,.25,.5,1,2,4};auto m=malloc_blas(6,6);
 if(strcmp(kind,"power")==0)cholesky_powerlawModel(m,4,3,1e-18,x,y,e,5,0);
 else if(strcmp(kind,"beta")==0)cholesky_powerlawModel_withBeta(m,4,.5,3,1e-18,x,y,e,5,0);
 else if(strcmp(kind,"qp")==0)cholesky_powerlawQPModel(m,4,3,1e-18,2,.25,.03,1,x,y,e,5,0);
 else if(strcmp(kind,"redpink")==0)cholesky_powerlawRedPinkModel(m,4,3,1e-18,1.5,15,x,y,e,5,0);
 else if(strcmp(kind,"sqexp")==0)cholesky_sqexp_kernel(m,.25,3,x,y,e,5,0);
 else if(strcmp(kind,"matern15")==0)cholesky_matern15_kernel(m,.25,3,x,y,e,5,0);
 else if(strcmp(kind,"matern25")==0)cholesky_matern25_kernel(m,.25,3,x,y,e,5,0);
 else if(strcmp(kind,"dmparam")==0)cholesky_dmModelCovarParam(m,1.5,.25,3,x,y,e,5,0);
 else if(strcmp(kind,"dm")==0)cholesky_dmModel(m,4,2,1400,x,y,e,5,0);
 else return 2;
 cholesky_covarFunc2matrix(m,samples,13,x,y,e,6,1);
 dump("covariance",m,6);for(int i=0;i<6;i++)m[i][i]+=.125;dump("positive",m,6);
 auto copy=malloc_blas(6,6),lower=malloc_blas(6,6),inverse=malloc_blas(6,6);
 for(int i=0;i<6;i++)memcpy(copy[i],m[i],6*sizeof(double));
 double diagonal[6],white[6];T2cholDecomposition(copy,6,diagonal);dump("decomposed",copy,6);vec("diagonal",diagonal,6);
 for(int i=0;i<6;i++)memcpy(copy[i],m[i],6*sizeof(double));cholesky_formL(lower,copy,6);dump("lower",lower,6);
 for(int i=0;i<6;i++)memcpy(copy[i],m[i],6*sizeof(double));cholesky_formLinv(inverse,copy,6);dump("inverse",inverse,6);
 T2getWhiteRes(x,y,e,6,lower,white);vec("white",white,6);
 auto*ps=new pulsar();auto d=malloc_blas(1,6);getCholeskyDiagonals(d,ps,x,y,e,6,0,NULL);vec("sigma",d[0],6);printf("VALUE determinant -1 -1 %.17g\n",ps->detL);
 return 0;
}
