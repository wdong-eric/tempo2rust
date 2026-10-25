// Natural reader/timing state; instrumentation stops before undefined reads/writes.
#include "tempo2.h"
#include "t2fit.h"
#include "choleskyRoutines.h"
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <map>
static std::map<void*,size_t> extent;
void subtractTNPoly(pulsar*,int,label);
int ae_solver_owner=-2;
void ae_svd(int,int,int,const longdouble*,double){}
void ae_update(pulsar*,int,int,int,const char*){}
void ae_prepared(int,pulsar*,const double*,const double*,const double*,const int*,int,double**,double**,double*,double**){}
void* ae_allocate(size_t bytes) {void*p=malloc(bytes);extent[p]=bytes;return p;}
void ae_check_extent(double*p,int i) {
 auto found=extent.find(p);
 if(found!=extent.end() && (i+1)*sizeof(double)>found->second) {
  printf("BOUNDARY allocation bytes=%zu first_invalid_double=%d\n",found->second,i);fflush(stdout);exit(86);
 }
}
void ae_observe_residuals(double*x,double*y,double*e,int n) {
 printf("EXTENT bytes=%zu count=%d\n",extent[x],n);
 for(int i=0;i<n;i++)printf("RES %d %.17g %.17g %.17g\n",i,x[i],y[i],e[i]);
}
void ae_initialized(int written,int n) {
 if(written!=n) {printf("BOUNDARY white_y initialized=%d required=%d\n",written,n);fflush(stdout);exit(86);}
}
int main(int argc,char**argv) {
 if(argc!=4)return 2;
 bool allocation=strcmp(argv[1],"allocation")==0;
 MAX_PSR=1;MAX_OBSN=allocation?20:512;auto*p=new pulsar[1];initialise(p,1);
 char par[1][MAX_FILELEN],tim[1][MAX_FILELEN];strcpy(par[0],argv[2]);strcpy(tim[0],argv[3]);
 readParfile(p,par,tim,1);readTimfile(p,tim,1);char*args[]={argv[0]};preProcess(p,1,1,args);formBatsAll(p,1);formResiduals(p,1,1);
 if(allocation) {
  double a,b,c,d,e;char output[]="unused.dcf";T2get_covFunc_automatic(p,40,output,&a,&b,&c,&d,&e,0,0);
 } else {
  bool fitted=strcmp(argv[1],"fitted")==0;
  for(int k=0;k<p->param[param_f].aSize;k++)p->param[param_f].fitFlag[k]=fitted&&k==0;
  p->TNsubtractPoly=1;
  for(int i=0;i<p->nobs;i++)p->obsn[i].TNRedSignal=1e-7*(i+1)+2e-9*(i%3);
  writeResiduals=7;subtractTNPoly(p,0,param_f);
  char value[128];quadmath_snprintf(value,sizeof value,"%.36Qg",p->param[param_f].val[0]);printf("F0 %s\n",value);
  printf("MASK %d\n",writeResiduals);
  for(int i=0;i<p->nobs;i++)printf("TN %d %.17g\n",i,p->obsn[i].TNRedSignal);
 }
 return 0;
}
