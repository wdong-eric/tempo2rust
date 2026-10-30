// Source formats: polyco.C:432/456 and t1polyco.c:106/169. No timing equations.
#include <cstdio>
#include <cstdlib>
#include <cfloat>
#include <quadmath.h>
int main() {
    const char* values[]={"0.1","-0.3","1.000000000000000000108420217248550443400745280086994171142578125",
                          "1.00000000000000011102230246251565404236316680908203125","1e-90","-1e90"};
    for(auto text:values) {
        char token[160],input[200],parsed[200];
        double d=strtod(text,nullptr);long double l=strtold(text,nullptr);__float128 q=strtoflt128(text,nullptr);
        for(int kind=0;kind<3;kind++) {
            int bits=kind==0?DBL_MANT_DIG:kind==1?LDBL_MANT_DIG:FLT128_MANT_DIG;
            if(kind==0){snprintf(token,sizeof token,"%.17e",d);snprintf(input,sizeof input,"%.100Lg",(long double)d);}
            if(kind==1){snprintf(token,sizeof token,"%.17Le",l);snprintf(input,sizeof input,"%.100Lg",l);}
            if(kind==2){quadmath_snprintf(token,sizeof token,"%.30Qe",q);quadmath_snprintf(input,sizeof input,"%.100Qg",q);}
            snprintf(parsed,sizeof parsed,"%.100Lg",strtold(token,nullptr));
            printf("%d\t%d\t%d\t%s\t%s\t%s\n",kind,bits,LDBL_MANT_DIG,input,token,parsed);
        }
    }
}
