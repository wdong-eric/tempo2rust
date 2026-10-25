#include "tempo2.h"
#include "t2fit.h"
#include "choleskyRoutines.h"
#include <cstdio>
#include <cstring>
static FILE*out;
static int factors=0;
int ae_solver_owner=-2;
void ae_svd(int,int,int,const longdouble*,double){}
void ae_update(pulsar*,int,int,int,const char*){}
void ae_prepared(int,pulsar*,const double*,const double*,const double*,const int*,int,double**,double**,double*,double**){}
void ae_factors(double**l,double*c,int n,int nd){
 ++factors;for(int i=0;i<=nd;i++)fprintf(out,"C %d %d %.17g\n",factors,i,c[i]);
 for(int i=0;i<n;i++)for(int j=0;j<n;j++)fprintf(out,"L %d %d %d %.17g\n",factors,i,j,l[i][j]);
}
int main(int argc,char**argv){
 if(argc!=4)return 2;out=fopen(argv[1],"w");if(!out)return 3;
 MAX_PSR=1;MAX_OBSN=512;auto*ps=new pulsar[1];initialise(ps,1);
 char par[1][MAX_FILELEN],tim[1][MAX_FILELEN];strcpy(par[0],argv[2]);strcpy(tim[0],argv[3]);
 readParfile(ps,par,tim,1);readTimfile(ps,tim,1);char*args[]={argv[0]};preProcess(ps,1,1,args);
 for(int it=0;it<ps[0].nits;it++){
  if(it)for(int l=0;l<MAX_PARAMS;l++)for(int k=0;k<ps[0].param[l].aSize;k++){ps[0].param[l].prefit[k]=ps[0].param[l].val[k];ps[0].param[l].prefitErr[k]=ps[0].param[l].err[k];}
  formBatsAll(ps,1);formResiduals(ps,1,1);t2Fit(ps,1,NULL);formBatsAll(ps,1);formResiduals(ps,1,1);
 }
 double fw,fr,alpha,model,white;char file[]="automatic.dcf";
 T2get_covFunc_automatic(ps,10,file,&fw,&fr,&alpha,&model,&white,0,0);
 double controls[]={fw,fr,alpha,model,white,EXPSMOOTH,UPW,NFIT,FCFINAL,FCALPHA,WNLEVEL};
 for(int i=0;i<11;i++)fprintf(out,"V %d %.17g\n",i,controls[i]);
 fclose(out);return 0;
}
