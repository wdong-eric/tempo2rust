// A process-isolated probe of the pinned TEMPO2 entry points. No fitting.
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>
#include <vector>
#include <stdexcept>
#include "tempo2.h"
#include "resource_trace.h"

static FILE *out;
static const char *stage;
static std::string number(longdouble value) {
    if (isnanq(value) || isinfq(value)) throw std::runtime_error("nonfinite exported value");
    char text[160];
    quadmath_snprintf(text, sizeof text, "%.36Qg", value);
    if (strtoflt128(text, NULL) != value) throw std::runtime_error("decimal round trip failed");
    return text;
}
static void field(int i, const char *name, const std::string &value) {
    if (value.find_first_of("\t\r\n") != std::string::npos) throw std::runtime_error("TSV control character");
    fprintf(out, "obs\t%s\t%d\t%s\t%s\n", stage, i, name, value.c_str());
}
static void numeric(int i, const char *name, longdouble value) {
    if (isnanq(value) || isinfq(value))
        throw std::runtime_error(std::string("nonfinite ")+stage+"["+std::to_string(i)+"]."+name);
    field(i, name, number(value));
}
static void config(const char *name, const std::string &value) {
    fprintf(out, "config\t%s\t%s\n", name, value.c_str());
}
static void snapshot(pulsar *p, const char *name, bool bats, bool residuals) {
    stage = name;
    for (int i=0; i<p->nobs; ++i) {
        const observation &o = p->obsn[i];
        field(i,"fname",o.fname); field(i,"site",o.telID);
        field(i,"deleted",std::to_string(o.deleted));
        field(i,"clockCorr",std::to_string(o.clockCorr));
        field(i,"delayCorr",std::to_string(o.delayCorr));
        numeric(i,"sat_mjd",o.sat);
        numeric(i,"frequency_mhz",o.freq); numeric(i,"toa_error_us",o.toaErr);
        for (int k=0; k<o.nFlags; ++k) {
            field(i,("flag_id_"+std::to_string(k)).c_str(),o.flagID[k]);
            field(i,("flag_value_"+std::to_string(k)).c_str(),o.flagVal[k]);
        }
        if (bats) {
            numeric(i,"bat_mjd",o.bat); numeric(i,"bbat_mjd",o.bbat);
            if (o.delayCorr) {
                numeric(i,"bat_correction_day",o.batCorr);
                numeric(i,"clock_tt_s",getCorrectionTT(p->obsn+i));
                for (int k=0; k<o.nclock_correction; ++k) {
                    field(i,("clock_to_"+std::to_string(k)).c_str(),o.correctionsTT[k].corrects_to);
                    numeric(i,("clock_step_s_"+std::to_string(k)).c_str(),o.correctionsTT[k].correction);
                }
                numeric(i,"tt_tb_s",o.correctionTT_TB);
                numeric(i,"tt_teph_s",o.correctionTT_Teph);
                numeric(i,"roemer_s",o.roemer);
                numeric(i,"shapiro_sun_s",o.shapiroDelaySun);
                numeric(i,"shapiro_jupiter_s",o.shapiroDelayJupiter);
                numeric(i,"shapiro_saturn_s",o.shapiroDelaySaturn);
                numeric(i,"shapiro_venus_s",o.shapiroDelayVenus);
                numeric(i,"shapiro_uranus_s",o.shapiroDelayUranus);
                numeric(i,"shapiro_neptune_s",o.shapiroDelayNeptune);
                numeric(i,"troposphere_s",o.troposphericDelay);
                numeric(i,"dispersion_s",o.tdis1); numeric(i,"solar_dispersion_s",o.tdis2);
                numeric(i,"frequency_ssb_hz",o.freqSSB);
                for (int k=0;k<3;++k) {
                    numeric(i,("earth_ssb_light_s_"+std::to_string(k)).c_str(),o.earth_ssb[k]);
                    numeric(i,("observatory_earth_light_s_"+std::to_string(k)).c_str(),o.observatory_earth[k]);
                }
            }
            if (strcmp(name,"direct_initial") && strcmp(name,"aligned") && strcmp(name,"perturbed"))
                numeric(i,"shklovskii_s",o.shklovskii);
        }
        if (residuals) {
            numeric(i,"phase_turn",o.phase); numeric(i,"nphase_turn",o.nphase);
            field(i,"pulse_number",std::to_string(o.pulseN));
            numeric(i,"residual_s",o.residual); numeric(i,"binary_delay_s",o.torb);
        }
    }
    fflush(out);
}

int main(int argc, char **argv) {
    if (argc != 5 || !getenv("TEMPO2")) {
        fprintf(stderr,"usage: TEMPO2=runtime export_reference PAR TIM standard|preprocess|subtractBATs|offsetSATs OUTPUT.tsv\n");
        return 2;
    }
    try {
        const std::string mode(argv[3]);
        if (mode!="standard" && mode!="preprocess" && mode!="subtractBATs" && mode!="offsetSATs")
            throw std::runtime_error("unknown scenario");
        char par[1][MAX_FILELEN], tim[1][MAX_FILELEN];
        if (strlen(argv[1])>=MAX_FILELEN || strlen(argv[2])>=MAX_FILELEN) throw std::runtime_error("input path too long");
        strcpy(par[0],argv[1]); strcpy(tim[0],argv[2]);
        out = fopen(argv[4],"w");
        if (!out) throw std::runtime_error("cannot open output");
        MAX_PSR=1;
        // Match upstream initialization; allocate the large pulsar struct off stack.
        pulsar *p = new pulsar;
        const char *pattern = getenv("REFERENCE_TEST_STORAGE_PATTERN");
        if (pattern) {
            if (strcmp(pattern,"55") && strcmp(pattern,"aa")) throw std::runtime_error("unknown storage test pattern");
            const int byte = !strcmp(pattern,"55") ? 0x55 : 0xaa;
            // Poison just the corrected collection before initialization, never after parsing.
            memset(&p->ne_sw_ifuncN,byte,sizeof p->ne_sw_ifuncN);
            memset(p->ne_sw_ifuncT,byte,sizeof p->ne_sw_ifuncT);
            memset(p->ne_sw_ifuncV,byte,sizeof p->ne_sw_ifuncV);
            memset(p->ne_sw_ifuncE,byte,sizeof p->ne_sw_ifuncE);
        }
        reference_stage("initialise"); initialise(p,0);
        reference_stage("readParfile"); readParfile(p,par,tim,1);
        reference_stage("readTimfile"); readTimfile(p,tim,1);
        if (p->nobs<=0) throw std::runtime_error("no TOAs");
        snapshot(p,"parsed",false,false);
        if (mode=="preprocess") { reference_stage("preProcessSimple"); preProcessSimple(p); }
        config("storage_initialization",pattern ? std::string("solar-wind storage poisoned 0x")+pattern+" before corrected initialise" : "corrected initialise; no added zero fill");
        config("solar_wind_sample_count",std::to_string(p->ne_sw_ifuncN));
        config("mode", mode); config("units",p->units==SI_UNITS?"TCB":"TDB");
        config("clock",p->clock); config("ephemeris",p->ephemeris);
        config("jpl_ephemeris",p->JPL_EPHEMERIS); config("eop_file",p->eopc04_file);
        config("time_ephemeris",p->timeEphemeris==IF99_TIMEEPH?"IF99":"FB90");
        config("t2c_method",std::to_string(p->t2cMethod));
        config("refphs",std::to_string(p->refphs)); config("remove_mean","0");
        config("binary_model",p->binaryModel); config("dilate_frequency",std::to_string(p->dilateFreq));
        config("planet_shapiro",std::to_string(p->planetShapiro));
        config("calc_shapiro",std::to_string(p->calcShapiro));
        config("correct_troposphere",std::to_string(p->correctTroposphere));
        config("ipm",std::to_string(p->ipm));
        config("fit_mode",std::to_string(p->fitMode));
        config("solar_wind_ne_cm3",number(p->ne_sw)); config("solar_wind_model",std::to_string(p->swm));
        config("use_calceph",std::to_string(p->useCalceph)); config("tzrsite",p->tzrsite);
        config("no_warnings","0"); config("fit_performed","false");
        for (int j=0;j<param_LAST;++j) for (int k=0;k<p->param[j].aSize;++k) {
            const parameter &v=p->param[j];
            if (v.paramSet[k] || j==param_f || j==param_track) {
                fprintf(out,"param\t%s\t%d\t%d\t%s\t%s\n",v.shortlabel[k],v.paramSet[k],v.fitFlag[k],number(v.val[k]).c_str(),v.label[k]);
            }
        }
        if (mode=="subtractBATs") {
            for (int i=0;i<p->nobs;++i) {
                if (p->obsn[i].delayCorr) throw std::runtime_error("subtractBATs requires barycentric inputs");
                p->obsn[i].bat=p->obsn[i].bbat=p->obsn[i].sat;
                p->obsn[i].delayCorr=0;
            }
            reference_stage("formResiduals"); formResiduals(p,1,0);
            snapshot(p,"direct_initial",true,true);
            for (int i=0;i<p->nobs;++i) {
                p->obsn[i].sat-=p->obsn[i].residual/SECDAYl;
                p->obsn[i].bat=p->obsn[i].bbat=p->obsn[i].sat;
            }
            formResiduals(p,1,0); snapshot(p,"aligned",true,true);
            for (int i=1;i<p->nobs;++i) {
                p->obsn[i].sat+=parse_longdouble("4e-9")/SECDAYl;
                p->obsn[i].bat=p->obsn[i].bbat=p->obsn[i].sat;
            }
            formResiduals(p,1,0); snapshot(p,"perturbed",true,true);
        } else {
            reference_stage("formBatsAll"); formBatsAll(p,1);
            snapshot(p,"bats",true,false);
            reference_stage("formResiduals"); formResiduals(p,1,0);
            snapshot(p,"residuals",true,true);
            if (mode=="offsetSATs") {
                // Diagnose the measured transformation response; do not assume unit slope.
                std::vector<longdouble> original;
                for (int i=0;i<p->nobs;++i) original.push_back(p->obsn[i].sat);
                const char *steps[]={"4e-9","0.001","-0.001"};
                const char *names[]={"offset_4ns","slope_plus","slope_minus"};
                for (int k=0;k<3;++k) {
                    for (int i=0;i<p->nobs;++i) p->obsn[i].sat=original[i]+parse_longdouble(steps[k])/SECDAYl;
                    formBatsAll(p,1); snapshot(p,names[k],true,false);
                }
            }
        }
        if (fclose(out)!=0) throw std::runtime_error("output close failed");
        destroyOne(p); delete p;
        return 0;
    } catch (const std::exception &e) {
        fprintf(stderr,"reference export failed: %s\n",e.what()); return 2;
    }
}
