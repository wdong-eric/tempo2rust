// AF source callers; scientific bodies are pinned native code, not harness models.
#include "tempo2.h"
#include "tempo2pred.h"
#define T1PolycoSet_GetNearest AF_obsolete_header_prototype
#include "tempo2pred_int.h"
#undef T1PolycoSet_GetNearest
// fda6ad4 internal header omits the set argument; match the actual C definition.
extern "C" T1Polyco* T1PolycoSet_GetNearest(T1PolycoSet*,long double);
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>
#include <vector>
#include <cassert>
extern "C" { void Cheby2D_Init(Cheby2D*,int,int); void Cheby2D_Destroy(Cheby2D*); long double Cheby2D_Evaluate(const Cheby2D*,long double,long double); }
extern void chebyModelFunc(long double*,long double*,int,int,long double*,void*);
struct ModelInfo{ChebyModel* model;const pulsar* psr;bool compute_dispersion_constant;};
static FILE* out;
static void value(const char* key,int i,long double x){fprintf(out,"%s\t%d\t%.36Lg\n",key,i,x);}
static void timing_value(const char* key,int i,longdouble x){char s[128];quadmath_snprintf(s,sizeof(s),"%.40Qg",x);fprintf(out,"%s\t%d\t%s\n",key,i,s);}
static std::string callback_kind="polynomial";
static void callback(long double*x,long double*y,int nx,int ny,long double*z,void*){for(int b=0;b<ny;b++)for(int a=0;a<nx;a++)z[b*nx+a]=callback_kind=="constant"?3.0L:callback_kind=="linear"?3.0L+2.0L*x[a]-0.5L*y[b]:3.0L+2.0L*x[a]-0.5L*y[b]+0.25L*x[a]*y[b]+0.125L*x[a]*x[a];}
static void word(const char* key,int i,const char* v){fprintf(out,"text.%s\t%d\t%s\n",key,i,v);}
static void dump(ChebyModelSet& s){value("segments",0,s.nsegments);for(int k=0;k<s.nsegments;k++){auto&m=s.segments[k];word("psrname",k,m.psrname);word("sitename",k,m.sitename);value("mjd_start",k,m.mjd_start);value("mjd_end",k,m.mjd_end);value("freq_start",k,m.freq_start);value("freq_end",k,m.freq_end);value("dispersion_constant",k,m.dispersion_constant);value("nx",k,m.cheby.nx);value("ny",k,m.cheby.ny);for(int i=0;i<m.cheby.nx*m.cheby.ny;i++){value(("coeff"+std::to_string(k)).c_str(),i,m.cheby.coeff[i]);value(("derivative"+std::to_string(k)).c_str(),i,m.frequency_cheby.coeff[i]);}}}
static void dump_t1(T1PolycoSet& s){value("segments",0,s.nsegments);for(int k=0;k<s.nsegments;k++){auto&m=s.segments[k];word("psrname",k,m.psrname);word("sitename",k,m.sitename);word("date_string",k,m.date_string);word("utc_string",k,m.utc_string);value("mjd_mid",k,m.mjd_mid);value("dm",k,m.dm);value("doppler",k,m.doppler);value("log10rms",k,m.log10rms);value("reference_phase",k,m.reference_phase);value("frequency_psr_0",k,m.frequency_psr_0);value("span",k,m.span);value("frequency_obs",k,m.frequency_obs);value("binary_phase",k,m.binary_phase);value("binary_frequency",k,m.binary_frequency);value("ncoeff",k,m.ncoeff);for(int i=0;i<m.ncoeff;i++)value(("coeff"+std::to_string(k)).c_str(),i,m.coeff[i]);}}
// Observation-only hook at tzFit's pre-serialization boundary, no numerical work.
void af_polyco(pulsar* p,int iref,int nc,longdouble* coeff,longdouble phase,longdouble dm,double doppler,double rms,double utc,const char* site,int span,double freq,const char* date,longdouble bp,longdouble bf){
 static int k=0;word("psrname",k,p->name);word("sitename",k,site);word("date_string",k,date);char u[64];snprintf(u,sizeof(u),"%.2f",utc);word("utc_string",k,u);
 value("mjd_mid",k,p->obsn[iref].sat);value("dm",k,(double)dm);value("doppler",k,doppler);value("log10rms",k,rms);value("reference_phase",k,phase);value("frequency_psr_0",k,p->param[param_f].val[0]);value("span",k,span);value("frequency_obs",k,freq);value("binary_phase",k,(double)bp);value("binary_frequency",k,(double)bf);value("ncoeff",k,nc);
 for(int i=0;i<nc;i++){value(("coeff"+std::to_string(k)).c_str(),i,coeff[i]);timing_value(("timing_coeff"+std::to_string(k)).c_str(),i,coeff[i]);}k++;
}
extern "C" void af_plan_interval(int i,const T2Predictor* p,long double start,long double end){
 value("start_mjd",i,start);value("end_mjd",i,end);
 int a,b;if(p->kind==Cheby){auto*set=&p->modelset.cheby;auto*x=ChebyModelSet_GetNearest(set,start);auto*y=ChebyModelSet_GetNearest(set,end);a=x?x-set->segments:-1;b=y?y-set->segments:-1;}else{auto*set=const_cast<T1PolycoSet*>(&p->modelset.t1);a=T1PolycoSet_GetNearest(set,start)-set->segments;b=T1PolycoSet_GetNearest(set,end)-set->segments;}
 value("start_segment",i,a);value("end_segment",i,b);value("out_of_range",i,a<0||b<0);
}
int main(int argc,char**argv){if(argc<3)return 2;out=fopen(argv[2],"w");if(!out)return 3;std::string mode=argv[1];
 if(mode=="component"){
  if(argc>3)callback_kind=argv[3];
  Cheby2D c,d;Cheby2D_Init(&c,8,6);Cheby2D_Init(&d,8,6);Cheby2D_Construct(&c,callback,nullptr);Cheby2D_Construct_x_Derivative(&d,&c);
  for(int i=0;i<48;i++){value("coeff",i,c.coeff[i]);value("derivative",i,d.coeff[i]);}
  for(int i=0;i<13;i++){long double x=-1.0L+2.0L*i/12,y=0.73L-0.1L*i;value("phase",i,Cheby2D_Evaluate(&c,x,y));value("frequency",i,Cheby2D_Evaluate(&d,x,y));}
  long double rms,mav;Cheby2D_Test(&c,17,11,callback,nullptr,&rms,&mav);value("rms",0,rms);value("mav",0,mav);Cheby2D_Destroy(&c);Cheby2D_Destroy(&d);
 }else if(mode=="history"){
  T2Predictor a,b;T2Predictor_Init(&a);T2Predictor_Init(&b);a.kind=Cheby;auto& set=a.modelset.cheby;set.nsegments=3;set.segments=(ChebyModel*)calloc(3,sizeof(ChebyModel));
  for(int i=0;i<3;i++){auto&m=set.segments[i];ChebyModel_Init(&m,3,2);memset(m.cheby.coeff,0,6*sizeof(long double));strcpy(m.psrname,"J0000");strcpy(m.sitename,"@");m.mjd_start=i==2?5:i;m.mjd_end=i==2?6:i+2;m.freq_start=1000;m.freq_end=2000;m.dispersion_constant=0;m.cheby.coeff[0]=12;m.cheby.coeff[1]=4;m.cheby.coeff[3]=2;Cheby2D_Construct_x_Derivative(&m.frequency_cheby,&m.cheby);}
  assert(ChebyModelSet_GetNearest(&set,1.5)==set.segments);assert(ChebyModelSet_GetNearest(&set,3)==set.segments+1);assert(!ChebyModelSet_GetNearest(&set,4));value("gap_phase",0,T2Predictor_GetPhase(&a,4,1400));
  T2Predictor_Copy(&b,&a);assert(T2Predictor_Insert(&b,&a)==0);T2Predictor_Destroy(&a);
  long double times[2]={5.5,1.5};T2Predictor_Keep(&b,2,times);assert(b.modelset.cheby.nsegments==2);assert(b.modelset.cheby.segments[0].mjd_start==0);assert(b.modelset.cheby.segments[1].mjd_start==5);T2Predictor_Destroy(&b);
  T2Predictor_Init(&a);FILE*f=tmpfile();fputs("invalid\n",f);rewind(f);assert(T2Predictor_FRead(&a,f)!=0);assert(ftell(f)==0);fclose(f);T2Predictor_Destroy(&a);value("lifecycle_passed",0,1);
 }else if(mode=="evaluate"){
  if(argc!=6)return 2;T2Predictor p;T2Predictor_Init(&p);if(T2Predictor_Read(&p,argv[3]))return 4;long double t=strtold(argv[4],nullptr),f=strtold(argv[5],nullptr);value("phase",0,T2Predictor_GetPhase(&p,t,f));value("frequency",0,T2Predictor_GetFrequency(&p,t,f));value("kind",0,p.kind);value("selected",0,p.kind==Cheby?(ChebyModelSet_GetNearest(&p.modelset.cheby,t)?ChebyModelSet_GetNearest(&p.modelset.cheby,t)-p.modelset.cheby.segments:-1):(T1PolycoSet_GetNearest(&p.modelset.t1,t)-p.modelset.t1.segments));value("out_of_range",0,ChebyModelSet_OutOfRange);T2Predictor_Destroy(&p);
 }else if(mode=="inspect"){
  if(argc!=4)return 2;T2Predictor p;T2Predictor_Init(&p);if(T2Predictor_Read(&p,argv[3]))return 4;if(p.kind==Cheby)dump(p.modelset.cheby);else dump_t1(p.modelset.t1);T2Predictor_Destroy(&p);
 }else if(mode=="plan"){
  if(argc!=8)return 2;long double phase,frequencies[4096];int count;char psr[64],site[64];int status=T2Predictor_GetPlan_Ext(argv[3],strtold(argv[4],nullptr),strtold(argv[5],nullptr),strtold(argv[6],nullptr),strtold(argv[7],nullptr),psr,site,&phase,&count,frequencies);value("status",0,status);if(!status){value("phase0",0,phase);value("count",0,count);for(int i=0;i<count;i++)value("average_hz",i,frequencies[i]);}
 }else if(mode=="quality"){
  if(argc!=5)return 2;T2Predictor p;T2Predictor_Init(&p);if(T2Predictor_Read(&p,argv[3]))return 4;
  MAX_PSR=1;MAX_OBSN=8192;auto* ps=new pulsar[1];initialise(ps,1);char par[1][MAX_FILELEN],tim[1][MAX_FILELEN]={{0}};strcpy(par[0],argv[4]);readParfile(ps,par,tim,1);preProcess(ps,1,0,nullptr);
  if(!ps->param[param_tzrmjd].paramSet[0]){ps->param[param_tzrmjd].paramSet[0]=1;ps->param[param_tzrmjd].val[0]=ps->param[param_pepoch].val[0];}if(!ps->param[param_tzrfrq].paramSet[0]){ps->param[param_tzrfrq].paramSet[0]=1;ps->param[param_tzrfrq].val[0]=1400;}if(!strcmp(ps->tzrsite,"NULL"))strcpy(ps->tzrsite,T2Predictor_GetSiteName(&p));
  int count=p.kind==Cheby?p.modelset.cheby.nsegments:p.modelset.t1.nsegments;
  timing_value("f0",0,ps->param[param_f].val[0]);int row=0;
  for(int segment=0;segment<count;segment++){
   ChebyModel m={};strcpy(m.sitename,T2Predictor_GetSiteName(&p));
   if(p.kind==Cheby){auto& a=p.modelset.cheby.segments[segment];m.mjd_start=a.mjd_start;m.mjd_end=a.mjd_end;m.freq_start=a.freq_start;m.freq_end=a.freq_end;}
   else{auto& a=p.modelset.t1.segments[segment];m.mjd_start=a.mjd_mid-a.span/2880.0L;m.mjd_end=a.mjd_mid+a.span/2880.0L;m.freq_start=a.frequency_obs*0.8;m.freq_end=a.frequency_obs*1.2;}
   long double dx=0.5L/86400*2/(m.mjd_end-m.mjd_start),x[15],y[3]={-0.7,0.1,0.8},z[45];double bases[5]={-0.87,-0.31,0.13,0.71,0.93};for(int j=0;j<5;j++){x[3*j]=bases[j]-dx;x[3*j+1]=bases[j];x[3*j+2]=bases[j]+dx;}ModelInfo info={&m,ps,false};chebyModelFunc(x,y,15,3,z,&info);
   for(int iy=0;iy<3;iy++)for(int ix=0;ix<5;ix++){
    int i=1+iy*15+ix*3+1;auto&o=ps->obsn[i];long double t=o.sat,f=o.freq;longdouble reference=o.phase-(p.kind==Cheby?ps->obsn[0].phase:0);longdouble hz=(ps->obsn[i+1].phase-ps->obsn[i-1].phase)/(ps->obsn[i+1].sat-ps->obsn[i-1].sat)/86400;
    value("phase",row,T2Predictor_GetPhase(&p,t,f));timing_value("full_phase",row,reference);value("frequency",row,T2Predictor_GetFrequency(&p,t,f));timing_value("full_frequency",row,hz);value("mjd",row,t);value("freq",row,f);row++;
   }
  }T2Predictor_Destroy(&p);destroyOne(ps);delete[] ps;
 }else if(mode=="cheby"||mode=="polyco"){
  if(argc<13)return 2;MAX_PSR=1;MAX_OBSN=8192;auto* ps=new pulsar[1];initialise(ps,1);char par[1][MAX_FILELEN],tim[1][MAX_FILELEN]={{0}};strcpy(par[0],argv[3]);readParfile(ps,par,tim,1);std::vector<char*> options={argv[0]};for(int i=13;i<argc;i++)options.push_back(argv[i]);preProcess(ps,1,options.size(),options.data());
  const char* site=argv[4];if(!ps->param[param_tzrmjd].paramSet[0]){ps->param[param_tzrmjd].paramSet[0]=1;ps->param[param_tzrmjd].val[0]=ps->param[param_pepoch].val[0];}if(!ps->param[param_tzrfrq].paramSet[0]){ps->param[param_tzrfrq].paramSet[0]=1;ps->param[param_tzrfrq].val[0]=1400;}if(!strcmp(ps->tzrsite,"NULL"))strcpy(ps->tzrsite,site);
  if(mode=="cheby"){
   int nt=atoi(argv[9]),nf=atoi(argv[10]);nt+=nt%2;nf+=nf%2;longdouble duration=parse_longdouble(argv[11])/SECDAYl;ChebyModelSet s;ChebyModelSet_Init(&s);ChebyModelSet_Construct(&s,ps,site,strtold(argv[5],nullptr),strtold(argv[6],nullptr),duration,duration*0.1,strtold(argv[7],nullptr),strtold(argv[8],nullptr),nt,nf);dump(s);FILE*f=fopen("t2pred.dat","w");if(!f)return 5;ChebyModelSet_Write(&s,f);fclose(f);long double rms,mav;ChebyModelSet_Test(&s,ps,nt*5*s.nsegments,nf*5*s.nsegments,&rms,&mav);value("rms",0,rms);value("mav",0,mav);ChebyModelSet_Destroy(&s);
  }else{
   longdouble coeff[MAX_COEFF];polyco(ps,1,parse_longdouble(argv[5]),parse_longdouble(argv[6]),atoi(argv[7]),atoi(argv[8]),parse_longdouble(argv[9]),argv[4],parse_longdouble(argv[10]),coeff,atoi(argv[11]),argv[12]);for(int i=0;i<atoi(argv[8]);i++)timing_value("last_coeff",i,coeff[i]);
  }
  for(int i=0;i<ps->nobs;i++){timing_value("sat",i,ps->obsn[i].sat);timing_value("bat",i,ps->obsn[i].bat);timing_value("phase",i,ps->obsn[i].phase);value("freq",i,ps->obsn[i].freq);}
  destroyOne(ps);delete[] ps;
 }else return 2;fclose(out);return 0;
}
