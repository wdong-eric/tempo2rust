// Z's caller/observer only: the ordinary primary and utility are separate TUs.
#include <cmath>
#include <cstdint>
#include <cstring>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <limits>
#include <vector>
#include <map>
#include "tempo2.h"
extern "C" {
#include "jpleph.h"
}
extern double random2(long*);
extern double gasdev(long*);
template<class T> struct Equal { static bool check(const T&a,const T&b){return memcmp(&a,&b,sizeof(T))==0;} };
template<class T> bool same(const T&a,const T&b){return Equal<T>::check(a,b);}
template<class T,size_t N> struct Equal<T[N]> {static bool check(const T(&a)[N],const T(&b)[N]){for(size_t i=0;i<N;i++)if(!same(a[i],b[i]))return false;return true;}};
#include "field_checks.h"
static std::string quote(const std::string&s){
  std::ostringstream o;o<<'"';for(unsigned char c:s){if(c=='"'||c=='\\')o<<'\\'<<c;
  else if(c<32||c>=127)o<<"\\u"<<std::hex<<std::setw(4)<<std::setfill('0')<<unsigned(c);else o<<c;}o<<'"';return o.str();
}
static std::string bytes(const void*p,size_t n){std::ostringstream o;auto*b=static_cast<const unsigned char*>(p);for(size_t i=0;i<n;i++)o<<std::hex<<std::setw(2)<<std::setfill('0')<<unsigned(b[i]);return o.str();}
static std::string number(double x){
 if(std::isnan(x))return "{\"class\":\"NaN\"}";if(std::isinf(x))return x>0?"{\"class\":\"+Infinity\"}":"{\"class\":\"-Infinity\"}";
 uint64_t u;memcpy(&u,&x,8);std::ostringstream o;o<<"{\"class\":\"finite\",\"bits\":\""<<std::hex<<std::setw(16)<<std::setfill('0')<<u<<"\"}";return o.str();
}
static std::string quad(longdouble x){if(isnanq(x))return "{\"class\":\"NaN\"}";if(isinfq(x))return x>0?"{\"class\":\"+Infinity\"}":"{\"class\":\"-Infinity\"}";return "{\"class\":\"finite\",\"little_endian_bytes\":"+quote(bytes(&x,sizeof x))+"}";}
template<size_t N> static std::string vector(const double(&v)[N]){std::ostringstream o;o<<'[';for(size_t i=0;i<N;i++){if(i)o<<',';o<<number(v[i]);}o<<']';return o.str();}
static std::string vector6(const double*v){std::ostringstream o;o<<'[';for(int i=0;i<6;i++){if(i)o<<',';o<<number(v[i]);}o<<']';return o.str();}
static std::string chars(const char*p,size_t n){return "{\"initialized_bytes\":"+std::to_string(n)+",\"hex\":"+quote(bytes(p,n))+"}";}
static const int params[]={param_dphaseplanet,param_dmassplanet,param_tel_dx,param_tel_dy,param_tel_dz};
static pulsar*p;static observation*obs;static long seeds[2]={0,0};
static std::string caller(){
 std::ostringstream o;o<<"{\"pulsars\":[";
 for(int j=0;j<2;j++){
  if(j)o<<',';auto&v=p[j];o<<"{\"owner\":"<<j<<",\"observation_owner\":"<<j*6<<",\"nobs\":"<<v.nobs<<",\"units\":"<<v.units<<",\"eclCoord\":"<<v.eclCoord<<",\"ephemeris\":"<<quote(v.JPL_EPHEMERIS)<<",\"ephemeris_tail\":"<<chars(v.JPL_EPHEMERIS+strlen(v.JPL_EPHEMERIS)+1,sizeof v.JPL_EPHEMERIS-strlen(v.JPL_EPHEMERIS)-1)<<",\"parameters\":[";
  for(int k=0;k<5;k++){if(k)o<<',';auto&pr=v.param[params[k]];o<<"{\"index\":"<<params[k]<<",\"owner\":"<<j*5+k<<",\"aSize\":"<<pr.aSize<<",\"values\":[";for(int t=0;t<9;t++){if(t)o<<',';o<<quad(pr.val[t]);}o<<"],\"set\":[";for(int t=0;t<9;t++){if(t)o<<',';o<<pr.paramSet[t];}o<<"]}";}
  o<<"],\"offsets\":[";int counts[]={v.nTelDX,v.nTelDY,v.nTelDZ};double*ts[]={v.telDX_t,v.telDY_t,v.telDZ_t};double*vs[]={v.telDX_v,v.telDY_v,v.telDZ_v};
  for(int ax=0;ax<3;ax++){if(ax)o<<',';o<<"{\"count\":"<<counts[ax]<<",\"backing\":[";for(int t=0;t<500;t++){if(t)o<<',';o<<'['<<number(ts[ax][t])<<','<<number(vs[ax][t])<<']';}o<<"]}";}o<<"]}";
 }
 o<<"],\"observations\":[";
 for(int j=0;j<12;j++){
  if(j)o<<',';auto&v=obs[j];o<<"{\"owner\":"<<j<<",\"sat\":"<<quad(v.sat)<<",\"teph\":"<<quad(v.correctionTT_Teph)<<",\"telID\":"<<chars(v.telID,sizeof v.telID)<<",\"nclock\":"<<v.nclock_correction<<",\"clock_steps\":[";
  for(int k=0;k<MAX_CLK_CORR;k++){if(k)o<<',';o<<'['<<number(v.correctionsTT[k].correction)<<','<<chars(v.correctionsTT[k].corrects_to,32)<<']';}o<<"],\"nFlags\":"<<v.nFlags<<",\"flags\":[";
  for(int k=0;k<MAX_FLAGS;k++){if(k)o<<',';o<<'['<<chars(v.flagID[k],MAX_FLAG_LEN)<<','<<chars(v.flagVal[k],MAX_FLAG_LEN)<<']';}o<<"],\"vectors\":{";
#define V(NAME) o<<"\"" #NAME "\":"<<vector(v.NAME)
  V(sun_ssb);o<<',';V(sun_earth);o<<',';V(jupiter_earth);o<<',';V(saturn_earth);o<<',';V(venus_earth);o<<',';V(uranus_earth);o<<',';V(neptune_earth);o<<',';V(earth_ssb);o<<',';V(earthMoonBary_ssb);o<<',';V(earthMoonBary_earth);o<<',';V(nutations);
#undef V
  o<<"},\"planets\":[";double(*arrays[3])[6]={v.planet_ssb,v.planet_ssb_tmr,v.planet_ssb_derv};for(int k=0;k<3;k++){if(k)o<<',';o<<'[';for(int z=0;z<9;z++){if(z)o<<',';o<<vector6(arrays[k][z]);}o<<']';}o<<"]}";
 }o<<"]}";return o.str();
}
static const longdouble *rr=nullptr,*rrResult=nullptr;static const int *iff=nullptr,*ix1=nullptr,*ix2=nullptr,*ix3=nullptr,*iset=nullptr;static const double*gset=nullptr;
static unsigned randomEntries=0;static std::vector<std::string>attempts;
void z_gaussian_attempt(double v1,double v2,double rsq){attempts.push_back("["+number(v1)+","+number(v2)+","+number(rsq)+"]");}
void z_random_observer(const longdouble*r,const longdouble*result,const int*f,const int*a,const int*b,const int*c){randomEntries++;rr=r;rrResult=result;iff=f;ix1=a;ix2=b;ix3=c;}
void z_gas_observer(const int*i,const double*g){iset=i;gset=g;}
static std::string privateState(){
 std::ostringstream o;o<<"{\"random\":";
 if(!rr)o<<"null";else{o<<"{\"iff\":"<<*iff<<",\"ix\":["<<*ix1<<','<<*ix2<<','<<*ix3<<"],\"result\":"<<quad(*rrResult)<<",\"r\":[";for(int k=0;k<100;k++){if(k)o<<',';o<<quad(rr[k]);}o<<"]}";}
 o<<",\"gaussian\":";if(!iset)o<<"null";else o<<"{\"iset\":"<<*iset<<",\"gset\":"<<number(*gset)<<'}';o<<",\"random_entries\":"<<randomEntries<<",\"attempts\":[";for(size_t i=0;i<attempts.size();i++){if(i)o<<',';o<<attempts[i];}o<<"]}";return o.str();
}
static std::vector<std::string>calls;static std::map<void*,int>handles;static int nextHandle=0;
void*z_init(const char*file,char names[][6],double*vals){
 void*h=jpl_init_ephemeris(file,names,vals);int id=h?nextHandle++:-1;if(h)handles[h]=id;
 calls.push_back("{\"op\":\"open\",\"path\":"+quote(file)+",\"handle\":"+std::to_string(id)+",\"status\":"+std::to_string(jpl_init_error_code())+"}");return h;
}
int z_pleph(void*h,double jd,int target,int centre,double*rrd,int velocity){
 int status=jpl_pleph(h,jd,target,centre,rrd,velocity);
 calls.push_back("{\"op\":\"pleph\",\"handle\":"+std::to_string(handles.at(h))+",\"jd\":"+number(jd)+",\"target\":"+std::to_string(target)+",\"centre\":"+std::to_string(centre)+",\"velocity\":"+std::to_string(velocity)+",\"status\":"+std::to_string(status)+",\"output\":"+vector6(rrd)+"}");return status;
}
void z_close(void*h){int id=handles.at(h);jpl_close_ephemeris(h);handles.erase(h);calls.push_back("{\"op\":\"close\",\"handle\":"+std::to_string(id)+"}");}
static std::string callState(){std::ostringstream o;o<<'[';for(size_t i=0;i<calls.size();i++){if(i)o<<',';o<<calls[i];}o<<']';return o.str();}
static std::string globals(){return "{\"debug\":"+std::to_string(debugFlag)+",\"cvs\":"+std::to_string(displayCVSversion)+",\"warn_count\":"+std::to_string(TK_warnCount)+",\"error_count\":"+std::to_string(TK_errorCount)+",\"jpl_init_status\":"+std::to_string(jpl_init_error_code())+"}";}
static pulsar* beforeDriver=nullptr;
static std::vector<observation> beforeObservations;
static longdouble beforeValues[2][5][9];static int beforeSets[2][5][9];
static bool retained(){
 if(!beforeDriver)return true;
 for(int j=0;j<2;j++)if(!same(p[j],beforeDriver[j]))return false;
 for(int j=0;j<12;j++)if(!same(obs[j],beforeObservations[j]))return false;
 for(int j=0;j<2;j++)for(int z=0;z<5;z++)for(int t=0;t<9;t++)if(!same(beforeValues[j][z][t],p[j].param[params[z]].val[t])||beforeSets[j][z][t]!=p[j].param[params[z]].paramSet[t])return false;
 return true;
}
static bool active=false;static size_t event=0;static std::ofstream*out;static std::string op;
static void emit(const char*phase,const std::string&result){
 *out<<"{\"event\":"<<event<<",\"phase\":"<<quote(phase)<<",\"operation\":"<<quote(op)<<",\"result\":"<<result<<",\"seeds\":["<<seeds[0]<<','<<seeds[1]<<"],\"globals\":"<<globals()<<",\"caller\":"<<((op=="driver"||op=="profile")?caller():"null");
#ifdef Z_OBSERVER
 *out<<",\"private\":"<<privateState()<<",\"calls\":"<<callState()<<",\"open_handles\":"<<handles.size();
#else
 *out<<",\"private\":null,\"calls\":null,\"open_handles\":null";
#endif
 *out<<"}\n";out->flush();
}
static void observeExit(){if(active){if(!retained())std::abort();emit("exit","{\"all_other_typed_fields_retained\":true,\"parameter_backing_retained\":true}");}}
static std::string decode(const std::string&s){if(s.empty()||s[0]!='x'||s.size()%2!=1)throw std::runtime_error("bad hex input");std::string r;for(size_t i=1;i<s.size();i+=2)r+=char(std::stoi(s.substr(i,2),nullptr,16));return r;}
int main(int argc,char**argv){
 if(argc!=4)return 2;setvbuf(stdout,nullptr,_IONBF,0);setvbuf(stderr,nullptr,_IONBF,0);
 std::ifstream input(argv[1]);std::ofstream output(argv[2]);out=&output;std::string line;std::string resourceRoot=argv[3];
 p=new pulsar[2]();obs=new observation[12]();
 for(int j=0;j<2;j++){p[j].obsn=obs+6*j;p[j].units=2;memset(p[j].JPL_EPHEMERIS,'~',sizeof p[j].JPL_EPHEMERIS);strcpy(p[j].JPL_EPHEMERIS,(resourceRoot+"/le.bin").c_str());
  for(int k:params){p[j].param[k].aSize=9;p[j].param[k].val=new longdouble[9]();p[j].param[k].paramSet=new int[9]();}
  double*ts[]={p[j].telDX_t,p[j].telDY_t,p[j].telDZ_t};double*vs[]={p[j].telDX_v,p[j].telDY_v,p[j].telDZ_v};for(int a=0;a<3;a++)for(int k=0;k<500;k++){ts[a][k]=49000+k;vs[a][k]=100*a+k*.25;}
 }
 for(int j=0;j<12;j++){auto&v=obs[j];v.sat=parse_longdouble("50010.25");memset(v.telID,'~',sizeof v.telID);strcpy(v.telID,"A");memset(v.flagID,'~',sizeof v.flagID);memset(v.flagVal,'!',sizeof v.flagVal);
  for(int k=0;k<MAX_CLK_CORR;k++){v.correctionsTT[k].correction=k*.125;memset(v.correctionsTT[k].corrects_to,'~',32);strcpy(v.correctionsTT[k].corrects_to,"UNUSED");}
  double*vs[]={v.sun_ssb,v.sun_earth,v.jupiter_earth,v.saturn_earth,v.venus_earth,v.uranus_earth,v.neptune_earth,v.earth_ssb,v.earthMoonBary_ssb,v.earthMoonBary_earth,v.nutations};
  for(int k=0;k<11;k++)for(int c=0;c<6;c++)vs[k][c]=1000+100*j+10*k+c+.25;
  for(int k=0;k<9;k++)for(int c=0;c<6;c++){v.planet_ssb[k][c]=2000+100*j+10*k+c+.25;v.planet_ssb_tmr[k][c]=3000+100*j+10*k+c+.75;v.planet_ssb_derv[k][c]=4000+100*j+10*k+c;}
 }
#ifdef Z_OBSERVER
 atexit(observeExit);
#endif
 while(std::getline(input,line)){
  std::vector<std::string>a;std::stringstream in(line);std::string word;while(std::getline(in,word,'\t'))a.push_back(decode(word));
  auto k=[&](size_t i){return std::stoi(a.at(i));};auto n=[&](size_t i){return strtod(a.at(i).c_str(),nullptr);};auto q=[&](size_t i){return parse_longdouble(a.at(i).c_str());};
  op=a.at(0);active=true;calls.clear();attempts.clear();printf("@BEGIN %zu %s\n",event,op.c_str());fprintf(stderr,"@BEGIN %zu %s\n",event,op.c_str());emit("begin","{}");std::ostringstream r;r<<'{';
  if(op=="profile")r<<"\"double_digits\":"<<std::numeric_limits<double>::digits<<",\"longdouble_digits\":"<<FLT128_MANT_DIG<<",\"longdouble_size\":"<<sizeof(longdouble)<<",\"long_size\":"<<sizeof(long)<<",\"max_flags\":"<<MAX_FLAGS<<",\"flag_length\":"<<MAX_FLAG_LEN;
  else if(op=="debug"){debugFlag=k(1);displayCVSversion=k(2);}
  else if(op=="pulsar"){auto&v=p[k(1)];v.nobs=k(2);v.units=k(3);v.eclCoord=k(4);strcpy(v.JPL_EPHEMERIS,(resourceRoot+"/"+a.at(5)+".bin").c_str());}
  else if(op=="observation"){auto&v=obs[k(1)];v.sat=q(2);strcpy(v.telID,a.at(3).c_str());v.correctionTT_Teph=q(4);}
  else if(op=="parameter"){auto&v=p[k(1)].param[params[a.at(2)=="phase"?0:1]];v.paramSet[k(3)]=k(4);v.val[k(3)]=q(5);}
  else if(op=="offset_mode"){p[k(1)].param[params[2+k(2)]].val[0]=q(3);}
  else if(op=="offset"){auto&v=p[k(1)];int*counts[]={&v.nTelDX,&v.nTelDY,&v.nTelDZ};double*ts[]={v.telDX_t,v.telDY_t,v.telDZ_t};double*vs[]={v.telDX_v,v.telDY_v,v.telDZ_v};*counts[k(2)]=k(3);for(int j=0;j<4;j++){ts[k(2)][j]=n(4+2*j);vs[k(2)][j]=n(5+2*j);}}
  else if(op=="tt"){auto&v=obs[k(1)];v.nclock_correction=k(2);for(size_t j=3;j<a.size();j++)v.correctionsTT[j-3].correction=n(j);}
  else if(op=="flag"){auto&v=obs[k(1)];strcpy(v.flagID[v.nFlags],a.at(2).c_str());strcpy(v.flagVal[v.nFlags],a.at(3).c_str());v.nFlags++;}
  else if(op=="clear_flags")obs[k(1)].nFlags=0;
  else if(op=="seed")seeds[k(1)]=std::stol(a.at(2));
  else if(op=="random")r<<"\"value\":"<<number(random2(seeds+k(1)));
  else if(op=="gaussian")r<<"\"value\":"<<number(gasdev(seeds+k(1)));
  else if(op=="driver"){
   beforeDriver=new pulsar[2]{p[0],p[1]};beforeObservations.assign(obs,obs+12);
   for(int j=0;j<2;j++)for(int z=0;z<5;z++)for(int t=0;t<9;t++){beforeValues[j][z][t]=p[j].param[params[z]].val[t];beforeSets[j][z][t]=p[j].param[params[z]].paramSet[t];}
   readEphemeris(p,k(1),k(2));
   if(!retained())return 10;
   delete[]beforeDriver;beforeDriver=nullptr;r<<"\"all_other_typed_fields_retained\":true,\"parameter_backing_retained\":true";
  }else return 3;
  r<<'}';emit("end",r.str());printf("@END %zu %s\n",event,op.c_str());fprintf(stderr,"@END %zu %s\n",event,op.c_str());active=false;event++;
 }
 for(int j=0;j<2;j++)for(int k:params){delete[]p[j].param[k].val;delete[]p[j].param[k].paramSet;}delete[]obs;delete[]p;return 0;
}
