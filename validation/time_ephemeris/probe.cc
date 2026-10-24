// Assignment V: raw files included verbatim solely to observe IFTE static state.
#include "ifteph.C"
#include "tt2tdb.C"
#include <fstream>
#include <sstream>
#include <map>
#include <vector>
#include <string>
#include <cassert>
#include <climits>
#include <cfloat>
#include <cmath>
#include <limits>
#include <type_traits>
#include <unistd.h>
using State=std::map<std::string,std::string>;
static std::ofstream events;
static pulsar *ps;
static const int NP=2,NO=4;
static double output[16];
static char chars[64], filename[1024];
static IFTE_interpolation_info direct{};
static bool iflive=false,flive=false;
static std::string ifowner="null",fowner="null";
static int igen=0,fgen=0;
static std::vector<FILE*> old;
static std::vector<bool> oldlive;
static std::string root;
static std::string q(const std::string&s){std::ostringstream o;o<<'"';for(unsigned char c:s){if(c=='"'||c=='\\')o<<'\\'<<c;else if(c<32||c>126){char b[8];sprintf(b,"\\u%04x",c);o<<b;}else o<<c;}o<<'"';return o.str();}
static std::string hex(const void*p,size_t n){auto b=(const unsigned char*)p;std::string s;const char*h="0123456789abcdef";for(size_t i=0;i<n;i++){s+=h[b[i]>>4];s+=h[b[i]&15];}return s;}
static std::string bytes(const void*p,size_t n){return "{\"known_bytes\":"+std::to_string(n)+",\"hex\":"+q(hex(p,n))+"}";}
static std::string num(double v){char s[64];snprintf(s,sizeof(s),"%a",v);return "{\"type\":\"binary64\",\"class\":"+q(std::isnan(v)?"NaN":std::isinf(v)?(std::signbit(v)?"-Infinity":"+Infinity"):"finite")+",\"hex\":"+q(s)+",\"bits\":"+q(hex(&v,8))+"}";}
static std::string num(float v){return "{\"type\":\"binary32\",\"class\":"+q(std::isnan(v)?"NaN":std::isinf(v)?(std::signbit(v)?"-Infinity":"+Infinity"):"finite")+",\"bits\":"+q(hex(&v,4))+"}";}
static std::string num(longdouble v){char s[128];quadmath_snprintf(s,sizeof(s),"%.36Qg",v);return "{\"type\":\"binary128\",\"value\":"+q(s)+",\"bits\":"+q(hex(&v,16))+"}";}
template<class T>static typename std::enable_if<std::is_integral<T>::value,std::string>::type num(T v){return std::to_string((long long)v);}
static std::string object(const State&s){std::string o="{";bool first=true;for(auto&kv:s){if(!first)o+=",";first=false;o+=q(kv.first)+":"+kv.second;}return o+"}";}
template<class T>static std::string array(const T*p,int n){std::string s="[";for(int i=0;i<n;i++){if(i)s+=",";s+=num(p[i]);}return s+"]";}
static void cache(State&s,std::string k,const IFTE_interpolation_info&c){s[k+"pc"]=array(c.pc,18);s[k+"vc"]=array(c.vc,18);s[k+"twot"]=num(c.twot);s[k+"np"]=num(c.np);s[k+"nv"]=num(c.nv);}
static void stream(State&s,std::string k,FILE*f){s[k+"position"]=num(ftell(f));s[k+"eof"]=num(feof(f));s[k+"error"]=num(ferror(f));}
static State snapshot(){State s;
#define I(f) s["ifte." #f]=num(ifte.f)
I(startJD);I(endJD);I(stepJD);I(ephver);I(L_C);I(swap_endian);I(reclen);I(irec);
#undef I
s["ifte.title"]=bytes(ifte.title,256);s["ifte.buf"]=array(ifte.buf,322);s["ifte.ipt0"]=array(ifte.ipt[0],3);s["ifte.ipt1"]=array(ifte.ipt[1],3);cache(s,"ifte.",ifte.iinfo);cache(s,"direct.",direct);
s["ifte.owner"]=q(ifowner);s["ifte.generation"]=num(igen);if(iflive)stream(s,"ifte.",ifte.f);
s["fortran.owner"]=q(fowner);s["fortran.generation"]=num(fgen);s["fortran.swapByte"]=num(swapByte);if(flive)stream(s,"fortran.",c_fileptr);
for(size_t i=0;i<old.size();i++){std::string k="old"+std::to_string(i)+".";s[k+"owner"]=q(oldlive[i]?"live-saved":"released-by-caller");if(oldlive[i])stream(s,k,old[i]);}
s["output"]=array(output,16);s["chars"]=bytes(chars,sizeof(chars));s["filename"]=bytes(filename,sizeof(filename));s["root"]=q(root);
s["displayCVSversion"]=num(displayCVSversion);s["debugFlag"]=num(debugFlag);s["quietFlag"]=num(quietFlag);s["TK_errorCount"]=num(TK_errorCount);s["TK_warnCount"]=num(TK_warnCount);s["TK_errorlog"]=bytes(TK_errorlog,sizeof(TK_errorlog));s["TK_warnlog"]=bytes(TK_warnlog,sizeof(TK_warnlog));
for(int p=0;p<NP;p++){std::string k="p"+std::to_string(p)+".";
#define P(f) s[k+#f]=num(ps[p].f)
P(nobs);P(timeEphemeris);P(units);P(dilateFreq);P(useCalceph);
#undef P
for(int i=0;i<NO;i++){auto&o=ps[p].obsn[i];std::string a=k+"o"+std::to_string(i)+".";
#define O(f) s[a+#f]=num(o.f)
O(sat);O(clockCorr);O(nclock_correction);O(correctionTT_TB);O(correctionTT_Teph);O(correctionTT_calcEph);O(einsteinRate);
#undef O
s[a+"telID"]=bytes(o.telID,sizeof(o.telID));s[a+"earth_ssb"]=array(o.earth_ssb,6);s[a+"observatory_earth"]=array(o.observatory_earth,3);s[a+"siteVel"]=array(o.siteVel,3);
for(int j=0;j<MAX_CLK_CORR;j++){s[a+"clock"+std::to_string(j)+".correction"]=num(o.correctionsTT[j].correction);s[a+"clock"+std::to_string(j)+".corrects_to"]=bytes(o.correctionsTT[j].corrects_to,sizeof(o.correctionsTT[j].corrects_to));}
}}
return s;}
static void event(const std::string&phase,int step,const std::string&op,const std::string&result="null"){events<<"{\"phase\":"<<q(phase)<<",\"action\":"<<step<<",\"op\":"<<q(op)<<",\"result\":"<<result<<",\"state\":"<<object(snapshot())<<"}"<<std::endl;}
static int step=-1;static std::string op="initial";
static void exit_state(){event("exit",step,op);}
static std::string decode(const std::string&s){assert(s[0]=='x');std::string r;for(size_t i=1;i<s.size();i+=2)r+=(char)strtol(s.substr(i,2).c_str(),nullptr,16);return r;}
static void reset_direct(double seed){for(int i=0;i<18;i++){direct.pc[i]=seed+i;direct.vc[i]=seed+30+i;}direct.pc[0]=1;direct.pc[1]=0;direct.vc[0]=0;direct.vc[1]=1;direct.vc[2]=0;direct.np=2;direct.nv=3;direct.twot=0;}
int main(int argc,char**argv){assert(argc==4);events.open(argv[2]);assert(events);setvbuf(stdout,nullptr,_IONBF,0);setvbuf(stderr,nullptr,_IONBF,0);
ps=new pulsar[NP]();for(int p=0;p<NP;p++){ps[p].obsn=new observation[NO]();ps[p].nobs=3;ps[p].timeEphemeris=1;ps[p].units=1;ps[p].dilateFreq=1;for(int i=0;i<NO;i++){auto&o=ps[p].obsn[i];o.sat=50001.0+i*0.125;o.clockCorr=1;strcpy(o.telID,"PKS");o.nclock_correction=3;for(int j=0;j<MAX_CLK_CORR;j++){o.correctionsTT[j].correction=(j+1)*0.125;snprintf(o.correctionsTT[j].corrects_to,sizeof(o.correctionsTT[j].corrects_to),"C%d",j);}o.correctionTT_TB=110+p*20+i;o.correctionTT_Teph=210+p*20+i;o.correctionTT_calcEph=0.125+p*0.25+i*0.0625;o.einsteinRate=310+p*20+i;for(int j=0;j<6;j++)o.earth_ssb[j]=(j+1)*0.000125;for(int j=0;j<3;j++){o.observatory_earth[j]=(j+1)*0.0125;o.siteVel[j]=(j+1)*0.000003125;}}}
for(int i=0;i<16;i++)output[i]=1000+i;memset(chars,0x5a,sizeof(chars));memset(filename,0x57,sizeof(filename));reset_direct(100);
int measured=0;volatile longdouble one=1,e=1;while(one+e!=one){e/=2;measured++;}assert(measured==113);assert(sizeof(int)==4&&sizeof(float)==4&&sizeof(double)==8&&sizeof(longdouble)==16);assert(std::numeric_limits<double>::is_iec559&&std::numeric_limits<float>::is_iec559);assert(std::numeric_limits<char>::is_signed);int minus=-1;assert(hex(&minus,4)=="ffffffff");
events<<"{\"phase\":\"launch\",\"nonce\":"<<q(argv[3])<<",\"abi\":{\"binary128_significand\":"<<measured<<",\"int_bits\":"<<sizeof(int)*CHAR_BIT<<",\"char_signed\":"<<std::numeric_limits<char>::is_signed<<",\"native_ld_bits\":"<<sizeof(long double)*CHAR_BIT<<",\"native_ld_mant\":"<<LDBL_MANT_DIG<<",\"MAX_CLK_CORR\":"<<MAX_CLK_CORR<<"},\"constants\":{\"K\":"<<num(IFTE_K)<<",\"KM1\":"<<num(IFTE_KM1)<<",\"LC\":"<<num(IFTE_LC)<<",\"TEPH0\":"<<num(IFTE_TEPH0)<<",\"MJD0\":"<<num(IFTE_MJD0)<<",\"JD0\":"<<num(IFTE_JD0)<<"}}"<<std::endl;
std::atexit(exit_state);event("initial",step,op);std::ifstream in(argv[1]);assert(in);std::string line;
while(std::getline(in,line)){std::vector<std::string>a;std::istringstream row(line);std::string part;while(std::getline(row,part,'\t'))a.push_back(decode(part));assert(!a.empty());op=a[0];step++;event("begin",step,op);std::string result="null";
if(op=="root"){root=a[1];assert(root.size()+64<500);setenv(TEMPO2_ENVIRON,root.c_str(),1);}
else if(op=="init"){assert(!iflive);IFTE_init(a[1].c_str());iflive=true;ifowner="live";igen++;}
else if(op=="initroot"){assert(!iflive);init_ifte();iflive=true;ifowner="live";igen++;}
else if(op=="save"){assert(iflive);old.push_back(ifte.f);oldlive.push_back(true);iflive=false;ifowner="saved-for-replacement";}
else if(op=="cleanup"){int i=stoi(a[1]);assert(oldlive.at(i));fclose(old[i]);oldlive[i]=false;}
else if(op=="close"){assert(iflive||ifowner=="null");IFTE_close_file();if(iflive)ifowner="released";iflive=false;}
else if(op=="get"){assert(iflive);double d=stod(a[2]),f=stod(a[3]);std::string m=a[1];
if(m=="values")IFTE_get_Vals(d,f,stoi(a[4]),output);
else if(m=="dtboth")IFTE_get_DeltaT_DeltaTDot(d,f,output+stoi(a[4]),output+stoi(a[5]));
else if(m=="dt")output[0]=IFTE_DeltaT(d,f);
else if(m=="dot")output[0]=IFTE_DeltaTDot(d,f);
else if(m=="vboth")IFTE_get_vE_vEDot(d,f,output+stoi(a[4]),output+stoi(a[5]));
else if(m=="v")IFTE_get_vE(d,f,output);
else if(m=="vdot")IFTE_get_vEDot(d,f,output);
else abort();}
else if(op=="ifdelta"){assert(iflive);output[0]=IF_deltaT(parse_longdouble(a[1].c_str()));}
else if(op=="resetdirect")reset_direct(stod(a[1]));
else if(op=="interp"){int ncf=stoi(a[1]),ncm=stoi(a[2]),na=stoi(a[3]),ifl=stoi(a[4]);double t[2]={stod(a[5]),stod(a[6])};assert(ncf>=0&&ncf<=18&&ncm>=0&&ncm<=4&&na>=1&&na<=8);std::vector<double>coef(ncf*ncm*na);for(size_t i=0;i<coef.size();i++)coef[i]=a[7]=="simple"?(i%ncf==0?2:i%ncf==1?3:i%ncf==2?5:0):((int)i%13-6)*0.03125;double zero=0;IFTEinterp(&direct,coef.empty()?&zero:coef.data(),t,ncf,ncm,na,ifl,output);}
else if(op=="swap"){alignas(double) unsigned char b[32];for(int i=0;i<32;i++)b[i]=i+1;int n=stoi(a[2]);std::string m=a[1];int rounds=stoi(a[3]);for(int k=0;k<rounds;k++){if(m=="4")IFTswap4((char*)b);else if(m=="8")IFTswap8((char*)b);else if(m=="8N")IFTswap8N((char*)b,n);else if(m=="int"){int x;memcpy(&x,b,4);IFTswapInt(&x);memcpy(b,&x,4);}else if(m=="ints"){int x[8];memcpy(x,b,32);IFTswapInts(x,n);memcpy(b,x,32);}else if(m=="double"){double x;memcpy(&x,b,8);IFTswapDouble(&x);memcpy(b,&x,8);}else if(m=="doubles"){double x[4];memcpy(x,b,32);IFTswapDoubles(x,n);memcpy(b,x,32);}else abort();}result=bytes(b,32);}
else if(op=="open"){assert(!flive);assert(a[1].size()<sizeof(filename));strcpy(filename,a[1].c_str());int r=open_file(filename);flive=r==0;fowner=flive?"live":"null-failed-open";fgen++;result=num(r);}
else if(op=="fclose"){assert(flive);close_file();flive=false;fowner="released";}
else if(op=="swapmode"){swapByte=stoi(a[1]);}
else if(op=="read"){assert(flive);if(a[1]=="char")result=num((int)read_char());else if(a[1]=="chars"){int n=stoi(a[2]);assert(n<=64);read_character(n,chars);}else if(a[1]=="int")result=num(read_int());else if(a[1]=="float")result=num(read_float());else if(a[1]=="double")result=num(read_double());else if(a[1]=="record")result=num(read_record_int());else abort();}
else if(op=="fb"){assert(!flive);flive=true;fowner="live-inside-FB";fgen++;output[0]=FB_deltaT(parse_longdouble(a[1].c_str()));flive=false;fowner="released-by-FB";}
else if(op=="p"){auto&p=ps[stoi(a[1])];std::string k=a[2];int v=stoi(a[3]);
#define SET(f) if(k==#f)p.f=v;else
SET(nobs) SET(timeEphemeris) SET(units) SET(dilateFreq) SET(useCalceph)
abort();
#undef SET
}
else if(op=="obs"){auto&o=ps[stoi(a[1])].obsn[stoi(a[2])];std::string k=a[3];
#define SET(f) if(k==#f)o.f=parse_longdouble(a[4].c_str());else
SET(sat) SET(clockCorr) SET(nclock_correction) SET(correctionTT_calcEph) SET(correctionTT_TB) SET(correctionTT_Teph) SET(einsteinRate)
if(k=="telID"){assert(a[4].size()<sizeof(o.telID));strcpy(o.telID,a[4].c_str());}else abort();
#undef SET
}
else if(op=="clock"){auto&o=ps[stoi(a[1])].obsn[stoi(a[2])];o.correctionsTT[stoi(a[3])].correction=stod(a[4]);}
else if(op=="vector"){auto&o=ps[stoi(a[1])].obsn[stoi(a[2])];double*v=a[3]=="earth"?o.earth_ssb:a[3]=="site"?o.siteVel:o.observatory_earth;v[stoi(a[4])]=stod(a[5]);}
else if(op=="tt"){assert(!iflive);bool present=a[2]=="present";if(present){iflive=true;ifowner="live-inside-tt";igen++;}tt2tb(ps,stoi(a[1]));iflive=false;ifowner="released-by-tt";for(int p=0;p<stoi(a[1]);p++)for(int i=0;i<ps[p].nobs;i++)if(ps[p].timeEphemeris==FB90_TIMEEPH&&(ps[p].obsn[i].clockCorr!=0||strcmp(ps[p].obsn[i].telID,"STL")==0)){fowner="released-by-FB";fgen++;}}
else if(op=="global"){if(a[1]=="cvs")displayCVSversion=stoi(a[2]);else if(a[1]=="debug")debugFlag=stoi(a[2]);else abort();}
else if(op=="replace"||op=="truncate"){assert(iflive);if(op=="replace"){std::string temp=a[1]+".replacement";std::ifstream src(a[2],std::ios::binary);std::ofstream dst(temp,std::ios::binary);dst<<src.rdbuf();dst.close();assert(rename(temp.c_str(),a[1].c_str())==0);}else assert(truncate(a[1].c_str(),stol(a[2]))==0);}
else abort();event("end",step,op,result);
}return 0;}
