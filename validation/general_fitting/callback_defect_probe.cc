#include "tempo2.h"
#include "t2fit_stdFitFuncs.h"
#include "t2fit_fitwaves.h"
#include "constraints_covar.h"
#include <cstdio>
#include <cmath>
int main(){
 MAX_PSR=2;MAX_OBSN=4;auto*p=new pulsar[2];initialise(p,1);
 for(int i=0;i<2;i++){p[i].nobs=1;p[i].posPulsar[0]=1+i;p[i].posPulsar[1]=p[i].posPulsar[2]=0;for(int j=0;j<6;j++)p[i].obsn[0].planet_ssb[0][j]=j==0?10+10*i:0;}
 double a=t2FitFunc_planet(p,0,0,0,param_dmassplanet,0),b=t2FitFunc_planet(p,1,0,0,param_dmassplanet,0);
 printf("PLANET %.17g %.17g EXPECTED 10 40\n",a,b);
 p[0].nWhite=1;p[0].nWhite_dm=2;p[0].waveScale=0;p[0].param[param_wave_dm].val[0]=0.1;p[0].obsn[0].freqSSB=1.4e9;
 double w=t2FitFunc_fitwaves(p,0,3,0,param_wave_dm,2);printf("WAVE %.17g EXPECTED %.17g\n",w,cos(0.1*2*3)/(DM_CONST*1400*1400));
 // A padded standalone row makes the wrong index observable without an OOB read.
 // The actual getCholeskyMatrix caller allocates only ifuncN entries.
 p[0].ifuncN=2;double row[3]={3,4,99};
 printf("IFUNC %.17g %.17g EXPECTED 3 4\n",constraints_covar_ifunc(p,0,constraint_ifunc_cov,param_ifunc,0,0,row),constraints_covar_ifunc(p,0,constraint_ifunc_cov,param_ifunc,0,1,row));
}
