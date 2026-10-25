// Fully initialized neighboring copyPSR case: distinguish reserved slot zero
// from the one-based active jump records without reading undefined memory.
#include "tempo2.h"
#include <cstdio>
int main(){
 auto*p=new pulsar[2]();p[0].nJumps=1;p[1].nfdJumps=1;
 for(int i=0;i<2;i++){
  p[0].jumpVal[i]=10+i;p[0].fdjumpVal[i]=20+i;
  p[1].jumpVal[i]=30+i;p[1].fdjumpVal[i]=40+i;
 }
 copyPSR(p,0,1);
 printf("JUMP %.17g %.17g FDJUMP %.17g %.17g\n",p[1].jumpVal[0],p[1].jumpVal[1],p[1].fdjumpVal[0],p[1].fdjumpVal[1]);
 delete[] p;
}
