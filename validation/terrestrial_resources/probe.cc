// Assignment U: unchanged raw source, read-only snapshots of addressable state.
#include <cmath>
#include <cstdint>
#include <cstring>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <limits>
#include "observatory.C"
#include "eop.C"
#include "tabulatedfunction.C"
#include "clkcorr.C"
#include "tropo.C"

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
static std::string table(const MeteorologyFunction&f,size_t nk,size_t hk,size_t sk) {
  std::ostringstream o;
  o<<"{\"filename\":"<<chars(f.table.fileName,nk,256)<<",\"header\":"<<chars(f.table.header_line,hk,256)
   <<",\"site\":"<<chars(f.siteName,sk,256)<<",\"samples\":[";
  for(size_t i=0;i<f.table.samples.size();i++) {if(i)o<<',';auto&s=f.table.samples[i];o<<'['<<number(s.x)<<','<<number(s.y)<<']';}
  o<<"]}";return o.str();
}
static std::string tables(const std::vector<MeteorologyFunction>&fs) {
  std::ostringstream o;o<<'[';size_t nk=0,hk=0,sk=0;
  for(size_t i=0;i<fs.size();i++) {
    auto&f=fs[i];nk=std::max(nk,strlen(f.table.fileName)+1);hk=std::max(hk,strlen(f.table.header_line)+1);sk=std::max(sk,strlen(f.siteName)+1);
    if(i)o<<',';o<<table(f,nk,hk,sk);
  }
  o<<']';return o.str();
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
  o<<"],\"meteorology_initialized\":"<<meteorology_tables_initialized<<",\"wet\":"<<tables(zenithWetDelayTables)<<",\"pressure\":"<<tables(surfaceAtmosphericPressureTables);
  o<<",\"clock_initialized\":"<<(g_clockCorrectionsInitialized?"true":"false")<<",\"clock_functions\":[";
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
static std::string observations(pulsar*p,observation*obs) {
  std::ostringstream o;o<<"{\"pulsars\":[";
  for(int i=0;i<2;i++) {if(i)o<<',';o<<"{\"nobs\":"<<p[i].nobs<<",\"correct\":"<<p[i].correctTroposphere<<",\"warnings\":"<<p[i].noWarnings<<",\"override\":"<<chars(p[i].clockFromOverride,sizeof(p[i].clockFromOverride),sizeof(p[i].clockFromOverride))<<",\"position\":[";for(int k=0;k<3;k++){if(k)o<<',';o<<number(p[i].posPulsarEquatorial[k]);}o<<"]}";}
  o<<"],\"observations\":[";
  for(int i=0;i<12;i++){if(i)o<<',';auto&v=obs[i];o<<"{\"sat_binary128\":"<<quote(bytes(&v.sat,16))<<",\"telID\":"<<chars(v.telID,sizeof(v.telID),sizeof(v.telID))<<",\"delayCorr\":"<<v.delayCorr<<",\"delay\":"<<number(v.troposphericDelay)<<",\"zenith\":[";for(int k=0;k<3;k++){if(k)o<<',';o<<number(v.zenith[k]);}o<<"]}";}
  o<<"]}";return o.str();
}
static std::string decode(const std::string&s){std::string v;for(size_t i=1;i<s.size();i+=2)v+=char(std::stoi(s.substr(i,2),nullptr,16));return v;}
static std::ofstream* exitOutput=nullptr;
static std::vector<EOPSample>* exitSamples=nullptr;
static MeteorologyFunction* exitDirect=nullptr;
static bool actionActive=false;
static size_t exitEvent=0,exitOldObs=0,exitOldAliases=0;
static std::string exitOperation;
static void observeExit() {
  if(!actionActive)return;
  // exit() skips automatic destruction. This callback runs before static
  // vector destruction and reads only live caller/global initialized storage.
  noteAppends(exitOldObs,exitOldAliases);
  auto&o=*exitOutput;
  o<<"{\"event\":"<<exitEvent<<",\"phase\":\"exit\",\"operation\":"<<quote(exitOperation)
   <<",\"state\":"<<state()<<",\"caller_eop_samples\":[";
  for(size_t j=0;j<exitSamples->size();j++){
    if(j)o<<',';auto&s=(*exitSamples)[j];o<<'['<<number(s.mjd)<<','<<number(s.xp)<<','<<number(s.yp)<<','<<number(s.dut1)<<']';
  }
  o<<"],\"caller_meteorology\":"<<table(*exitDirect,256,256,256)<<"}\n";o.flush();
}

int main(int argc,char**argv) {
  if(argc!=3)return 2;
  setvbuf(stdout,nullptr,_IONBF,0);setvbuf(stderr,nullptr,_IONBF,0);
  std::ifstream input(argv[1]);std::ofstream out(argv[2]);std::string line;
  auto*p=new pulsar[2]();auto*obs=new observation[12]();p[0].obsn=obs;p[1].obsn=obs+6;
  MeteorologyFunction direct;memset(direct.table.fileName,'~',256);memset(direct.table.header_line,'~',256);memset(direct.siteName,'~',256);
  std::vector<EOPSample> samples;size_t event=0;
  exitOutput=&out;exitSamples=&samples;exitDirect=&direct;atexit(observeExit);
  while(std::getline(input,line)) {
    std::vector<std::string>a;std::stringstream in(line);std::string word;
    while(std::getline(in,word,'\t'))a.push_back(decode(word));
    auto n=[&](size_t i){return strtod(a.at(i).c_str(),nullptr);};auto c=[&](size_t i){return const_cast<char*>(a.at(i).c_str());};auto k=[&](size_t i){return std::stoi(a.at(i));};
    const auto&op=a.at(0);size_t oldObs=observatories.size(),oldAliases=observatoryAliasLists.size();
    actionActive=true;exitEvent=event;exitOperation=op;exitOldObs=oldObs;exitOldAliases=oldAliases;
    printf("@BEGIN %zu %s\n",event,op.c_str());fprintf(stderr,"@BEGIN %zu %s\n",event,op.c_str());
    out<<"{\"event\":"<<event<<",\"phase\":\"begin\",\"operation\":"<<quote(op)<<"}\n";out.flush();
    std::ostringstream r;r<<'{';
    if(op=="profile")r<<"\"double_digits\":"<<std::numeric_limits<double>::digits<<",\"longdouble_size\":"<<sizeof(longdouble)<<",\"longdouble_digits\":"<<FLT128_MANT_DIG<<",\"observation_size\":"<<sizeof(observation)<<",\"pulsar_size\":"<<sizeof(pulsar)<<",\"glob_nomatch\":"<<GLOB_NOMATCH;
    else if(op=="env"){setenv(a[1].c_str(),a[2].c_str(),1);}
    else if(op=="unset")unsetenv(a[1].c_str());
    else if(op=="root_key")strcpy(TEMPO2_ENVIRON,c(1));
    else if(op=="layout"){nextGroups.clear();for(size_t j=1;j<a.size();j++)nextGroups.push_back(k(j));}
    else if(op=="debug"){debugFlag=k(1);displayCVSversion=k(2);quietFlag=k(3);}
    else if(op=="read_obs")readObservatoryFile(c(1));
    else if(op=="read_alias")readAliases(c(1));
    else if(op=="init_obs")initObservatories();
    else if(op=="alias"){
      char source[1024],dest[1024];memset(source,'~',sizeof source);memset(dest,'!',sizeof dest);strcpy(source,c(1));
      lookup_observatory_alias(source,k(2)?source:dest);
      r<<"\"input\":"<<chars(source,1024,1024)<<",\"output\":"<<chars(k(2)?source:dest,1024,1024);
    }else if(op=="lookup"){
      auto*site=getObservatory(c(1));r<<"\"index\":"<<(site-observatories.data())<<",\"coordinates\":"<<coordinates(*site)<<",\"name\":"<<quote(site->name);
    }else if(op=="fang")r<<"\"value\":"<<number(fang(k(1),n(2)));
    else if(op=="geometry"){
      observatory v={};strcpy(v.name,"geometry");strcpy(v.code,"G");strcpy(v.clock_name,"UTC");
      if(a[1]=="forward"){v.latitude_grs80=n(2);v.longitude_grs80=n(3);v.height_grs80=n(4);GRS80_to_ITRF(&v);}
      else {v.x=n(2);v.y=n(3);v.z=n(4);ITRF_to_GRS80(&v);}
      r<<"\"first\":"<<coordinates(v);
      if(a[1]=="forward")ITRF_to_GRS80(&v);else GRS80_to_ITRF(&v);
      r<<",\"roundtrip\":"<<coordinates(v);
    }else if(op=="eop_seed")samples.push_back({n(1),n(2),n(3),n(4)});
    else if(op=="load_eop"){
      load_EOP(samples,c(1));r<<"\"samples\":[";for(size_t j=0;j<samples.size();j++){if(j)r<<',';auto&s=samples[j];r<<'['<<number(s.mjd)<<','<<number(s.xp)<<','<<number(s.yp)<<','<<number(s.dut1)<<']';}r<<']';
    }else if(op=="eop"){
      double v[4]={901,902,903,904};int mask=k(4);
      get_EOP(n(1),v,v+(mask==1?0:1),v+(mask?0:2),v+(mask==2?0:3),k(2),c(3));
      r<<"\"outputs\":[";for(int j=0;j<4;j++){if(j)r<<',';r<<number(v[j]);}r<<']';
    }else if(op=="met_load"){
      MeteorologyFunction_load(&direct,c(1));r<<"\"table\":"<<table(direct,256,256,256);
    }else if(op=="met_value")r<<"\"value\":"<<number(MeteorologyFunction_getValue(&direct,n(1)));
    else if(op=="met_bounds")r<<"\"start\":"<<number(MeteorologyFunction_getStartMJD(&direct))<<",\"end\":"<<number(MeteorologyFunction_getEndMJD(&direct));
    else if(op=="met_init")initialize_meteorology_tables(k(1));
    else if(op=="met_kind")initialize_meteorology_table(k(1),"atmosphere",c(2),a[2]=="zwd"?&zenithWetDelayTables:&surfaceAtmosphericPressureTables,c(2));
    else if(op=="met")r<<"\"value\":"<<number(getMeteorologicalValue(a[1]=="zwd"?zenithWetDelayTables:surfaceAtmosphericPressureTables,c(2),n(3),k(4)));
    else if(op=="wet")r<<"\"value\":"<<number(getZenithWetDelay(c(1),n(2),k(3)));
    else if(op=="pressure")r<<"\"value\":"<<number(getSurfaceAtmosphericPressure(c(1),n(2),k(3)));
    else if(op=="nmf")r<<"\"hydrostatic\":"<<number(NMF_hydrostatic(n(1),n(2),n(3),n(4)))<<",\"wet\":"<<number(NMF_wet(n(2),n(4)));
    else if(op=="pulsar"){
      auto&v=p[k(1)];v.nobs=k(2);v.correctTroposphere=k(3);v.noWarnings=k(4);strcpy(v.clockFromOverride,c(5));for(int j=0;j<3;j++)v.posPulsarEquatorial[j]=n(6+j);
      r<<"\"state\":"<<observations(p,obs);
    }else if(op=="observation"){
      auto&v=obs[k(1)];v.sat=parse_longdouble(c(2));strcpy(v.telID,c(3));v.delayCorr=k(4);v.troposphericDelay=n(5);for(int j=0;j<3;j++)v.zenith[j]=n(6+j);
      r<<"\"state\":"<<observations(p,obs);
    }else if(op=="compute"){
      auto*before=new pulsar[2]{p[0],p[1]};std::vector<observation> old(obs,obs+12);
      r<<"\"before\":"<<observations(p,obs);compute_tropospheric_delays(p,k(1));
      for(int j=0;j<2;j++)if(!same(p[j],before[j])){actionActive=false;return 10;}
      for(int j=0;j<12;j++)if(!same(obs[j],old[j])){actionActive=false;return 11;}
      delete[]before;r<<",\"after\":"<<observations(p,obs)<<",\"all_other_typed_fields_retained\":true";
    }else if(op=="predict"){
      // Supporting helper consistency only; compute_tropospheric_delays remains
      // the oracle. Run this in its own history to avoid preconditioning caches.
      int ip=k(1),io=k(2);auto&v=p[ip].obsn[io];auto*site=getObservatory(v.telID);
      double elevation=asin(dotproduct(v.zenith,p[ip].posPulsarEquatorial)/site->height_grs80);
      double pressure=getSurfaceAtmosphericPressure(site->code,v.sat,p[ip].noWarnings);
      double correction=getCorrection(&v,p[ip].clockFromOverride,"UTC",p[ip].noWarnings);
      double utc=v.sat+correction/SECDAY;
      double zhd=0.02268*pressure/(SPEED_LIGHT*(1.0-0.00266*cos(site->latitude_grs80)-2.8e-7*site->height_grs80));
      double mh=NMF_hydrostatic(utc,site->latitude_grs80,site->height_grs80,elevation);
      double wet=getZenithWetDelay(site->code,v.sat,p[ip].noWarnings);
      double mw=NMF_wet(site->latitude_grs80,elevation);
      r<<"\"elevation\":"<<number(elevation)<<",\"utc_mjd\":"<<number(utc)<<",\"correction\":"<<number(correction)<<",\"pressure\":"<<number(pressure)<<",\"wet\":"<<number(wet)<<",\"hydrostatic_mapping\":"<<number(mh)<<",\"wet_mapping\":"<<number(mw)<<",\"delay\":"<<number(zhd*mh+wet*mw);
    }else if(op=="correction")r<<"\"value\":"<<number(getCorrection(obs+k(1),c(2),"UTC",k(3)));
    else if(op=="copy"){
      std::ifstream src(a[1],std::ios::binary);std::ofstream dst(a[2],std::ios::binary);if(!src||!dst){actionActive=false;return 4;}dst<<src.rdbuf();
    }else if(op=="remove"){if(remove(c(1))){actionActive=false;return 5;}}
    else if(op=="glob_nomatch"){
      glob_t g;int status=glob(c(1),0,nullptr,&g);
      r<<"\"status\":"<<status<<",\"path_count\":"<<g.gl_pathc<<",\"pathv_null\":"<<(g.gl_pathv==nullptr?"true":"false");globfree(&g);
    }else {actionActive=false;return 7;}
    noteAppends(oldObs,oldAliases);r<<'}';
    out<<"{\"event\":"<<event<<",\"phase\":\"end\",\"operation\":"<<quote(op)<<",\"result\":"<<r.str()<<",\"state\":"<<state()<<"}\n";out.flush();
    actionActive=false;
    printf("@END %zu %s\n",event,op.c_str());fprintf(stderr,"@END %zu %s\n",event,op.c_str());event++;
  }
  delete[]obs;delete[]p;return 0;
}
