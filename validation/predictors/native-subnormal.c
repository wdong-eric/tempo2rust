#include <quadmath.h>
#include <stdint.h>
#include <string.h>
#include <stdio.h>
static uint64_t bits(double d){uint64_t u;memcpy(&u,&d,8);return u;}
static double value(uint64_t u){double d;memcpy(&d,&u,8);return d;}
int main(void){volatile __float128 x=ldexpq(1,-1075)+ldexpq(1,-1180);volatile double a=value(0x0010000000000001ULL),b=value(0x3c9fffffffffffffULL);printf("conversion native=%016llx\nmultiply native=%016llx\n",(unsigned long long)bits((double)x),(unsigned long long)bits(a*b));}
