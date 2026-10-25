// AD integration probe: real file readers, preprocess, formBatsAll and formResiduals.
// No harness-supplied physical vectors, clocks, or source parameter values.
#include "tempo2.h"
#include "enum_str.h"
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>
#include <vector>
#include <array>
#include <fstream>
#include <map>
static FILE* output;
struct Cell {longdouble v,e;int set,fit;};
static std::vector<std::map<std::string,int>> original_rows;
static pulsar* active;
static void field(int p,int i,const char* key,const std::string& value){fprintf(output,"%d\t%d\t%s\t%s\n",p,i,key,value.c_str());}
static void number(int p,int i,const char* key,longdouble value){char s[128];quadmath_snprintf(s,sizeof s,"%.36Qg",value);field(p,i,key,s);}
static void row(int p,int i,const observation& o){
 if(i>=0)number(p,i,"input_index",original_rows[p].at(o.fname));
 if(i>=0 && active[p].AverageResiduals){number(p,i,"avg_bat",o.averagebat);number(p,i,"avg_residual",o.averageres);number(p,i,"avg_uncertainty",o.averageerr);}
 if(i>=0 && active[p].AverageDMResiduals){number(p,i,"avg_dm_bat",o.averagedmbat);number(p,i,"avg_dm_residual",o.averagedmres);number(p,i,"avg_dm_uncertainty",o.averagedmerr);}
 field(p,i,"filename",o.fname);field(p,i,"site",o.telID);
 number(p,i,"deleted",o.deleted);number(p,i,"clock_correction",o.clockCorr);number(p,i,"delay_correction",o.delayCorr);
 number(p,i,"sat_mjd",o.sat);number(p,i,"bat_mjd",o.bat);number(p,i,"bbat_mjd",o.bbat);
 number(p,i,"frequency",o.freq);number(p,i,"frequency_ssb",o.freqSSB);number(p,i,"uncertainty",o.toaErr);
 number(p,i,"nFlags",o.nFlags);for(int k=0;k<o.nFlags;k++){field(p,i,("flag_id_"+std::to_string(k)).c_str(),o.flagID[k]);field(p,i,("flag_value_"+std::to_string(k)).c_str(),o.flagVal[k]);}
 number(p,i,"nclock",o.nclock_correction);for(int k=0;k<o.nclock_correction;k++){field(p,i,("clock_to_"+std::to_string(k)).c_str(),o.correctionsTT[k].corrects_to);number(p,i,("clock_s_"+std::to_string(k)).c_str(),o.correctionsTT[k].correction);}
 number(p,i,"shklovskii",o.shklovskii);
 number(p,i,"pet_mjd",o.pet);number(p,i,"binary_delay_s",o.torb);number(p,i,"phase_turn",o.phase);number(p,i,"nphase_turn",o.nphase);
 field(p,i,"pulse_number",std::to_string(o.pulseN));number(p,i,"residual_s",o.residual);number(p,i,"residualtn_s",o.residualtn);
 if(o.delayCorr){
  number(p,i,"tt_to_tb",o.correctionTT_TB);number(p,i,"tt_to_teph",o.correctionTT_Teph);number(p,i,"roemer",o.roemer);
  number(p,i,"dispersion_chromatic",o.tdis1);number(p,i,"solar_wind",o.tdis2);number(p,i,"troposphere",o.troposphericDelay);number(p,i,"bat_correction",o.batCorr);
  number(p,i,"shapiro_sun",o.shapiroDelaySun);number(p,i,"shapiro_jupiter",o.shapiroDelayJupiter);number(p,i,"shapiro_saturn",o.shapiroDelaySaturn);number(p,i,"shapiro_venus",o.shapiroDelayVenus);number(p,i,"shapiro_uranus",o.shapiroDelayUranus);number(p,i,"shapiro_neptune",o.shapiroDelayNeptune);
 }
}
int main(int argc,char**argv){
 if(argc<6)return 2;
 output=fopen(argv[1],"w");if(!output)return 3;
 int mean=atoi(argv[2]);int n=atoi(argv[3]);if(n<1||argc<4+2*n)return 2;
 MAX_PSR=n;MAX_OBSN=64;
 auto* ps=new pulsar[n];active=ps;initialise(ps,1);
 std::vector<std::array<char,MAX_FILELEN>> pars(n),tims(n);
 for(int p=0;p<n;p++){strcpy(pars[p].data(),argv[4+2*p]);strcpy(tims[p].data(),argv[5+2*p]);}
 readParfile(ps,reinterpret_cast<char(*)[MAX_FILELEN]>(pars.data()),reinterpret_cast<char(*)[MAX_FILELEN]>(tims.data()),n);
 readTimfile(ps,reinterpret_cast<char(*)[MAX_FILELEN]>(tims.data()),n);
 original_rows.resize(n);for(int p=0;p<n;p++)for(int i=0;i<ps[p].nobs;i++){if(!original_rows[p].emplace(ps[p].obsn[i].fname,i).second)return 4;}
 std::vector<char*> options;options.push_back(argv[0]);for(int k=4+2*n;k<argc;k++)options.push_back(argv[k]);
 preProcess(ps,n,options.size(),options.data());
 const char* mode=getenv("AD_SCENARIO");const char* control=getenv("AD_CONTROL");
 if(mode && std::string(mode)=="eccentric"){
   formBatsAll(ps,n);formResiduals(ps,n,0);FILE* f=fopen(control,"r");if(!f)return 5;readSimpleParfile(f,&ps[0]);fclose(f);
 }else if(mode && std::string(mode)=="state"){
   std::ifstream f(control);std::string key,value;int k;
   while(f>>key>>k>>value){int l=key=="DF1"?param_df1:param_stateSwitchT;ps[0].param[l].val[k]=strtoflt128(value.c_str(),nullptr);ps[0].param[l].paramSet[k]=1;}
 }else if(mode && std::string(mode)=="noise"){
   ps[0].AverageDMResiduals=1;std::ifstream f(control);int i;double r,d,c,s,t;
   while(f>>i>>r>>d>>c>>s>>t){auto& o=ps[0].obsn[i];o.TNRedSignal=r;o.TNDMSignal=d;o.TNChromSignal=c;o.SWGPSignal=s;o.tobs=t;}
 }
 std::vector<std::vector<std::vector<Cell>>> before(n);
 for(int p=0;p<n;p++){before[p].resize(param_LAST);for(int l=0;l<param_LAST;l++)for(int k=0;k<ps[p].param[l].aSize;k++){auto& v=ps[p].param[l];before[p][l].push_back({v.val[k],v.err[k],v.paramSet[k],v.fitFlag[k]});}}
 formBatsAll(ps,n);formResiduals(ps,n,mean);
 for(int p=0;p<n;p++){
  field(p,-2,"name",ps[p].name);number(p,-2,"units",ps[p].units);number(p,-2,"frame",ps[p].eclCoord);number(p,-2,"time_ephemeris",ps[p].timeEphemeris);number(p,-2,"nobs",ps[p].nobs);
  int mutations=0;
  for(int l=0;l<param_LAST;l++)for(int k=0;k<ps[p].param[l].aSize;k++){
   auto& v=ps[p].param[l];auto a=before[p][l][k];
   auto same=[](longdouble a,longdouble b){return (a==b && signbitq(a)==signbitq(b))||(isnanq(a)&&isnanq(b)&&signbitq(a)==signbitq(b));};
   if(same(a.v,v.val[k])&&same(a.e,v.err[k])&&a.set==v.paramSet[k]&&a.fit==v.fitFlag[k])continue;
   std::string prefix="mutation"+std::to_string(mutations++)+".";
   field(p,-2,(prefix+"label").c_str(),label_str[l]);number(p,-2,(prefix+"index").c_str(),k);
   number(p,-2,(prefix+"before.value").c_str(),a.v);number(p,-2,(prefix+"before.error").c_str(),a.e);number(p,-2,(prefix+"before.set").c_str(),a.set);number(p,-2,(prefix+"before.fit").c_str(),a.fit);
   number(p,-2,(prefix+"after.value").c_str(),v.val[k]);number(p,-2,(prefix+"after.error").c_str(),v.err[k]);number(p,-2,(prefix+"after.set").c_str(),v.paramSet[k]);number(p,-2,(prefix+"after.fit").c_str(),v.fitFlag[k]);
  }
  number(p,-2,"mutations",mutations);
  for(int i=0;i<ps[p].nobs;i++)row(p,i,ps[p].obsn[i]);
  if(ps[p].refphs==REFPHS_TZR)row(p,-1,ps[p].tzrobs);
 }
 fclose(output);return 0;
}
