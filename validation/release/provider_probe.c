/* Actual provider ABI and loaded-artifact witness. No timing engine algorithm. */
#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif
#include <assert.h>
#include <dlfcn.h>
#include <float.h>
#include <stdio.h>
#include <string.h>
#include <calceph.h>
#ifdef __APPLE__
#include <Accelerate/Accelerate.h>
typedef __CLPK_integer provider_int;
#else
#include <cblas.h>
#include <lapack.h>
typedef blasint provider_int;
_Static_assert(sizeof(lapack_int) == 4, "LAPACK LP64 required");
#endif
_Static_assert(sizeof(provider_int) == 4, "BLAS LP64 required");
void al_fortran_abi(int *, int *, int *);
void t2_dgemv(int, int, const double *, const double *, double *);
void t2_dgemm(int, int, int, const double *, const double *, double *);
int t2_dpotrf(int, double *);
int t2_dpotf2(int, double *);
int t2_dtptri(int, double *, int);
int t2_dgels(int, int, double *, double *, double *, int);
void t2_dtrmm(int, double, double *, double *);
int main(int argc, char **argv) {
    assert(argc == 2);
    printf("int=%zu pointer=%zu double=%zu long_double=%zu long_double_bits=%d\n",
           sizeof(int), sizeof(void *), sizeof(double), sizeof(long double), LDBL_MANT_DIG);
    unsigned word=1; printf("little_endian=%d\n",*(unsigned char *)&word==1);
    int native_bits=0,c_bits=0,round_trip=41;
    al_fortran_abi(&native_bits,&c_bits,&round_trip);
    assert(native_bits==32 && c_bits==32 && round_trip==42);
    printf("fortran_default_integer_bits=%d c_integer_bits=%d round_trip=%d\n",native_bits,c_bits,round_trip);
    Dl_info image;
#ifdef __APPLE__
    assert(dladdr((void *)dgemm_, &image));
#else
    assert(dladdr((void *)cblas_dgemm, &image));
    printf("openblas_config=%s\nopenblas_threads=%d\nopenblas_parallel=%d\n",
           openblas_get_config(),openblas_get_num_threads(),openblas_get_parallel());
    assert(!strstr(openblas_get_config(),"USE64BITINT"));
#endif
    printf("blas_loaded=%s\n",image.dli_fname);
    double a[]={1,2,3,4,5,6},x[]={2,3,4},y[2]={0,0},c[4]={0,0,0,0};
    t2_dgemv(2,3,a,x,y);assert(y[0]==20 && y[1]==47);
    double b[]={1,0,0,1,1,1};t2_dgemm(2,3,2,a,b,c);
    assert(c[0]==4 && c[1]==5 && c[2]==10 && c[3]==11);
    double upper[]={4,2,2,5},lower[]={4,2,2,5};
    assert(t2_dpotrf(2,upper)==0 && t2_dpotf2(2,lower)==0);
    assert(upper[0]==2 && upper[2]==1 && upper[3]==2);
    assert(lower[0]==2 && lower[1]==1 && lower[3]==2);
    double packed[]={2,1,2};assert(t2_dtptri(2,packed,1)==0);
    assert(packed[0]==.5 && packed[1]==-.25 && packed[2]==.5);
    double triangular[]={2,0,1,2},identity[]={1,0,0,1};
    t2_dtrmm(2,1,triangular,identity);
    assert(identity[0]==2 && identity[1]==1 && identity[2]==0 && identity[3]==2);
    double design[]={1,0,0,1,1,1},rhs[]={1,2,3},query=0;
    assert(t2_dgels(3,2,design,rhs,&query,-1)==0 && query>0);
    double work[1024];assert(query<=1024);
    assert(t2_dgels(3,2,design,rhs,work,1024)==0);
    assert(rhs[0]>.999999999999 && rhs[0]<1.000000000001);
    assert(rhs[1]>1.999999999999 && rhs[1]<2.000000000001);
    printf("matrix_abi_checks=pass\ncalceph_version=%s\n",CALCEPH_VERSION_STRING);
    t_calcephbin *e=calceph_open(argv[1]);assert(e);
    double start,end,pv[6]={0};int continuous;char version[CALCEPH_MAX_CONSTANTVALUE];
    assert(calceph_gettimespan(e,&start,&end,&continuous));
    assert(calceph_getfileversion(e,version));
    printf("ephemeris=%s span=%.17g,%.17g continuous=%d timescale=%d\n",version,start,end,continuous,calceph_gettimescale(e));
    int ok=calceph_compute(e,2458000,.125,16,0,pv);
    printf("target16=%d value=%.17g\n",ok,pv[0]);calceph_close(e);assert(ok);
    return 0;
}
