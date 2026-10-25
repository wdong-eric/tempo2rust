// AC integration probe: real source file readers, preprocess and formBatsAll.
// No harness-supplied physical vectors, clocks, or source parameter values.
#include "tempo2.h"
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>
#include <vector>
#include <array>
static FILE* output;
static void field(int p,int i,const char* key,const std::string& value){fprintf(output,"%d\t%d\t%s\t%s\n",p,i,key,value.c_str());}
static void number(int p,int i,const char* key,longdouble value){char s[128];quadmath_snprintf(s,sizeof s,"%.36Qg",value);field(p,i,key,s);}
static void row(int p,int i,const observation& o){
 field(p,i,"filename",o.fname);field(p,i,"site",o.telID);
 number(p,i,"deleted",o.deleted);number(p,i,"clock_correction",o.clockCorr);number(p,i,"delay_correction",o.delayCorr);
 number(p,i,"sat_mjd",o.sat);number(p,i,"bat_mjd",o.bat);number(p,i,"bbat_mjd",o.bbat);
 number(p,i,"frequency",o.freq);number(p,i,"frequency_ssb",o.freqSSB);number(p,i,"uncertainty",o.toaErr);
 number(p,i,"nFlags",o.nFlags);for(int k=0;k<o.nFlags;k++){field(p,i,("flag_id_"+std::to_string(k)).c_str(),o.flagID[k]);field(p,i,("flag_value_"+std::to_string(k)).c_str(),o.flagVal[k]);}
 number(p,i,"nclock",o.nclock_correction);for(int k=0;k<o.nclock_correction;k++){field(p,i,("clock_to_"+std::to_string(k)).c_str(),o.correctionsTT[k].corrects_to);number(p,i,("clock_s_"+std::to_string(k)).c_str(),o.correctionsTT[k].correction);}
 number(p,i,"shklovskii",o.shklovskii);
 if(o.delayCorr){
  number(p,i,"tt_to_tb",o.correctionTT_TB);number(p,i,"tt_to_teph",o.correctionTT_Teph);number(p,i,"roemer",o.roemer);
  number(p,i,"dispersion_chromatic",o.tdis1);number(p,i,"solar_wind",o.tdis2);number(p,i,"troposphere",o.troposphericDelay);number(p,i,"bat_correction",o.batCorr);
  number(p,i,"shapiro_sun",o.shapiroDelaySun);number(p,i,"shapiro_jupiter",o.shapiroDelayJupiter);number(p,i,"shapiro_saturn",o.shapiroDelaySaturn);number(p,i,"shapiro_venus",o.shapiroDelayVenus);number(p,i,"shapiro_uranus",o.shapiroDelayUranus);number(p,i,"shapiro_neptune",o.shapiroDelayNeptune);
 }
}
int main(int argc,char**argv){
 if(argc<5)return 2;
 output=fopen(argv[1],"w");if(!output)return 3;
 int n=atoi(argv[2]);if(n<1||argc<3+2*n)return 2;
 MAX_PSR=n;MAX_OBSN=64;
 auto* ps=new pulsar[n];initialise(ps,1);
 std::vector<std::array<char,MAX_FILELEN>> pars(n),tims(n);
 for(int p=0;p<n;p++){strcpy(pars[p].data(),argv[3+2*p]);strcpy(tims[p].data(),argv[4+2*p]);}
 readParfile(ps,reinterpret_cast<char(*)[MAX_FILELEN]>(pars.data()),reinterpret_cast<char(*)[MAX_FILELEN]>(tims.data()),n);
 readTimfile(ps,reinterpret_cast<char(*)[MAX_FILELEN]>(tims.data()),n);
 std::vector<char*> options;options.push_back(argv[0]);for(int k=3+2*n;k<argc;k++)options.push_back(argv[k]);
 preProcess(ps,n,options.size(),options.data());formBatsAll(ps,n);
 for(int p=0;p<n;p++){
  field(p,-2,"name",ps[p].name);number(p,-2,"units",ps[p].units);number(p,-2,"frame",ps[p].eclCoord);number(p,-2,"time_ephemeris",ps[p].timeEphemeris);number(p,-2,"nobs",ps[p].nobs);
  for(int i=0;i<ps[p].nobs;i++)row(p,i,ps[p].obsn[i]);
  if(ps[p].refphs==REFPHS_TZR)row(p,-1,ps[p].tzrobs);
 }
 fclose(output);return 0;
}
