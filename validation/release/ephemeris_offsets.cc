// Diagnostic callback for readEphemeris.C's three offset axes. The generated
// include contains the original slope, intercept and evaluation expressions.
#include <cassert>
#include <cinttypes>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <initializer_list>

void al_ephemeris_offset(int axis, int pulsar, int observation, int interval,
                        double t0, double t1, double v0, double v1, double sat,
                        double actual_m, double actual_c, double actual_offset) {
    double m, c, ival;
#include "ephemeris_interpolation.inc"
    FILE *out = fopen(getenv("AL_EPHEMERIS_OFFSETS"), "a");
    assert(out);
    fprintf(out, "%d\t%d\t%d\t%d", axis, pulsar, observation, interval);
    for (double value : {t0, t1, v0, v1, sat, actual_m, actual_c, actual_offset, m, c, ival}) {
        uint64_t bits;
        memcpy(&bits, &value, sizeof bits);
        fprintf(out, "\t%016" PRIx64, bits);
    }
    assert(fprintf(out, "\n") > 0);
    assert(fclose(out) == 0);
}
