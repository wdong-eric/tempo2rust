// Generated test visitor for tempo2.h::observation; frozen source field order.
use crate::{
    PRECISION_BITS,
    tempo2::{SourceClockCorrection, SourceObservation},
};
use rug::Float;
use serde_json::{Value, json};
fn long(seed: i32, id: usize, index: usize) -> Float {
    if seed == 0 {
        return Float::with_val(128, 0);
    }
    let mut v = Float::with_val(128, seed + id as i32 * 100 + index as i32);
    v += 0.125;
    v += Float::with_val(128, 1) >> 100;
    v
}
fn double(seed: i32, id: usize, index: usize) -> f64 {
    if seed == 0 {
        0.0
    } else {
        (seed + id as i32 * 100 + index as i32) as f64 + 0.125
    }
}
fn integer(seed: i32, id: usize, index: usize) -> i32 {
    if seed == 0 {
        0
    } else {
        -(seed + id as i32 * 100 + index as i32)
    }
}
fn byte(seed: i32, id: usize, index: usize) -> u8 {
    if seed == 0 || index % 32 == 5 {
        0
    } else {
        ((seed as usize + id + index) % 251) as u8
    }
}
fn correction(seed: i32, id: usize, index: usize) -> SourceClockCorrection {
    SourceClockCorrection {
        correction: Some(double(seed, id, index)),
        corrects_to: std::array::from_fn(|j| Some(byte(seed, id, index * 32 + j))),
    }
}
pub(super) fn seeded(seed: i32) -> SourceObservation {
    SourceObservation {
        sat: Some(long(seed, 1, 0)),
        origsat: Some(long(seed, 2, 0)),
        sat_day: Some(long(seed, 3, 0)),
        sat_sec: Some(long(seed, 4, 0)),
        bat: Some(long(seed, 5, 0)),
        batCorr: Some(long(seed, 6, 0)),
        bbat: Some(long(seed, 7, 0)),
        pet: Some(long(seed, 8, 0)),
        clockCorr: Some(integer(seed, 9, 0)),
        delayCorr: Some(integer(seed, 10, 0)),
        deleted: Some(integer(seed, 11, 0)),
        prefitResidual: Some(long(seed, 12, 0)),
        residual: Some(long(seed, 13, 0)),
        residualtn: Some(long(seed, 14, 0)),
        addedNoise: Some(double(seed, 15, 0)),
        TNRedSignal: Some(double(seed, 16, 0)),
        TNRedErr: Some(double(seed, 17, 0)),
        TNDMSignal: Some(double(seed, 18, 0)),
        TNDMErr: Some(double(seed, 19, 0)),
        SWGPSignal: Some(double(seed, 20, 0)),
        SWGPErr: Some(double(seed, 21, 0)),
        TNGroupSignal: Some(double(seed, 22, 0)),
        TNGroupErr: Some(double(seed, 23, 0)),
        TNChromSignal: Some(double(seed, 24, 0)),
        TNChromErr: Some(double(seed, 25, 0)),
        freq: Some(double(seed, 26, 0)),
        freqSSB: Some(double(seed, 27, 0)),
        toaErr: Some(double(seed, 28, 0)),
        toaDMErr: Some(double(seed, 29, 0)),
        origErr: Some(double(seed, 30, 0)),
        phaseOffset: Some(double(seed, 31, 0)),
        averagebat: Some(double(seed, 32, 0)),
        averageres: Some(double(seed, 33, 0)),
        averageerr: Some(double(seed, 34, 0)),
        averagedmbat: Some(double(seed, 35, 0)),
        averagedmres: Some(double(seed, 36, 0)),
        averagedmerr: Some(double(seed, 37, 0)),
        fname: std::array::from_fn(|i| Some(byte(seed, 38, i))),
        telID: std::array::from_fn(|i| Some(byte(seed, 39, i))),
        correctionsTT: std::array::from_fn(|i| correction(seed, 40, i)),
        nclock_correction: Some(integer(seed, 41, 0)),
        correctionTT_TB: Some(long(seed, 42, 0)),
        einsteinRate: Some(double(seed, 43, 0)),
        correctionTT_calcEph: Some(long(seed, 44, 0)),
        correctionTT_Teph: Some(long(seed, 45, 0)),
        correctionUT1: Some(long(seed, 46, 0)),
        sun_ssb: std::array::from_fn(|i| Some(double(seed, 47, i))),
        sun_earth: std::array::from_fn(|i| Some(double(seed, 48, i))),
        planet_ssb: std::array::from_fn(|i| {
            std::array::from_fn(|j| Some(double(seed, 49, i * 6 + j)))
        }),
        planet_ssb_tmr: std::array::from_fn(|i| {
            std::array::from_fn(|j| Some(double(seed, 50, i * 6 + j)))
        }),
        planet_ssb_derv: std::array::from_fn(|i| {
            std::array::from_fn(|j| Some(double(seed, 51, i * 6 + j)))
        }),
        jupiter_earth: std::array::from_fn(|i| Some(double(seed, 52, i))),
        saturn_earth: std::array::from_fn(|i| Some(double(seed, 53, i))),
        venus_earth: std::array::from_fn(|i| Some(double(seed, 54, i))),
        uranus_earth: std::array::from_fn(|i| Some(double(seed, 55, i))),
        neptune_earth: std::array::from_fn(|i| Some(double(seed, 56, i))),
        earthMoonBary_ssb: std::array::from_fn(|i| Some(double(seed, 57, i))),
        earthMoonBary_earth: std::array::from_fn(|i| Some(double(seed, 58, i))),
        earth_ssb: std::array::from_fn(|i| Some(double(seed, 59, i))),
        observatory_earth: std::array::from_fn(|i| Some(double(seed, 60, i))),
        psrPos: std::array::from_fn(|i| Some(double(seed, 61, i))),
        zenith: std::array::from_fn(|i| Some(double(seed, 62, i))),
        nutations: std::array::from_fn(|i| Some(double(seed, 63, i))),
        siteVel: std::array::from_fn(|i| Some(double(seed, 64, i))),
        shklovskii: Some(long(seed, 65, 0)),
        shapiroDelaySun: Some(double(seed, 66, 0)),
        shapiroDelayJupiter: Some(double(seed, 67, 0)),
        shapiroDelaySaturn: Some(double(seed, 68, 0)),
        shapiroDelayVenus: Some(double(seed, 69, 0)),
        shapiroDelayUranus: Some(double(seed, 70, 0)),
        shapiroDelayNeptune: Some(double(seed, 71, 0)),
        troposphericDelay: Some(double(seed, 72, 0)),
        tdis1: Some(double(seed, 73, 0)),
        tdis2: Some(double(seed, 74, 0)),
        roemer: Some(long(seed, 75, 0)),
        torb: Some(long(seed, 76, 0)),
        nphase: Some(long(seed, 77, 0)),
        phase: Some(long(seed, 78, 0)),
        pulseN: Some(integer(seed, 79, 0) as i64),
        flagID: std::array::from_fn(|i| std::array::from_fn(|j| Some(byte(seed, 80, i * 32 + j)))),
        flagVal: std::array::from_fn(|i| std::array::from_fn(|j| Some(byte(seed, 81, i * 32 + j)))),
        nFlags: Some(integer(seed, 82, 0)),
        jump: std::array::from_fn(|i| Some(integer(seed, 83, i))),
        jumpScale: std::array::from_fn(|i| Some(double(seed, 84, i))),
        obsNjump: Some(integer(seed, 85, 0)),
        fdjump: std::array::from_fn(|i| Some(integer(seed, 86, i))),
        obsNfdjump: Some(integer(seed, 87, 0)),
        efac: Some(double(seed, 88, 0)),
        equad: Some(double(seed, 89, 0)),
        snr: Some(double(seed, 90, 0)),
        pnoise: Some(double(seed, 91, 0)),
        tobs: Some(double(seed, 92, 0)),
        chisq: Some(double(seed, 93, 0)),
        bline: Some(double(seed, 94, 0)),
        spherical_solar_wind: Some(double(seed, 95, 0)),
    }
}
trait Cell {
    fn cell(&self) -> Value;
}
fn number(v: &Float) -> Value {
    if v.is_nan() {
        json!({"class":"NaN"})
    } else if v.is_infinite() {
        json!({"class":if v.is_sign_negative(){"-Infinity"}else{"+Infinity"}})
    } else {
        let (n, e) = v.to_integer_exp().unwrap();
        json!({"class":"finite","integer":n.to_string(),"exponent":e,"negative":v.is_sign_negative()})
    }
}
impl Cell for Float {
    fn cell(&self) -> Value {
        assert_eq!(self.prec(), PRECISION_BITS);
        number(self)
    }
}
impl Cell for f64 {
    fn cell(&self) -> Value {
        number(&Float::with_val(128, *self))
    }
}
impl Cell for i32 {
    fn cell(&self) -> Value {
        json!(self)
    }
}
impl Cell for i64 {
    fn cell(&self) -> Value {
        json!(self.to_string())
    }
}
impl Cell for u8 {
    fn cell(&self) -> Value {
        json!(self)
    }
}
impl<T: Cell> Cell for Option<T> {
    fn cell(&self) -> Value {
        self.as_ref().map(Cell::cell).unwrap_or(Value::Null)
    }
}
impl<T: Cell, const N: usize> Cell for [T; N] {
    fn cell(&self) -> Value {
        json!(self.iter().map(Cell::cell).collect::<Vec<_>>())
    }
}
impl Cell for SourceClockCorrection {
    fn cell(&self) -> Value {
        json!({"correction":self.correction.cell(),"corrects_to":self.corrects_to.cell()})
    }
}
pub(super) fn state(o: &SourceObservation) -> Value {
    let mut fields = Vec::new();
    fields.push(json!({"name":"sat","type":"longdouble","dimensions":[],"value":o.sat.cell()}));
    fields.push(
        json!({"name":"origsat","type":"longdouble","dimensions":[],"value":o.origsat.cell()}),
    );
    fields.push(
        json!({"name":"sat_day","type":"longdouble","dimensions":[],"value":o.sat_day.cell()}),
    );
    fields.push(
        json!({"name":"sat_sec","type":"longdouble","dimensions":[],"value":o.sat_sec.cell()}),
    );
    fields.push(json!({"name":"bat","type":"longdouble","dimensions":[],"value":o.bat.cell()}));
    fields.push(
        json!({"name":"batCorr","type":"longdouble","dimensions":[],"value":o.batCorr.cell()}),
    );
    fields.push(json!({"name":"bbat","type":"longdouble","dimensions":[],"value":o.bbat.cell()}));
    fields.push(json!({"name":"pet","type":"longdouble","dimensions":[],"value":o.pet.cell()}));
    fields
        .push(json!({"name":"clockCorr","type":"int","dimensions":[],"value":o.clockCorr.cell()}));
    fields
        .push(json!({"name":"delayCorr","type":"int","dimensions":[],"value":o.delayCorr.cell()}));
    fields.push(json!({"name":"deleted","type":"int","dimensions":[],"value":o.deleted.cell()}));
    fields.push(json!({"name":"prefitResidual","type":"longdouble","dimensions":[],"value":o.prefitResidual.cell()}));
    fields.push(
        json!({"name":"residual","type":"longdouble","dimensions":[],"value":o.residual.cell()}),
    );
    fields.push(json!({"name":"residualtn","type":"longdouble","dimensions":[],"value":o.residualtn.cell()}));
    fields.push(
        json!({"name":"addedNoise","type":"double","dimensions":[],"value":o.addedNoise.cell()}),
    );
    fields.push(
        json!({"name":"TNRedSignal","type":"double","dimensions":[],"value":o.TNRedSignal.cell()}),
    );
    fields
        .push(json!({"name":"TNRedErr","type":"double","dimensions":[],"value":o.TNRedErr.cell()}));
    fields.push(
        json!({"name":"TNDMSignal","type":"double","dimensions":[],"value":o.TNDMSignal.cell()}),
    );
    fields.push(json!({"name":"TNDMErr","type":"double","dimensions":[],"value":o.TNDMErr.cell()}));
    fields.push(
        json!({"name":"SWGPSignal","type":"double","dimensions":[],"value":o.SWGPSignal.cell()}),
    );
    fields.push(json!({"name":"SWGPErr","type":"double","dimensions":[],"value":o.SWGPErr.cell()}));
    fields.push(json!({"name":"TNGroupSignal","type":"double","dimensions":[],"value":o.TNGroupSignal.cell()}));
    fields.push(
        json!({"name":"TNGroupErr","type":"double","dimensions":[],"value":o.TNGroupErr.cell()}),
    );
    fields.push(json!({"name":"TNChromSignal","type":"double","dimensions":[],"value":o.TNChromSignal.cell()}));
    fields.push(
        json!({"name":"TNChromErr","type":"double","dimensions":[],"value":o.TNChromErr.cell()}),
    );
    fields.push(json!({"name":"freq","type":"double","dimensions":[],"value":o.freq.cell()}));
    fields.push(json!({"name":"freqSSB","type":"double","dimensions":[],"value":o.freqSSB.cell()}));
    fields.push(json!({"name":"toaErr","type":"double","dimensions":[],"value":o.toaErr.cell()}));
    fields
        .push(json!({"name":"toaDMErr","type":"double","dimensions":[],"value":o.toaDMErr.cell()}));
    fields.push(json!({"name":"origErr","type":"double","dimensions":[],"value":o.origErr.cell()}));
    fields.push(
        json!({"name":"phaseOffset","type":"double","dimensions":[],"value":o.phaseOffset.cell()}),
    );
    fields.push(
        json!({"name":"averagebat","type":"double","dimensions":[],"value":o.averagebat.cell()}),
    );
    fields.push(
        json!({"name":"averageres","type":"double","dimensions":[],"value":o.averageres.cell()}),
    );
    fields.push(
        json!({"name":"averageerr","type":"double","dimensions":[],"value":o.averageerr.cell()}),
    );
    fields.push(json!({"name":"averagedmbat","type":"double","dimensions":[],"value":o.averagedmbat.cell()}));
    fields.push(json!({"name":"averagedmres","type":"double","dimensions":[],"value":o.averagedmres.cell()}));
    fields.push(json!({"name":"averagedmerr","type":"double","dimensions":[],"value":o.averagedmerr.cell()}));
    fields.push(json!({"name":"fname","type":"char","dimensions":[501],"value":o.fname.cell()}));
    fields.push(json!({"name":"telID","type":"char","dimensions":[100],"value":o.telID.cell()}));
    fields.push(json!({"name":"correctionsTT","type":"clock_correction","dimensions":[30],"value":o.correctionsTT.cell()}));
    fields.push(json!({"name":"nclock_correction","type":"int","dimensions":[],"value":o.nclock_correction.cell()}));
    fields.push(json!({"name":"correctionTT_TB","type":"longdouble","dimensions":[],"value":o.correctionTT_TB.cell()}));
    fields.push(json!({"name":"einsteinRate","type":"double","dimensions":[],"value":o.einsteinRate.cell()}));
    fields.push(json!({"name":"correctionTT_calcEph","type":"longdouble","dimensions":[],"value":o.correctionTT_calcEph.cell()}));
    fields.push(json!({"name":"correctionTT_Teph","type":"longdouble","dimensions":[],"value":o.correctionTT_Teph.cell()}));
    fields.push(json!({"name":"correctionUT1","type":"longdouble","dimensions":[],"value":o.correctionUT1.cell()}));
    fields
        .push(json!({"name":"sun_ssb","type":"double","dimensions":[6],"value":o.sun_ssb.cell()}));
    fields.push(
        json!({"name":"sun_earth","type":"double","dimensions":[6],"value":o.sun_earth.cell()}),
    );
    fields.push(json!({"name":"planet_ssb","type":"double","dimensions":[9, 6],"value":o.planet_ssb.cell()}));
    fields.push(json!({"name":"planet_ssb_tmr","type":"double","dimensions":[9, 6],"value":o.planet_ssb_tmr.cell()}));
    fields.push(json!({"name":"planet_ssb_derv","type":"double","dimensions":[9, 6],"value":o.planet_ssb_derv.cell()}));
    fields.push(json!({"name":"jupiter_earth","type":"double","dimensions":[6],"value":o.jupiter_earth.cell()}));
    fields.push(json!({"name":"saturn_earth","type":"double","dimensions":[6],"value":o.saturn_earth.cell()}));
    fields.push(
        json!({"name":"venus_earth","type":"double","dimensions":[6],"value":o.venus_earth.cell()}),
    );
    fields.push(json!({"name":"uranus_earth","type":"double","dimensions":[6],"value":o.uranus_earth.cell()}));
    fields.push(json!({"name":"neptune_earth","type":"double","dimensions":[6],"value":o.neptune_earth.cell()}));
    fields.push(json!({"name":"earthMoonBary_ssb","type":"double","dimensions":[6],"value":o.earthMoonBary_ssb.cell()}));
    fields.push(json!({"name":"earthMoonBary_earth","type":"double","dimensions":[6],"value":o.earthMoonBary_earth.cell()}));
    fields.push(
        json!({"name":"earth_ssb","type":"double","dimensions":[6],"value":o.earth_ssb.cell()}),
    );
    fields.push(json!({"name":"observatory_earth","type":"double","dimensions":[6],"value":o.observatory_earth.cell()}));
    fields.push(json!({"name":"psrPos","type":"double","dimensions":[3],"value":o.psrPos.cell()}));
    fields.push(json!({"name":"zenith","type":"double","dimensions":[3],"value":o.zenith.cell()}));
    fields.push(
        json!({"name":"nutations","type":"double","dimensions":[6],"value":o.nutations.cell()}),
    );
    fields
        .push(json!({"name":"siteVel","type":"double","dimensions":[3],"value":o.siteVel.cell()}));
    fields.push(json!({"name":"shklovskii","type":"longdouble","dimensions":[],"value":o.shklovskii.cell()}));
    fields.push(json!({"name":"shapiroDelaySun","type":"double","dimensions":[],"value":o.shapiroDelaySun.cell()}));
    fields.push(json!({"name":"shapiroDelayJupiter","type":"double","dimensions":[],"value":o.shapiroDelayJupiter.cell()}));
    fields.push(json!({"name":"shapiroDelaySaturn","type":"double","dimensions":[],"value":o.shapiroDelaySaturn.cell()}));
    fields.push(json!({"name":"shapiroDelayVenus","type":"double","dimensions":[],"value":o.shapiroDelayVenus.cell()}));
    fields.push(json!({"name":"shapiroDelayUranus","type":"double","dimensions":[],"value":o.shapiroDelayUranus.cell()}));
    fields.push(json!({"name":"shapiroDelayNeptune","type":"double","dimensions":[],"value":o.shapiroDelayNeptune.cell()}));
    fields.push(json!({"name":"troposphericDelay","type":"double","dimensions":[],"value":o.troposphericDelay.cell()}));
    fields.push(json!({"name":"tdis1","type":"double","dimensions":[],"value":o.tdis1.cell()}));
    fields.push(json!({"name":"tdis2","type":"double","dimensions":[],"value":o.tdis2.cell()}));
    fields
        .push(json!({"name":"roemer","type":"longdouble","dimensions":[],"value":o.roemer.cell()}));
    fields.push(json!({"name":"torb","type":"longdouble","dimensions":[],"value":o.torb.cell()}));
    fields
        .push(json!({"name":"nphase","type":"longdouble","dimensions":[],"value":o.nphase.cell()}));
    fields.push(json!({"name":"phase","type":"longdouble","dimensions":[],"value":o.phase.cell()}));
    fields
        .push(json!({"name":"pulseN","type":"long long","dimensions":[],"value":o.pulseN.cell()}));
    fields
        .push(json!({"name":"flagID","type":"char","dimensions":[40, 32],"value":o.flagID.cell()}));
    fields.push(
        json!({"name":"flagVal","type":"char","dimensions":[40, 32],"value":o.flagVal.cell()}),
    );
    fields.push(json!({"name":"nFlags","type":"int","dimensions":[],"value":o.nFlags.cell()}));
    fields.push(json!({"name":"jump","type":"int","dimensions":[40],"value":o.jump.cell()}));
    fields.push(
        json!({"name":"jumpScale","type":"double","dimensions":[40],"value":o.jumpScale.cell()}),
    );
    fields.push(json!({"name":"obsNjump","type":"int","dimensions":[],"value":o.obsNjump.cell()}));
    fields.push(json!({"name":"fdjump","type":"int","dimensions":[40],"value":o.fdjump.cell()}));
    fields.push(
        json!({"name":"obsNfdjump","type":"int","dimensions":[],"value":o.obsNfdjump.cell()}),
    );
    fields.push(json!({"name":"efac","type":"double","dimensions":[],"value":o.efac.cell()}));
    fields.push(json!({"name":"equad","type":"double","dimensions":[],"value":o.equad.cell()}));
    fields.push(json!({"name":"snr","type":"double","dimensions":[],"value":o.snr.cell()}));
    fields.push(json!({"name":"pnoise","type":"double","dimensions":[],"value":o.pnoise.cell()}));
    fields.push(json!({"name":"tobs","type":"double","dimensions":[],"value":o.tobs.cell()}));
    fields.push(json!({"name":"chisq","type":"double","dimensions":[],"value":o.chisq.cell()}));
    fields.push(json!({"name":"bline","type":"double","dimensions":[],"value":o.bline.cell()}));
    fields.push(json!({"name":"spherical_solar_wind","type":"double","dimensions":[],"value":o.spherical_solar_wind.cell()}));
    json!(fields)
}
