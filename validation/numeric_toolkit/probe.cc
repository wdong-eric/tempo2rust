// Assignment M: declared caller storage only; raw TEMPO2 units compile separately.
#include "tempo2.h"
#include "tempo2Util.h"
#include <cassert>
#include <cfloat>
#include <climits>
#include <cstdarg>
#include <cstring>
#include <cerrno>
#include <cmath>
#include <fstream>
#include <sstream>
#include <vector>
#include <string>
#include <type_traits>
#include <cstddef>
#include <cstdint>
// Neither selected header declares these C++ entry points.
extern int ld_vsprintf(char*,const char*,va_list);
extern double fortran_mod(double,double);
static_assert(std::is_same<decltype(fortran_mod(0.0,1.0)),double>::value,"binary64 overload");
static_assert(std::is_same<decltype(fortran_mod(0.0Q,1.0Q)),longdouble>::value,"binary128 overload");
static_assert(std::is_same<decltype(pow(1.0Q,2.0)),longdouble>::value,"binary64 exponent to binary128 pow");

static std::string q(const std::string &s){
    std::string o="\"";for(unsigned char c:s){
        if(c=='"'||c=='\\'){o+='\\';o+=c;}
        else if(c<32||c>=127){char b[8];sprintf(b,"\\u%04x",c);o+=b;}else o+=c;
    }return o+'"';
}
static std::string hex(const void*p,size_t n){
    static const char*d="0123456789abcdef";std::string s;
    for(size_t i=0;i<n;i++){unsigned char c=((const unsigned char*)p)[i];s+=d[c>>4];s+=d[c&15];}return s;
}
static std::string num(longdouble v){
    if(isnanq(v))return "{\"class\":\"NaN\",\"type\":\"binary128\"}";
    if(isinfq(v))return "{\"class\":"+q(signbitq(v)?"-Infinity":"+Infinity")+",\"type\":\"binary128\"}";
    char b[128];quadmath_snprintf(b,sizeof(b),"%.36Qg",v);
    longdouble r=strtoflt128(b,nullptr);assert(r==v&&!!signbitq(r)==!!signbitq(v));
    return "{\"class\":\"finite\",\"type\":\"binary128\",\"decimal\":"+q(b)+",\"bytes_le\":"+q(hex(&v,sizeof v))+"}";
}
static std::string num(double v){
    if(std::isnan(v))return "{\"class\":\"NaN\",\"type\":\"binary64\"}";
    if(std::isinf(v))return "{\"class\":"+q(std::signbit(v)?"-Infinity":"+Infinity")+",\"type\":\"binary64\"}";
    char b[80];sprintf(b,"%.17g",v);double r=strtod(b,nullptr);assert(r==v&&std::signbit(r)==std::signbit(v));
    return "{\"class\":\"finite\",\"type\":\"binary64\",\"decimal\":"+q(b)+",\"bytes_le\":"+q(hex(&v,sizeof v))+"}";
}
static std::string array(double*v,int n){std::string s="[";for(int i=0;i<n;i++){if(i)s+=',';s+=num(v[i]);}return s+"]";}
static std::string bytes(const void*p,size_t n){
    // Lossless sparse encoding of initialized field bytes; omitted bytes are zero.
    auto b=(const unsigned char*)p;std::string s="[";bool comma=false;
    for(size_t i=0;i<n;){if(!b[i]){i++;continue;}size_t j=i+1;while(j<n&&b[j])j++;
        if(comma)s+=',';comma=true;s+="["+std::to_string(i)+","+q(hex(b+i,j-i))+"]";i=j;}
    return s+"]";
}
static std::string mask(const void*a,const void*b,size_t n){
    auto x=(const unsigned char*)a;auto y=(const unsigned char*)b;std::string s="[";bool comma=false;
    for(size_t i=0;i<n;){if(x[i]==y[i]){i++;continue;}size_t j=i+1;while(j<n&&x[j]!=y[j])j++;
        if(comma)s+=',';comma=true;s+="["+std::to_string(i)+","+std::to_string(j)+"]";i=j;}
    return s+"]";
}
static std::string field(const char*n,size_t off,const void*src,const void*before,const void*after,size_t len,bool pointer=false){
    return "{\"field\":"+q(n)+",\"offset\":"+std::to_string(off)+",\"bytes\":"+std::to_string(len)+
        ",\"retained\":"+(memcmp(before,after,len)==0?"true":"false")+
        ",\"equals_source\":"+(memcmp(src,after,len)==0?"true":"false")+
        ",\"changed_byte_ranges\":"+mask(before,after,len)+
        (pointer?",\"representation\":\"pointer identities omitted; retention checked\"":",\"source\":"+bytes(src,len)+",\"before\":"+bytes(before,len)+",\"after\":"+bytes(after,len))+"}";
}
struct ParamStorage {
    parameter p{};longdouble val[4]{},err[4]{},pre[4]{},preerr[4]{};int fit[4]{},set[4]{};
    char label[4][64]{},shortlabel[4][64]{};char *lp[4],*sp[4];
    ParamStorage(int seed,int size){
        p.aSize=size;p.val=val;p.err=err;p.prefit=pre;p.prefitErr=preerr;p.fitFlag=fit;p.paramSet=set;p.label=lp;p.shortlabel=sp;
        p.nLinkTo=1;p.nLinkFrom=2;
        for(int i=0;i<5;i++){p.linkTo[i]=seed+i;p.linkFrom[i]=seed+10+i;}
        for(int i=0;i<4;i++){val[i]=seed+i+0.125Q;err[i]=seed+i+0.25Q;pre[i]=seed+i+0.5Q;preerr[i]=seed+i+0.75Q;fit[i]=seed+i;set[i]=seed+i+4;
            sprintf(label[i],"label-%d-%d",seed,i);sprintf(shortlabel[i],"short-%d-%d",seed,i);lp[i]=label[i];sp[i]=shortlabel[i];}
    }
    std::string state(){
        std::string s="{\"aSize\":"+std::to_string(p.aSize)+",\"nLinkTo\":"+std::to_string(p.nLinkTo)+",\"nLinkFrom\":"+std::to_string(p.nLinkFrom)+",\"linkTo\":[";
        for(int i=0;i<5;i++){if(i)s+=',';s+=std::to_string(p.linkTo[i]);}s+="],\"linkFrom\":[";
        for(int i=0;i<5;i++){if(i)s+=',';s+=std::to_string(p.linkFrom[i]);}s+="],\"capacity\":4,\"cells\":[";
        for(int i=0;i<4;i++){if(i)s+=',';s+="{\"val\":"+num(val[i])+",\"err\":"+num(err[i])+",\"prefit\":"+num(pre[i])+",\"prefitErr\":"+num(preerr[i])+",\"fitFlag\":"+std::to_string(fit[i])+",\"paramSet\":"+std::to_string(set[i])+",\"label_bytes\":"+q(hex(label[i],64))+",\"shortlabel_bytes\":"+q(hex(shortlabel[i],64))+"}";}
        return s+"]}";
    }
};
static std::string copyparam(int count){
    ParamStorage src(10,count),dst(70,4);auto sb=src.state(),db=dst.state();parameter pointers=dst.p;
    copyParam(src.p,&dst.p);
    assert(pointers.val==dst.p.val&&pointers.err==dst.p.err&&pointers.prefit==dst.p.prefit&&pointers.prefitErr==dst.p.prefitErr&&pointers.label==dst.p.label&&pointers.shortlabel==dst.p.shortlabel&&pointers.fitFlag==dst.p.fitFlag&&pointers.paramSet==dst.p.paramSet);
    return "{\"source_before\":"+sb+",\"source_after\":"+src.state()+",\"destination_before\":"+db+",\"destination_after\":"+dst.state()+",\"pointers_retained\":true}";
}
static std::string copypsr(int nobs,int nj,int nfd){
    pulsar *p=new pulsar[2]{};memset(p,0,sizeof(pulsar)*2);
    observation obs[2][3]{};memset(obs,0,sizeof obs);ParamStorage param0(10,2),param1(70,3);
    p[0].param[0]=param0.p;p[1].param[0]=param1.p;
    for(int k=0;k<2;k++){
        auto &v=p[k];int base=k?70:10;v.obsn=obs[k];v.nobs=k?3:nobs;v.nJumps=k?3:nj;v.nfdJumps=k?nfd:1;
        sprintf(v.name,"caller-%d",base);sprintf(v.binaryModel,"model-%d",base);sprintf(v.clock,"clock-%d",base);
        sprintf(v.clockFromOverride,"override-%d",base);sprintf(v.JPL_EPHEMERIS,"jpl-%d",base);sprintf(v.ephemeris,"eph-%d",base);
        v.fitMode=base;v.nits=base+1;v.eclCoord=base+2;v.nFit=base+3;v.offset=base+0.25;
        v.calcShapiro=base+4;v.units=base+5;v.tempo1=base+6;v.dilateFreq=base+7;v.timeEphemeris=base+8;v.t2cMethod=base+9;v.correctTroposphere=base+10;
        v.dm_series_type=k?series_taylor_pn:series_simple_pn;v.offset_e=base+0.5;v.ne_sw=base+0.75;v.fdjump_log=k;v.fdjump_pint_format=k;
        for(int i=0;i<3;i++){v.posPulsar[i]=base+i+0.125;v.velPulsar[i]=base+i+0.25;v.accPulsar[i]=base+i+0.5;
            obs[k][i].sat=base+i+0.125Q;obs[k][i].residual=base+i+0.25Q;obs[k][i].freq=base+i+0.5;obs[k][i].pulseN=base+i;
            sprintf(obs[k][i].fname,"obs-%d-%d",base,i);sprintf(obs[k][i].telID,"tel-%d",base);obs[k][i].nFlags=1;strcpy(obs[k][i].flagID[0],"-x");sprintf(obs[k][i].flagVal[0],"%d",base);}
        for(int i=0;i<4;i++){v.jumpVal[i]=base+i+0.125;v.jumpValErr[i]=base+i+0.25;v.fitJump[i]=base+i;v.jumpSAT[i]=k;v.jumpScaled[i]=k;
            sprintf(v.jumpStr[i],"jump-%d-%d",base,i);v.fdjumpVal[i]=base+i+0.5;v.fdjumpValErr[i]=base+i+0.75;v.fitfdJump[i]=base+i;v.fdjumpIdx[i]=base+i+4;sprintf(v.fdjumpStr[i],"fdjump-%d-%d",base,i);}
    }
    pulsar *old=new pulsar[2];memcpy(old,p,sizeof(pulsar)*2);observation ob[2][3];memcpy(ob,obs,sizeof obs);
    std::string ps0=param0.state(),ps1=param1.state();copyPSR(p,0,1);
    std::string s="{\"source_object_unchanged\":"+std::string(memcmp(p,old,sizeof(pulsar))==0?"true":"false")+",\"source_observations_unchanged\":"+(memcmp(obs[0],ob[0],sizeof obs[0])==0?"true":"false")+",\"fields\":[";bool comma=false;
#define FIELD(f) if(comma)s+=',';comma=true;s+=field(#f,offsetof(pulsar,f),&p[0].f,&old[1].f,&p[1].f,sizeof(p[1].f),std::is_pointer<decltype(p[1].f)>::value||std::string(#f)=="param")
#include "pulsar_fields.h"
#undef FIELD
    s+="],\"observations\":[";
    for(int i=0;i<3;i++){if(i)s+=',';s+='[';comma=false;
#define FIELD(f) if(comma)s+=',';comma=true;s+=field(#f,offsetof(observation,f),&obs[0][i].f,&ob[1][i].f,&obs[1][i].f,sizeof(obs[1][i].f))
#include "observation_fields.h"
#undef FIELD
        s+=']';}
    s+="],\"source_parameter_before\":"+ps0+",\"source_parameter_after\":"+param0.state()+",\"destination_parameter_before\":"+ps1+",\"destination_parameter_after\":"+param1.state()+"}";
    delete[] old;delete[] p;return s;
}
static std::string logs(){
    std::string s="{\"warnings\":"+std::to_string(TK_warnCount)+",\"errors\":"+std::to_string(TK_errorCount)+",\"quiet\":"+std::to_string(quietFlag)+",\"debug\":"+std::to_string(debugFlag)+",\"warning_storage\":[";
    for(int i=0;i<TK_MAX_ERRORS;i++){if(i)s+=',';s+=q(TK_warnlog[i]);}s+="],\"error_storage\":[";
    for(int i=0;i<TK_MAX_ERRORS;i++){if(i)s+=',';s+=q(TK_errorlog[i]);}return s+"]}";
}
static int callvs(char*b,const char*f,...){va_list a;va_start(a,f);int r=ld_vsprintf(b,f,a);va_end(a);return r;}
static std::string formatting(int entry,int which){
    char b[4096];memset(b,0x5a,sizeof b);FILE*f=tmpfile();assert(f);int r=-1;
#define CALL(...) (entry==0?ld_sprintf(b,__VA_ARGS__):entry==1?callvs(b,__VA_ARGS__):entry==2?ld_fprintf(f,__VA_ARGS__):ld_printf(__VA_ARGS__))
    if(which==0)r=CALL("literal %% end");
    if(which==1)r=CALL("i=%+06d u=%u x=%#x X=%X o=%o v=%i ll=%lld c=%c s=%8.3s!",-42,42u,42u,42u,42u,42,1234567890123LL,'Z',"abcdef");
    if(which==2)r=CALL("d=%+.8f e=%12.4e g=%.17g!",-0.0,1.25e-9,1.2345678901234567);
    if(which==3)r=CALL("q=%+.36Lg f=%10.4Lf e=%.8Le!",1.0000000000000000000000000000000002Q,-0.0Q,1.25e30Q);
    if(which==4)r=CALL("s=%s q=%.20Lg i=%d d=%.8g c=%c %%!","text",1.25Q,-9,2.5,'R');
#undef CALL
    std::string out;if(entry<2)out=std::string(b);else if(entry==2){fflush(f);rewind(f);int c;while((c=fgetc(f))!=EOF)out+=(char)c;}
    fclose(f);
    return "{\"return\":"+std::to_string(r)+",\"text\":"+q(out)+",\"buffer_bytes\":"+(entry<2?q(hex(b,sizeof b)):"null")+"}";
}
static std::string decode(const std::string&s){assert(s[0]=='x');std::string o;for(size_t i=1;i<s.size();i+=2)o+=(char)strtol(s.substr(i,2).c_str(),nullptr,16);return o;}
int main(int argc,char**argv){
    assert(argc==4);setvbuf(stdout,nullptr,_IONBF,0);setvbuf(stderr,nullptr,_IONBF,0);
    std::ofstream e(argv[2]);assert(e);e<<"{\"phase\":\"launch\",\"nonce\":"<<q(argv[3])<<"}"<<std::endl;
    std::ifstream input(argv[1]);std::string line;int action=0;
    while(std::getline(input,line)){
        std::vector<std::string>a;std::istringstream row(line);std::string t;while(std::getline(row,t,'\t'))a.push_back(decode(t));assert(!a.empty());
        e<<"{\"phase\":\"begin\",\"action\":"<<action<<",\"op\":"<<q(a[0])<<",\"stdout_position\":"<<ftell(stdout)<<"}"<<std::endl;
        std::string out="null";
        if(a[0]=="profile"){
            volatile longdouble one=1.0Q,epsilon=1.0Q;int measured=1;
            while(one+epsilon/2!=one){epsilon/=2;measured++;}
            assert(measured==113);
            out="{\"CHAR_BIT\":"+std::to_string(CHAR_BIT)+",\"measured_longdouble_mant\":"+std::to_string(measured)+",\"float_bytes\":"+std::to_string(sizeof(float))+",\"float_mant\":"+std::to_string(FLT_MANT_DIG)+",\"double_bytes\":"+std::to_string(sizeof(double))+",\"double_mant\":"+std::to_string(DBL_MANT_DIG)+",\"native_long_double_bytes\":"+std::to_string(sizeof(long double))+",\"native_long_double_mant\":"+std::to_string(LDBL_MANT_DIG)+",\"longdouble_bytes\":"+std::to_string(sizeof(longdouble))+",\"longdouble_mant\":"+std::to_string(FLT128_MANT_DIG)+",\"int_bytes\":"+std::to_string(sizeof(int))+",\"long_bytes\":"+std::to_string(sizeof(long))+",\"pulsar_bytes\":"+std::to_string(sizeof(pulsar))+",\"observation_bytes\":"+std::to_string(sizeof(observation))+",\"LD_PI\":"+num(LD_PI)+",\"LONGDOUBLE_ONE\":"+num(LONGDOUBLE_ONE)+",\"obliquity\":"+num(ECLIPTIC_OBLIQUITY)+",\"M_PI\":"+num(M_PI)+",\"SPEED_LIGHT\":"+num(SPEED_LIGHT)+",\"SECDAY\":"+num(SECDAY)+",\"PCM\":"+num(PCM)+",\"MASYR2RADS\":"+num(MASYR2RADS)+",\"FMT_LD\":"+q(FMT_LD)+"}";
        }else if(a[0]=="parse"){
            errno=0;longdouble v=parse_longdouble(a[1].c_str());int err=errno;
            out="{\"value\":"+num(v)+",\"errno\":"+std::to_string(err)+",\"display\":"+q(print_longdouble(v))+"}";
        }else if(a[0]=="format")out=formatting(stoi(a[1]),stoi(a[2]));
        else if(a[0]=="coord"){
            char b[128];memset(b,0x5a,sizeof b);assert(a[2].size()<39);strcpy(b,a[2].c_str());
            double v=a[1]=="hms"?hms_turn(b):dms_turn(b);
            out="{\"turn\":"+num(v)+",\"degrees\":"+num(turn_deg(v))+",\"input_bytes_after\":"+q(hex(b,sizeof b))+"}";
        }else if(a[0]=="turn"){
            double v=strtod(a[2].c_str(),nullptr);char b[128];memset(b,0x5a,sizeof b);
            double primary=(a[1]=="hms"?v*24:std::fabs(v)*360);
            assert(std::isfinite(primary)&&std::trunc(primary)>=INT_MIN&&std::trunc(primary)<=INT_MAX);
            int r=a[1]=="hms"?turn_hms(v,b):turn_dms(v,b);
            out="{\"input\":"+num(v)+",\"return\":"+std::to_string(r)+",\"text\":"+q(b)+",\"buffer_bytes\":"+q(hex(b,sizeof b))+"}";
        }else if(a[0]=="mod64"){double x=strtod(a[1].c_str(),nullptr),p=strtod(a[2].c_str(),nullptr);double quotient=x/p;assert(std::isfinite(quotient)&&std::trunc(quotient)>=INT_MIN&&std::trunc(quotient)<=INT_MAX);out=num(fortran_mod(x,p));}
        else if(a[0]=="mod128"){longdouble x=strtoflt128(a[1].c_str(),nullptr),p=strtoflt128(a[2].c_str(),nullptr),quotient=x/p;
            assert(!isnanq(quotient)&&!isinfq(quotient)&&truncq(quotient)>=INT_MIN&&truncq(quotient)<=INT_MAX);out=num(fortran_mod(x,p));}
        else if(a[0]=="nint"){double x=strtod(a[1].c_str(),nullptr),rounded=x>0?x+0.5:x-0.5;
            assert(std::isfinite(rounded)&&std::trunc(rounded)>=INT_MIN&&std::trunc(rounded)<=INT_MAX);out=std::to_string(fortran_nint(x));}
        else if(a[0]=="nlong"){longdouble x=strtoflt128(a[1].c_str(),nullptr),rounded=x>0?x+0.5:x-0.5;
            assert(!isnanq(rounded)&&!isinfq(rounded)&&truncq(rounded)>=static_cast<longdouble>(LONG_MIN)&&truncq(rounded)<=static_cast<longdouble>(LONG_MAX));out=std::to_string(fortran_nlong(x));}
        else if(a[0]=="vector"){
            double v[11]={0,0,0,0,0,0,0,0,9,10,11};for(int i=0;i<8;i++)v[i]=strtod(a[1+i].c_str(),nullptr);auto before=array(v,11);
            double dot=dotproduct(v,v+3);vectorsum(v+stoi(a[9]),v,v+3);auto sum=array(v,11);vectorscale(v+stoi(a[10]),strtod(a[11].c_str(),nullptr));
            out="{\"before\":"+before+",\"dot\":"+num(dot)+",\"after_sum\":"+sum+",\"after_scale\":"+array(v,11)+"}";
        }else if(a[0]=="rotation"){
            double v[3];for(int i=0;i<3;i++)v[i]=strtod(a[1+i].c_str(),nullptr);ECLIPTIC_OBLIQUITY=strtod(a[4].c_str(),nullptr);auto before=array(v,3);
            if(a[5]=="forward")equ2ecl(v);else ecl2equ(v);auto rotated=array(v,3);if(a[5]=="forward")ecl2equ(v);else equ2ecl(v);
            out="{\"obliquity\":"+num(ECLIPTIC_OBLIQUITY)+",\"before\":"+before+",\"rotated\":"+rotated+",\"roundtrip\":"+array(v,3)+"}";
        }else if(a[0]=="inline"){
            longdouble x=strtoflt128(a[1].c_str(),nullptr),y=strtoflt128(a[2].c_str(),nullptr);double exponent=strtod(a[3].c_str(),nullptr);
            out="{\"cos\":"+num(cos(x))+",\"sin\":"+num(sin(x))+",\"fabs\":"+num(fabs(x))+",\"sqrt\":"+num(sqrt(x))+",\"asin\":"+num(asin(x))+",\"atan2\":"+num(atan2(x,y))+",\"pow\":"+num(pow(x,exponent))+",\"exponent_binary64\":"+num(exponent)+",\"exp\":"+num(exp(x))+",\"log10\":"+num(log10(x))+",\"log\":"+num(log(x))+",\"tan\":"+num(tan(x))+",\"cosl\":"+num(cosl(x))+",\"sinl\":"+num(sinl(x))+",\"floorl\":"+num(floorl(x))+",\"fabsl\":"+num(fabsl(x))+",\"powl\":"+num(powl(x,y))+"}";
        }else if(a[0]=="copyparam")out=copyparam(stoi(a[1]));
        else if(a[0]=="copypsr")out=copypsr(stoi(a[1]),stoi(a[2]),stoi(a[3]));
        else if(a[0]=="link"){
            pulsar*p=new pulsar{};longdouble v[MAX_PARAMS][2]{};int ids[]={param_daop,param_pbdot,param_pb,param_pmra,param_pmdec,param_kin,param_stig,param_sini,param_dshk,param_px,param_dmmodel,param_dm,param_f};
            for(int id:ids){p->param[id].val=v[id];p->param[id].aSize=2;v[id][0]=3.0Q;v[id][1]=17.0Q;}
            v[param_pbdot][0]=1e-12Q;v[param_pb][0]=2;v[param_pmra][0]=10;v[param_pmdec][0]=20;v[param_stig][0]=0.5Q;v[param_kin][0]=30;v[param_px][0]=2;v[param_dm][0]=123.25Q;
            int target=ids[stoi(a[1])],from=ids[stoi(a[2])];p->param[target].nLinkTo=stoi(a[3]);p->param[target].linkTo[0]=from;p->param[target].linkTo[1]=from;
            if(a.size()>5)v[from][0]=strtoflt128(a[5].c_str(),nullptr);
            // A linked dependency remains a stored value: getParameterValue is nonrecursive.
            p->param[from].nLinkTo=1;p->param[from].linkTo[0]=from==param_kin?param_stig:param_px;
            parameter prior[MAX_PARAMS];memcpy(prior,p->param,sizeof prior);longdouble prior_values[MAX_PARAMS][2];memcpy(prior_values,v,sizeof v);
            longdouble first=getParameterValue(p,target,stoi(a[4])),second=getParameterValue(p,target,stoi(a[4]));
            out="{\"value\":"+num(first)+",\"repeat_value\":"+num(second)+",\"parameters_unchanged\":"+(memcmp(prior,p->param,sizeof prior)==0?"true":"false")+",\"values_unchanged\":"+(memcmp(prior_values,v,sizeof v)==0?"true":"false")+"}";delete p;
        }else if(a[0]=="controls"){quietFlag=stoi(a[1]);debugFlag=stoi(a[2]);out=logs();}
        else if(a[0]=="message"){displayMsg(stoi(a[1]),a[2].c_str(),a[3].c_str(),a[4].c_str(),stoi(a[5]));out=logs();}
        else if(a[0]=="checklogs"){int r=logerr_check();out="{\"return\":"+std::to_string(r)+",\"state\":"+logs()+"}";}
        else abort();
        e<<"{\"phase\":\"end\",\"action\":"<<action<<",\"op\":"<<q(a[0])<<",\"stdout_position\":"<<ftell(stdout)<<",\"result\":"<<out<<"}"<<std::endl;action++;
    }return 0;
}
