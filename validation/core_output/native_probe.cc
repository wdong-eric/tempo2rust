// AH typed, deterministic boundary witness; no timing-engine reimplementation.
#include "tempo2.h"
#include "TKlog.h"
#include <cmath>
#include <cstring>
#include <cstdlib>
#include <cstdio>

extern double m2(longdouble,longdouble,longdouble);
extern double dglep(const pulsar&,int,double);
extern void t2fit_prefit(pulsar*,int);
extern unsigned int t2Fit_getFitData(pulsar*,double*,double*,double*,int*);

int main(int argc,char** argv) {
    if(argc<2)return 2;
    if(!strcmp(argv[1],"format")) {
        const char* formats[]={"%Lg","%+.0Lf","% 020.8Lf","%-22.10Le","%#.6Lg","%.36Lg","%+.3Le"};
        const char* values[]={"0","-0","1.25","-0.125","65536","0x1p-20","1.9999847412109375","inf","-inf","nan"};
        for(auto f:formats)for(auto v:values){char b[4096];int n=ld_sprintf(b,f,parse_longdouble(v));printf("%s\t%s\t%d\t%s\n",f,v,n,b);}
        char b[4096];int n=ld_sprintf(b,"%+07d %#08x %.0u %-6.3s %% %c %.17g",-23,42u,0u,"abcdef",'X',-0.0);printf("mixed\t%d\t%s\n",n,b);
        n=ld_sprintf(b,"%hhd %hd %#.0x %#.0o %+u %08.4x",257,65537,0u,0u,42u,42u);printf("integers\t%d\t%s\n",n,b);
        return 0;
    }
    if(!strcmp(argv[1],"log")) {
        for(int i=0;i<32;i++){TK_STORE_WARNING("warn %d",i);}
        for(int i=0;i<31;i++){TK_STORE_ERROR("err %d",i);}
        printf("count=%d\n",logerr_check());
        TK_STORE_WARNING("after reset");printf("count=%d\n",logerr_check());
        _TKchklog(stderr,"\033[31mconfigured stdout controls this stderr colour\033[0m\n");
        char b[500];memset(b,'z',sizeof b);b[350]=27;b[351]='[';b[352]='3';b[353]='1';b[354]='m';b[499]=0;_TKchklog(stdout,"%s",b);
        return 0;
    }
    MAX_PSR=2;MAX_OBSN=!strcmp(argv[1],"ut1")?100:3;auto p=new pulsar[2];initialise(p,2);
    if(!strcmp(argv[1],"ut1")) {
        if(argc!=5)return 2;
        char par[1][MAX_FILELEN],tim[1][MAX_FILELEN];strcpy(par[0],argv[2]);strcpy(tim[0],argv[3]);
        readParfile(p,par,tim,1);readTimfile(p,tim,1);char name[]="ut1-witness";char* av[]={name};preProcess(p,1,1,av);
        int flag=atoi(argv[4]);for(int i=0;i<p[0].nobs;i++){p[0].obsn[i].clockCorr=flag;p[0].obsn[i].correctionUT1=-12345;}
        formBatsAll(p,1);formResiduals(p,1,1);
        FILE* state=fopen("state.tsv","w");FILE* actual=fopen("ut1.tsv","w");FILE* expected=fopen("expected.tsv","w");
        for(int i=0;i<p[0].nobs;i++){
            auto& o=p[0].obsn[i];
            ld_fprintf(state,"%d %.48Lg %.48Lg %.48Lg %.48Lg %.48Lg %.48Lg\n",i,o.sat,o.bat,o.bbat,o.pet,o.residual,o.phase);
            for(int k=0;k<3;k++)fprintf(state,"%.17g %.17g %.17g\n",o.observatory_earth[k],o.siteVel[k],o.zenith[k]);
            fprintf(actual,"%d %.17g\n",i,(double)o.correctionUT1);
            double clock=0;if(flag!=0&&flag!=2)clock=getCorrection(&o,p[0].clockFromOverride,"UTC",p[0].noWarnings);
            longdouble utc=o.sat;if(flag!=0&&flag!=2)utc+=clock/SECDAY;
            double xp,yp,dut1,dot;get_EOP((double)utc,&xp,&yp,&dut1,&dot,2,p[0].eopc04_file);
            fprintf(expected,"%d %.17g\n",i,clock+dut1);
        }
        fclose(state);fclose(actual);fclose(expected);return 0;
    }
    if(!strcmp(argv[1],"activation")) {
        p[0].nobs=3;p[0].globalNfit=-12345;
        for(auto label:{param_start,param_finish,param_ephver}) {p[0].param[label].prefit[0]=-12345;p[0].param[label].prefitErr[0]=-12345;}
        for(int i=0;i<3;i++){auto& o=p[0].obsn[i];o.deleted=0;o.sat=o.bat=o.bbat=50000+i;o.residual=0;o.toaErr=2;}
        preProcessSimple1(p,0,1);
        t2fit_prefit(p,1);
        double x[3],y[3],e[3];int ip[3];t2Fit_getFitData(p,x,y,e,ip);
        for(auto label:{param_start,param_finish,param_ephver}) ld_printf("activation %d %.36Lg %.36Lg %.36Lg\n",label,p[0].param[label].val[0],p[0].param[label].prefit[0],p[0].param[label].prefitErr[0]);
        printf("globalNfit %d\n",p[0].globalNfit);return 0;
    }
    if(!strcmp(argv[1],"snapshot") || !strcmp(argv[1],"snapshot-poison")) {
        if(argc!=3)return 2;
        char par[1][MAX_FILELEN],tim[1][MAX_FILELEN];strcpy(par[0],argv[2]);strcpy(tim[0],"");readParfile(p,par,tim,1);char name[]="ah-snapshot";char* av[]={name};preProcess(p,1,1,av);
        p[0].nobs=3;p[0].nFit=3;p[0].nParam=2;p[0].fitNfree=1;p[0].fitChisq=2;p[0].offset=0.125;p[0].offset_e=0.0625;p[0].globalNfit=2;p[0].nGlobal=1;
        if(!strcmp(argv[1],"snapshot-poison")){p[0].fitMode=0;for(auto label:{param_tzrmjd,param_tzrfrq,param_tres}){p[0].param[label].prefit[0]=-12345;p[0].param[label].prefitErr[0]=-12345;}}
        strcpy(p[0].rajStrPost,p[0].rajStrPre);strcpy(p[0].decjStrPost,p[0].decjStrPre);
        p[0].fitinfo.nParams=2;p[0].fitinfo.output.totalNfit=2;
        for(int i=0;i<2;i++){p[0].fitinfo.paramIndex[i]=i==0?param_ZERO:param_f;p[0].fitinfo.paramCounters[i]=0;p[0].fitinfo.output.indexParam[i]=i==0?param_f:param_ZERO;p[0].fitinfo.output.indexCounter[i]=0;p[0].fitinfo.output.indexPsr[i]=i==0?-1:0;for(int j=0;j<2;j++)p[0].covar[i][j]=4;}
        for(int i=0;i<3;i++){auto& o=p[0].obsn[i];o.deleted=0;o.sat=o.bat=50000+i;o.toaErr=2;o.freq=1408;o.prefitResidual=(i-1)*0.001953125;o.residual=(i-1)*0.0009765625;o.residualtn=(i-1)*0.00048828125;strcpy(o.telID,"@");}
        textOutput(p,1,0,0,1,1,"snapshot.par");
        FILE* state=fopen("state.tsv","w");ld_fprintf(state,"TZRMJD %.36Lg\nTZRFRQ %.36Lg\nTRES %.36Lg\nTRESTN %.36Lg\n",p[0].param[param_tzrmjd].val[0],p[0].param[param_tzrfrq].val[0],p[0].param[param_tres].val[0],p[0].param[param_trestn].val[0]);fclose(state);
        return 0;
    }
    if(!strcmp(argv[1],"helpers")) {
        for(int k=0;k<2;k++){
            auto& q=p[k];q.nobs=3;q.fitMode=1;q.param[param_f].val[0]=8;
            q.param[param_start].paramSet[0]=q.param[param_finish].paramSet[0]=1;
            q.param[param_start].fitFlag[0]=q.param[param_finish].fitFlag[0]=1;
            q.param[param_start].val[0]=50000+10*k;q.param[param_finish].val[0]=50002+10*k;
            for(int i=0;i<3;i++){auto& o=q.obsn[i];o.deleted=(i==2);o.sat=50000+10*k+i;o.bat=o.sat;o.toaErr=2;o.prefitResidual=(i-1)*0.0009765625;o.residual=i*0.000244140625;o.residualtn=i*0.0001220703125;}
            double w=calcRMS(p,k);printf("rms %d %.17g %.17g %.17g %.17g %d\n",k,w,q.rmsPre,q.rmsPost,q.rmstn,q.param[param_tres].paramSet[0]);
        }
        for(double si:{1.0,0.5,0.0}){double result=m2((longdouble)0.125,(longdouble)si,(longdouble)1.5);printf("m2 %.17g %.17g\n",si,result);}
        p[0].param[param_glf0].val[0]=0.0009765625;p[0].param[param_glf1].val[0]=0;p[0].param[param_glf0d].val[0]=0;p[0].param[param_gltd].val[0]=0;
        for(double phase:{0.0,0.125,-0.25})printf("glitch %.17g %.17g\n",phase,dglep(p[0],0,phase));
        return 0;
    }
    return 2;
}
