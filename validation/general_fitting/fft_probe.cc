#include "TKspectrum.h"
#include <cstdio>
#include <cstdlib>
#include <vector>
int main(int argc,char**argv){
 if(argc!=3)return 2;int d=atoi(argv[1]),n=atoi(argv[2]);std::vector<double>re(n),im(n);
 for(int i=0;i<n;i++){re[i]=((i*7%19)-9)/8.0;im[i]=((i*3%17)-8)/16.0;}
 TK_fft(d,n,re.data(),im.data());
 for(int i=0;i<n;i++)printf("%d %.17g %.17g\n",i,re[i],im[i]);
}
