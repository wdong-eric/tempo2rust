/* Separately compiled C11/C++ client. Assertions protect public ownership/state;
 * printed native values compare against the source library in another process. */
#include "tempo2pred_int.h"
#include <assert.h>
#include <float.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#include <stddef.h>
#include <dlfcn.h>

static int calls;
static void surface(long double *x,long double *y,int nx,int ny,long double *z,void *info) {
    assert(info==&calls); ++calls;
    for(int j=0;j<ny;++j) for(int i=0;i<nx;++i) z[j*nx+i]=1+2*x[i]+3*y[j]+x[i]*y[j];
}
static FILE *stream(const char *s) {
    FILE *f=tmpfile(); assert(f); assert(fwrite(s,1,strlen(s),f)==strlen(s));rewind(f);return f;
}
static void value(const char *name,long double x){printf("V %s %La\n",name,x);}
static void model(ChebyModel *m,long double start) {
    ChebyModel_Init(m,4,2);strcpy(m->psrname,"JTEST");strcpy(m->sitename,"PKS");
    m->mjd_start=start;m->mjd_end=start+1;m->freq_start=1000;m->freq_end=2000;m->dispersion_constant=8;
    Cheby2D_Construct(&m->cheby,surface,&calls);
    Cheby2D_Construct_x_Derivative(&m->frequency_cheby,&m->cheby);
}
static void t1init(T1Polyco *p) {
    memset(p,0,sizeof(*p));strcpy(p->psrname,"JTEST");strcpy(p->sitename,"@");
    strcpy(p->date_string,"01-JAN-00");strcpy(p->utc_string,"000000.00");
    p->mjd_mid=58000;p->reference_phase=7;p->frequency_psr_0=100;
    p->frequency_obs=1400;p->ncoeff=3;p->span=60;p->coeff[0]=.25;p->coeff[1]=15;p->coeff[2]=.5;
}
int main(int argc,char **argv){
    if(argc==2 && !strncmp(argv[1],"unsupported-",12)) {
        T2Predictor p,q;T2Predictor_Init(&p);T2Predictor_Init(&q);p.kind=T1;
        if(!strcmp(argv[1],"unsupported-copy"))T2Predictor_Copy(&q,&p);
        else if(!strcmp(argv[1],"unsupported-insert"))T2Predictor_Insert(&q,&p);
        else T2Predictor_Keep(&p,0,NULL);
        return 99;
    }
    if(argc==2 && !strcmp(argv[1],"write-failure")) {
        T2Predictor p;T2Predictor_Init(&p);T2Predictor_Write(&p,(char*)"absent-dir/predictor");return 99;
    }
    if(argc==3 && !strcmp(argv[1],"exchange")) {
        T2Predictor p; T2Predictor_Init(&p);assert(T2Predictor_Read(&p,argv[2])==0);
        value("exchange-phase",T2Predictor_GetPhase(&p,58000.001L,1400));
        value("exchange-frequency",T2Predictor_GetFrequency(&p,58000.001L,1400));
        T2Predictor_Write(&p,(char*)"exchanged.dat");T2Predictor_Destroy(&p);return 0;
    }
    if(argc==2 && !strcmp(argv[1],"compiled-test")) {testCheby2D();return 0;}
    assert(argc==1);
    printf("ABI %d %zu %zu %zu %zu %zu %zu %zu %zu %zu %zu %zu %d %d %d\n",LDBL_MANT_DIG,
        sizeof(long double),sizeof(Cheby2D),sizeof(ChebyModel),sizeof(ChebyModelSet),sizeof(T1Polyco),sizeof(T1PolycoSet),sizeof(T2Predictor),
        offsetof(T1Polyco,mjd_mid),offsetof(T1Polyco,coeff),offsetof(ChebyModel,cheby),offsetof(T2Predictor,modelset),NonePredType,Cheby,T1);
    assert(tempo2_verbose==1);tempo2_verbose=0;
    Dl_info loaded;assert(dladdr((void*)&T2Predictor_Init,&loaded));fprintf(stderr,"LOADED %s\n",loaded.dli_fname);
    Cheby2D empty;Cheby2D_Init(&empty,0,3);assert(empty.nx==0 && empty.ny==3 && empty.coeff==NULL);Cheby2D_Destroy(&empty);
    Cheby2D_Init(&empty,4,0);assert(empty.nx==4 && empty.ny==0 && empty.coeff==NULL);Cheby2D_Destroy(&empty);
    Cheby2D p,d,c;Cheby2D_Init(&p,4,2);Cheby2D_Init(&d,4,2);Cheby2D_Init(&c,0,0);
    Cheby2D_Construct(&p,surface,&calls);assert(calls==1);
    for(int i=0;i<8;++i)value("coefficient",p.coeff[i]);
    Cheby2D_Construct_x_Derivative(&d,&p);Cheby2D_Copy(&c,&p);
    for(int i=0;i<8;++i)value("derivative-coefficient",d.coeff[i]);
    long double rms,mav;Cheby2D_Test(&p,5,3,surface,&calls,&rms,&mav);assert(calls==2);
    value("rms",rms);value("mav",mav);value("polynomial",Cheby2D_Evaluate(&p,.25L,-.5L));
    value("derivative",Cheby2D_Evaluate(&d,.25L,-.5L));
    Cheby2D_Destroy(&p);assert(p.coeff==NULL && p.nx==0 && p.ny==0);
    value("copy",Cheby2D_Evaluate(&c,.25L,-.5L));
#ifndef NATIVE
    Cheby2D_Copy(&c,&c);value("self-copy",Cheby2D_Evaluate(&c,.25L,-.5L));
#endif
    Cheby2D_Destroy(&c);Cheby2D_Destroy(&d);
    long double xx[]={.25L},yy[]={-.5L},zz[1];testFunc(xx,yy,1,1,zz,NULL);value("testFunc",zz[0]);
    ChebyModel m,clone;model(&m,58000);ChebyModel_Init(&clone,0,0);ChebyModel_Copy(&clone,&m);
    assert(clone.cheby.coeff!=m.cheby.coeff);
#ifndef NATIVE
    ChebyModel_Copy(&clone,&clone);assert(clone.cheby.nx==4);
#endif
    value("model-phase",ChebyModel_GetPhase(&m,58000.25L,1400));
    value("model-frequency",ChebyModel_GetFrequency(&m,58000.25L,1400));
    FILE *f=tmpfile();assert(f);ChebyModel_Write(&m,f);rewind(f);assert(ChebyModel_Read(&clone,f)==0);assert(!ferror(f));fclose(f);
    value("model-exchange",ChebyModel_GetPhase(&clone,58000.25L,1400));ChebyModel_Destroy(&m);ChebyModel_Destroy(&clone);
    T2Predictor q,r;T2Predictor_Init(&q);T2Predictor_Init(&r);assert(T2Predictor_Kind(&q)==NonePredType);
    q.kind=Cheby;q.modelset.cheby.nsegments=3;q.modelset.cheby.segments=(ChebyModel*)malloc(3*sizeof(ChebyModel));assert(q.modelset.cheby.segments);
    model(&q.modelset.cheby.segments[0],58000);model(&q.modelset.cheby.segments[1],58000);model(&q.modelset.cheby.segments[2],58002);
    assert(ChebyModelSet_GetNearestIndex(&q.modelset.cheby,57999)==-1 && ChebyModelSet_OutOfRange==0);
    assert(ChebyModelSet_GetNearest(&q.modelset.cheby,57999)==NULL && ChebyModelSet_OutOfRange==1);
    assert(ChebyModelSet_GetNearest(&q.modelset.cheby,58000.5L)==q.modelset.cheby.segments && ChebyModelSet_OutOfRange==0);
    assert(ChebyModelSet_GetNearest(&q.modelset.cheby,58002.5L)==q.modelset.cheby.segments+2);
    assert(T2Predictor_GetPSRName(&q)==q.modelset.cheby.segments[0].psrname);
    assert(T2Predictor_GetSiteName(&q)==q.modelset.cheby.segments[0].sitename);
    T2Predictor_GetPSRName(&q)[1]='X';assert(!strcmp(q.modelset.cheby.segments[0].psrname,"JXEST"));
    value("start",T2Predictor_GetStartMJD(&q));value("end",T2Predictor_GetEndMJD(&q));
    value("fstart",T2Predictor_GetStartFreq(&q));value("fend",T2Predictor_GetEndFreq(&q));
    value("set-phase",T2Predictor_GetPhase(&q,58000.25L,1400));value("set-frequency",T2Predictor_GetFrequency(&q,58000.25L,1400));
    value("miss",T2Predictor_GetFrequency(&q,58001.5L,1400));assert(ChebyModelSet_OutOfRange==1);
    T2Predictor_Copy(&r,&q);assert(r.modelset.cheby.segments!=q.modelset.cheby.segments);
    assert(T2Predictor_Insert(&r,&q)==0 && r.modelset.cheby.nsegments==6);
    long double keep[]={58000.25L,58002.25L};T2Predictor_Keep(&r,2,keep);assert(r.modelset.cheby.nsegments==2);
    T2Predictor_Write(&r,(char*)"cheby.dat");T2Predictor_Destroy(&q);
    assert(T2Predictor_Read(&q,(char*)"cheby.dat")==0 && T2Predictor_Kind(&q)==Cheby);
    f=tmpfile();assert(f);T2Predictor_FWrite(&q,f);long end=ftell(f);fputs("TAIL\n",f);rewind(f);
    assert(T2Predictor_FRead(&r,f)==0 && ftell(f)==end);assert(fgetc(f)=='T');fclose(f);
    long double phase,freqs[100];int count;char name[64],site[64];
    assert(T2Predictor_GetPlan((char*)"cheby.dat",58000,58000.01L,300,1400,&phase,&count,freqs)==0 && count==3);
    value("plan-phase",phase);for(int i=0;i<count;++i)value("plan-frequency",freqs[i]);
    assert(T2Predictor_GetPlan_Ext((char*)"cheby.dat",58000,58000.01L,300,1400,name,site,&phase,&count,freqs)==0);
    assert(!strcmp(name,"JXEST") && !strcmp(site,"PKS"));
#ifndef NATIVE
    T2Predictor_Copy(&q,&q);assert(q.modelset.cheby.nsegments==2);
    assert(T2Predictor_Insert(&q,&q)==0 && q.modelset.cheby.nsegments==4);
    f=stream("not a predictor\n");assert(T2Predictor_FRead(&q,f)==-1 && ftell(f)==0 && q.modelset.cheby.nsegments==4);fclose(f);
    assert(T2Predictor_Read(&q,(char*)"cheby.dat")==0 && q.modelset.cheby.nsegments==2);
    assert(T2Predictor_GetPlan((char*)"cheby.dat",58000,58001,0,1400,&phase,&count,freqs)==-1);
    f=stream("ChebyModelSet 2 segments\nChebyModel BEGIN\nNCOEFF_TIME 4\nNCOEFF_FREQ 2\nCOEFFS 1 bad\n");
    ChebyModelSet bad;ChebyModelSet_Init(&bad);assert(ChebyModelSet_Read(&bad,f)!=0);ChebyModelSet_Destroy(&bad);fclose(f);
#endif
    T2Predictor_Destroy(&q);T2Predictor_Destroy(&r);assert(q.kind==NonePredType);
    T1Polyco t;t1init(&t);ChebyModelSet_OutOfRange=17;
    value("t1-phase",T1Polyco_GetPhase(&t,58000.001L,700));
    value("t1-frequency",T1Polyco_GetFrequency(&t,58000.001L,700));assert(ChebyModelSet_OutOfRange==17);
    f=fopen("t1.dat","w");assert(f);T1Polyco_Write(&t,f);fclose(f);
    T1Polyco copy;t1init(&copy);f=fopen("t1.dat","r");assert(f && T1Polyco_Read(&copy,f)==0);fclose(f);
    value("t1-exchange",T1Polyco_GetFrequency(&copy,58000.001L,700));
    assert(T2Predictor_Read(&q,(char*)"t1.dat")==0 && q.kind==T1);
    assert(T1PolycoSet_GetNearest(&q.modelset.t1,60000)==q.modelset.t1.segments);
    assert(T2Predictor_GetPSRName(&q)==q.modelset.t1.segments[0].psrname);
    assert(T2Predictor_GetSiteName(&q)==q.modelset.t1.segments[0].sitename);
    value("t1-start",T2Predictor_GetStartMJD(&q));value("t1-end",T2Predictor_GetEndMJD(&q));
    value("t1-fstart",T2Predictor_GetStartFreq(&q));value("t1-fend",T2Predictor_GetEndFreq(&q));
    value("t1-set-phase",T2Predictor_GetPhase(&q,58000.001L,700));value("t1-set-frequency",T2Predictor_GetFrequency(&q,58000.001L,700));
    f=tmpfile();T2Predictor_FWrite(&q,f);assert(!ferror(f));fclose(f);
    assert(T2Predictor_GetPlan_Ext((char*)"t1.dat",58000,58000.001L,60,1400,name,site,&phase,&count,freqs)==0 && count==2);
    value("t1-plan-phase",phase);for(int i=0;i<count;++i)value("t1-plan-frequency",freqs[i]);
#ifndef NATIVE
    /* Replacing an existing T1 by Cheby is the corrected wrong-union case. */
    assert(T2Predictor_Read(&q,(char*)"cheby.dat")==0 && q.kind==Cheby);
#endif
    T2Predictor_Destroy(&q);T2Predictor_Destroy(&q);
    char text[]=" 12.5D+1 ",word[64];T1P_grabString(text,0,9,word);assert(!strcmp(word,"12.5D+1"));
    value("grab-double",T1P_grabLongDouble(text,0,9));char integer[]=" 123 ";value("grab-int",T1P_grabInt(integer,0,5));
    value("parse-ld",parse_ld_cheby("-0x1.000000000000001p-10"));
    puts("PASS");return 0;
}
