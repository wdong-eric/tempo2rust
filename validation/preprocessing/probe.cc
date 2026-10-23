// Assignment P caller: unchanged raw primary/support translation units are linked.
#include "tempo2.h"
#include "tempo2Util.h"
#include "ifteph.h"
#include "names.h"
#include <fstream>
#include <sstream>
#include <map>
#include <vector>
#include <string>
#include <cstdint>
#include <cassert>
#include <type_traits>
#include <cfloat>
#include <cmath>
#include <limits>
#include <climits>
#include <cstring>
extern void scale_param(parameter*,int,longdouble);
extern void xform_mjd(parameter*,int,longdouble);
extern void readWhiteNoiseModelFile(pulsar*,int);
using State=std::map<std::string,std::string>;
static std::ofstream events;
static pulsar *ps=nullptr;
static const int N=12;
static int capacities[2]={N,N};
static int np=2;
static std::string q(const std::string&s){std::ostringstream o;o<<'"';for(unsigned char c:s){if(c=='"'||c=='\\')o<<'\\'<<c;else if(c<32||c>126){char b[8];sprintf(b,"\\u%04x",c);o<<b;}else o<<c;}o<<'"';return o.str();}
static std::string hex(const void*p,size_t n){auto b=(const unsigned char*)p;std::string s;const char*h="0123456789abcdef";for(size_t i=0;i<n;i++){s+=h[b[i]>>4];s+=h[b[i]&15];}return s;}
// Lossless representation of initialized bytes, zero runs implicit. No pointers exported.
static std::string bytes(const void*p,size_t n){auto b=(const unsigned char*)p;std::string s="{\"size\":"+std::to_string(n)+",\"nonzero\":[";bool first=true;for(size_t i=0;i<n;){if(!b[i]){i++;continue;}size_t j=i+1;while(j<n&&b[j])j++;if(!first)s+=",";first=false;s+="["+std::to_string(i)+","+q(hex(b+i,j-i))+"]";i=j;}return s+"]}";}
static std::string num(longdouble v){char s[128];quadmath_snprintf(s,sizeof(s),"%.36Qg",v);return "{\"type\":\"binary128\",\"value\":"+q(s)+",\"bits_le\":"+q(hex(&v,sizeof(v)))+"}";}
static std::string num(double v){return "{\"type\":\"binary64\",\"class\":"+q(std::isnan(v)?"NaN":std::isinf(v)?(std::signbit(v)?"-Infinity":"+Infinity"):"finite")+",\"bits_le\":"+q(hex(&v,sizeof(v)))+"}";}
static std::string num(float v){return "{\"type\":\"binary32\",\"class\":"+q(std::isnan(v)?"NaN":std::isinf(v)?(std::signbit(v)?"-Infinity":"+Infinity"):"finite")+",\"bits_le\":"+q(hex(&v,sizeof(v)))+"}";}
template<class T>static typename std::enable_if<(std::is_integral<T>::value||std::is_enum<T>::value),std::string>::type num(T v){return std::to_string((long long)v);}
static std::string object(const State&s){std::string o="{";bool first=true;for(auto &kv:s){if(!first)o+=",";first=false;o+=q(kv.first)+":"+kv.second;}return o+"}";}
static std::string delta(const State&a,const State&b){State d;for(auto&kv:b){auto old=a.find(kv.first);if(old==a.end()||old->second!=kv.second)d[kv.first]="{\"before\":"+(old==a.end()?"null":old->second)+",\"after\":"+kv.second+"}";}for(auto&kv:a)if(!b.count(kv.first))d[kv.first]="{\"before\":"+kv.second+",\"after\":null}";return object(d);}

static int index(const std::string&s){for(unsigned i=0;i<sizeof(names)/sizeof(*names);i++)if(s==names[i])return i;abort();}
template<class T>static void append(std::vector<unsigned char>&v,const T*p,size_t n){auto b=(const unsigned char*)p;v.insert(v.end(),b,b+sizeof(T)*n);}
static State snapshot(){State s;
#define G(f) s[#f]=num(f)
G(TK_errorCount);G(TK_warnCount);G(debugFlag);G(quietFlag);G(tcheck);G(displayCVSversion);G(MAX_OBSN);G(MAX_PSR);G(ECLIPTIC_OBLIQUITY);G(forceGlobalFit);G(tempo2_plug_path_len);
#undef G
s["TK_warnlog"]=bytes(TK_warnlog,sizeof(TK_warnlog));s["TK_errorlog"]=bytes(TK_errorlog,sizeof(TK_errorlog));
s["tempo2MachineType"]=bytes(tempo2MachineType,sizeof tempo2MachineType);s["tempo2_plug_path"]=bytes(tempo2_plug_path,sizeof tempo2_plug_path);
for(int z=0;z<np;z++){auto&p=ps[z];std::string root="p"+std::to_string(z)+".";
#define FIELD(f) s[root+#f]=bytes(&p.f,sizeof(p.f))
#include "projected_fields.h"
#undef FIELD
std::map<std::string,std::vector<unsigned char>> a;
for(int i=0;i<MAX_PARAMS;i++){auto&v=p.param[i];
#define P(f) append(a[#f],&v.f,1)
P(aSize);P(nLinkTo);P(nLinkFrom);
#undef P
append(a["linkTo"],v.linkTo,5);append(a["linkFrom"],v.linkFrom,5);
#define P(f) append(a[#f],v.f,v.aSize)
P(val);P(err);P(prefit);P(prefitErr);P(fitFlag);P(paramSet);
#undef P
for(int j=0;j<v.aSize;j++){append(a["label"],v.label[j],100);append(a["shortlabel"],v.shortlabel[j],100);}}
for(auto&kv:a)s[root+"param."+kv.first]=bytes(kv.second.data(),kv.second.size());
s[root+"caller_capacity"]=num(capacities[z]);
for(int i=0;i<capacities[z];i++){auto&o=p.obsn[i];std::string k=root+"obs"+std::to_string(i)+".";s[k+"whole_record"]=bytes(&o,sizeof(o));
#define O(f) s[k+#f]=num(o.f)
O(sat);O(origsat);O(bat);O(freq);O(toaErr);O(origErr);O(phaseOffset);O(nFlags);O(obsNjump);O(obsNfdjump);O(deleted);O(tobs);
#undef O
}}
return s;}
static std::string layout(){State s;
#define FIELD(f) s["observation." #f]="{\"offset\":"+std::to_string(offsetof(observation,f))+",\"bytes\":"+std::to_string(sizeof(((observation*)0)->f))+"}"
#include "observation_fields.h"
#undef FIELD
#define FIELD(f) s["pulsar." #f]="{\"offset\":"+std::to_string(offsetof(pulsar,f))+",\"bytes\":"+std::to_string(sizeof(((pulsar*)0)->f))+"}"
#include "projected_fields.h"
#undef FIELD
return object(s);}
static State prior;
static void exit_state(){if(ps)events<<"{\"phase\":\"exit_state\",\"changes\":"<<delta(prior,snapshot())<<"}"<<std::endl;}
static std::string decode(const std::string&s){assert(s[0]=='x');std::string r;for(size_t i=1;i<s.size();i+=2)r+=(char)strtol(s.substr(i,2).c_str(),nullptr,16);return r;}
int main(int argc,char**argv){assert(argc==4);events.open(argv[2]);assert(events);setvbuf(stdout,nullptr,_IONBF,0);setvbuf(stderr,nullptr,_IONBF,0);events<<"{\"phase\":\"launch\",\"nonce\":"<<q(argv[3])<<"}"<<std::endl;
MAX_PSR=np;MAX_OBSN=N;ps=(pulsar*)calloc(np,sizeof(pulsar));assert(ps);
for(int z=0;z<np;z++){auto&p=ps[z];initialiseOne(&p,0,0);memset(p.obsn,0,N*sizeof(observation));
for(int i=0;i<MAX_PARAMS;i++)for(int j=0;j<p.param[i].aSize;j++){p.param[i].prefit[j]=17;p.param[i].prefitErr[j]=19;}
sprintf(p.name,"CALLER_%d",z);p.nobs=4;p.param[param_f].val[0]=2+z;p.param[param_f].paramSet[0]=1;
p.param[param_pepoch].val[0]=58000;p.param[param_posepoch].val[0]=58000;p.param[param_dmepoch].val[0]=58000;
p.param[param_raj].val[0]=1;p.param[param_decj].val[0]=0.25Q;
for(int j=0;j<8;j++){p.jumpVal[j]=11+j;p.jumpValErr[j]=21+j;p.fitJump[j]=1;p.jumpSAT[j]=j%2;p.jumpScaled[j]=j%2;p.fdjumpVal[j]=31+j;p.fdjumpValErr[j]=41+j;p.fitfdJump[j]=1;p.fdjumpIdx[j]=j+2;p.phaseJumpID[j]=7;p.phaseJumpDir[j]=-1;}
for(int i=0;i<N;i++){auto&o=p.obsn[i];o.sat=58000+z*10+i;o.origsat=49000+i;o.bat=o.sat+0.25Q;o.freq=700+700*(i%3);o.toaErr=1+i;o.origErr=20+i;o.phaseOffset=0;o.pulseN=1000*z+i;o.tobs=777;
for(int j=0;j<MAX_FLAGS;j++){o.jump[j]=100+j;o.fdjump[j]=200+j;o.jumpScale[j]=j+0.5;}
for(int j=0;j<6;j++){o.earthMoonBary_ssb[j]=1+j;o.earthMoonBary_earth[j]=11+j;o.observatory_earth[j]=21+j;o.earth_ssb[j]=31+j;o.sun_ssb[j]=41+j;}
sprintf(o.fname,"CALLER_%d_%d.tim",z,i);strcpy(o.telID,i%2?"JBO":"PKS");o.nFlags=2;strcpy(o.flagID[0],"-be");strcpy(o.flagVal[0],i%2?"B":"A");strcpy(o.flagID[1],"-n");sprintf(o.flagVal[1],"%d",i);}}
int measured=0;volatile longdouble one=1,epsilon=1;while(one+epsilon!=one){epsilon/=2;measured++;}assert(measured==113);
std::atexit(exit_state);prior=snapshot();events<<"{\"phase\":\"initial\",\"state\":"<<object(prior)<<",\"abi\":{\"float_bits\":"<<sizeof(float)*CHAR_BIT<<",\"double_bits\":"<<sizeof(double)*CHAR_BIT<<",\"long_double_bits\":"<<sizeof(long double)*CHAR_BIT<<",\"native_ld_mant\":"<<LDBL_MANT_DIG<<",\"int_bits\":"<<sizeof(int)*CHAR_BIT<<",\"char_signed\":"<<std::numeric_limits<char>::is_signed<<",\"binary128_mant\":"<<FLT128_MANT_DIG<<",\"measured_binary128_significand\":"<<measured<<",\"sizeof_observation\":"<<sizeof(observation)<<",\"sizeof_pulsar\":"<<sizeof(pulsar)<<",\"MAX_FLAGS\":"<<MAX_FLAGS<<",\"MAX_PARAMS\":"<<MAX_PARAMS<<",\"IFTE_MJD0\":"<<num(IFTE_MJD0)<<",\"IFTE_KM1\":"<<num(IFTE_KM1)<<",\"IFTE_K\":"<<num(IFTE_K)<<",\"M_PI\":"<<num(M_PI)<<",\"START_FINISH_DELTA\":"<<num(START_FINISH_DELTA)<<",\"layout\":"<<layout()<<"}}"<<std::endl;
std::ifstream in(argv[1]);std::string line;int step=0;
while(std::getline(in,line)){std::vector<std::string>a;std::istringstream row(line);std::string part;while(std::getline(row,part,'\t'))a.push_back(decode(part));assert(a.size());events<<"{\"phase\":\"begin\",\"action\":"<<step<<",\"op\":"<<q(a[0])<<"}"<<std::endl;std::string result="null";prior=snapshot();
if(a[0]=="simple")preProcessSimple(ps+stoi(a[1]));
else if(a[0]=="stage1")preProcessSimple1(ps+stoi(a[1]),stoi(a[2]),stod(a[3]));
else if(a[0]=="stage2"){std::vector<float>d;for(size_t j=5;j<a.size();j++)d.push_back(stof(a[j]));int n=stoi(a[3]);assert(n<=(int)d.size());preProcessSimple2(ps+stoi(a[1]),stof(a[2]),n,d.empty()?nullptr:d.data(),stoi(a[4]));}
else if(a[0]=="dmnull")preProcessSimple2(ps,58000,3,nullptr,0);
else if(a[0]=="stage3")preProcessSimple3(ps+stoi(a[1]));
else if(a[0]=="wrapper"){std::vector<char*>v;v.push_back((char*)"preprocessing_probe");for(size_t i=2;i<a.size();i++)v.push_back(const_cast<char*>(a[i].c_str()));int n=stoi(a[1]);assert(n>=0&&n<=np);preProcess(ps,n,v.size(),v.data());}
else if(a[0]=="units")transform_units(ps+stoi(a[1]),stoi(a[2]),stoi(a[3]));
else if(a[0]=="scale"||a[0]=="mjd"){auto&p=ps[stoi(a[1])].param[index(a[2])];int j=stoi(a[3]);assert(j>=0&&j<p.aSize);if(a[0]=="scale")scale_param(&p,j,parse_longdouble(a[4].c_str()));else xform_mjd(&p,j,parse_longdouble(a[4].c_str()));}
else if(a[0]=="epoch")updateEpoch(ps,stoi(a[1]),parse_longdouble(a[2].c_str()));
else if(a[0]=="epochstr")updateEpoch_str(ps,stoi(a[1]),a[2].c_str());
else if(a[0]=="select")useSelectFile(const_cast<char*>(a[1].c_str()),ps,stoi(a[2]));
else if(a[0]=="logic")logicFlag(const_cast<char*>(a[1].c_str()),ps,stoi(a[2]));
else if(a[0]=="process")processFlag(const_cast<char*>(a[1].c_str()),ps,stoi(a[2]));
else if(a[0]=="simul")processSimultaneous(const_cast<char*>(a[1].c_str()),ps,stoi(a[2]));
else if(a[0]=="white")readWhiteNoiseModelFile(ps,stoi(a[1]));
else if(a[0]=="param"){auto&p=ps[stoi(a[1])].param[index(a[2])];int j=stoi(a[3]);assert(j>=0&&j<p.aSize);p.val[j]=parse_longdouble(a[4].c_str());p.prefit[j]=parse_longdouble(a[5].c_str());p.err[j]=parse_longdouble(a[6].c_str());p.prefitErr[j]=parse_longdouble(a[7].c_str());p.paramSet[j]=stoi(a[8]);p.fitFlag[j]=stoi(a[9]);}
else if(a[0]=="seed_units"){auto&p=ps[stoi(a[1])];for(int i=0;i<MAX_PARAMS;i++)for(int j=0;j<p.param[i].aSize;j++){auto&v=p.param[i];v.val[j]=50000+i+j*0.125Q;v.prefit[j]=51000+i+j*0.25Q;v.err[j]=0.5Q+j;v.prefitErr[j]=0.75Q+j;v.paramSet[j]=j%2;v.fitFlag[j]=j%3;}p.param[param_fddi].val[0]=1.5Q;p.TNChromIdx=4.25;}
else if(a[0]=="link"){auto&p=ps[stoi(a[1])];p.param[index(a[2])].nLinkTo=1;p.param[index(a[2])].linkTo[0]=index(a[3]);}
else if(a[0]=="set"){auto&p=ps[stoi(a[1])];auto k=a[2];auto v=a[3];
#define SET(f) if(k==#f)p.f=static_cast<decltype(p.f)>(parse_longdouble(v.c_str()));else
SET(nobs) SET(nWhite) SET(nWhite_dm) SET(units) SET(fitMode) SET(nJumps) SET(nfdJumps) SET(nPhaseJump) SET(dm_series_type) SET(T2globalEfac) SET(TNGlobalEF) SET(TNGlobalEQ) SET(TNChromIdx) SET(nTNSECORR)
#define STR(f) if(k==#f)strcpy(p.f,v.c_str());else
STR(filterStr) STR(passStr) STR(fjumpID) STR(tzrsite) STR(whiteNoiseModelFile) STR(clock) STR(name) STR(binaryModel)
if(k=="debug")debugFlag=stoi(v);else if(k=="quiet")quietFlag=stoi(v);else if(k=="cvs")displayCVSversion=stoi(v);else abort();
#undef SET
#undef STR
}else if(a[0]=="obs"){auto&o=ps[stoi(a[1])].obsn[stoi(a[2])];auto k=a[3];auto v=a[4];
#define SET(f) if(k==#f)o.f=parse_longdouble(v.c_str());else
SET(sat) SET(bat) SET(freq) SET(phaseOffset) SET(origErr) SET(toaErr) SET(deleted) SET(nFlags) SET(obsNjump) SET(obsNfdjump)
if(k=="fname")strcpy(o.fname,v.c_str());else if(k=="telID")strcpy(o.telID,v.c_str());else abort();
#undef SET
}else if(a[0]=="flag"){auto&o=ps[stoi(a[1])].obsn[stoi(a[2])];assert(o.nFlags<MAX_FLAGS);assert(a[3].size()<MAX_FLAG_LEN&&a[4].size()<MAX_FLAG_LEN);strcpy(o.flagID[o.nFlags],a[3].c_str());strcpy(o.flagVal[o.nFlags++],a[4].c_str());}
else if(a[0]=="offset"){auto&p=ps[stoi(a[1])];int i=p.nToffset++;assert(i<MAX_TOFFSET);p.tOffset[i]=stod(a[2]);p.tOffset_f1[i]=stod(a[3]);p.tOffset_f2[i]=stod(a[4]);p.tOffset_t1[i]=stod(a[5]);p.tOffset_t2[i]=stod(a[6]);strcpy(p.tOffsetSite[i],a[7].c_str());strcpy(p.tOffsetFlags[i],a[8].c_str());}
else if(a[0]=="jump"){auto&p=ps[stoi(a[1])];if(a[2]=="fd"){int j=++p.nfdJumps;assert(j<MAX_JUMPS);strcpy(p.fdjumpStr[j],a[3].c_str());}else{int j=++p.nJumps;assert(j<MAX_JUMPS);strcpy(p.jumpStr[j],a[3].c_str());}}
else if(a[0]=="phase"){auto&p=ps[stoi(a[1])];int j=p.nPhaseJump++;p.phaseJump[j]=parse_longdouble(a[2].c_str());p.phaseJumpID[j]=stoi(a[3]);}
else if(a[0]=="noise"){auto&p=ps[stoi(a[1])];auto k=a[2];
#define NOISE(n,f) if(k==#f){int j=p.n++;assert(j<100);strcpy(p.f##FlagID[j],a[3].c_str());strcpy(p.f##FlagVal[j],a[4].c_str());p.f##Val[j]=stod(a[5]);}else
NOISE(nT2efac,T2efac) NOISE(nT2equad,T2equad) NOISE(nTNEF,TNEF) NOISE(nTNEQ,TNEQ) NOISE(nTNSQ,TNSQ)
abort();
#undef NOISE
}else if(a[0]=="plugins"){tempo2_plug_path_len=a.size()-1;assert(tempo2_plug_path_len>0&&tempo2_plug_path_len<=32);strcpy(tempo2MachineType,"fixture");for(size_t j=1;j<a.size();j++)strcpy(tempo2_plug_path[j-1],a[j].c_str());}
else if(a[0]=="fjump_capacity"){auto&p=ps[0];free(p.obsn);capacities[0]=50;p.obsn=(observation*)calloc(50,sizeof(observation));assert(p.obsn);p.nobs=50;strcpy(p.fjumpID,"-f");for(int i=0;i<50;i++){auto&o=p.obsn[i];o.nFlags=40;for(int j=0;j<40;j++){strcpy(o.flagID[j],"-f");sprintf(o.flagVal[j],"v%d",40*i+j);}}}
else if(a[0]=="reverse"){auto&p=ps[stoi(a[1])];for(int i=0;i<p.nobs/2;i++)std::swap(p.obsn[i],p.obsn[p.nobs-i-1]);}
else abort();
State after=snapshot();events<<"{\"phase\":\"end\",\"action\":"<<step<<",\"op\":"<<q(a[0])<<",\"result\":"<<result<<",\"changes\":"<<delta(prior,after)<<"}"<<std::endl;prior=after;step++;
}return 0;}
