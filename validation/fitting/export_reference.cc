// Assignment E reference observer. Calls the unchanged A2 TEMPO2 library.
// No replacement fitting equations: columns, updates, SVD and solves are source calls.
#include <cstdio>
#include <cstdlib>
#include <cmath>
#include <cstring>
#include <fstream>
#include <string>
#include <vector>
#include <stdexcept>
#include "tempo2.h"
#include "TKfit.h"
#include "TKsvd.h"
#include "TKmatrix.h"
#include "t2fit.h"
#include "t2fit_stdFitFuncs.h"
#include "resource_trace.h"
using std::string;
static string number(longdouble x) {
    if (isnanq(x)) return "\"NaN\"";
    if (isinfq(x)) return x > 0 ? "\"+Infinity\"" : "\"-Infinity\"";
    char b[160]; quadmath_snprintf(b,sizeof b,"%.36Qg",x);
    if (strtoflt128(b,NULL)!=x) throw std::runtime_error("decimal round trip");
    return string("\"")+b+"\"";
}
template<class T> static string array(const T* a,int n) {
    string s="["; for(int i=0;i<n;i++) { if(i)s+=","; s+=number(a[i]); } return s+"]";
}
static string matrix(double** a,int n,int m) {
    string s="["; for(int i=0;i<n;i++){if(i)s+=",";s+=array(a[i],m);}return s+"]";
}
static string diagnostics(double** aw,int n,int m) {
    auto a=malloc_2dLL(n,m),v=malloc_2dLL(m,m),u=malloc_2dLL(n,m);
    std::vector<longdouble>w(m);
    for(int i=0;i<n;i++)for(int j=0;j<m;j++)a[i][j]=aw[i][j];
    TKsingularValueDecomposition_lsq(a,n,m,v,w.data(),u);
    longdouble wm=TKfindMax_Ld(w.data(),m),cut=(longdouble)(double)1e-27*wm;
    string s="\"singular_values\":"+array(w.data(),m)+",\"retained\":[";int rank=0;
    for(int j=0;j<m;j++){if(j)s+=",";bool keep=!(w[j]<cut)&&w[j]!=0;s+=keep?"true":"false";rank+=keep;}
    s+="],\"rank\":"+std::to_string(rank);
    free_2dLL(a);free_2dLL(v);free_2dLL(u);return s;
}
static string solved(double** a,double** aw,double* y,int n,int m,double* p,double* e,double** cov,double chi,bool rescale) {
    std::vector<double>raw(m),lin(n);for(int j=0;j<m;j++)raw[j]=sqrt(cov[j][j]);
    for(int i=0;i<n;i++){double sum=0;for(int j=0;j<m;j++)sum+=a[i][j]*p[j];lin[i]=y[i]-sum;}
    double multiplier=rescale?sqrt(chi/(n-m)):1;
    return "{"+diagnostics(aw,n,m)+",\"dof\":"+std::to_string(n-m)+",\"increments\":"+array(p,m)+",\"errors\":"+array(e,m)+",\"errors_unscaled\":"+array(raw.data(),m)+",\"covariance\":"+matrix(cov,m,m)+",\"chi_square\":"+number(chi)+",\"error_multiplier\":"+number(multiplier)+",\"linearized_s\":"+array(lin.data(),n)+"}";
}
static string snapshot(pulsar* p) {
    formBatsAll(p,1);formResiduals(p,1,0);
    std::vector<longdouble>raw(p->nobs);longdouble mean=0;
    string rows="[";
    for(int i=0;i<p->nobs;i++){
        auto &o=p->obsn[i];raw[i]=o.residual;mean+=o.residual;
        if(i)rows+=",";
        rows+="{\"index\":"+std::to_string(i)+",\"sat_mjd\":"+number(o.sat)+",\"bat_mjd\":"+number(o.bat)+",\"bbat_mjd\":"+number(o.bbat)+",\"phase_turn\":"+number(o.phase)+",\"nphase_turn\":\""+std::to_string((long long)o.nphase)+"\",\"pulse_number\":\""+std::to_string(o.pulseN)+"\",\"residual_s\":"+number(o.residual)+"}";
    }
    mean/=p->nobs;
    formResiduals(p,1,1);std::vector<longdouble>center(p->nobs);
    for(int i=0;i<p->nobs;i++)center[i]=p->obsn[i].residual;
    return "{\"mean_s\":"+number(mean)+",\"observations\":"+rows+"],\"centered_s\":"+array(center.data(),p->nobs)+"}";
}
static pulsar* load(const char* parpath,const char* timpath) {
    char par[1][MAX_FILELEN],tim[1][MAX_FILELEN];
    if(strlen(parpath)>=MAX_FILELEN||strlen(timpath)>=MAX_FILELEN)throw std::runtime_error("path length");
    strcpy(par[0],parpath);strcpy(tim[0],timpath);MAX_PSR=1;
    auto p=new pulsar;initialise(p,0);readParfile(p,par,tim,1);readTimfile(p,tim,1);return p;
}
int main(int argc,char**argv) {
    if(argc<4)return 2;
    std::ofstream out(argv[argc-1]);if(!out)return 2;
    reference_stage("fitting");
    string action=argv[1];
    if(action=="solver"){
        std::ifstream in(argv[2]);int n,m,mode,rescale;in>>n>>m>>mode>>rescale;
        auto a=malloc_blas(n,m),aw=malloc_blas(n,m),cov=malloc_blas(m,m);
        std::vector<double>y(n),yw(n),sigma(n),p(m,12345),e(m,12345);
        for(int i=0;i<n;i++){in>>y[i]>>sigma[i];double den=mode?sigma[i]:1;yw[i]=y[i]/den;for(int j=0;j<m;j++){in>>a[i][j];aw[i][j]=a[i][j]/den;}}
        if(!in)throw std::runtime_error("solver input");
        double chi=TKleastSquares(y.data(),yw.data(),a,aw,n,m,1e-27,rescale,p.data(),e.data(),cov);
        if(n<m){out<<"{\"source_outcome\":\"returns_without_initializing_outputs\",\"sentinels_unchanged\":"<<((p[0]==12345&&e[0]==12345)?"true":"false")<<"}";return 0;}
        out<<"{\"matrix\":"<<matrix(a,n,m)<<",\"white_matrix\":"<<matrix(aw,n,m)<<",\"data_s\":"<<array(y.data(),n)<<",\"white_data\":"<<array(yw.data(),n)<<",\"sigma_s\":"<<array(sigma.data(),n)<<",\"mode\":"<<mode<<",\"rescale_effective\":"<<(rescale?"true":"false")<<",\"solver\":"<<solved(a,aw,y.data(),n,m,p.data(),e.data(),cov,chi,rescale)<<"}\n";
        return 0;
    }
    if(action=="callbacks"){
        MAX_PSR=1;auto p=new pulsar;initialise(p,0);
        const char* vals[]={"123.45678901234567890123456789","-1e-15","2e-25","-3e-35"};
        for(int k=0;k<4;k++)p->param[param_f].val[k]=parse_longdouble(vals[k]);
        double x=-1234.56789,cols[5]={1},inc[4],errs[4];
        for(int k=0;k<4;k++){cols[k+1]=t2FitFunc_stdFreq(p,0,x,0,param_f,k);inc[k]=.25*(k+1);errs[k]=.1*(k+1);}
        out<<"{\"x_days\":"<<number(x)<<",\"columns\":"<<array(cols,5)<<",\"before\":"<<array(p->param[param_f].val,4)<<",\"increments\":"<<array(inc,4)<<",\"errors\":"<<array(errs,4);
        for(int k=0;k<4;k++)t2UpdateFunc_stdFreq(p,0,param_f,k,inc[k],errs[k]);
        out<<",\"after\":"<<array(p->param[param_f].val,4)<<",\"physical_errors\":"<<array(p->param[param_f].err,4);
        for(int k=0;k<4;k++)t2UpdateFunc_stdFreq(p,0,param_f,k,0,NAN);
        out<<",\"after_nan_errors\":"<<array(p->param[param_f].err,4);
        for(int k=0;k<4;k++)t2UpdateFunc_stdFreq(p,0,param_f,k,0,INFINITY);
        out<<",\"after_positive_infinity_errors\":"<<array(p->param[param_f].err,4);
        for(int k=0;k<4;k++)t2UpdateFunc_stdFreq(p,0,param_f,k,0,-INFINITY);
        out<<",\"after_negative_infinity_errors\":"<<array(p->param[param_f].err,4)<<"}";return 0;
    }
    if(argc!=6)return 2;
    auto p=load(argv[2],argv[3]);
    if(action=="align"){
        // Source-driven alignment: only the disposable input SATs change.
        for(int it=0;it<12;it++){
            formBatsAll(p,1);formResiduals(p,1,0);
            for(int i=1;i<p->nobs;i++)p->obsn[i].sat-=p->obsn[i].residual/86400;
        }
        formBatsAll(p,1);formResiduals(p,1,0);longdouble maxerr=0;
        out<<"FORMAT 1\n";
        const int perturb[]={0,3,-2,4,1,-5,6,-3,2,-1,5,-4,1,2,-2,3};
        for(int i=0;i<p->nobs;i++){
            if(fabs(p->obsn[i].residual)>maxerr)maxerr=fabs(p->obsn[i].residual);
            longdouble sat=p->obsn[i].sat+(longdouble)perturb[i%16]*1e-6Q/86400;
            auto ns=number(sat);out<<"toa"<<i<<" 1400 "<<ns.substr(1,ns.size()-2)<<" "<<(1+i%5)<<" "<<p->obsn[i].telID<<"\n";
        }
        fprintf(stderr,"alignment_max_residual_s=%s\n",number(maxerr).c_str());return 0;
    }
    p->rescaleErrChisq=atoi(argv[4]);bool offset_known=false;
    out<<"{\"mode\":"<<p->fitMode<<",\"nits\":"<<p->nits<<",\"rescale_requested\":"<<(p->rescaleErrChisq?"true":"false")<<",\"columns\":[\"ZERO\"";
    for(int k=0;k<4;k++)if(p->param[param_f].fitFlag[k]==1)out<<",\"F"<<k<<"\"";
    out<<"],\"cycles\":[";
    for(int it=0;it<p->nits;it++){
        if(it) {out<<",";for(int k=0;k<4;k++){p->param[param_f].prefit[k]=p->param[param_f].val[k];p->param[param_f].prefitErr[k]=p->param[param_f].err[k];}}
        out<<"{\"iteration\":"<<it+1<<",\"before_values\":"<<array(p->param[param_f].val,4)<<",\"before_errors\":"<<array(p->param[param_f].err,4)<<",\"prefit\":"<<snapshot(p);
        int n=p->nobs;std::vector<double>x(n),y(n),sigma(n),yw(n);std::vector<int>idx(n);
        t2Fit_getFitData(p,x.data(),y.data(),sigma.data(),idx.data());
        FitInfo globals;t2Fit_fillGlobalFitInfo(p,1,globals);t2Fit_fillFitInfo(p,p->fitinfo,globals,x.data(),idx.data(),n);
        int m=p->fitinfo.nParams;auto a=malloc_blas(n,m),aw=malloc_blas(n,m);
        for(int i=0;i<n;i++){t2Fit_buildDesignMatrix(p,0,x[i],idx[i],a[i]);double den=p->fitMode?sigma[i]:1;yw[i]=y[i]/den;for(int j=0;j<m;j++)aw[i][j]=a[i][j]/den;}
        out<<",\"x_days\":"<<array(x.data(),n)<<",\"data_s\":"<<array(y.data(),n)<<",\"sigma_s\":"<<array(sigma.data(),n)<<",\"white_data\":"<<array(yw.data(),n)<<",\"matrix\":"<<matrix(a,n,m)<<",\"white_matrix\":"<<matrix(aw,n,m);
        t2Fit(p,1,NULL);
        bool rescale=p->fitMode==0||(p->fitMode==1&&p->rescaleErrChisq);
        double *est=p->fitinfo.output.parameterEstimates,*err=p->fitinfo.output.errorEstimates;
        out<<",\"solver\":"<<solved(a,aw,y.data(),n,m,est,err,p->covar,p->fitChisq,rescale)<<",\"after_values\":"<<array(p->param[param_f].val,4)<<",\"after_errors\":"<<array(p->param[param_f].err,4)<<",\"offset_s\":"<<number(p->offset);
        offset_known|=err[0]>=0;
        out<<",\"offset_error_s\":"<<(offset_known?number(p->offset_e):"null")<<",\"postfit\":"<<snapshot(p)<<"}";
        free_blas(a);free_blas(aw);
    }
    out<<"]}\n";destroyOne(p);delete p;return 0;
}
