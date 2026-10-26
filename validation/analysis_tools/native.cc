// AG observers operate on source APIs; output stays outside scientific routines.
#include "tempo2.h"
#include "T2toolkit.h"
#include "TKspectrum.h"
#include "TKmatrix.h"
#include "GWsim.h"
#include "native_observer_stubs.h"
#include <vector>
#include <string>
#include <cstring>
#include <cstdint>
#include <map>
#include <cstdio>
#include <cstdlib>
#include <algorithm>
extern unsigned long long ag_draws;extern int ag_index();extern unsigned long ag_word(int);extern int gwsim_Ngrid;
static FILE*out;
static void emit(std::string key,const char*unit,longdouble v){char b[150];quadmath_snprintf(b,sizeof b,"%.40Qg",v);fprintf(out,"%s\t%s\t%s\n",key.c_str(),unit,b);}
static void text(std::string key,const char*unit,const char*v){fprintf(out,"%s\t%s\t%s\n",key.c_str(),unit,v);}
static void vector(std::string key,const char*unit,const double*v,int n){emit(key+".len","exact",n);for(int i=0;i<n;i++)emit(key+"."+std::to_string(i),unit,v[i]);}
static void toolkit(long seed,int draws){
 double x[]={2,-3,0,-0.0,5.25,2,1e-5},e[]={1,2,0.5,3,0.75,1.25,2.5};float f[7];TKconvertFloat1(x,f,7);
 const char*names[]={"min","max","median","mean","rms","variance","range"};double values[]={TKfindMin_d(x,7),TKfindMax_d(x,7),TKfindMedian_d(x,7),TKmean_d(x,7),TKfindRMS_d(x,7),TKvariance_d(x,7),TKrange_d(x,7)};
 for(int i=0;i<7;i++)emit(std::string("stat.")+names[i],i<2?"f64_exact":"stat",values[i]);
 const char*fn[]={"min","max","median","mean","rms","range","weighted_rms"};float fv[]={TKfindMin_f(f,7),TKfindMax_f(f,7),TKfindMedian_f(f,7),TKmean_f(f,7),TKfindRMS_f(f,7),TKrange_f(f,7),TKfindRMSweight_d(x,e,7)};
 for(int i=0;i<7;i++)emit(std::string("float.")+fn[i],i<2?"f32_exact":"f32",fv[i]);
 double a[7],b[7],c[7];memcpy(a,x,sizeof a);memcpy(b,e,sizeof b);memcpy(c,x,sizeof c);TKsort_3d(a,b,c,7);
 double*vs[]={a,b,c};const char*sn[]={"sort.x","sort.y","sort.z"};for(int j=0;j<3;j++)for(int i=0;i<7;i++){uint64_t v;memcpy(&v,&vs[j][i],8);emit(std::string(sn[j])+"."+std::to_string(i),"bits",v);}
 float af[7],bf[7];TKconvertFloat2(x,e,af,bf,7);TKsort_2f(af,bf,7);for(int j=0;j<2;j++)for(int i=0;i<7;i++){uint32_t v;memcpy(&v,&(j?bf:af)[i],4);emit(std::string(j?"sort2.y.":"sort2.x.")+std::to_string(i),"bits",v);}
 memcpy(a,x,sizeof a);TKzeromean_d(7,a);vector("zero_mean","stat",a,7);
 const char*prefix[]={"ret.1.-0","ret.-2.1","ret.3.-1","ret.1.NaN"};double aa[]={1,-2,3,1},bb[]={-0.0,1,-1,NAN};for(int j=0;j<4;j++){emit(std::string(prefix[j])+".sign","class",TKsign_d(aa[j],bb[j]));emit(std::string(prefix[j])+".max","class",TKretMax_d(aa[j],bb[j]));emit(std::string(prefix[j])+".min","class",TKretMin_d(aa[j],bb[j]));}
 emit("seed.clock","exact",TKsetSeed());emit("capacity","exact",MAX_OBSN);
 for(int i=0;i<draws;i++)emit("default."+std::to_string(i),"exact",genrand_int32());
 for(int i=0;i<draws;i++){if(i==621)seed=-17;if(i==1000)seed=-12345;double v=i%3==0?TKgaussDev(&seed):TKranDev(&seed);std::string key="rng."+std::to_string(i);emit(key,i%3==0?"gaussian":"uniform",v);emit(key+".seed","exact",seed);emit(key+".index","exact",ag_index());emit(key+".draws","exact",ag_draws);}
 for(int i=0;i<624;i++)emit("state."+std::to_string(i),"exact",ag_word(i));
}
static pulsar* load(const char*parfile,const char*timfile,const char*runtime){
 setenv("TEMPO2",runtime,1);MAX_PSR=1;auto*ps=new pulsar[1];initialise(ps,1);
 char par[1][MAX_FILELEN],tim[1][MAX_FILELEN];strcpy(par[0],parfile);strcpy(tim[0],timfile);readParfile(ps,par,tim,1);readTimfile(ps,tim,1);char arg[]="analysis";char*args[]={arg};preProcess(ps,1,1,args);formBatsAll(ps,1);formResiduals(ps,1,1);return ps;
}
static void spectrum(pulsar*ps,int estimator,int output_kind,int prewhite,int weights,double mintau){
 int n=ps->nobs;std::vector<double>x(n),y(n),e(n);for(int i=0;i<n;i++){x[i]=ps->obsn[i].sat-ps->param[param_pepoch].val[0];y[i]=ps->obsn[i].residual;e[i]=ps->obsn[i].toaErr*1e-6;emit("row."+std::to_string(i)+".deleted","exact",ps->obsn[i].deleted);}
 vector("input.x","days",x.data(),n);vector("input.y","seconds",y.data(),n);vector("input.e","seconds",e.data(),n);
 std::vector<double>tx=x,ty=y,te=e,ox(n*4),oy(n*4),re(n*4),im(n*4);int reported=0;TKspectrum(tx.data(),ty.data(),te.data(),n,0,1,0,0,prewhite,estimator,1,1,output_kind,ox.data(),oy.data(),&reported,0,0,re.data(),im.data());
 emit("spectrum.reported","exact",reported);const char*unit=output_kind==1?"yr^3":output_kind==2?"s":output_kind==3?"s^2":estimator==2?"dimensionless":"s^2";if(estimator==5)unit=output_kind==1?"yr^3 * yr / s^2":output_kind==2?"yr^(3/2)":"yr^3";
 text("spectrum.units","text",unit);vector("spectrum.frequency","frequency_day",ox.data(),reported+1);vector("spectrum.values",unit,oy.data(),reported+1);vector("spectrum.real","coefficient_s",re.data(),estimator==1?n:estimator==4?reported+1:0);vector("spectrum.imaginary","coefficient_s",im.data(),estimator==1?n:estimator==4?reported+1:0);
 double**L=malloc_blas(n,n);for(int i=0;i<n;i++)for(int j=0;j<n;j++)L[i][j]=i==j?e[i]:0;double err[3];calcSpectraErr_complex(L,x.data(),y.data(),n,ox.data(),re.data(),im.data(),err,3);
 vector("cov.frequency","frequency_day",ox.data(),3);vector("cov.real","sqrt_year3",re.data(),3);vector("cov.imaginary","sqrt_year3",im.data(),3);vector("cov.error","yr^3",err,3);
 for(char mode:std::string("NTI")){calcSpectra_ri_T(L,x.data(),y.data(),n,ox.data(),re.data(),im.data(),3,x[n-1]-x[0],mode,ps);vector(std::string("cov")+mode+".real","sqrt_year3",re.data(),3);vector(std::string("cov")+mode+".imaginary","sqrt_year3",im.data(),3);}
 double tau[100],sz[100],plus[100],minus[100];int count;TKcalcSigmaz(*ps,weights,tau,sz,plus,minus,&count,mintau);emit("sigma.count","exact",count);for(int i=0;i<count;i++){std::string key="sigma."+std::to_string(i);emit(key+".tau","log_sigma",tau[i]);emit(key+".sigma","log_sigma",sz[i]);emit(key+".plus","log_sigma",plus[i]);emit(key+".minus","log_sigma",minus[i]);}free_blas(L);
}
static void gw(pulsar*ps,long seed,int count,const char*prefix,int loglin){
 longdouble flo=(double)1e-9,fhi=(double)1e-7,dist=(double)3.08568e19,kp[3];double amp=1e-15,alpha=-2.0/3.0;setupPulsar_GWsim(ps->param[param_raj].val[0],ps->param[param_decj].val[0],kp);
 std::vector<gwSrc>tensor(count),aniso(count),dipole(count);double h0[]={0,0,1},h1[]={1,-1,0.2};double*h[]={h0,h1};double d[]={0.01,0.005,-0.003};
 GWbackground(tensor.data(),count,&seed,flo,fhi,amp,alpha,loglin);GWanisotropicbackground(aniso.data(),count,&seed,flo,fhi,amp,alpha,loglin,h,2);GWdipolebackground(dipole.data(),count,&seed,flo,fhi,amp,alpha,loglin,d);
 std::vector<gwSrc>*ts[]={&tensor,&aniso,&dipole};const char*tn[]={"tensor","anisotropic","dipole"};for(int j=0;j<3;j++){for(auto&g:*ts[j])setupGW(&g);for(int i=0;i<ps->nobs;i++){longdouble r=0,time=(ps->obsn[i].sat-ps->param[param_pepoch].val[0])*86400.;for(auto&g:*ts[j])r+=calculateResidualGW(kp,&g,time,dist);emit(std::string(tn[j])+".residual."+std::to_string(i),"seconds",r);}}
 gwgenSpec s={amp,amp,amp,amp,alpha,alpha,alpha,alpha};int counts[]={count,count,count,count},nh[]={2,2,2,2};double**hs[]={h,h,h,h};std::vector<gwgeneralSrc>general(4*count),ganiso(4*count);
 GWgeneralbackground(general.data(),counts,&seed,flo,fhi,s,loglin);GWgeneralanisotropicbackground(ganiso.data(),counts,&seed,flo,fhi,s,loglin,hs,nh);
 std::vector<gwgeneralSrc>*gs[]={&general,&ganiso};const char*gn[]={"general","general_anisotropic"};for(int j=0;j<2;j++){for(auto&g:*gs[j])setupgeneralGW(&g);for(int i=0;i<ps->nobs;i++){longdouble r=0,time=(ps->obsn[i].sat-ps->param[param_pepoch].val[0])*86400.;for(auto&g:*gs[j])r+=calculateResidualgeneralGW(kp,&g,time,dist);emit(std::string(gn[j])+".residual."+std::to_string(i),"seconds",r);}}
 std::string tp=std::string(prefix)+".tensor.bin",gp=std::string(prefix)+".general.bin";FILE*f=fopen(tp.c_str(),"wb");GWbackground_write(tensor.data(),f,count,2);GWbackground_write(tensor.data(),f,count,9);fclose(f);f=fopen(gp.c_str(),"wb");GWgeneralbackground_write(general.data(),f,4*count,2);GWgeneralbackground_write(general.data(),f,4*count,9);fclose(f);
 f=fopen(tp.c_str(),"rb");emit("file.tensor.count","exact",GWbackground_read(tensor.data(),f,9));emit("file.tensor.cursor","exact",ftell(f));fclose(f);f=fopen(gp.c_str(),"rb");emit("file.general.count","exact",GWgeneralbackground_read(general.data(),f,9));emit("file.general.cursor","exact",ftell(f));fclose(f);emit("seed.final","exact",seed);emit("rng.draws","exact",ag_draws);
}

#include "components.h"
#include "exchange.h"
int main(int argc,char**argv){if(argc<3)return 2;out=fopen(argv[2],"w");if(!out)return 3;std::string mode=argv[1];if(mode=="bugs")bugs();else if(mode=="diagnostic")diagnostics(atof(argv[3]));else if(mode=="wire-generate")wire_generate(argv[3]);else if(mode=="wire")wire(argv[3],argv[4]);else if(mode=="components-spectrum")spectrum_components();else if(mode=="components-sigma")sigma_components();else if(mode=="components-gw")gw_components();else if(mode=="toolkit")toolkit(argc>3?atol(argv[3]):-12345,argc>4?atoi(argv[4]):1300);else{if(argc<6)return 2;pulsar*ps=load(argv[3],argv[4],argv[5]);if(mode=="eccentric")eccentric(ps,argv[6]);else if(mode=="spectrum")spectrum(ps,argc>6?atoi(argv[6]):2,argc>7?atoi(argv[7]):1,argc>8?atoi(argv[8]):0,argc>9?atoi(argv[9]):-1,argc>10?atof(argv[10]):0.01);else if(mode=="gw")gw(ps,-12345,4,argv[6],argc>7?atoi(argv[7]):1);else return 2;destroyOne(ps);delete[]ps;}fclose(out);return 0;}
