// Native wire-profile witness; no library functions or invalid memory are used.
#include "GWsim.h"
#include <cstddef>
#include <cstdio>
#include <cstring>
#include <limits>
int main() {
  const unsigned int endian=1;
  printf("int=%zu long=%zu scalar=%zu align=%zu little=%d\n",sizeof(int),sizeof(long),sizeof(longdouble),alignof(longdouble),*(const unsigned char*)&endian);
  printf("tensor=%zu prefix_end=%zu general=%zu prefix_end=%zu\n",sizeof(gwSrc),offsetof(gwSrc,phi_bin),sizeof(gwgeneralSrc),offsetof(gwgeneralSrc,phi_bin));
  longdouble values[]={0.0Q,-0.0Q,1.0Q,-2.5Q,0x1p-16494Q,0x1p-16382Q,HUGE_VALQ,-HUGE_VALQ,nanq("")};
  for(auto x:values){unsigned char b[16];memcpy(b,&x,16);for(auto v:b)printf("%02x",v);puts("");}
}
