#include <cstdio>
#include <cfloat>
#include <quadmath.h>
int main() {
    __float128 eps = 1;
    int bits = 0;
    while (1 + eps != 1) { eps /= 2; ++bits; }
    const __float128 offset = strtoflt128("4e-9", NULL);
    const __float128 at = strtoflt128("50000", NULL);
    const __float128 recovered = ((at + offset / 86400) - at) * 86400;
    char value[128], error[128];
    quadmath_snprintf(value, sizeof value, "%.36Qg", recovered);
    quadmath_snprintf(error, sizeof error, "%.36Qg", fabsq(recovered-offset));
    printf("{\"backend\":\"__float128\",\"significand_bits\":%d,\"sizeof_bytes\":%zu,"
           "\"decimal_digits\":%d,\"native_long_double_bits\":%d,\"recovered_4ns_s\":\"%s\","
           "\"absolute_error_s\":\"%s\"}\n", bits, sizeof(at), FLT128_DIG, LDBL_MANT_DIG, value, error);
    return bits == 113 && strtoflt128(value, NULL) == recovered && fabsq(recovered-offset) < 1e-20Q ? 0 : 1;
}
