/* 0084 supplied-storage witness, reused with an external library linkage.
 * Driver from central-storage-preparation.py; no predictor algorithm copied. */
#include <assert.h>
#include <float.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tempo2pred_int.h"

static void initial(T1Polyco *p, int count, long double c1, double dm) {
    memset(p, 0, sizeof(*p));
    strcpy(p->psrname, "INITIAL"); strcpy(p->date_string, "DATE");
    strcpy(p->utc_string, "UTC"); strcpy(p->sitename, "@");
    p->mjd_mid=58000; p->reference_phase=7; p->frequency_psr_0=100;
    p->frequency_obs=1400; p->span=60; p->ncoeff=count; p->dm=dm;
    for(int i=0;i<32;i++)p->coeff[i]=(i+1)/16.0L;
    p->coeff[0]=.25L; p->coeff[1]=c1; p->coeff[2]=.5L;
}
static void read_case(FILE *out, const char *name, T1Polyco *p, const char *text) {
    FILE *input=tmpfile(); assert(input);
    assert(fwrite(text,1,strlen(text),input)==strlen(text)); rewind(input);
    long double tail[29]; memcpy(tail,p->coeff+3,sizeof(tail));
    int status=T1Polyco_Read_NewFormat(p,input);
    long position=ftell(input); assert(position>=0);
    int eof=feof(input), error=ferror(input);
    fprintf(out,"READ\t%s\t%d\t%ld\t%zu\t%d\t%La\t%La\t%La\t%a\t%a\t%La\t%La\t%La\t%d\t%d\t%d\t%s\n",
        name,status,position,strlen(text),p->ncoeff,p->mjd_mid,
        p->reference_phase,p->frequency_psr_0,p->dm,p->doppler,
        p->coeff[0],p->coeff[1],p->coeff[2],eof,error,
        memcmp(tail,p->coeff+3,sizeof(tail))==0,p->psrname);
    /* The caller can still inspect the borrowed stream and owns its close. */
    fprintf(out,"TAIL\t%s\t",name);
    int c;while((c=fgetc(input))!=EOF)fprintf(out,"%02x",(unsigned)c);
    fprintf(out,"\n");assert(!ferror(input));assert(fclose(input)==0);
}
int main(int argc,char **argv) {
    assert(argc==2);FILE *out=fopen(argv[1],"wb");assert(out);
    fprintf(out,"TYPE\t%d\t%zu\t%zu\n",LDBL_MANT_DIG,sizeof(long double),sizeof(T1Polyco));
    int counts[]={0,1,2,3,32};long double coeff1[]={0,15,30};
    for(int n=0;n<5;n++)for(int c=0;c<3;c++)for(int d=0;d<2;d++)for(int offset=-1;offset<=1;offset++) {
        T1Polyco p;initial(&p,counts[n],coeff1[c],d?8:0);
        unsigned char before[sizeof(p)];memcpy(before,&p,sizeof(p));
        long double mjd=p.mjd_mid+offset/1024.0L;
        long double phase=T1Polyco_GetPhase(&p,mjd,700);
        long double frequency=T1Polyco_GetFrequency(&p,mjd,700);
        fprintf(out,"EVAL\t%d\t%d\t%d\t%d\t%La\t%La\t%d\n",counts[n],c,d,offset,
            phase,frequency,memcmp(before,&p,sizeof(p))==0);
    }
    /* Frequency is a separate source operation: these fields are not read. */
    for(int n=0;n<5;n++) {
        T1Polyco p;initial(&p,counts[n],15,8);
        long double mjd=p.mjd_mid+1/1024.0L;
        long double original=T1Polyco_GetFrequency(&p,mjd,700);
        p.coeff[0]=NAN; p.reference_phase=NAN;
        p.dm=NAN; p.doppler=NAN; p.frequency_obs=NAN;
        long double after=T1Polyco_GetFrequency(&p,mjd,NAN);
        fprintf(out,"LEAF\t%d\t%La\t%La\n",counts[n],original,after);
    }
    for(int count=0;count<3;count++)for(int seed=15;seed<=30;seed+=15) {
        T1Polyco p;initial(&p,7,seed,8);char text[512],name[64];
        snprintf(name,sizeof(name),"count-%d-seed-%d",count,seed);
        const char *coeff=count==0?"":count==1?"0.5\n":"0.5 60\n";
        snprintf(text,sizeof(text),"READ DATE UTC 58000 0 0 -6 7 100 @ 60 %d 1400 0 0\n%sTAIL\n",count,coeff);
        read_case(out,name,&p,text);
        fprintf(out,"READFREQ\t%s\t%La\n",name,T1Polyco_GetFrequency(&p,58000,700));
    }
    T1Polyco p;initial(&p,7,15,8);
    read_case(out,"header-failure",&p,"BROKEN DATE UTC 59000 16 bad\nTAIL\n");
    read_case(out,"coefficient-failure",&p,"READ DATE UTC 59000 16 2 -6 11 200 @ 90 3 1400 0 0\n0.5 30 bad\nTAIL\n");
    read_case(out,"recovery",&p,"RECOVER DATE UTC 58001 4 0 -4 9 50 @ 60 2 1000 0 0\n0.125 60\nTAIL\n");
    fprintf(out,"READFREQ\trecovery\t%La\n",T1Polyco_GetFrequency(&p,58001,700));
    assert(fclose(out)==0);return 0;
}
