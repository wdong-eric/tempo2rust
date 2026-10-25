// t2fit.C caller-order witness; the native helpers themselves are unmodified.
#include "tempo2.h"
#include "t2fit.h"
#include <cstdio>
#include <cstring>
int main() {
  MAX_PSR=1;MAX_OBSN=4;
  auto* ps=new pulsar[1];initialise(ps,1);auto& p=ps[0];
  p.nobs=3;p.param[param_pepoch].val[0]=58000;
  for(bool ordered: {false,true})for(bool corrected: {false,true}) {
    p.sorted=0;
    for(int i=0;i<3;i++) {
      int k=ordered?i:(i+2)%3;auto& o=p.obsn[i];
      o.sat=o.bat=o.bbat=58000+k;o.residual=k+1;o.toaErr=10*(k+1);
      o.deleted=0;snprintf(o.fname,sizeof o.fname,"row%d",k);
    }
    double x[3],y[3],e[3];int ip[3];
    if(corrected)sortToAs(ps);
    unsigned n=t2Fit_getFitData(ps,x,y,e,ip);
    if(!corrected)sortToAs(ps);
    int mismatches=0;
    for(unsigned i=0;i<n;i++) {
      bool same=y[i]==p.obsn[ip[i]].residual && e[i]==p.obsn[ip[i]].toaErr*1e-6;
      mismatches+=!same;
      printf("ROW %d %d %u %.17g %.17g %s %d\n",ordered,corrected,i,y[i],e[i],p.obsn[ip[i]].fname,same);
    }
    printf("ORDER %d CORRECTED %d MISMATCHES %d\n",ordered,corrected,mismatches);
    if(mismatches!=((!ordered&&!corrected)?3:0))return 1;
  }
}
