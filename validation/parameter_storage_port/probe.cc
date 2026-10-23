// O supplemental caller. Pinned routines compile unchanged in separate units.
// Only declared initialized values/bytes are inspected; unwritten source fields
// are emitted as null without dereferencing them. No invalid native cases run.
#include "tempo2.h"
#include "tempo2Util.h"
#include <cassert>
#include <cstring>
#include <string>
#include <fstream>
static std::string q(const std::string &s){
    std::string o="\"";for(unsigned char c:s){
        if(c=='"'||c=='\\'){o+='\\';o+=c;}
        else if(c<32||c>=127){char b[8];sprintf(b,"\\u%04x",c);o+=b;}else o+=c;
    }return o+'"';
}
static std::string hex(const void*p,size_t n){
    static const char*d="0123456789abcdef";std::string s;
    for(size_t i=0;i<n;i++){unsigned char c=((const unsigned char*)p)[i];s+=d[c>>4];s+=d[c&15];}return s;
}
static std::string num(longdouble v){
    if(isnanq(v))return "{\"class\":\"NaN\",\"type\":\"binary128\"}";
    if(isinfq(v))return "{\"class\":"+q(signbitq(v)?"-Infinity":"+Infinity")+",\"type\":\"binary128\"}";
    char b[128];quadmath_snprintf(b,sizeof(b),"%.36Qg",v);
    longdouble r=strtoflt128(b,nullptr);assert(r==v&&!!signbitq(r)==!!signbitq(v));
    return "{\"class\":\"finite\",\"type\":\"binary128\",\"decimal\":"+q(b)+",\"bytes_le\":"+q(hex(&v,sizeof v))+"}";
}
struct ParamStorage {
    parameter p{};longdouble val[4]{},err[4]{},pre[4]{},preerr[4]{};int fit[4]{},set[4]{};
    char label[4][64]{},shortlabel[4][64]{};char *lp[4],*sp[4];
    ParamStorage(int seed,int size){
        p.aSize=size;p.val=val;p.err=err;p.prefit=pre;p.prefitErr=preerr;p.fitFlag=fit;p.paramSet=set;p.label=lp;p.shortlabel=sp;
        p.nLinkTo=1;p.nLinkFrom=2;
        for(int i=0;i<5;i++){p.linkTo[i]=seed+i;p.linkFrom[i]=seed+10+i;}
        for(int i=0;i<4;i++){val[i]=seed+i+0.125Q;err[i]=seed+i+0.25Q;pre[i]=seed+i+0.5Q;preerr[i]=seed+i+0.75Q;fit[i]=seed+i;set[i]=seed+i+4;
            sprintf(label[i],"label-%d-%d",seed,i);sprintf(shortlabel[i],"short-%d-%d",seed,i);lp[i]=label[i];sp[i]=shortlabel[i];}
    }
    std::string state(){
        std::string s="{\"aSize\":"+std::to_string(p.aSize)+",\"nLinkTo\":"+std::to_string(p.nLinkTo)+",\"nLinkFrom\":"+std::to_string(p.nLinkFrom)+",\"linkTo\":[";
        for(int i=0;i<5;i++){if(i)s+=',';s+=std::to_string(p.linkTo[i]);}s+="],\"linkFrom\":[";
        for(int i=0;i<5;i++){if(i)s+=',';s+=std::to_string(p.linkFrom[i]);}s+="],\"capacity\":4,\"cells\":[";
        for(int i=0;i<4;i++){if(i)s+=',';s+="{\"val\":"+num(val[i])+",\"err\":"+num(err[i])+",\"prefit\":"+num(pre[i])+",\"prefitErr\":"+num(preerr[i])+",\"fitFlag\":"+std::to_string(fit[i])+",\"paramSet\":"+std::to_string(set[i])+",\"label_bytes\":"+q(hex(label[i],64))+",\"shortlabel_bytes\":"+q(hex(shortlabel[i],64))+"}";}
        return s+"]}";
    }
};
static std::string copyparam(int count,bool shorter){
    ParamStorage src(10,count),dst(70,4);if(shorter){strcpy(src.label[0],"x");strcpy(src.shortlabel[0],"y");}auto sb=src.state(),db=dst.state();parameter pointers=dst.p;
    copyParam(src.p,&dst.p);
    assert(pointers.val==dst.p.val&&pointers.err==dst.p.err&&pointers.prefit==dst.p.prefit&&pointers.prefitErr==dst.p.prefitErr&&pointers.label==dst.p.label&&pointers.shortlabel==dst.p.shortlabel&&pointers.fitFlag==dst.p.fitFlag&&pointers.paramSet==dst.p.paramSet);
    return "{\"source_before\":"+sb+",\"source_after\":"+src.state()+",\"destination_before\":"+db+",\"destination_after\":"+dst.state()+",\"pointers_retained\":true}";
}

static std::string natural(parameter &p){
    std::string s="{\"aSize\":"+std::to_string(p.aSize)+",\"capacity\":"+std::to_string(p.aSize)+",\"nLinkTo\":"+std::to_string(p.nLinkTo)+",\"nLinkFrom\":"+std::to_string(p.nLinkFrom)+",\"linkTo\":[null,null,null,null,null],\"linkFrom\":[null,null,null,null,null],\"cells\":[";
    for(int j=0;j<p.aSize;j++){if(j)s+=',';s+="{\"val\":"+num(p.val[j])+",\"err\":"+num(p.err[j])+",\"prefit\":null,\"prefitErr\":null,\"fitFlag\":"+std::to_string(p.fitFlag[j])+",\"paramSet\":"+std::to_string(p.paramSet[j])+",\"label_bytes\":"+q(hex(p.label[j],100))+",\"shortlabel_bytes\":"+q(hex(p.shortlabel[j],100))+"}";}
    return s+"]}";
}
int main(int argc,char**argv){
    assert(argc==3);std::ofstream out(argv[1]);assert(out);
    out<<"{\"token\":"<<q(argv[2])<<",\"supplemental\":["<<copyparam(-7,false)<<","<<copyparam(1,true)<<"],\"initialized_bytes\":[";
    MAX_PSR=1;MAX_OBSN=2;pulsar*p=new pulsar;initialiseOne(p,0,0);
    for(int i=0;i<MAX_PARAMS;i++){if(i)out<<',';out<<natural(p->param[i]);}
    out<<"]}";destroyOne(p);delete p;
}
