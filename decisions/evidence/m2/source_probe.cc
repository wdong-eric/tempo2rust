// Central M2 design evidence. Calls the accepted A2 library; no replacement solver.
#include <cfloat>
#include <cstdio>
#include <cstdlib>
#include <cmath>
#include <type_traits>
#include "tempo2.h"
#include "TKfit.h"
#include "T2toolkit.h"
#include "t2fit_stdFitFuncs.h"

static void quad(FILE *out, longdouble value) {
    char text[160];
    quadmath_snprintf(text, sizeof text, "%.36Qg", value);
    fprintf(out, "\"%s\"", text);
}

static void solve(FILE *out, const char *name, int kind, bool rescale) {
    const int n = kind == 4 ? 2 : 6;
    const double x[] = {-3, -1, 0, 2, 5, 9};
    const double noise[] = {3e-6, -2e-6, 4e-6, 1e-6, -5e-6, 6e-6};
    const double sigma[] = {1e-6, 2e-6, 1.5e-6, .7e-6, 3e-6, 1.1e-6};
    double **a = malloc_blas(n, 2), **aw = malloc_blas(n, 2);
    double **cov = malloc_blas(2, 2), y[6], yw[6], p[2], e[2];
    for (int i=0; i<n; ++i) {
        const double s = kind == 0 ? sigma[i] : 1;
        a[i][0] = kind < 2 ? 1 : (i == 0 ? 1 : 0);
        a[i][1] = kind == 0 ? x[i] : kind == 1 ? 1 :
            i == 1 ? (kind == 2 ? 1e-28 : kind == 3 ? 1e-26 : kind == 5 ? 1e-27 : 1) : 0;
        y[i] = noise[i];
        yw[i] = y[i]/s;
        for (int j=0; j<2; ++j) aw[i][j] = a[i][j]/s;
    }
    const double chi = TKleastSquares(y, yw, a, aw, n, 2, 1e-27, rescale, p, e, cov);
    fprintf(out, "\"%s\":{\"n\":%d,\"nparams\":2,\"rescale\":%s,"
        "\"chi_square\":%.17g,\"solution\":[%.17g,%.17g],"
        "\"covariance\":[[%.17g,%.17g],[%.17g,%.17g]],"
        "\"errors_finite\":[%s,%s],\"errors\":[",
        name, n, rescale ? "true" : "false", chi, p[0], p[1],
        cov[0][0], cov[0][1], cov[1][0], cov[1][1],
        std::isfinite(e[0]) ? "true" : "false", std::isfinite(e[1]) ? "true" : "false");
    for (int j=0; j<2; ++j) {
        if (j) fputc(',', out);
        if (std::isfinite(e[j])) fprintf(out, "%.17g", e[j]); else fputs("null", out);
    }
    fputs("]}", out);
    free_blas(a); free_blas(aw); free_blas(cov);
}

int main(int argc, char **argv) {
    if (argc != 2) return 2;
    FILE *out = fopen(argv[1], "w");
    if (!out) return 2;
    fprintf(out, "{\"native_long_double_bits\":%d,\"project_longdouble_bits\":%d,"
        "\"sign_helper_returns_double\":%s,\"svd_tolerance\":%.17g,",
        LDBL_MANT_DIG, __FLT128_MANT_DIG__,
        std::is_same<decltype(TKsign_d(1.0, 1.0)), double>::value ? "true" : "false", 1e-27);
    solve(out, "weighted_unscaled", 0, false); fputc(',', out);
    solve(out, "weighted_rescaled", 0, true); fputc(',', out);
    solve(out, "dependent_columns", 1, true); fputc(',', out);
    solve(out, "below_cutoff", 2, false); fputc(',', out);
    solve(out, "above_cutoff", 3, false); fputc(',', out);
    solve(out, "at_cutoff", 5, false); fputc(',', out);
    solve(out, "zero_degrees_of_freedom", 4, true); fputc(',', out);
    solve(out, "zero_degrees_of_freedom_unscaled", 4, false); fputc(',', out);

    MAX_PSR = 1;
    pulsar *p = new pulsar;
    initialise(p, 0);
    p->param[param_f].val[0] = parse_longdouble("123.45678901234567890123456789");
    p->param[param_f].val[1] = parse_longdouble("-1e-15");
    p->param[param_f].val[2] = parse_longdouble("2e-25");
    p->param[param_f].val[3] = parse_longdouble("-3e-35");
    p->param[param_brake].paramSet[0] = 0;
    const double x = -1234.56789;
    fprintf(out, "\"spin\":{\"x_days\":%.17g,\"columns\":[1", x);
    for (int k=0; k<4; ++k)
        fprintf(out, ",%.17g", t2FitFunc_stdFreq(p, 0, x, 0, param_f, k));
    fputs("],\"before\":[", out);
    for (int k=0; k<4; ++k) { if (k) fputc(',', out); quad(out, p->param[param_f].val[k]); }
    fputs("],\"increments\":[0.25,0.5,0.75,1],\"errors_in_fit_coordinates\":[", out);
    for (int k=0; k<4; ++k) { if (k) fputc(',', out); fprintf(out, "%.17g", .1*(k+1)); }
    fputs("],\"after\":[", out);
    for (int k=0; k<4; ++k) t2UpdateFunc_stdFreq(p, 0, param_f, k, .25*(k+1), .1*(k+1));
    for (int k=0; k<4; ++k) { if (k) fputc(',', out); quad(out, p->param[param_f].val[k]); }
    fputs("],\"physical_errors\":[", out);
    for (int k=0; k<4; ++k) { if (k) fputc(',', out); quad(out, p->param[param_f].err[k]); }
    fputs("]}}\n", out);
    fclose(out);
    destroyOne(p);
    delete p;
    return 0;
}
