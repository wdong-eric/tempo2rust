// Legacy adapters, using naturally loaded and timed canonical pulsars.
#include "tempo2.h"
#include "t2fit.h"
#include "t2fit_stdFitFuncs.h"
#include "TKfit.h"
#include <cstdio>
#include <cstdlib>
#include <cstring>
static FILE*out;
void ae_solver_svd(int n,const longdouble*w){for(int i=0;i<n;i++){char s[128];quadmath_snprintf(s,sizeof s,"%.36Qg",w[i]);fprintf(out,"S %d %s\n",i,s);}}
void ae_solver_input(double*y,double*wy,double**m,double**wm,double**c,int n,int nf,int nc){
 fprintf(out,"SHAPE %d %d %d\n",n,nf,nc);
 for(int i=0;i<n;i++){fprintf(out,"D %d %.17g %.17g\n",i,y[i],wy[i]);for(int j=0;j<nf;j++)fprintf(out,"A %d %d %.17g %.17g\n",i,j,m[i][j],wm[i][j]);}
 for(int i=0;i<nc;i++)for(int j=0;j<nf;j++)fprintf(out,"Q %d %d %.17g\n",i,j,c[i][j]);
}
void basis(double x,double*row,int n,pulsar*ps,int o,int p){
 row[0]=t2FitFunc_stdFreq(ps,p,x,o,param_f,0);row[1]=1;
 for(int k=2;k<n;k++)row[k]=t2FitFunc_stdFreq(ps,p,x,o,param_f,k-1);
}
int main(int argc,char**argv){
 if(argc!=7)return 2;out=fopen(argv[1],"w");if(!out)return 3;
 bool global=strcmp(argv[2],"global")==0;int count=global?2:1;MAX_PSR=count;MAX_OBSN=512;auto*ps=new pulsar[count];initialise(ps,1);
 char par[2][MAX_FILELEN],tim[2][MAX_FILELEN];for(int p=0;p<count;p++){strcpy(par[p],argv[4+p]);strcpy(tim[p],argv[6]);}
 readParfile(ps,par,tim,count);readTimfile(ps,tim,count);char*args[]={argv[0]};preProcess(ps,count,1,args);formBatsAll(ps,count);formResiduals(ps,count,1);
 double*x[2],*y[2],*e[2],**l[2];int*ip[2],n[2];
 for(int p=0;p<count;p++){
  int size=ps[p].nobs;x[p]=new double[size];y[p]=new double[size];e[p]=new double[size];ip[p]=new int[size];n[p]=t2Fit_getFitData(ps+p,x[p],y[p],e[p],ip[p]);
  bool full=strcmp(argv[3],"full")==0;l[p]=malloc_blas(full?n[p]:1,n[p]);
  for(int i=0;i<n[p];i++)if(full){l[p][i][i]=e[p][i];if(i>0)l[p][i][i-1]=e[p][i]*.125;}else l[p][0][i]=1/e[p][i];
 }
 int nf[2]={3,2},total=global?6:4;double*p=new double[total],*error=new double[total],**cov=malloc_blas(total,total),chi;
 if(global)TKleastSquares_global_pulsar(x,y,n,p,error,nf,1,cov,&chi,basis,ps,1e-27,ip,0,l,2);
 else if(strcmp(argv[2],"single")==0)TKleastSquares_single_pulsar(x[0],y[0],n[0],p,error,4,cov,&chi,basis,ps,1e-27,ip[0],0,l[0]);
 else if(strcmp(argv[2],"dcm")==0)TKleastSquares_svd_psr_dcm(x[0],y[0],e[0],n[0],p,error,4,cov,&chi,basis,1,ps,1e-27,ip[0],l[0]);
 else if(strcmp(argv[2],"sigma")==0)TKleastSquares_svd_psr(x[0],y[0],e[0],n[0],p,error,4,cov,&chi,basis,1,ps,1e-27,ip[0]);
 else return 2;
 fprintf(out,"CHI %.17g\n",chi);for(int j=0;j<total;j++){fprintf(out,"P %d %.17g\n",j,p[j]);fprintf(out,"E %d %.17g\n",j,error[j]);for(int k=0;k<total;k++)fprintf(out,"C %d %d %.17g\n",j,k,cov[j][k]);}
 fclose(out);return 0;
}
