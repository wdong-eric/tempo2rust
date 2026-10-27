/* T2accel.C/readEphemeris_calceph.C provider ABI glue only.
 * Compile against the selected provider headers, never the legacy timing core.
 * Scalar pointers below live for the synchronous call. Rust validates array
 * extents and unique mutable borrows; providers neither retain nor free them. */
_Static_assert(sizeof(int) == 4, "32-bit C int required");
_Static_assert(sizeof(double) == 8, "binary64 C double required");
#ifdef T2_BLAS
#include <Accelerate/Accelerate.h>
_Static_assert(sizeof(__CLPK_integer) == 4, "LP64 LAPACK integer required");
_Static_assert(sizeof(__CLPK_doublereal) == 8, "binary64 required");
/* fda6ad4 T2accel.C:16-35 hides these under HAVE_LAPACK despite BLAS-only
 * callers. Separate declaration guards fix that demonstrated build defect. */
void t2_dgemv(int rows, int cols, const double *a, const double *x, double *y) {
    int one=1; double alpha=1, beta=0; char trans='T';
    dgemv_(&trans,&cols,&rows,&alpha,a,&cols,x,&one,&beta,y,&one);
}
void t2_dgemm(int rows, int inner, int cols, const double *a, const double *b, double *c) {
    double alpha=1, beta=0; char trans='N';
    dgemm_(&trans,&trans,&cols,&rows,&inner,&alpha,b,&cols,a,&inner,&beta,c,&cols);
}
#ifdef T2_LAPACK
int t2_dpotrf(int n, double *a) {
    int info; char uplo='U'; dpotrf_(&uplo,&n,a,&n,&info); return info;
}
int t2_dpotf2(int n, double *a) {
    int info; char uplo='L'; dpotf2_(&uplo,&n,a,&n,&info); return info;
}
int t2_dtptri(int n, double *a, int upper) {
    int info; char uplo=upper?'U':'L', diag='N';
    dtptri_(&uplo,&diag,&n,a,&info); return info;
}
int t2_dgels(int rows, int cols, double *a, double *b, double *work, int lwork) {
    int info, nrhs=1; char trans='T';
    dgels_(&trans,&cols,&rows,&nrhs,a,&cols,b,&rows,work,&lwork,&info); return info;
}
void t2_dtrmm(int n, double alpha, double *a, double *b) {
    char side='R', uplo='U', trans='T', diag='N';
    dtrmm_(&side,&uplo,&trans,&diag,&n,&n,&alpha,a,&n,b,&n);
}
#endif
#endif
#ifdef T2_CALCEPH
#include <calceph.h>
int t2_calceph_position(t_calcephbin *eph, double jd0, double jd1, int target, int centre, double *pv) {
    return calceph_compute_unit(eph,jd0,jd1,target,centre,CALCEPH_UNIT_KM|CALCEPH_UNIT_SEC,pv);
}
#endif
