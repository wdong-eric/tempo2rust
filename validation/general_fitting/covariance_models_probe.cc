// Covariance models on canonical loaded/timed data; observe C before mutation.
#include "tempo2.h"
#include "t2fit.h"
#include "cholesky.h"
#include "TKmatrix.h"
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include "native_observer_stubs.h"
static FILE*out;
void dump(const char*name,double**m,int n){for(int i=0;i<n;i++)for(int j=0;j<n;j++)fprintf(out,"VALUE %s %d %d %.17g\n",name,i,j,m[i][j]);}
void ae_covariance(double**m,int n){dump("covariance",m,n);}
int main(int argc,char**argv){
 if(argc!=6)return 2;out=fopen(argv[1],"w");if(!out)return 3;
 MAX_PSR=1;MAX_OBSN=512;auto*ps=new pulsar[1];initialise(ps,1);
 char par[1][MAX_FILELEN],tim[1][MAX_FILELEN];strcpy(par[0],argv[2]);strcpy(tim[0],argv[3]);
 readParfile(ps,par,tim,1);readTimfile(ps,tim,1);char*args[]={argv[0]};preProcess(ps,1,1,args);formBatsAll(ps,1);formResiduals(ps,1,1);
 int size=ps->nobs;auto*x=new double[size],*y=new double[size],*e=new double[size];auto*ip=new int[size];
 int n=t2Fit_getFitData(ps,x,y,e,ip);auto l=malloc_blas(n,n);debugFlag=atoi(argv[5]);
 getCholeskyMatrix(l,argv[4],ps,x,y,e,n,0,ip);dump("lower",l,n);
 fprintf(out,"VALUE determinant -1 -1 %.17g\n",ps->detL);
 for(int i=0;i<n;i++)fprintf(out,"INDEX %d %d\n",i,ip[i]);fclose(out);return 0;
}
