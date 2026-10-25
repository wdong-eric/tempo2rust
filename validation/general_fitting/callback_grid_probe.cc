#include "tempo2.h"
#include "t2fit.h"
#include <cstdio>
#include <cstring>
#include <vector>
#include "native_observer_stubs.h"
extern paramDerivFunc getDerivFunction(pulsar*,param_label,int);
int main(int argc,char**argv){
 if(argc!=4)return 2;MAX_PSR=1;MAX_OBSN=512;auto*p=new pulsar[1];initialise(p,1);
 char par[1][MAX_FILELEN],tim[1][MAX_FILELEN];strcpy(par[0],argv[2]);strcpy(tim[0],argv[3]);
 readParfile(p,par,tim,1);readTimfile(p,tim,1);char*args[]={argv[0]};preProcess(p,1,1,args);formBatsAll(p,1);formResiduals(p,1,1);
 std::vector<double>x(p->nobs),y(p->nobs),e(p->nobs);std::vector<int>ip(p->nobs);
 int n=t2Fit_getFitData(p,x.data(),y.data(),e.data(),ip.data());auto*g=new FitInfo{};
 t2Fit_fillFitInfo(p,p->fitinfo,*g,x.data(),ip.data(),n);
 FILE*out=fopen(argv[1],"w");if(!out)return 3;
 for(unsigned j=0;j<p->fitinfo.nParams;j++){
  auto l=p->fitinfo.paramIndex[j];int k=p->fitinfo.paramCounters[j];
  for(int i=0;i<n;i++)fprintf(out,"VALUE %d_%d_derivative %d -1 %.17g\n",l,k,i,p->fitinfo.paramDerivs[j](p,0,x[i],ip[i],l,k));
  if(l==param_ZERO)continue;
  double*av=nullptr,*ae=nullptr;
  if(l==param_tel_dx){av=&p->telDX_v[k];ae=&p->telDX_e[k];}
  if(l==param_tel_dy){av=&p->telDY_v[k];ae=&p->telDY_e[k];}
  if(l==param_tel_dz){av=&p->telDZ_v[k];ae=&p->telDZ_e[k];}
  longdouble old=av?*av:p->param[l].val[k],error=ae?*ae:p->param[l].err[k];
  p->fitinfo.updateFunctions[j](p,0,l,k,1e-9,2e-10);
  longdouble value=av?*av:p->param[l].val[k],err_value=ae?*ae:p->param[l].err[k];
  char v[128],err[128];quadmath_snprintf(v,sizeof v,"%.36Qg",value-old);quadmath_snprintf(err,sizeof err,"%.36Qg",err_value);
  fprintf(out,"VALUE %d_%d_change -1 -1 %s\nVALUE %d_%d_error -1 -1 %s\n",l,k,v,l,k,err);
  if(av){*av=old;*ae=error;}else{p->param[l].val[k]=old;p->param[l].err[k]=error;}
 }
 if(getDerivFunction(p,param_gwecc,0)!=nullptr)return 86;
 fclose(out);return 0;
}
