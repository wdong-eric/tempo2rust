#include "tempo2.h"
#include "t2fit.h"
#include <cstdio>
#include <vector>
#include <cstring>
int main(){
 MAX_PSR=1;MAX_OBSN=2;auto*p=new pulsar[1];initialise(p,1);
 p->param[param_start].val[0]=58000;p->param[param_finish].val[0]=58100;
 p->TNDMAmp=-14;p->TNDMGam=4;p->TNDMC=1;
 p->TNChromAmp=-14;p->TNChromGam=4;p->TNChromC=1;p->TNChromIdx=4;
 auto*g=new FitInfo{};
 double x[1]={0};int ip[1]={0};
 for(int nlog: {0,1}){
  p->TNDM_log_freqs=p->TNChrom_log_freqs=nlog;
  t2Fit_fillFitInfo(p,p->fitinfo,*g,x,ip,0);
  auto&f=p->fitinfo;
  for(unsigned j=0;j<f.nConstraints;j++){
   std::vector<double> row(f.nParams);t2Fit_buildConstraintsMatrix(p,0,j,row.data());
   int populated=0;for(double v:row)if(v!=0)populated++;
   printf("PRIOR %d %u %d %d %d\n",nlog,j,f.constraintIndex[j],f.constraintCounters[j],populated);
  }
 }
 // A real second configured constraint makes the wrong IFUNC label observable
 // without reading uninitialized storage or invoking its separate OOB callback.
 p->TNDMAmp=p->TNChromAmp=0;p->nconstraints=2;
 p->constraints[0]=constraint_ifunc_cov;p->constraints[1]=constraint_param;
 p->constraint_special[0]=strdup("MODEL T2SqExp 1e-12 10");
 p->constraint_special[1]=strdup("F0 10 1");
 p->param[param_f].val[0]=10;p->param[param_f].paramSet[0]=1;
 p->param[param_ifunc].val[0]=2;p->param[param_ifunc].paramSet[0]=1;p->param[param_ifunc].fitFlag[0]=1;
 p->ifuncN=2;p->ifuncT[0]=58000;p->ifuncT[1]=58100;p->ifuncV[0]=p->ifuncV[1]=0;
 t2Fit_fillFitInfo(p,p->fitinfo,*g,x,ip,0);
 for(unsigned j=0;j<p->fitinfo.nConstraints;j++)printf("IFUNCLABEL %u %d\n",j,p->fitinfo.constraintIndex[j]);
}
