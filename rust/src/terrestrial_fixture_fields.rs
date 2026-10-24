// W generated caller-only value initialization; never a production default.
fn zero_clock_correction(x: &mut SourceClockCorrection) {
x.correction=Some(0.0);
for cell0 in x.corrects_to.iter_mut() {
*cell0=Some(0);
}
}
fn retained_clock_correction(a:&SourceClockCorrection,b:&SourceClockCorrection) {
assert_eq!(format!("{:?}",a.correction),format!("{:?}",b.correction),"clock_correction.correction");
assert_eq!(format!("{:?}",a.corrects_to),format!("{:?}",b.corrects_to),"clock_correction.corrects_to");
}
fn zero_observation(x: &mut SourceObservation) {
x.sat=Some(Float::with_val(128,0));
x.origsat=Some(Float::with_val(128,0));
x.sat_day=Some(Float::with_val(128,0));
x.sat_sec=Some(Float::with_val(128,0));
x.bat=Some(Float::with_val(128,0));
x.batCorr=Some(Float::with_val(128,0));
x.bbat=Some(Float::with_val(128,0));
x.pet=Some(Float::with_val(128,0));
x.clockCorr=Some(0);
x.delayCorr=Some(0);
x.deleted=Some(0);
x.prefitResidual=Some(Float::with_val(128,0));
x.residual=Some(Float::with_val(128,0));
x.residualtn=Some(Float::with_val(128,0));
x.addedNoise=Some(0.0);
x.TNRedSignal=Some(0.0);
x.TNRedErr=Some(0.0);
x.TNDMSignal=Some(0.0);
x.TNDMErr=Some(0.0);
x.SWGPSignal=Some(0.0);
x.SWGPErr=Some(0.0);
x.TNGroupSignal=Some(0.0);
x.TNGroupErr=Some(0.0);
x.TNChromSignal=Some(0.0);
x.TNChromErr=Some(0.0);
x.freq=Some(0.0);
x.freqSSB=Some(0.0);
x.toaErr=Some(0.0);
x.toaDMErr=Some(0.0);
x.origErr=Some(0.0);
x.phaseOffset=Some(0.0);
x.averagebat=Some(0.0);
x.averageres=Some(0.0);
x.averageerr=Some(0.0);
x.averagedmbat=Some(0.0);
x.averagedmres=Some(0.0);
x.averagedmerr=Some(0.0);
for cell0 in x.fname.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.telID.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.correctionsTT.iter_mut() {
zero_clock_correction(&mut *cell0);
}
x.nclock_correction=Some(0);
x.correctionTT_TB=Some(Float::with_val(128,0));
x.einsteinRate=Some(0.0);
x.correctionTT_calcEph=Some(Float::with_val(128,0));
x.correctionTT_Teph=Some(Float::with_val(128,0));
x.correctionUT1=Some(Float::with_val(128,0));
for cell0 in x.sun_ssb.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.sun_earth.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.planet_ssb.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0.0);
}
}
for cell0 in x.planet_ssb_tmr.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0.0);
}
}
for cell0 in x.planet_ssb_derv.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0.0);
}
}
for cell0 in x.jupiter_earth.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.saturn_earth.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.venus_earth.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.uranus_earth.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.neptune_earth.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.earthMoonBary_ssb.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.earthMoonBary_earth.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.earth_ssb.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.observatory_earth.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.psrPos.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.zenith.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.nutations.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.siteVel.iter_mut() {
*cell0=Some(0.0);
}
x.shklovskii=Some(Float::with_val(128,0));
x.shapiroDelaySun=Some(0.0);
x.shapiroDelayJupiter=Some(0.0);
x.shapiroDelaySaturn=Some(0.0);
x.shapiroDelayVenus=Some(0.0);
x.shapiroDelayUranus=Some(0.0);
x.shapiroDelayNeptune=Some(0.0);
x.troposphericDelay=Some(0.0);
x.tdis1=Some(0.0);
x.tdis2=Some(0.0);
x.roemer=Some(Float::with_val(128,0));
x.torb=Some(Float::with_val(128,0));
x.nphase=Some(Float::with_val(128,0));
x.phase=Some(Float::with_val(128,0));
x.pulseN=Some(0);
for cell0 in x.flagID.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
for cell0 in x.flagVal.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
x.nFlags=Some(0);
for cell0 in x.jump.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.jumpScale.iter_mut() {
*cell0=Some(0.0);
}
x.obsNjump=Some(0);
for cell0 in x.fdjump.iter_mut() {
*cell0=Some(0);
}
x.obsNfdjump=Some(0);
x.efac=Some(0.0);
x.equad=Some(0.0);
x.snr=Some(0.0);
x.pnoise=Some(0.0);
x.tobs=Some(0.0);
x.chisq=Some(0.0);
x.bline=Some(0.0);
x.spherical_solar_wind=Some(0.0);
}
fn retained_observation(a:&SourceObservation,b:&SourceObservation) {
assert_eq!(format!("{:?}",a.sat),format!("{:?}",b.sat),"observation.sat");
assert_eq!(format!("{:?}",a.origsat),format!("{:?}",b.origsat),"observation.origsat");
assert_eq!(format!("{:?}",a.sat_day),format!("{:?}",b.sat_day),"observation.sat_day");
assert_eq!(format!("{:?}",a.sat_sec),format!("{:?}",b.sat_sec),"observation.sat_sec");
assert_eq!(format!("{:?}",a.bat),format!("{:?}",b.bat),"observation.bat");
assert_eq!(format!("{:?}",a.batCorr),format!("{:?}",b.batCorr),"observation.batCorr");
assert_eq!(format!("{:?}",a.bbat),format!("{:?}",b.bbat),"observation.bbat");
assert_eq!(format!("{:?}",a.pet),format!("{:?}",b.pet),"observation.pet");
assert_eq!(format!("{:?}",a.clockCorr),format!("{:?}",b.clockCorr),"observation.clockCorr");
assert_eq!(format!("{:?}",a.delayCorr),format!("{:?}",b.delayCorr),"observation.delayCorr");
assert_eq!(format!("{:?}",a.deleted),format!("{:?}",b.deleted),"observation.deleted");
assert_eq!(format!("{:?}",a.prefitResidual),format!("{:?}",b.prefitResidual),"observation.prefitResidual");
assert_eq!(format!("{:?}",a.residual),format!("{:?}",b.residual),"observation.residual");
assert_eq!(format!("{:?}",a.residualtn),format!("{:?}",b.residualtn),"observation.residualtn");
assert_eq!(format!("{:?}",a.addedNoise),format!("{:?}",b.addedNoise),"observation.addedNoise");
assert_eq!(format!("{:?}",a.TNRedSignal),format!("{:?}",b.TNRedSignal),"observation.TNRedSignal");
assert_eq!(format!("{:?}",a.TNRedErr),format!("{:?}",b.TNRedErr),"observation.TNRedErr");
assert_eq!(format!("{:?}",a.TNDMSignal),format!("{:?}",b.TNDMSignal),"observation.TNDMSignal");
assert_eq!(format!("{:?}",a.TNDMErr),format!("{:?}",b.TNDMErr),"observation.TNDMErr");
assert_eq!(format!("{:?}",a.SWGPSignal),format!("{:?}",b.SWGPSignal),"observation.SWGPSignal");
assert_eq!(format!("{:?}",a.SWGPErr),format!("{:?}",b.SWGPErr),"observation.SWGPErr");
assert_eq!(format!("{:?}",a.TNGroupSignal),format!("{:?}",b.TNGroupSignal),"observation.TNGroupSignal");
assert_eq!(format!("{:?}",a.TNGroupErr),format!("{:?}",b.TNGroupErr),"observation.TNGroupErr");
assert_eq!(format!("{:?}",a.TNChromSignal),format!("{:?}",b.TNChromSignal),"observation.TNChromSignal");
assert_eq!(format!("{:?}",a.TNChromErr),format!("{:?}",b.TNChromErr),"observation.TNChromErr");
assert_eq!(format!("{:?}",a.freq),format!("{:?}",b.freq),"observation.freq");
assert_eq!(format!("{:?}",a.freqSSB),format!("{:?}",b.freqSSB),"observation.freqSSB");
assert_eq!(format!("{:?}",a.toaErr),format!("{:?}",b.toaErr),"observation.toaErr");
assert_eq!(format!("{:?}",a.toaDMErr),format!("{:?}",b.toaDMErr),"observation.toaDMErr");
assert_eq!(format!("{:?}",a.origErr),format!("{:?}",b.origErr),"observation.origErr");
assert_eq!(format!("{:?}",a.phaseOffset),format!("{:?}",b.phaseOffset),"observation.phaseOffset");
assert_eq!(format!("{:?}",a.averagebat),format!("{:?}",b.averagebat),"observation.averagebat");
assert_eq!(format!("{:?}",a.averageres),format!("{:?}",b.averageres),"observation.averageres");
assert_eq!(format!("{:?}",a.averageerr),format!("{:?}",b.averageerr),"observation.averageerr");
assert_eq!(format!("{:?}",a.averagedmbat),format!("{:?}",b.averagedmbat),"observation.averagedmbat");
assert_eq!(format!("{:?}",a.averagedmres),format!("{:?}",b.averagedmres),"observation.averagedmres");
assert_eq!(format!("{:?}",a.averagedmerr),format!("{:?}",b.averagedmerr),"observation.averagedmerr");
assert_eq!(format!("{:?}",a.fname),format!("{:?}",b.fname),"observation.fname");
assert_eq!(format!("{:?}",a.telID),format!("{:?}",b.telID),"observation.telID");
assert_eq!(format!("{:?}",a.correctionsTT),format!("{:?}",b.correctionsTT),"observation.correctionsTT");
assert_eq!(format!("{:?}",a.nclock_correction),format!("{:?}",b.nclock_correction),"observation.nclock_correction");
assert_eq!(format!("{:?}",a.correctionTT_TB),format!("{:?}",b.correctionTT_TB),"observation.correctionTT_TB");
assert_eq!(format!("{:?}",a.einsteinRate),format!("{:?}",b.einsteinRate),"observation.einsteinRate");
assert_eq!(format!("{:?}",a.correctionTT_calcEph),format!("{:?}",b.correctionTT_calcEph),"observation.correctionTT_calcEph");
assert_eq!(format!("{:?}",a.correctionTT_Teph),format!("{:?}",b.correctionTT_Teph),"observation.correctionTT_Teph");
assert_eq!(format!("{:?}",a.correctionUT1),format!("{:?}",b.correctionUT1),"observation.correctionUT1");
assert_eq!(format!("{:?}",a.sun_ssb),format!("{:?}",b.sun_ssb),"observation.sun_ssb");
assert_eq!(format!("{:?}",a.sun_earth),format!("{:?}",b.sun_earth),"observation.sun_earth");
assert_eq!(format!("{:?}",a.planet_ssb),format!("{:?}",b.planet_ssb),"observation.planet_ssb");
assert_eq!(format!("{:?}",a.planet_ssb_tmr),format!("{:?}",b.planet_ssb_tmr),"observation.planet_ssb_tmr");
assert_eq!(format!("{:?}",a.planet_ssb_derv),format!("{:?}",b.planet_ssb_derv),"observation.planet_ssb_derv");
assert_eq!(format!("{:?}",a.jupiter_earth),format!("{:?}",b.jupiter_earth),"observation.jupiter_earth");
assert_eq!(format!("{:?}",a.saturn_earth),format!("{:?}",b.saturn_earth),"observation.saturn_earth");
assert_eq!(format!("{:?}",a.venus_earth),format!("{:?}",b.venus_earth),"observation.venus_earth");
assert_eq!(format!("{:?}",a.uranus_earth),format!("{:?}",b.uranus_earth),"observation.uranus_earth");
assert_eq!(format!("{:?}",a.neptune_earth),format!("{:?}",b.neptune_earth),"observation.neptune_earth");
assert_eq!(format!("{:?}",a.earthMoonBary_ssb),format!("{:?}",b.earthMoonBary_ssb),"observation.earthMoonBary_ssb");
assert_eq!(format!("{:?}",a.earthMoonBary_earth),format!("{:?}",b.earthMoonBary_earth),"observation.earthMoonBary_earth");
assert_eq!(format!("{:?}",a.earth_ssb),format!("{:?}",b.earth_ssb),"observation.earth_ssb");
assert_eq!(format!("{:?}",a.observatory_earth),format!("{:?}",b.observatory_earth),"observation.observatory_earth");
assert_eq!(format!("{:?}",a.psrPos),format!("{:?}",b.psrPos),"observation.psrPos");
assert_eq!(format!("{:?}",a.zenith),format!("{:?}",b.zenith),"observation.zenith");
assert_eq!(format!("{:?}",a.nutations),format!("{:?}",b.nutations),"observation.nutations");
assert_eq!(format!("{:?}",a.siteVel),format!("{:?}",b.siteVel),"observation.siteVel");
assert_eq!(format!("{:?}",a.shklovskii),format!("{:?}",b.shklovskii),"observation.shklovskii");
assert_eq!(format!("{:?}",a.shapiroDelaySun),format!("{:?}",b.shapiroDelaySun),"observation.shapiroDelaySun");
assert_eq!(format!("{:?}",a.shapiroDelayJupiter),format!("{:?}",b.shapiroDelayJupiter),"observation.shapiroDelayJupiter");
assert_eq!(format!("{:?}",a.shapiroDelaySaturn),format!("{:?}",b.shapiroDelaySaturn),"observation.shapiroDelaySaturn");
assert_eq!(format!("{:?}",a.shapiroDelayVenus),format!("{:?}",b.shapiroDelayVenus),"observation.shapiroDelayVenus");
assert_eq!(format!("{:?}",a.shapiroDelayUranus),format!("{:?}",b.shapiroDelayUranus),"observation.shapiroDelayUranus");
assert_eq!(format!("{:?}",a.shapiroDelayNeptune),format!("{:?}",b.shapiroDelayNeptune),"observation.shapiroDelayNeptune");
assert_eq!(format!("{:?}",a.tdis1),format!("{:?}",b.tdis1),"observation.tdis1");
assert_eq!(format!("{:?}",a.tdis2),format!("{:?}",b.tdis2),"observation.tdis2");
assert_eq!(format!("{:?}",a.roemer),format!("{:?}",b.roemer),"observation.roemer");
assert_eq!(format!("{:?}",a.torb),format!("{:?}",b.torb),"observation.torb");
assert_eq!(format!("{:?}",a.nphase),format!("{:?}",b.nphase),"observation.nphase");
assert_eq!(format!("{:?}",a.phase),format!("{:?}",b.phase),"observation.phase");
assert_eq!(format!("{:?}",a.pulseN),format!("{:?}",b.pulseN),"observation.pulseN");
assert_eq!(format!("{:?}",a.flagID),format!("{:?}",b.flagID),"observation.flagID");
assert_eq!(format!("{:?}",a.flagVal),format!("{:?}",b.flagVal),"observation.flagVal");
assert_eq!(format!("{:?}",a.nFlags),format!("{:?}",b.nFlags),"observation.nFlags");
assert_eq!(format!("{:?}",a.jump),format!("{:?}",b.jump),"observation.jump");
assert_eq!(format!("{:?}",a.jumpScale),format!("{:?}",b.jumpScale),"observation.jumpScale");
assert_eq!(format!("{:?}",a.obsNjump),format!("{:?}",b.obsNjump),"observation.obsNjump");
assert_eq!(format!("{:?}",a.fdjump),format!("{:?}",b.fdjump),"observation.fdjump");
assert_eq!(format!("{:?}",a.obsNfdjump),format!("{:?}",b.obsNfdjump),"observation.obsNfdjump");
assert_eq!(format!("{:?}",a.efac),format!("{:?}",b.efac),"observation.efac");
assert_eq!(format!("{:?}",a.equad),format!("{:?}",b.equad),"observation.equad");
assert_eq!(format!("{:?}",a.snr),format!("{:?}",b.snr),"observation.snr");
assert_eq!(format!("{:?}",a.pnoise),format!("{:?}",b.pnoise),"observation.pnoise");
assert_eq!(format!("{:?}",a.tobs),format!("{:?}",b.tobs),"observation.tobs");
assert_eq!(format!("{:?}",a.chisq),format!("{:?}",b.chisq),"observation.chisq");
assert_eq!(format!("{:?}",a.bline),format!("{:?}",b.bline),"observation.bline");
assert_eq!(format!("{:?}",a.spherical_solar_wind),format!("{:?}",b.spherical_solar_wind),"observation.spherical_solar_wind");
}
fn zero_storePrecision(x: &mut StorePrecision) {
x.minPrec=Some(Float::with_val(128,0));
for cell0 in x.routine.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.comment.iter_mut() {
*cell0=Some(0);
}
}
fn retained_storePrecision(a:&StorePrecision,b:&StorePrecision) {
assert_eq!(format!("{:?}",a.minPrec),format!("{:?}",b.minPrec),"storePrecision.minPrec");
assert_eq!(format!("{:?}",a.routine),format!("{:?}",b.routine),"storePrecision.routine");
assert_eq!(format!("{:?}",a.comment),format!("{:?}",b.comment),"storePrecision.comment");
}
fn zero_FitOutput(x: &mut FitOutput) {
for cell0 in x.parameterEstimates.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.errorEstimates.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.indexPsr.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.indexParam.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.indexCounter.iter_mut() {
*cell0=Some(0);
}
x.totalNfit=Some(0);
}
fn retained_FitOutput(a:&FitOutput,b:&FitOutput) {
assert_eq!(format!("{:?}",a.parameterEstimates),format!("{:?}",b.parameterEstimates),"FitOutput.parameterEstimates");
assert_eq!(format!("{:?}",a.errorEstimates),format!("{:?}",b.errorEstimates),"FitOutput.errorEstimates");
assert_eq!(format!("{:?}",a.indexPsr),format!("{:?}",b.indexPsr),"FitOutput.indexPsr");
assert_eq!(format!("{:?}",a.indexParam),format!("{:?}",b.indexParam),"FitOutput.indexParam");
assert_eq!(format!("{:?}",a.indexCounter),format!("{:?}",b.indexCounter),"FitOutput.indexCounter");
assert_eq!(format!("{:?}",a.totalNfit),format!("{:?}",b.totalNfit),"FitOutput.totalNfit");
}
fn zero_FitInfo(x: &mut FitInfo) {
x.nParams=Some(0);
x.nConstraints=Some(0);
for cell0 in x.paramIndex.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.constraintIndex.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.paramCounters.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.constraintCounters.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.paramDerivs.iter_mut() {
*cell0=Some(None);
}
for cell0 in x.constraintDerivs.iter_mut() {
*cell0=Some(None);
}
for cell0 in x.constraintSpecial.iter_mut() {
*cell0=Some(None);
}
for cell0 in x.constraintValue.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.updateFunctions.iter_mut() {
*cell0=Some(None);
}
zero_FitOutput(&mut x.output);
}
fn retained_FitInfo(a:&FitInfo,b:&FitInfo) {
assert_eq!(format!("{:?}",a.nParams),format!("{:?}",b.nParams),"FitInfo.nParams");
assert_eq!(format!("{:?}",a.nConstraints),format!("{:?}",b.nConstraints),"FitInfo.nConstraints");
assert_eq!(format!("{:?}",a.paramIndex),format!("{:?}",b.paramIndex),"FitInfo.paramIndex");
assert_eq!(format!("{:?}",a.constraintIndex),format!("{:?}",b.constraintIndex),"FitInfo.constraintIndex");
assert_eq!(format!("{:?}",a.paramCounters),format!("{:?}",b.paramCounters),"FitInfo.paramCounters");
assert_eq!(format!("{:?}",a.constraintCounters),format!("{:?}",b.constraintCounters),"FitInfo.constraintCounters");
assert_eq!(format!("{:?}",a.paramDerivs),format!("{:?}",b.paramDerivs),"FitInfo.paramDerivs");
assert_eq!(format!("{:?}",a.constraintDerivs),format!("{:?}",b.constraintDerivs),"FitInfo.constraintDerivs");
assert_eq!(format!("{:?}",a.constraintSpecial),format!("{:?}",b.constraintSpecial),"FitInfo.constraintSpecial");
assert_eq!(format!("{:?}",a.constraintValue),format!("{:?}",b.constraintValue),"FitInfo.constraintValue");
assert_eq!(format!("{:?}",a.updateFunctions),format!("{:?}",b.updateFunctions),"FitInfo.updateFunctions");
assert_eq!(format!("{:?}",a.output),format!("{:?}",b.output),"FitInfo.output");
}
fn zero_pulsar(x: &mut SourcePulsar) {
for cell0 in x.name.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.eopc04_file.iter_mut() {
*cell0=Some(0);
}
x.fixedFormat=Some(0);
for s in &mut x.param.slots {s.a_size=Some(0);s.link_from.fill(Some(0));s.link_to.fill(Some(0));s.n_link_to=Some(0);s.n_link_from=Some(0);}
for cell0 in x.rajStrPre.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.decjStrPre.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.rajStrPost.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.decjStrPost.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.binaryModel.iter_mut() {
*cell0=Some(0);
}
x.ToAextraCovar=Some(None);
x.dmoffsDMnum=Some(0);
x.dmoffsCMnum=Some(0);
for cell0 in x.dmoffsDM_mjd.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.dmoffsDM.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.dmoffsDM_error.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.dmoffsDM_weight.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.dmoffsCM_mjd.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.dmoffsCM.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.dmoffsCM_error.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.dmoffsCM_weight.iter_mut() {
*cell0=Some(0.0);
}
x.gwsrc_ra=Some(0.0);
x.gwsrc_dec=Some(0.0);
x.gwsrc_aplus_r=Some(0.0);
x.gwsrc_aplus_i=Some(0.0);
x.gwsrc_across_r=Some(0.0);
x.gwsrc_across_i=Some(0.0);
x.gwsrc_aplus_r_e=Some(0.0);
x.gwsrc_aplus_i_e=Some(0.0);
x.gwsrc_across_r_e=Some(0.0);
x.gwsrc_across_i_e=Some(0.0);
x.gwsrc_epoch=Some(0.0);
x.gwsrc_psrdist=Some(0.0);
x.cgw_h0=Some(0.0);
x.cgw_cosinc=Some(0.0);
x.cgw_angpol=Some(0.0);
x.cgw_mc=Some(0.0);
x.gwm_raj=Some(0.0);
x.gwm_decj=Some(0.0);
x.gwm_epoch=Some(0.0);
x.gwm_phi=Some(0.0);
x.gwm_dphase=Some(0.0);
x.gwcs_raj=Some(0.0);
x.gwcs_decj=Some(0.0);
x.gwcs_epoch=Some(0.0);
x.gwcs_width=Some(0.0);
x.gwcs_geom_p=Some(0.0);
x.gwcs_geom_c=Some(0.0);
x.gwb_epoch=Some(0.0);
x.gwb_width=Some(0.0);
x.gwb_raj=Some(0.0);
x.gwb_decj=Some(0.0);
x.gwb_geom_c=Some(0.0);
x.gwb_geom_p=Some(0.0);
x.gwecc_ra=Some(0.0);
x.gwecc_dec=Some(0.0);
x.gwecc_m1=Some(0.0);
x.gwecc_m2=Some(0.0);
x.gwecc_e=Some(0.0);
x.gwecc_inc=Some(0.0);
x.gwecc_theta_nodes=Some(0.0);
x.gwecc_nodes_orientation=Some(0.0);
x.gwecc_theta_0=Some(0.0);
x.gwecc_orbital_period=Some(0.0);
x.gwecc_distance=Some(0.0);
x.gwecc_redshift=Some(0.0);
x.gwecc_epoch=Some(0.0);
x.gwecc_psrdist=Some(0.0);
x.gwecc_pulsarTermOn=Some(0);
for cell0 in x.posPulsar.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.velPulsar.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.accPulsar.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.phaseJump.iter_mut() {
*cell0=Some(Float::with_val(128,0));
}
for cell0 in x.phaseJumpDir.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.phaseJumpID.iter_mut() {
*cell0=Some(0);
}
x.nPhaseJump=Some(0);
x.dmOffset=Some(0.0);
x.ne_sw=Some(0.0);
x.nCompanion=Some(0);
x.eclCoord=Some(0);
x.nJumps=Some(0);
for cell0 in x.fjumpID.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.jumpVal.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.jumpSAT.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.fitJump.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.jumpValErr.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.jumpStr.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
for cell0 in x.jumpScaled.iter_mut() {
*cell0=Some(0);
}
x.nfdJumps=Some(0);
x.fdjump_pint_format=Some(0);
for cell0 in x.fdjumpVal.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.fdjumpIdx.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.fitfdJump.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.fdjumpValErr.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.fdjumpStr.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
x.fdjump_log=Some(0);
for cell0 in x.filterStr.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.passStr.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.tOffset.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.tOffset_f1.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.tOffset_f2.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.tOffset_t1.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.tOffset_t2.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.tOffsetSite.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
for cell0 in x.tOffsetFlags.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
x.nToffset=Some(0);
x.ndmx=Some(0);
x.nSx=Some(0);
x.nchromx=Some(0);
x.fitChisq=Some(0.0);
x.fitNfree=Some(0);
x.globalNfit=Some(0);
x.globalNoConstrain=Some(0);
x.nFit=Some(0);
x.nParam=Some(0);
x.nGlobal=Some(0);
x.fitMode=Some(0);
x.robust=Some(0);
x.rescaleErrChisq=Some(0);
x.offset=Some(0.0);
x.offset_e=Some(0.0);
x.covar=Some(None);
x.calcShapiro=Some(0);
x.planetShapiro=Some(0);
x.jboFormat=Some(0);
x.obsn=Some(None);
x.nobs=Some(0);
x.units=Some(0);
x.setUnits=Some(0);
x.tempo1=Some(0);
x.dilateFreq=Some(0);
x.timeEphemeris=Some(0);
x.t2cMethod=Some(0);
x.correctTroposphere=Some(0);
x.noWarnings=Some(0);
x.sorted=Some(0);
for cell0 in x.clock.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.clockFromOverride.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.JPL_EPHEMERIS.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.ephemeris.iter_mut() {
*cell0=Some(0);
}
x.useCalceph=Some(0);
for cell0 in x.storePrec.iter_mut() {
zero_storePrecision(&mut *cell0);
}
x.nStorePrecision=Some(0);
x.bootStrap=Some(0);
for cell0 in x.tzrsite.iter_mut() {
*cell0=Some(0);
}
x.rmsPre=Some(0.0);
x.rmsPost=Some(0.0);
x.rmstn=Some(0.0);
for cell0 in x.deleteFileName.iter_mut() {
*cell0=Some(0);
}
x.nits=Some(0);
x.ipm=Some(0);
x.swm=Some(0);
for cell0 in x.wave_sine.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.wave_sine_err.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.wave_cos.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.wave_cos_err.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.wave_sine_dm.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.wave_sine_dm_err.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.wave_cos_dm.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.wave_cos_dm_err.iter_mut() {
*cell0=Some(0.0);
}
x.nWhite=Some(0);
x.nWhite_dm=Some(0);
x.waveScale=Some(0.0);
for cell0 in x.quad_aplus_r.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.quad_aplus_r_e.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.quad_aplus_i.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.quad_aplus_i_e.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.quad_across_r.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.quad_across_r_e.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.quad_across_i.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.quad_across_i_e.iter_mut() {
*cell0=Some(0.0);
}
x.quadEpoch=Some(0.0);
x.quadRA=Some(0.0);
x.quadDEC=Some(0.0);
x.nQuad=Some(0);
for cell0 in x.ifuncT.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.ifuncV.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.ifuncE.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.ifunc_weights.iter_mut() {
*cell0=Some(0.0);
}
x.ifuncN=Some(0);
for cell0 in x.orbifuncT.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.orbifuncV.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.orbifuncE.iter_mut() {
*cell0=Some(0.0);
}
x.orbifuncN=Some(0);
for cell0 in x.clk_offsT.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.clk_offsV.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.clk_offsE.iter_mut() {
*cell0=Some(0.0);
}
x.clkOffsN=Some(0);
for cell0 in x.quad_ifuncT_p.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.quad_ifuncV_p.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.quad_ifuncE_p.iter_mut() {
*cell0=Some(0.0);
}
x.quad_ifuncN_p=Some(0);
for cell0 in x.quad_ifuncT_c.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.quad_ifuncV_c.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.quad_ifuncE_c.iter_mut() {
*cell0=Some(0.0);
}
x.quad_ifuncN_c=Some(0);
x.quad_ifunc_p_RA=Some(0.0);
x.quad_ifunc_p_DEC=Some(0.0);
x.quad_ifunc_c_RA=Some(0.0);
x.quad_ifunc_c_DEC=Some(0.0);
x.quad_ifunc_geom_p=Some(0.0);
x.quad_ifunc_geom_c=Some(0.0);
x.nTelDX=Some(0);
x.setTelVelX=Some(0);
for cell0 in x.telDX_t.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.telDX_v.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.telDX_e.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.telDX_vel.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.telDX_vel_e.iter_mut() {
*cell0=Some(0.0);
}
x.nTelDY=Some(0);
x.setTelVelY=Some(0);
for cell0 in x.telDY_t.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.telDY_v.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.telDY_e.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.telDY_vel.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.telDY_vel_e.iter_mut() {
*cell0=Some(0.0);
}
x.nTelDZ=Some(0);
x.setTelVelZ=Some(0);
for cell0 in x.telDZ_v.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.telDZ_t.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.telDZ_e.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.telDZ_vel.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.telDZ_vel_e.iter_mut() {
*cell0=Some(0.0);
}
x.nT2efac=Some(0);
x.nT2equad=Some(0);
for cell0 in x.T2efacFlagID.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
for cell0 in x.T2efacFlagVal.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
for cell0 in x.T2efacVal.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.T2equadFlagID.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
for cell0 in x.T2equadFlagVal.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
for cell0 in x.T2equadVal.iter_mut() {
*cell0=Some(0.0);
}
x.T2globalEfac=Some(0.0);
x.nTNEF=Some(0);
x.nTNEQ=Some(0);
x.nTNSQ=Some(0);
x.nTNECORR=Some(0);
x.nTNSECORR=Some(0);
for cell0 in x.TNEFFlagID.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
for cell0 in x.TNEFFlagVal.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
for cell0 in x.TNEFVal.iter_mut() {
*cell0=Some(0.0);
}
x.TNGlobalEF=Some(0.0);
for cell0 in x.TNEQFlagID.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
for cell0 in x.TNEQFlagVal.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
for cell0 in x.TNEQVal.iter_mut() {
*cell0=Some(0.0);
}
x.TNGlobalEQ=Some(0.0);
x.addTNGlobalEQ=Some(0.0);
for cell0 in x.TNSQFlagID.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
for cell0 in x.TNSQFlagVal.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
for cell0 in x.TNSQVal.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.TNECORRFlagID.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
for cell0 in x.TNECORRFlagVal.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
for cell0 in x.TNECORRVal.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.TNSECORRFlagID.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
for cell0 in x.TNSECORRFlagVal.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
for cell0 in x.TNSECORRVal.iter_mut() {
*cell0=Some(0.0);
}
x.TNRedAmp=Some(0.0);
x.TNRedGam=Some(0.0);
x.TNRedAmp2=Some(0.0);
x.TNRedGam2=Some(0.0);
x.TNRedC=Some(0);
for cell0 in x.TNRedCoeffs.iter_mut() {
*cell0=Some(0.0);
}
x.TNRedFLow=Some(0.0);
x.TNRedCorner=Some(0.0);
x.TNDMAmp=Some(0.0);
x.TNDMGam=Some(0.0);
x.TNDMC=Some(0);
for cell0 in x.TNDMCoeffs.iter_mut() {
*cell0=Some(0.0);
}
x.SWGPAmp=Some(0.0);
x.SWGPGam=Some(0.0);
x.SWGPC=Some(0);
for cell0 in x.SWGPCoeffs.iter_mut() {
*cell0=Some(0.0);
}
x.TNChromAmp=Some(0.0);
x.TNChromGam=Some(0.0);
x.TNChromIdx=Some(0.0);
x.TNChromC=Some(0);
for cell0 in x.TNChromCoeffs.iter_mut() {
*cell0=Some(0.0);
}
x.TNsubtractDM=Some(0);
x.TNsubtractRed=Some(0);
x.TNsubtractChrom=Some(0);
x.TNsubtractSWGP=Some(0);
x.AverageResiduals=Some(0);
x.AverageDMResiduals=Some(0);
for cell0 in x.AverageFlag.iter_mut() {
*cell0=Some(0);
}
x.AverageEpochWidth=Some(0.0);
x.detL=Some(0.0);
x.outputTMatrix=Some(0);
x.useTNOrth=Some(0);
x.TNBandDMAmp=Some(0.0);
x.TNBandDMGam=Some(0.0);
x.TNBandDMC=Some(0);
x.nTNBandNoise=Some(0);
for cell0 in x.TNBandNoiseLF.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.TNBandNoiseHF.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.TNBandNoiseAmp.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.TNBandNoiseGam.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.TNBandNoiseC.iter_mut() {
*cell0=Some(0);
}
x.nTNGroupNoise=Some(0);
for cell0 in x.TNGroupNoiseFlagID.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
for cell0 in x.TNGroupNoiseFlagVal.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0);
}
}
for cell0 in x.TNGroupNoiseAmp.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.TNGroupNoiseGam.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.TNGroupNoiseC.iter_mut() {
*cell0=Some(0);
}
x.nDMEvents=Some(0);
x.nDMEventC=Some(0);
for cell0 in x.TNDMEvStart.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.TNDMEvLength.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.TNDMEvAmp.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.TNDMEvGam.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.TNDMEvOff.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.TNDMEvLin.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.TNDMEvQuad.iter_mut() {
*cell0=Some(0);
}
x.nTNShapeletEvents=Some(0);
for cell0 in x.TNShapeletEvN.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.TNShapeletEvPos.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.TNShapeletEvWidth.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.TNShapeletEvFScale.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.TNShapeletEvCoef.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0.0);
}
}
for cell0 in x.TNShapeletEvCoefErr.iter_mut() {
for cell1 in cell0.iter_mut() {
*cell1=Some(0.0);
}
}
for cell0 in x.TNShapeletEvFitFlag.iter_mut() {
*cell0=Some(0);
}
for cell0 in x.whiteNoiseModelFile.iter_mut() {
*cell0=Some(0);
}
x.rasim=Some(0.0);
x.decsim=Some(0.0);
x.simflag=Some(0);
for cell0 in x.fitFunc.iter_mut() {
*cell0=Some(0);
}
x.nconstraints=Some(0);
x.constraint_efactor=Some(0.0);
for cell0 in x.constraints.iter_mut() {
*cell0=Some(Constraint::constraint_dmmodel_mean);
}
x.auto_constraints=Some(0);
for cell0 in x.constraint_special.iter_mut() {
*cell0=Some(None);
}
zero_FitInfo(&mut x.fitinfo);
x.brace=Some(0);
zero_observation(&mut x.tzrobs);
x.refphs=Some(0);
for cell0 in x.posPulsarEquatorial.iter_mut() {
*cell0=Some(0.0);
}
x.dm_series_type=Some(SeriesType::series_undefined);
x.TN_QpPeriod=Some(0.0);
x.TN_QpSig=Some(0.0);
x.TN_QpLam=Some(0.0);
x.TN_QpRatio=Some(0.0);
x.TNRed_log_freqs=Some(0);
x.TNRed_log_factor=Some(0.0);
for cell0 in x.ne_sw_ifuncT.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.ne_sw_ifuncV.iter_mut() {
*cell0=Some(0.0);
}
for cell0 in x.ne_sw_ifuncE.iter_mut() {
*cell0=Some(0.0);
}
x.ne_sw_ifuncN=Some(0);
x.TNDM_log_freqs=Some(0);
x.TNDM_log_factor=Some(0.0);
x.TNChrom_log_freqs=Some(0);
x.TNChrom_log_factor=Some(0.0);
x.TNsubtractPoly=Some(0);
}
fn retained_pulsar(a:&SourcePulsar,b:&SourcePulsar) {
assert_eq!(format!("{:?}",a.name),format!("{:?}",b.name),"pulsar.name");
assert_eq!(format!("{:?}",a.eopc04_file),format!("{:?}",b.eopc04_file),"pulsar.eopc04_file");
assert_eq!(format!("{:?}",a.fixedFormat),format!("{:?}",b.fixedFormat),"pulsar.fixedFormat");
assert_eq!(format!("{:?}",a.param),format!("{:?}",b.param),"pulsar.param");
assert_eq!(format!("{:?}",a.rajStrPre),format!("{:?}",b.rajStrPre),"pulsar.rajStrPre");
assert_eq!(format!("{:?}",a.decjStrPre),format!("{:?}",b.decjStrPre),"pulsar.decjStrPre");
assert_eq!(format!("{:?}",a.rajStrPost),format!("{:?}",b.rajStrPost),"pulsar.rajStrPost");
assert_eq!(format!("{:?}",a.decjStrPost),format!("{:?}",b.decjStrPost),"pulsar.decjStrPost");
assert_eq!(format!("{:?}",a.binaryModel),format!("{:?}",b.binaryModel),"pulsar.binaryModel");
assert_eq!(format!("{:?}",a.ToAextraCovar),format!("{:?}",b.ToAextraCovar),"pulsar.ToAextraCovar");
assert_eq!(format!("{:?}",a.dmoffsDMnum),format!("{:?}",b.dmoffsDMnum),"pulsar.dmoffsDMnum");
assert_eq!(format!("{:?}",a.dmoffsCMnum),format!("{:?}",b.dmoffsCMnum),"pulsar.dmoffsCMnum");
assert_eq!(format!("{:?}",a.dmoffsDM_mjd),format!("{:?}",b.dmoffsDM_mjd),"pulsar.dmoffsDM_mjd");
assert_eq!(format!("{:?}",a.dmoffsDM),format!("{:?}",b.dmoffsDM),"pulsar.dmoffsDM");
assert_eq!(format!("{:?}",a.dmoffsDM_error),format!("{:?}",b.dmoffsDM_error),"pulsar.dmoffsDM_error");
assert_eq!(format!("{:?}",a.dmoffsDM_weight),format!("{:?}",b.dmoffsDM_weight),"pulsar.dmoffsDM_weight");
assert_eq!(format!("{:?}",a.dmoffsCM_mjd),format!("{:?}",b.dmoffsCM_mjd),"pulsar.dmoffsCM_mjd");
assert_eq!(format!("{:?}",a.dmoffsCM),format!("{:?}",b.dmoffsCM),"pulsar.dmoffsCM");
assert_eq!(format!("{:?}",a.dmoffsCM_error),format!("{:?}",b.dmoffsCM_error),"pulsar.dmoffsCM_error");
assert_eq!(format!("{:?}",a.dmoffsCM_weight),format!("{:?}",b.dmoffsCM_weight),"pulsar.dmoffsCM_weight");
assert_eq!(format!("{:?}",a.gwsrc_ra),format!("{:?}",b.gwsrc_ra),"pulsar.gwsrc_ra");
assert_eq!(format!("{:?}",a.gwsrc_dec),format!("{:?}",b.gwsrc_dec),"pulsar.gwsrc_dec");
assert_eq!(format!("{:?}",a.gwsrc_aplus_r),format!("{:?}",b.gwsrc_aplus_r),"pulsar.gwsrc_aplus_r");
assert_eq!(format!("{:?}",a.gwsrc_aplus_i),format!("{:?}",b.gwsrc_aplus_i),"pulsar.gwsrc_aplus_i");
assert_eq!(format!("{:?}",a.gwsrc_across_r),format!("{:?}",b.gwsrc_across_r),"pulsar.gwsrc_across_r");
assert_eq!(format!("{:?}",a.gwsrc_across_i),format!("{:?}",b.gwsrc_across_i),"pulsar.gwsrc_across_i");
assert_eq!(format!("{:?}",a.gwsrc_aplus_r_e),format!("{:?}",b.gwsrc_aplus_r_e),"pulsar.gwsrc_aplus_r_e");
assert_eq!(format!("{:?}",a.gwsrc_aplus_i_e),format!("{:?}",b.gwsrc_aplus_i_e),"pulsar.gwsrc_aplus_i_e");
assert_eq!(format!("{:?}",a.gwsrc_across_r_e),format!("{:?}",b.gwsrc_across_r_e),"pulsar.gwsrc_across_r_e");
assert_eq!(format!("{:?}",a.gwsrc_across_i_e),format!("{:?}",b.gwsrc_across_i_e),"pulsar.gwsrc_across_i_e");
assert_eq!(format!("{:?}",a.gwsrc_epoch),format!("{:?}",b.gwsrc_epoch),"pulsar.gwsrc_epoch");
assert_eq!(format!("{:?}",a.gwsrc_psrdist),format!("{:?}",b.gwsrc_psrdist),"pulsar.gwsrc_psrdist");
assert_eq!(format!("{:?}",a.cgw_h0),format!("{:?}",b.cgw_h0),"pulsar.cgw_h0");
assert_eq!(format!("{:?}",a.cgw_cosinc),format!("{:?}",b.cgw_cosinc),"pulsar.cgw_cosinc");
assert_eq!(format!("{:?}",a.cgw_angpol),format!("{:?}",b.cgw_angpol),"pulsar.cgw_angpol");
assert_eq!(format!("{:?}",a.cgw_mc),format!("{:?}",b.cgw_mc),"pulsar.cgw_mc");
assert_eq!(format!("{:?}",a.gwm_raj),format!("{:?}",b.gwm_raj),"pulsar.gwm_raj");
assert_eq!(format!("{:?}",a.gwm_decj),format!("{:?}",b.gwm_decj),"pulsar.gwm_decj");
assert_eq!(format!("{:?}",a.gwm_epoch),format!("{:?}",b.gwm_epoch),"pulsar.gwm_epoch");
assert_eq!(format!("{:?}",a.gwm_phi),format!("{:?}",b.gwm_phi),"pulsar.gwm_phi");
assert_eq!(format!("{:?}",a.gwm_dphase),format!("{:?}",b.gwm_dphase),"pulsar.gwm_dphase");
assert_eq!(format!("{:?}",a.gwcs_raj),format!("{:?}",b.gwcs_raj),"pulsar.gwcs_raj");
assert_eq!(format!("{:?}",a.gwcs_decj),format!("{:?}",b.gwcs_decj),"pulsar.gwcs_decj");
assert_eq!(format!("{:?}",a.gwcs_epoch),format!("{:?}",b.gwcs_epoch),"pulsar.gwcs_epoch");
assert_eq!(format!("{:?}",a.gwcs_width),format!("{:?}",b.gwcs_width),"pulsar.gwcs_width");
assert_eq!(format!("{:?}",a.gwcs_geom_p),format!("{:?}",b.gwcs_geom_p),"pulsar.gwcs_geom_p");
assert_eq!(format!("{:?}",a.gwcs_geom_c),format!("{:?}",b.gwcs_geom_c),"pulsar.gwcs_geom_c");
assert_eq!(format!("{:?}",a.gwb_epoch),format!("{:?}",b.gwb_epoch),"pulsar.gwb_epoch");
assert_eq!(format!("{:?}",a.gwb_width),format!("{:?}",b.gwb_width),"pulsar.gwb_width");
assert_eq!(format!("{:?}",a.gwb_raj),format!("{:?}",b.gwb_raj),"pulsar.gwb_raj");
assert_eq!(format!("{:?}",a.gwb_decj),format!("{:?}",b.gwb_decj),"pulsar.gwb_decj");
assert_eq!(format!("{:?}",a.gwb_geom_c),format!("{:?}",b.gwb_geom_c),"pulsar.gwb_geom_c");
assert_eq!(format!("{:?}",a.gwb_geom_p),format!("{:?}",b.gwb_geom_p),"pulsar.gwb_geom_p");
assert_eq!(format!("{:?}",a.gwecc_ra),format!("{:?}",b.gwecc_ra),"pulsar.gwecc_ra");
assert_eq!(format!("{:?}",a.gwecc_dec),format!("{:?}",b.gwecc_dec),"pulsar.gwecc_dec");
assert_eq!(format!("{:?}",a.gwecc_m1),format!("{:?}",b.gwecc_m1),"pulsar.gwecc_m1");
assert_eq!(format!("{:?}",a.gwecc_m2),format!("{:?}",b.gwecc_m2),"pulsar.gwecc_m2");
assert_eq!(format!("{:?}",a.gwecc_e),format!("{:?}",b.gwecc_e),"pulsar.gwecc_e");
assert_eq!(format!("{:?}",a.gwecc_inc),format!("{:?}",b.gwecc_inc),"pulsar.gwecc_inc");
assert_eq!(format!("{:?}",a.gwecc_theta_nodes),format!("{:?}",b.gwecc_theta_nodes),"pulsar.gwecc_theta_nodes");
assert_eq!(format!("{:?}",a.gwecc_nodes_orientation),format!("{:?}",b.gwecc_nodes_orientation),"pulsar.gwecc_nodes_orientation");
assert_eq!(format!("{:?}",a.gwecc_theta_0),format!("{:?}",b.gwecc_theta_0),"pulsar.gwecc_theta_0");
assert_eq!(format!("{:?}",a.gwecc_orbital_period),format!("{:?}",b.gwecc_orbital_period),"pulsar.gwecc_orbital_period");
assert_eq!(format!("{:?}",a.gwecc_distance),format!("{:?}",b.gwecc_distance),"pulsar.gwecc_distance");
assert_eq!(format!("{:?}",a.gwecc_redshift),format!("{:?}",b.gwecc_redshift),"pulsar.gwecc_redshift");
assert_eq!(format!("{:?}",a.gwecc_epoch),format!("{:?}",b.gwecc_epoch),"pulsar.gwecc_epoch");
assert_eq!(format!("{:?}",a.gwecc_psrdist),format!("{:?}",b.gwecc_psrdist),"pulsar.gwecc_psrdist");
assert_eq!(format!("{:?}",a.gwecc_pulsarTermOn),format!("{:?}",b.gwecc_pulsarTermOn),"pulsar.gwecc_pulsarTermOn");
assert_eq!(format!("{:?}",a.posPulsar),format!("{:?}",b.posPulsar),"pulsar.posPulsar");
assert_eq!(format!("{:?}",a.velPulsar),format!("{:?}",b.velPulsar),"pulsar.velPulsar");
assert_eq!(format!("{:?}",a.accPulsar),format!("{:?}",b.accPulsar),"pulsar.accPulsar");
assert_eq!(format!("{:?}",a.phaseJump),format!("{:?}",b.phaseJump),"pulsar.phaseJump");
assert_eq!(format!("{:?}",a.phaseJumpDir),format!("{:?}",b.phaseJumpDir),"pulsar.phaseJumpDir");
assert_eq!(format!("{:?}",a.phaseJumpID),format!("{:?}",b.phaseJumpID),"pulsar.phaseJumpID");
assert_eq!(format!("{:?}",a.nPhaseJump),format!("{:?}",b.nPhaseJump),"pulsar.nPhaseJump");
assert_eq!(format!("{:?}",a.dmOffset),format!("{:?}",b.dmOffset),"pulsar.dmOffset");
assert_eq!(format!("{:?}",a.ne_sw),format!("{:?}",b.ne_sw),"pulsar.ne_sw");
assert_eq!(format!("{:?}",a.nCompanion),format!("{:?}",b.nCompanion),"pulsar.nCompanion");
assert_eq!(format!("{:?}",a.eclCoord),format!("{:?}",b.eclCoord),"pulsar.eclCoord");
assert_eq!(format!("{:?}",a.nJumps),format!("{:?}",b.nJumps),"pulsar.nJumps");
assert_eq!(format!("{:?}",a.fjumpID),format!("{:?}",b.fjumpID),"pulsar.fjumpID");
assert_eq!(format!("{:?}",a.jumpVal),format!("{:?}",b.jumpVal),"pulsar.jumpVal");
assert_eq!(format!("{:?}",a.jumpSAT),format!("{:?}",b.jumpSAT),"pulsar.jumpSAT");
assert_eq!(format!("{:?}",a.fitJump),format!("{:?}",b.fitJump),"pulsar.fitJump");
assert_eq!(format!("{:?}",a.jumpValErr),format!("{:?}",b.jumpValErr),"pulsar.jumpValErr");
assert_eq!(format!("{:?}",a.jumpStr),format!("{:?}",b.jumpStr),"pulsar.jumpStr");
assert_eq!(format!("{:?}",a.jumpScaled),format!("{:?}",b.jumpScaled),"pulsar.jumpScaled");
assert_eq!(format!("{:?}",a.nfdJumps),format!("{:?}",b.nfdJumps),"pulsar.nfdJumps");
assert_eq!(format!("{:?}",a.fdjump_pint_format),format!("{:?}",b.fdjump_pint_format),"pulsar.fdjump_pint_format");
assert_eq!(format!("{:?}",a.fdjumpVal),format!("{:?}",b.fdjumpVal),"pulsar.fdjumpVal");
assert_eq!(format!("{:?}",a.fdjumpIdx),format!("{:?}",b.fdjumpIdx),"pulsar.fdjumpIdx");
assert_eq!(format!("{:?}",a.fitfdJump),format!("{:?}",b.fitfdJump),"pulsar.fitfdJump");
assert_eq!(format!("{:?}",a.fdjumpValErr),format!("{:?}",b.fdjumpValErr),"pulsar.fdjumpValErr");
assert_eq!(format!("{:?}",a.fdjumpStr),format!("{:?}",b.fdjumpStr),"pulsar.fdjumpStr");
assert_eq!(format!("{:?}",a.fdjump_log),format!("{:?}",b.fdjump_log),"pulsar.fdjump_log");
assert_eq!(format!("{:?}",a.filterStr),format!("{:?}",b.filterStr),"pulsar.filterStr");
assert_eq!(format!("{:?}",a.passStr),format!("{:?}",b.passStr),"pulsar.passStr");
assert_eq!(format!("{:?}",a.tOffset),format!("{:?}",b.tOffset),"pulsar.tOffset");
assert_eq!(format!("{:?}",a.tOffset_f1),format!("{:?}",b.tOffset_f1),"pulsar.tOffset_f1");
assert_eq!(format!("{:?}",a.tOffset_f2),format!("{:?}",b.tOffset_f2),"pulsar.tOffset_f2");
assert_eq!(format!("{:?}",a.tOffset_t1),format!("{:?}",b.tOffset_t1),"pulsar.tOffset_t1");
assert_eq!(format!("{:?}",a.tOffset_t2),format!("{:?}",b.tOffset_t2),"pulsar.tOffset_t2");
assert_eq!(format!("{:?}",a.tOffsetSite),format!("{:?}",b.tOffsetSite),"pulsar.tOffsetSite");
assert_eq!(format!("{:?}",a.tOffsetFlags),format!("{:?}",b.tOffsetFlags),"pulsar.tOffsetFlags");
assert_eq!(format!("{:?}",a.nToffset),format!("{:?}",b.nToffset),"pulsar.nToffset");
assert_eq!(format!("{:?}",a.ndmx),format!("{:?}",b.ndmx),"pulsar.ndmx");
assert_eq!(format!("{:?}",a.nSx),format!("{:?}",b.nSx),"pulsar.nSx");
assert_eq!(format!("{:?}",a.nchromx),format!("{:?}",b.nchromx),"pulsar.nchromx");
assert_eq!(format!("{:?}",a.fitChisq),format!("{:?}",b.fitChisq),"pulsar.fitChisq");
assert_eq!(format!("{:?}",a.fitNfree),format!("{:?}",b.fitNfree),"pulsar.fitNfree");
assert_eq!(format!("{:?}",a.globalNfit),format!("{:?}",b.globalNfit),"pulsar.globalNfit");
assert_eq!(format!("{:?}",a.globalNoConstrain),format!("{:?}",b.globalNoConstrain),"pulsar.globalNoConstrain");
assert_eq!(format!("{:?}",a.nFit),format!("{:?}",b.nFit),"pulsar.nFit");
assert_eq!(format!("{:?}",a.nParam),format!("{:?}",b.nParam),"pulsar.nParam");
assert_eq!(format!("{:?}",a.nGlobal),format!("{:?}",b.nGlobal),"pulsar.nGlobal");
assert_eq!(format!("{:?}",a.fitMode),format!("{:?}",b.fitMode),"pulsar.fitMode");
assert_eq!(format!("{:?}",a.robust),format!("{:?}",b.robust),"pulsar.robust");
assert_eq!(format!("{:?}",a.rescaleErrChisq),format!("{:?}",b.rescaleErrChisq),"pulsar.rescaleErrChisq");
assert_eq!(format!("{:?}",a.offset),format!("{:?}",b.offset),"pulsar.offset");
assert_eq!(format!("{:?}",a.offset_e),format!("{:?}",b.offset_e),"pulsar.offset_e");
assert_eq!(format!("{:?}",a.covar),format!("{:?}",b.covar),"pulsar.covar");
assert_eq!(format!("{:?}",a.calcShapiro),format!("{:?}",b.calcShapiro),"pulsar.calcShapiro");
assert_eq!(format!("{:?}",a.planetShapiro),format!("{:?}",b.planetShapiro),"pulsar.planetShapiro");
assert_eq!(format!("{:?}",a.jboFormat),format!("{:?}",b.jboFormat),"pulsar.jboFormat");
assert_eq!(format!("{:?}",a.obsn),format!("{:?}",b.obsn),"pulsar.obsn");
assert_eq!(format!("{:?}",a.nobs),format!("{:?}",b.nobs),"pulsar.nobs");
assert_eq!(format!("{:?}",a.units),format!("{:?}",b.units),"pulsar.units");
assert_eq!(format!("{:?}",a.setUnits),format!("{:?}",b.setUnits),"pulsar.setUnits");
assert_eq!(format!("{:?}",a.tempo1),format!("{:?}",b.tempo1),"pulsar.tempo1");
assert_eq!(format!("{:?}",a.dilateFreq),format!("{:?}",b.dilateFreq),"pulsar.dilateFreq");
assert_eq!(format!("{:?}",a.timeEphemeris),format!("{:?}",b.timeEphemeris),"pulsar.timeEphemeris");
assert_eq!(format!("{:?}",a.t2cMethod),format!("{:?}",b.t2cMethod),"pulsar.t2cMethod");
assert_eq!(format!("{:?}",a.correctTroposphere),format!("{:?}",b.correctTroposphere),"pulsar.correctTroposphere");
assert_eq!(format!("{:?}",a.noWarnings),format!("{:?}",b.noWarnings),"pulsar.noWarnings");
assert_eq!(format!("{:?}",a.sorted),format!("{:?}",b.sorted),"pulsar.sorted");
assert_eq!(format!("{:?}",a.clock),format!("{:?}",b.clock),"pulsar.clock");
assert_eq!(format!("{:?}",a.clockFromOverride),format!("{:?}",b.clockFromOverride),"pulsar.clockFromOverride");
assert_eq!(format!("{:?}",a.JPL_EPHEMERIS),format!("{:?}",b.JPL_EPHEMERIS),"pulsar.JPL_EPHEMERIS");
assert_eq!(format!("{:?}",a.ephemeris),format!("{:?}",b.ephemeris),"pulsar.ephemeris");
assert_eq!(format!("{:?}",a.useCalceph),format!("{:?}",b.useCalceph),"pulsar.useCalceph");
assert_eq!(format!("{:?}",a.storePrec),format!("{:?}",b.storePrec),"pulsar.storePrec");
assert_eq!(format!("{:?}",a.nStorePrecision),format!("{:?}",b.nStorePrecision),"pulsar.nStorePrecision");
assert_eq!(format!("{:?}",a.bootStrap),format!("{:?}",b.bootStrap),"pulsar.bootStrap");
assert_eq!(format!("{:?}",a.tzrsite),format!("{:?}",b.tzrsite),"pulsar.tzrsite");
assert_eq!(format!("{:?}",a.rmsPre),format!("{:?}",b.rmsPre),"pulsar.rmsPre");
assert_eq!(format!("{:?}",a.rmsPost),format!("{:?}",b.rmsPost),"pulsar.rmsPost");
assert_eq!(format!("{:?}",a.rmstn),format!("{:?}",b.rmstn),"pulsar.rmstn");
assert_eq!(format!("{:?}",a.deleteFileName),format!("{:?}",b.deleteFileName),"pulsar.deleteFileName");
assert_eq!(format!("{:?}",a.nits),format!("{:?}",b.nits),"pulsar.nits");
assert_eq!(format!("{:?}",a.ipm),format!("{:?}",b.ipm),"pulsar.ipm");
assert_eq!(format!("{:?}",a.swm),format!("{:?}",b.swm),"pulsar.swm");
assert_eq!(format!("{:?}",a.wave_sine),format!("{:?}",b.wave_sine),"pulsar.wave_sine");
assert_eq!(format!("{:?}",a.wave_sine_err),format!("{:?}",b.wave_sine_err),"pulsar.wave_sine_err");
assert_eq!(format!("{:?}",a.wave_cos),format!("{:?}",b.wave_cos),"pulsar.wave_cos");
assert_eq!(format!("{:?}",a.wave_cos_err),format!("{:?}",b.wave_cos_err),"pulsar.wave_cos_err");
assert_eq!(format!("{:?}",a.wave_sine_dm),format!("{:?}",b.wave_sine_dm),"pulsar.wave_sine_dm");
assert_eq!(format!("{:?}",a.wave_sine_dm_err),format!("{:?}",b.wave_sine_dm_err),"pulsar.wave_sine_dm_err");
assert_eq!(format!("{:?}",a.wave_cos_dm),format!("{:?}",b.wave_cos_dm),"pulsar.wave_cos_dm");
assert_eq!(format!("{:?}",a.wave_cos_dm_err),format!("{:?}",b.wave_cos_dm_err),"pulsar.wave_cos_dm_err");
assert_eq!(format!("{:?}",a.nWhite),format!("{:?}",b.nWhite),"pulsar.nWhite");
assert_eq!(format!("{:?}",a.nWhite_dm),format!("{:?}",b.nWhite_dm),"pulsar.nWhite_dm");
assert_eq!(format!("{:?}",a.waveScale),format!("{:?}",b.waveScale),"pulsar.waveScale");
assert_eq!(format!("{:?}",a.quad_aplus_r),format!("{:?}",b.quad_aplus_r),"pulsar.quad_aplus_r");
assert_eq!(format!("{:?}",a.quad_aplus_r_e),format!("{:?}",b.quad_aplus_r_e),"pulsar.quad_aplus_r_e");
assert_eq!(format!("{:?}",a.quad_aplus_i),format!("{:?}",b.quad_aplus_i),"pulsar.quad_aplus_i");
assert_eq!(format!("{:?}",a.quad_aplus_i_e),format!("{:?}",b.quad_aplus_i_e),"pulsar.quad_aplus_i_e");
assert_eq!(format!("{:?}",a.quad_across_r),format!("{:?}",b.quad_across_r),"pulsar.quad_across_r");
assert_eq!(format!("{:?}",a.quad_across_r_e),format!("{:?}",b.quad_across_r_e),"pulsar.quad_across_r_e");
assert_eq!(format!("{:?}",a.quad_across_i),format!("{:?}",b.quad_across_i),"pulsar.quad_across_i");
assert_eq!(format!("{:?}",a.quad_across_i_e),format!("{:?}",b.quad_across_i_e),"pulsar.quad_across_i_e");
assert_eq!(format!("{:?}",a.quadEpoch),format!("{:?}",b.quadEpoch),"pulsar.quadEpoch");
assert_eq!(format!("{:?}",a.quadRA),format!("{:?}",b.quadRA),"pulsar.quadRA");
assert_eq!(format!("{:?}",a.quadDEC),format!("{:?}",b.quadDEC),"pulsar.quadDEC");
assert_eq!(format!("{:?}",a.nQuad),format!("{:?}",b.nQuad),"pulsar.nQuad");
assert_eq!(format!("{:?}",a.ifuncT),format!("{:?}",b.ifuncT),"pulsar.ifuncT");
assert_eq!(format!("{:?}",a.ifuncV),format!("{:?}",b.ifuncV),"pulsar.ifuncV");
assert_eq!(format!("{:?}",a.ifuncE),format!("{:?}",b.ifuncE),"pulsar.ifuncE");
assert_eq!(format!("{:?}",a.ifunc_weights),format!("{:?}",b.ifunc_weights),"pulsar.ifunc_weights");
assert_eq!(format!("{:?}",a.ifuncN),format!("{:?}",b.ifuncN),"pulsar.ifuncN");
assert_eq!(format!("{:?}",a.orbifuncT),format!("{:?}",b.orbifuncT),"pulsar.orbifuncT");
assert_eq!(format!("{:?}",a.orbifuncV),format!("{:?}",b.orbifuncV),"pulsar.orbifuncV");
assert_eq!(format!("{:?}",a.orbifuncE),format!("{:?}",b.orbifuncE),"pulsar.orbifuncE");
assert_eq!(format!("{:?}",a.orbifuncN),format!("{:?}",b.orbifuncN),"pulsar.orbifuncN");
assert_eq!(format!("{:?}",a.clk_offsT),format!("{:?}",b.clk_offsT),"pulsar.clk_offsT");
assert_eq!(format!("{:?}",a.clk_offsV),format!("{:?}",b.clk_offsV),"pulsar.clk_offsV");
assert_eq!(format!("{:?}",a.clk_offsE),format!("{:?}",b.clk_offsE),"pulsar.clk_offsE");
assert_eq!(format!("{:?}",a.clkOffsN),format!("{:?}",b.clkOffsN),"pulsar.clkOffsN");
assert_eq!(format!("{:?}",a.quad_ifuncT_p),format!("{:?}",b.quad_ifuncT_p),"pulsar.quad_ifuncT_p");
assert_eq!(format!("{:?}",a.quad_ifuncV_p),format!("{:?}",b.quad_ifuncV_p),"pulsar.quad_ifuncV_p");
assert_eq!(format!("{:?}",a.quad_ifuncE_p),format!("{:?}",b.quad_ifuncE_p),"pulsar.quad_ifuncE_p");
assert_eq!(format!("{:?}",a.quad_ifuncN_p),format!("{:?}",b.quad_ifuncN_p),"pulsar.quad_ifuncN_p");
assert_eq!(format!("{:?}",a.quad_ifuncT_c),format!("{:?}",b.quad_ifuncT_c),"pulsar.quad_ifuncT_c");
assert_eq!(format!("{:?}",a.quad_ifuncV_c),format!("{:?}",b.quad_ifuncV_c),"pulsar.quad_ifuncV_c");
assert_eq!(format!("{:?}",a.quad_ifuncE_c),format!("{:?}",b.quad_ifuncE_c),"pulsar.quad_ifuncE_c");
assert_eq!(format!("{:?}",a.quad_ifuncN_c),format!("{:?}",b.quad_ifuncN_c),"pulsar.quad_ifuncN_c");
assert_eq!(format!("{:?}",a.quad_ifunc_p_RA),format!("{:?}",b.quad_ifunc_p_RA),"pulsar.quad_ifunc_p_RA");
assert_eq!(format!("{:?}",a.quad_ifunc_p_DEC),format!("{:?}",b.quad_ifunc_p_DEC),"pulsar.quad_ifunc_p_DEC");
assert_eq!(format!("{:?}",a.quad_ifunc_c_RA),format!("{:?}",b.quad_ifunc_c_RA),"pulsar.quad_ifunc_c_RA");
assert_eq!(format!("{:?}",a.quad_ifunc_c_DEC),format!("{:?}",b.quad_ifunc_c_DEC),"pulsar.quad_ifunc_c_DEC");
assert_eq!(format!("{:?}",a.quad_ifunc_geom_p),format!("{:?}",b.quad_ifunc_geom_p),"pulsar.quad_ifunc_geom_p");
assert_eq!(format!("{:?}",a.quad_ifunc_geom_c),format!("{:?}",b.quad_ifunc_geom_c),"pulsar.quad_ifunc_geom_c");
assert_eq!(format!("{:?}",a.nTelDX),format!("{:?}",b.nTelDX),"pulsar.nTelDX");
assert_eq!(format!("{:?}",a.setTelVelX),format!("{:?}",b.setTelVelX),"pulsar.setTelVelX");
assert_eq!(format!("{:?}",a.telDX_t),format!("{:?}",b.telDX_t),"pulsar.telDX_t");
assert_eq!(format!("{:?}",a.telDX_v),format!("{:?}",b.telDX_v),"pulsar.telDX_v");
assert_eq!(format!("{:?}",a.telDX_e),format!("{:?}",b.telDX_e),"pulsar.telDX_e");
assert_eq!(format!("{:?}",a.telDX_vel),format!("{:?}",b.telDX_vel),"pulsar.telDX_vel");
assert_eq!(format!("{:?}",a.telDX_vel_e),format!("{:?}",b.telDX_vel_e),"pulsar.telDX_vel_e");
assert_eq!(format!("{:?}",a.nTelDY),format!("{:?}",b.nTelDY),"pulsar.nTelDY");
assert_eq!(format!("{:?}",a.setTelVelY),format!("{:?}",b.setTelVelY),"pulsar.setTelVelY");
assert_eq!(format!("{:?}",a.telDY_t),format!("{:?}",b.telDY_t),"pulsar.telDY_t");
assert_eq!(format!("{:?}",a.telDY_v),format!("{:?}",b.telDY_v),"pulsar.telDY_v");
assert_eq!(format!("{:?}",a.telDY_e),format!("{:?}",b.telDY_e),"pulsar.telDY_e");
assert_eq!(format!("{:?}",a.telDY_vel),format!("{:?}",b.telDY_vel),"pulsar.telDY_vel");
assert_eq!(format!("{:?}",a.telDY_vel_e),format!("{:?}",b.telDY_vel_e),"pulsar.telDY_vel_e");
assert_eq!(format!("{:?}",a.nTelDZ),format!("{:?}",b.nTelDZ),"pulsar.nTelDZ");
assert_eq!(format!("{:?}",a.setTelVelZ),format!("{:?}",b.setTelVelZ),"pulsar.setTelVelZ");
assert_eq!(format!("{:?}",a.telDZ_v),format!("{:?}",b.telDZ_v),"pulsar.telDZ_v");
assert_eq!(format!("{:?}",a.telDZ_t),format!("{:?}",b.telDZ_t),"pulsar.telDZ_t");
assert_eq!(format!("{:?}",a.telDZ_e),format!("{:?}",b.telDZ_e),"pulsar.telDZ_e");
assert_eq!(format!("{:?}",a.telDZ_vel),format!("{:?}",b.telDZ_vel),"pulsar.telDZ_vel");
assert_eq!(format!("{:?}",a.telDZ_vel_e),format!("{:?}",b.telDZ_vel_e),"pulsar.telDZ_vel_e");
assert_eq!(format!("{:?}",a.nT2efac),format!("{:?}",b.nT2efac),"pulsar.nT2efac");
assert_eq!(format!("{:?}",a.nT2equad),format!("{:?}",b.nT2equad),"pulsar.nT2equad");
assert_eq!(format!("{:?}",a.T2efacFlagID),format!("{:?}",b.T2efacFlagID),"pulsar.T2efacFlagID");
assert_eq!(format!("{:?}",a.T2efacFlagVal),format!("{:?}",b.T2efacFlagVal),"pulsar.T2efacFlagVal");
assert_eq!(format!("{:?}",a.T2efacVal),format!("{:?}",b.T2efacVal),"pulsar.T2efacVal");
assert_eq!(format!("{:?}",a.T2equadFlagID),format!("{:?}",b.T2equadFlagID),"pulsar.T2equadFlagID");
assert_eq!(format!("{:?}",a.T2equadFlagVal),format!("{:?}",b.T2equadFlagVal),"pulsar.T2equadFlagVal");
assert_eq!(format!("{:?}",a.T2equadVal),format!("{:?}",b.T2equadVal),"pulsar.T2equadVal");
assert_eq!(format!("{:?}",a.T2globalEfac),format!("{:?}",b.T2globalEfac),"pulsar.T2globalEfac");
assert_eq!(format!("{:?}",a.nTNEF),format!("{:?}",b.nTNEF),"pulsar.nTNEF");
assert_eq!(format!("{:?}",a.nTNEQ),format!("{:?}",b.nTNEQ),"pulsar.nTNEQ");
assert_eq!(format!("{:?}",a.nTNSQ),format!("{:?}",b.nTNSQ),"pulsar.nTNSQ");
assert_eq!(format!("{:?}",a.nTNECORR),format!("{:?}",b.nTNECORR),"pulsar.nTNECORR");
assert_eq!(format!("{:?}",a.nTNSECORR),format!("{:?}",b.nTNSECORR),"pulsar.nTNSECORR");
assert_eq!(format!("{:?}",a.TNEFFlagID),format!("{:?}",b.TNEFFlagID),"pulsar.TNEFFlagID");
assert_eq!(format!("{:?}",a.TNEFFlagVal),format!("{:?}",b.TNEFFlagVal),"pulsar.TNEFFlagVal");
assert_eq!(format!("{:?}",a.TNEFVal),format!("{:?}",b.TNEFVal),"pulsar.TNEFVal");
assert_eq!(format!("{:?}",a.TNGlobalEF),format!("{:?}",b.TNGlobalEF),"pulsar.TNGlobalEF");
assert_eq!(format!("{:?}",a.TNEQFlagID),format!("{:?}",b.TNEQFlagID),"pulsar.TNEQFlagID");
assert_eq!(format!("{:?}",a.TNEQFlagVal),format!("{:?}",b.TNEQFlagVal),"pulsar.TNEQFlagVal");
assert_eq!(format!("{:?}",a.TNEQVal),format!("{:?}",b.TNEQVal),"pulsar.TNEQVal");
assert_eq!(format!("{:?}",a.TNGlobalEQ),format!("{:?}",b.TNGlobalEQ),"pulsar.TNGlobalEQ");
assert_eq!(format!("{:?}",a.addTNGlobalEQ),format!("{:?}",b.addTNGlobalEQ),"pulsar.addTNGlobalEQ");
assert_eq!(format!("{:?}",a.TNSQFlagID),format!("{:?}",b.TNSQFlagID),"pulsar.TNSQFlagID");
assert_eq!(format!("{:?}",a.TNSQFlagVal),format!("{:?}",b.TNSQFlagVal),"pulsar.TNSQFlagVal");
assert_eq!(format!("{:?}",a.TNSQVal),format!("{:?}",b.TNSQVal),"pulsar.TNSQVal");
assert_eq!(format!("{:?}",a.TNECORRFlagID),format!("{:?}",b.TNECORRFlagID),"pulsar.TNECORRFlagID");
assert_eq!(format!("{:?}",a.TNECORRFlagVal),format!("{:?}",b.TNECORRFlagVal),"pulsar.TNECORRFlagVal");
assert_eq!(format!("{:?}",a.TNECORRVal),format!("{:?}",b.TNECORRVal),"pulsar.TNECORRVal");
assert_eq!(format!("{:?}",a.TNSECORRFlagID),format!("{:?}",b.TNSECORRFlagID),"pulsar.TNSECORRFlagID");
assert_eq!(format!("{:?}",a.TNSECORRFlagVal),format!("{:?}",b.TNSECORRFlagVal),"pulsar.TNSECORRFlagVal");
assert_eq!(format!("{:?}",a.TNSECORRVal),format!("{:?}",b.TNSECORRVal),"pulsar.TNSECORRVal");
assert_eq!(format!("{:?}",a.TNRedAmp),format!("{:?}",b.TNRedAmp),"pulsar.TNRedAmp");
assert_eq!(format!("{:?}",a.TNRedGam),format!("{:?}",b.TNRedGam),"pulsar.TNRedGam");
assert_eq!(format!("{:?}",a.TNRedAmp2),format!("{:?}",b.TNRedAmp2),"pulsar.TNRedAmp2");
assert_eq!(format!("{:?}",a.TNRedGam2),format!("{:?}",b.TNRedGam2),"pulsar.TNRedGam2");
assert_eq!(format!("{:?}",a.TNRedC),format!("{:?}",b.TNRedC),"pulsar.TNRedC");
assert_eq!(format!("{:?}",a.TNRedCoeffs),format!("{:?}",b.TNRedCoeffs),"pulsar.TNRedCoeffs");
assert_eq!(format!("{:?}",a.TNRedFLow),format!("{:?}",b.TNRedFLow),"pulsar.TNRedFLow");
assert_eq!(format!("{:?}",a.TNRedCorner),format!("{:?}",b.TNRedCorner),"pulsar.TNRedCorner");
assert_eq!(format!("{:?}",a.TNDMAmp),format!("{:?}",b.TNDMAmp),"pulsar.TNDMAmp");
assert_eq!(format!("{:?}",a.TNDMGam),format!("{:?}",b.TNDMGam),"pulsar.TNDMGam");
assert_eq!(format!("{:?}",a.TNDMC),format!("{:?}",b.TNDMC),"pulsar.TNDMC");
assert_eq!(format!("{:?}",a.TNDMCoeffs),format!("{:?}",b.TNDMCoeffs),"pulsar.TNDMCoeffs");
assert_eq!(format!("{:?}",a.SWGPAmp),format!("{:?}",b.SWGPAmp),"pulsar.SWGPAmp");
assert_eq!(format!("{:?}",a.SWGPGam),format!("{:?}",b.SWGPGam),"pulsar.SWGPGam");
assert_eq!(format!("{:?}",a.SWGPC),format!("{:?}",b.SWGPC),"pulsar.SWGPC");
assert_eq!(format!("{:?}",a.SWGPCoeffs),format!("{:?}",b.SWGPCoeffs),"pulsar.SWGPCoeffs");
assert_eq!(format!("{:?}",a.TNChromAmp),format!("{:?}",b.TNChromAmp),"pulsar.TNChromAmp");
assert_eq!(format!("{:?}",a.TNChromGam),format!("{:?}",b.TNChromGam),"pulsar.TNChromGam");
assert_eq!(format!("{:?}",a.TNChromIdx),format!("{:?}",b.TNChromIdx),"pulsar.TNChromIdx");
assert_eq!(format!("{:?}",a.TNChromC),format!("{:?}",b.TNChromC),"pulsar.TNChromC");
assert_eq!(format!("{:?}",a.TNChromCoeffs),format!("{:?}",b.TNChromCoeffs),"pulsar.TNChromCoeffs");
assert_eq!(format!("{:?}",a.TNsubtractDM),format!("{:?}",b.TNsubtractDM),"pulsar.TNsubtractDM");
assert_eq!(format!("{:?}",a.TNsubtractRed),format!("{:?}",b.TNsubtractRed),"pulsar.TNsubtractRed");
assert_eq!(format!("{:?}",a.TNsubtractChrom),format!("{:?}",b.TNsubtractChrom),"pulsar.TNsubtractChrom");
assert_eq!(format!("{:?}",a.TNsubtractSWGP),format!("{:?}",b.TNsubtractSWGP),"pulsar.TNsubtractSWGP");
assert_eq!(format!("{:?}",a.AverageResiduals),format!("{:?}",b.AverageResiduals),"pulsar.AverageResiduals");
assert_eq!(format!("{:?}",a.AverageDMResiduals),format!("{:?}",b.AverageDMResiduals),"pulsar.AverageDMResiduals");
assert_eq!(format!("{:?}",a.AverageFlag),format!("{:?}",b.AverageFlag),"pulsar.AverageFlag");
assert_eq!(format!("{:?}",a.AverageEpochWidth),format!("{:?}",b.AverageEpochWidth),"pulsar.AverageEpochWidth");
assert_eq!(format!("{:?}",a.detL),format!("{:?}",b.detL),"pulsar.detL");
assert_eq!(format!("{:?}",a.outputTMatrix),format!("{:?}",b.outputTMatrix),"pulsar.outputTMatrix");
assert_eq!(format!("{:?}",a.useTNOrth),format!("{:?}",b.useTNOrth),"pulsar.useTNOrth");
assert_eq!(format!("{:?}",a.TNBandDMAmp),format!("{:?}",b.TNBandDMAmp),"pulsar.TNBandDMAmp");
assert_eq!(format!("{:?}",a.TNBandDMGam),format!("{:?}",b.TNBandDMGam),"pulsar.TNBandDMGam");
assert_eq!(format!("{:?}",a.TNBandDMC),format!("{:?}",b.TNBandDMC),"pulsar.TNBandDMC");
assert_eq!(format!("{:?}",a.nTNBandNoise),format!("{:?}",b.nTNBandNoise),"pulsar.nTNBandNoise");
assert_eq!(format!("{:?}",a.TNBandNoiseLF),format!("{:?}",b.TNBandNoiseLF),"pulsar.TNBandNoiseLF");
assert_eq!(format!("{:?}",a.TNBandNoiseHF),format!("{:?}",b.TNBandNoiseHF),"pulsar.TNBandNoiseHF");
assert_eq!(format!("{:?}",a.TNBandNoiseAmp),format!("{:?}",b.TNBandNoiseAmp),"pulsar.TNBandNoiseAmp");
assert_eq!(format!("{:?}",a.TNBandNoiseGam),format!("{:?}",b.TNBandNoiseGam),"pulsar.TNBandNoiseGam");
assert_eq!(format!("{:?}",a.TNBandNoiseC),format!("{:?}",b.TNBandNoiseC),"pulsar.TNBandNoiseC");
assert_eq!(format!("{:?}",a.nTNGroupNoise),format!("{:?}",b.nTNGroupNoise),"pulsar.nTNGroupNoise");
assert_eq!(format!("{:?}",a.TNGroupNoiseFlagID),format!("{:?}",b.TNGroupNoiseFlagID),"pulsar.TNGroupNoiseFlagID");
assert_eq!(format!("{:?}",a.TNGroupNoiseFlagVal),format!("{:?}",b.TNGroupNoiseFlagVal),"pulsar.TNGroupNoiseFlagVal");
assert_eq!(format!("{:?}",a.TNGroupNoiseAmp),format!("{:?}",b.TNGroupNoiseAmp),"pulsar.TNGroupNoiseAmp");
assert_eq!(format!("{:?}",a.TNGroupNoiseGam),format!("{:?}",b.TNGroupNoiseGam),"pulsar.TNGroupNoiseGam");
assert_eq!(format!("{:?}",a.TNGroupNoiseC),format!("{:?}",b.TNGroupNoiseC),"pulsar.TNGroupNoiseC");
assert_eq!(format!("{:?}",a.nDMEvents),format!("{:?}",b.nDMEvents),"pulsar.nDMEvents");
assert_eq!(format!("{:?}",a.nDMEventC),format!("{:?}",b.nDMEventC),"pulsar.nDMEventC");
assert_eq!(format!("{:?}",a.TNDMEvStart),format!("{:?}",b.TNDMEvStart),"pulsar.TNDMEvStart");
assert_eq!(format!("{:?}",a.TNDMEvLength),format!("{:?}",b.TNDMEvLength),"pulsar.TNDMEvLength");
assert_eq!(format!("{:?}",a.TNDMEvAmp),format!("{:?}",b.TNDMEvAmp),"pulsar.TNDMEvAmp");
assert_eq!(format!("{:?}",a.TNDMEvGam),format!("{:?}",b.TNDMEvGam),"pulsar.TNDMEvGam");
assert_eq!(format!("{:?}",a.TNDMEvOff),format!("{:?}",b.TNDMEvOff),"pulsar.TNDMEvOff");
assert_eq!(format!("{:?}",a.TNDMEvLin),format!("{:?}",b.TNDMEvLin),"pulsar.TNDMEvLin");
assert_eq!(format!("{:?}",a.TNDMEvQuad),format!("{:?}",b.TNDMEvQuad),"pulsar.TNDMEvQuad");
assert_eq!(format!("{:?}",a.nTNShapeletEvents),format!("{:?}",b.nTNShapeletEvents),"pulsar.nTNShapeletEvents");
assert_eq!(format!("{:?}",a.TNShapeletEvN),format!("{:?}",b.TNShapeletEvN),"pulsar.TNShapeletEvN");
assert_eq!(format!("{:?}",a.TNShapeletEvPos),format!("{:?}",b.TNShapeletEvPos),"pulsar.TNShapeletEvPos");
assert_eq!(format!("{:?}",a.TNShapeletEvWidth),format!("{:?}",b.TNShapeletEvWidth),"pulsar.TNShapeletEvWidth");
assert_eq!(format!("{:?}",a.TNShapeletEvFScale),format!("{:?}",b.TNShapeletEvFScale),"pulsar.TNShapeletEvFScale");
assert_eq!(format!("{:?}",a.TNShapeletEvCoef),format!("{:?}",b.TNShapeletEvCoef),"pulsar.TNShapeletEvCoef");
assert_eq!(format!("{:?}",a.TNShapeletEvCoefErr),format!("{:?}",b.TNShapeletEvCoefErr),"pulsar.TNShapeletEvCoefErr");
assert_eq!(format!("{:?}",a.TNShapeletEvFitFlag),format!("{:?}",b.TNShapeletEvFitFlag),"pulsar.TNShapeletEvFitFlag");
assert_eq!(format!("{:?}",a.whiteNoiseModelFile),format!("{:?}",b.whiteNoiseModelFile),"pulsar.whiteNoiseModelFile");
assert_eq!(format!("{:?}",a.rasim),format!("{:?}",b.rasim),"pulsar.rasim");
assert_eq!(format!("{:?}",a.decsim),format!("{:?}",b.decsim),"pulsar.decsim");
assert_eq!(format!("{:?}",a.simflag),format!("{:?}",b.simflag),"pulsar.simflag");
assert_eq!(format!("{:?}",a.fitFunc),format!("{:?}",b.fitFunc),"pulsar.fitFunc");
assert_eq!(format!("{:?}",a.nconstraints),format!("{:?}",b.nconstraints),"pulsar.nconstraints");
assert_eq!(format!("{:?}",a.constraint_efactor),format!("{:?}",b.constraint_efactor),"pulsar.constraint_efactor");
assert_eq!(format!("{:?}",a.constraints),format!("{:?}",b.constraints),"pulsar.constraints");
assert_eq!(format!("{:?}",a.auto_constraints),format!("{:?}",b.auto_constraints),"pulsar.auto_constraints");
assert_eq!(format!("{:?}",a.constraint_special),format!("{:?}",b.constraint_special),"pulsar.constraint_special");
assert_eq!(format!("{:?}",a.fitinfo),format!("{:?}",b.fitinfo),"pulsar.fitinfo");
assert_eq!(format!("{:?}",a.brace),format!("{:?}",b.brace),"pulsar.brace");
assert_eq!(format!("{:?}",a.tzrobs),format!("{:?}",b.tzrobs),"pulsar.tzrobs");
assert_eq!(format!("{:?}",a.refphs),format!("{:?}",b.refphs),"pulsar.refphs");
assert_eq!(format!("{:?}",a.posPulsarEquatorial),format!("{:?}",b.posPulsarEquatorial),"pulsar.posPulsarEquatorial");
assert_eq!(format!("{:?}",a.dm_series_type),format!("{:?}",b.dm_series_type),"pulsar.dm_series_type");
assert_eq!(format!("{:?}",a.TN_QpPeriod),format!("{:?}",b.TN_QpPeriod),"pulsar.TN_QpPeriod");
assert_eq!(format!("{:?}",a.TN_QpSig),format!("{:?}",b.TN_QpSig),"pulsar.TN_QpSig");
assert_eq!(format!("{:?}",a.TN_QpLam),format!("{:?}",b.TN_QpLam),"pulsar.TN_QpLam");
assert_eq!(format!("{:?}",a.TN_QpRatio),format!("{:?}",b.TN_QpRatio),"pulsar.TN_QpRatio");
assert_eq!(format!("{:?}",a.TNRed_log_freqs),format!("{:?}",b.TNRed_log_freqs),"pulsar.TNRed_log_freqs");
assert_eq!(format!("{:?}",a.TNRed_log_factor),format!("{:?}",b.TNRed_log_factor),"pulsar.TNRed_log_factor");
assert_eq!(format!("{:?}",a.ne_sw_ifuncT),format!("{:?}",b.ne_sw_ifuncT),"pulsar.ne_sw_ifuncT");
assert_eq!(format!("{:?}",a.ne_sw_ifuncV),format!("{:?}",b.ne_sw_ifuncV),"pulsar.ne_sw_ifuncV");
assert_eq!(format!("{:?}",a.ne_sw_ifuncE),format!("{:?}",b.ne_sw_ifuncE),"pulsar.ne_sw_ifuncE");
assert_eq!(format!("{:?}",a.ne_sw_ifuncN),format!("{:?}",b.ne_sw_ifuncN),"pulsar.ne_sw_ifuncN");
assert_eq!(format!("{:?}",a.TNDM_log_freqs),format!("{:?}",b.TNDM_log_freqs),"pulsar.TNDM_log_freqs");
assert_eq!(format!("{:?}",a.TNDM_log_factor),format!("{:?}",b.TNDM_log_factor),"pulsar.TNDM_log_factor");
assert_eq!(format!("{:?}",a.TNChrom_log_freqs),format!("{:?}",b.TNChrom_log_freqs),"pulsar.TNChrom_log_freqs");
assert_eq!(format!("{:?}",a.TNChrom_log_factor),format!("{:?}",b.TNChrom_log_factor),"pulsar.TNChrom_log_factor");
assert_eq!(format!("{:?}",a.TNsubtractPoly),format!("{:?}",b.TNsubtractPoly),"pulsar.TNsubtractPoly");
}
