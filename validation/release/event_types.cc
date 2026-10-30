// Actual source callback calls and ordinary-operation observations, decision 0104.
#include "tempo2.h"
#include "t2fit_glitch.h"
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <string>
#include <type_traits>
#include <cfloat>
static double frequency;
static long double dt,amp,width,freq,gamma,val;
static longdouble bbat,epoch;
static_assert(std::is_same_v<decltype(frequency/1.4e9),double>);
static_assert(std::is_same_v<decltype(bbat-epoch),longdouble>);
static_assert(std::is_same_v<decltype(powl(freq,gamma)),longdouble>);
static_assert(std::is_same_v<decltype(exp(-dt/width)),long double>);
static_assert(std::is_same_v<decltype(expl(-dt/width)),long double>);
static_assert(std::is_same_v<decltype(log(freq)),long double>);
static_assert(std::is_same_v<decltype(amp*log(freq)),long double>);
static_assert(std::is_same_v<decltype(val=amp*powl(freq,gamma)*expl(-dt/width)),long double&>);
static_assert(std::is_same_v<decltype(t2FitFunc_expdip(nullptr,0,0,0,param_expph,0)),double>);
static void emit(const std::string& name,const char* key,long double value) {
    printf("%s\t%s\t%.40Lg\n",name.c_str(),key,value);
}
int main(int argc,char**argv) {
    if(argc!=2)return 2;
    printf("profile\tordinary_bits\t%d\n",LDBL_MANT_DIG);
    printf("profile\ttiming_bits\t%d\n",FLT128_MANT_DIG);
    MAX_PSR=1;auto p=new pulsar;initialise(p,0);p->nobs=1;
    std::ifstream input(argv[1]);std::string name,b,e,a,w,g;int set;
    while(input>>name>>b>>e>>a>>w>>g>>set>>frequency) {
        bbat=strtoflt128(b.c_str(),nullptr);epoch=strtoflt128(e.c_str(),nullptr);
        p->obsn[0].bbat=bbat;p->obsn[0].freqSSB=frequency;
        for(auto label:{param_expep,param_gausep})p->param[label].val[0]=epoch;
        for(auto label:{param_expph,param_gausamp})p->param[label].val[0]=strtoflt128(a.c_str(),nullptr);
        for(auto label:{param_exptau,param_gaussig})p->param[label].val[0]=strtoflt128(w.c_str(),nullptr);
        for(auto label:{param_expindex,param_gausindex}){p->param[label].val[0]=strtoflt128(g.c_str(),nullptr);p->param[label].paramSet[0]=set;}
        dt=bbat-epoch;amp=p->param[param_expph].val[0];width=p->param[param_exptau].val[0];freq=frequency/1.4e9;
        long double exp_arg=-dt/width,gaus_arg=-dt*dt/2./width/width;
        gamma=set?p->param[param_gausindex].val[0]:0;
        val=amp*powl(freq,gamma)*exp(gaus_arg);
        emit(name,"freq",freq);emit(name,"dt",dt);emit(name,"amp",amp);emit(name,"width",width);
        emit(name,"exp_arg",exp_arg);emit(name,"exp",exp(exp_arg));emit(name,"expl",expl(exp_arg));
        emit(name,"log",log(freq));emit(name,"gaus_arg",gaus_arg);emit(name,"gaus_exp",exp(gaus_arg));emit(name,"gaus_val",val);
        for(auto label:{param_expph,param_exptau,param_expindex,param_expep,param_gausamp,param_gausep,param_gaussig,param_gausindex}) {
            double result=label==param_expph||label==param_exptau||label==param_expindex||label==param_expep
                ?t2FitFunc_expdip(p,0,0,0,label,0):t2FitFunc_gausdip(p,0,0,0,label,0);
            std::string key="callback_"+std::to_string((int)label);emit(name,key.c_str(),result);
        }
    }
    if(!input.eof())return 3;
    destroyOne(p);delete p;
}
