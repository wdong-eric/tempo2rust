// Assignment S: observation/ownership harness around unchanged pinned native routines.
#include "tempo2.h"
#include "TKmatrix.h"
#include <algorithm>
#include <cassert>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <map>
#include <string>
#include <type_traits>
#include <vector>
#include <climits>
#include <cfloat>

static FILE* out;
static bool comma;
static std::map<const void*,std::string> names;
static char opaque[2][32] = {"opaque-source", "opaque-destination"};
static double deriv0(pulsar*,int,double,int,param_label,int){return 0;}
static double deriv1(pulsar*,int,double,int,param_label,int){return 1;}
static double constraint0(pulsar*,int,constraint_label,param_label,int,int,void*){return 0;}
static double constraint1(pulsar*,int,constraint_label,param_label,int,int,void*){return 1;}
static void update0(pulsar*,int,param_label,int,double,double){}
static void update1(pulsar*,int,param_label,int,double,double){}
static std::string q(const std::string& s){std::string r="\"";for(unsigned char c:s){if(c=='"'||c=='\\')r+='\\';if(c<32){char b[7];sprintf(b,"\\u%04x",c);r+=b;}else r+=c;}return r+'"';}
static std::string hex(const unsigned char* p,size_t n){const char*h="0123456789abcdef";std::string r;for(size_t i=0;i<n;i++){r+=h[p[i]>>4];r+=h[p[i]&15];}return r;}
static void key(const std::string& p){if(comma)fputc(',',out);comma=true;fprintf(out,"{\"path\":%s,",q(p).c_str());}
static void unwritten(const std::string& p){key(p);fputs("\"state\":\"unwritten\"}",out);}
static void bytes(const std::string& p,const void* addr,size_t count,size_t width){
 key(p);fprintf(out,"\"unit\":%zu,\"count\":%zu,\"runs\":[",width,count);
 const unsigned char* b=static_cast<const unsigned char*>(addr);size_t at=0;bool sep=false;
 while(at<count){size_t end=at+1;while(end<count && memcmp(b+at*width,b+end*width,width)==0)++end;
 if(sep)fputc(',',out);sep=true;fprintf(out,"[%zu,\"%s\"]",end-at,hex(b+at*width,width).c_str());at=end;}fputs("]}",out);
}
static std::string tag(const void* p){if(!p)return "null";auto i=names.find(p);assert(i!=names.end());return i->second;}
static std::string tag(paramDerivFunc p){if(!p)return "null";assert(p==deriv0||p==deriv1);return p==deriv0?"callback:deriv0":"callback:deriv1";}
static std::string tag(constraintDerivFunc p){if(!p)return "null";assert(p==constraint0||p==constraint1);return p==constraint0?"callback:constraint0":"callback:constraint1";}
static std::string tag(paramUpdateFunc p){if(!p)return "null";assert(p==update0||p==update1);return p==update0?"callback:update0":"callback:update1";}
static void text(const std::string&p,const std::string&s){key(p);fprintf(out,"\"symbol\":%s}",q(s).c_str());}
static void prefix(const std::string&p,const char*s,size_t capacity){size_t n=strlen(s)+1;assert(n<=capacity);bytes(p,s,n,1);text(p+".suffix","unwritten:"+std::to_string(capacity-n));}

template<class T> typename std::enable_if<!std::is_pointer<T>::value && !std::is_array<T>::value && (std::is_arithmetic<T>::value||std::is_enum<T>::value),void>::type seed(T&x,int v){x=static_cast<T>(v);}
static void seed(__float128&x,int v){x=v+0.125Q;}
static void seed(double&x,int v){x=v+0.25;}
static void seed(float&x,int v){x=v+0.5f;}
static void seed(enum constraint&x,int v){x=v==11?constraint_dmmodel_mean:constraint_dmmodel_cw_0;}
static void seed(enum series_type&x,int v){x=static_cast<series_type>(v==11?0:1);}
template<class T> void seed(T*&x,int){x=nullptr;}
static void seed(void*&x,int v){x=opaque[v==11?0:1];}
static void seed(paramDerivFunc&x,int v){x=v==11?deriv0:deriv1;}
static void seed(constraintDerivFunc&x,int v){x=v==11?constraint0:constraint1;}
static void seed(paramUpdateFunc&x,int v){x=v==11?update0:update1;}
template<size_t N> void seed(char(&x)[N],int v){std::fill(x,x+N,char('A'+v%20));assert(N>4);x[0]='s';x[1]=char('0'+v/10);x[2]=char('0'+v%10);x[3]=0;}
// Forward declarations allow recursive arrays of nested source types.
void seed(clock_correction&,int);void seed(observation&,int);void seed(storePrecision&,int);void seed(FitOutput&,int);void seed(FitInfo&,int);void seed(pulsar&,int);
template<class T,size_t N> void seed(T(&x)[N],int v){for(size_t i=0;i<N;i++)seed(x[i],v);}
template<class T> typename std::enable_if<std::is_arithmetic<T>::value||std::is_enum<T>::value,void>::type dump(const std::string&p,const T&x){bytes(p,&x,1,sizeof x);}
static void dump(const std::string&p,const __float128&x){bytes(p,&x,1,sizeof x);}
template<class T> void dump(const std::string&p,T*const&x){text(p,tag(x));}
void dump(const std::string&,const clock_correction&);void dump(const std::string&,const observation&);void dump(const std::string&,const storePrecision&);void dump(const std::string&,const FitOutput&);void dump(const std::string&,const FitInfo&);void dump(const std::string&,const pulsar&);
template<class T> using Leaf=typename std::remove_all_extents<T>::type;
template<class T,size_t N> typename std::enable_if<std::is_arithmetic<Leaf<T>>::value||std::is_enum<Leaf<T>>::value||std::is_same<Leaf<T>,__float128>::value,void>::type dump(const std::string&p,const T(&x)[N]){bytes(p,x,sizeof x/sizeof(Leaf<T>),sizeof(Leaf<T>));}
template<class T,size_t N> typename std::enable_if<!(std::is_arithmetic<Leaf<T>>::value||std::is_enum<Leaf<T>>::value||std::is_same<Leaf<T>,__float128>::value||std::is_pointer<Leaf<T>>::value),void>::type dump(const std::string&p,const T(&x)[N]){for(size_t i=0;i<N;i++)dump(p+"["+std::to_string(i)+"]",x[i]);}
template<class T,size_t N> typename std::enable_if<std::is_pointer<T>::value,void>::type dump(const std::string&p,const T(&x)[N]){
 key(p);fprintf(out,"\"pointer_count\":%zu,\"symbols\":[",N);size_t at=0;bool sep=false;
 while(at<N){size_t end=at+1;while(end<N && x[end]==x[at])end++;if(sep)fputc(',',out);sep=true;fprintf(out,"[%zu,%s]",end-at,q(tag(x[at])).c_str());at=end;}fputs("]}",out);
}
static void params(const std::string&p,const pulsar&x,bool full){
 for(int i=0;i<MAX_PARAMS;i++){const parameter& a=x.param[i];std::string n=p+"["+std::to_string(i)+"]";
 dump(n+".aSize",a.aSize);dump(n+".nLinkTo",a.nLinkTo);dump(n+".nLinkFrom",a.nLinkFrom);
 if(full){dump(n+".linkTo",a.linkTo);dump(n+".linkFrom",a.linkFrom);}else{unwritten(n+".linkTo");unwritten(n+".linkFrom");}
 #define PTR(f) dump(n+"." #f,a.f)
 PTR(val);PTR(err);PTR(prefit);PTR(prefitErr);PTR(fitFlag);PTR(paramSet);PTR(label);PTR(shortlabel);
 #undef PTR
 #define DATA(f) bytes(n+"." #f ".data",a.f,a.aSize,sizeof *a.f)
 DATA(val);DATA(err);DATA(fitFlag);DATA(paramSet);
 if(full){DATA(prefit);DATA(prefitErr);}else{unwritten(n+".prefit.data");unwritten(n+".prefitErr.data");}
 #undef DATA
 for(int j=0;j<a.aSize;j++){std::string b="["+std::to_string(j)+"]";dump(n+".label"+b,a.label[j]);dump(n+".shortlabel"+b,a.shortlabel[j]);bytes(n+".label"+b+".data",a.label[j],100,1);bytes(n+".shortlabel"+b+".data",a.shortlabel[j],100,1);}
 }
}
static void constraintPayloads(const std::string&p,const pulsar&x){for(int i=0;i<MAX_PARAMS;i++)if(x.constraint_special[i])bytes(p+".constraint_special["+std::to_string(i)+"].data",x.constraint_special[i],strlen(x.constraint_special[i])+1,1);}
#include "generated.h"
static void registerParams(const pulsar&x,const std::string&p){for(int i=0;i<MAX_PARAMS;i++){const parameter&a=x.param[i];std::string n=p+".param["+std::to_string(i)+"]";
 #define REG(f) names[a.f]=n+"." #f
 REG(val);REG(err);REG(prefit);REG(prefitErr);REG(fitFlag);REG(paramSet);REG(label);REG(shortlabel);
 #undef REG
 for(int j=0;j<a.aSize;j++){names[a.label[j]]=n+".label["+std::to_string(j)+"]";names[a.shortlabel[j]]=n+".shortlabel["+std::to_string(j)+"]";}}}
static void registerOwner(const pulsar&x,const std::string&p){names[x.obsn]=p+".observations@0";names[x.covar]=p+".covariance";registerParams(x,p);}
static void matrix(const std::string&p,double**m){size_t r=get_blas_rows(m),c=get_blas_cols(m);dump(p+".rows",r);dump(p+".cols",c);
 std::vector<size_t> offsets;for(size_t i=0;i<r;i++)offsets.push_back(m[i]-m[0]);bytes(p+".row_offsets",offsets.data(),r,sizeof(size_t));bytes(p+".cells",m[0],r*c,sizeof(double));}
static void begin(const std::string&phase){fflush(stdout);fprintf(out,"{\"phase\":%s,\"stdout_position\":%ld,\"state\":[",q(phase).c_str(),ftell(stdout));comma=false;}
static void end(){fputs("]}\n",out);fflush(out);}
static void state(const std::string&phase,const pulsar&x,bool full){begin(phase);if(full)dump("p",x);else natural("p",x);matrix("p.covar.data",x.covar);if(full)for(int i=0;i<MAX_OBSN;i++)dump("p.obsn.data["+std::to_string(i)+"]",x.obsn[i]);else unwritten("p.obsn.data");end();}
static void seedParams(pulsar&x,int v){for(int i=0;i<MAX_PARAMS;i++){parameter&a=x.param[i];seed(a.nLinkTo,v);seed(a.nLinkFrom,v);seed(a.linkTo,v);seed(a.linkFrom,v);
 for(int j=0;j<a.aSize;j++){seed(a.val[j],v);seed(a.err[j],v);seed(a.prefit[j],v);seed(a.prefitErr[j],v);seed(a.fitFlag[j],v);seed(a.paramSet[j],v);std::fill(a.label[j],a.label[j]+100,char('A'+v%20));std::fill(a.shortlabel[j],a.shortlabel[j]+100,char('A'+v%20));sprintf(a.label[j],"label-%d",v);sprintf(a.shortlabel[j],"s%d",v);}}}
static void seedOwner(pulsar&x,int v,const std::string&p){seed(x,v);allocateMemory(&x,0);seedParams(x,v);x.obsn=static_cast<observation*>(malloc(sizeof(observation)*MAX_OBSN));for(int i=0;i<MAX_OBSN;i++)seed(x.obsn[i],v+i);x.covar=malloc_blas(2,3);for(int i=0;i<6;i++)x.covar[0][i]=v+0.25;
 x.ToAextraCovar=malloc_blas(1,2);x.ToAextraCovar[0][0]=v;x.ToAextraCovar[0][1]=v+1;names[x.ToAextraCovar]=p+".extraCovar";registerOwner(x,p);for(int i=0;i<MAX_PARAMS;i++){std::string value="constraint-"+std::to_string(v)+"-"+std::to_string(i);x.constraint_special[i]=strdup(value.c_str());names[x.constraint_special[i]]=p+".constraint["+std::to_string(i)+"]";}}
static void extras(const std::string&p,const pulsar&x){matrix(p+".ToAextraCovar.data",x.ToAextraCovar);}
int main(int argc,char**argv){assert(argc>=4);out=fopen(argv[1],"wb");assert(out);fprintf(out,"{\"phase\":\"launch\",\"nonce\":%s}\n",q(argv[2]).c_str());fflush(out);
 names[opaque[0]]="opaque:source";names[opaque[1]]="opaque:destination";MAX_OBSN=4;MAX_PSR=1;debugFlag=0;displayCVSversion=0;
 std::string op=argv[3];auto arg=[&](int n){assert(3+n<argc);return atoi(argv[3+n]);};
 begin("profile");bytes("opaque_source",opaque[0],32,1);bytes("opaque_destination",opaque[1],32,1);int mant=0;__float128 eps=1;while(1+eps!=1){eps/=2;mant++;}dump("longdouble_mantissa",mant);size_t a=sizeof(pulsar),b=sizeof(observation),c=sizeof(size_t);dump("pulsar_bytes",a);dump("observation_bytes",b);dump("size_t_bytes",c);dump("MAX_OBSN",MAX_OBSN);int fit=MAX_FIT;dump("MAX_FIT",fit);end();
 if(op=="matrix"||op=="uinv"){
  begin("before_allocate");end();double**m=op=="uinv"?malloc_uinv(arg(1)):malloc_blas(arg(1),arg(2));begin("allocated");matrix("matrix",m);end();size_t r=get_blas_rows(m),c=get_blas_cols(m);m[0][0]=3.25;m[r-1][c-1]=-7.5;begin("caller_writes");matrix("matrix",m);end();if(op=="uinv")free_uinv(m);else free_blas(m);begin("destroy_returned");end();
 }else if(op=="natural"||op=="wrapper"){
  int count=op=="wrapper"?arg(1):1;MAX_PSR=count;pulsar*p=new pulsar[count];if(argc>6)debugFlag=arg(3);if(argc>7)displayCVSversion=arg(4);begin("before_initialise");text("all_pulsars","unwritten");dump("MAX_PSR",MAX_PSR);end();
  if(op=="wrapper")initialise(p,arg(2));else initialiseOne(p,arg(2),arg(1));for(int i=0;i<count;i++){registerOwner(p[i],"owner"+std::to_string(i));state("initialised_"+std::to_string(i),p[i],false);}
  for(int i=0;i<count;i++)destroyOne(p+i);delete[]p;begin("destroy_returned");end();
 }else if(op=="allocate"){
  pulsar*p=new pulsar;begin("before_allocate");text("p","unwritten");end();allocateMemory(p,arg(1));registerParams(*p,"first");begin("allocated");params("p.param",*p,false);end();seedParams(*p,11);pulsar*old=new pulsar;std::copy(p->param,p->param+MAX_PARAMS,old->param);begin("caller_seeded");params("p.param",*p,true);end();allocateMemory(p,-arg(1));registerParams(*p,"second");begin("reallocated");params("p.param",*p,false);params("old.param",*old,true);end();destroyMemory(p);destroyMemory(old);delete p;delete old;begin("destroy_returned");end();
 }else if(op=="repeat"){
  pulsar*p=new pulsar;seedOwner(*p,11,"first");state("caller_seeded",*p,true);pulsar*old=new pulsar(*p);initialiseOne(p,arg(1),arg(2));registerOwner(*p,"second");begin("reinitialised");retainedAfterInit("p",*p);matrix("p.covar.data",p->covar);unwritten("p.obsn.data");dump("old",*old);matrix("old.covar.data",old->covar);extras("old",*old);for(int i=0;i<MAX_OBSN;i++)dump("old.obsn.data["+std::to_string(i)+"]",old->obsn[i]);end();
  seedParams(*p,22);for(int i=0;i<MAX_OBSN;i++)seed(p->obsn[i],22+i);state("second_caller_seeded",*p,true);pulsar*old2=new pulsar(*p);initialiseOne(p,arg(2),arg(1));registerOwner(*p,"third");begin("initialised_again");retainedAfterInit("p",*p);matrix("p.covar.data",p->covar);unwritten("p.obsn.data");dump("old2",*old2);matrix("old2.covar.data",old2->covar);for(int i=0;i<MAX_OBSN;i++)dump("old2.obsn.data["+std::to_string(i)+"]",old2->obsn[i]);end();
  destroyOne(p);destroyOne(old2);free_blas(old->ToAextraCovar);destroyOne(old);delete p;delete old;delete old2;begin("destroy_returned");end();
 }else if(op=="destroy"||op=="destroy_null"){
  pulsar*p=new pulsar;initialiseOne(p,0,1);registerOwner(*p,"owner");if(op=="destroy_null"){free(p->obsn);p->obsn=nullptr;}
  for(int i: {0,MAX_PARAMS-1}){p->constraint_special[i]=strdup(i?"last":"first");names[p->constraint_special[i]]="constraint:"+std::to_string(i);}
  state("before_destroy",*p,false);begin("constraint_payloads");for(int i: {0,MAX_PARAMS-1})prefix(std::to_string(i),p->constraint_special[i],i?5:6);end();destroyOne(p);delete p;begin("destroy_returned");end();
 }else if(op=="copy"||op=="copy_repeat"){
  pulsar*p=new pulsar[2];seedOwner(p[0],11,"source");seedOwner(p[1],22,"destination");p[0].nobs=arg(1);p[0].nJumps=arg(2);p[1].nfdJumps=arg(3);p[0].nfdJumps=arg(4);std::string alias=argv[8];observation*shared=nullptr;
  if(alias!="disjoint"){free(p[0].obsn);free(p[1].obsn);shared=static_cast<observation*>(malloc(5*sizeof(observation)));for(int i=0;i<5;i++){seed(shared[i],31+i);names[shared+i]="shared-observations@"+std::to_string(i);}p[0].obsn=shared+(alias=="backward_overlap");p[1].obsn=shared+(alias=="forward_overlap");}
  for(int round=0;round<(op=="copy_repeat"?2:1);round++){
   auto snapshot=[&](const std::string&phase){begin(phase);dump("source",p[0]);dump("destination",p[1]);matrix("source.covar.data",p[0].covar);matrix("destination.covar.data",p[1].covar);extras("source",p[0]);extras("destination",p[1]);if(shared){for(int i=0;i<5;i++)dump("shared.data["+std::to_string(i)+"]",shared[i]);}else for(int k=0;k<2;k++)for(int i=0;i<MAX_OBSN;i++)dump(std::string(k?"destination":"source")+".obsn.data["+std::to_string(i)+"]",p[k].obsn[i]);end();};
   snapshot("before_copy_"+std::to_string(round));copyPSR(p,0,1);snapshot("after_copy_"+std::to_string(round));if(round==0&&op=="copy_repeat"){p[0].nobs=1;p[0].nJumps=0;p[1].nfdJumps=1;}
  }
  if(shared){p[0].obsn=nullptr;p[1].obsn=nullptr;free(shared);}for(int k=0;k<2;k++){free_blas(p[k].ToAextraCovar);destroyOne(p+k);}delete[]p;begin("destroy_returned");end();
 }else assert(false);
 fclose(out);return 0;
}
