// Assignment J: raw primary sources, included to inspect initialized private state.
// Controlled inputs only; no source initializer or cache reset is called.
#include <cmath>
#include <cstdint>
#include <cstring>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <vector>
#include <limits>
// Rename only the table value symbol; the forwarding boundary records actual
// binary64 arguments/results without changing the raw source arithmetic.
#define TabulatedFunction_getValue J_Table_getValue
#include "tabulatedfunction.C"
#undef TabulatedFunction_getValue
struct TableQuery { std::string name; double epoch; double value; };
static std::vector<TableQuery> tableQueries;
double TabulatedFunction_getValue(TabulatedFunction *table, double epoch) {
  const double value=J_Table_getValue(table,epoch);
  tableQueries.push_back({table->fileName,epoch,value});
  return value;
}
#include "clkcorr.C"
#include "toa2utc.C"

static std::string quoted(const std::string &s) {
  std::ostringstream o; o << '"';
  for (unsigned char c : s) {
    if(c=='"' || c=='\\') o << '\\' << c;
    else if(c<32) o << "\\u" << std::hex << std::setw(4) << std::setfill('0') << int(c);
    else o << c;
  }
  o << '"'; return o.str();
}
static std::string hexbytes(const void *p, size_t n) {
  std::ostringstream o;
  for (size_t i=0;i<n;++i) o << std::hex << std::setw(2) << std::setfill('0') << unsigned(((const unsigned char*)p)[i]);
  return o.str();
}
static std::string number(double x) {
  std::ostringstream o;
  const char *kind = std::isnan(x)?"NaN":std::isinf(x)?(x>0?"+Infinity":"-Infinity"):"finite";
  o << "{\"class\":" << quoted(kind);
  if(std::isfinite(x)) { uint64_t bits; memcpy(&bits,&x,8);
    o << ",\"decimal\":\"" << std::setprecision(17) << x << "\",\"bits\":\""
      << std::hex << std::setw(16) << std::setfill('0') << bits << '"'; }
  o << '}'; return o.str();
}
static std::string weight(float x) {
  std::ostringstream o; o << "{\"value\":" << number(x);
  if(std::isfinite(x)) {uint32_t bits; memcpy(&bits,&x,4);o << ",\"binary32\":\"" << std::hex << std::setw(8) << std::setfill('0') << bits << '"';}
  o << '}';return o.str();
}
static std::string extended(longdouble x) {
#ifdef LONGDOUBLE_IS_FLOAT128
  char buf[160]; quadmath_snprintf(buf,sizeof(buf),"%.36Qg",x); return quoted(buf);
#else
  std::ostringstream o;o<<std::setprecision(21)<<x;return quoted(o.str());
#endif
}
static void indices(std::ostream &o,const std::vector<size_t>& v) {
  o << '[';for(size_t i=0;i<v.size();++i){if(i)o<<',';o<<v[i];}o<<']';
}
static std::string clockState() {
  std::ostringstream o;
  o << "{\"initialized\":" << (g_clockCorrectionsInitialized?"true":"false") << ",\"functions\":[";
  for(size_t i=0;i<g_clockCorrectionFunctions.size();++i){
    const auto &f=g_clockCorrectionFunctions[i];if(i)o<<',';
    o << "{\"path\":"<<quoted(f.fullPath)<<",\"name\":"<<quoted(f.fileName)<<",\"from\":"<<quoted(f.clockFrom)
      <<",\"to\":"<<quoted(f.clockTo)<<",\"badness\":"<<weight(f.badness)<<",\"loaded\":"<<(f.loaded?"true":"false")
      <<",\"start\":"<<number(f.startMJD)<<",\"end\":"<<number(f.endMJD);
    if(f.loaded)o<<",\"samples\":"<<f.table.samples.size()<<",\"header\":"<<quoted(f.table.header_line);
    o<<'}';
  }
  o<<"],\"sequences\":[";
  for(size_t i=0;i<g_clockCorrectionSequences.size();++i){const auto &s=g_clockCorrectionSequences[i];if(i)o<<',';
    o<<"{\"from\":"<<quoted(s.clockFrom)<<",\"to\":"<<quoted(s.clockTo)<<",\"from_key\":"<<quoted(s.clockFromKey)
     <<",\"to_key\":"<<quoted(s.clockToKey)<<",\"start\":"<<number(s.startMJD)<<",\"end\":"<<number(s.endMJD)<<",\"indices\":";
    indices(o,s.functionIndices);o<<'}';}
  o<<"],\"warnings\":"<<TK_warnCount<<",\"errors\":"<<TK_errorCount<<'}';return o.str();
}
static std::string tableState(TabulatedFunction &t) {
  std::ostringstream o;o<<"{\"name\":"<<quoted(t.fileName)<<",\"header\":"<<quoted(t.header_line)
    <<",\"start\":"<<number(TabulatedFunction_getStartX(&t))<<",\"end\":"<<number(TabulatedFunction_getEndX(&t))<<",\"samples\":[";
  for(size_t i=0;i<t.samples.size();++i){if(i)o<<',';o<<'['<<number(t.samples[i].x)<<','<<number(t.samples[i].y)<<']';}
  o<<"]}";return o.str();
}
static std::string obsState(const observation &o,const observation *before=nullptr) {
  std::ostringstream s;s<<"{\"sat\":"<<extended(o.sat)<<",\"sat_binary64\":"<<number((double)o.sat)
    <<",\"telID\":"<<quoted(o.telID)<<",\"clockCorr\":"<<o.clockCorr<<",\"count\":"<<o.nclock_correction<<",\"slots\":[";
  for(int i=0;i<MAX_CLK_CORR;++i){if(i)s<<',';
    s<<"{\"value\":"<<number(o.correctionsTT[i].correction)<<",\"destination_bytes\":"<<quoted(hexbytes(o.correctionsTT[i].corrects_to,32));
    if(before){std::string mask;const auto *a=(const unsigned char*)&o.correctionsTT[i];const auto*b=(const unsigned char*)&before->correctionsTT[i];
      for(size_t j=0;j<sizeof(clock_correction);++j)mask+=a[j]==b[j]?'0':'1';s<<",\"changed_bytes\":"<<quoted(mask);}
    s<<'}';}
  s<<"]}";return s.str();
}
static void setupObs(observation &o,const std::vector<std::string>& a) {
  memset(&o,0,sizeof(o));o.sat=parse_longdouble(a.at(2).c_str());strcpy(o.telID,a.at(3).c_str());o.clockCorr=std::stoi(a.at(4));o.nclock_correction=std::stoi(a.at(5));
  for(int i=0;i<MAX_CLK_CORR;++i){o.correctionsTT[i].correction=900+i;memset(o.correctionsTT[i].corrects_to,'~',32);snprintf(o.correctionsTT[i].corrects_to,31,"sentinel_%02d",i);}
}
static std::string decode(const std::string& s){std::string out;for(size_t i=1;i<s.size();i+=2)out+=(char)std::stoi(s.substr(i,2),nullptr,16);return out;}
static const char *nullable(const std::string& s){return s=="@null"?nullptr:s.c_str();}

int main(int argc,char **argv){
  if(argc!=3)return 2;
  setvbuf(stdout,nullptr,_IONBF,0);setvbuf(stderr,nullptr,_IONBF,0);
  std::ifstream commands(argv[1]);std::ofstream output(argv[2]);
  auto *psr=new pulsar[2]();auto *obs=new observation[6]();
  psr[0].obsn=obs;psr[1].obsn=obs+3;
  TabulatedFunction table;
  std::string line;size_t event=0;
  while(std::getline(commands,line)){
    std::vector<std::string>a;std::stringstream in(line);std::string word;
    while(std::getline(in,word,'\t'))a.push_back(decode(word));
    const auto &op=a.at(0);
    tableQueries.clear();
    printf("@BEGIN %zu %s\n",event,op.c_str());fprintf(stderr,"@BEGIN %zu %s\n",event,op.c_str());
    output<<"{\"event\":"<<event<<",\"phase\":\"begin\",\"operation\":"<<quoted(op)<<"}\n";output.flush();
    std::ostringstream result;result<<'{';
    if(op=="profile")result<<"\"float_digits\":"<<std::numeric_limits<float>::digits<<",\"double_digits\":"<<std::numeric_limits<double>::digits<<",\"longdouble_size\":"<<sizeof(longdouble)<<",\"longdouble_digits\":"<<FLT128_MANT_DIG<<",\"clock_capacity\":"<<MAX_CLK_CORR<<",\"table_size\":"<<sizeof(table)<<",\"observation_size\":"<<sizeof(observation);
    else if(op=="env"){
      if(a[1]=="@unset")unsetenv("TEMPO2");else setenv("TEMPO2",a[1].c_str(),1);
      strcpy(tempo2_clock_path,a[2].c_str());
    }else if(op=="index")ensureClockCorrectionsInitialized(std::stoi(a[1]));
    else if(op=="index_file")addIndexedClockFile(a[1],std::stoi(a[2]));
    else if(op=="obs"){setupObs(obs[std::stoi(a[1])],a);result<<"\"observation\":"<<obsState(obs[std::stoi(a[1])]);}
    else if(op=="slot"){auto&o=obs[std::stoi(a[1])];int k=std::stoi(a[2]);o.correctionsTT[k].correction=strtod(a[3].c_str(),nullptr);strcpy(o.correctionsTT[k].corrects_to,a[4].c_str());}
    else if(op=="load"){TabulatedFunction_load(&table,const_cast<char*>(a[1].c_str()));result<<"\"table\":"<<tableState(table);}
    else if(op=="value")result<<"\"query\":"<<number(strtod(a[1].c_str(),nullptr))<<",\"value\":"<<number(TabulatedFunction_getValue(&table,strtod(a[1].c_str(),nullptr)));
    else if(op=="empty_value")result<<"\"value\":"<<number(TabulatedFunction_getValue(&table,1));
    else if(op=="empty_start")result<<"\"value\":"<<number(TabulatedFunction_getStartX(&table));
    else if(op=="empty_end")result<<"\"value\":"<<number(TabulatedFunction_getEndX(&table));
    else if(op=="define")defineClockCorrectionSequence(const_cast<char*>(nullable(a[1])),std::stoi(a[2]));
    else if(op=="clock"||op=="correction"||op=="tt"){
      auto&o=obs[std::stoi(a[1])];observation before=o;
      if(op=="clock")getClockCorrections(&o,nullable(a[2]),nullable(a[3]),std::stoi(a[4]));
      if(op=="correction")result<<"\"value\":"<<number(getCorrection(&o,nullable(a[2]),nullable(a[3]),std::stoi(a[4])))<<',';
      if(op=="tt")result<<"\"value\":"<<number(getCorrectionTT(&o))<<',';
      result<<"\"before\":"<<obsState(before)<<",\"after\":"<<obsState(o,&before);
    }else if(op=="path"){
      ensureClockCorrectionsInitialized(0);auto p=buildDirectedPath(a[1],a[2],strtod(a[3].c_str(),nullptr));
      result<<"\"source_present\":"<<(p.sourcePresent?"true":"false")<<",\"target_present\":"<<(p.targetPresent?"true":"false")<<",\"reverse_hint\":"<<(p.reverseHint?"true":"false")<<",\"indices\":";indices(result,p.functionIndices);
    }else if(op=="helper_fill"||op=="helper_accumulate"){
      auto&o=obs[std::stoi(a[1])];observation before=o;double total=strtod(a[5].c_str(),nullptr);auto*s=&g_clockCorrectionSequences.at(std::stoi(a[2]));
      bool ok=op=="helper_fill"?fillCorrectionsFromSequence(&o,s,a[4],a[3],&total):accumulateCorrectionFromSequence(s,&o,a[4],a[3],&total);
      result<<"\"ok\":"<<(ok?"true":"false")<<",\"value\":"<<number(total)<<",\"before\":"<<obsState(before)<<",\"after\":"<<obsState(o,&before);
    }else if(op=="pulsar"){
      auto&p=psr[std::stoi(a[1])];p.nobs=std::stoi(a[2]);p.nJumps=std::stoi(a[3]);strcpy(p.clockFromOverride,a[4].c_str());strcpy(p.clock,a[5].c_str());p.noWarnings=std::stoi(a[6]);
    }else if(op=="jump"){
      auto&p=psr[std::stoi(a[1])];int k=std::stoi(a[2]);p.jumpVal[k]=strtod(a[3].c_str(),nullptr);p.jumpSAT[k]=std::stoi(a[4]);strcpy(p.jumpStr[k],a[5].c_str());
    }else if(op=="members"){
      auto&o=obs[std::stoi(a[1])];o.obsNjump=(int)a.size()-2;for(size_t i=2;i<a.size();++i)o.jump[i-2]=std::stoi(a[i]);
    }else if(op=="toa"){
      std::vector<observation>before(obs,obs+6);toa2utc(psr,std::stoi(a[1]));result<<"\"observations\":[";bool comma=false;
      for(int p=0;p<std::stoi(a[1]);++p)for(int i=0;i<psr[p].nobs;++i){int k=p*3+i;if(comma)result<<',';comma=true;result<<"{\"index\":"<<k<<",\"before\":"<<obsState(before[k])<<",\"after\":"<<obsState(obs[k],&before[k])<<'}';}result<<']';
    }else if(op=="copy"){
      std::ifstream source(a[1],std::ios::binary);std::ofstream target(a[2],std::ios::binary);if(!source||!target)return 4;target<<source.rdbuf();
    }else if(op=="replace"){if(rename(a[1].c_str(),a[2].c_str()))return 5;}
    else if(op=="remove"){if(remove(a[1].c_str()))return 6;}
    else if(op=="debug"){debugFlag=std::stoi(a[1]);displayCVSversion=std::stoi(a[2]);}
    else return 7;
    result<<'}';
    output<<"{\"event\":"<<event<<",\"phase\":\"end\",\"operation\":"<<quoted(op)<<",\"result\":"<<result.str()<<",\"clock_state\":"<<clockState()<<",\"table_queries\":[";
    for(size_t q=0;q<tableQueries.size();++q){if(q)output<<',';const auto &t=tableQueries[q];output<<"{\"name\":"<<quoted(t.name)<<",\"epoch\":"<<number(t.epoch)<<",\"value\":"<<number(t.value)<<'}';}
    output<<"]}\n";output.flush();
    printf("@END %zu %s\n",event,op.c_str());fprintf(stderr,"@END %zu %s\n",event,op.c_str());++event;
  }
  delete[]obs;delete[]psr;return 0;
}
