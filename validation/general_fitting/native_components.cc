// Standalone helper calls on the same real reader/preprocessing/timing route.
#include "tempo2.h"
#include "constraints.h"
#include <cstdio>
#include <cstdlib>
#include <cstring>
void matrixDMConstraintWeights(pulsar*);
int ae_solver_owner=-2;
void ae_svd(int,int,int,const longdouble*,double){}
void ae_update(pulsar*,int,int,int,const char*){}
void ae_prepared(int,pulsar*,const double*,const double*,const double*,const int*,int,double**,double**,double*,double**){}
int main(int argc,char**argv){
 if(argc<5)return 2;
 MAX_PSR=1;MAX_OBSN=512;auto*p=new pulsar[1];initialise(p,1);
 char par[1][MAX_FILELEN],tim[1][MAX_FILELEN];strcpy(par[0],argv[3]);strcpy(tim[0],argv[4]);
 readParfile(p,par,tim,1);readTimfile(p,tim,1);char*args[]={argv[0]};preProcess(p,1,1,args);formBatsAll(p,1);formResiduals(p,1,1);
 if(strcmp(argv[2],"grid-weights")==0)matrixDMConstraintWeights(p);
 else if(strcmp(argv[2],"grid-auto")==0 && argc==10)autosetDMCM(p,atof(argv[5]),atof(argv[6]),atof(argv[7]),atof(argv[8]),atoi(argv[9]));
 else return 2;
 auto*out=fopen(argv[1],"w");if(!out)return 3;
 for(int i=0;i<p->dmoffsDMnum;i++)fprintf(out,"DM %d %.17g %.17g %.17g\n",i,p->dmoffsDM_mjd[i],p->dmoffsDM[i],p->dmoffsDM_weight[i]);
 for(int i=0;i<p->dmoffsCMnum;i++)fprintf(out,"CM %d %.17g %.17g %.17g\n",i,p->dmoffsCM_mjd[i],p->dmoffsCM[i],p->dmoffsCM_weight[i]);
 fclose(out);return 0;
}
