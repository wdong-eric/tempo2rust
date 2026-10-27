/* Native ABI/ownership adapter for pinned fda6ad4 tempo2pred.c, cheby2d.c,
 * t1polyco.c. Retained source scanning/formatting preserves FILE partial state.
 * Every scientific arithmetic body delegates to the canonical Rust leaf. */
#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif
#include "tempo2pred_int.h"
#include <string.h>
#include <strings.h>
#include <math.h>
#include <stdlib.h>
#include <stdint.h>
#include <limits.h>
#include <float.h>
#include <errno.h>
#include <assert.h>
#include <locale.h>
#ifdef __APPLE__
#include <xlocale.h>
#endif

#if defined(__APPLE__) && defined(__aarch64__)
_Static_assert(LDBL_MANT_DIG == 53 && sizeof(long double) == 8, "native long double profile");
#elif defined(__linux__) && defined(__x86_64__)
_Static_assert(LDBL_MANT_DIG == 64 && sizeof(long double) == 16, "native long double profile");
#else
#error Unqualified ordinary long double target
#endif
_Static_assert(sizeof(int) == 4, "source int ABI");

/* Rust sees only pointers, int and binary64 source fields. %La is exact, and
 * decimal output has >= LDBL_DECIMAL_DIG digits. No wider value visits f64.
 * Arrays are live for the synchronous call; callbacks must initialize all z. */
/* Private representation conversion must not inherit the caller's decimal
 * separator. This locale is local to the conversion, never process-global;
 * source public text readers/writers still use the caller's locale. */
int t2r_ld_get(const void *p, size_t i, char *s) {
    locale_t locale = newlocale(LC_NUMERIC_MASK, "C", (locale_t)0);
    if (!locale) return -1;
#ifdef __APPLE__
    int n = snprintf_l(s, 128, locale, "%La", ((const long double *)p)[i]);
#else
    locale_t previous = uselocale(locale);
    if (!previous) { freelocale(locale); return -1; }
    int n = snprintf(s, 128, "%La", ((const long double *)p)[i]);
    uselocale(previous);
#endif
    freelocale(locale);
    return n >= 0 && n < 128 ? 0 : -1;
}
int t2r_ld_set(void *p, size_t i, const char *s) {
    locale_t locale = newlocale(LC_NUMERIC_MASK, "C", (locale_t)0);
    if (!locale) return -1;
    char *end;
    long double value = strtold_l(s, &end, locale);
    freelocale(locale);
    if (end == s || *end) return -1;
    ((long double *)p)[i] = value;
    return 0;
}
void *t2r_ld_alloc(size_t n) { return n <= SIZE_MAX/sizeof(long double) ? malloc(n*sizeof(long double)) : NULL; }
void t2r_ld_free(void *p) { free(p); }
void t2r_phase_call(const T2Predictor *p, const void *mjd, const void *freq, void *out) {
    *(long double *)out = T2Predictor_GetPhase(p, *(const long double *)mjd, *(const long double *)freq);
}
typedef void (*callback)(long double *, long double *, int, int, long double *, void *);
extern int t2r_evaluate(const void *, int, int, const void *, const void *, void *);
extern int t2r_derivative(const void *, int, int, void *);
extern int t2r_construct(int, int, void *, callback, void *);
extern int t2r_test(const void *, int, int, int, int, callback, void *, void *, void *);
extern int t2r_model(const void *, int, int, const void *, int, void *);
extern int t2r_t1phase(const void *, int, const void *, double, double, double, void *);
extern int t2r_t1frequency(const void *, int, const void *, void *);
extern int t2r_nearest(const void *, int, int, const void *);
extern int t2r_plan(const void *, const void *, void *, int *, void *);
extern int t2r_testfunc(const void *, const void *, int, int, void *);
extern int t2r_span(const void *, int, int, void *);
//  Copyright (C) 2006,2007,2008,2009, George Hobbs, Russel Edwards

/*
 *    This file is part of TEMPO2. 
 * 
 *    TEMPO2 is free software: you can redistribute it and/or modify 
 *    it under the terms of the GNU General Public License as published by 
 *    the Free Software Foundation, either version 3 of the License, or 
 *    (at your option) any later version. 
 *    TEMPO2 is distributed in the hope that it will be useful, 
 *    but WITHOUT ANY WARRANTY; without even the implied warranty of 
 *    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the 
 *    GNU General Public License for more details. 
 *    You should have received a copy of the GNU General Public License 
 *    along with TEMPO2.  If not, see <http://www.gnu.org/licenses/>. 
 */

/*
 *    If you use TEMPO2 then please acknowledge it by citing 
 *    Hobbs, Edwards & Manchester (2006) MNRAS, Vol 369, Issue 2, 
 *    pp. 655-672 (bibtex: 2006MNRAS.369..655H)
 *    or Edwards, Hobbs & Manchester (2006) MNRAS, VOl 372, Issue 4,
 *    pp. 1549-1574 (bibtex: 2006MNRAS.372.1549E) when discussing the
 *    timing model.
 */




unsigned tempo2_verbose = 1;

    int
T2Predictor_Read(T2Predictor *t2p, char *fname)
{
    FILE *f = fopen(fname, "r");
    int ret;
    if (!f)
        return -1;
    ret = T2Predictor_FRead(t2p, f);
    fclose(f);
    return ret;
}

    int
T2Predictor_FRead(T2Predictor *t2p, FILE *f)
{
    /* fda6ad4 tempo2pred.c:51-71 trial-reads a possibly T1 union as Cheby.
     * Use initialized temporary storage, publish only a completed valid owner;
     * preserve source trial order and rewind on total failure. */
    long pos=ftell(f);
    T2Predictor tmp; T2Predictor_Init(&tmp);
    tmp.kind=Cheby;
    if (ChebyModelSet_Read(&tmp.modelset.cheby,f)==0) {
        T2Predictor_Destroy(t2p); *t2p=tmp; return 0;
    }
    T2Predictor_Destroy(&tmp);
    /* Source attempts rewind and ignores its status; libc owns the resulting
     * buffered cursor. Do not preempt either trial on a nonseekable stream. */
    fseek(f,pos,SEEK_SET);
    tmp.kind=T1;
    if (T1PolycoSet_Read(&tmp.modelset.t1,f)==0) {
        T2Predictor_Destroy(t2p); *t2p=tmp; return 0;
    }
    T2Predictor_Destroy(&tmp); fseek(f,pos,SEEK_SET); return -1;
}

    void
T2Predictor_Write(const T2Predictor *t2p, char *fname)
{
    FILE *f = fopen(fname, "w");
    if (!f)
    {
        fprintf(stderr, "Fatal error: T2Predictor_Write unable to open file %s\n",
                fname);
        exit(1);
    }
    T2Predictor_FWrite(t2p, f);
    fclose(f);
}

    void 
T2Predictor_FWrite(const T2Predictor *t2p, FILE *f)
{
    switch (t2p->kind)
    {
        default:
            break;
        case Cheby:
            ChebyModelSet_Write(&t2p->modelset.cheby, f);
            break;
        case T1:
            T1PolycoSet_Write(&t2p->modelset.t1, f);
            break;
    }
}

    void 
T2Predictor_Init(T2Predictor *t2p)
{
    t2p->kind = NonePredType;
    memset (&t2p->modelset, 0, sizeof(t2p->modelset));
}

    void
T2Predictor_Copy(T2Predictor *into_t2p, const T2Predictor *from_t2p)
{
    /* tempo2pred.c:114 self-copy destroyed its source owner. */
    if (into_t2p==from_t2p) return;
    T2Predictor_Destroy(into_t2p);
    T2Predictor_Insert(into_t2p, from_t2p);
}

    int
T2Predictor_Insert(T2Predictor *into_t2p, const T2Predictor *from_t2p)
{
    if (into_t2p->kind == NonePredType)
        into_t2p->kind = from_t2p->kind;

    if (into_t2p->kind != from_t2p->kind) {
        fprintf (stderr, "T2Predictor_Insert: not of the same kind!\n");
        return -1;
    }

    switch (into_t2p->kind)
    {
        default:
            break;
        case Cheby:
            return ChebyModelSet_Insert(&into_t2p->modelset.cheby,
                    &from_t2p->modelset.cheby);
        case T1:
            fprintf (stderr, "T2Predictor_Insert: not implemented for T1 polyco!\n");
            exit (-1);
            break;
    }
    return 0;
}

void T2Predictor_Keep(T2Predictor* t2p, unsigned nmjd, const long double* mjd)
{
    switch (t2p->kind)
    {
        default:
            break;
        case Cheby:
            ChebyModelSet_Keep(&t2p->modelset.cheby, nmjd, mjd);
            break;
        case T1:
            fprintf (stderr, "T2Predictor_Keep: not implemented for T1 polyco!\n");
            exit (-1);
            break;
    }
}

    void
T2Predictor_Destroy(T2Predictor *t2p)
{
    switch (t2p->kind)
    {
        default:
            break;
        case Cheby:
            ChebyModelSet_Destroy(&t2p->modelset.cheby);
            break;
        case T1:
            T1PolycoSet_Destroy(&t2p->modelset.t1);
            break;
    }
    T2Predictor_Init(t2p);
}

/* Information */
    char * 
T2Predictor_GetPSRName(T2Predictor *t2p)
{
    /* tempo2pred.c metadata dereferences absent first/last backing; report an
     * explicit empty sentinel instead of reading an invalid object. */
    if ((t2p->kind==Cheby && t2p->modelset.cheby.nsegments<=0) ||
        (t2p->kind==T1 && t2p->modelset.t1.nsegments<=0)) return NULL;

    char *ret = NULL;
    switch (t2p->kind)
    {
        default:
            break;
        case Cheby:
            ret = t2p->modelset.cheby.segments[0].psrname;
            break;
        case T1:
            ret = t2p->modelset.t1.segments[0].psrname;
            break;
    }
    return ret;
}
char * T2Predictor_GetSiteName(T2Predictor *t2p)
{
    /* tempo2pred.c metadata dereferences absent first/last backing; report an
     * explicit empty sentinel instead of reading an invalid object. */
    if ((t2p->kind==Cheby && t2p->modelset.cheby.nsegments<=0) ||
        (t2p->kind==T1 && t2p->modelset.t1.nsegments<=0)) return NULL;

    char *ret = NULL; 
    switch (t2p->kind)
    {
        default:
            break;
        case Cheby:
            ret = t2p->modelset.cheby.segments[0].sitename;
            break;
        case T1:
            ret = t2p->modelset.t1.segments[0].sitename;
            break;
    }
    return ret;
}
long double T2Predictor_GetStartMJD(T2Predictor *t2p)
{
    /* tempo2pred.c metadata dereferences absent first/last backing; report an
     * explicit empty sentinel instead of reading an invalid object. */
    if ((t2p->kind==Cheby && t2p->modelset.cheby.nsegments<=0) ||
        (t2p->kind==T1 && t2p->modelset.t1.nsegments<=0)) return NAN;

    long double ret = NAN;
    switch (t2p->kind)
    {
        default:
            break;
        case Cheby:
            ret = t2p->modelset.cheby.segments[0].mjd_start;
            break;
        case T1:
            t2r_span(&t2p->modelset.t1.segments[0].mjd_mid, t2p->modelset.t1.segments[0].span, -1, &ret);
            break;
    }
    return ret;
}
long double T2Predictor_GetEndMJD(T2Predictor *t2p)
{
    /* tempo2pred.c metadata dereferences absent first/last backing; report an
     * explicit empty sentinel instead of reading an invalid object. */
    if ((t2p->kind==Cheby && t2p->modelset.cheby.nsegments<=0) ||
        (t2p->kind==T1 && t2p->modelset.t1.nsegments<=0)) return NAN;

    long double ret = NAN;
    switch (t2p->kind)
    {
        default:
            break;
        case Cheby:
            ret = t2p->modelset.cheby.segments
                [t2p->modelset.cheby.nsegments-1].mjd_end;
            break;
        case T1:
            t2r_span(&t2p->modelset.t1.segments[t2p->modelset.t1.nsegments-1].mjd_mid, t2p->modelset.t1.segments[t2p->modelset.t1.nsegments-1].span, 1, &ret);
            break;
    }
    return ret;
} 
long double T2Predictor_GetStartFreq(T2Predictor *t2p) // MHz
{
    /* tempo2pred.c metadata dereferences absent first/last backing; report an
     * explicit empty sentinel instead of reading an invalid object. */
    if ((t2p->kind==Cheby && t2p->modelset.cheby.nsegments<=0) ||
        (t2p->kind==T1 && t2p->modelset.t1.nsegments<=0)) return NAN;

    long double ret = NAN;
    switch (t2p->kind)
    {
        default:
            break;
        case Cheby:
            ret = t2p->modelset.cheby.segments[0].freq_start;    
            break;
        case T1:
            ret = t2p->modelset.t1.segments[0].frequency_obs;    
            break;
    }
    return ret;
}
long double T2Predictor_GetEndFreq(T2Predictor *t2p)  // MHz
{
    /* tempo2pred.c metadata dereferences absent first/last backing; report an
     * explicit empty sentinel instead of reading an invalid object. */
    if ((t2p->kind==Cheby && t2p->modelset.cheby.nsegments<=0) ||
        (t2p->kind==T1 && t2p->modelset.t1.nsegments<=0)) return NAN;

    long double ret = NAN;
    switch (t2p->kind)
    {
        default:
            break;
        case Cheby:
            ret = t2p->modelset.cheby.segments[0].freq_end;    
            break;
        case T1:
            ret = t2p->modelset.t1.segments[0].frequency_obs;    
            break;
    }
    return ret;
}
T2PredictorKind T2Predictor_Kind(T2Predictor *t2p)
{
    return t2p->kind;
}

/* Prediction */
long double T2Predictor_GetPhase(const T2Predictor *t2p, long double mjd,
        long double freq) // freq in MHz
{
    long double ret = NAN;
    switch (t2p->kind)
    {
        default:
            break;
        case Cheby:
            ret = ChebyModelSet_GetPhase(&t2p->modelset.cheby, mjd, freq);    
            break;
        case T1:
            ret = T1PolycoSet_GetPhase(&t2p->modelset.t1, mjd, freq); 
            break;
    }
    return ret;
}

long double T2Predictor_GetFrequency(const T2Predictor *t2p, long double mjd,
        long double freq) // freq in MHz
{
    long double ret = NAN;
    switch (t2p->kind)
    {
        default:
            break;
        case Cheby:
            ret = ChebyModelSet_GetFrequency(&t2p->modelset.cheby, mjd, freq);    
            break;
        case T1:
            ret = T1PolycoSet_GetFrequency(&t2p->modelset.t1, mjd, freq);    
            break;
    }
    return ret;
}


int T2Predictor_GetPlan(char *filename,
        long double mjd_start,
        long double mjd_end,
        long double step, // seconds
        long double freq, // MHz
        // remaining arguments are returned
        long double *phase0,
        int *nsegments,
        long double *pulse_frequencies)
{
    return T2Predictor_GetPlan_Ext(filename, mjd_start, mjd_end, step, freq,
            NULL, NULL, phase0, nsegments, pulse_frequencies);
}

int T2Predictor_GetPlan_Ext(char *filename,
        long double mjd_start,
        long double mjd_end,
        long double step, // seconds
        long double freq, // MHz
        // remaining arguments are returned
        char *psrname, char *sitename,
        long double *phase0,
        int *nsegments,
        long double *pulse_frequencies)
{
    /* fda6ad4 tempo2pred.c:350 reads an uninitialized union. Initialize and
     * destroy on every path; the shared Rust engine supplies all plan arithmetic. */
    T2Predictor pred; T2Predictor_Init(&pred);
    int ret=T2Predictor_Read(&pred, filename);
    if (ret) { T2Predictor_Destroy(&pred); return ret; }
    if (!T2Predictor_GetPSRName(&pred)) { T2Predictor_Destroy(&pred); return -1; }
    if (psrname) strcpy(psrname, T2Predictor_GetPSRName(&pred));
    if (sitename) strcpy(sitename, T2Predictor_GetSiteName(&pred));
    long double v[] = {mjd_start, mjd_end, step, freq};
    ret=t2r_plan(&pred, v, phase0, nsegments, pulse_frequencies);
    T2Predictor_Destroy(&pred); return ret;
}


//  Copyright (C) 2006,2007,2008,2009, George Hobbs, Russel Edwards

/*
 *    This file is part of TEMPO2. 
 * 
 *    TEMPO2 is free software: you can redistribute it and/or modify 
 *    it under the terms of the GNU General Public License as published by 
 *    the Free Software Foundation, either version 3 of the License, or 
 *    (at your option) any later version. 
 *    TEMPO2 is distributed in the hope that it will be useful, 
 *    but WITHOUT ANY WARRANTY; without even the implied warranty of 
 *    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the 
 *    GNU General Public License for more details. 
 *    You should have received a copy of the GNU General Public License 
 *    along with TEMPO2.  If not, see <http://www.gnu.org/licenses/>. 
 */

/*
 *    If you use TEMPO2 then please acknowledge it by citing 
 *    Hobbs, Edwards & Manchester (2006) MNRAS, Vol 369, Issue 2, 
 *    pp. 655-672 (bibtex: 2006MNRAS.369..655H)
 *    or Edwards, Hobbs & Manchester (2006) MNRAS, VOl 372, Issue 4,
 *    pp. 1549-1574 (bibtex: 2006MNRAS.372.1549E) when discussing the
 *    timing model.
 */

// redwards code for finding and evaluating 2D Chebyshev polynomials.
// 
// Note, 2D (Cartesian) Chebyshev polynomials actually follow quite trivially
// from the 1D case.
//
// Defining 
//
//   Tnm(x) = Tn(x)Tm(y)  , where Tn(x) is the Chebyshev polnomial of deg n,
//
//  the discrete orthogonality condition,
//
//    sum_a=1^n sum_b=1^m Tij(xa,yb)Tkl(xa,yb) = 0
//
//  is satisfied by the standard 1D choice of xa, yb, i.e. the zeros of
//  the 1D basis function, i.e. x = cos(pi[k-1/2]/n) for Tn . Then,
//  rewriting we have
//
//  sum_a=1^n Tij(xa,yb) sum_b=1^m Tkl(xa,yb) = 0 
//
//  which implies  ! (j==l && i==k)
//
//  i.e. the bases are orthonormal
//
// this means we can define
//
// c_ij = sum_a=1^n sum_b=1^m f(xa,yb)Tij(xa,yb)
//
// and then have f(x,y) ~= sum_i=1^n sum_j=1^m  c_ij Tij(x,y)
//
//  where the relationship is exact for x=xa , y=ya and pretty good elsewhere


#ifdef sun
#endif


#ifndef M_PIl
#define M_PIl 3.14159265358979323846264338327950288L
#endif

// #define _DEBUG 1

long double parse_ld_cheby(const char* str){
    /* cheby2d.c:82 returns unwritten ld on conversion failure. */
    long double out=NAN; if (sscanf(str,"%Lf",&out)!=1) errno=EINVAL; return out;
}


void Cheby2D_Init(Cheby2D *cheby, int nx, int ny)
{
    /* cheby2d.c:93 multiplies signed unchecked dimensions before malloc.
     * Check the native allocation extent; (0,0) remains the empty initializer. */
    cheby->nx=cheby->ny=0; cheby->coeff=NULL;
    if (nx>=0 && ny>=0 && (nx==0 || ny==0)) { cheby->nx=nx; cheby->ny=ny; return; }
    if (nx<=0 || ny<=0 || nx>INT_MAX/ny) { errno=EINVAL; return; }
    long double *p=t2r_ld_alloc((size_t)nx*ny);
    if (!p) { errno=ENOMEM; return; }
    cheby->nx=nx; cheby->ny=ny; cheby->coeff=p;
}

void Cheby2D_Destroy(Cheby2D *cheby)
{
#if _DEBUG
  fprintf (stderr, "Cheby2D_Destroy this=%p coeff=%p\n", 
           (void*)cheby, (void*)(cheby->coeff));
#endif

  if (cheby->coeff)
    free(cheby->coeff);

  cheby->coeff = NULL;
  cheby->nx = cheby->ny = 0;
}

void Cheby2D_Copy(Cheby2D *cheby, const Cheby2D* from)
{
    /* cheby2d.c:124 frees its own source for self-copy; retain the owner. */
    if (cheby==from) return;
    Cheby2D tmp; Cheby2D_Init(&tmp,from->nx,from->ny);
    if (from->nx && from->ny && !tmp.coeff) return;
    if (tmp.coeff) memcpy(tmp.coeff,from->coeff,(size_t)from->nx*from->ny*sizeof(long double));
    Cheby2D_Destroy(cheby); *cheby=tmp;
}

void
Cheby2D_Construct(Cheby2D *cheby,
        void
        (*func)(long double *x, long double *y, 
            int nx, int ny, long double *z, void *info),
        void *info)
{
    t2r_construct(cheby->nx, cheby->ny, cheby->coeff, func, info);
}


// Here is a function to construct a function that's the partial derivative
// wrt x of the a second chebyshev polynomial
    void
Cheby2D_Construct_x_Derivative(Cheby2D *dcheby, const Cheby2D *cheby)
{
    if (dcheby->nx != cheby->nx || dcheby->ny != cheby->ny) {
        fprintf(stderr, "Cheby derivative dimensions differ\n"); return;
    }
    t2r_derivative(cheby->coeff, cheby->nx, cheby->ny, dcheby->coeff);
}

// here is the function to evaulate a 2D Chebyshev polynomial,
// f(x,y) ~= sum_i=1^n sum_j=1^m  c_ij Tij(x,y)
//         = sum_j=1^m Tj(yb) sum_i=1^n c_ij Ti(xa)
//  This can be tackled in much the same way as the 1D case:
//  the inner loop is a simple 1D Chebyshev with c_j as the coefficients,
// while the outer loop is a 1D Cheby with the result of the inner as the
// coefficients.
// These are evaluated using Clenshaw's recurrence, as in Numerical Recipes,
//   f(x) = xd_1 - d_2 + c_0/2
//   d_j = 2xd_(j+1) - d_(j+2) + c_j
//   d_(m+1) = d_m = 0

    long double 
Cheby2D_Evaluate(const Cheby2D *cheby, long double x, long double y)
{
    long double out = NAN;
    t2r_evaluate(cheby->coeff, cheby->nx, cheby->ny, &x, &y, &out);
    return out;
}


void
Cheby2D_Test(Cheby2D *cheby, int nx_test, int ny_test,
        void
        (*func)(long double *x, long double *y, 
            int nx, int ny, long double *z, void *info),
        void *info,
        long double *residualRMS, long double *residualMAV)
{
    t2r_test(cheby->coeff, cheby->nx, cheby->ny, nx_test, ny_test, func, info, residualRMS, residualMAV);
}

void testFunc(long double *x, long double *y, 
        int nx, int ny, long double *z, void *info)
{
    (void)info; t2r_testfunc(x, y, nx, ny, z);
}

void testCheby2D()
{
    int nx=30, ny=30;
    long double rms, mav;
    Cheby2D cheby;
    Cheby2D_Init(&cheby, nx, ny);
    printf("Constructing..."); fflush(stdout);
    Cheby2D_Construct(&cheby, testFunc, NULL);
    printf("\nTesting..."); fflush(stdout);
    Cheby2D_Test(&cheby, nx*3, ny*3,testFunc, NULL, &rms, &mav);
    printf("\nRMS= %Lg MAV= %Lg\n", rms, mav);
    Cheby2D_Destroy(&cheby);
}


    void
ChebyModel_Init(ChebyModel *cm, int nmjdcoeff, int nfreqcoeff)
{
    Cheby2D_Init(&cm->cheby, nmjdcoeff, nfreqcoeff);
    Cheby2D_Init(&cm->frequency_cheby, nmjdcoeff, nfreqcoeff);
}

    void
ChebyModel_Copy(ChebyModel *cm, const ChebyModel *from)
{
    /* cheby2d.c:373 self-copy aliases strcpy and destroys its own arrays. */
    if (cm==from) return;
    strcpy (cm->psrname, from->psrname);
    strcpy (cm->sitename, from->sitename);
    cm->mjd_start = from->mjd_start;
    cm->mjd_end = from->mjd_end;
    cm->freq_start = from->freq_start;
    cm->freq_end = from->freq_end;
    cm->dispersion_constant = from->dispersion_constant;

    Cheby2D_Copy(&cm->cheby, &from->cheby);
    Cheby2D_Copy(&cm->frequency_cheby, &from->frequency_cheby);
}

    void
ChebyModel_Destroy(ChebyModel *cm)
{
    if (cm == NULL)
        return;

#if _DEBUG
    fprintf (stderr, "ChebyModel_Destroy this=%p cheby=%p fcheby=%p\n",
             (void*)cm, (void*)&(cm->cheby), (void*)&(cm->frequency_cheby));
#endif

    Cheby2D_Destroy(&cm->cheby);
    Cheby2D_Destroy(&cm->frequency_cheby);
}

    long double
ChebyModel_GetPhase(const ChebyModel *cm, long double mjd, long double freq)
{
    if (!cm) return -1;
    long double v[] = {cm->mjd_start, cm->mjd_end, cm->freq_start, cm->freq_end,
        cm->dispersion_constant, mjd, freq}, out = NAN;
    t2r_model(cm->cheby.coeff, cm->cheby.nx, cm->cheby.ny, v, 0, &out);
    return out;
}

    long double
ChebyModel_GetFrequency(const ChebyModel *cm, long double mjd, long double freq)
{
    if (!cm) return -1;
    long double v[] = {cm->mjd_start, cm->mjd_end, cm->freq_start, cm->freq_end,
        0, mjd, freq}, out = NAN;
    t2r_model(cm->frequency_cheby.coeff, cm->frequency_cheby.nx, cm->frequency_cheby.ny, v, 1, &out);
    return out;
}

/* fda6ad4 cheby2d.c:430-443 native long double was passed to binary128 varargs.
 * Native fprintf here is type-correct; precision/order remain source-defined. */
void ChebyModel_Write(const ChebyModel *cm, FILE *f)
{
    int ix, iy;

    fprintf(f, "ChebyModel BEGIN\n");
    fprintf(f, "PSRNAME %s\n", cm->psrname);
    fprintf(f, "SITENAME %s\n", cm->sitename);
    fprintf(f, "TIME_RANGE %.34Lg %.34Lg\n", cm->mjd_start, cm->mjd_end);
    fprintf(f, "FREQ_RANGE %.34Lg %.34Lg\n", cm->freq_start, cm->freq_end);
    fprintf(f, "DISPERSION_CONSTANT %.34Lg\n", cm->dispersion_constant);
    fprintf(f, "NCOEFF_TIME %d\n", cm->cheby.nx);
    fprintf(f, "NCOEFF_FREQ %d\n", cm->cheby.ny);

    for (ix=0; ix < cm->cheby.nx; ix++)
    {
        fprintf(f, "COEFFS");
        for (iy=0; iy < cm->cheby.ny; iy++)
        {
            //	fprintf(f, " %.34Lg", cm->cheby.coeff[iy*cm->cheby.nx+ix]);
            fprintf(f, " %.25Lg", cm->cheby.coeff[iy*cm->cheby.nx+ix]);

            
            if ((iy+1)%3==0) fprintf(f, "\n");  // Every 3 coefficients put a new line
        }
        fprintf(f, "\n");
    }
    fprintf(f, "ChebyModel END\n");
}

int ChebyModel_Read(ChebyModel *cm, FILE *f)
{
    int done = 0;
    int first = 1;
    char line[1024], keyword[64], arg[64], junk[1024];
    int nx=-1, ny=-1, ix=0, iy=0;
    int ichar=0, nread=0;

    ChebyModel_Destroy (cm);
    errno=0;

    char str1[128];
    char str2[128];

    do
    {
        if (fgets(line, 1024, f)!=line)
            return -1;

        if (sscanf(line, "%63s", keyword)!=1)
            continue; // skip blank lines
        if (sscanf(line, "%63s %63s", keyword, arg)!=2)
            return -2;
        if (line[0]=='#')
            continue; // skip comment lines
        // check first line
        if (first && (strcasecmp(keyword, "ChebyModel")||strcasecmp(arg, "BEGIN")))
            return -3;
        // parse based on keyword
        if (!strcasecmp(keyword, "PSRNAME"))
            strcpy(cm->psrname, arg);
        else if (!strcasecmp(keyword, "SITENAME"))
            strcpy(cm->sitename, arg);
        else if (!strcasecmp(keyword, "TIME_RANGE"))
        {
            if (sscanf(line, "%*s %127s %127s", str1,str2)!=2)
                return -4;
            cm->mjd_start = (long double)parse_ld_cheby(str1);
            cm->mjd_end = (long double)parse_ld_cheby(str2);
        }
        else if (!strcasecmp(keyword, "FREQ_RANGE"))
        {
            if (sscanf(line, "%*s %127s %127s", str1, str2)!=2)
                return -5;
            cm->freq_start = (long double)parse_ld_cheby(str1);
            cm->freq_end = (long double)parse_ld_cheby(str2);
        }
        else if (!strcasecmp(keyword, "DISPERSION_CONSTANT"))
        {
            if (sscanf(arg, "%127s", str1)!=1)
                return -6;
            cm->dispersion_constant = (long double)parse_ld_cheby(str1);
        }
        else if (!strcasecmp(keyword, "NCOEFF_TIME"))
        {
            if (sscanf(arg, "%d", &nx)!=1) 
                return -7;
        }
        else if (!strcasecmp(keyword, "NCOEFF_FREQ"))
        {
            if (sscanf(arg, "%d", &ny)!=1)
                return -8;
        }
        else if (!strcasecmp(keyword, "COEFFS"))
        {
            if (cm->cheby.coeff==NULL) // first instance of COEFF keyword
            {
                if (nx < 2 || ny < 1 || nx > INT_MAX/ny) // oops, these should come first!
                    return -8;
                /* cheby2d.c:517 && admits a missing dimension; derivative reads nx-2.
                 * AF bounds fix, plus checked native allocation before writes. */
                ChebyModel_Init(cm, nx, ny);
                if (!cm->cheby.coeff || !cm->frequency_cheby.coeff) return -8;
            }
            if (cm->cheby.nx!=nx || cm->cheby.ny!=ny) return -8;
            if (ix >= nx)
                return -9; // too many coefficient lines!!

            sscanf(line, "%*s %n", &ichar);
            if (ny<4) // All on one line
            {
                for (iy=0; iy < cm->cheby.ny; iy++)
                {
                    if (sscanf(line+ichar, "%127s %n",str1, &nread)!=1)
                        return -10;
                    cm->cheby.coeff[iy*cm->cheby.nx+ix] = (long double)parse_ld_cheby(str1);

                    ichar += nread;
                }
            }
            else   // Code added by G. Hobbs for multiple lines in the predictor file
            {
                for (iy=0; iy < cm->cheby.ny; iy++)
                {
                    if (sscanf(line+ichar, "%127s %n",str1, &nread)!=1)
                        return -10;
                    cm->cheby.coeff[iy*cm->cheby.nx+ix] = (long double)parse_ld_cheby(str1);

                    ichar += nread;
                    if ((iy+1)%3==0)
                    {
                        if (sscanf(line+ichar, "%1023s", junk)==1)
                            return -11; // excess stuff at end of line		  
                        ichar = 0;
                        if (fgets(line, 1024, f)!=line)
                            return -1;
                    }
                }
            }
            if (sscanf(line+ichar, "%1023s", junk)==1)
                return -11; // excess stuff at end of line
            ix++;
        }
        else if (!strcasecmp(keyword, "ChebyModel"))
        {
            if ((!first) && !strcasecmp(arg, "BEGIN"))
                return -12;
            else if (!strcasecmp(arg, "END"))
            {
                if (cm->cheby.coeff==NULL || ix!=nx)
                    return -13; // haven't read enough coefficients yet!!
                else
                {
                    Cheby2D_Construct_x_Derivative(&cm->frequency_cheby, &cm->cheby);

                    done = 1;
                }
            }

        }
        else
            return -14; // unrecognized keyword!! 
        if (errno==EINVAL) return -10;
        first = 0;    
    } while (!done);

    return 0;
}


int ChebyModelSet_GetNearestIndex(const ChebyModelSet *cms, long double mjd)
{
    ChebyModelSet_OutOfRange = 0;
    if (cms->nsegments < 0) return -1;
    long double *v = t2r_ld_alloc((size_t)cms->nsegments*2);
    if (cms->nsegments && !v) return -1;
    for (int i=0; i<cms->nsegments; ++i) {
        v[2*i] = cms->segments[i].mjd_start; v[2*i+1] = cms->segments[i].mjd_end;
    }
    int i = t2r_nearest(v, cms->nsegments, 1, &mjd); free(v); return i;
}


int ChebyModelSet_OutOfRange = 0;


ChebyModel *ChebyModelSet_GetNearest(const ChebyModelSet *cms, long double mjd)
{
    int inearest = ChebyModelSet_GetNearestIndex(cms,mjd);

    if (inearest < 0) {
        ChebyModelSet_OutOfRange = 1;
        return 0;
    }

    return &cms->segments[inearest];
}


long double ChebyModelSet_GetPhase(const ChebyModelSet *cms, long double mjd, long double freq)
{
    return ChebyModel_GetPhase(ChebyModelSet_GetNearest(cms, mjd), mjd, freq);
}
long double ChebyModelSet_GetFrequency(const ChebyModelSet *cms, long double mjd, long double freq)
{
    return ChebyModel_GetFrequency(ChebyModelSet_GetNearest(cms, mjd), mjd, freq);
}



void ChebyModelSet_Write(const ChebyModelSet *cms, FILE *f)
{
    int iseg;

    fprintf(f, "ChebyModelSet %d segments\n", cms->nsegments);
    for (iseg=0; iseg < cms->nsegments ; iseg++)
        ChebyModel_Write(&cms->segments[iseg], f);
}

int ChebyModelSet_Read(ChebyModelSet *cms, FILE *f)
{
    /* cheby2d.c:688 publishes all segments before initializing them; failure
     * can make Destroy traverse uninitialized pointers. Initialize all owners
     * before reading; retain partial source fields/cursor and original statuses. */
    ChebyModelSet_Destroy(cms);
    char line[1024],keyword[64]; int n;
    if (!fgets(line,sizeof(line),f) || sscanf(line,"%63s %d",keyword,&n)!=2 ||
        strcasecmp(keyword,"ChebyModelSet") || n<0 || (size_t)n>SIZE_MAX/sizeof(ChebyModel)) return -1;
    if (n==0) return 0;
    cms->segments=calloc((size_t)n,sizeof(ChebyModel));
    if (!cms->segments) return -1;
    cms->nsegments=n;
    for(int i=0;i<n;++i) {int ret=ChebyModel_Read(&cms->segments[i],f);if(ret) return ret;}
    return 0;
}

void ChebyModelSet_Init(ChebyModelSet *cms)
{
    cms->segments = NULL;
    cms->nsegments = 0;
}

int ChebyModelSet_Insert(ChebyModelSet *cms, const ChebyModelSet *from)
{
    /* cheby2d.c:714 loses the allocation on failed realloc and aliases source
     * on self-insert. Snapshot only that alias; preserve source append order. */
    if (cms==from) {
        ChebyModelSet tmp; ChebyModelSet_Init(&tmp);
        int ret=ChebyModelSet_Insert(&tmp,from);
        if (!ret) ret=ChebyModelSet_Insert(cms,&tmp);
        ChebyModelSet_Destroy(&tmp); return ret;
    }
    if (from->nsegments<0 || cms->nsegments<0 || from->nsegments>INT_MAX-cms->nsegments) return -1;
    int old=cms->nsegments, total=old+from->nsegments;
    if (total==0) return 0;
    if ((size_t)total>SIZE_MAX/sizeof(ChebyModel)) return -1;
    ChebyModel *p=realloc(cms->segments,(size_t)total*sizeof(ChebyModel));
    if (!p) return -1;
    cms->segments=p;
    for(int i=old;i<total;++i) {
        ChebyModel_Init(&p[i],0,0); ChebyModel_Copy(&p[i],&from->segments[i-old]);
        cms->nsegments=i+1;
        if ((p[i].cheby.nx && p[i].cheby.ny && !p[i].cheby.coeff) ||
            (p[i].frequency_cheby.nx && p[i].frequency_cheby.ny && !p[i].frequency_cheby.coeff)) return -1;
    }
    return 0;
}

/*
   This method destroys all ChebyModel elements that are no longer required
   The MJD array defines what is required; only the nearest ChebyModel
   to each MJD is kept.
   */ 
    void
ChebyModelSet_Keep(ChebyModelSet *cms, unsigned nmjd, const long double* mjd)
{
    unsigned nseg = cms->nsegments;  // number of segments in input
    unsigned new_nseg = nseg;        // number of segments kept

    unsigned iseg = 0;               // current segment index
    unsigned rem_iseg = 0;           // remaining segment index

    unsigned i = 0;                  // counter
    char* keep = malloc (nseg);      // array of "ChebyModel to be kept" flags

    if (nseg && !keep) { errno=ENOMEM; return; }
    if (nseg) memset (keep, 0, nseg);          // set all keep flags to false

    for (i=0; i<nmjd; i++)
    {
        int index = ChebyModelSet_GetNearestIndex (cms, mjd[i]);
        if (index >= 0)
            keep[index] = 1;         // set flag to true
    }

    for (i=0; i<nseg; i++)
        if ( keep[i] )
            iseg ++;
        else
        {
            ChebyModel_Destroy(&cms->segments[iseg]);
            // shift the remaining ChebyModel elements "to the left"
            for (rem_iseg = iseg; rem_iseg < new_nseg-1; rem_iseg++)
                cms->segments[rem_iseg] = cms->segments[rem_iseg+1];

            new_nseg --;
        }

    cms->nsegments = new_nseg;

    free (keep);
}


void ChebyModelSet_Destroy(ChebyModelSet *cms)
{
#if _DEBUG
    fprintf (stderr, "ChebyModelSet_Destroy this=%p segments=%p\n",
             (void*)cms, (void*)(cms->segments));
#endif

    int iseg;
    for (iseg=0; iseg < cms->nsegments ; iseg++)
        ChebyModel_Destroy(&cms->segments[iseg]);

    if (cms->segments)
      free(cms->segments);

    cms->segments = NULL;
    cms->nsegments = 0;
}


//  Copyright (C) 2006,2007,2008,2009, George Hobbs, Russel Edwards

/*
 *    This file is part of TEMPO2. 
 * 
 *    TEMPO2 is free software: you can redistribute it and/or modify 
 *    it under the terms of the GNU General Public License as published by 
 *    the Free Software Foundation, either version 3 of the License, or 
 *    (at your option) any later version. 
 *    TEMPO2 is distributed in the hope that it will be useful, 
 *    but WITHOUT ANY WARRANTY; without even the implied warranty of 
 *    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the 
 *    GNU General Public License for more details. 
 *    You should have received a copy of the GNU General Public License 
 *    along with TEMPO2.  If not, see <http://www.gnu.org/licenses/>. 
 */

/*
 *    If you use TEMPO2 then please acknowledge it by citing 
 *    Hobbs, Edwards & Manchester (2006) MNRAS, Vol 369, Issue 2, 
 *    pp. 655-672 (bibtex: 2006MNRAS.369..655H)
 *    or Edwards, Hobbs & Manchester (2006) MNRAS, VOl 372, Issue 4,
 *    pp. 1549-1574 (bibtex: 2006MNRAS.372.1549E) when discussing the
 *    timing model.
 */


/* Tempo1 polyco functions destined for tempo2pred library (C linkage) */

long double T1Polyco_GetPhase(const T1Polyco *t1p, long double mjd, long double freq)
{
    /* t1polyco.c:43-45 diagnostics are unconditional, even tempo2_verbose=0. */
    printf("WARNIGN WARNGING: Silly update in polyco read\n");
    printf("Have %.20lg %.20lg\n", (double)mjd, (double)t1p->mjd_mid);
    printf("Diff = %.15lg\n", (double)(mjd-t1p->mjd_mid));
    long double v[] = {t1p->mjd_mid, t1p->reference_phase, t1p->frequency_psr_0, mjd, freq}, out=NAN;
    t2r_t1phase(t1p->coeff, t1p->ncoeff, v, t1p->dm, t1p->doppler, t1p->frequency_obs, &out);
    return out;
}

long double T1Polyco_GetFrequency(const T1Polyco *t1p, long double mjd, 
        long double freq)
{
    (void)freq; /* Source does not read phase, C0, DM or observing frequency. */
    long double v[] = {t1p->mjd_mid, t1p->frequency_psr_0, mjd}, out=NAN;
    t2r_t1frequency(t1p->coeff+1, t1p->ncoeff, v, &out); return out;
}

void T1Polyco_Write(const T1Polyco *t1p, FILE *fout)
{
    int i;

    if (t1p->psrname[0]=='J') 
        fprintf(fout,"%-10.10s ",t1p->psrname+1);
    else 
        fprintf(fout,"%-10.10s ",t1p->psrname);
    fprintf(fout,"%9.9s",t1p->date_string);
    fprintf(fout,"%11.11s",t1p->utc_string);
    fprintf(fout,"%20.11Lf",(long double)(t1p->mjd_mid));
    fprintf(fout,"%21.6lf ",t1p->dm);
    fprintf(fout,"%6.3lf",t1p->doppler);
    fprintf(fout,"%7.3lf",t1p->log10rms);
    fprintf(fout,"\n");
    fprintf(fout,"%20.6Lf",(long double)t1p->reference_phase);
    fprintf(fout,"%18.12Lf",(long double)t1p->frequency_psr_0);
    fprintf(fout,"%5s",t1p->sitename);
    fprintf(fout,"%5d",t1p->span);
    fprintf(fout,"%5d",t1p->ncoeff);
    fprintf(fout,"%10.3lf",t1p->frequency_obs);
    fprintf(fout,"%16.14lf",t1p->binary_phase);
    fprintf(fout,"\n");
    for (i=0;i<t1p->ncoeff;i++)
    {
        fprintf(fout,"%25.17Le", (long double)t1p->coeff[i]);
        if ((i+1)%3==0) fprintf(fout,"\n");
    }
    /* t1polyco.c:111 omitted final newline, joining the next record (AF fix). */
    if (t1p->ncoeff % 3) fprintf(fout,"\n");
}

/* helper functions for T1Polyco_Read */
void T1P_grabString(char *str, int istart, int nchar, char *out)
{
    /* t1polyco.c:119 copies beyond short lines and leaves blank output unwritten.
     * Reject invalid extents; blank strings are defined empty. */
    out[0]='\0';
    if (istart < 0 || nchar < 0 || nchar >= 64 || strlen(str) < (size_t)(istart+nchar)) { errno=EINVAL; return; }
    char grabbed[64]; memcpy(grabbed,str+istart,(size_t)nchar); grabbed[nchar]='\0';
    sscanf(grabbed,"%63s",out);
}

long double T1P_grabLongDouble(char *str, int istart, int nchar)
{
    char grabbed[64]; T1P_grabString(str,istart,nchar,grabbed);
    for (char *c=grabbed; *c; ++c) if (*c=='D') *c='e';
    long double out=NAN;
    if (sscanf(grabbed,"%Lf",&out)!=1) errno=EINVAL;
    return out;
}

long double T1P_grabInt(char *str, int istart, int nchar)
{
    char grabbed[64]; T1P_grabString(str,istart,nchar,grabbed); int out=0;
    if (sscanf(grabbed,"%d",&out)!=1) { errno=EINVAL; return NAN; }
    return out;
}

/* fda6ad4 t1polyco.c:149 unbounded %s can overflow exposed fixed arrays.
 * Widths include space for terminators; all successful partial writes remain. */
int T1Polyco_Read_NewFormat(T1Polyco *t1p, FILE *f)
{
    int ic;
    char junk[1024];
    long double mjd_mid, reference_phase,frequency_psr_0,coeff;

    if (fscanf(f, "%63s %9s %12s %Lf %lf %lf %lf %Lf %Lf %4s %d %d %lf %lf %lf",
                t1p->psrname, t1p->date_string, t1p->utc_string,
                &mjd_mid, &t1p->dm, &t1p->doppler, &t1p->log10rms,
                &reference_phase, &frequency_psr_0,
                t1p->sitename, &t1p->span, &t1p->ncoeff,
                &t1p->frequency_obs, &t1p->binary_phase, &t1p->binary_frequency)
            != 15)
        return -1;
    t1p->mjd_mid = mjd_mid;
    t1p->reference_phase = reference_phase;
    t1p->frequency_psr_0 = frequency_psr_0;

    /* t1polyco.c:164 writes past fixed coeff[32] for malformed counts.
     * Bound only capacity, not the owned Rust constructor's minimum of two. */
    if (t1p->ncoeff < 0 || t1p->ncoeff > 32) return -1;
    for (ic=0; ic < t1p->ncoeff; ic++){
        if (fscanf(f, "%Lf", &coeff)!=1)
            return -1;
        t1p->coeff[ic] = coeff;
    }
    fgets(junk, 1024, f); /* eat final newline */
    return 0;
}

int T1Polyco_Read(T1Polyco *t1p, FILE *f)
{
    char line[128];
    int ic;
    errno=0;

    if (fgets(line, 128, f)!=line)
        return -1;

    if (!strncasecmp(line, "TEMPO2:", 6))
        return T1Polyco_Read_NewFormat(t1p, f);

    T1P_grabString(line, 0, 11, t1p->psrname);
    T1P_grabString(line, 11, 9, t1p->date_string);
    T1P_grabString(line, 20, 11, t1p->utc_string);
    t1p->mjd_mid = T1P_grabLongDouble(line, 31, 20);
    t1p->dm = T1P_grabLongDouble(line, 51, 21);
    /* AF correction: polyco.C:419 minimum-width Doppler can exceed six columns. */
    char *dot=strlen(line)>73 ? strchr(line+73,'.') : NULL;
    int width=dot ? (int)(dot-(line+73))+4 : 6; if (width<6) width=6;
    t1p->doppler = T1P_grabLongDouble(line,73,width);
    t1p->log10rms = T1P_grabLongDouble(line,73+width,7);

    if (fgets(line, 128, f)!=line)
        return -1;
    t1p->reference_phase = T1P_grabLongDouble(line, 0, 20);
    t1p->frequency_psr_0 = T1P_grabLongDouble(line, 21, 18);
    T1P_grabString(line, 38, 5, t1p->sitename);
    long double sp=T1P_grabInt(line,43,5); if (!isfinite(sp)) return -1; t1p->span=(int)sp;
    long double nc=T1P_grabInt(line,48,5);
    if (!isfinite(nc) || nc < 0 || nc > 32) return -1;
    t1p->ncoeff=(int)nc;
    t1p->frequency_obs = T1P_grabLongDouble(line, 53, 10);
    /* AF correction: blank isolated binary metadata has new-format zero pair. */
    char bin[64]; T1P_grabString(line,63,16,bin);
    if (!bin[0]) { t1p->binary_phase=0; t1p->binary_frequency=0; }
    else { t1p->binary_phase=T1P_grabLongDouble(line,63,7); t1p->binary_frequency=T1P_grabLongDouble(line,70,9); }
    if (errno==EINVAL) return -1;

    for (ic=0; ic < t1p->ncoeff; ic+=3)
    {
        if (fgets(line, 128, f)!=line)
            return -1;
        t1p->coeff[ic] = T1P_grabLongDouble(line, 0, 25);
        if (ic+1 < t1p->ncoeff)
            t1p->coeff[ic+1] = T1P_grabLongDouble(line, 25, 25);
        if (ic+2 < t1p->ncoeff)
            t1p->coeff[ic+2] = T1P_grabLongDouble(line, 50, 25);
    }

    return errno==EINVAL ? -1 : 0;
}

T1Polyco *T1PolycoSet_GetNearest(const T1PolycoSet *t1ps, long double mjd)
{
    if (t1ps->nsegments <= 0) return NULL; /* Source empty-set dereference was undefined. */
    long double *v = t2r_ld_alloc((size_t)t1ps->nsegments);
    if (!v) return NULL;
    for (int i=0; i<t1ps->nsegments; ++i) v[i] = t1ps->segments[i].mjd_mid;
    int i=t2r_nearest(v, t1ps->nsegments, 0, &mjd); free(v);
    return i < 0 ? NULL : &t1ps->segments[i];
}

long double T1PolycoSet_GetPhase(const T1PolycoSet *t1ps, long double mjd, long double freq)
{
    T1Polyco *p=T1PolycoSet_GetNearest(t1ps,mjd); return p ? T1Polyco_GetPhase(p,mjd,freq) : NAN;
}

long double T1PolycoSet_GetFrequency(const T1PolycoSet *t1ps, long double mjd, long double freq)
{
    T1Polyco *p=T1PolycoSet_GetNearest(t1ps,mjd); return p ? T1Polyco_GetFrequency(p,mjd,freq) : NAN;
}

void T1PolycoSet_Write(const T1PolycoSet *t1ps, FILE *f)
{
    int iseg;
    for (iseg=0; iseg < t1ps->nsegments ; iseg++)
        T1Polyco_Write(&t1ps->segments[iseg], f);
}

int T1PolycoSet_Read(T1PolycoSet *t1ps, FILE *f)
{
    /* t1polyco.c:255 overwrites old backing and unchecked realloc loses it.
     * Reuse Destroy and checked temporaries; completed prefix still succeeds. */
    T1PolycoSet_Destroy(t1ps);
    size_t capacity=5;
    t1ps->segments=malloc(capacity*sizeof(T1Polyco));
    if (!t1ps->segments) return -1;
    while (T1Polyco_Read(&t1ps->segments[t1ps->nsegments],f)==0) {
        /* Reading counts 0/1 is defined. Leave omitted backing untouched:
         * callers may inspect metadata and supply C0/C1 before evaluation. */
        ++t1ps->nsegments;
        if ((size_t)t1ps->nsegments==capacity) {
            if (capacity>(SIZE_MAX/sizeof(T1Polyco))-5 || capacity>INT_MAX-5) return -1;
            capacity+=5; T1Polyco *p=realloc(t1ps->segments,capacity*sizeof(T1Polyco));
            if (!p) return -1; t1ps->segments=p;
        }
    }
    return t1ps->nsegments>0 ? 0 : -1;
}

void T1PolycoSet_Init(T1PolycoSet *t1ps)
{
  t1ps->segments = NULL;
  t1ps->nsegments = 0;
}

void T1PolycoSet_Destroy(T1PolycoSet *t1ps)
{
  if (t1ps->segments)
    free(t1ps->segments);
  T1PolycoSet_Init(t1ps);
}

