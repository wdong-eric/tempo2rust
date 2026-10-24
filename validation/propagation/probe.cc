// AB: explicit caller backing; no source initializer or replacement dependency.
#include "tempo2.h"
#include "ifunc.h"
#include "shapelet.h"
#include <cassert>
#include <cfloat>
#include <cstdint>
#include <cstring>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <limits>
#include <map>
#include <sstream>
#include <stdexcept>
#include <type_traits>
#include <vector>
extern double iter_factorial(unsigned);
extern void TNothpl(int,double,double*);
extern double iterative_TNothpl(int,double,double&,double&);
extern double amod(double,double),findAngle(double,double,double,double);
extern double elsun2(int,int,double,double*,double*,double*);
extern void calcRotN(double,double,int*,int*,double*),mjd2date(int,int*,int*,int*,int*,int*);
extern void convertEcliptic(double,double,double*,double*),outputResults(double*,double*);
extern int readCurrentSheet(char*,float*,float*);
extern void mcl2(double,double,int,int,double,double,double*,double*,double*,double*,double*,double*,double*,double*,double*,double*,int*);
static std::string quote(const std::string&s){std::ostringstream o;o<<'"';for(unsigned char c:s){if(c=='"'||c=='\\')o<<'\\'<<c;else if(c<32||c>=127)o<<"\\u"<<std::hex<<std::setw(4)<<std::setfill('0')<<unsigned(c);else o<<c;}o<<'"';return o.str();}
static std::string hex(const void*p,size_t n){std::ostringstream o;auto*b=(const unsigned char*)p;for(size_t i=0;i<n;i++)o<<std::hex<<std::setw(2)<<std::setfill('0')<<unsigned(b[i]);return o.str();}
template<class T> std::string scalar(const T&x){return quote(hex(&x,sizeof x));}
static std::string scalar(const double&x){if(std::isnan(x))return "\"NaN\"";if(std::isinf(x))return x>0?"\"+Inf\"":"\"-Inf\"";return quote(hex(&x,sizeof x));}
static std::string scalar(const float&x){if(std::isnan(x))return "\"NaN\"";if(std::isinf(x))return x>0?"\"+Inf\"":"\"-Inf\"";return quote(hex(&x,sizeof x));}
static std::string scalar(const __float128&x){if(isnanq(x))return "\"NaN\"";if(isinfq(x))return x>0?"\"+Inf\"":"\"-Inf\"";return quote(hex(&x,sizeof x));}
static std::map<const void*,std::string> owners;
static std::map<std::string,std::string> fields,previous;
static std::string tag(const void*p){if(!p)return "null";auto it=owners.find(p);assert(it!=owners.end());return it->second;}
template<class T>void cells(const std::string&p,const T*x,size_t n){std::ostringstream o;o<<"{\"width\":"<<sizeof(T)<<",\"count\":"<<n<<",\"runs\":[";size_t at=0;while(at<n){size_t end=at+1;while(end<n&&memcmp(x+at,x+end,sizeof(T))==0)end++;if(at)o<<',';o<<'['<<end-at<<','<<scalar(x[at])<<']';at=end;}o<<"]}";fields[p]=o.str();}
template<class T>typename std::enable_if<std::is_arithmetic<T>::value||std::is_enum<T>::value,void>::type dump(const std::string&p,const T&x){cells(p,&x,1);}
static void dump(const std::string&p,const __float128&x){cells(p,&x,1);}
template<class T>void dump(const std::string&p,T*const&x){fields[p]="{\"owner\":"+quote(tag(reinterpret_cast<const void*>(x)))+"}";}
void dump(const std::string&,const clock_correction&);void dump(const std::string&,const observation&);void dump(const std::string&,const storePrecision&);void dump(const std::string&,const FitOutput&);void dump(const std::string&,const FitInfo&);void dump(const std::string&,const parameter&);void dump(const std::string&,const pulsar&);
template<class T>using Leaf=typename std::remove_all_extents<T>::type;
template<class T,size_t N>typename std::enable_if<std::is_arithmetic<Leaf<T>>::value||std::is_enum<Leaf<T>>::value||std::is_same<Leaf<T>,__float128>::value,void>::type dump(const std::string&p,const T(&x)[N]){cells(p,reinterpret_cast<const Leaf<T>*>(x),sizeof x/sizeof(Leaf<T>));}
template<class T,size_t N>typename std::enable_if<!(std::is_arithmetic<Leaf<T>>::value||std::is_enum<Leaf<T>>::value||std::is_same<Leaf<T>,__float128>::value||std::is_pointer<T>::value),void>::type dump(const std::string&p,const T(&x)[N]){for(size_t i=0;i<N;i++)dump(p+"["+std::to_string(i)+"]",x[i]);}
template<class T,size_t N>typename std::enable_if<std::is_pointer<T>::value,void>::type dump(const std::string&p,const T(&x)[N]){std::ostringstream o;o<<"{\"pointer_count\":"<<N<<",\"runs\":[";size_t at=0;while(at<N){size_t end=at+1;while(end<N&&x[end]==x[at])end++;if(at)o<<',';o<<'['<<end-at<<','<<quote(tag(reinterpret_cast<const void*>(x[at])))<<']';at=end;}o<<"]}";fields[p]=o.str();}
template<class T>typename std::enable_if<std::is_arithmetic<T>::value||std::is_enum<T>::value,void>::type seed(T&x,int v){x=static_cast<T>(v);}
static void seed(__float128&x,int v){x=v+0.125Q;}
static void seed(double&x,int v){x=v+.25;}
void seed(clock_correction&,int);void seed(observation&,int);
template<size_t N>void seed(char(&x)[N],int v){memset(x,'~',N);snprintf(x,N,"seed%d",v);}
template<class T,size_t N>void seed(T(&x)[N],int v){for(size_t i=0;i<N;i++)seed(x[i],v);}
template<size_t N>void dump(const std::string&p,const parameter(&x)[N]);
#include "generated.h"
static std::string parameterRecord(const parameter&x){auto retained=std::move(fields);fields.clear();dump("",x);std::ostringstream o;o<<'{';bool comma=false;for(auto&kv:fields){if(comma)o<<',';comma=true;o<<quote(kv.first.substr(1))<<':'<<kv.second;}o<<'}';fields=std::move(retained);return o.str();}
// Compare declared members only: struct padding is not an observation.
static bool sameParameter(const parameter&a,const parameter&b){return a.label==b.label&&a.shortlabel==b.shortlabel&&a.val==b.val&&a.err==b.err&&a.fitFlag==b.fitFlag&&a.paramSet==b.paramSet&&a.prefit==b.prefit&&a.prefitErr==b.prefitErr&&a.aSize==b.aSize&&a.nLinkTo==b.nLinkTo&&a.nLinkFrom==b.nLinkFrom&&memcmp(a.linkTo,b.linkTo,sizeof a.linkTo)==0&&memcmp(a.linkFrom,b.linkFrom,sizeof a.linkFrom)==0;}
template<size_t N>void dump(const std::string&p,const parameter(&x)[N]){std::ostringstream o;o<<"{\"parameter_count\":"<<N<<",\"runs\":[";size_t at=0;while(at<N){size_t end=at+1;while(end<N&&sameParameter(x[at],x[end]))end++;if(at)o<<',';o<<'['<<end-at<<','<<parameterRecord(x[at])<<']';at=end;}o<<"]}";fields[p]=o.str();}

static pulsar* ps;
static observation* ob;
static const std::map<std::string,int> params={
#define P(n) {#n,param_##n}
 P(raj),P(decj),P(posepoch),P(pmra),P(pmdec),P(pmrv),P(px),P(dm),P(cm),P(dmepoch),P(dmmodel),P(dmx),P(dmxr1),P(dmxr2),P(dm_sin1yr),P(dm_cos1yr),P(chromx),P(chromxr1),P(chromxr2),P(ne_sw_sin),P(telx),P(tely),P(telz),P(dshk),P(tzrmjd),P(tzrfrq)
#undef P
};
static void snapshot(std::ostream&o){fields.clear();for(int j=0;j<2;j++){std::string p="p"+std::to_string(j);dump(p,ps[j]);for(auto q:params){auto&a=ps[j].param[q.second];cells(p+".param["+std::to_string(q.second)+"].val.data",a.val,16);cells(p+".param["+std::to_string(q.second)+"].paramSet.data",a.paramSet,16);}}for(int j=0;j<12;j++)dump("obs"+std::to_string(j),ob[j]);dump("TK_warnCount",TK_warnCount);dump("TK_errorCount",TK_errorCount);o<<"{\"field_count\":"<<fields.size()<<",\"changes\":[";bool comma=false;for(auto&kv:fields)if(!previous.count(kv.first)||previous[kv.first]!=kv.second){if(comma)o<<',';comma=true;o<<'['<<quote(kv.first)<<','<<kv.second<<']';}o<<"]}";assert(previous.empty()||previous.size()==fields.size());previous=fields;}
static const int* mask=nullptr;
static std::vector<std::string> extra;
void ab_mask(const int*t){mask=t;}
void ab_call(const char*name,int p,int i,double delt,double dt){extra.push_back("{\"kind\":\"call\",\"routine\":"+quote(name)+",\"p\":"+std::to_string(p)+",\"i\":"+std::to_string(i)+",\"delt\":"+scalar(delt)+",\"dt\":"+scalar(dt)+"}");}
void ab_iteration(int p,int i,int loop,double old,double now,double delt){extra.push_back("{\"kind\":\"iteration\",\"p\":"+std::to_string(p)+",\"i\":"+std::to_string(i)+",\"loop\":"+std::to_string(loop)+",\"old\":"+scalar(old)+",\"new\":"+scalar(now)+",\"delt\":"+scalar(delt)+"}");}
void ab_tt(observation*p){extra.push_back("{\"kind\":\"TT\",\"owner\":"+quote(tag(p))+"}");}
void ab_sheet(const char*f,int n,double lon,double lat){extra.push_back("{\"kind\":\"sheet\",\"file\":"+quote(f)+",\"count\":"+std::to_string(n)+",\"lon\":"+scalar(lon)+",\"lat\":"+scalar(lat)+"}");}
static void privateState(std::ostream&o){o<<"{\"mask\":"<<(mask?std::to_string(*mask):"null")<<",\"events\":[";for(size_t i=0;i<extra.size();i++){if(i)o<<',';o<<extra[i];}o<<"]}";extra.clear();}
static std::ofstream* activeOut=nullptr;
static bool active=false;
static int activeEvent=0;
static std::string activeOp;
static void observeExit(){if(!active)return;*activeOut<<"{\"event\":"<<activeEvent<<",\"phase\":\"exit\",\"op\":"<<quote(activeOp)<<",\"state\":";snapshot(*activeOut);*activeOut<<",\"private\":";privateState(*activeOut);*activeOut<<"}\n";activeOut->flush();}
static std::string decode(const std::string&s){assert(!s.empty()&&s[0]=='x'&&s.size()%2==1);std::string r;for(size_t i=1;i<s.size();i+=2)r+=char(std::stoi(s.substr(i,2),nullptr,16));return r;}
int main(int argc,char**argv){assert(argc==4);setvbuf(stdout,nullptr,_IONBF,0);setvbuf(stderr,nullptr,_IONBF,0);std::ifstream input(argv[1]);std::ofstream out(argv[2]);out<<"{\"launch\":"<<quote(argv[3])<<"}\n";out.flush();MAX_OBSN=6;MAX_PSR=2;ps=new pulsar[2]();ob=new observation[12]();
 for(int j=0;j<12;j++){owners[ob+j]="obs"+std::to_string(j);auto&v=ob[j];seed(v,11+j);v.nFlags=0;v.obsNjump=0;v.sat=parse_longdouble("58000.25");v.bat=v.sat;v.bbat=v.sat-1;v.freq=1400;v.delayCorr=1;v.clockCorr=1;v.einsteinRate=1.00001;strcpy(v.telID,"A");v.nclock_correction=2;v.correctionsTT[0].correction=37;v.correctionsTT[1].correction=32.184;v.correctionTT_TB=0.001;v.troposphericDelay=0.001+j*0.00001;v.tdis1=.002;v.tdis2=.003;v.spherical_solar_wind=.004;v.shapiroDelaySun=.005;v.shapiroDelayJupiter=.006;v.shapiroDelaySaturn=.007;v.shapiroDelayVenus=.008;v.shapiroDelayUranus=.009;v.shapiroDelayNeptune=.010;v.shklovskii=.011;v.roemer=100;v.batCorr=0.1Q;v.deleted=0;
  for(int k=0;k<6;k++){v.earth_ssb[k]=k<3?100+20*k:1e-5*(k-2);v.sun_ssb[k]=k<3?2+k:1e-7;v.observatory_earth[k]=k<3?.01*(k+1):71+k;v.jupiter_earth[k]=1000+50*k;v.saturn_earth[k]=2000+50*k;v.venus_earth[k]=100+50*k;v.uranus_earth[k]=3000+50*k;v.neptune_earth[k]=4000+50*k;}for(int k=0;k<3;k++){v.siteVel[k]=(k+1)*1e-7;v.zenith[k]=k==2?1:0;}}
 for(int j=0;j<2;j++){auto&p=ps[j];p.obsn=ob+j*6;p.nobs=1;p.planetShapiro=1;p.correctTroposphere=2;p.dilateFreq=1;p.ipm=1;p.ne_sw=4;p.dm_series_type=series_simple_pn;p.posPulsar[0]=.6;p.posPulsar[1]=.8;p.posPulsarEquatorial[0]=.6;p.posPulsarEquatorial[1]=.8;p.velPulsar[0]=1e-5;p.velPulsar[1]=-2e-5;p.accPulsar[2]=1e-8;p.ne_sw_ifuncN=0;p.tzrobs=ob[j*6+5];strcpy(p.tzrsite,"A");
  for(auto q:params){auto&a=p.param[q.second];a.aSize=16;a.val=new longdouble[16]();a.paramSet=new int[16]();owners[a.val]="p"+std::to_string(j)+"."+q.first+".val";owners[a.paramSet]="p"+std::to_string(j)+"."+q.first+".flags";}
  for(int q:{param_posepoch,param_dmepoch,param_tzrmjd})p.param[q].val[0]=58000;
  p.param[param_dm].val[0]=10+j;p.param[param_cm].val[0]=.0002;p.param[param_raj].val[0]=1;p.param[param_decj].val[0]=.3;p.param[param_tzrfrq].val[0]=1400;p.param[param_dshk].val[0]=1;p.param[param_pmra].val[0]=3;p.param[param_pmdec].val[0]=4;p.param[param_px].val[0]=1;
 }
 activeOut=&out;
#ifdef AB_OBSERVER
 atexit(observeExit);
#endif
 std::string line;int event=0;
 while(std::getline(input,line)){std::vector<std::string>a;std::stringstream in(line);std::string word;while(std::getline(in,word,'\t'))a.push_back(decode(word));auto k=[&](int i){return std::stoi(a.at(i));};auto n=[&](int i){return strtod(a.at(i).c_str(),nullptr);};auto q=[&](int i){return parse_longdouble(a.at(i).c_str());};auto c=[&](int i){return const_cast<char*>(a.at(i).c_str());};auto op=a.at(0);active=true;activeEvent=event;activeOp=op;
  printf("@BEGIN %d %s\n",event,op.c_str());fprintf(stderr,"@BEGIN %d %s\n",event,op.c_str());out<<"{\"event\":"<<event<<",\"phase\":\"begin\",\"op\":"<<quote(op)<<",\"state\":";snapshot(out);out<<"}\n";out.flush();std::ostringstream r;r<<'[';
  if(op=="profile")r<<sizeof(double)<<','<<DBL_MANT_DIG<<','<<sizeof(long double)<<','<<LDBL_MANT_DIG<<','<<sizeof(longdouble)<<','<<FLT128_MANT_DIG;
  else if(op=="env")setenv(c(1),c(2),1);
  else if(op=="debug"){debugFlag=k(1);displayCVSversion=k(2);quietFlag=k(3);}
  else if(op=="fast")veryFast=k(1);
  else if(op=="p")setfield(ps[k(1)],a[2],k(3),a[4]);
  else if(op=="o")setfield(ob[k(1)],a[2],k(3),a[4]);
  else if(op=="parameter"){auto&v=ps[k(1)].param[params.at(a[2])];assert(k(3)>=0&&k(3)<16);v.val[k(3)]=q(4);v.paramSet[k(3)]=k(5);}
  else if(op=="link"){auto&v=ps[k(1)].param[params.at(a[2])];v.nLinkTo=k(3);v.linkTo[0]=params.at(a[4]);}
  else if(op=="flag"){auto&v=ob[k(1)];assert(v.nFlags<MAX_FLAGS);strcpy(v.flagID[v.nFlags],c(2));strcpy(v.flagVal[v.nFlags],c(3));v.nFlags++;}
  else if(op=="shape_event"){auto&p=ps[k(1)];int i=k(2);assert(i>=0&&i<2);p.nTNShapeletEvents=i+1;p.TNShapeletEvFScale[i]=n(3);p.TNShapeletEvN[i]=k(4);p.TNShapeletEvPos[i]=n(5);p.TNShapeletEvWidth[i]=n(6);for(int z=0;z<8;z++)p.TNShapeletEvCoef[i][z]=n(7+z);}
  else if(op=="bclt")calculate_bclt(ps,k(1));
  else if(op=="shapiro")shapiro_delay(ps,2,k(1),k(2),n(3),n(4));
  else if(op=="dm")dm_delays(ps,2,k(1),k(2),n(3),n(4));
  else if(op=="bats")formBats(ps,k(1));
  else if(op=="secular")secularMotion(ps,k(1));
  else if(op=="init")refphs_init(ps,k(1));
  else if(op=="clean")refphs_clean(ps,k(1));
  else if(op=="precision")recordPrecision(ps,q(1),c(2),c(3));
  else if(op=="solar")r<<scalar(solarWindModel(ps+k(1),k(2)));
  else if(op=="ifunc"||op=="basis"||op=="sinc"){double t[6],v[6];for(int z=0;z<6;z++){t[z]=n(4+z*2);v[z]=n(5+z*2);}double ans=op=="ifunc"?ifunc(t,v,n(1),k(2)):op=="basis"?ifunc(t,n(1),k(2),k(3)):sinfunc(t,n(1),k(3));r<<scalar(ans);}
  else if(op=="factorial")r<<scalar(iter_factorial(k(1)));
  else if(op=="hermite"){double v[12];for(int z=0;z<12;z++)v[z]=91+z;TNothpl(k(1),n(2),v);for(int z=0;z<12;z++){if(z)r<<',';r<<scalar(v[z]);}}
  else if(op=="recurrence"){double y0=n(3),y1=n(4);double ans=iterative_TNothpl(k(1),n(2),y0,y1);r<<scalar(ans)<<','<<scalar(y0)<<','<<scalar(y1);}
  else if(op=="shapelet"){double v[12];for(int z=0;z<12;z++)v[z]=n(5+z);r<<scalar(evaluateShapelet(k(1),n(2),n(3),v,n(4)));}
  else if(op=="angle")r<<scalar(findAngle(n(1),n(2),n(3),n(4)));
  else if(op=="amod")r<<scalar(amod(n(1),n(2)));
  else if(op=="ecliptic"){double lon=81,lat=82;convertEcliptic(n(1),n(2),&lon,&lat);r<<scalar(lon)<<','<<scalar(lat);}
  else if(op=="elsun"){double gst=81,ra=82,dec=83;double ans=elsun2(k(1),k(2),n(3),&gst,&ra,&dec);r<<scalar(ans)<<','<<scalar(gst)<<','<<scalar(ra)<<','<<scalar(dec);}
  else if(op=="date"){int v[5]={81,82,83,84,85};mjd2date(k(1),v,v+1,v+2,v+3,v+4);for(int z=0;z<5;z++){if(z)r<<',';r<<v[z];}}
  else if(op=="rotation"){int x=81,y=82;double b=83;calcRotN(n(1),n(2),&x,&y,&b);r<<x<<','<<y<<','<<scalar(b);}
  else if(op=="sheet"){float lon[500],lat[500];for(int z=0;z<500;z++){lon[z]=81;lat[z]=82;}int count=readCurrentSheet(c(1),lon,lat);r<<count;for(int z=0;z<500;z++)r<<','<<scalar(lon[z])<<','<<scalar(lat[z]);}
  else if(op=="mcl"){double h[36],c[36],t[36],v[7];int count=-1;for(int z=0;z<36;z++){h[z]=81;c[z]=82;t[z]=83;}for(double&x:v)x=84;mcl2(n(1),n(2),k(3),k(4),n(5),n(6),h,c,t,v,v+1,v+2,v+3,v+4,v+5,v+6,&count);r<<count;for(auto*x:{h,c,t})for(int z=0;z<36;z++)r<<','<<scalar(x[z]);for(double x:v)r<<','<<scalar(x);}
  else if(op=="output"){double c[36],h[36];for(int z=0;z<36;z++){c[z]=n(1)+z*n(2);h[z]=n(3)+z*n(4);}outputResults(c,h);}
  else throw std::runtime_error("unknown action "+op);
  r<<']';out<<"{\"event\":"<<event<<",\"phase\":\"end\",\"op\":"<<quote(op)<<",\"result\":"<<r.str()<<",\"state\":";snapshot(out);out<<",\"private\":";
#ifdef AB_OBSERVER
  privateState(out);
#else
  out<<"null";
#endif
  out<<"}\n";out.flush();printf("@END %d %s\n",event,op.c_str());fprintf(stderr,"@END %d %s\n",event,op.c_str());active=false;event++;
 }
}
