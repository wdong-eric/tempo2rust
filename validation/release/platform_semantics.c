/* TKfit.C:461 square-fit error scaling and t1polyco.c:106 serialization.
 * Observe native arithmetic/format boundaries without normalizing NaN signs. */
#include <float.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
int main(void) {
    volatile double zero = 0.0;
    double multiplier = sqrt(zero / zero);
    printf("square-fit\t%d\t%d\t%a\n", !!isnan(multiplier),
           !!signbit(multiplier), multiplier);
    double input[] = {0.1, 0.2, 0.3};
    printf("long-double-bits\t%d\n", LDBL_MANT_DIG);
    for (int i = 0; i < 3; i++) {
        char text[64];
        snprintf(text, sizeof(text), "%25.17Le", (long double)input[i]);
        printf("coefficient\t%d\t%s\t%La\t%La\n", i, text,
               (long double)input[i], strtold(text, NULL));
    }
}
