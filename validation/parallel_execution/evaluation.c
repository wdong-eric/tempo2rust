/* Single caller: identical owned Cheby file and separate phase/frequency calls.
 * Timed interval excludes parse/publication; no T1 diagnostics are suppressed. */
#include "tempo2pred.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
int main(int argc,char **argv) {
    assert(argc==3);
    T2Predictor model; T2Predictor_Init(&model);
    assert(T2Predictor_Read(&model,argv[1])==0);
    assert(T2Predictor_Kind(&model)==Cheby);
    int n=atoi(argv[2]); assert(n>0);
    struct timespec start,end;
    assert(clock_gettime(CLOCK_MONOTONIC,&start)==0);
    long double sum=0;
    for(int i=0;i<n;i++) {
        long double epoch=58000.01L+(i%7)*0.000001L;
        sum+=T2Predictor_GetPhase(&model,epoch,1400);
        sum+=T2Predictor_GetFrequency(&model,epoch,1400);
    }
    assert(clock_gettime(CLOCK_MONOTONIC,&end)==0);
    printf("calls %d checksum %La evaluate_s %.9f\n",2*n,sum,
        (end.tv_sec-start.tv_sec)+(end.tv_nsec-start.tv_nsec)*1e-9);
    T2Predictor_Destroy(&model);
    return 0;
}
