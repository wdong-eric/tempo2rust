#include "tempo2.h"
#include "t2fit_nestlike.h"
#include <cstdio>
#include <cstdlib>
int main(int argc,char**argv){
 MAX_PSR=1;MAX_OBSN=8;auto* ps=new pulsar[1];initialise(ps,1);
 ps[0].nobs=5;ps[0].TNRedC=2;ps[0].TNRedFLow=0;
 ps[0].TNsubtractPoly=t2EncodeTNsubtractPoly(2,2,2,2);
 ps[0].param[param_start].val[0]=58000;ps[0].param[param_finish].val[0]=58100;
 ps[0].param[param_pepoch].val[0]=58000;
 double t[5]={0,20,45,70,100};for(int i=0;i<5;i++)ps[0].obsn[i].bbat=58000+t[i];
 int mode=argc>1?atoi(argv[1]):0;
 if(mode==0)printf("BEFORE %.17g\n",t2FitFunc_nestlike_red(ps,0,30,0,param_red_sin,0));
 ps[0].obsn[2].bbat=58060;
 printf("TOA %.17g\n",t2FitFunc_nestlike_red(ps,0,30,0,param_red_sin,0));
 ps[0].TNRedFLow=0.2;
 printf("FREQ %.17g\n",t2FitFunc_nestlike_red(ps,0,30,0,param_red_sin,0));
}
