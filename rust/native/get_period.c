/* getPeriod.C at fda6ad4: same atof/double calculation and printf bytes.
 * Source argv access and uninitialized predictor were undefined on missing
 * arguments/read failure, and the owner leaked. Check and release that owner. */
#include "tempo2pred_int.h"
#include <stdlib.h>
#include <math.h>
int main(int argc, char **argv) {
    if (argc < 3) { fprintf(stderr,"usage: getPeriod-rust PREDICTOR MJD\n"); return 1; }
    T2Predictor pred; T2Predictor_Init(&pred);
    if (T2Predictor_Read(&pred,argv[1])) {
        fprintf(stderr,"getPeriod-rust: cannot read %s\n",argv[1]);
        T2Predictor_Destroy(&pred); return 1;
    }
    double mjd=atof(argv[2]);
    /* getPeriod.C evaluates fresh file backing immediately. t1polyco.c reads
     * C1 regardless of declared count, so a low-count file has no producer for
     * that cell here. The public C reader still accepts it for callers that
     * initialize exposed backing; this client reports its own missing input. */
    if (pred.kind==T1) {
        T1Polyco *selected=T1PolycoSet_GetNearest(&pred.modelset.t1,mjd);
        if (!selected || selected->ncoeff<2) {
            fprintf(stderr,"getPeriod-rust: file does not supply frequency coefficient C1\n");
            T2Predictor_Destroy(&pred); return 1;
        }
    }
    const double frequency=(double)T2Predictor_GetFrequency(&pred,mjd,1400);
    printf("%lf %24.20lf %24.20lf\n",mjd,frequency,1.0/frequency);
    T2Predictor_Destroy(&pred); return 0;
}
