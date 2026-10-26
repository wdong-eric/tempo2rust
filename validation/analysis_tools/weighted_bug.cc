#include "tempo2.h"
#include "TKspectrum.h"
#include <cstdio>
int main(){
 double x[]={0,0.3,1.1,2.7,4.2,6.0,7.4,10}, y[8],e[]={1,2,1.5,0.8,1.2,2.1,0.9,1.4},f[8],p[8],re[8],im[8];int n;
 double w=2*M_PI/(10.0*8.0/7.0);
 for(int i=0;i<8;i++)y[i]=2*sin(w*x[i])+3*cos(w*x[i]);
 TK_weightLS(x,y,e,8,f,p,&n,re,im);
 printf("original %.17g %.17g intended 3 2\n",re[0],im[0]);
 // The same two-column normal equations, evaluated independently in binary64.
 double ss=0,cc=0,sc=0,ys=0,yc=0;
 for(int i=0;i<8;i++){double s=sin(w*x[i]),c=cos(w*x[i]),wt=1/e[i]/e[i];ss+=s*s*wt;cc+=c*c*wt;sc+=s*c*wt;ys+=y[i]*s*wt;yc+=y[i]*c*wt;}
 double b=(yc-ys*sc/ss)/(cc-sc*sc/ss),a=(ys-b*sc)/ss;
 printf("normal_equations %.17g %.17g cross %.17g\n",b,a,sc);
}
