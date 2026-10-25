#include "tempo2.h"
#include "t2fit.h"
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cstdint>
double random(long*);
static FILE* output;
static int iteration;
#include "native_observer_stubs.h"
void ae_bootstrap_begin(int i){
 if(i>=4096){fprintf(output,"BOUNDARY MAX_ITER %d\n",i);fflush(output);exit(86);}
 iteration=i;
}
void ae_bootstrap_sample(int i,int j){fprintf(output,"S %d %d %d\n",iteration,i,j);}
static void parameters(pulsar*p,char stage){
 for(int l=0;l<MAX_PARAMS;l++)for(int k=0;k<p->param[l].aSize;k++)if(p->param[l].fitFlag[k]==1){
  char value[128],error[128];quadmath_snprintf(value,sizeof value,"%.36Qg",p->param[l].val[k]);quadmath_snprintf(error,sizeof error,"%.36Qg",p->param[l].err[k]);
  fprintf(output,"%c %d %d %d %s %s\n",stage,iteration,l,k,value,error);
 }
}
void ae_bootstrap_fit(pulsar*p){parameters(p,'P');}
static void observations(pulsar*ps,int n,const char*stage){
 for(int p=0;p<n;p++)for(int i=0;i<ps[p].nobs;i++){
  auto&o=ps[p].obsn[i];char sat[128],bat[128],bbat[128];
  quadmath_snprintf(sat,sizeof sat,"%.36Qg",o.sat);quadmath_snprintf(bat,sizeof bat,"%.36Qg",o.bat);quadmath_snprintf(bbat,sizeof bbat,"%.36Qg",o.bbat);
  fprintf(output,"O %s %d %d %s %s %s %.17g %.17g %.17g %d %s %d",stage,p,i,sat,bat,bbat,o.freq,o.freqSSB,o.toaErr,o.deleted,o.fname,o.nFlags);
  for(int j=0;j<o.nFlags;j++)fprintf(output," %s %s",o.flagID[j],o.flagVal[j]);
  // State-history probe uses terrestrial input, where all six vector slots
  // have producers. The observer omits vectors for barycentric input.
  if(strcmp(o.telID,"@")!=0)for(int j=0;j<6;j++)fprintf(output," %.17g %.17g",o.earth_ssb[j],o.observatory_earth[j]);
  fprintf(output,"\n");
 }
}
int main(int argc,char**argv){
 if(argc==4 && strcmp(argv[1],"rng")==0){
  long seed=atol(argv[2]);for(int i=0;i<atoi(argv[3]);i++){
   double x=random(&seed);uint64_t bits;memcpy(&bits,&x,sizeof bits);printf("%016llx\n",(unsigned long long)bits);
  }printf("SEED %ld\n",seed);return 0;
 }
 if(argc!=4&&argc!=6)return 2;output=fopen(argv[1],"w");if(!output)return 3;
 int n=(argc-2)/2;MAX_PSR=n+1;MAX_OBSN=512;auto*ps=new pulsar[n+1];initialise(ps,1);
 char par[2][MAX_FILELEN],tim[2][MAX_FILELEN];for(int p=0;p<n;p++){strcpy(par[p],argv[2+2*p]);strcpy(tim[p],argv[3+2*p]);}
 readParfile(ps,par,tim,n);readTimfile(ps,tim,n);char*args[]={argv[0]};preProcess(ps,n,1,args);
 for(int it=0;it<ps[0].nits;it++){
  if(it)for(int p=0;p<n;p++)for(int l=0;l<MAX_PARAMS;l++)for(int k=0;k<ps[p].param[l].aSize;k++){ps[p].param[l].prefit[k]=ps[p].param[l].val[k];ps[p].param[l].prefitErr[k]=ps[p].param[l].err[k];}
  formBatsAll(ps,n);formResiduals(ps,n,1);t2Fit(ps,n,NULL);formBatsAll(ps,n);formResiduals(ps,n,1);
 }
 observations(ps,n,"before");bootstrap(ps,n-1,n);observations(ps,n,"after");parameters(ps+n-1,'F');fclose(output);return 0;
}
