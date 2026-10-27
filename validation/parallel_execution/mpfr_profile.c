/* Query the actual static MPFR linked into the tested Rust profile. */
#include <mpfr.h>
#include <stdio.h>
int main(void) {
    printf("MPFR %s TLS %d\n",mpfr_get_version(),mpfr_buildopt_tls_p());
    return mpfr_buildopt_tls_p() ? 0 : 1;
}
