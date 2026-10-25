// Stop before the source's unwritten cosmic-string prefactor is evaluated.
#include "tempo2.h"
#include "t2fit_gw.h"
#include <cstdio>
#include <cstdlib>
void ae_gw_initialized(bool initialized){
 if(!initialized){printf("UNWRITTEN cosmic-string prefactor\n");exit(86);}
}
int main(int argc,char**argv){
 MAX_PSR=1;MAX_OBSN=1;auto*p=new pulsar[1];initialise(p,1);
 p->param[param_gwcs_amp].paramSet[0]=1;
 p->gwcs_epoch=54000;p->gwcs_width=2;p->gwcs_geom_p=1;p->gwcs_geom_c=2;
 double t=atof(argv[1]);p->obsn[0].sat=t;p->obsn[0].bbat=t;
 for(int k=0;k<2;k++)printf("VALUE %d %.17g\n",k,t2FitFunc_gwcs_amp(p,0,0,0,param_gwcs_amp,k));
}
