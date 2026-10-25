// Focused storage-membership evidence. Sentinel values identify unintended reads;
// they are not physical defaults and never enter the public timing recipe.
#include "tempo2.h"
#include <cstdio>
#include <cstring>
#include <string>
static void number(const char* name,longdouble x){char b[128];quadmath_snprintf(b,sizeof b,"%.36Qg",x);printf("%s\t%s\n",name,b);}
int main(int argc,char**argv){
 if(argc!=5)return 2;MAX_PSR=1;MAX_OBSN=64;
 auto* p=new pulsar;p->AverageDMResiduals=763;p->nWhite_dm=41;initialise(p,0);
 number("default.AverageDMResiduals",p->AverageDMResiduals);number("default.nWhite_dm",p->nWhite_dm);
 // Isolate residual membership from the separate omitted-initializer defect.
 p->AverageDMResiduals=0;p->nWhite_dm=0;
 char par[1][MAX_FILELEN],tim[1][MAX_FILELEN];strcpy(par[0],argv[2]);strcpy(tim[0],argv[3]);
 readParfile(p,par,tim,1);readTimfile(p,tim,1);char* options[]={argv[0]};preProcess(p,1,1,options);formBatsAll(p,1);
 int n=p->nobs;double spare=strtod(argv[4],nullptr);
 for(int i=0;i<n;i++)p->obsn[i].TNRedSignal=(i+1)*1e-6;
 p->TNsubtractRed=1;
 // A native allocated spare row is deliberately fully defined, so its mistaken
 // inclusion is observable without reading unwritten memory in this experiment.
 p->obsn[n].TNRedSignal=spare;p->obsn[n].residual=0.125;p->obsn[n].residualtn=991;p->obsn[n].deleted=0;
 p->tzrobs.TNRedSignal=spare;p->tzrobs.deleted=0;
 formResiduals(p,1,1);
 for(int i=0;i<n;i++){std::string key="residualtn."+std::to_string(i);number(key.c_str(),p->obsn[i].residualtn);}
 number("spare.residualtn",p->obsn[n].residualtn);number("nobs",p->nobs);
 return 0;
}
