/* Decision 0136: caller-owned coefficients and retained owning-path contracts.
 * Compare the complete transcript against the immutable pre-view library. */
#include "tempo2pred_int.h"
#include <assert.h>
#include <float.h>
#include <limits.h>
#include <math.h>
#include <stdint.h>
#include <string.h>

/* Private bridge calls exercise existing null/error ordering, not a new API. */
#ifdef __cplusplus
extern "C" {
#endif
int t2r_evaluate(const void *, int, int, const void *, const void *, void *);
int t2r_model(const void *, int, int, const void *, int, void *);
#ifdef __cplusplus
}
#endif

static void value(const char *name, int index, long double x) {
    printf("%s %d %La %d %d\n", name, index, x, !!signbit(x), fpclassify(x));
}

static void mutate_callback(long double *x, long double *y, int nx, int ny,
                            long double *z, void *info) {
    (void)x; (void)y;
    Cheby2D *p = (Cheby2D *)info;
    p->coeff[0] = 40;
    for (int i = 0; i < nx * ny; ++i) z[i] = 1;
}

int main(void) {
    long double tiny = nextafterl(0.L, 1.L);
    long double classes[40] = {
        0.L, -0.L, tiny, -tiny, 2*tiny, -2*tiny,
        nextafterl(LDBL_MIN, 0.L), LDBL_MIN, nextafterl(LDBL_MIN, INFINITY),
        -LDBL_MIN, nextafterl(1.L, 0.L), 1.L, nextafterl(1.L, INFINITY),
        -1.L, nextafterl(LDBL_MAX, 0.L), LDBL_MAX, -LDBL_MAX,
        INFINITY, -INFINITY, NAN, -NAN,
    };
    int nclasses = 21;
#if defined(__APPLE__) && defined(__aarch64__)
    const uint64_t nan_bits[] = {
        UINT64_C(0x7ff0000000000001), UINT64_C(0xfff0000000000001),
        UINT64_C(0x7ff7ffffffffffff), UINT64_C(0xfff7ffffffffffff),
        UINT64_C(0x7ff8000000000001), UINT64_C(0xfff8000000000001),
    };
    assert(sizeof(long double) == sizeof(uint64_t));
    for (unsigned i = 0; i < sizeof nan_bits / sizeof *nan_bits; ++i)
        memcpy(&classes[nclasses++], &nan_bits[i], sizeof(uint64_t));
#elif defined(__linux__) && defined(__x86_64__)
    classes[nclasses++] = 0x1.0000000000000002p0L;
    classes[nclasses++] = 0x1p-16445L;
    classes[nclasses++] = 0x1p16382L;
#else
#error Unqualified coefficient-view witness target
#endif
    const int shapes[][2] = {{1,1}, {1,4}, {4,1}, {2,2}, {8,4}};
    const long double points[][2] = {{-1,-1}, {.125L,-.375L}, {1,1}, {1.25L,-.75L}};
    long double backing[32];
    unsigned char saved[sizeof backing];
    int row = 0;
    for (unsigned shape = 0; shape < sizeof shapes / sizeof *shapes; ++shape) {
        Cheby2D p = {shapes[shape][0], shapes[shape][1], backing};
        ChebyModel model;
        memset(&model, 0, sizeof model);
        model.mjd_start = 0; model.mjd_end = 1;
        model.freq_start = 1000; model.freq_end = 2000;
        model.dispersion_constant = 3.5L;
        model.cheby = model.frequency_cheby = p;
        for (int position = 0; position < p.nx*p.ny; ++position) {
            for (int kind = 0; kind < nclasses; ++kind) {
                memset(backing, 0, sizeof backing);
                for (int i = 0; i < p.nx*p.ny; ++i)
                    backing[i] = (i % 2 ? -1.L : 1.L) * (i+1) / 32;
                memcpy(&backing[position], &classes[kind], sizeof(long double));
                memcpy(saved, backing, sizeof backing);
                printf("CASE %u %d %d\n", shape, position, kind);
                for (unsigned point = 0; point < sizeof points / sizeof *points; ++point)
                    value("EVALUATE", row++, Cheby2D_Evaluate(&p, points[point][0], points[point][1]));
                value("PHASE", row++, ChebyModel_GetPhase(&model, .25L, 1250));
                value("FREQUENCY", row++, ChebyModel_GetFrequency(&model, .25L, 1250));
                assert(memcmp(saved, backing, sizeof backing) == 0);
            }
        }
    }

    long double a[] = {4}, b[] = {12};
    Cheby2D p = {1, 1, a};
    assert(Cheby2D_Evaluate(&p, .125L, .375L) == 1);
    a[0] = 8; assert(Cheby2D_Evaluate(&p, .125L, .375L) == 2);
    a[0] = 4; assert(Cheby2D_Evaluate(&p, .125L, .375L) == 1);
    p.coeff = b; assert(Cheby2D_Evaluate(&p, .125L, .375L) == 3);
    p.coeff = a; assert(Cheby2D_Evaluate(&p, .125L, .375L) == 1);
    memcpy(a, b, sizeof a); assert(Cheby2D_Evaluate(&p, .125L, .375L) == 3);
    a[0] = Cheby2D_Evaluate(&p, .125L, .375L);
    assert(Cheby2D_Evaluate(&p, .125L, .375L) == .75L);
    puts("MUTATION restoration replacement repeated-address publication passed");

    long double callback_coeff[] = {4, 0, 0, 0}, rms = -1, mav = -1;
    Cheby2D snapshot = {2, 2, callback_coeff};
    Cheby2D_Test(&snapshot, 2, 2, mutate_callback, &snapshot, &rms, &mav);
    assert(rms == 0 && mav == 0 && callback_coeff[0] == 40);
    assert(Cheby2D_Evaluate(&snapshot, .25L, .5L) == 10);
    puts("CALLBACK owned snapshot passed");

    long double original[] = {4, -2, 3, -1, 2, 1, -3, 4}, separate[8] = {0};
    Cheby2D source = {4, 2, original}, derivative = {4, 2, separate};
    Cheby2D_Construct_x_Derivative(&derivative, &source);
    Cheby2D_Construct_x_Derivative(&source, &source);
    for (int i = 0; i < 8; ++i) {
        assert(original[i] == separate[i] && !!signbit(original[i]) == !!signbit(separate[i]));
        value("DERIVATIVE_ALIAS", i, original[i]);
    }

    const int invalid[][2] = {{0,1}, {-1,1}, {INT_MAX,2}};
    for (unsigned i = 0; i < sizeof invalid / sizeof *invalid; ++i) {
        Cheby2D bad = {invalid[i][0], invalid[i][1], NULL};
        assert(isnan(Cheby2D_Evaluate(&bad, 0, 0)));
    }
    Cheby2D missing = {1,1,NULL};
    assert(isnan(Cheby2D_Evaluate(&missing, 0, 0)));
    long double x = 0, out = 123, scalars[] = {0,1,1000,2000,0,.25L,1250};
    assert(t2r_evaluate(NULL, 0, 1, NULL, NULL, NULL) == -1);
    assert(t2r_evaluate(NULL, 1, 1, NULL, NULL, NULL) == -1);
    assert(t2r_evaluate(a, 1, 1, NULL, &x, &out) == -1 && out == 123);
    assert(t2r_evaluate(a, 1, 1, &x, NULL, &out) == -1 && out == 123);
    assert(t2r_evaluate(a, 1, 1, &x, &x, NULL) == -1);
    assert(t2r_model(NULL, 0, 1, NULL, 0, NULL) == -1);
    assert(t2r_model(NULL, 0, 1, scalars, 0, NULL) == -1);
    assert(t2r_model(NULL, 1, 1, scalars, 0, NULL) == -1);
    assert(t2r_model(a, 1, 1, scalars, 0, NULL) == -1);
    puts("ERROR dimension/null order and unchanged output passed");
    return 0;
}
