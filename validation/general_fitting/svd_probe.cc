#include "tempo2.h"
#include "TKsvd.h"
#include "TKmatrix.h"
#include "TKfit.h"
#include <cstdio>
static void value(const char*key,int i,int j,longdouble v){
 char text[128];quadmath_snprintf(text,sizeof text,"%.36Qg",v);printf("%s %d %d %s\n",key,i,j,text);
}
int main(){
 const int n=5,nf=3;double data[n][nf]={{1,2,.5},{2,-1,3},{.5,4,-2},{-1,.25,1},{3,1,2}};
 longdouble**a=malloc_2dLL(n,nf),**v=malloc_2dLL(nf,nf),**unused=malloc_2dLL(n,nf);
 longdouble w[nf],rv[nf],norm,b[n],x[nf];
 for(int i=0;i<n;i++)for(int j=0;j<nf;j++)a[i][j]=data[i][j];
 TKbidiagonal(a,&norm,n,nf,v,w,unused,rv);
 value("bidiagonal.norm",-1,-1,norm);
 for(int j=0;j<nf;j++){value("bidiagonal.diagonal",j,-1,w[j]);value("bidiagonal.superdiagonal",j,-1,rv[j]);}
 for(int i=0;i<n;i++)for(int j=0;j<nf;j++)value("bidiagonal.u",i,j,a[i][j]);
 for(int i=0;i<nf;i++)for(int j=0;j<nf;j++)value("bidiagonal.v",i,j,v[i][j]);
 for(int i=0;i<n;i++)for(int j=0;j<nf;j++)a[i][j]=data[i][j];
 TKsingularValueDecomposition_lsq(a,n,nf,v,w,unused);
 for(int j=0;j<nf;j++)value("decomposition.singular_values",j,-1,w[j]);
 for(int i=0;i<n;i++)for(int j=0;j<nf;j++)value("decomposition.u",i,j,a[i][j]);
 for(int i=0;i<nf;i++)for(int j=0;j<nf;j++)value("decomposition.v",i,j,v[i][j]);
 const char*rhs[]={"1e-400","-2e-400","3e-400","4e-400","-1e-400"};
 for(int i=0;i<n;i++)b[i]=strtoflt128(rhs[i],NULL);
 TKbacksubstitution_svd(v,w,a,b,x,n,nf);for(int j=0;j<nf;j++)value("solution",j,-1,x[j]);
 w[1]=0;TKbacksubstitution_svd(v,w,a,b,x,n,nf);for(int j=0;j<nf;j++)value("dropped_solution",j,-1,x[j]);
 const char*pairs[][2]={{"0","0"},{"-3","4"},{"1e300","1e-300"},{"1e-400","2e-400"}};
 for(int i=0;i<4;i++)value("pythag",i,-1,TKpythag(strtoflt128(pairs[i][0],NULL),strtoflt128(pairs[i][1],NULL)));
 longdouble maxima[]={strtoflt128("-1e400",NULL),strtoflt128("2e400",NULL),strtoflt128("1e400",NULL)};
 value("maximum",-1,-1,TKfindMax_Ld(maxima,3));
 free_2dLL(a);free_2dLL(v);free_2dLL(unused);
}
