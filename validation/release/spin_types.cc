// 0109: actual native callbacks; the private observed copy is generated from A2.
#include "tempo2.h"
#include "t2fit_stdFitFuncs.h"
#include <cassert>
#include <cfloat>
#include <cmath>
#include <cstdio>
#include <fstream>
#include <string>
#include <type_traits>
static std::string case_name;
static void ordinary(const char* key, long double value) {
    printf("%s\t%s\t%.40Lg\n",case_name.c_str(),key,value);
}
static void wide(const char* key, longdouble value) {
    char text[128];quadmath_snprintf(text,sizeof text,"%.40Qg",value);
    printf("%s\t%s\t%s\n",case_name.c_str(),key,text);
}
static_assert(std::is_same_v<decltype(1.0L/24.0L*double{}),long double>);
static_assert(std::is_same_v<decltype(powl(double{},6.0L)),longdouble>);
static_assert(std::is_same_v<decltype(double{}/pow(24.0*3600.0,10)),double>);
#ifdef AL_SPIN_COPY
#include "spin_functions.inc"
#define SPIN al_spin
#define UPDATE al_update
#else
#define SPIN t2FitFunc_stdFreq
#define UPDATE t2UpdateFunc_stdFreq
#endif
int main(int argc,char**argv) {
    if(argc!=2)return 2;
    printf("profile\tordinary_bits\t%d\nprofile\ttiming_bits\t%d\n",LDBL_MANT_DIG,FLT128_MANT_DIG);
    MAX_PSR=1;auto p=new pulsar;initialise(p,0);
    p->param[param_f].val[0]=strtoflt128("123.45678901234567890123456789",nullptr);
    p->param[param_f].val[1]=strtoflt128("-1.2345678901234567890123456789e-15",nullptr);
    std::ifstream input(argv[1]);std::string name;double x;int brake;
    while(input>>name>>x>>brake) {
        p->param[param_brake].paramSet[0]=brake;
        p->param[param_brake].val[0]=3; // Source deliberately initializes bindex to zero.
        for(int k=0;k<=14;++k) {
            case_name=name+"-F"+std::to_string(k);
            double value=SPIN(p,0,x,0,param_f,k);
            ordinary("callback",value);
            if(k<4 && brake==0)ordinary("bounded",value);
        }
        case_name=name+"-brake";ordinary("callback",SPIN(p,0,x,0,param_brake,0));
    }
    if(!input.eof())return 3;
    for(int k:{9,10})for(double val:{-0.125,0.125}) {
        case_name="update"+std::to_string(k)+(val<0?"-negative":"-positive");
        p->param[param_f].val[k]=0;p->param[param_f].err[k]=0;
        UPDATE(p,0,param_f,k,val,0.25);
        wide("value",p->param[param_f].val[k]);wide("error",p->param[param_f].err[k]);
    }
    destroyOne(p);delete p;
}
