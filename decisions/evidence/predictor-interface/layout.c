#include "tempo2pred.h"
#include <stddef.h>
#include <float.h>
#ifdef __cplusplus
#define ALIGNOF(T) alignof(T)
#else
#define ALIGNOF(T) _Alignof(T)
#endif
#define TYPE(T) printf("TYPE %s %zu %zu\n", #T, sizeof(T), ALIGNOF(T))
#define FIELD(T,F) printf("FIELD %s.%s %zu %zu\n", #T, #F, offsetof(T,F), sizeof(((T*)0)->F))
int main(void) {
    printf("LONG_DOUBLE %d %d %d\n", LDBL_MANT_DIG, LDBL_MIN_EXP, LDBL_MAX_EXP);
    printf("KINDS %d %d %d\n", NonePredType, Cheby, T1);
    TYPE(long double);
    TYPE(T2PredictorKind);
    TYPE(Cheby2D);
    FIELD(Cheby2D,nx);
    FIELD(Cheby2D,ny);
    FIELD(Cheby2D,coeff);
    TYPE(ChebyModel);
    FIELD(ChebyModel,psrname);
    FIELD(ChebyModel,sitename);
    FIELD(ChebyModel,mjd_start);
    FIELD(ChebyModel,mjd_end);
    FIELD(ChebyModel,freq_start);
    FIELD(ChebyModel,freq_end);
    FIELD(ChebyModel,dispersion_constant);
    FIELD(ChebyModel,cheby);
    FIELD(ChebyModel,frequency_cheby);
    TYPE(ChebyModelSet);
    FIELD(ChebyModelSet,segments);
    FIELD(ChebyModelSet,nsegments);
    TYPE(T1Polyco);
    FIELD(T1Polyco,psrname);
    FIELD(T1Polyco,date_string);
    FIELD(T1Polyco,utc_string);
    FIELD(T1Polyco,mjd_mid);
    FIELD(T1Polyco,dm);
    FIELD(T1Polyco,doppler);
    FIELD(T1Polyco,log10rms);
    FIELD(T1Polyco,reference_phase);
    FIELD(T1Polyco,frequency_psr_0);
    FIELD(T1Polyco,sitename);
    FIELD(T1Polyco,span);
    FIELD(T1Polyco,ncoeff);
    FIELD(T1Polyco,frequency_obs);
    FIELD(T1Polyco,binary_phase);
    FIELD(T1Polyco,binary_frequency);
    FIELD(T1Polyco,coeff);
    TYPE(T1PolycoSet);
    FIELD(T1PolycoSet,segments);
    FIELD(T1PolycoSet,nsegments);
    TYPE(T2Predictor);
    FIELD(T2Predictor,kind);
    FIELD(T2Predictor,modelset);
    return 0;
}
