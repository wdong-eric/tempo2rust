/* Central 27 October low-count set-reader regression, extended to C11.
 * Only metadata is read before the caller initializes required C0/C1 backing. */
#include "tempo2pred_int.h"
#include <assert.h>
#include <stdlib.h>
#include <string.h>
static void record(FILE *f,int count) {
    fprintf(f,"TEMPO2:\nREAD DATE UTC 58000 0 0 -6 7 100 @ 60 %d 1400 0 0\n",count);
    if(count==1) fputs("0.5\n",f);
    if(count==2) fputs("0.5 60\n",f);
}
static void check(int count,int prefix,int generic) {
    FILE *f=tmpfile();assert(f);if(prefix)record(f,2);record(f,count);rewind(f);
    T1PolycoSet set;T1PolycoSet_Init(&set);T2Predictor p;T2Predictor_Init(&p);
    int status=generic ? T2Predictor_FRead(&p,f) : T1PolycoSet_Read(&set,f);
    T1PolycoSet *v=generic ? &p.modelset.t1 : &set;
    assert(status==0 && v->nsegments==1+prefix);
    printf("READ %d %d %d %d %d %ld %d %d\n",count,prefix,generic,status,v->nsegments,ftell(f),feof(f),ferror(f));
    if(generic) {assert(p.kind==T1);assert(!strcmp(T2Predictor_GetPSRName(&p),"READ"));assert(!strcmp(T2Predictor_GetSiteName(&p),"@"));}
    for(int i=0;i<v->nsegments;++i){
        T1Polyco *s=v->segments+i;assert(s->ncoeff==(prefix && i==0 ? 2 : count));
        printf("META %d %d %d %d %d %La %s\n",count,prefix,generic,i,s->ncoeff,s->mjd_mid,s->psrname);
        s->coeff[0]=.25L;s->coeff[1]=30;
        long double freq=T1Polyco_GetFrequency(s,58000,700);assert(freq==100.5L);
        printf("FREQUENCY %La\n",freq);
    }
    if(generic)T2Predictor_Destroy(&p);else T1PolycoSet_Destroy(&set);
    assert(!ferror(f));fclose(f);
}
int main(void){for(int c=0;c<3;++c)for(int p=0;p<2;++p)for(int g=0;g<2;++g)check(c,p,g);return 0;}
