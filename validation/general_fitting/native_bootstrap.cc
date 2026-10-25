#include "tempo2.h"
#include "t2fit.h"
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cstdint>
double random(long*);
static FILE* output;
static int iteration;
int ae_solver_owner=-2;
void ae_svd(int,int,int,const longdouble*,double){}
void ae_update(pulsar*,int,int,int,const char*){}
void ae_prepared(int,pulsar*,const double*,const double*,const double*,const int*,int,double**,double**,double*,double**){}
void ae_bootstrap_begin(int i){iteration=i;}
void ae_bootstrap_sample(int i,int j){fprintf(output,"S %d %d %d\n",iteration,i,j);}
static void parameters(pulsar*p,char stage){
 for(int l=0;l<MAX_PARAMS;l++)for(int k=0;k<p->param[l].aSize;k++)if(p->param[l].fitFlag[k]==1){
  char value[128],error[128];quadmath_snprintf(value,sizeof value,"%.36Qg",p->param[l].val[k]);quadmath_snprintf(error,sizeof error,"%.36Qg",p->param[l].err[k]);
  fprintf(output,"%c %d %d %d %s %s\n",stage,iteration,l,k,value,error);
 }
}
void ae_bootstrap_fit(pulsar*p){parameters(p,'P');}
int main(int argc,char**argv){
 if(argc==4 && strcmp(argv[1],"rng")==0){
  long seed=atol(argv[2]);for(int i=0;i<atoi(argv[3]);i++){
   double x=random(&seed);uint64_t bits;memcpy(&bits,&x,sizeof bits);printf("%016llx\n",(unsigned long long)bits);
  }printf("SEED %ld\n",seed);return 0;
 }
 if(argc!=4)return 2;output=fopen(argv[1],"w");if(!output)return 3;
 MAX_PSR=2;MAX_OBSN=512;auto*ps=new pulsar[2];initialise(ps,1);
 char par[1][MAX_FILELEN],tim[1][MAX_FILELEN];strcpy(par[0],argv[2]);strcpy(tim[0],argv[3]);
 readParfile(ps,par,tim,1);readTimfile(ps,tim,1);char*args[]={argv[0]};preProcess(ps,1,1,args);
 for(int it=0;it<ps[0].nits;it++){
  if(it)for(int l=0;l<MAX_PARAMS;l++)for(int k=0;k<ps[0].param[l].aSize;k++){ps[0].param[l].prefit[k]=ps[0].param[l].val[k];ps[0].param[l].prefitErr[k]=ps[0].param[l].err[k];}
  formBatsAll(ps,1);formResiduals(ps,1,1);t2Fit(ps,1,NULL);formBatsAll(ps,1);formResiduals(ps,1,1);
 }
 bootstrap(ps,0,1);parameters(ps,'F');fclose(output);return 0;
}
