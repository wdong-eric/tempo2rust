// Controlled callback coordinates expose actual legacy global constraint writes.
#include "tempo2.h"
#include "TKfit.h"
#include "constraints.h"
#include <cstdio>
#include <cstdlib>
void ae_legacy_matrix(double**m,double**c,int nr,int nf,int nc) {
 printf("SHAPE %d %d %d\n",nr,nf,nc);
 for(int i=0;i<nc;i++)for(int j=0;j<nf;j++)printf("CONSTRAINT %d %d %.17g\n",i,j,c[i][j]);
}
void basis(double x,double*row,int n,pulsar*,int,int p) {
 row[0]=1;for(int j=1;j<n;j++)row[j]=row[j-1]*x;
}
int main(int argc,char**argv) {
 if(argc!=2)return 2;int mode=atoi(argv[1]);MAX_PSR=2;MAX_OBSN=16;auto*ps=new pulsar[2];initialise(ps,1);
 int n[2]={6,6},nf[2]={mode==1?1:3,3},ng=mode==1?2:0;
 double xx[2][6],yy[2][6],*x[]={xx[0],xx[1]},*y[]={yy[0],yy[1]};int ii[2][6],*ip[]={ii[0],ii[1]};double**lower[2];
 for(int p=0;p<2;p++) {
  auto&q=ps[p];q.nobs=6;q.robust=0;q.nconstraints=mode==2?0:1;q.constraint_efactor=1;
  q.param[param_pepoch].val[0]=54000;q.param[param_ifunc].paramSet[0]=1;q.param[param_ifunc].fitFlag[0]=(mode==1&&p==0)?2:1;
  q.ifuncN=2;q.ifuncT[0]=53999;q.ifuncT[1]=54005;q.constraints[0]=p==0?constraint_ifunc_0:constraint_ifunc_1;
  lower[p]=malloc_blas(1,6);
  for(int i=0;i<6;i++) {
   xx[p][i]=i/4.;yy[p][i]=i+1+.25*p;ii[p][i]=i;lower[p][0][i]=1;
   q.obsn[i].deleted=0;q.obsn[i].sat=54000+i;q.obsn[i].toaErr=1;
  }
 }
 int total=nf[0]+nf[1]+ng;double*p=new double[total],*e=new double[total],**c=malloc_blas(total,total),chi;
 TKleastSquares_global_pulsar(x,y,n,p,e,nf,ng,c,&chi,basis,ps,1e-27,ip,0,lower,2);
 return 0;
}
