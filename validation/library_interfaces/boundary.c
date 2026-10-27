/* Central AJ regressions: defined pipe histories and locale-independent
 * private scalar marshalling. The public text formats retain source locale. */
#ifndef __APPLE__
#define _POSIX_C_SOURCE 200809L
#endif
#include "tempo2pred_int.h"
#include <assert.h>
#include <locale.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#ifdef __APPLE__
#include <xlocale.h>
#endif

static void stream_case(const char *text, int is_pipe, int kind, int next) {
    FILE *f;
    size_t n = strlen(text);
    if (is_pipe) {
        int fd[2]; assert(pipe(fd) == 0);
        assert(write(fd[1], text, n) == (ssize_t)n);
        assert(close(fd[1]) == 0); f = fdopen(fd[0], "r");
    } else {
        f = tmpfile(); assert(f); assert(fwrite(text, 1, n, f) == n); rewind(f);
    }
    assert(f);
    T2Predictor p; T2Predictor_Init(&p);
    int status = T2Predictor_FRead(&p, f);
    assert(status == 0 && (int)p.kind == kind);
    int count = p.kind == Cheby ? p.modelset.cheby.nsegments : p.modelset.t1.nsegments;
    assert(count == 1);
    if (p.kind == T1) {
        const T1Polyco *q = &p.modelset.t1.segments[0];
        assert(!strcmp(q->psrname, "READ") && !strcmp(q->date_string, "DATE") && !strcmp(q->utc_string, "UTC"));
        assert(q->mjd_mid == 58000 && q->dm == 0 && q->doppler == 0 && q->log10rms == -6);
        assert(q->reference_phase == 7 && q->frequency_psr_0 == 100 && !strcmp(q->sitename, "@"));
        assert(q->span == 60 && q->ncoeff == 2 && q->frequency_obs == 1400);
        assert(q->binary_phase == 0 && q->binary_frequency == 0 && q->coeff[0] == .5L && q->coeff[1] == 60);
    }
    printf("STREAM %d %d %d %d %s\n", is_pipe, status, (int)p.kind, count, T2Predictor_GetPSRName(&p));
    int c = fgetc(f); assert(c == next && !ferror(f));
    printf("CURSOR %d %d %d\n", is_pipe, c, ferror(f));
    T2Predictor_Destroy(&p); assert(fclose(f) == 0);
}
int main(void) {
    const char *cheby = "ChebyModelSet 1 segments\nChebyModel BEGIN\nPSRNAME PIPE\nSITENAME PKS\nTIME_RANGE 58000 58001\nFREQ_RANGE 1000 2000\nDISPERSION_CONSTANT 0\nNCOEFF_TIME 2\nNCOEFF_FREQ 2\nCOEFFS 2 0\nCOEFFS 4 0\nChebyModel END\nTAIL\n";
    const char *t1 = "TEMPO2:\nREAD DATE UTC 58000 0 0 -6 7 100 @ 60 2 1400 0 0\n0.5 60\n";
    /* The generic source consumes its failed Cheby header. With a pipe that
     * cannot rewind, a leading discarded line leaves a complete T1 record.
     * A bare new-format marker would be consumed, invoking the fixed reader on
     * short body lines (unchecked source conversions read unwritten locals). */
    const char *t1_pipe = "discarded\nTEMPO2:\nREAD DATE UTC 58000 0 0 -6 7 100 @ 60 2 1400 0 0\n0.5 60\n";
    for (int pipe = 0; pipe < 2; ++pipe) {
        stream_case(cheby, pipe, Cheby, 'T');
        stream_case(pipe ? t1_pipe : t1, pipe, T1, EOF);
    }
    int fd[2]; assert(pipe(fd) == 0);
    assert(write(fd[1], t1, strlen(t1)) == (ssize_t)strlen(t1)); close(fd[1]);
    FILE *f = fdopen(fd[0], "r"); assert(f);
    long pos = ftell(f); assert(pos == -1);
    ChebyModelSet set; ChebyModelSet_Init(&set);
    assert(ChebyModelSet_Read(&set, f) == -1);
    assert(fseek(f, pos, SEEK_SET) == -1);
    int next = fgetc(f); assert(next == 'R');
    printf("FAILED-TRIAL next=%c error=%d\n", next, ferror(f));
    ChebyModelSet_Destroy(&set); fclose(f);
    assert(pipe(fd) == 0);
    assert(write(fd[1], t1, strlen(t1)) == (ssize_t)strlen(t1)); close(fd[1]);
    f = fdopen(fd[0], "r"); assert(f);
    T1PolycoSet polycos; T1PolycoSet_Init(&polycos);
    assert(T1PolycoSet_Read(&polycos, f) == 0 && polycos.nsegments == 1);
    assert(polycos.segments[0].ncoeff == 2 && polycos.segments[0].coeff[1] == 60);
    printf("DIRECT-T1-PIPE %d %d %La\n", polycos.nsegments, polycos.segments[0].ncoeff, polycos.segments[0].coeff[1]);
    T1PolycoSet_Destroy(&polycos); fclose(f);
    const char *locales[] = {"C", "de_DE.UTF-8", "fr_FR.UTF-8", "C"};
    for (int j = 0; j < 2; ++j) {
        T1Polyco p; memset(&p, 0, sizeof p);
        p.ncoeff = 2; p.mjd_mid = j ? 1 : 58000;
        p.frequency_psr_0 = j ? 1 : 100.125L; p.coeff[1] = j ? 1 : 30;
        long double reference = T1Polyco_GetFrequency(&p, p.mjd_mid, 700);
        assert(isfinite(reference) && reference != floorl(reference));
        for (int i = 0; i < 4; ++i) {
            assert(setlocale(LC_NUMERIC, locales[i]));
            char before[128]; snprintf(before, sizeof before, "%s", setlocale(LC_NUMERIC, NULL));
            long double result = T1Polyco_GetFrequency(&p, p.mjd_mid, 700);
            assert(strcmp(before, setlocale(LC_NUMERIC, NULL)) == 0);
            assert(isfinite(result) && result == reference);
            assert(setlocale(LC_NUMERIC, "C"));
            printf("LOCALE %d %s %La\n", j, locales[i], result);
        }
        for (int i = 1; i < 3; ++i) {
            locale_t local = newlocale(LC_NUMERIC_MASK, locales[i], (locale_t)0);
            assert(local); locale_t previous = uselocale(local); assert(previous);
            long double result = T1Polyco_GetFrequency(&p, p.mjd_mid, 700);
            assert(uselocale((locale_t)0) == local);
            assert(!strcmp(setlocale(LC_NUMERIC, NULL), "C"));
            assert(result == reference); uselocale(previous); freelocale(local);
            printf("THREAD-LOCALE %d %s %La\n", j, locales[i], result);
        }
    }
    T1Polyco p; memset(&p, 0, sizeof p); p.ncoeff = 2;
    long double classes[] = {-0.L, nextafterl(0.L, 1.L), INFINITY, NAN};
    for (int i = 0; i < 4; ++i) {
        p.frequency_psr_0 = classes[i]; p.coeff[1] = -0.L;
        long double result = T1Polyco_GetFrequency(&p, 0, 700);
        if (isnan(classes[i])) assert(isnan(result));
        else assert(result == classes[i] && !!signbit(result) == !!signbit(classes[i]));
        printf("CLASS %d %La\n", i, result);
    }
    return 0;
}
