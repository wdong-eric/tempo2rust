/* Complete outputs/state, separate from evaluation.c's timed checksum loop. */
#define _GNU_SOURCE
#define _DARWIN_C_SOURCE
#include "tempo2pred_int.h"
#include <assert.h>
#include <dlfcn.h>
#include <float.h>
#include <math.h>
#include <stdio.h>
#include <string.h>

static void value(const char *name, int i, long double x) {
    printf("%s %d %La sign=%d finite=%d nan=%d\n", name, i, x,
           !!signbit(x), !!isfinite(x), !!isnan(x));
}

int main(int argc, char **argv) {
    assert(argc == 2);
    Dl_info info;
    assert(dladdr((void *)T2Predictor_GetPhase, &info));
    fprintf(stderr, "LOADED %s\n", info.dli_fname);
    printf("ABI %zu %d\n", sizeof(long double), LDBL_MANT_DIG);
    T2Predictor p; T2Predictor_Init(&p);
    assert(T2Predictor_Read(&p, argv[1]) == 0 && p.kind == Cheby);
    printf("META %s %s %d\n", T2Predictor_GetPSRName(&p), T2Predictor_GetSiteName(&p), p.modelset.cheby.nsegments);
    ChebyModel *m = &p.modelset.cheby.segments[0];
    long double *coeff = m->cheby.coeff, *frequency = m->frequency_cheby.coeff;
    long double c0 = coeff[0], f0 = frequency[0];
    for (int i = 0; i < m->cheby.nx*m->cheby.ny; ++i) value("COEFF", i, coeff[i]);
    for (int i = 0; i < m->frequency_cheby.nx*m->frequency_cheby.ny; ++i) value("DERIVATIVE", i, frequency[i]);
    /* The pointer is unchanged across mutations. Both arrays remain caller
     * visible; a pointer-only conversion cache fails this control. */
    for (int i = 0; i < 10000; ++i) {
        if (i == 64) { coeff[0] += 4; frequency[0] += 8; }
        if (i == 128) { coeff[0] = c0; frequency[0] = f0; }
        long double epoch = 58000.01L+(i%7)*0.000001L;
        value("PHASE", i, T2Predictor_GetPhase(&p, epoch, 1400));
        value("FREQUENCY", i, T2Predictor_GetFrequency(&p, epoch, 1400));
        printf("STATE %d %d %d %d\n", i, ChebyModelSet_OutOfRange,
               coeff == m->cheby.coeff, frequency == m->frequency_cheby.coeff);
    }
    value("MISS", 0, T2Predictor_GetPhase(&p, m->mjd_end+1, 1400));
    printf("MISS-STATE %d\n", ChebyModelSet_OutOfRange);
    value("RECOVER", 0, T2Predictor_GetPhase(&p, 58000.01L, 1400));
    printf("RECOVER-STATE %d\n", ChebyModelSet_OutOfRange);
    T2Predictor_Destroy(&p);

    T1Polyco t; memset(&t, 0, sizeof t);
    t.mjd_mid = 58000; t.reference_phase = 7; t.frequency_psr_0 = 100.125L;
    t.frequency_obs = 1400; t.dm = 2; t.doppler = .01;
    for (int i = 0; i < 32; ++i) t.coeff[i] = (i+1)/65536.L;
    const int counts[] = {0, 1, 2, 6, 32};
    for (int i = 0; i < 5; ++i) {
        t.ncoeff = counts[i];
        for (int j = 0; j < 7; ++j) {
            long double epoch = 58000.L+(j-3)/1048576.L;
            value("T1-PHASE", i*7+j, T1Polyco_GetPhase(&t, epoch, 700));
            value("T1-FREQUENCY", i*7+j, T1Polyco_GetFrequency(&t, epoch, 700));
        }
    }
    /* Unused T1 phase cells are NaN: frequency must read only its source inputs. */
    t.ncoeff = 2; t.reference_phase = t.coeff[0] = NAN; t.coeff[1] = -0.L;
    long double classes[] = {0.L, -0.L, LDBL_MIN, -LDBL_MIN,
        nextafterl(0.L, 1.L), -nextafterl(0.L, 1.L), LDBL_MAX, -LDBL_MAX,
        INFINITY, -INFINITY, NAN, -NAN, 0x1.0000000000000002p0L};
    for (int i = 0; i < (int)(sizeof classes/sizeof *classes); ++i) {
        t.frequency_psr_0 = classes[i];
        value("T1-CLASS", i, T1Polyco_GetFrequency(&t, t.mjd_mid, 0));
    }
    return 0;
}
