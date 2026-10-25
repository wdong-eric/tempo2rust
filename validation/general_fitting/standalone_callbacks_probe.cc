#include "tempo2.h"
#include "t2fit_ifunc.h"
#include "t2fit_glitch.h"
#include "t2fit_gw.h"
#include <cstdio>
#include <cstring>
int ae_solver_owner=-2;
void ae_svd(int,int,int,const longdouble*,double){}
void ae_update(pulsar*,int,int,int,const char*){}
void ae_prepared(int,pulsar*,const double*,const double*,const double*,const int*,int,double**,double**,double*,double**){}
int main(int argc,char**argv){
 if(argc!=4)return 2;MAX_PSR=1;MAX_OBSN=512;auto*p=new pulsar[1];initialise(p,1);
 char par[1][MAX_FILELEN],tim[1][MAX_FILELEN];strcpy(par[0],argv[2]);strcpy(tim[0],argv[3]);
 readParfile(p,par,tim,1);readTimfile(p,tim,1);char*args[]={argv[0]};preProcess(p,1,1,args);formBatsAll(p,1);formResiduals(p,1,1);
 auto*out=fopen(argv[1],"w");if(!out)return 3;
 const char*names[]={"ifunc","plus","cross"};param_label labels[]={param_ifunc,param_quad_ifunc_p,param_quad_ifunc_c};
 for(int j=0;j<3;j++)for(int k=0;k<3;k++)for(int i=0;i<p->nobs;i++)fprintf(out,"VALUE %s_%d %d -1 %.17g\n",names[j],k,i,t2FitFunc_sifunc(p,0,0,i,labels[j],k));
 for(int k=0;k<2;k++)for(int i=0;i<p->nobs;i++)fprintf(out,"VALUE gwb_%d %d -1 %.17g\n",k,i,t2FitFunc_gwb_amp(p,0,0,i,param_gwb_amp,k));
 for(int j=0;j<2;j++){
  auto label=j==0?param_glf0:param_glf2;t2UpdateFunc_stdGlitch(p,0,label,0,j==0?1e-9:.2,j==0?2e-9:.03);
  char v[128],e[128];quadmath_snprintf(v,sizeof v,"%.36Qg",p->param[label].val[0]);quadmath_snprintf(e,sizeof e,"%.36Qg",p->param[label].err[0]);
  fprintf(out,"VALUE glf%d_value -1 -1 %s\nVALUE glf%d_error -1 -1 %s\n",j*2,v,j*2,e);
 }
 fclose(out);return 0;
}
