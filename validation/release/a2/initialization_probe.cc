#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include "tempo2.h"

int main(int argc, char **argv) {
    if (argc != 4) return 2;
    const int expected = atoi(argv[3]);
    if (expected != 0 && expected != 2) return 2;
    pulsar *p = new pulsar;
    // A nonzero storage pattern must not become a parser index or delay input.
    memset(p,0x55,sizeof(*p));
    MAX_PSR=1; initialise(p,0);
    assert(p->ne_sw_ifuncN == 0);
    char par[1][MAX_FILELEN],tim[1][MAX_FILELEN];
    strcpy(par[0],argv[1]); strcpy(tim[0],argv[2]);
    readParfile(p,par,tim,1); readTimfile(p,tim,1);
    assert(p->ne_sw_ifuncN == expected);
    if (expected) {
        assert(p->ne_sw_ifuncT[0] == 49000 && p->ne_sw_ifuncT[1] == 51000);
        assert(p->ne_sw_ifuncV[0] == 2 && p->ne_sw_ifuncV[1] == 6);
        assert(p->ne_sw_ifuncE[0] == .25 && p->ne_sw_ifuncE[1] == .5);
    }
    preProcessSimple(p);
    assert(p->ne_sw_ifuncN == expected);
    formBatsAll(p,1); formResiduals(p,1,0);
    assert(p->ne_sw_ifuncN == expected);
    double max_error=0;
    for (int i=0; i<p->nobs; ++i) {
        const observation &o=p->obsn[i];
        double extra=0;
        if (expected) {
            const double t=static_cast<double>(o.sat);
            extra = t < 49000 ? 2 : t >= 51000 ? 6 : 2+4*(t-49000)/2000;
        }
        const double predicted=p->ne_sw*o.spherical_solar_wind + extra*o.spherical_solar_wind;
        assert(std::isfinite(o.tdis2) && !isnanq(o.residual) && !isinfq(o.residual));
        max_error=fmax(max_error,fabs(o.tdis2-predicted));
        assert(fabs(o.tdis2-predicted) <= 1e-20);
        if (expected) assert(o.tdis2 > p->ne_sw*o.spherical_solar_wind);
    }
    if (expected) {
        assert(p->ne_sw_ifuncT[0] == 49000 && p->ne_sw_ifuncT[1] == 51000);
        assert(p->ne_sw_ifuncV[0] == 2 && p->ne_sw_ifuncV[1] == 6);
        assert(p->ne_sw_ifuncE[0] == .25 && p->ne_sw_ifuncE[1] == .5);
    }
    printf("{\"storage_pattern\":\"0x55 whole struct\",\"after_initialise\":0,\"after_parse\":%d,\"after_preprocess\":%d,\"after_delays\":%d,\"sample_values_preserved\":true,\"max_delay_error_s\":\"%.17g\",\"delay_tolerance_s\":\"1e-20\",\"observations\":%d}\n",expected,expected,p->ne_sw_ifuncN,max_error,p->nobs);
    destroyOne(p); delete p;
}
