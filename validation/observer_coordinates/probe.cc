// Assignment X: unchanged raw source, read-only snapshots of addressable state.
#include <cmath>
#include <cstdint>
#include <cstring>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <limits>
#include "observatory.C"
#include "tabulatedfunction.C"
#include "clkcorr.C"

template<class T> struct Equal {
  static bool check(const T&a,const T&b) { return memcmp(&a,&b,sizeof(T))==0; }
};
template<class T> bool same(const T&a,const T&b) { return Equal<T>::check(a,b); }
template<class T,size_t N> struct Equal<T[N]> {
  static bool check(const T(&a)[N],const T(&b)[N]) {
    for(size_t i=0;i<N;i++) if(!same(a[i],b[i])) return false;
    return true;
  }
};
#include "field_checks.h"

static std::string quote(const std::string&s) {
  std::ostringstream o;o<<'"';
  for(unsigned char c:s) {
    if(c=='"'||c=='\\')o<<'\\'<<c;
    else if(c<32||c>=127)o<<"\\u"<<std::hex<<std::setw(4)<<std::setfill('0')<<unsigned(c);
    else o<<c;
  }
  o<<'"';return o.str();
}
static std::string bytes(const void*p,size_t n) {
  std::ostringstream o;auto*b=static_cast<const unsigned char*>(p);
  for(size_t i=0;i<n;i++)o<<std::hex<<std::setw(2)<<std::setfill('0')<<unsigned(b[i]);
  return o.str();
}
static std::string number(double x) {
  if(std::isnan(x))return "{\"class\":\"NaN\"}";
  if(std::isinf(x))return x>0?"{\"class\":\"+Infinity\"}":"{\"class\":\"-Infinity\"}";
  uint64_t u;memcpy(&u,&x,8);std::ostringstream o;
  o<<"{\"class\":\"finite\",\"bits\":\""<<std::hex<<std::setw(16)<<std::setfill('0')<<u<<"\"}";
  return o.str();
}
static std::string chars(const char*p,size_t known,size_t capacity) {
  return "{\"known\":"+std::to_string(known)+",\"capacity\":"+std::to_string(capacity)+",\"bytes\":"+quote(bytes(p,known))+"}";
}
static std::string coordinates(const observatory&o) {
  return "["+number(o.x)+","+number(o.y)+","+number(o.z)+","+number(o.longitude_grs80)+","+number(o.latitude_grs80)+","+number(o.height_grs80)+"]";
}
struct Extents {size_t name,code,clock;};
static std::vector<Extents> obsKnown;
static std::vector<size_t> aliasKnown, nextGroups;
static void noteAppends(size_t oldObs,size_t oldAliases) {
  if(observatories.size()>oldObs) {
    if(nextGroups.empty())nextGroups.push_back(observatories.size()-oldObs);
    size_t i=oldObs;
    for(size_t n:nextGroups) {
      Extents k={0,0,0};
      for(size_t j=0;j<n;j++,i++) {
        const auto&o=observatories.at(i);
        k.name=std::max(k.name,strlen(o.name)+1); k.code=std::max(k.code,strlen(o.code)+1);
        k.clock=std::max(k.clock,strlen(o.clock_name)+1);obsKnown.push_back(k);
      }
    }
    if(i!=observatories.size())throw std::runtime_error("file boundary metadata mismatch");
    nextGroups.clear();
  }
  size_t extent=0;
  for(size_t i=oldAliases;i<observatoryAliasLists.size();i++) {
    extent=std::max(extent,strlen(observatoryAliasLists[i].code)+1);aliasKnown.push_back(extent);
  }
}
static std::string state() {
  std::ostringstream o;o<<"{\"observatories_initialized\":"<<observatories_initialised<<",\"observatories\":[";
  for(size_t i=0;i<observatories.size();i++) {
    if(i)o<<',';auto&r=observatories[i];auto&k=obsKnown.at(i);
    o<<"{\"coordinates\":"<<coordinates(r)<<",\"name\":"<<chars(r.name,k.name,32)<<",\"code\":"<<chars(r.code,k.code,16)<<",\"clock\":"<<chars(r.clock_name,k.clock,16)<<'}';
  }
  o<<"],\"aliases\":[";
  for(size_t i=0;i<observatoryAliasLists.size();i++) {
    if(i)o<<',';auto&r=observatoryAliasLists[i];o<<"{\"code\":"<<chars(r.code,aliasKnown.at(i),100)<<",\"warned\":"<<(r.warned?"true":"false")<<",\"aliases\":[";
    for(size_t j=0;j<r.aliases.size();j++){if(j)o<<',';o<<quote(r.aliases[j]);}o<<"]}";
  }
  o<<"],\"clock_initialized\":"<<(g_clockCorrectionsInitialized?"true":"false")<<",\"clock_functions\":[";
  for(size_t i=0;i<g_clockCorrectionFunctions.size();i++) {
    if(i)o<<',';auto&f=g_clockCorrectionFunctions[i];
    o<<"{\"path\":"<<quote(f.fullPath)<<",\"name\":"<<quote(f.fileName)<<",\"from\":"<<quote(f.clockFrom)<<",\"to\":"<<quote(f.clockTo)<<",\"badness\":"<<number(f.badness)<<",\"loaded\":"<<(f.loaded?"true":"false")<<",\"start\":"<<number(f.startMJD)<<",\"end\":"<<number(f.endMJD)<<",\"samples\":[";
    if(f.loaded)for(size_t j=0;j<f.table.samples.size();j++){if(j)o<<',';o<<'['<<number(f.table.samples[j].x)<<','<<number(f.table.samples[j].y)<<']';}
    o<<"]}";
  }
  o<<"],\"clock_sequences\":[";
  for(size_t i=0;i<g_clockCorrectionSequences.size();i++) {
    if(i)o<<',';auto&s=g_clockCorrectionSequences[i];
    o<<"{\"from\":"<<quote(s.clockFrom)<<",\"to\":"<<quote(s.clockTo)<<",\"from_key\":"<<quote(s.clockFromKey)<<",\"to_key\":"<<quote(s.clockToKey)<<",\"start\":"<<number(s.startMJD)<<",\"end\":"<<number(s.endMJD)<<",\"indices\":[";
    for(size_t j=0;j<s.functionIndices.size();j++){if(j)o<<',';o<<s.functionIndices[j];}o<<"]}";
  }
  o<<"],\"warnings\":"<<TK_warnCount<<",\"errors\":"<<TK_errorCount<<",\"warning_log\":[";
  for(unsigned i=0;i<std::min(TK_warnCount,unsigned(TK_MAX_ERRORS));i++){if(i)o<<',';o<<quote(TK_warnlog[i]);}
  o<<"],\"error_log\":[";for(unsigned i=0;i<std::min(TK_errorCount,unsigned(TK_MAX_ERRORS));i++){if(i)o<<',';o<<quote(TK_errorlog[i]);}
  o<<"]}";return o.str();
}

extern double ut1red(double,int);
extern double lmst(double,double,double*,double*);
extern double get_precessionMatrix(double[3][3],double,double,double);
extern double ang(int,double);
extern void remove_white(char*);
static const int *utRead=nullptr,*utMjd=nullptr,*utEntry=nullptr,*utCount=nullptr,*utCount2=nullptr;
static const double *utLo=nullptr,*utHi=nullptr;
void x_ut1_observer(const int*r,const double*lo,const double*hi,const int*m,const int*e,const int*c,const int*c2) {
  utRead=r;utLo=lo;utHi=hi;utMjd=m;utEntry=e;utCount=c;utCount2=c2;
}
static std::string privateState() {
  if(!utRead)return "null";
  std::ostringstream o;o<<"{\"read\":"<<*utRead<<",\"mjd1\":"<<number(*utLo)<<",\"mjd2\":"<<number(*utHi)
   <<",\"count\":"<<*utCount<<",\"count2\":"<<*utCount2<<",\"mjdVal\":[";
  for(int i=0;i<1000;i++){if(i)o<<',';o<<utMjd[i];}o<<"],\"entry\":[";
  for(int i=0;i<10000;i++){if(i)o<<',';o<<utEntry[i];}o<<"]}";return o.str();
}
static std::string quad(longdouble x) {
  if(isnanq(x))return "{\"class\":\"NaN\"}";
  if(isinfq(x))return x>0?"{\"class\":\"+Infinity\"}":"{\"class\":\"-Infinity\"}";
  return "{\"class\":\"finite\",\"little_endian_bytes\":"+quote(bytes(&x,sizeof x))+"}";
}
template<size_t N> static std::string vector(const double(&v)[N]) {
  std::ostringstream o;o<<'[';for(size_t i=0;i<N;i++){if(i)o<<',';o<<number(v[i]);}o<<']';return o.str();
}
static const int params[]={param_raj,param_decj,param_pmra,param_pmdec,param_pmra2,param_pmdec2,
 param_telx,param_tely,param_telz,param_telEpoch,param_tel_dx,param_tel_dy,param_tel_dz};
static std::string caller(pulsar*p,observation*obs) {
  std::ostringstream o;o<<"{\"pulsars\":[";
  for(int j=0;j<2;j++) {
    if(j)o<<',';auto&v=p[j];o<<"{\"owner\":"<<j<<",\"observation_owner\":"<<j*6<<",\"nobs\":"<<v.nobs
      <<",\"method\":"<<v.t2cMethod<<",\"ecliptic\":"<<v.eclCoord<<",\"warnings\":"<<v.noWarnings
      <<",\"troposphere\":"<<v.correctTroposphere<<",\"override\":"<<chars(v.clockFromOverride,sizeof v.clockFromOverride,sizeof v.clockFromOverride)
      <<",\"eop_file\":"<<chars(v.eopc04_file,sizeof v.eopc04_file,sizeof v.eopc04_file)
      <<",\"position\":"<<vector(v.posPulsar)<<",\"equatorial\":"<<vector(v.posPulsarEquatorial)
      <<",\"velocity\":"<<vector(v.velPulsar)<<",\"acceleration\":"<<vector(v.accPulsar)<<",\"parameters\":[";
    for(size_t q=0;q<sizeof(params)/sizeof(*params);q++) {
      if(q)o<<',';auto&pr=v.param[params[q]];o<<"{\"index\":"<<params[q]<<",\"values\":[";
      for(int k=0;k<4;k++){if(k)o<<',';o<<quad(pr.val[k]);}o<<"],\"set\":[";
      for(int k=0;k<4;k++){if(k)o<<',';o<<pr.paramSet[k];}o<<"]}";
    }
    o<<"],\"offsets\":[";
    int counts[]={v.nTelDX,v.nTelDY,v.nTelDZ};double*ts[]={v.telDX_t,v.telDY_t,v.telDZ_t};double*vs[]={v.telDX_v,v.telDY_v,v.telDZ_v};
    for(int ax=0;ax<3;ax++){if(ax)o<<',';o<<"{\"count\":"<<counts[ax]<<",\"backing\":[";
      for(int i=0;i<4;i++){if(i)o<<',';o<<'['<<number(ts[ax][i])<<','<<number(vs[ax][i])<<']';}o<<"]}";}o<<"]}";
  }
  o<<"],\"observations\":[";
  for(int j=0;j<12;j++) {
    if(j)o<<',';auto&v=obs[j];o<<"{\"owner\":"<<j<<",\"sat\":"<<quad(v.sat)<<",\"telID\":"<<chars(v.telID,sizeof v.telID,sizeof v.telID)
     <<",\"delayCorr\":"<<v.delayCorr<<",\"clockCorr\":"<<v.clockCorr<<",\"correctionUT1\":"<<number(v.correctionUT1)
     <<",\"position\":"<<vector(v.observatory_earth)<<",\"velocity\":"<<vector(v.siteVel)<<",\"zenith\":"<<vector(v.zenith)
     <<",\"nutations\":"<<vector(v.nutations)<<",\"nclock\":"<<v.nclock_correction<<",\"clock_steps\":[";
    for(int i=0;i<2;i++){if(i)o<<',';o<<'['<<number(v.correctionsTT[i].correction)<<','<<chars(v.correctionsTT[i].corrects_to,sizeof v.correctionsTT[i].corrects_to,sizeof v.correctionsTT[i].corrects_to)<<']';}
    o<<"],\"nFlags\":"<<v.nFlags<<",\"flags\":[";
    for(int i=0;i<v.nFlags;i++){if(i)o<<',';o<<'['<<chars(v.flagID[i],sizeof v.flagID[i],sizeof v.flagID[i])<<','<<chars(v.flagVal[i],sizeof v.flagVal[i],sizeof v.flagVal[i])<<']';}o<<"]}";
  }o<<"]}";return o.str();
}
static std::string decode(const std::string&s) {
  if(s.empty()||s[0]!='x'||s.size()%2!=1)throw std::runtime_error("bad hex input");
  std::string out;for(size_t i=1;i<s.size();i+=2)out+=char(std::stoi(s.substr(i,2),nullptr,16));return out;
}
static bool actionActive=false;
static size_t eventIndex=0,oldObs=0,oldAliases=0;
static std::ofstream*outFile;
static std::string operation;
static pulsar*pulsars;
static observation*observations;
static void observeExit() {
  if(!actionActive)return;
  noteAppends(oldObs,oldAliases);
  *outFile<<"{\"event\":"<<eventIndex<<",\"phase\":\"exit\",\"operation\":"<<quote(operation)
   <<",\"state\":"<<state()<<",\"caller\":"<<caller(pulsars,observations)<<",\"private\":"<<privateState()<<"}\n";outFile->flush();
}
int main(int argc,char**argv) {
  if(argc!=3)return 2;
  setvbuf(stdout,nullptr,_IONBF,0);setvbuf(stderr,nullptr,_IONBF,0);
  std::ifstream input(argv[1]);std::ofstream out(argv[2]);std::string line;
  auto*p=new pulsar[2]();auto*obs=new observation[12]();p[0].obsn=obs;p[1].obsn=obs+6;
  for(int j=0;j<2;j++) {
    p[j].correctTroposphere=1;strcpy(p[j].eopc04_file,"eop");
    for(int q:params){p[j].param[q].val=new longdouble[4]();p[j].param[q].paramSet=new int[4]();}
    for(int k=0;k<3;k++){p[j].posPulsar[k]=100+j*10+k;p[j].posPulsarEquatorial[k]=200+j*10+k;p[j].velPulsar[k]=300+j*10+k;p[j].accPulsar[k]=400+j*10+k;}
  }
  for(int j=0;j<12;j++) {
    auto&v=obs[j];memset(v.telID,'~',sizeof v.telID);strcpy(v.telID,"A");v.sat=parse_longdouble("58010.25");
    v.delayCorr=1;v.clockCorr=1;v.correctionUT1=900+j;
    for(int k=0;k<6;k++){v.observatory_earth[k]=1000+10*j+k;v.nutations[k]=(k+1)*1e-6;}
    for(int k=0;k<3;k++){v.siteVel[k]=2000+10*j+k;v.zenith[k]=3000+10*j+k;}
    memset(v.flagID,'~',sizeof v.flagID);memset(v.flagVal,'!',sizeof v.flagVal);
  }
  pulsars=p;observations=obs;outFile=&out;
#ifdef X_OBSERVER
  atexit(observeExit);
#endif
  while(std::getline(input,line)) {
    std::vector<std::string>a;std::stringstream in(line);std::string word;
    while(std::getline(in,word,'\t'))a.push_back(decode(word));
    auto n=[&](size_t i){return strtod(a.at(i).c_str(),nullptr);};auto c=[&](size_t i){return const_cast<char*>(a.at(i).c_str());};auto k=[&](size_t i){return std::stoi(a.at(i));};
    auto q=[&](size_t i){return parse_longdouble(c(i));};
    const auto&op=a.at(0);oldObs=observatories.size();oldAliases=observatoryAliasLists.size();operation=op;actionActive=true;
    printf("@BEGIN %zu %s\n",eventIndex,op.c_str());fprintf(stderr,"@BEGIN %zu %s\n",eventIndex,op.c_str());
    out<<"{\"event\":"<<eventIndex<<",\"phase\":\"begin\",\"operation\":"<<quote(op)<<",\"caller\":"<<caller(p,obs)<<"}\n";out.flush();
    std::ostringstream r;r<<'{';
    if(op=="profile")r<<"\"double_digits\":"<<std::numeric_limits<double>::digits<<",\"longdouble_size\":"<<sizeof(longdouble)<<",\"longdouble_digits\":"<<FLT128_MANT_DIG;
    else if(op=="env")setenv(c(1),c(2),1);
    else if(op=="debug"){debugFlag=k(1);displayCVSversion=k(2);quietFlag=k(3);}
    else if(op=="pulsar"){auto&v=p[k(1)];v.nobs=k(2);v.t2cMethod=k(3);v.eclCoord=k(4);v.noWarnings=k(5);strcpy(v.clockFromOverride,c(6));strcpy(v.eopc04_file,c(7));}
    else if(op=="observation"){auto&v=obs[k(1)];v.sat=q(2);strcpy(v.telID,c(3));v.delayCorr=k(4);v.clockCorr=k(5);v.correctionUT1=n(6);}
    else if(op=="parameter"){auto&v=p[k(1)].param[params[k(2)]];v.val[k(3)]=q(4);v.paramSet[k(3)]=k(5);}
    else if(op=="flag"){auto&v=obs[k(1)];strcpy(v.flagID[v.nFlags],c(2));strcpy(v.flagVal[v.nFlags],c(3));v.nFlags++;}
    else if(op=="clear_flags")obs[k(1)].nFlags=0;
    else if(op=="tt"){auto&v=obs[k(1)];v.nclock_correction=k(2);v.correctionsTT[0].correction=n(3);v.correctionsTT[1].correction=n(4);strcpy(v.correctionsTT[0].corrects_to,"TAI");strcpy(v.correctionsTT[1].corrects_to,"TT(TAI)");}
    else if(op=="nutation"){auto&v=obs[k(1)];for(int j=0;j<6;j++)v.nutations[j]=n(2+j);}
    else if(op=="offset"){auto&v=p[k(1)];int ax=k(2),count=k(3);int*counts[]={&v.nTelDX,&v.nTelDY,&v.nTelDZ};double*ts[]={v.telDX_t,v.telDY_t,v.telDZ_t};double*vs[]={v.telDX_v,v.telDY_v,v.telDZ_v};*counts[ax]=count;v.param[params[10+ax]].val[0]=q(4);for(int j=0;j<4;j++){ts[ax][j]=n(5+2*j);vs[ax][j]=n(6+2*j);}}
    else if(op=="vector"||op=="driver"||op=="tai") {
      auto*before=new pulsar[2]{p[0],p[1]};std::vector<observation> prev(obs,obs+12);
      longdouble values[2][13][4];int sets[2][13][4];
      for(int j=0;j<2;j++)for(int z=0;z<13;z++)for(int t=0;t<4;t++){values[j][z][t]=p[j].param[params[z]].val[t];sets[j][z][t]=p[j].param[params[z]].paramSet[t];}
      if(op=="vector")vectorPulsar(p,k(1));else if(op=="driver")get_obsCoord(p,k(1));else tai2ut1(p,k(1));
      for(int j=0;j<2;j++)if(!same(p[j],before[j])){actionActive=false;return 10;}
      for(int j=0;j<12;j++)if(!same(obs[j],prev[j])){actionActive=false;return 11;}
      for(int j=0;j<2;j++)for(int z=0;z<13;z++)for(int t=0;t<4;t++)if(!same(values[j][z][t],p[j].param[params[z]].val[t])||sets[j][z][t]!=p[j].param[params[z]].paramSet[t]){actionActive=false;return 12;}
      delete[]before;r<<"\"other_typed_fields_retained\":true";
    }else if(op=="ut1")r<<"\"value\":"<<number(ut1red(n(1),k(2)));
    else if(op=="lmst"){double v[2]={123,456};double ret=lmst(n(1),n(2),v,v+1);r<<"\"return\":"<<number(ret)<<",\"outputs\":"<<vector(v);}
    else if(op=="precession"){double v[3][3]={{1,2,3},{4,5,6},{7,8,9}};double ret=get_precessionMatrix(v,n(1),n(2),n(3));r<<"\"return\":"<<number(ret)<<",\"matrix\":["<<vector(v[0])<<','<<vector(v[1])<<','<<vector(v[2])<<']';}
    else if(op=="ang")r<<"\"value\":"<<number(ang(k(1),n(2)));
    else if(op=="white"){char b[64];memset(b,'~',sizeof b);strcpy(b,c(1));remove_white(b);r<<"\"buffer\":"<<chars(b,sizeof b,sizeof b);}
    else if(op=="iau"){double trs[3],zen[3],pos[3]={601,602,603},zout[3]={701,702,703},vel[3]={801,802,803};for(int j=0;j<3;j++){trs[j]=n(4+j);zen[j]=n(7+j);}get_obsCoord_IAU2000B(trs,zen,q(1),q(2),pos,zout,vel,c(3));r<<"\"input_position\":"<<vector(trs)<<",\"input_zenith\":"<<vector(zen)<<",\"position\":"<<vector(pos)<<",\"zenith\":"<<vector(zout)<<",\"velocity\":"<<vector(vel);}
    else if(op=="lookup"){auto*o=getObservatory(c(1));r<<"\"owner\":"<<(o-observatories.data())<<",\"coordinates\":"<<coordinates(*o);}
    else if(op=="correction")r<<"\"value\":"<<number(getCorrection(obs+k(1),c(2),c(3),k(4)));
    else if(op=="copy"){std::ifstream src(a[1],std::ios::binary);std::ofstream dst(a[2],std::ios::binary);if(!src||!dst)return 4;dst<<src.rdbuf();}
    else if(op=="remove"){if(remove(c(1)))return 5;}
    else return 7;
    r<<'}';noteAppends(oldObs,oldAliases);
    out<<"{\"event\":"<<eventIndex<<",\"phase\":\"end\",\"operation\":"<<quote(op)<<",\"result\":"<<r.str()<<",\"state\":"<<state()<<",\"caller\":"<<caller(p,obs)<<",\"private\":"<<privateState()<<"}\n";out.flush();
    printf("@END %zu %s\n",eventIndex,op.c_str());fprintf(stderr,"@END %zu %s\n",eventIndex,op.c_str());actionActive=false;eventIndex++;
  }
  return 0;
}
