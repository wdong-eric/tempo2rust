// Assignment N caller, raw pinned routines are separate unchanged translation units.
#include "tempo2.h"
#include "tempo2Util.h"
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
extern void readTim(char*,pulsar*,int*,int*);
extern void removeCR2(char*);
extern int compareObs(const void*,const void*);
extern double date2mjd(int);
extern void swap4(char*);
extern void swap8(char*);
extern void swap8(double*);
extern void makechars(char*,char[25][9]);
extern void open_file2(char*,int*);
extern void close_file2();
extern void read_character2(int,char*);
extern int read_int2();
extern float read_float2();
extern double read_double2();
extern int read_record_int2();
extern int swapByte2;
extern FILE *c_fileptr2;
using State=std::map<std::string,std::string>;
static std::ofstream events;
static pulsar *ps=nullptr;
static const int N=12;
static int np=2,jump=7,fdjump=13;
static bool opened=false;
static longdouble f0[2]={2,2};
static std::string q(const std::string&s){std::ostringstream o;o<<'"';for(unsigned char c:s){if(c=='"'||c=='\\')o<<'\\'<<c;else if(c<32||c>126){char b[8];sprintf(b,"\\u%04x",c);o<<b;}else o<<c;}o<<'"';return o.str();}
static std::string hex(const void*p,size_t n){auto b=(const unsigned char*)p;std::string s;const char*h="0123456789abcdef";for(size_t i=0;i<n;i++){s+=h[b[i]>>4];s+=h[b[i]&15];}return s;}
// Lossless representation of initialized bytes, zero runs implicit. No pointers exported.
static std::string bytes(const void*p,size_t n){auto b=(const unsigned char*)p;std::string s="{\"size\":"+std::to_string(n)+",\"nonzero\":[";bool first=true;for(size_t i=0;i<n;){if(!b[i]){i++;continue;}size_t j=i+1;while(j<n&&b[j])j++;if(!first)s+=",";first=false;s+="["+std::to_string(i)+","+q(hex(b+i,j-i))+"]";i=j;}return s+"]}";}
static std::string num(longdouble v){char s[128];quadmath_snprintf(s,sizeof(s),"%.36Qg",v);return "{\"type\":\"binary128\",\"value\":"+q(s)+",\"bits_le\":"+q(hex(&v,sizeof(v)))+"}";}
static std::string num(double v){return "{\"type\":\"binary64\",\"class\":"+q(std::isnan(v)?"NaN":std::isinf(v)?(std::signbit(v)?"-Infinity":"+Infinity"):"finite")+",\"bits_le\":"+q(hex(&v,sizeof(v)))+"}";}
static std::string num(float v){return "{\"type\":\"binary32\",\"class\":"+q(std::isnan(v)?"NaN":std::isinf(v)?(std::signbit(v)?"-Infinity":"+Infinity"):"finite")+",\"bits_le\":"+q(hex(&v,sizeof(v)))+"}";}
template<class T>static typename std::enable_if<std::is_integral<T>::value,std::string>::type num(T v){return std::to_string((long long)v);}
static std::string object(const State&s){std::string o="{";bool first=true;for(auto &kv:s){if(!first)o+=",";first=false;o+=q(kv.first)+":"+kv.second;}return o+"}";}
static std::string delta(const State&a,const State&b){State d;for(auto&kv:b){auto old=a.find(kv.first);if(old==a.end()||old->second!=kv.second)d[kv.first]="{\"before\":"+(old==a.end()?"null":old->second)+",\"after\":"+kv.second+"}";}for(auto&kv:a)if(!b.count(kv.first))d[kv.first]="{\"before\":"+kv.second+",\"after\":null}";return object(d);}
static State snapshot(){State s;s["jump"]=num(jump);s["fdjump"]=num(fdjump);s["swapByte2"]=num(swapByte2);s["stream_open"]=opened?"true":"false";if(opened){s["position"]=num(ftell(c_fileptr2));s["eof"]=num(feof(c_fileptr2));s["ferror"]=num(ferror(c_fileptr2));}
#define G(f) s[#f]=num(f)
G(TK_errorCount);G(TK_warnCount);G(debugFlag);G(quietFlag);G(tcheck);G(displayCVSversion);G(MAX_OBSN);G(MAX_PSR);
#undef G
s["TK_warnlog"]=bytes(TK_warnlog,sizeof(TK_warnlog));s["TK_errorlog"]=bytes(TK_errorlog,sizeof(TK_errorlog));
for(int z=0;z<np;z++){auto&p=ps[z];std::string root="p"+std::to_string(z)+".";
#define F(f) s[root+#f]=num(p.f)
F(nobs);F(sorted);F(fixedFormat);F(jboFormat);F(noWarnings);F(fitMode);F(nPhaseJump);F(nT2efac);F(nT2equad);F(T2globalEfac);F(nToffset);
#undef F
#define B(f) s[root+#f]=bytes(&p.f,sizeof(p.f))
B(name);B(deleteFileName);B(phaseJump);B(phaseJumpID);B(phaseJumpDir);B(T2efacFlagID);B(T2efacFlagVal);B(T2efacVal);B(T2equadFlagID);B(T2equadFlagVal);B(T2equadVal);B(tOffset);B(tOffset_f1);B(tOffset_f2);B(tOffset_t1);B(tOffset_t2);B(tOffsetSite);B(tOffsetFlags);
#undef B
s[root+"F0"]=num(f0[z]);
for(int i=0;i<N;i++){auto&o=p.obsn[i];std::string k=root+"obs"+std::to_string(i)+".";s[k+"whole_record"]=bytes(&o,sizeof(o));
#define O(f) s[k+#f]=num(o.f)
O(sat);O(sat_day);O(sat_sec);O(bat);O(freq);O(toaErr);O(origErr);O(phaseOffset);O(toaDMErr);O(efac);O(equad);O(nFlags);O(obsNjump);O(obsNfdjump);O(deleted);O(clockCorr);O(delayCorr);O(pulseN);
#undef O
}}
return s;}
static State prior;
static void exit_state(){if(ps)events<<"{\"phase\":\"exit_state\",\"changes\":"<<delta(prior,snapshot())<<"}"<<std::endl;}
static std::string decode(const std::string&s){assert(s[0]=='x');std::string r;for(size_t i=1;i<s.size();i+=2)r+=(char)strtol(s.substr(i,2).c_str(),nullptr,16);return r;}
int main(int argc,char**argv){assert(argc==4);events.open(argv[2]);assert(events);setvbuf(stdout,nullptr,_IONBF,0);setvbuf(stderr,nullptr,_IONBF,0);events<<"{\"phase\":\"launch\",\"nonce\":"<<q(argv[3])<<"}"<<std::endl;
MAX_PSR=np;MAX_OBSN=N;ps=(pulsar*)calloc(np,sizeof(pulsar));assert(ps);
for(int z=0;z<np;z++){auto&p=ps[z];p.obsn=(observation*)calloc(N,sizeof(observation));assert(p.obsn);sprintf(p.name,"CALLER_%d",z);strcpy(p.deleteFileName,"NONE");p.T2globalEfac=1;p.param[param_f].aSize=1;p.param[param_f].val=&f0[z];for(int i=0;i<N;i++){auto&o=p.obsn[i];o.sat=51000+i;o.origsat=49000+i;o.sat_day=23;o.sat_sec=29;o.bat=48000+i;o.freq=700+i;o.toaErr=3;o.origErr=5;o.toaDMErr=11;o.phaseOffset=0.25;o.efac=1;o.equad=0;o.pulseN=1000*z+i;o.obsNjump=2;o.obsNfdjump=2;o.nFlags=1;for(int j=0;j<MAX_FLAGS;j++){o.jump[j]=100+j;o.fdjump[j]=200+j;o.jumpScale[j]=j+0.5;}strcpy(o.flagID[0],"-retained");strcpy(o.flagVal[0],"caller");sprintf(o.fname,"CALLER_%d_%d",z,i);strcpy(o.telID,"PKS");}}
int measured=0;volatile longdouble one=1,epsilon=1;while(one+epsilon!=one){epsilon/=2;measured++;}assert(measured==113);
std::atexit(exit_state);prior=snapshot();events<<"{\"phase\":\"initial\",\"state\":"<<object(prior)<<",\"abi\":{\"float_bits\":"<<sizeof(float)*CHAR_BIT<<",\"double_bits\":"<<sizeof(double)*CHAR_BIT<<",\"long_double_bits\":"<<sizeof(long double)*CHAR_BIT<<",\"native_ld_mant\":"<<LDBL_MANT_DIG<<",\"int_bits\":"<<sizeof(int)*CHAR_BIT<<",\"char_signed\":"<<std::numeric_limits<char>::is_signed<<",\"binary128_mant\":"<<FLT128_MANT_DIG<<",\"measured_binary128_significand\":"<<measured<<",\"sizeof_observation\":"<<sizeof(observation)<<",\"MAX_FLAGS\":"<<MAX_FLAGS<<",\"MAX_FLAG_LEN\":"<<MAX_FLAG_LEN<<",\"MAX_T2EFAC\":"<<MAX_T2EFAC<<",\"MAX_T2EQUAD\":"<<MAX_T2EQUAD<<",\"MAX_TOFFSET\":"<<MAX_TOFFSET<<",\"MAX_JUMPS\":"<<MAX_JUMPS<<",\"MAX_FILELEN\":"<<MAX_FILELEN<<"}}"<<std::endl;
std::ifstream in(argv[1]);std::string line;int step=0;
while(std::getline(in,line)){std::vector<std::string>a;std::istringstream row(line);std::string part;while(std::getline(row,part,'\t'))a.push_back(decode(part));assert(a.size());events<<"{\"phase\":\"begin\",\"action\":"<<step<<",\"op\":"<<q(a[0])<<"}"<<std::endl;std::string result="null";prior=snapshot();
if(a[0]=="read")readTim(const_cast<char*>(a[2].c_str()),ps+stoi(a[1]),&jump,&fdjump);
else if(a[0]=="full"){char files[2][MAX_FILELEN]={{0}};for(size_t i=1;i<a.size();i++)strcpy(files[i-1],a[i].c_str());readTimfile(ps,files,a.size()-1);}
else if(a[0]=="jbo")readJBO_bat(const_cast<char*>(a[2].c_str()),ps+stoi(a[1]),stoi(a[1]));
else if(a[0]=="sort")sortToAs(ps+stoi(a[1]));
else if(a[0]=="compare"){observation x{},y{};x.sat=parse_longdouble(a[1].c_str());y.sat=parse_longdouble(a[2].c_str());result=num(compareObs(&x,&y));}
else if(a[0]=="write")writeTim(a[2].c_str(),ps+stoi(a[1]),a[3].c_str());
else if(a[0]=="set"){auto&p=ps[stoi(a[1])];auto k=a[2];auto v=a[3];
#define SET(f) if(k==#f)p.f=stoi(v);else
SET(nobs) SET(sorted) SET(fixedFormat) SET(jboFormat) SET(noWarnings) SET(fitMode)
if(k=="deleteFileName")strcpy(p.deleteFileName,v.c_str());else if(k=="F0")f0[stoi(a[1])]=parse_longdouble(v.c_str());else if(k=="jump")jump=stoi(v);else if(k=="fdjump")fdjump=stoi(v);else if(k=="debug")debugFlag=stoi(v);else if(k=="quiet")quietFlag=stoi(v);else if(k=="tcheck")tcheck=stoi(v);else if(k=="cvs")displayCVSversion=stoi(v);else if(k=="maxobs")MAX_OBSN=stoi(v);else abort();
#undef SET
}else if(a[0]=="obs"){auto&o=ps[stoi(a[1])].obsn[stoi(a[2])];auto k=a[3];auto v=a[4];
#define SET(f) if(k==#f)o.f=parse_longdouble(v.c_str());else
SET(sat) SET(freq) SET(phaseOffset) SET(origErr) SET(toaErr) SET(efac) SET(equad) SET(deleted) SET(nFlags)
if(k=="fname")strcpy(o.fname,v.c_str());else if(k=="telID")strcpy(o.telID,v.c_str());else abort();
#undef SET
}else if(a[0]=="flag"){auto&o=ps[stoi(a[1])].obsn[stoi(a[2])];assert(o.nFlags<MAX_FLAGS);strcpy(o.flagID[o.nFlags],a[3].c_str());strcpy(o.flagVal[o.nFlags++],a[4].c_str());}
else if(a[0]=="offset"){auto&p=ps[stoi(a[1])];int i=p.nToffset++;p.tOffset[i]=stod(a[2]);p.tOffset_f1[i]=stod(a[3]);p.tOffset_f2[i]=stod(a[4]);p.tOffset_t1[i]=stod(a[5]);p.tOffset_t2[i]=stod(a[6]);strcpy(p.tOffsetSite[i],a[7].c_str());strcpy(p.tOffsetFlags[i],a[8].c_str());}
else if(a[0]=="date")result=num(date2mjd(stoi(a[1])));
else if(a[0]=="remove"){assert(a[1].size());char b[1024]={0};strcpy(b,a[1].c_str());removeCR2(b);result=bytes(b,a[1].size()+1);}
else if(a[0]=="swap"){alignas(double) char b[16]={0};memcpy(b,a[2].data(),a[2].size());if(a[1]=="4")swap4(b);else if(a[1]=="8")swap8(b);else {double d;memcpy(&d,b,8);swap8(&d);memcpy(b,&d,8);}result=bytes(b,16);}
else if(a[0]=="chars"){char b[200],c[25][9];assert(a[1].size()==200);memcpy(b,a[1].data(),200);makechars(b,c);result=bytes(c,sizeof(c));}
else if(a[0]=="open"){char b[1024]={0};strcpy(b,a[1].c_str());int swap=-1;open_file2(b,&swap);opened=true;result="{\"filename\":"+q(b)+",\"returned_swap\":"+num(swap)+"}";}
else if(a[0]=="close"){close_file2();opened=false;}
else if(a[0]=="swapmode")swapByte2=stoi(a[1]);
else if(a[0]=="primitive"){assert(opened);if(a[1]=="int")result=num(read_int2());else if(a[1]=="float")result=num(read_float2());else if(a[1]=="double")result=num(read_double2());else if(a[1]=="record")result=num(read_record_int2());else {char b[64]={0};int n=stoi(a[2]);assert(n<=64);read_character2(n,b);result=bytes(b,n);}}
else abort();
State after=snapshot();events<<"{\"phase\":\"end\",\"action\":"<<step<<",\"op\":"<<q(a[0])<<",\"result\":"<<result<<",\"changes\":"<<delta(prior,after)<<"}"<<std::endl;prior=after;step++;
}return 0;}
