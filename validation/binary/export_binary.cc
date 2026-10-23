// TEMPO2 counterparts: *model.C entry points/update*; t2fit_stdFitFuncs.C binary
// callbacks; readParfile.C/readTimfile.C; formBats.C and formResiduals.C.
// Observer only: all model, propagation, residual and update arithmetic is upstream.
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <fstream>
#include <string>
#include <vector>
#include <stdexcept>
#include "tempo2.h"
#include "t2fit_stdFitFuncs.h"
#include "enum_str.h"
#include "resource_trace.h"
using std::string;

static string quote(const string& s) {
    string r="\"";
    for(char c:s) { if(c=='\\'||c=='\"')r+='\\'; if(c=='\n')r+="\\n"; else r+=c; }
    return r+'\"';
}
static string number(longdouble x) {
    if(isnanq(x))return quote("NaN");
    if(isinfq(x))return quote(x>0?"+Infinity":"-Infinity");
    char b[160];quadmath_snprintf(b,sizeof b,"%.36Qg",x);
    if(strtoflt128(b,nullptr)!=x)throw std::runtime_error("decimal round trip");
    return quote(b);
}
static string classification(longdouble x) {
    return quote(isnanq(x)?"NaN":isinfq(x)?(x>0?"+Infinity":"-Infinity"):"finite");
}
template<class T> static string array(const T* a,int n) {
    string s="[";for(int i=0;i<n;i++){if(i)s+=",";s+=number(a[i]);}return s+"]";
}
static int label(const string& s) {
    if(s=="delay")return -1;
    if(s=="derived")return -2;
    for(int i=0;i<param_LAST;i++)if(s==label_str[i])return i;
    throw std::runtime_error("unknown parameter "+s);
}
static longdouble native(pulsar* p,int i,int l,int k) {
    const string m=p->binaryModel;
    if(m=="BT")return BTmodel(p,0,i,l);
    if(m=="BTJ")return BTJmodel(p,0,i,l,k);
    if(m=="BTX")return BTXmodel(p,0,i,l,k);
    if(m=="ELL1")return ELL1model(p,0,i,l,k);
    if(m=="DD")return DDmodel(p,0,i,l);
    if(m=="DDK")return DDKmodel(p,0,i,l);
    if(m=="DDS")return DDSmodel(p,0,i,l);
    if(m=="MSS")return MSSmodel(p,0,i,l);
    if(m=="DDGR")return DDGRmodel(p,0,i,l);
    if(m=="T2")return T2model(p,0,i,l,k);
    if(m=="T2-PTA")return T2_PTAmodel(p,0,i,l,k);
    if(m=="DDH")return DDHmodel(p,0,i,l);
    if(m=="ELL1H")return ELL1Hmodel(p,0,i,l);
    if(m=="ELL1k")return ELL1kmodel(p,0,i,l);
    throw std::runtime_error("unknown direct dispatch");
}
static void update(pulsar* p,int l,int k,double v,double e) {
    const string m=p->binaryModel;
    if(m=="BT")updateBT(p,v,e,l);
    else if(m=="BTJ")updateBTJ(p,v,e,l,k);
    else if(m=="BTX")updateBTX(p,v,e,l,k);
    else if(m=="ELL1")updateELL1(p,v,e,l,k);
    else if(m=="DD")updateDD(p,v,e,l);
    else if(m=="DDK")updateDDK(p,v,e,l);
    else if(m=="DDS")updateDDS(p,v,e,l);
    else if(m=="MSS")updateMSS(p,v,e,l);
    else if(m=="DDGR")updateDDGR(p,v,e,l);
    else if(m=="T2")updateT2(p,v,e,l,k);
    else if(m=="T2-PTA")updateT2_PTA(p,v,e,l,k);
    else if(m=="DDH")updateDDH(p,v,e,l);
    else if(m=="ELL1H")updateELL1H(p,v,e,l);
    else if(m=="ELL1k")updateELL1k(p,v,e,l);
    else throw std::runtime_error("unknown update dispatch");
}
struct Slot {int l,k,set,fit;longdouble val,err;};
static std::vector<Slot> snapshot(pulsar* p) {
    std::vector<Slot> s;
    for(int l=0;l<param_LAST;l++)for(int k=0;k<p->param[l].aSize;k++){
        auto& a=p->param[l];s.push_back({l,k,a.paramSet[k],a.fitFlag[k],a.val[k],a.err[k]});
    }return s;
}
static void restore(pulsar* p,const std::vector<Slot>& s) {
    for(auto& a:s){auto& v=p->param[a.l];v.val[a.k]=a.val;v.err[a.k]=a.err;v.paramSet[a.k]=a.set;v.fitFlag[a.k]=a.fit;}
}
static string state(const Slot& s) {
    return "{\"value\":"+number(s.val)+",\"error\":"+number(s.err)+",\"set\":"+std::to_string(s.set)+",\"fit\":"+std::to_string(s.fit)+"}";
}
static string changes(const std::vector<Slot>& a,const std::vector<Slot>& b) {
    string s="[";bool first=true;
    for(size_t i=0;i<a.size();i++)if(state(a[i])!=state(b[i])){
        if(!first)s+=",";first=false;
        s+="{\"label\":"+quote(label_str[a[i].l])+",\"k\":"+std::to_string(a[i].k)+",\"before\":"+state(a[i])+",\"after\":"+state(b[i])+"}";
    }return s+"]";
}
static string observations(pulsar* p,bool residuals) {
    string s="[";
    for(int i=0;i<p->nobs;i++){
        if(i)s+=",";auto& o=p->obsn[i];
        s+="{\"index\":"+std::to_string(i)+",\"name\":"+quote(o.fname)+",\"site\":"+quote(o.telID)
          +",\"sat_mjd\":"+number(o.sat)+",\"bat_mjd\":"+number(o.bat)+",\"bbat_mjd\":"+number(o.bbat)
          +",\"frequency_mhz\":"+number(o.freq)+",\"toa_error_us\":"+number(o.toaErr)
          +",\"clock_correction\":"+std::to_string(o.clockCorr)+",\"delay_correction\":"+std::to_string(o.delayCorr)
          +",\"psr_position_unit\":"+array(o.psrPos,3)+",\"earth_ssb_light_s\":"+array(o.earth_ssb,3);
        if(o.delayCorr)s+=",\"clock_tt_s\":"+number(getCorrectionTT(&o))+",\"tt_tb_s\":"+number(o.correctionTT_TB)
          +",\"roemer_s\":"+number(o.roemer)+",\"troposphere_s\":"+number(o.troposphericDelay)
          +",\"dispersion_s\":"+number(o.tdis1)+",\"solar_dispersion_s\":"+number(o.tdis2)
          +",\"shapiro_sun_s\":"+number(o.shapiroDelaySun)+",\"observatory_earth_light_s\":"+array(o.observatory_earth,3);
        if(residuals){
            // Inverse of formResiduals' final phaseint addition; explicitly a reconstruction.
            longdouble phaseint=(int)p->param[param_f].val[0]*((int)o.bbat-(int)p->param[param_pepoch].val[0])*86400.0;
            s+=",\"torb_s\":"+number(o.torb)+",\"pet_mjd\":"+number(o.pet)+",\"phase_turn\":"+number(o.phase)
              +",\"phase_integer_turn\":"+number(phaseint)+",\"reduced_phase_reconstructed_turn\":"+number(o.phase-phaseint)
              +",\"nphase_turn\":"+number(o.nphase)+",\"pulse_number\":"+quote(std::to_string(o.pulseN))+",\"residual_s\":"+number(o.residual);
        }
        s+="}";
    }return s+"]";
}
int main(int argc,char** argv) {
    if(argc!=6)return 2;
    // Each executable invocation owns its pulsar. No blanket zero-fill or new initializer patch.
    MAX_PSR=1;auto p=new pulsar;initialise(p,0);
    std::ofstream out(argv[5]);if(!out)return 2;
    char par[1][MAX_FILELEN],tim[1][MAX_FILELEN];
    if(strlen(argv[1])>=MAX_FILELEN||strlen(argv[2])>=MAX_FILELEN)return 2;
    strcpy(par[0],argv[1]);strcpy(tim[0],argv[2]);
    reference_stage("readParfile");readParfile(p,par,tim,1);
    reference_stage("readTimfile");readTimfile(p,tim,1);
    out<<"{\"record\":\"parsed\",\"model\":"<<quote(p->binaryModel)<<",\"n_companion\":"<<p->nCompanion<<",\"parameters\":[";
    std::ifstream labels(argv[4]);string name;bool first=true;
    while(labels>>name){int l=label(name);auto& v=p->param[l];
        for(int k=0;k<v.aSize;k++){
            if(!first)out<<",";first=false;
            Slot s={l,k,v.paramSet[k],v.fitFlag[k],v.val[k],v.err[k]};
            out<<"{\"label\":"<<quote(name)<<",\"k\":"<<k<<",\"state\":"<<state(s)<<",\"link_to\":"<<array(v.linkTo,v.nLinkTo)<<",\"link_from\":"<<array(v.linkFrom,v.nLinkFrom)<<"}";
        }
    }
    out<<"],\"configuration\":{\"units\":"<<quote(p->units==SI_UNITS?"TCB":"TDB")<<",\"clock\":"<<quote(p->clock)
       <<",\"ephemeris\":"<<quote(p->ephemeris)<<",\"time_ephemeris\":"<<p->timeEphemeris<<",\"refphs\":"<<int(p->refphs)
       <<",\"troposphere\":"<<p->correctTroposphere<<",\"ipm\":"<<p->ipm<<",\"ne_sw\":"<<number(p->ne_sw)
       <<",\"t2c_method\":"<<p->t2cMethod<<",\"dilate_frequency\":"<<p->dilateFreq<<",\"planet_shapiro\":"<<p->planetShapiro<<",\"calc_shapiro\":"<<p->calcShapiro<<",\"swm\":"<<p->swm<<",\"ne_sw_ifunc_count\":"<<p->ne_sw_ifuncN<<",\"ecliptic\":"<<int(p->eclCoord)<<"}}"<<std::endl;
    reference_stage("propagation");formBatsAll(p,1);
    const auto base=snapshot(p);
    std::vector<observation> obs(p->nobs);memcpy(obs.data(),p->obsn,p->nobs*sizeof(observation));
    out<<"{\"record\":\"prepared\",\"observations\":"<<observations(p,false)<<"}"<<std::endl;
    std::ifstream requests(argv[3]);string op,lab,increment,error,validity;int k,i,repeats,id=0;
    while(requests>>op>>lab>>k>>i>>repeats>>increment>>error>>validity){
        restore(p,base);memcpy(p->obsn,obs.data(),p->nobs*sizeof(observation));
        int l=label(lab);double v=strtod(increment.c_str(),nullptr),e=strtod(error.c_str(),nullptr);
        for(int repeat=0;repeat<repeats;repeat++){
            const auto before=snapshot(p);
            // A durable start record is the only claim if the source exits/hangs.
            out<<"{\"record\":\"start\",\"id\":"<<id<<",\"repeat\":"<<repeat<<",\"op\":"<<quote(op)<<",\"label\":"<<quote(lab)<<",\"k\":"<<k<<",\"obs\":"<<i<<"}"<<std::endl;
            reference_stage("binary_call");
            longdouble value=0;
            if(op=="native")value=native(p,i,l,k);
            else if(op=="wrapper")value=t2FitFunc_binaryModels(p,0,0,i,l,k);
            else if(op=="update_native")update(p,l,k,v,e);
            else if(op=="update_wrapper")t2UpdateFunc_binaryModels(p,0,l,k,v,e);
            else if(op=="update_delay"){update(p,l,k,v,e);value=native(p,i,-1,0);}
            else if(op=="residual")formResiduals(p,1,k);
            else throw std::runtime_error("bad operation");
            out<<"{\"record\":\"result\",\"id\":"<<id<<",\"repeat\":"<<repeat<<",\"mutations\":"<<changes(before,snapshot(p));
            if(op=="residual")out<<",\"observations\":"<<observations(p,true);
            else if(op=="native"||op=="wrapper"||op=="update_delay"){
                // Undefined-source paths are executed in isolation; their return is never a golden.
                out<<",\"source_semantics\":"<<quote(validity);
                if(validity=="defined")out<<",\"value\":"<<number(value)<<",\"class\":"<<classification(value)<<",\"return_type\":"<<quote(op!="wrapper"&&string(p->binaryModel)=="DD"?"binary128":"binary64")<<",\"narrowed_binary64\":"<<number((double)value);
            }
            if(op.find("update")==0){
                out<<",\"increment_binary64\":"<<number(v)<<",\"error_binary64\":"<<number(e);
                for(auto& a:before)if(a.l==l&&a.k==k){
                    auto& t=p->param[l];Slot b={l,k,t.paramSet[k],t.fitFlag[k],t.val[k],t.err[k]};
                    out<<",\"target_before\":"<<state(a)<<",\"target_after\":"<<state(b);
                }
            }
            out<<"}"<<std::endl;
        }id++;
    }
    destroyOne(p);delete p;return 0;
}
