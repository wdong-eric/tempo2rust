#include "TKfit.h"
#include "TKmatrix.h"
#include <cstdio>
int main(){
  double **m=malloc_blas(3,1),**c=malloc_blas(1,1),**cov=malloc_blas(1,1);
  for(int i=0;i<3;i++)m[i][0]=1;
  c[0][0]=2;
  double y[3]={0,0,0},target[1],p[1],e[1];
  for(double t: {0.0,4.0}) {
    target[0]=t;
    double chi=TKrobustDefConstrainedLeastSquares(y,y,m,m,c,3,1,1,1e-27,0,p,e,cov,0,target);
    printf("TARGET %.17g P %.17g E %.17g C %.17g CHI %.17g\n",t,p[0],e[0],cov[0][0],chi);
  }
  double outlier[3]={1,2,100};
  for(char code: {'0','H','B','W','X'}){
    double chi=TKrobustDefConstrainedLeastSquares(outlier,outlier,m,m,nullptr,3,1,0,1e-27,0,p,e,cov,code,nullptr);
    printf("ROBUST %c P %.17g E %.17g C %.17g CHI %.17g\n",code,p[0],e[0],cov[0][0],chi);
  }
  double chi=TKrobustLeastSquares(outlier,outlier,m,m,3,1,1e-27,0,p,e,cov,'W');
  printf("WRAPPER W P %.17g E %.17g C %.17g CHI %.17g\n",p[0],e[0],cov[0][0],chi);
  free_blas(m);free_blas(c);free_blas(cov);
}
