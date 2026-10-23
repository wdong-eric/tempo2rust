// Assignment Q: all native routines are separate, unchanged pinned translation units.
#include "tempo2.h"
#include "tempo2Util.h"
#include "t2fit_nestlike.h"
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
#include <unistd.h>
#include <sys/stat.h>
extern void checkLine(pulsar*,char*,FILE*,parameter*,parameter*);
extern int setupParameterFileDefaults(pulsar*);
using State=std::map<std::string,std::string>;
static std::ofstream events;
static pulsar *ps=nullptr;
static State prior;
static FILE *active=nullptr;
static std::map<char*,size_t> special_size;
static std::string q(const std::string&s){std::ostringstream o;o<<'"';for(unsigned char c:s){if(c=='"'||c=='\\')o<<'\\'<<c;else if(c<32||c>126){char b[8];sprintf(b,"\\u%04x",c);o<<b;}else o<<c;}o<<'"';return o.str();}
static std::string hex(const void*p,size_t n){auto b=(const unsigned char*)p;std::string s;const char*h="0123456789abcdef";for(size_t i=0;i<n;i++){s+=h[b[i]>>4];s+=h[b[i]&15];}return s;}
// Lossless representation of initialized bytes, zero runs implicit. No pointers exported.
static std::string bytes(const void*p,size_t n){auto b=(const unsigned char*)p;std::string s="{\"size\":"+std::to_string(n)+",\"nonzero\":[";bool first=true;for(size_t i=0;i<n;){if(!b[i]){i++;continue;}size_t j=i+1;while(j<n&&b[j])j++;if(!first)s+=",";first=false;s+="["+std::to_string(i)+","+q(hex(b+i,j-i))+"]";i=j;}return s+"]}";}
static std::string num(longdouble v){char s[128];quadmath_snprintf(s,sizeof(s),"%.36Qg",v);return "{\"type\":\"binary128\",\"value\":"+q(s)+",\"bits_le\":"+q(hex(&v,sizeof(v)))+"}";}
static std::string num(double v){return "{\"type\":\"binary64\",\"class\":"+q(std::isnan(v)?"NaN":std::isinf(v)?(std::signbit(v)?"-Infinity":"+Infinity"):"finite")+",\"bits_le\":"+q(hex(&v,sizeof(v)))+"}";}
static std::string num(float v){return "{\"type\":\"binary32\",\"class\":"+q(std::isnan(v)?"NaN":std::isinf(v)?(std::signbit(v)?"-Infinity":"+Infinity"):"finite")+",\"bits_le\":"+q(hex(&v,sizeof(v)))+"}";}
template<class T>static typename std::enable_if<std::is_integral<T>::value,std::string>::type num(T v){return std::to_string(v);}
static std::string object(const State&s){std::string o="{";bool first=true;for(auto &kv:s){if(!first)o+=",";first=false;o+=q(kv.first)+":"+kv.second;}return o+"}";}
static std::string delta(const State&a,const State&b){State d;for(auto&kv:b){auto old=a.find(kv.first);if(old==a.end()||old->second!=kv.second)d[kv.first]="{\"before\":"+(old==a.end()?"null":old->second)+",\"after\":"+kv.second+"}";}for(auto&kv:a)if(!b.count(kv.first))d[kv.first]="{\"before\":"+kv.second+",\"after\":null}";return object(d);}
// Every exported fixed field is explicitly initialized before canonical initialization.
// This is caller state, not a replacement default or a native initialization fix.
template<class T> static typename std::enable_if<!std::is_enum<T>::value>::type seed(T&v,int n){v=(T)(100+n%700);}
template<class T> static typename std::enable_if<std::is_enum<T>::value>::type seed(T&v,int){v=(T)0;}
template<size_t N> static void seed(char (&v)[N],int n){memset(v,0,N);if(N>30)snprintf(v,N,"caller_%d_retained_suffix",n);}
template<class T,size_t N> static void seed(T (&v)[N],int n){for(size_t i=0;i<N;i++)seed(v[i],n+(int)i);}
static std::string remaining(FILE*f){long p=ftell(f);struct stat st;assert(fstat(fileno(f),&st)==0 && p>=0 && p<=st.st_size);std::string r(st.st_size-p,'\0');if(!r.empty())assert(pread(fileno(f),&r[0],r.size(),p)==(ssize_t)r.size());return r;}
template<class T> static void project(State&s,const std::string&k,const T&v){s[k]=bytes(&v,sizeof(v));}
template<size_t N,size_t M> static void project(State&s,const std::string&k,const char (&v)[N][M]){for(size_t i=0;i<N;i++)s[k+"["+std::to_string(i)+"]"]=bytes(v[i],M);}
static State snapshot(){State s;
s["TK_errorCount"]=num(TK_errorCount);s["TK_warnCount"]=num(TK_warnCount);s["TK_errorlog"]=bytes(TK_errorlog,sizeof(TK_errorlog));s["TK_warnlog"]=bytes(TK_warnlog,sizeof(TK_warnlog));
for(int z=0;z<2;z++){auto&p=ps[z];std::string root="p"+std::to_string(z)+".";
#define FIELD(f) project(s,root+#f,p.f)
#include "fields.h"
#undef FIELD
for(int i=0;i<MAX_PARAMS;i++){auto&v=p.param[i];std::string k=root+"param["+std::to_string(i)+"].";
s[k+"aSize"]=num(v.aSize);
#define F(f) s[k+#f]=bytes(&v.f,sizeof(v.f))
F(nLinkTo);F(nLinkFrom);F(linkTo);F(linkFrom);
#undef F
#define F(f) s[k+#f]=bytes(v.f,v.aSize*sizeof(*v.f))
F(val);F(prefit);F(err);F(prefitErr);F(fitFlag);F(paramSet);
#undef F
std::string labels,shorts;for(int j=0;j<v.aSize;j++){labels.append(v.label[j],100);shorts.append(v.shortlabel[j],100);}s[k+"labels"]=bytes(labels.data(),labels.size());s[k+"shortlabels"]=bytes(shorts.data(),shorts.size());
}
for(int j=0;j<MAX_PARAMS;j++)if(p.constraint_special[j]){auto it=special_size.find(p.constraint_special[j]);assert(it!=special_size.end());size_t n=it->second;std::string k=root+"constraint_special["+std::to_string(j)+"]";s[k]="{\"allocated\":"+std::to_string(n+1)+",\"initialized_prefix\":"+bytes(p.constraint_special[j],n)+",\"unwritten_tail\":1}";}
}
if(active){s["stream.eof"]=num(feof(active));s["stream.error"]=num(ferror(active));s["stream.position"]=num(ftell(active));s["stream.remaining"]=q(remaining(active));}
return s;}
static void exit_state(){if(!ps)return;events<<"{\"phase\":\"exit_state\",\"changes\":"<<delta(prior,snapshot())<<"}"<<std::endl;for(int z=0;z<2;z++)destroyOne(ps+z);free(ps);ps=nullptr;}
static std::string decode(const std::string&s){assert(s[0]=='x');std::string r;for(size_t i=1;i<s.size();i+=2)r+=(char)strtol(s.substr(i,2).c_str(),nullptr,16);return r;}
int main(int argc,char**argv){assert(argc==4);events.open(argv[2]);assert(events);setvbuf(stdout,nullptr,_IONBF,0);setvbuf(stderr,nullptr,_IONBF,0);events<<"{\"phase\":\"launch\",\"nonce\":"<<q(argv[3])<<"}"<<std::endl;
MAX_PSR=2;MAX_OBSN=2;ps=(pulsar*)malloc(2*sizeof(pulsar));assert(ps);
for(int z=0;z<2;z++){auto&p=ps[z];int field=10+z*1000;
#define FIELD(f) seed(p.f,field++)
#include "fields.h"
#undef FIELD
p.ne_sw_ifuncN=0;p.nWhite_dm=0;
initialiseOne(&p,0,0);
for(int i=0;i<MAX_PARAMS;i++){auto&v=p.param[i];for(int j=0;j<v.aSize;j++){v.prefit[j]=17+z;v.prefitErr[j]=19+z;}for(size_t j=0;j<sizeof(v.linkTo)/sizeof(*v.linkTo);j++)v.linkTo[j]=param_f;for(size_t j=0;j<sizeof(v.linkFrom)/sizeof(*v.linkFrom);j++)v.linkFrom[j]=param_dm;}
for(int i=0;i<MAX_JUMPS;i++){p.jumpValErr[i]=7+i+z;p.fdjumpValErr[i]=9+i+z;p.jumpScaled[i]=i%3;}
strcpy(p.clock,"TT(TAI)");strcpy(p.ephemeris,"CALLER_EPHEM");strcpy(p.JPL_EPHEMERIS,"CALLER_PATH");
}
int measured=0;volatile longdouble one=1,epsilon=1;while(one+epsilon!=one){epsilon/=2;measured++;}assert(measured==113);
std::atexit(exit_state);prior=snapshot();events<<"{\"phase\":\"initial\",\"state\":"<<object(prior)<<",\"abi\":{\"binary128_significand\":"<<measured<<",\"pulsar_bytes\":"<<sizeof(pulsar)<<",\"MAX_PARAMS\":"<<MAX_PARAMS<<",\"MAX_IFUNC\":"<<MAX_IFUNC<<",\"MAX_WHITE\":"<<MAX_WHITE<<",\"MAX_JUMPS\":"<<MAX_JUMPS<<",\"MAX_STRLEN\":"<<MAX_STRLEN<<",\"endian\":\"little\"}}"<<std::endl;
// Direct scratch has defined metadata. Reader-owned scratch is left untouched.
parameter e[2]{};longdouble ev[2]={0,0},ee[2]={0,0},ep[2]={17,17},epe[2]={19,19};int ef[2]={0,0},es[2]={0,0};char el[2][100]={{0},{0}},esh[2][100]={{0},{0}};char*elp[2]={el[0],el[1]},*esp[2]={esh[0],esh[1]};
for(int i=0;i<2;i++){e[i].aSize=1;e[i].val=ev+i;e[i].err=ee+i;e[i].prefit=ep+i;e[i].prefitErr=epe+i;e[i].fitFlag=ef+i;e[i].paramSet=es+i;e[i].label=elp+i;e[i].shortlabel=esp+i;}
std::ifstream in(argv[1]);std::string line;int step=0;
while(std::getline(in,line)){std::vector<std::string>a;std::istringstream row(line);std::string part;while(std::getline(row,part,'\t'))a.push_back(decode(part));assert(a.size());events<<"{\"phase\":\"begin\",\"action\":"<<step<<",\"op\":"<<q(a[0])<<"}"<<std::endl;std::string result="null";prior=snapshot();
if(a[0]=="dispatch"){
int z=stoi(a[1]);active=tmpfile();assert(active);fwrite(a[2].data(),1,a[2].size(),active);rewind(active);char key[1000];int token=0;
while(fscanf(active,"%999s",key)==1){auto&p=ps[z];auto before=snapshot();std::string rest=remaining(active);int nc=p.nconstraints;size_t extent=0;
if(strcasecmp(key,"CONSTRAIN")==0){std::istringstream r(rest);std::string cn;r>>cn;if(cn=="PARAM"||cn=="NE_SW_IFUNC_SIN"||cn=="NE_SW_IFUNC_SIGMA"||cn=="IFUNC_COV"){size_t start=rest.find(cn)+cn.size();size_t end=rest.find('\n',start);extent=std::min(size_t(1023),end==std::string::npos?rest.size()-start:end-start+1)+1;}}
events<<"{\"phase\":\"dispatch_begin\",\"action\":"<<step<<",\"token\":"<<token<<",\"keyword\":"<<q(key)<<",\"position\":"<<ftell(active)<<",\"remaining\":"<<q(rest)<<"}"<<std::endl;
checkLine(ps+z,key,active,e,e+1);
if(extent){assert(p.nconstraints==nc+1);special_size[p.constraint_special[nc]]=extent;}
events<<"{\"phase\":\"dispatch_end\",\"action\":"<<step<<",\"token\":"<<token++<<",\"changes\":"<<delta(before,snapshot())<<"}"<<std::endl;
}result="{\"position\":"+num(ftell(active))+",\"remaining\":"+q(remaining(active))+"}";fclose(active);active=nullptr;
}else if(a[0]=="set"){auto&p=ps[stoi(a[1])];auto k=a[2];auto v=a[3];
#define S(f) if(k==#f)p.f=stoi(v);else
S(nJumps) S(nfdJumps) S(nToffset) S(dmoffsDMnum) S(dmoffsCMnum) S(ne_sw_ifuncN) S(nWhite) S(nWhite_dm) S(nconstraints) S(nT2efac) S(nT2equad) S(nTNEF) S(nTNEQ) S(nTNSQ) S(nTNECORR) S(nTNSECORR) S(nTNBandNoise) S(nTNGroupNoise) S(nDMEvents) S(nTNShapeletEvents) S(nPhaseJump)
if(k=="jumpSAT1")p.jumpSAT[1]=stoi(v);else if(k=="TNsubtractPoly")p.TNsubtractPoly=strtoull(v.c_str(),nullptr,0);else if(k=="gwecc_paramSet"){p.param[param_gwecc].paramSet[0]=stoi(v);strcpy(p.param[param_gwecc].label[0],"CALLER_GWECC");strcpy(p.param[param_gwecc].shortlabel[0],"CALLER_GWECC");}else abort();
#undef S
}else if(a[0]=="simple"){active=fopen(a[2].c_str(),"r");assert(active);int rc=readSimpleParfile(active,ps+stoi(a[1]));result="{\"status\":"+num(rc)+",\"position\":"+num(ftell(active))+",\"remaining\":"+q(remaining(active))+"}";fclose(active);active=nullptr;
}else if(a[0]=="full"||a[0]=="global"){char par[2][MAX_FILELEN]={{0}},tim[2][MAX_FILELEN]={{0}};int n=stoi(a[1]);assert(n==1||n==2);for(int i=0;i<n;i++)strcpy(par[i],a[2+i].c_str());if(a[0]=="full")readParfile(ps,par,tim,n);else readParfileGlobal(ps,n,par,tim);
}else if(a[0]=="helpers"){uint64_t v=strtoull(a[1].c_str(),nullptr,0);int r=stoi(a[2]),d=stoi(a[3]),c=stoi(a[4]);State s;s["flags"]=num(t2DecodeTNsubtractPolyFlags(v));s["red"]=num(t2DecodeTNRedSubPolyOrd(v));s["dm"]=num(t2DecodeTNDMSubPolyOrd(v));s["chrom"]=num(t2DecodeTNChromSubPolyOrd(v));s["encode"]=num(t2EncodeTNsubtractPoly(v,r,d,c));s["set_red"]=num(t2SetTNRedSubPolyOrd(v,r));s["set_dm"]=num(t2SetTNDMSubPolyOrd(v,d));s["set_chrom"]=num(t2SetTNChromSubPolyOrd(v,c));result=object(s);
}else if(a[0]=="route"){observation o{};o.sat=parse_longdouble(a[1].c_str());getClockCorrections(&o,a[2].c_str(),a[3].c_str(),0);State s;s["count"]=num(o.nclock_correction);for(int i=0;i<o.nclock_correction;i++){s[std::to_string(i)+".correction"]=num(o.correctionsTT[i].correction);s[std::to_string(i)+".to"]=q(o.correctionsTT[i].corrects_to);}result=object(s);
}else abort();
State after=snapshot();events<<"{\"phase\":\"end\",\"action\":"<<step<<",\"op\":"<<q(a[0])<<",\"result\":"<<result<<",\"changes\":"<<delta(prior,after)<<"}"<<std::endl;prior=after;step++;
}return 0;}
