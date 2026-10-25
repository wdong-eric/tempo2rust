// AE public workflow observer: actual PAR/TIM, preprocessing, timing and t2Fit.
#include "tempo2.h"
#include "t2fit.h"
#include "enum_str.h"
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <vector>
#include <array>
static FILE* out;static int cycle;
int ae_solver_owner=-2;
static void value(const char* phase,int p,int row,const char* key,longdouble v){char text[128];quadmath_snprintf(text,sizeof text,"%.36Qg",v);fprintf(out,"%d\t%s\t%d\t%d\t%s\t%s\n",cycle,phase,p,row,key,text);}
void ae_update(pulsar* ps,int p,int label,int k,const char* stage){
 if(label!=param_f)return;
 char key[64];snprintf(key,sizeof key,"value.%d",k);value(stage,p,label,key,ps[p].param[label].val[k]);snprintf(key,sizeof key,"error.%d",k);value(stage,p,label,key,ps[p].param[label].err[k]);
}
void ae_svd(int n,int np,int nc,const longdouble* w,double tol){
 if(ae_solver_owner==-2)return;
 longdouble max=w[0];for(int i=0;i<np;i++)if(w[i]>max)max=w[i];
 value("svd",ae_solver_owner,-1,"ndata",n);value("svd",ae_solver_owner,-1,"nparams",np);value("svd",ae_solver_owner,-1,"nconstraints",nc);
 for(int i=0;i<np;i++){value("svd",ae_solver_owner,i,"raw",w[i]);value("svd",ae_solver_owner,i,"retained",w[i]!=0 && !(w[i]<tol*max));}
}
void ae_prepared(int p,pulsar* ps,const double* x,const double* y,const double* e,const int* ip,int n,double** m,double** w,double* wy,double** c){
 value("prepared",p,-1,"fit_mode",ps->fitMode);value("prepared",p,-1,"robust",ps->robust);value("prepared",p,-1,"nits",ps->nits);value("prepared",p,-1,"rescale_err_chisq",ps->rescaleErrChisq);
 auto& f=ps->fitinfo;value("prepared",p,-1,"ndata",n);value("prepared",p,-1,"nparams",f.nParams);value("prepared",p,-1,"nconstraints",f.nConstraints);
 for(int i=0;i<n;i++){value("prepared",p,i,"index",ip[i]);value("prepared",p,i,"x",x[i]);value("prepared",p,i,"y",y[i]);value("prepared",p,i,"sigma",e[i]);value("prepared",p,i,"raw_sigma",ps->obsn[ip[i]].toaErr*1e-6);value("prepared",p,i,"white_y",wy[i]);for(unsigned j=0;j<f.nParams;j++){char key[64];snprintf(key,sizeof key,"matrix.%u",j);value("prepared",p,i,key,m[i][j]);snprintf(key,sizeof key,"white_matrix.%u",j);value("prepared",p,i,key,w[i][j]);}}
 for(unsigned j=0;j<f.nParams;j++){value("column",p,j,"label",f.paramIndex[j]);value("column",p,j,"counter",f.paramCounters[j]);}
 for(unsigned i=0;i<f.nConstraints;i++){value("constraint",p,i,"label",f.constraintIndex[i]);value("constraint",p,i,"counter",f.constraintCounters[i]);value("constraint",p,i,"target",f.constraintValue[i]);for(unsigned j=0;j<f.nParams;j++){char key[64];snprintf(key,sizeof key,"matrix.%u",j);value("constraint",p,i,key,c[i][j]);}}
}
static void timing(pulsar* ps,int n,const char* stage){for(int p=0;p<n;p++){
 for(int i=0;i<ps[p].nobs;i++){auto&o=ps[p].obsn[i];value(stage,p,i,"residual",o.residual);value(stage,p,i,"phase",o.phase);value(stage,p,i,"bbat",o.bbat);value(stage,p,i,"pulse_number",o.pulseN);value(stage,p,i,"nphase",o.nphase);value(stage,p,i,"sat",o.sat);value(stage,p,i,"frequency",o.freq);value(stage,p,i,"uncertainty",o.toaErr);value(stage,p,i,"deleted",o.deleted);}
 for(int l=0;l<param_LAST;l++)for(int k=0;k<ps[p].param[l].aSize;k++)if(ps[p].param[l].paramSet[k]){char key[64];snprintf(key,sizeof key,"value.%d",k);value(stage,p,l,key,ps[p].param[l].val[k]);snprintf(key,sizeof key,"error.%d",k);value(stage,p,l,key,ps[p].param[l].err[k]);}
}}
int main(int argc,char**argv){
 if(argc<5)return 2;out=fopen(argv[1],"w");if(!out)return 3;int n=atoi(argv[2]);if(n<1||argc!=3+2*n)return 2;
 const bool ideal=getenv("AE_IDEAL")!=NULL;
 MAX_PSR=n;MAX_OBSN=512;auto*ps=new pulsar[n];initialise(ps,ideal?0:1);
 std::vector<std::array<char,MAX_FILELEN>> pars(n),tims(n);for(int p=0;p<n;p++){strcpy(pars[p].data(),argv[3+2*p]);strcpy(tims[p].data(),argv[4+2*p]);}
 readParfile(ps,reinterpret_cast<char(*)[MAX_FILELEN]>(pars.data()),reinterpret_cast<char(*)[MAX_FILELEN]>(tims.data()),n);readTimfile(ps,reinterpret_cast<char(*)[MAX_FILELEN]>(tims.data()),n);char*args[]={argv[0]};
 if(ideal){for(int p=0;p<n;p++){preProcessSimple(ps+p);ps[p].noWarnings=2;}}else preProcess(ps,n,1,args);
 const char*cov=getenv("AE_COVARIANCE");for(int p=0;p<n;p++)ps[p].rescaleErrChisq=getenv("AE_NORESCALE")?0:1;
 if(const char*flag=getenv("AE_WRITE_RESIDUALS"))writeResiduals=atoi(flag);
 int nits=ps[0].nits;
 for(cycle=1;cycle<=nits;cycle++){
  if(cycle>1)for(int p=0;p<n;p++)for(int l=0;l<param_LAST;l++)for(int k=0;k<ps[p].param[l].aSize;k++){ps[p].param[l].prefit[k]=ps[p].param[l].val[k];ps[p].param[l].prefitErr[k]=ps[p].param[l].err[k];}
  value("workflow",-1,-1,"remove_mean",ideal?0:1);
  formBatsAll(ps,n);formResiduals(ps,n,ideal?0:1);timing(ps,n,"prefit");t2Fit(ps,n,ideal?"NULL":cov);
  for(int p=0;p<n;p++){auto&f=ps[p].fitinfo;value("fit",p,-1,"chi",ps[p].fitChisq);value("fit",p,-1,"dof",ps[p].fitNfree);value("fit",p,-1,"detL",ps[p].detL);value("fit",p,-1,"total",f.output.totalNfit);
   for(int j=0;j<f.output.totalNfit;j++){value("fit",p,j,"owner",f.output.indexPsr[j]);value("fit",p,j,"label",f.output.indexParam[j]);value("fit",p,j,"counter",f.output.indexCounter[j]);value("fit",p,j,"increment",f.output.parameterEstimates[j]);value("fit",p,j,"error",f.output.errorEstimates[j]);for(int k=0;k<f.output.totalNfit;k++){char key[64];snprintf(key,sizeof key,"covariance.%d",k);value("fit",p,j,key,ps[p].covar[j][k]);}}
   for(int i=0;i<ps[p].nobs;i++){auto&o=ps[p].obsn[i];value("noise",p,i,"red",o.TNRedSignal);value("noise",p,i,"red_error",o.TNRedErr);value("noise",p,i,"dm",o.TNDMSignal);value("noise",p,i,"dm_error",o.TNDMErr);value("noise",p,i,"chrom",o.TNChromSignal);value("noise",p,i,"chrom_error",o.TNChromErr);value("noise",p,i,"sw",o.SWGPSignal);value("noise",p,i,"sw_error",o.SWGPErr);}
  }
  formBatsAll(ps,n);formResiduals(ps,n,ideal?0:1);timing(ps,n,"postfit");
 }
 fclose(out);return 0;
}
