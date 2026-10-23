// Assignment K native caller. Canonical routines are compiled separately without edits.
// All borrowed strings/FILE handles live through each synchronous call. Only explicit
// controlled inputs are read as numeric snapshots; natural prefit storage is never read.
#include "tempo2.h"
#include "tempo2Util.h"
#include "names.h"
#include <fstream>
#include <sstream>
#include <map>
#include <vector>
#include <string>
#include <cstring>
#include <cstdlib>
#include <cstdint>
#include <type_traits>
#include <cassert>
#include <cmath>

extern int readValue(pulsar*,char*,FILE*,parameter*,int);
extern void checkLine(pulsar*,char*,FILE*,parameter*,parameter*);
extern void checkAllSet(pulsar*,parameter,parameter,char*);
extern int setupParameterFileDefaults(pulsar*);
extern void getValue(char*,int,int,pulsar*,int,int);
extern void removeCR(char*);
extern void allocateMemory(pulsar*,int);
extern void destroyMemory(pulsar*);

using State=std::map<std::string,std::string>;
static std::ofstream events;
static std::string q(const std::string &s) {
    std::ostringstream o; o<<'"';
    for(unsigned char c:s) {
        if(c=='"'||c=='\\')o<<'\\'<<c;
        else if(c<32){char b[8];sprintf(b,"\\u%04x",c);o<<b;}
        else o<<c;
    }
    o<<'"';return o.str();
}
static std::string num(longdouble v) {
    if(isnanq(v))return "{\"type\":\"binary128\",\"class\":\"NaN\"}";
    if(isinfq(v))return std::string("{\"type\":\"binary128\",\"class\":\"")+(signbitq(v)?"-Infinity":"+Infinity")+"\"}";
    char b[128];quadmath_snprintf(b,sizeof(b),"%.36Qg",v);
    assert(strtoflt128(b,nullptr)==v && !!signbitq(strtoflt128(b,nullptr))==!!signbitq(v));
    return "{\"type\":\"binary128\",\"class\":\"finite\",\"value\":"+q(b)+"}";
}
static std::string num(double v) {
    uint64_t bits;memcpy(&bits,&v,8);char b[40];sprintf(b,"%016llx",(unsigned long long)bits);
    return "{\"type\":\"binary64\",\"class\":"+q(std::isnan(v)?"NaN":std::isinf(v)?(std::signbit(v)?"-Infinity":"+Infinity"):"finite")+",\"bits\":"+q(b)+"}";
}
static std::string num(float v) {
    uint32_t bits;memcpy(&bits,&v,4);char b[24];sprintf(b,"%08x",bits);
    return "{\"type\":\"binary32\",\"class\":"+q(std::isnan(v)?"NaN":std::isinf(v)?(std::signbit(v)?"-Infinity":"+Infinity"):"finite")+",\"bits\":"+q(b)+"}";
}
static std::string num(const char *s){return q(s);}
template<class T> static typename std::enable_if<std::is_integral<T>::value||std::is_enum<T>::value,std::string>::type num(T v){return std::to_string((long long)v);}
static std::string object(const State &s){std::string o="{";bool first=true;for(auto &kv:s){if(!first)o+=",";first=false;o+=q(kv.first)+":"+kv.second;}return o+"}";}
static std::string delta(const State &a,const State &b){
    State changes;
    for(auto &kv:b){auto old=a.find(kv.first);if(old==a.end()||old->second!=kv.second)changes[kv.first]="{\"before\":"+(old==a.end()?"null":old->second)+",\"after\":"+kv.second+"}";}
    for(auto &kv:a)if(!b.count(kv.first))changes[kv.first]="{\"before\":"+kv.second+",\"after\":null}";
    return object(changes);
}
static std::string pname(int i){return i<(int)(sizeof(names)/sizeof(*names))?names[i]:"unnamed_"+std::to_string(i);}
static int index(const std::string &s){for(int i=0;i<(int)(sizeof(names)/sizeof(*names));i++)if(s==names[i])return i;abort();}
static State controls(pulsar &p,bool controlled){State s;
#define FIELD(f) s[#f]=num(p.f)
#include "controls.h"
    if(controlled){FIELD(clock);FIELD(ephemeris);FIELD(JPL_EPHEMERIS);FIELD(rajStrPre);FIELD(rajStrPost);FIELD(decjStrPre);FIELD(decjStrPost);FIELD(ne_sw_ifuncN);FIELD(nchromx);FIELD(clockFromOverride);FIELD(waveScale);FIELD(setTelVelX);FIELD(setTelVelY);FIELD(setTelVelZ);}
#undef FIELD
    return s;
}
static State params(parameter *ps,int count,bool natural){State s;
    for(int i=0;i<count;i++){
        parameter &p=ps[i];std::string root=pname(i)+".";
        s[root+"aSize"]=num(p.aSize);s[root+"nLinkTo"]=num(p.nLinkTo);s[root+"nLinkFrom"]=num(p.nLinkFrom);
        for(int j=0;j<p.nLinkTo;j++)s[root+"linkTo["+std::to_string(j)+"]"]=num(p.linkTo[j]);
        for(int j=0;j<p.nLinkFrom;j++)s[root+"linkFrom["+std::to_string(j)+"]"]=num(p.linkFrom[j]);
        for(int j=0;j<p.aSize;j++){
            std::string k=root+std::to_string(j)+".";
            s[k+"val"]=num(p.val[j]);s[k+"err"]=num(p.err[j]);s[k+"fitFlag"]=num(p.fitFlag[j]);s[k+"paramSet"]=num(p.paramSet[j]);
            s[k+"label"]=q(p.label[j]);s[k+"shortlabel"]=q(p.shortlabel[j]);
            if(!natural){s[k+"prefit"]=num(p.prefit[j]);s[k+"prefitErr"]=num(p.prefitErr[j]);}
        }
    }return s;
}
static State snapshot(pulsar *p,int n){State s;s["global.TK_errorCount"]=num(TK_errorCount);s["global.TK_warnCount"]=num(TK_warnCount);
    for(int i=0;i<n;i++){
        State one=params(p[i].param,sizeof(names)/sizeof(*names),false), c=controls(p[i],true);one.insert(c.begin(),c.end());
        for(auto &kv:one)s[std::to_string(i)+"."+kv.first]=kv.second;
    }return s;
}
static FILE *stream(const std::string &s){FILE*f=tmpfile();assert(f);assert(fwrite(s.data(),1,s.size(),f)==s.size());rewind(f);return f;}
static std::string next(FILE*f){long pos=ftell(f);char s[1000];int n=fscanf(f,"%999s",s);fseek(f,pos,SEEK_SET);return n==1?s:"<EOF>";}
static void seed(pulsar &p){
    for(int i=0;i<MAX_PARAMS;i++)for(int j=0;j<p.param[i].aSize;j++){p.param[i].prefit[j]=17;p.param[i].prefitErr[j]=19;}
    strcpy(p.clock,"CALLER_CLOCK");strcpy(p.ephemeris,"CALLER_EPHEM");strcpy(p.JPL_EPHEMERIS,"CALLER_PATH");
    strcpy(p.rajStrPre,"CALLER_RA_PRE");strcpy(p.rajStrPost,"CALLER_RA_POST");strcpy(p.decjStrPre,"CALLER_DEC_PRE");strcpy(p.decjStrPost,"CALLER_DEC_POST");
    p.ne_sw_ifuncN=117;p.nchromx=0;p.waveScale=23;p.setTelVelX=7;p.setTelVelY=7;p.setTelVelZ=7;
}
static std::string decode(const std::string &s){assert(s[0]=='x');std::string r;for(size_t i=1;i<s.size();i+=2)r+=(char)strtol(s.substr(i,2).c_str(),nullptr,16);return r;}
int main(int argc,char **argv){
    assert(argc==4);events.open(argv[2]);assert(events);setvbuf(stdout,nullptr,_IONBF,0);setvbuf(stderr,nullptr,_IONBF,0);
    events<<"{\"phase\":\"launch\",\"nonce\":"<<q(argv[3])<<"}"<<std::endl;
    MAX_PSR=2;MAX_OBSN=2;
    pulsar *ps=(pulsar*)malloc(sizeof(pulsar)*2);assert(ps);
    bool ready=false;int nps=0;
    // Ecliptic scratch is caller-owned with explicit initialized fields. The wrappers
    // use their own natural scratch allocations; these must not be confused.
    longdouble ev[2]={0,0},ee[2]={0,0},ep[2]={17,17},epe[2]={19,19};int ef[2]={0,0},es[2]={0,0};
    char el[2][100]={{0},{0}},esh[2][100]={{0},{0}};char *elp[2]={el[0],el[1]},*esp[2]={esh[0],esh[1]};parameter e[2]{};
    for(int i=0;i<2;i++){e[i].aSize=1;e[i].val=ev+i;e[i].err=ee+i;e[i].prefit=ep+i;e[i].prefitErr=epe+i;e[i].fitFlag=ef+i;e[i].paramSet=es+i;e[i].label=elp+i;e[i].shortlabel=esp+i;}
    std::ifstream input(argv[1]);std::string line;int action=0;
    while(std::getline(input,line)){
        std::vector<std::string>a;std::istringstream row(line);std::string part;while(std::getline(row,part,'\t'))a.push_back(decode(part));assert(!a.empty());
        events<<"{\"action\":"<<action<<",\"phase\":\"begin\",\"op\":"<<q(a[0])<<"}"<<std::endl;
        State before=ready?snapshot(ps,nps):State();std::string result="{}";int rc=0;
        if(a[0]=="init"){
            assert(!ready);nps=stoi(a[1]);assert(nps>=1&&nps<=2);int full=stoi(a[2]),warn=stoi(a[3]);
            for(int i=0;i<nps;i++){ps[i].ne_sw_ifuncN=117;ps[i].nchromx=119;}
            if(a.size()>4&&a[4]=="all")initialise(ps,warn);
            else for(int i=0;i<nps;i++)initialiseOne(ps+i,warn,full);
            State c=controls(ps[0],false);c["caller_ne_sw_ifuncN_after"]=num(ps[0].ne_sw_ifuncN);c["caller_nchromx_after"]=num(ps[0].nchromx);
            result="{\"natural_controls\":"+object(c)+",\"MAX_PSR\":2,\"MAX_OBSN\":2,\"MAX_PARAMS\":"+num(MAX_PARAMS)+",\"MAX_FIT\":"+num(MAX_FIT)+",\"sizeof_pulsar\":"+num(sizeof(pulsar))+",\"longdouble_bytes\":"+num(sizeof(longdouble))+"}";
            for(int i=0;i<nps;i++)seed(ps[i]);ready=true;before=snapshot(ps,nps);
        }else if(a[0]=="registry"){
            assert(ready);result=object(params(ps[0].param,MAX_PARAMS,true));
        }else if(a[0]=="allocate"){
            assert(!ready);allocateMemory(ps,stoi(a[1]));result=object(params(ps->param,MAX_PARAMS,true));destroyMemory(ps);
        }else if(a[0]=="parse")result=num(parse_longdouble(a[1].c_str()));
        else if(a[0]=="coord"){
            char b[1000];strcpy(b,a[2].c_str());double v=a[1]=="hms"?hms_turn(b):dms_turn(b);
            result="{\"turn\":"+num(v)+",\"degree\":"+num(turn_deg(v))+",\"input_after\":"+q(b)+"}";
        }else if(a[0]=="remove"){
            assert(!a[1].empty());char b[1000];strcpy(b,a[1].c_str());removeCR(b);result=q(b);
        }else if(a[0]=="prior"){
            parameter &p=ps[0].param[index(a[1])];int j=stoi(a[2]);assert(j>=0&&j<p.aSize);
            p.val[j]=parse_longdouble(a[3].c_str());p.prefit[j]=parse_longdouble(a[4].c_str());p.err[j]=parse_longdouble(a[5].c_str());p.prefitErr[j]=parse_longdouble(a[6].c_str());p.fitFlag[j]=stoi(a[7]);p.paramSet[j]=stoi(a[8]);
        }else if(a[0]=="read"){
            FILE*f=stream(a[4]);int j=stoi(a[3]);parameter &p=ps[0].param[index(a[2])];assert(j>=0&&j<p.aSize);
            rc=readValue(ps,const_cast<char*>(a[1].c_str()),f,&p,j);
            result="{\"status\":"+num(rc)+",\"position\":"+num(ftell(f))+",\"next\":"+q(next(f))+"}";fclose(f);
        }else if(a[0]=="stream"){
            FILE*f=stream(a[1]);char b[1000];int token=0;std::string arr="[";
            while(fscanf(f,"%999s",b)==1){std::string key=b;long pos=ftell(f);State old=snapshot(ps,nps);State eb=params(e,2,false);
                checkLine(ps,b,f,e,e+1);
                if(token++)arr+=",";
                arr+="{\"keyword\":"+q(key)+",\"start\":"+num(pos)+",\"end\":"+num(ftell(f))+",\"next\":"+q(next(f))+",\"changes\":"+delta(old,snapshot(ps,nps))+",\"ecliptic_changes\":"+delta(eb,params(e,2,false))+"}";
            }result=arr+"]";fclose(f);
        }else if(a[0]=="copy"){copyParam(ps[0].param[index(a[1])],&ps[0].param[index(a[2])]);
        }else if(a[0]=="get")result=num(getParameterValue(ps,index(a[1]),stoi(a[2])));
        else if(a[0]=="final")checkAllSet(ps,*e,*(e+1),(char*)"caller.par");
        else if(a[0]=="defaults")result=num(setupParameterFileDefaults(ps));
        else if(a[0]=="env"){
            if(a[1]=="unset")unsetenv("TEMPO2");else setenv("TEMPO2",a[1].c_str(),1);
        }else if(a[0]=="counts"){
            ps[0].nJumps=3;ps[0].nPhaseJump=2;ps[0].param[param_sini].nLinkTo=1;ps[0].param[param_sini].linkTo[0]=param_kin;ps[0].param[param_kin].nLinkFrom=1;ps[0].param[param_kin].linkFrom[0]=param_sini;
        }else if(a[0]=="simple"){
            FILE*f=fopen(a[1].c_str(),"r");assert(f);rc=readSimpleParfile(f,ps);result="{\"status\":"+num(rc)+",\"position\":"+num(ftell(f))+"}";fclose(f);
        }else if(a[0]=="full"||a[0]=="global"){
            char par[MAX_STRLEN][MAX_FILELEN]={{0}},tim[MAX_STRLEN][MAX_FILELEN]={{0}};
            int count=stoi(a[1]);assert(count<=nps);
            for(int i=0;i<count;i++){strcpy(par[i],a[2+2*i].c_str());strcpy(tim[i],a[3+2*i].c_str());}
            if(a[0]=="full")readParfile(ps,par,tim,count);else readParfileGlobal(ps,count,par,tim);
        }else if(a[0]=="fixed"){
            char b[1000];strcpy(b,a[5].c_str());getValue(b,stoi(a[3]),stoi(a[4]),ps,index(a[1]),stoi(a[2]));
        }else if(a[0]=="destroy"){
            assert(ready);for(int i=0;i<nps;i++)destroyOne(ps+i);ready=false;before.clear();result="{\"matching_lifetime_complete\":true}";
        }else abort();
        State after=ready?snapshot(ps,nps):State();
        events<<"{\"action\":"<<action<<",\"phase\":\"end\",\"op\":"<<q(a[0])<<",\"result\":"<<result<<",\"changes\":"<<delta(before,after)<<"}"<<std::endl;action++;
    }
    if(ready)for(int i=0;i<nps;i++)destroyOne(ps+i);
    free(ps);return 0;
}
