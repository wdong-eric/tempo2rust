// 0119: retain the ordinary profile and observe only its nonfinite event paths.
#define main event_profile_main
#include "event_types.cc"
#undef main

// Runtime operands and a separate observer prevent constant folding or inferred
// exp/sign facts from replacing the actual ordinary/quad result being observed.
template<class T> __attribute__((noipa)) T product(T a,T b) { return a*b; }
template<class T> __attribute__((noipa)) T quotient(T a,T b) { return a/b; }

int main(int argc,char**argv) {
    int status=event_profile_main(argc,argv);if(status)return status;
    const char* tokens[]={"nan","-nan","0","-0","inf","-inf","1","-1"};
    for(auto left:tokens) {
        long double a=strtold(left,nullptr);longdouble aw=strtoflt128(left,nullptr);
        std::string name=std::string("assign_")+left;
        emit(name,"ordinary_to_wide",(longdouble)a);
        emit(name,"wide_to_ordinary",(long double)aw);
        for(auto right:tokens) {
            long double b=strtold(right,nullptr);longdouble bw=strtoflt128(right,nullptr);
            name=std::string("arithmetic_")+left+"_"+right;
            emit(name,"ordinary_mul",product(a,b));emit(name,"ordinary_div",quotient(a,b));
            emit(name,"wide_mul",product(aw,bw));emit(name,"wide_div",quotient(aw,bw));
        }
    }
    std::ifstream input(argv[1]);std::string name,b,e,a,w,g;int set;double frequency;
    while(input>>name>>b>>e>>a>>w>>g>>set>>frequency) {
        if(name!="zero_amplitude" && name!="negative_zero_amplitude" && name!="zero_width" && name!="zero_width_onset")continue;
        long double dt=strtoflt128(b.c_str(),nullptr)-strtoflt128(e.c_str(),nullptr);
        long double amp=strtoflt128(a.c_str(),nullptr),width=strtoflt128(w.c_str(),nullptr),freq=frequency/1.4e9;
        long double gamma=set?strtoflt128(g.c_str(),nullptr):0;
        // t2fit_glitch.C:239: each prefix retains the original type and order.
        long double gm=-dt*dt,half=gm/2.,div1=half/width,arg=div1/width;
        longdouble ap=amp*powl(freq,gamma),wide=ap*exp(arg);long double val=wide;
        emit(name,"trace_gaus_mul",gm);emit(name,"trace_gaus_half",half);
        emit(name,"trace_gaus_div1",div1);emit(name,"trace_gaus_arg",arg);
        emit(name,"trace_gaus_amp_power",ap);emit(name,"trace_gaus_wide",wide);emit(name,"trace_gaus_val",val);
        emit(name,"trace_gaus_amp_return",val/amp);
        long double epoch_factor=-dt/width/width,width_factor=dt*dt/6./width/width/width;
        emit(name,"trace_gaus_epoch_factor",epoch_factor);emit(name,"trace_gaus_epoch_return",val*epoch_factor);
        emit(name,"trace_gaus_width_factor",width_factor);emit(name,"trace_gaus_width_return",val*width_factor);
        emit(name,"trace_gaus_index_return",val*log(freq));
        // The exponential callbacks do not evaluate these expressions at onset.
        if(dt>0) {
            gamma=set?strtoflt128(g.c_str(),nullptr):-2;
            longdouble base=amp*powl(freq,gamma)*expl(-dt/width),numerator=base*dt,first=numerator/width;
            emit(name,"trace_exp_base",base);emit(name,"trace_exp_tau_numerator",numerator);
            emit(name,"trace_exp_tau_div1",first);emit(name,"trace_exp_tau_div2",first/width);
            emit(name,"trace_exp_epoch_return",base/width);
        }
    }
    return input.eof()?0:3;
}
