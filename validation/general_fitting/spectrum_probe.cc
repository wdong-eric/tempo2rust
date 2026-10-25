// T2fitSpectra state and stdin prompt history, in a fresh process per scenario.
#include "choleskyRoutines.h"
#include <cstdio>
#include <cstdlib>
int main(int argc,char**argv) {
 if(argc!=2)return 2;int scenario=atoi(argv[1]);
 double x[]={.125,.25,.5,1,2,4,8,16},y[]={4,1,.25,.125,.0625,.03125,.015625,.0078125};
 double alpha=3,fc=.2,scale=-10,var=-20,beta=.5;int nfit=4;
 for(int call=0;call<(scenario==0?4:1);call++) {
  int automatic=scenario==0&&call==0;
  double corner=scenario==0?(call<2?.2:-1):-2;
  double exponent=scenario==0?(call<2?3:0):-2;
  int points=scenario==0?(call<2?4:-1):(scenario==1?-2:1);
  double cutoff=scenario==2?4:(scenario==3?1:.2);
  double amplitude=scenario==4?8:-1;
  int usebeta=scenario==0&&call==3;
  int result=T2fitSpectra(x,y,8,&alpha,&fc,&nfit,&scale,&var,automatic,0,corner,exponent,points,amplitude,usebeta,&beta,cutoff);
  printf("STATE %d %d %.17g %.17g %d %.17g %.17g %.17g\n",call,result,alpha,fc,nfit,scale,var,beta);
 }
}
