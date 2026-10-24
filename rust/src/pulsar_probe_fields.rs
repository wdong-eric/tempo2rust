// Generated typed test visitors; source behavior is called, never emulated.
fn seed_clock_correction(x: &mut SourceClockCorrection, v:i32, opaque:PayloadView) {
    x.correction=Some(v as f64+0.25);
    seed_string(&mut x.corrects_to[..],v);
}
fn dump_clock_correction(out:&mut Dump,p:&str,x:&SourceClockCorrection,tags:&Tags) {
    out.values(&format!("{p}.correction"),std::iter::once(x.correction));
    out.values(&format!("{p}.corrects_to"),x.corrects_to.iter().copied());
}
fn seed_observation(x: &mut SourceObservation, v:i32, opaque:PayloadView) {
    x.sat=Some(Float::with_val(128,v as f64+0.125));
    x.origsat=Some(Float::with_val(128,v as f64+0.125));
    x.sat_day=Some(Float::with_val(128,v as f64+0.125));
    x.sat_sec=Some(Float::with_val(128,v as f64+0.125));
    x.bat=Some(Float::with_val(128,v as f64+0.125));
    x.batCorr=Some(Float::with_val(128,v as f64+0.125));
    x.bbat=Some(Float::with_val(128,v as f64+0.125));
    x.pet=Some(Float::with_val(128,v as f64+0.125));
    x.clockCorr=Some(v);
    x.delayCorr=Some(v);
    x.deleted=Some(v);
    x.prefitResidual=Some(Float::with_val(128,v as f64+0.125));
    x.residual=Some(Float::with_val(128,v as f64+0.125));
    x.residualtn=Some(Float::with_val(128,v as f64+0.125));
    x.addedNoise=Some(v as f64+0.25);
    x.TNRedSignal=Some(v as f64+0.25);
    x.TNRedErr=Some(v as f64+0.25);
    x.TNDMSignal=Some(v as f64+0.25);
    x.TNDMErr=Some(v as f64+0.25);
    x.SWGPSignal=Some(v as f64+0.25);
    x.SWGPErr=Some(v as f64+0.25);
    x.TNGroupSignal=Some(v as f64+0.25);
    x.TNGroupErr=Some(v as f64+0.25);
    x.TNChromSignal=Some(v as f64+0.25);
    x.TNChromErr=Some(v as f64+0.25);
    x.freq=Some(v as f64+0.25);
    x.freqSSB=Some(v as f64+0.25);
    x.toaErr=Some(v as f64+0.25);
    x.toaDMErr=Some(v as f64+0.25);
    x.origErr=Some(v as f64+0.25);
    x.phaseOffset=Some(v as f64+0.25);
    x.averagebat=Some(v as f64+0.25);
    x.averageres=Some(v as f64+0.25);
    x.averageerr=Some(v as f64+0.25);
    x.averagedmbat=Some(v as f64+0.25);
    x.averagedmres=Some(v as f64+0.25);
    x.averagedmerr=Some(v as f64+0.25);
    seed_string(&mut x.fname[..],v);
    seed_string(&mut x.telID[..],v);
    for cell in x.correctionsTT.iter_mut() {seed_clock_correction(cell,v,opaque);}
    x.nclock_correction=Some(v);
    x.correctionTT_TB=Some(Float::with_val(128,v as f64+0.125));
    x.einsteinRate=Some(v as f64+0.25);
    x.correctionTT_calcEph=Some(Float::with_val(128,v as f64+0.125));
    x.correctionTT_Teph=Some(Float::with_val(128,v as f64+0.125));
    x.correctionUT1=Some(Float::with_val(128,v as f64+0.125));
    for cell0 in x.sun_ssb.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.sun_earth.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.planet_ssb.iter_mut() {
    for cell1 in cell0.iter_mut() {
        *cell1=Some(v as f64+0.25);
    }
    }
    for cell0 in x.planet_ssb_tmr.iter_mut() {
    for cell1 in cell0.iter_mut() {
        *cell1=Some(v as f64+0.25);
    }
    }
    for cell0 in x.planet_ssb_derv.iter_mut() {
    for cell1 in cell0.iter_mut() {
        *cell1=Some(v as f64+0.25);
    }
    }
    for cell0 in x.jupiter_earth.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.saturn_earth.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.venus_earth.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.uranus_earth.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.neptune_earth.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.earthMoonBary_ssb.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.earthMoonBary_earth.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.earth_ssb.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.observatory_earth.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.psrPos.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.zenith.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.nutations.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.siteVel.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.shklovskii=Some(Float::with_val(128,v as f64+0.125));
    x.shapiroDelaySun=Some(v as f64+0.25);
    x.shapiroDelayJupiter=Some(v as f64+0.25);
    x.shapiroDelaySaturn=Some(v as f64+0.25);
    x.shapiroDelayVenus=Some(v as f64+0.25);
    x.shapiroDelayUranus=Some(v as f64+0.25);
    x.shapiroDelayNeptune=Some(v as f64+0.25);
    x.troposphericDelay=Some(v as f64+0.25);
    x.tdis1=Some(v as f64+0.25);
    x.tdis2=Some(v as f64+0.25);
    x.roemer=Some(Float::with_val(128,v as f64+0.125));
    x.torb=Some(Float::with_val(128,v as f64+0.125));
    x.nphase=Some(Float::with_val(128,v as f64+0.125));
    x.phase=Some(Float::with_val(128,v as f64+0.125));
    x.pulseN=Some(v as i64);
    for row in x.flagID.iter_mut() {seed_string(row,v);}
    for row in x.flagVal.iter_mut() {seed_string(row,v);}
    x.nFlags=Some(v);
    for cell0 in x.jump.iter_mut() {
        *cell0=Some(v);
    }
    for cell0 in x.jumpScale.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.obsNjump=Some(v);
    for cell0 in x.fdjump.iter_mut() {
        *cell0=Some(v);
    }
    x.obsNfdjump=Some(v);
    x.efac=Some(v as f64+0.25);
    x.equad=Some(v as f64+0.25);
    x.snr=Some(v as f64+0.25);
    x.pnoise=Some(v as f64+0.25);
    x.tobs=Some(v as f64+0.25);
    x.chisq=Some(v as f64+0.25);
    x.bline=Some(v as f64+0.25);
    x.spherical_solar_wind=Some(v as f64+0.25);
}
fn dump_observation(out:&mut Dump,p:&str,x:&SourceObservation,tags:&Tags) {
    out.values(&format!("{p}.sat"),std::iter::once(x.sat.clone()));
    out.values(&format!("{p}.origsat"),std::iter::once(x.origsat.clone()));
    out.values(&format!("{p}.sat_day"),std::iter::once(x.sat_day.clone()));
    out.values(&format!("{p}.sat_sec"),std::iter::once(x.sat_sec.clone()));
    out.values(&format!("{p}.bat"),std::iter::once(x.bat.clone()));
    out.values(&format!("{p}.batCorr"),std::iter::once(x.batCorr.clone()));
    out.values(&format!("{p}.bbat"),std::iter::once(x.bbat.clone()));
    out.values(&format!("{p}.pet"),std::iter::once(x.pet.clone()));
    out.values(&format!("{p}.clockCorr"),std::iter::once(x.clockCorr));
    out.values(&format!("{p}.delayCorr"),std::iter::once(x.delayCorr));
    out.values(&format!("{p}.deleted"),std::iter::once(x.deleted));
    out.values(&format!("{p}.prefitResidual"),std::iter::once(x.prefitResidual.clone()));
    out.values(&format!("{p}.residual"),std::iter::once(x.residual.clone()));
    out.values(&format!("{p}.residualtn"),std::iter::once(x.residualtn.clone()));
    out.values(&format!("{p}.addedNoise"),std::iter::once(x.addedNoise));
    out.values(&format!("{p}.TNRedSignal"),std::iter::once(x.TNRedSignal));
    out.values(&format!("{p}.TNRedErr"),std::iter::once(x.TNRedErr));
    out.values(&format!("{p}.TNDMSignal"),std::iter::once(x.TNDMSignal));
    out.values(&format!("{p}.TNDMErr"),std::iter::once(x.TNDMErr));
    out.values(&format!("{p}.SWGPSignal"),std::iter::once(x.SWGPSignal));
    out.values(&format!("{p}.SWGPErr"),std::iter::once(x.SWGPErr));
    out.values(&format!("{p}.TNGroupSignal"),std::iter::once(x.TNGroupSignal));
    out.values(&format!("{p}.TNGroupErr"),std::iter::once(x.TNGroupErr));
    out.values(&format!("{p}.TNChromSignal"),std::iter::once(x.TNChromSignal));
    out.values(&format!("{p}.TNChromErr"),std::iter::once(x.TNChromErr));
    out.values(&format!("{p}.freq"),std::iter::once(x.freq));
    out.values(&format!("{p}.freqSSB"),std::iter::once(x.freqSSB));
    out.values(&format!("{p}.toaErr"),std::iter::once(x.toaErr));
    out.values(&format!("{p}.toaDMErr"),std::iter::once(x.toaDMErr));
    out.values(&format!("{p}.origErr"),std::iter::once(x.origErr));
    out.values(&format!("{p}.phaseOffset"),std::iter::once(x.phaseOffset));
    out.values(&format!("{p}.averagebat"),std::iter::once(x.averagebat));
    out.values(&format!("{p}.averageres"),std::iter::once(x.averageres));
    out.values(&format!("{p}.averageerr"),std::iter::once(x.averageerr));
    out.values(&format!("{p}.averagedmbat"),std::iter::once(x.averagedmbat));
    out.values(&format!("{p}.averagedmres"),std::iter::once(x.averagedmres));
    out.values(&format!("{p}.averagedmerr"),std::iter::once(x.averagedmerr));
    out.values(&format!("{p}.fname"),x.fname.iter().copied());
    out.values(&format!("{p}.telID"),x.telID.iter().copied());
    for (i,cell) in x.correctionsTT.iter().enumerate() {dump_clock_correction(out,&format!("{p}.correctionsTT[{i}]"),cell,tags);}
    out.values(&format!("{p}.nclock_correction"),std::iter::once(x.nclock_correction));
    out.values(&format!("{p}.correctionTT_TB"),std::iter::once(x.correctionTT_TB.clone()));
    out.values(&format!("{p}.einsteinRate"),std::iter::once(x.einsteinRate));
    out.values(&format!("{p}.correctionTT_calcEph"),std::iter::once(x.correctionTT_calcEph.clone()));
    out.values(&format!("{p}.correctionTT_Teph"),std::iter::once(x.correctionTT_Teph.clone()));
    out.values(&format!("{p}.correctionUT1"),std::iter::once(x.correctionUT1.clone()));
    out.values(&format!("{p}.sun_ssb"),x.sun_ssb.iter().copied());
    out.values(&format!("{p}.sun_earth"),x.sun_earth.iter().copied());
    out.values(&format!("{p}.planet_ssb"),x.planet_ssb.iter().flatten().copied());
    out.values(&format!("{p}.planet_ssb_tmr"),x.planet_ssb_tmr.iter().flatten().copied());
    out.values(&format!("{p}.planet_ssb_derv"),x.planet_ssb_derv.iter().flatten().copied());
    out.values(&format!("{p}.jupiter_earth"),x.jupiter_earth.iter().copied());
    out.values(&format!("{p}.saturn_earth"),x.saturn_earth.iter().copied());
    out.values(&format!("{p}.venus_earth"),x.venus_earth.iter().copied());
    out.values(&format!("{p}.uranus_earth"),x.uranus_earth.iter().copied());
    out.values(&format!("{p}.neptune_earth"),x.neptune_earth.iter().copied());
    out.values(&format!("{p}.earthMoonBary_ssb"),x.earthMoonBary_ssb.iter().copied());
    out.values(&format!("{p}.earthMoonBary_earth"),x.earthMoonBary_earth.iter().copied());
    out.values(&format!("{p}.earth_ssb"),x.earth_ssb.iter().copied());
    out.values(&format!("{p}.observatory_earth"),x.observatory_earth.iter().copied());
    out.values(&format!("{p}.psrPos"),x.psrPos.iter().copied());
    out.values(&format!("{p}.zenith"),x.zenith.iter().copied());
    out.values(&format!("{p}.nutations"),x.nutations.iter().copied());
    out.values(&format!("{p}.siteVel"),x.siteVel.iter().copied());
    out.values(&format!("{p}.shklovskii"),std::iter::once(x.shklovskii.clone()));
    out.values(&format!("{p}.shapiroDelaySun"),std::iter::once(x.shapiroDelaySun));
    out.values(&format!("{p}.shapiroDelayJupiter"),std::iter::once(x.shapiroDelayJupiter));
    out.values(&format!("{p}.shapiroDelaySaturn"),std::iter::once(x.shapiroDelaySaturn));
    out.values(&format!("{p}.shapiroDelayVenus"),std::iter::once(x.shapiroDelayVenus));
    out.values(&format!("{p}.shapiroDelayUranus"),std::iter::once(x.shapiroDelayUranus));
    out.values(&format!("{p}.shapiroDelayNeptune"),std::iter::once(x.shapiroDelayNeptune));
    out.values(&format!("{p}.troposphericDelay"),std::iter::once(x.troposphericDelay));
    out.values(&format!("{p}.tdis1"),std::iter::once(x.tdis1));
    out.values(&format!("{p}.tdis2"),std::iter::once(x.tdis2));
    out.values(&format!("{p}.roemer"),std::iter::once(x.roemer.clone()));
    out.values(&format!("{p}.torb"),std::iter::once(x.torb.clone()));
    out.values(&format!("{p}.nphase"),std::iter::once(x.nphase.clone()));
    out.values(&format!("{p}.phase"),std::iter::once(x.phase.clone()));
    out.values(&format!("{p}.pulseN"),std::iter::once(x.pulseN));
    out.values(&format!("{p}.flagID"),x.flagID.iter().flatten().copied());
    out.values(&format!("{p}.flagVal"),x.flagVal.iter().flatten().copied());
    out.values(&format!("{p}.nFlags"),std::iter::once(x.nFlags));
    out.values(&format!("{p}.jump"),x.jump.iter().copied());
    out.values(&format!("{p}.jumpScale"),x.jumpScale.iter().copied());
    out.values(&format!("{p}.obsNjump"),std::iter::once(x.obsNjump));
    out.values(&format!("{p}.fdjump"),x.fdjump.iter().copied());
    out.values(&format!("{p}.obsNfdjump"),std::iter::once(x.obsNfdjump));
    out.values(&format!("{p}.efac"),std::iter::once(x.efac));
    out.values(&format!("{p}.equad"),std::iter::once(x.equad));
    out.values(&format!("{p}.snr"),std::iter::once(x.snr));
    out.values(&format!("{p}.pnoise"),std::iter::once(x.pnoise));
    out.values(&format!("{p}.tobs"),std::iter::once(x.tobs));
    out.values(&format!("{p}.chisq"),std::iter::once(x.chisq));
    out.values(&format!("{p}.bline"),std::iter::once(x.bline));
    out.values(&format!("{p}.spherical_solar_wind"),std::iter::once(x.spherical_solar_wind));
}
fn seed_storePrecision(x: &mut StorePrecision, v:i32, opaque:PayloadView) {
    x.minPrec=Some(Float::with_val(128,v as f64+0.125));
    seed_string(&mut x.routine[..],v);
    seed_string(&mut x.comment[..],v);
}
fn dump_storePrecision(out:&mut Dump,p:&str,x:&StorePrecision,tags:&Tags) {
    out.values(&format!("{p}.minPrec"),std::iter::once(x.minPrec.clone()));
    out.values(&format!("{p}.routine"),x.routine.iter().copied());
    out.values(&format!("{p}.comment"),x.comment.iter().copied());
}
fn seed_FitOutput(x: &mut FitOutput, v:i32, opaque:PayloadView) {
    for cell0 in x.parameterEstimates.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.errorEstimates.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.indexPsr.iter_mut() {
        *cell0=Some(v);
    }
    for cell0 in x.indexParam.iter_mut() {
        *cell0=Some(v);
    }
    for cell0 in x.indexCounter.iter_mut() {
        *cell0=Some(v);
    }
    x.totalNfit=Some(v);
}
fn dump_FitOutput(out:&mut Dump,p:&str,x:&FitOutput,tags:&Tags) {
    out.values(&format!("{p}.parameterEstimates"),x.parameterEstimates.iter().copied());
    out.values(&format!("{p}.errorEstimates"),x.errorEstimates.iter().copied());
    out.values(&format!("{p}.indexPsr"),x.indexPsr.iter().copied());
    out.values(&format!("{p}.indexParam"),x.indexParam.iter().copied());
    out.values(&format!("{p}.indexCounter"),x.indexCounter.iter().copied());
    out.values(&format!("{p}.totalNfit"),std::iter::once(x.totalNfit));
}
fn seed_FitInfo(x: &mut FitInfo, v:i32, opaque:PayloadView) {
    x.nParams=Some(v as u32);
    x.nConstraints=Some(v as u32);
    for cell0 in x.paramIndex.iter_mut() {
        *cell0=Some(v);
    }
    for cell0 in x.constraintIndex.iter_mut() {
        *cell0=Some(v);
    }
    for cell0 in x.paramCounters.iter_mut() {
        *cell0=Some(v);
    }
    for cell0 in x.constraintCounters.iter_mut() {
        *cell0=Some(v);
    }
    for cell0 in x.paramDerivs.iter_mut() {
        *cell0=Some(Some(Callback {identity:if v==11 {0} else {1},callable:deriv}));
    }
    for cell0 in x.constraintDerivs.iter_mut() {
        *cell0=Some(Some(Callback {identity:if v==11 {0} else {1},callable:constraint}));
    }
    for cell0 in x.constraintSpecial.iter_mut() {
        *cell0=Some(Some(opaque));
    }
    for cell0 in x.constraintValue.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.updateFunctions.iter_mut() {
        *cell0=Some(Some(Callback {identity:if v==11 {0} else {1},callable:update}));
    }
    seed_FitOutput(&mut x.output,v,opaque);
}
fn dump_FitInfo(out:&mut Dump,p:&str,x:&FitInfo,tags:&Tags) {
    out.values(&format!("{p}.nParams"),std::iter::once(x.nParams));
    out.values(&format!("{p}.nConstraints"),std::iter::once(x.nConstraints));
    out.values(&format!("{p}.paramIndex"),x.paramIndex.iter().copied());
    out.values(&format!("{p}.constraintIndex"),x.constraintIndex.iter().copied());
    out.values(&format!("{p}.paramCounters"),x.paramCounters.iter().copied());
    out.values(&format!("{p}.constraintCounters"),x.constraintCounters.iter().copied());
    out.pointers(&format!("{p}.paramDerivs"),x.paramDerivs.iter().map(|v|v.as_ref().map(|v|v.as_ref().map_or_else(||"null".into(),|v|format!("callback:deriv{}",v.identity)))));
    out.pointers(&format!("{p}.constraintDerivs"),x.constraintDerivs.iter().map(|v|v.as_ref().map(|v|v.as_ref().map_or_else(||"null".into(),|v|format!("callback:constraint{}",v.identity)))));
    out.pointers(&format!("{p}.constraintSpecial"),x.constraintSpecial.iter().map(|v|v.as_ref().map(|v|v.as_ref().map_or_else(||"null".into(),|v|tags.payload(v)))));
    out.values(&format!("{p}.constraintValue"),x.constraintValue.iter().copied());
    out.pointers(&format!("{p}.updateFunctions"),x.updateFunctions.iter().map(|v|v.as_ref().map(|v|v.as_ref().map_or_else(||"null".into(),|v|format!("callback:update{}",v.identity)))));
    dump_FitOutput(out,&format!("{p}.output"),&x.output,tags);
}
fn seed_pulsar(x: &mut SourcePulsar, v:i32, opaque:PayloadView) {
    seed_string(&mut x.name[..],v);
    seed_string(&mut x.eopc04_file[..],v);
    x.fixedFormat=Some(v);
    seed_string(&mut x.rajStrPre[..],v);
    seed_string(&mut x.decjStrPre[..],v);
    seed_string(&mut x.rajStrPost[..],v);
    seed_string(&mut x.decjStrPost[..],v);
    seed_string(&mut x.binaryModel[..],v);
    x.ToAextraCovar=Some(None);
    x.dmoffsDMnum=Some(v);
    x.dmoffsCMnum=Some(v);
    for cell0 in x.dmoffsDM_mjd.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.dmoffsDM.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.dmoffsDM_error.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.dmoffsDM_weight.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.dmoffsCM_mjd.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.dmoffsCM.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.dmoffsCM_error.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.dmoffsCM_weight.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.gwsrc_ra=Some(v as f64+0.25);
    x.gwsrc_dec=Some(v as f64+0.25);
    x.gwsrc_aplus_r=Some(v as f64+0.25);
    x.gwsrc_aplus_i=Some(v as f64+0.25);
    x.gwsrc_across_r=Some(v as f64+0.25);
    x.gwsrc_across_i=Some(v as f64+0.25);
    x.gwsrc_aplus_r_e=Some(v as f64+0.25);
    x.gwsrc_aplus_i_e=Some(v as f64+0.25);
    x.gwsrc_across_r_e=Some(v as f64+0.25);
    x.gwsrc_across_i_e=Some(v as f64+0.25);
    x.gwsrc_epoch=Some(v as f64+0.25);
    x.gwsrc_psrdist=Some(v as f64+0.25);
    x.cgw_h0=Some(v as f64+0.25);
    x.cgw_cosinc=Some(v as f64+0.25);
    x.cgw_angpol=Some(v as f64+0.25);
    x.cgw_mc=Some(v as f64+0.25);
    x.gwm_raj=Some(v as f64+0.25);
    x.gwm_decj=Some(v as f64+0.25);
    x.gwm_epoch=Some(v as f64+0.25);
    x.gwm_phi=Some(v as f64+0.25);
    x.gwm_dphase=Some(v as f64+0.25);
    x.gwcs_raj=Some(v as f64+0.25);
    x.gwcs_decj=Some(v as f64+0.25);
    x.gwcs_epoch=Some(v as f64+0.25);
    x.gwcs_width=Some(v as f64+0.25);
    x.gwcs_geom_p=Some(v as f64+0.25);
    x.gwcs_geom_c=Some(v as f64+0.25);
    x.gwb_epoch=Some(v as f64+0.25);
    x.gwb_width=Some(v as f64+0.25);
    x.gwb_raj=Some(v as f64+0.25);
    x.gwb_decj=Some(v as f64+0.25);
    x.gwb_geom_c=Some(v as f64+0.25);
    x.gwb_geom_p=Some(v as f64+0.25);
    x.gwecc_ra=Some(v as f64+0.25);
    x.gwecc_dec=Some(v as f64+0.25);
    x.gwecc_m1=Some(v as f64+0.25);
    x.gwecc_m2=Some(v as f64+0.25);
    x.gwecc_e=Some(v as f64+0.25);
    x.gwecc_inc=Some(v as f64+0.25);
    x.gwecc_theta_nodes=Some(v as f64+0.25);
    x.gwecc_nodes_orientation=Some(v as f64+0.25);
    x.gwecc_theta_0=Some(v as f64+0.25);
    x.gwecc_orbital_period=Some(v as f64+0.25);
    x.gwecc_distance=Some(v as f64+0.25);
    x.gwecc_redshift=Some(v as f64+0.25);
    x.gwecc_epoch=Some(v as f64+0.25);
    x.gwecc_psrdist=Some(v as f64+0.25);
    x.gwecc_pulsarTermOn=Some(v);
    for cell0 in x.posPulsar.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.velPulsar.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.accPulsar.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.phaseJump.iter_mut() {
        *cell0=Some(Float::with_val(128,v as f64+0.125));
    }
    for cell0 in x.phaseJumpDir.iter_mut() {
        *cell0=Some(v);
    }
    for cell0 in x.phaseJumpID.iter_mut() {
        *cell0=Some(v);
    }
    x.nPhaseJump=Some(v);
    x.dmOffset=Some(v as f64+0.25);
    x.ne_sw=Some(v as f64+0.25);
    x.nCompanion=Some(v);
    x.eclCoord=Some(v);
    x.nJumps=Some(v);
    seed_string(&mut x.fjumpID[..],v);
    for cell0 in x.jumpVal.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    seed_string(&mut x.jumpSAT[..],v);
    for cell0 in x.fitJump.iter_mut() {
        *cell0=Some(v);
    }
    for cell0 in x.jumpValErr.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for row in x.jumpStr.iter_mut() {seed_string(row,v);}
    seed_string(&mut x.jumpScaled[..],v);
    x.nfdJumps=Some(v);
    x.fdjump_pint_format=Some(v as u8);
    for cell0 in x.fdjumpVal.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.fdjumpIdx.iter_mut() {
        *cell0=Some(v);
    }
    for cell0 in x.fitfdJump.iter_mut() {
        *cell0=Some(v);
    }
    for cell0 in x.fdjumpValErr.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for row in x.fdjumpStr.iter_mut() {seed_string(row,v);}
    x.fdjump_log=Some(v as u8);
    seed_string(&mut x.filterStr[..],v);
    seed_string(&mut x.passStr[..],v);
    for cell0 in x.tOffset.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.tOffset_f1.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.tOffset_f2.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.tOffset_t1.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.tOffset_t2.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for row in x.tOffsetSite.iter_mut() {seed_string(row,v);}
    for row in x.tOffsetFlags.iter_mut() {seed_string(row,v);}
    x.nToffset=Some(v);
    x.ndmx=Some(v);
    x.nSx=Some(v);
    x.nchromx=Some(v);
    x.fitChisq=Some(v as f64+0.25);
    x.fitNfree=Some(v);
    x.globalNfit=Some(v);
    x.globalNoConstrain=Some(v);
    x.nFit=Some(v);
    x.nParam=Some(v);
    x.nGlobal=Some(v);
    x.fitMode=Some(v);
    x.robust=Some(v as u8);
    x.rescaleErrChisq=Some(v);
    x.offset=Some(v as f64+0.25);
    x.offset_e=Some(v as f64+0.25);
    x.covar=Some(None);
    x.calcShapiro=Some(v);
    x.planetShapiro=Some(v);
    x.jboFormat=Some(v);
    x.obsn=Some(None);
    x.nobs=Some(v);
    x.units=Some(v);
    x.setUnits=Some(v);
    x.tempo1=Some(v);
    x.dilateFreq=Some(v);
    x.timeEphemeris=Some(v);
    x.t2cMethod=Some(v);
    x.correctTroposphere=Some(v);
    x.noWarnings=Some(v);
    x.sorted=Some(v as u8);
    seed_string(&mut x.clock[..],v);
    seed_string(&mut x.clockFromOverride[..],v);
    seed_string(&mut x.JPL_EPHEMERIS[..],v);
    seed_string(&mut x.ephemeris[..],v);
    x.useCalceph=Some(v);
    for cell in x.storePrec.iter_mut() {seed_storePrecision(cell,v,opaque);}
    x.nStorePrecision=Some(v);
    x.bootStrap=Some(v);
    seed_string(&mut x.tzrsite[..],v);
    x.rmsPre=Some(v as f64+0.25);
    x.rmsPost=Some(v as f64+0.25);
    x.rmstn=Some(v as f64+0.25);
    seed_string(&mut x.deleteFileName[..],v);
    x.nits=Some(v);
    x.ipm=Some(v);
    x.swm=Some(v);
    for cell0 in x.wave_sine.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.wave_sine_err.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.wave_cos.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.wave_cos_err.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.wave_sine_dm.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.wave_sine_dm_err.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.wave_cos_dm.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.wave_cos_dm_err.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.nWhite=Some(v);
    x.nWhite_dm=Some(v);
    x.waveScale=Some(v as f64+0.25);
    for cell0 in x.quad_aplus_r.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.quad_aplus_r_e.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.quad_aplus_i.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.quad_aplus_i_e.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.quad_across_r.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.quad_across_r_e.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.quad_across_i.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.quad_across_i_e.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.quadEpoch=Some(v as f64+0.25);
    x.quadRA=Some(v as f64+0.25);
    x.quadDEC=Some(v as f64+0.25);
    x.nQuad=Some(v);
    for cell0 in x.ifuncT.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.ifuncV.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.ifuncE.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.ifunc_weights.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.ifuncN=Some(v);
    for cell0 in x.orbifuncT.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.orbifuncV.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.orbifuncE.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.orbifuncN=Some(v);
    for cell0 in x.clk_offsT.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.clk_offsV.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.clk_offsE.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.clkOffsN=Some(v);
    for cell0 in x.quad_ifuncT_p.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.quad_ifuncV_p.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.quad_ifuncE_p.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.quad_ifuncN_p=Some(v);
    for cell0 in x.quad_ifuncT_c.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.quad_ifuncV_c.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.quad_ifuncE_c.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.quad_ifuncN_c=Some(v);
    x.quad_ifunc_p_RA=Some(v as f64+0.25);
    x.quad_ifunc_p_DEC=Some(v as f64+0.25);
    x.quad_ifunc_c_RA=Some(v as f64+0.25);
    x.quad_ifunc_c_DEC=Some(v as f64+0.25);
    x.quad_ifunc_geom_p=Some(v as f64+0.25);
    x.quad_ifunc_geom_c=Some(v as f64+0.25);
    x.nTelDX=Some(v);
    x.setTelVelX=Some(v);
    for cell0 in x.telDX_t.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.telDX_v.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.telDX_e.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.telDX_vel.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.telDX_vel_e.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.nTelDY=Some(v);
    x.setTelVelY=Some(v);
    for cell0 in x.telDY_t.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.telDY_v.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.telDY_e.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.telDY_vel.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.telDY_vel_e.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.nTelDZ=Some(v);
    x.setTelVelZ=Some(v);
    for cell0 in x.telDZ_v.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.telDZ_t.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.telDZ_e.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.telDZ_vel.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.telDZ_vel_e.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.nT2efac=Some(v);
    x.nT2equad=Some(v);
    for row in x.T2efacFlagID.iter_mut() {seed_string(row,v);}
    for row in x.T2efacFlagVal.iter_mut() {seed_string(row,v);}
    for cell0 in x.T2efacVal.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for row in x.T2equadFlagID.iter_mut() {seed_string(row,v);}
    for row in x.T2equadFlagVal.iter_mut() {seed_string(row,v);}
    for cell0 in x.T2equadVal.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.T2globalEfac=Some(v as f64+0.25);
    x.nTNEF=Some(v);
    x.nTNEQ=Some(v);
    x.nTNSQ=Some(v);
    x.nTNECORR=Some(v);
    x.nTNSECORR=Some(v);
    for row in x.TNEFFlagID.iter_mut() {seed_string(row,v);}
    for row in x.TNEFFlagVal.iter_mut() {seed_string(row,v);}
    for cell0 in x.TNEFVal.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.TNGlobalEF=Some(v as f64+0.25);
    for row in x.TNEQFlagID.iter_mut() {seed_string(row,v);}
    for row in x.TNEQFlagVal.iter_mut() {seed_string(row,v);}
    for cell0 in x.TNEQVal.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.TNGlobalEQ=Some(v as f64+0.25);
    x.addTNGlobalEQ=Some(v as f64+0.25);
    for row in x.TNSQFlagID.iter_mut() {seed_string(row,v);}
    for row in x.TNSQFlagVal.iter_mut() {seed_string(row,v);}
    for cell0 in x.TNSQVal.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for row in x.TNECORRFlagID.iter_mut() {seed_string(row,v);}
    for row in x.TNECORRFlagVal.iter_mut() {seed_string(row,v);}
    for cell0 in x.TNECORRVal.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for row in x.TNSECORRFlagID.iter_mut() {seed_string(row,v);}
    for row in x.TNSECORRFlagVal.iter_mut() {seed_string(row,v);}
    for cell0 in x.TNSECORRVal.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.TNRedAmp=Some(v as f64+0.25);
    x.TNRedGam=Some(v as f64+0.25);
    x.TNRedAmp2=Some(v as f64+0.25);
    x.TNRedGam2=Some(v as f64+0.25);
    x.TNRedC=Some(v);
    for cell0 in x.TNRedCoeffs.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.TNRedFLow=Some(v as f64+0.25);
    x.TNRedCorner=Some(v as f64+0.25);
    x.TNDMAmp=Some(v as f64+0.25);
    x.TNDMGam=Some(v as f64+0.25);
    x.TNDMC=Some(v);
    for cell0 in x.TNDMCoeffs.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.SWGPAmp=Some(v as f64+0.25);
    x.SWGPGam=Some(v as f64+0.25);
    x.SWGPC=Some(v);
    for cell0 in x.SWGPCoeffs.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.TNChromAmp=Some(v as f64+0.25);
    x.TNChromGam=Some(v as f64+0.25);
    x.TNChromIdx=Some(v as f64+0.25);
    x.TNChromC=Some(v);
    for cell0 in x.TNChromCoeffs.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.TNsubtractDM=Some(v);
    x.TNsubtractRed=Some(v);
    x.TNsubtractChrom=Some(v);
    x.TNsubtractSWGP=Some(v);
    x.AverageResiduals=Some(v);
    x.AverageDMResiduals=Some(v);
    seed_string(&mut x.AverageFlag[..],v);
    x.AverageEpochWidth=Some(v as f32+0.5);
    x.detL=Some(v as f64+0.25);
    x.outputTMatrix=Some(v);
    x.useTNOrth=Some(v);
    x.TNBandDMAmp=Some(v as f64+0.25);
    x.TNBandDMGam=Some(v as f64+0.25);
    x.TNBandDMC=Some(v);
    x.nTNBandNoise=Some(v);
    for cell0 in x.TNBandNoiseLF.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.TNBandNoiseHF.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.TNBandNoiseAmp.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.TNBandNoiseGam.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.TNBandNoiseC.iter_mut() {
        *cell0=Some(v);
    }
    x.nTNGroupNoise=Some(v);
    for row in x.TNGroupNoiseFlagID.iter_mut() {seed_string(row,v);}
    for row in x.TNGroupNoiseFlagVal.iter_mut() {seed_string(row,v);}
    for cell0 in x.TNGroupNoiseAmp.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.TNGroupNoiseGam.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.TNGroupNoiseC.iter_mut() {
        *cell0=Some(v);
    }
    x.nDMEvents=Some(v);
    x.nDMEventC=Some(v);
    for cell0 in x.TNDMEvStart.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.TNDMEvLength.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.TNDMEvAmp.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.TNDMEvGam.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.TNDMEvOff.iter_mut() {
        *cell0=Some(v);
    }
    for cell0 in x.TNDMEvLin.iter_mut() {
        *cell0=Some(v);
    }
    for cell0 in x.TNDMEvQuad.iter_mut() {
        *cell0=Some(v);
    }
    x.nTNShapeletEvents=Some(v);
    for cell0 in x.TNShapeletEvN.iter_mut() {
        *cell0=Some(v);
    }
    for cell0 in x.TNShapeletEvPos.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.TNShapeletEvWidth.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.TNShapeletEvFScale.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.TNShapeletEvCoef.iter_mut() {
    for cell1 in cell0.iter_mut() {
        *cell1=Some(v as f64+0.25);
    }
    }
    for cell0 in x.TNShapeletEvCoefErr.iter_mut() {
    for cell1 in cell0.iter_mut() {
        *cell1=Some(v as f64+0.25);
    }
    }
    for cell0 in x.TNShapeletEvFitFlag.iter_mut() {
        *cell0=Some(v);
    }
    seed_string(&mut x.whiteNoiseModelFile[..],v);
    x.rasim=Some(v as f64+0.25);
    x.decsim=Some(v as f64+0.25);
    x.simflag=Some(v);
    seed_string(&mut x.fitFunc[..],v);
    x.nconstraints=Some(v);
    x.constraint_efactor=Some(v as f64+0.25);
    for cell0 in x.constraints.iter_mut() {
        *cell0=Some(if v==11 {Constraint::constraint_dmmodel_mean} else {Constraint::constraint_dmmodel_cw_0});
    }
    x.auto_constraints=Some(v as u8);
    for cell0 in x.constraint_special.iter_mut() {
        *cell0=Some(None);
    }
    seed_FitInfo(&mut x.fitinfo,v,opaque);
    x.brace=Some(v);
    seed_observation(&mut x.tzrobs,v,opaque);
    x.refphs=Some(v as u8);
    for cell0 in x.posPulsarEquatorial.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.dm_series_type=Some(if v==11 {SeriesType::series_undefined} else {SeriesType::series_simple_pn});
    x.TN_QpPeriod=Some(v as f64+0.25);
    x.TN_QpSig=Some(v as f64+0.25);
    x.TN_QpLam=Some(v as f64+0.25);
    x.TN_QpRatio=Some(v as f64+0.25);
    x.TNRed_log_freqs=Some(v);
    x.TNRed_log_factor=Some(v as f64+0.25);
    for cell0 in x.ne_sw_ifuncT.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.ne_sw_ifuncV.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    for cell0 in x.ne_sw_ifuncE.iter_mut() {
        *cell0=Some(v as f64+0.25);
    }
    x.ne_sw_ifuncN=Some(v);
    x.TNDM_log_freqs=Some(v);
    x.TNDM_log_factor=Some(v as f64+0.25);
    x.TNChrom_log_freqs=Some(v);
    x.TNChrom_log_factor=Some(v as f64+0.25);
    x.TNsubtractPoly=Some(v as u64);
}
fn dump_pulsar(out:&mut Dump,p:&str,x:&SourcePulsar,tags:&Tags) {
    out.values(&format!("{p}.name"),x.name.iter().copied());
    out.values(&format!("{p}.eopc04_file"),x.eopc04_file.iter().copied());
    out.values(&format!("{p}.fixedFormat"),std::iter::once(x.fixedFormat));
    params(out,&format!("{p}.param"),&x.param,tags);
    out.values(&format!("{p}.rajStrPre"),x.rajStrPre.iter().copied());
    out.values(&format!("{p}.decjStrPre"),x.decjStrPre.iter().copied());
    out.values(&format!("{p}.rajStrPost"),x.rajStrPost.iter().copied());
    out.values(&format!("{p}.decjStrPost"),x.decjStrPost.iter().copied());
    out.values(&format!("{p}.binaryModel"),x.binaryModel.iter().copied());
    match &x.ToAextraCovar {None=>out.unwritten(&format!("{p}.ToAextraCovar")),Some(None)=>out.symbol(&format!("{p}.ToAextraCovar"),"null"),Some(Some(v))=>out.symbol(&format!("{p}.ToAextraCovar"),&tags.matrix(v))};
    out.values(&format!("{p}.dmoffsDMnum"),std::iter::once(x.dmoffsDMnum));
    out.values(&format!("{p}.dmoffsCMnum"),std::iter::once(x.dmoffsCMnum));
    out.values(&format!("{p}.dmoffsDM_mjd"),x.dmoffsDM_mjd.iter().copied());
    out.values(&format!("{p}.dmoffsDM"),x.dmoffsDM.iter().copied());
    out.values(&format!("{p}.dmoffsDM_error"),x.dmoffsDM_error.iter().copied());
    out.values(&format!("{p}.dmoffsDM_weight"),x.dmoffsDM_weight.iter().copied());
    out.values(&format!("{p}.dmoffsCM_mjd"),x.dmoffsCM_mjd.iter().copied());
    out.values(&format!("{p}.dmoffsCM"),x.dmoffsCM.iter().copied());
    out.values(&format!("{p}.dmoffsCM_error"),x.dmoffsCM_error.iter().copied());
    out.values(&format!("{p}.dmoffsCM_weight"),x.dmoffsCM_weight.iter().copied());
    out.values(&format!("{p}.gwsrc_ra"),std::iter::once(x.gwsrc_ra));
    out.values(&format!("{p}.gwsrc_dec"),std::iter::once(x.gwsrc_dec));
    out.values(&format!("{p}.gwsrc_aplus_r"),std::iter::once(x.gwsrc_aplus_r));
    out.values(&format!("{p}.gwsrc_aplus_i"),std::iter::once(x.gwsrc_aplus_i));
    out.values(&format!("{p}.gwsrc_across_r"),std::iter::once(x.gwsrc_across_r));
    out.values(&format!("{p}.gwsrc_across_i"),std::iter::once(x.gwsrc_across_i));
    out.values(&format!("{p}.gwsrc_aplus_r_e"),std::iter::once(x.gwsrc_aplus_r_e));
    out.values(&format!("{p}.gwsrc_aplus_i_e"),std::iter::once(x.gwsrc_aplus_i_e));
    out.values(&format!("{p}.gwsrc_across_r_e"),std::iter::once(x.gwsrc_across_r_e));
    out.values(&format!("{p}.gwsrc_across_i_e"),std::iter::once(x.gwsrc_across_i_e));
    out.values(&format!("{p}.gwsrc_epoch"),std::iter::once(x.gwsrc_epoch));
    out.values(&format!("{p}.gwsrc_psrdist"),std::iter::once(x.gwsrc_psrdist));
    out.values(&format!("{p}.cgw_h0"),std::iter::once(x.cgw_h0));
    out.values(&format!("{p}.cgw_cosinc"),std::iter::once(x.cgw_cosinc));
    out.values(&format!("{p}.cgw_angpol"),std::iter::once(x.cgw_angpol));
    out.values(&format!("{p}.cgw_mc"),std::iter::once(x.cgw_mc));
    out.values(&format!("{p}.gwm_raj"),std::iter::once(x.gwm_raj));
    out.values(&format!("{p}.gwm_decj"),std::iter::once(x.gwm_decj));
    out.values(&format!("{p}.gwm_epoch"),std::iter::once(x.gwm_epoch));
    out.values(&format!("{p}.gwm_phi"),std::iter::once(x.gwm_phi));
    out.values(&format!("{p}.gwm_dphase"),std::iter::once(x.gwm_dphase));
    out.values(&format!("{p}.gwcs_raj"),std::iter::once(x.gwcs_raj));
    out.values(&format!("{p}.gwcs_decj"),std::iter::once(x.gwcs_decj));
    out.values(&format!("{p}.gwcs_epoch"),std::iter::once(x.gwcs_epoch));
    out.values(&format!("{p}.gwcs_width"),std::iter::once(x.gwcs_width));
    out.values(&format!("{p}.gwcs_geom_p"),std::iter::once(x.gwcs_geom_p));
    out.values(&format!("{p}.gwcs_geom_c"),std::iter::once(x.gwcs_geom_c));
    out.values(&format!("{p}.gwb_epoch"),std::iter::once(x.gwb_epoch));
    out.values(&format!("{p}.gwb_width"),std::iter::once(x.gwb_width));
    out.values(&format!("{p}.gwb_raj"),std::iter::once(x.gwb_raj));
    out.values(&format!("{p}.gwb_decj"),std::iter::once(x.gwb_decj));
    out.values(&format!("{p}.gwb_geom_c"),std::iter::once(x.gwb_geom_c));
    out.values(&format!("{p}.gwb_geom_p"),std::iter::once(x.gwb_geom_p));
    out.values(&format!("{p}.gwecc_ra"),std::iter::once(x.gwecc_ra));
    out.values(&format!("{p}.gwecc_dec"),std::iter::once(x.gwecc_dec));
    out.values(&format!("{p}.gwecc_m1"),std::iter::once(x.gwecc_m1));
    out.values(&format!("{p}.gwecc_m2"),std::iter::once(x.gwecc_m2));
    out.values(&format!("{p}.gwecc_e"),std::iter::once(x.gwecc_e));
    out.values(&format!("{p}.gwecc_inc"),std::iter::once(x.gwecc_inc));
    out.values(&format!("{p}.gwecc_theta_nodes"),std::iter::once(x.gwecc_theta_nodes));
    out.values(&format!("{p}.gwecc_nodes_orientation"),std::iter::once(x.gwecc_nodes_orientation));
    out.values(&format!("{p}.gwecc_theta_0"),std::iter::once(x.gwecc_theta_0));
    out.values(&format!("{p}.gwecc_orbital_period"),std::iter::once(x.gwecc_orbital_period));
    out.values(&format!("{p}.gwecc_distance"),std::iter::once(x.gwecc_distance));
    out.values(&format!("{p}.gwecc_redshift"),std::iter::once(x.gwecc_redshift));
    out.values(&format!("{p}.gwecc_epoch"),std::iter::once(x.gwecc_epoch));
    out.values(&format!("{p}.gwecc_psrdist"),std::iter::once(x.gwecc_psrdist));
    out.values(&format!("{p}.gwecc_pulsarTermOn"),std::iter::once(x.gwecc_pulsarTermOn));
    out.values(&format!("{p}.posPulsar"),x.posPulsar.iter().copied());
    out.values(&format!("{p}.velPulsar"),x.velPulsar.iter().copied());
    out.values(&format!("{p}.accPulsar"),x.accPulsar.iter().copied());
    out.values(&format!("{p}.phaseJump"),x.phaseJump.iter().cloned());
    out.values(&format!("{p}.phaseJumpDir"),x.phaseJumpDir.iter().copied());
    out.values(&format!("{p}.phaseJumpID"),x.phaseJumpID.iter().copied());
    out.values(&format!("{p}.nPhaseJump"),std::iter::once(x.nPhaseJump));
    out.values(&format!("{p}.dmOffset"),std::iter::once(x.dmOffset));
    out.values(&format!("{p}.ne_sw"),std::iter::once(x.ne_sw));
    out.values(&format!("{p}.nCompanion"),std::iter::once(x.nCompanion));
    out.values(&format!("{p}.eclCoord"),std::iter::once(x.eclCoord));
    out.values(&format!("{p}.nJumps"),std::iter::once(x.nJumps));
    out.values(&format!("{p}.fjumpID"),x.fjumpID.iter().copied());
    out.values(&format!("{p}.jumpVal"),x.jumpVal.iter().copied());
    out.values(&format!("{p}.jumpSAT"),x.jumpSAT.iter().copied());
    out.values(&format!("{p}.fitJump"),x.fitJump.iter().copied());
    out.values(&format!("{p}.jumpValErr"),x.jumpValErr.iter().copied());
    out.values(&format!("{p}.jumpStr"),x.jumpStr.iter().flatten().copied());
    out.values(&format!("{p}.jumpScaled"),x.jumpScaled.iter().copied());
    out.values(&format!("{p}.nfdJumps"),std::iter::once(x.nfdJumps));
    out.values(&format!("{p}.fdjump_pint_format"),std::iter::once(x.fdjump_pint_format));
    out.values(&format!("{p}.fdjumpVal"),x.fdjumpVal.iter().copied());
    out.values(&format!("{p}.fdjumpIdx"),x.fdjumpIdx.iter().copied());
    out.values(&format!("{p}.fitfdJump"),x.fitfdJump.iter().copied());
    out.values(&format!("{p}.fdjumpValErr"),x.fdjumpValErr.iter().copied());
    out.values(&format!("{p}.fdjumpStr"),x.fdjumpStr.iter().flatten().copied());
    out.values(&format!("{p}.fdjump_log"),std::iter::once(x.fdjump_log));
    out.values(&format!("{p}.filterStr"),x.filterStr.iter().copied());
    out.values(&format!("{p}.passStr"),x.passStr.iter().copied());
    out.values(&format!("{p}.tOffset"),x.tOffset.iter().copied());
    out.values(&format!("{p}.tOffset_f1"),x.tOffset_f1.iter().copied());
    out.values(&format!("{p}.tOffset_f2"),x.tOffset_f2.iter().copied());
    out.values(&format!("{p}.tOffset_t1"),x.tOffset_t1.iter().copied());
    out.values(&format!("{p}.tOffset_t2"),x.tOffset_t2.iter().copied());
    out.values(&format!("{p}.tOffsetSite"),x.tOffsetSite.iter().flatten().copied());
    out.values(&format!("{p}.tOffsetFlags"),x.tOffsetFlags.iter().flatten().copied());
    out.values(&format!("{p}.nToffset"),std::iter::once(x.nToffset));
    out.values(&format!("{p}.ndmx"),std::iter::once(x.ndmx));
    out.values(&format!("{p}.nSx"),std::iter::once(x.nSx));
    out.values(&format!("{p}.nchromx"),std::iter::once(x.nchromx));
    out.values(&format!("{p}.fitChisq"),std::iter::once(x.fitChisq));
    out.values(&format!("{p}.fitNfree"),std::iter::once(x.fitNfree));
    out.values(&format!("{p}.globalNfit"),std::iter::once(x.globalNfit));
    out.values(&format!("{p}.globalNoConstrain"),std::iter::once(x.globalNoConstrain));
    out.values(&format!("{p}.nFit"),std::iter::once(x.nFit));
    out.values(&format!("{p}.nParam"),std::iter::once(x.nParam));
    out.values(&format!("{p}.nGlobal"),std::iter::once(x.nGlobal));
    out.values(&format!("{p}.fitMode"),std::iter::once(x.fitMode));
    out.values(&format!("{p}.robust"),std::iter::once(x.robust));
    out.values(&format!("{p}.rescaleErrChisq"),std::iter::once(x.rescaleErrChisq));
    out.values(&format!("{p}.offset"),std::iter::once(x.offset));
    out.values(&format!("{p}.offset_e"),std::iter::once(x.offset_e));
    match &x.covar {None=>out.unwritten(&format!("{p}.covar")),Some(None)=>out.symbol(&format!("{p}.covar"),"null"),Some(Some(v))=>out.symbol(&format!("{p}.covar"),&tags.matrix(v))};
    out.values(&format!("{p}.calcShapiro"),std::iter::once(x.calcShapiro));
    out.values(&format!("{p}.planetShapiro"),std::iter::once(x.planetShapiro));
    out.values(&format!("{p}.jboFormat"),std::iter::once(x.jboFormat));
    match &x.obsn {None=>out.unwritten(&format!("{p}.obsn")),Some(None)=>out.symbol(&format!("{p}.obsn"),"null"),Some(Some(v))=>out.symbol(&format!("{p}.obsn"),&tags.observation(v))};
    out.values(&format!("{p}.nobs"),std::iter::once(x.nobs));
    out.values(&format!("{p}.units"),std::iter::once(x.units));
    out.values(&format!("{p}.setUnits"),std::iter::once(x.setUnits));
    out.values(&format!("{p}.tempo1"),std::iter::once(x.tempo1));
    out.values(&format!("{p}.dilateFreq"),std::iter::once(x.dilateFreq));
    out.values(&format!("{p}.timeEphemeris"),std::iter::once(x.timeEphemeris));
    out.values(&format!("{p}.t2cMethod"),std::iter::once(x.t2cMethod));
    out.values(&format!("{p}.correctTroposphere"),std::iter::once(x.correctTroposphere));
    out.values(&format!("{p}.noWarnings"),std::iter::once(x.noWarnings));
    out.values(&format!("{p}.sorted"),std::iter::once(x.sorted));
    out.values(&format!("{p}.clock"),x.clock.iter().copied());
    out.values(&format!("{p}.clockFromOverride"),x.clockFromOverride.iter().copied());
    out.values(&format!("{p}.JPL_EPHEMERIS"),x.JPL_EPHEMERIS.iter().copied());
    out.values(&format!("{p}.ephemeris"),x.ephemeris.iter().copied());
    out.values(&format!("{p}.useCalceph"),std::iter::once(x.useCalceph));
    for (i,cell) in x.storePrec.iter().enumerate() {dump_storePrecision(out,&format!("{p}.storePrec[{i}]"),cell,tags);}
    out.values(&format!("{p}.nStorePrecision"),std::iter::once(x.nStorePrecision));
    out.values(&format!("{p}.bootStrap"),std::iter::once(x.bootStrap));
    out.values(&format!("{p}.tzrsite"),x.tzrsite.iter().copied());
    out.values(&format!("{p}.rmsPre"),std::iter::once(x.rmsPre));
    out.values(&format!("{p}.rmsPost"),std::iter::once(x.rmsPost));
    out.values(&format!("{p}.rmstn"),std::iter::once(x.rmstn));
    out.values(&format!("{p}.deleteFileName"),x.deleteFileName.iter().copied());
    out.values(&format!("{p}.nits"),std::iter::once(x.nits));
    out.values(&format!("{p}.ipm"),std::iter::once(x.ipm));
    out.values(&format!("{p}.swm"),std::iter::once(x.swm));
    out.values(&format!("{p}.wave_sine"),x.wave_sine.iter().copied());
    out.values(&format!("{p}.wave_sine_err"),x.wave_sine_err.iter().copied());
    out.values(&format!("{p}.wave_cos"),x.wave_cos.iter().copied());
    out.values(&format!("{p}.wave_cos_err"),x.wave_cos_err.iter().copied());
    out.values(&format!("{p}.wave_sine_dm"),x.wave_sine_dm.iter().copied());
    out.values(&format!("{p}.wave_sine_dm_err"),x.wave_sine_dm_err.iter().copied());
    out.values(&format!("{p}.wave_cos_dm"),x.wave_cos_dm.iter().copied());
    out.values(&format!("{p}.wave_cos_dm_err"),x.wave_cos_dm_err.iter().copied());
    out.values(&format!("{p}.nWhite"),std::iter::once(x.nWhite));
    out.values(&format!("{p}.nWhite_dm"),std::iter::once(x.nWhite_dm));
    out.values(&format!("{p}.waveScale"),std::iter::once(x.waveScale));
    out.values(&format!("{p}.quad_aplus_r"),x.quad_aplus_r.iter().copied());
    out.values(&format!("{p}.quad_aplus_r_e"),x.quad_aplus_r_e.iter().copied());
    out.values(&format!("{p}.quad_aplus_i"),x.quad_aplus_i.iter().copied());
    out.values(&format!("{p}.quad_aplus_i_e"),x.quad_aplus_i_e.iter().copied());
    out.values(&format!("{p}.quad_across_r"),x.quad_across_r.iter().copied());
    out.values(&format!("{p}.quad_across_r_e"),x.quad_across_r_e.iter().copied());
    out.values(&format!("{p}.quad_across_i"),x.quad_across_i.iter().copied());
    out.values(&format!("{p}.quad_across_i_e"),x.quad_across_i_e.iter().copied());
    out.values(&format!("{p}.quadEpoch"),std::iter::once(x.quadEpoch));
    out.values(&format!("{p}.quadRA"),std::iter::once(x.quadRA));
    out.values(&format!("{p}.quadDEC"),std::iter::once(x.quadDEC));
    out.values(&format!("{p}.nQuad"),std::iter::once(x.nQuad));
    out.values(&format!("{p}.ifuncT"),x.ifuncT.iter().copied());
    out.values(&format!("{p}.ifuncV"),x.ifuncV.iter().copied());
    out.values(&format!("{p}.ifuncE"),x.ifuncE.iter().copied());
    out.values(&format!("{p}.ifunc_weights"),x.ifunc_weights.iter().copied());
    out.values(&format!("{p}.ifuncN"),std::iter::once(x.ifuncN));
    out.values(&format!("{p}.orbifuncT"),x.orbifuncT.iter().copied());
    out.values(&format!("{p}.orbifuncV"),x.orbifuncV.iter().copied());
    out.values(&format!("{p}.orbifuncE"),x.orbifuncE.iter().copied());
    out.values(&format!("{p}.orbifuncN"),std::iter::once(x.orbifuncN));
    out.values(&format!("{p}.clk_offsT"),x.clk_offsT.iter().copied());
    out.values(&format!("{p}.clk_offsV"),x.clk_offsV.iter().copied());
    out.values(&format!("{p}.clk_offsE"),x.clk_offsE.iter().copied());
    out.values(&format!("{p}.clkOffsN"),std::iter::once(x.clkOffsN));
    out.values(&format!("{p}.quad_ifuncT_p"),x.quad_ifuncT_p.iter().copied());
    out.values(&format!("{p}.quad_ifuncV_p"),x.quad_ifuncV_p.iter().copied());
    out.values(&format!("{p}.quad_ifuncE_p"),x.quad_ifuncE_p.iter().copied());
    out.values(&format!("{p}.quad_ifuncN_p"),std::iter::once(x.quad_ifuncN_p));
    out.values(&format!("{p}.quad_ifuncT_c"),x.quad_ifuncT_c.iter().copied());
    out.values(&format!("{p}.quad_ifuncV_c"),x.quad_ifuncV_c.iter().copied());
    out.values(&format!("{p}.quad_ifuncE_c"),x.quad_ifuncE_c.iter().copied());
    out.values(&format!("{p}.quad_ifuncN_c"),std::iter::once(x.quad_ifuncN_c));
    out.values(&format!("{p}.quad_ifunc_p_RA"),std::iter::once(x.quad_ifunc_p_RA));
    out.values(&format!("{p}.quad_ifunc_p_DEC"),std::iter::once(x.quad_ifunc_p_DEC));
    out.values(&format!("{p}.quad_ifunc_c_RA"),std::iter::once(x.quad_ifunc_c_RA));
    out.values(&format!("{p}.quad_ifunc_c_DEC"),std::iter::once(x.quad_ifunc_c_DEC));
    out.values(&format!("{p}.quad_ifunc_geom_p"),std::iter::once(x.quad_ifunc_geom_p));
    out.values(&format!("{p}.quad_ifunc_geom_c"),std::iter::once(x.quad_ifunc_geom_c));
    out.values(&format!("{p}.nTelDX"),std::iter::once(x.nTelDX));
    out.values(&format!("{p}.setTelVelX"),std::iter::once(x.setTelVelX));
    out.values(&format!("{p}.telDX_t"),x.telDX_t.iter().copied());
    out.values(&format!("{p}.telDX_v"),x.telDX_v.iter().copied());
    out.values(&format!("{p}.telDX_e"),x.telDX_e.iter().copied());
    out.values(&format!("{p}.telDX_vel"),x.telDX_vel.iter().copied());
    out.values(&format!("{p}.telDX_vel_e"),x.telDX_vel_e.iter().copied());
    out.values(&format!("{p}.nTelDY"),std::iter::once(x.nTelDY));
    out.values(&format!("{p}.setTelVelY"),std::iter::once(x.setTelVelY));
    out.values(&format!("{p}.telDY_t"),x.telDY_t.iter().copied());
    out.values(&format!("{p}.telDY_v"),x.telDY_v.iter().copied());
    out.values(&format!("{p}.telDY_e"),x.telDY_e.iter().copied());
    out.values(&format!("{p}.telDY_vel"),x.telDY_vel.iter().copied());
    out.values(&format!("{p}.telDY_vel_e"),x.telDY_vel_e.iter().copied());
    out.values(&format!("{p}.nTelDZ"),std::iter::once(x.nTelDZ));
    out.values(&format!("{p}.setTelVelZ"),std::iter::once(x.setTelVelZ));
    out.values(&format!("{p}.telDZ_v"),x.telDZ_v.iter().copied());
    out.values(&format!("{p}.telDZ_t"),x.telDZ_t.iter().copied());
    out.values(&format!("{p}.telDZ_e"),x.telDZ_e.iter().copied());
    out.values(&format!("{p}.telDZ_vel"),x.telDZ_vel.iter().copied());
    out.values(&format!("{p}.telDZ_vel_e"),x.telDZ_vel_e.iter().copied());
    out.values(&format!("{p}.nT2efac"),std::iter::once(x.nT2efac));
    out.values(&format!("{p}.nT2equad"),std::iter::once(x.nT2equad));
    out.values(&format!("{p}.T2efacFlagID"),x.T2efacFlagID.iter().flatten().copied());
    out.values(&format!("{p}.T2efacFlagVal"),x.T2efacFlagVal.iter().flatten().copied());
    out.values(&format!("{p}.T2efacVal"),x.T2efacVal.iter().copied());
    out.values(&format!("{p}.T2equadFlagID"),x.T2equadFlagID.iter().flatten().copied());
    out.values(&format!("{p}.T2equadFlagVal"),x.T2equadFlagVal.iter().flatten().copied());
    out.values(&format!("{p}.T2equadVal"),x.T2equadVal.iter().copied());
    out.values(&format!("{p}.T2globalEfac"),std::iter::once(x.T2globalEfac));
    out.values(&format!("{p}.nTNEF"),std::iter::once(x.nTNEF));
    out.values(&format!("{p}.nTNEQ"),std::iter::once(x.nTNEQ));
    out.values(&format!("{p}.nTNSQ"),std::iter::once(x.nTNSQ));
    out.values(&format!("{p}.nTNECORR"),std::iter::once(x.nTNECORR));
    out.values(&format!("{p}.nTNSECORR"),std::iter::once(x.nTNSECORR));
    out.values(&format!("{p}.TNEFFlagID"),x.TNEFFlagID.iter().flatten().copied());
    out.values(&format!("{p}.TNEFFlagVal"),x.TNEFFlagVal.iter().flatten().copied());
    out.values(&format!("{p}.TNEFVal"),x.TNEFVal.iter().copied());
    out.values(&format!("{p}.TNGlobalEF"),std::iter::once(x.TNGlobalEF));
    out.values(&format!("{p}.TNEQFlagID"),x.TNEQFlagID.iter().flatten().copied());
    out.values(&format!("{p}.TNEQFlagVal"),x.TNEQFlagVal.iter().flatten().copied());
    out.values(&format!("{p}.TNEQVal"),x.TNEQVal.iter().copied());
    out.values(&format!("{p}.TNGlobalEQ"),std::iter::once(x.TNGlobalEQ));
    out.values(&format!("{p}.addTNGlobalEQ"),std::iter::once(x.addTNGlobalEQ));
    out.values(&format!("{p}.TNSQFlagID"),x.TNSQFlagID.iter().flatten().copied());
    out.values(&format!("{p}.TNSQFlagVal"),x.TNSQFlagVal.iter().flatten().copied());
    out.values(&format!("{p}.TNSQVal"),x.TNSQVal.iter().copied());
    out.values(&format!("{p}.TNECORRFlagID"),x.TNECORRFlagID.iter().flatten().copied());
    out.values(&format!("{p}.TNECORRFlagVal"),x.TNECORRFlagVal.iter().flatten().copied());
    out.values(&format!("{p}.TNECORRVal"),x.TNECORRVal.iter().copied());
    out.values(&format!("{p}.TNSECORRFlagID"),x.TNSECORRFlagID.iter().flatten().copied());
    out.values(&format!("{p}.TNSECORRFlagVal"),x.TNSECORRFlagVal.iter().flatten().copied());
    out.values(&format!("{p}.TNSECORRVal"),x.TNSECORRVal.iter().copied());
    out.values(&format!("{p}.TNRedAmp"),std::iter::once(x.TNRedAmp));
    out.values(&format!("{p}.TNRedGam"),std::iter::once(x.TNRedGam));
    out.values(&format!("{p}.TNRedAmp2"),std::iter::once(x.TNRedAmp2));
    out.values(&format!("{p}.TNRedGam2"),std::iter::once(x.TNRedGam2));
    out.values(&format!("{p}.TNRedC"),std::iter::once(x.TNRedC));
    out.values(&format!("{p}.TNRedCoeffs"),x.TNRedCoeffs.iter().copied());
    out.values(&format!("{p}.TNRedFLow"),std::iter::once(x.TNRedFLow));
    out.values(&format!("{p}.TNRedCorner"),std::iter::once(x.TNRedCorner));
    out.values(&format!("{p}.TNDMAmp"),std::iter::once(x.TNDMAmp));
    out.values(&format!("{p}.TNDMGam"),std::iter::once(x.TNDMGam));
    out.values(&format!("{p}.TNDMC"),std::iter::once(x.TNDMC));
    out.values(&format!("{p}.TNDMCoeffs"),x.TNDMCoeffs.iter().copied());
    out.values(&format!("{p}.SWGPAmp"),std::iter::once(x.SWGPAmp));
    out.values(&format!("{p}.SWGPGam"),std::iter::once(x.SWGPGam));
    out.values(&format!("{p}.SWGPC"),std::iter::once(x.SWGPC));
    out.values(&format!("{p}.SWGPCoeffs"),x.SWGPCoeffs.iter().copied());
    out.values(&format!("{p}.TNChromAmp"),std::iter::once(x.TNChromAmp));
    out.values(&format!("{p}.TNChromGam"),std::iter::once(x.TNChromGam));
    out.values(&format!("{p}.TNChromIdx"),std::iter::once(x.TNChromIdx));
    out.values(&format!("{p}.TNChromC"),std::iter::once(x.TNChromC));
    out.values(&format!("{p}.TNChromCoeffs"),x.TNChromCoeffs.iter().copied());
    out.values(&format!("{p}.TNsubtractDM"),std::iter::once(x.TNsubtractDM));
    out.values(&format!("{p}.TNsubtractRed"),std::iter::once(x.TNsubtractRed));
    out.values(&format!("{p}.TNsubtractChrom"),std::iter::once(x.TNsubtractChrom));
    out.values(&format!("{p}.TNsubtractSWGP"),std::iter::once(x.TNsubtractSWGP));
    out.values(&format!("{p}.AverageResiduals"),std::iter::once(x.AverageResiduals));
    out.values(&format!("{p}.AverageDMResiduals"),std::iter::once(x.AverageDMResiduals));
    out.values(&format!("{p}.AverageFlag"),x.AverageFlag.iter().copied());
    out.values(&format!("{p}.AverageEpochWidth"),std::iter::once(x.AverageEpochWidth));
    out.values(&format!("{p}.detL"),std::iter::once(x.detL));
    out.values(&format!("{p}.outputTMatrix"),std::iter::once(x.outputTMatrix));
    out.values(&format!("{p}.useTNOrth"),std::iter::once(x.useTNOrth));
    out.values(&format!("{p}.TNBandDMAmp"),std::iter::once(x.TNBandDMAmp));
    out.values(&format!("{p}.TNBandDMGam"),std::iter::once(x.TNBandDMGam));
    out.values(&format!("{p}.TNBandDMC"),std::iter::once(x.TNBandDMC));
    out.values(&format!("{p}.nTNBandNoise"),std::iter::once(x.nTNBandNoise));
    out.values(&format!("{p}.TNBandNoiseLF"),x.TNBandNoiseLF.iter().copied());
    out.values(&format!("{p}.TNBandNoiseHF"),x.TNBandNoiseHF.iter().copied());
    out.values(&format!("{p}.TNBandNoiseAmp"),x.TNBandNoiseAmp.iter().copied());
    out.values(&format!("{p}.TNBandNoiseGam"),x.TNBandNoiseGam.iter().copied());
    out.values(&format!("{p}.TNBandNoiseC"),x.TNBandNoiseC.iter().copied());
    out.values(&format!("{p}.nTNGroupNoise"),std::iter::once(x.nTNGroupNoise));
    out.values(&format!("{p}.TNGroupNoiseFlagID"),x.TNGroupNoiseFlagID.iter().flatten().copied());
    out.values(&format!("{p}.TNGroupNoiseFlagVal"),x.TNGroupNoiseFlagVal.iter().flatten().copied());
    out.values(&format!("{p}.TNGroupNoiseAmp"),x.TNGroupNoiseAmp.iter().copied());
    out.values(&format!("{p}.TNGroupNoiseGam"),x.TNGroupNoiseGam.iter().copied());
    out.values(&format!("{p}.TNGroupNoiseC"),x.TNGroupNoiseC.iter().copied());
    out.values(&format!("{p}.nDMEvents"),std::iter::once(x.nDMEvents));
    out.values(&format!("{p}.nDMEventC"),std::iter::once(x.nDMEventC));
    out.values(&format!("{p}.TNDMEvStart"),x.TNDMEvStart.iter().copied());
    out.values(&format!("{p}.TNDMEvLength"),x.TNDMEvLength.iter().copied());
    out.values(&format!("{p}.TNDMEvAmp"),x.TNDMEvAmp.iter().copied());
    out.values(&format!("{p}.TNDMEvGam"),x.TNDMEvGam.iter().copied());
    out.values(&format!("{p}.TNDMEvOff"),x.TNDMEvOff.iter().copied());
    out.values(&format!("{p}.TNDMEvLin"),x.TNDMEvLin.iter().copied());
    out.values(&format!("{p}.TNDMEvQuad"),x.TNDMEvQuad.iter().copied());
    out.values(&format!("{p}.nTNShapeletEvents"),std::iter::once(x.nTNShapeletEvents));
    out.values(&format!("{p}.TNShapeletEvN"),x.TNShapeletEvN.iter().copied());
    out.values(&format!("{p}.TNShapeletEvPos"),x.TNShapeletEvPos.iter().copied());
    out.values(&format!("{p}.TNShapeletEvWidth"),x.TNShapeletEvWidth.iter().copied());
    out.values(&format!("{p}.TNShapeletEvFScale"),x.TNShapeletEvFScale.iter().copied());
    out.values(&format!("{p}.TNShapeletEvCoef"),x.TNShapeletEvCoef.iter().flatten().copied());
    out.values(&format!("{p}.TNShapeletEvCoefErr"),x.TNShapeletEvCoefErr.iter().flatten().copied());
    out.values(&format!("{p}.TNShapeletEvFitFlag"),x.TNShapeletEvFitFlag.iter().copied());
    out.values(&format!("{p}.whiteNoiseModelFile"),x.whiteNoiseModelFile.iter().copied());
    out.values(&format!("{p}.rasim"),std::iter::once(x.rasim));
    out.values(&format!("{p}.decsim"),std::iter::once(x.decsim));
    out.values(&format!("{p}.simflag"),std::iter::once(x.simflag));
    out.values(&format!("{p}.fitFunc"),x.fitFunc.iter().copied());
    out.values(&format!("{p}.nconstraints"),std::iter::once(x.nconstraints));
    out.values(&format!("{p}.constraint_efactor"),std::iter::once(x.constraint_efactor));
    out.values(&format!("{p}.constraints"),x.constraints.iter().map(|v|v.map(|n| n as i32)));
    out.values(&format!("{p}.auto_constraints"),std::iter::once(x.auto_constraints));
    out.pointers(&format!("{p}.constraint_special"),x.constraint_special.iter().map(|v|v.as_ref().map(|v|v.as_ref().map_or_else(||"null".into(),|v|tags.payload(v)))));
    dump_FitInfo(out,&format!("{p}.fitinfo"),&x.fitinfo,tags);
    out.values(&format!("{p}.brace"),std::iter::once(x.brace));
    dump_observation(out,&format!("{p}.tzrobs"),&x.tzrobs,tags);
    out.values(&format!("{p}.refphs"),std::iter::once(x.refphs));
    out.values(&format!("{p}.posPulsarEquatorial"),x.posPulsarEquatorial.iter().copied());
    out.values(&format!("{p}.dm_series_type"),std::iter::once(x.dm_series_type.map(|n|n as i32)));
    out.values(&format!("{p}.TN_QpPeriod"),std::iter::once(x.TN_QpPeriod));
    out.values(&format!("{p}.TN_QpSig"),std::iter::once(x.TN_QpSig));
    out.values(&format!("{p}.TN_QpLam"),std::iter::once(x.TN_QpLam));
    out.values(&format!("{p}.TN_QpRatio"),std::iter::once(x.TN_QpRatio));
    out.values(&format!("{p}.TNRed_log_freqs"),std::iter::once(x.TNRed_log_freqs));
    out.values(&format!("{p}.TNRed_log_factor"),std::iter::once(x.TNRed_log_factor));
    out.values(&format!("{p}.ne_sw_ifuncT"),x.ne_sw_ifuncT.iter().copied());
    out.values(&format!("{p}.ne_sw_ifuncV"),x.ne_sw_ifuncV.iter().copied());
    out.values(&format!("{p}.ne_sw_ifuncE"),x.ne_sw_ifuncE.iter().copied());
    out.values(&format!("{p}.ne_sw_ifuncN"),std::iter::once(x.ne_sw_ifuncN));
    out.values(&format!("{p}.TNDM_log_freqs"),std::iter::once(x.TNDM_log_freqs));
    out.values(&format!("{p}.TNDM_log_factor"),std::iter::once(x.TNDM_log_factor));
    out.values(&format!("{p}.TNChrom_log_freqs"),std::iter::once(x.TNChrom_log_freqs));
    out.values(&format!("{p}.TNChrom_log_factor"),std::iter::once(x.TNChrom_log_factor));
    out.values(&format!("{p}.TNsubtractPoly"),std::iter::once(x.TNsubtractPoly));
}
fn natural(out:&mut Dump,p:&str,x:&SourcePulsar,tags:&Tags) {
    out.prefix(&format!("{p}.name"),&x.name[..]);
    out.prefix(&format!("{p}.eopc04_file"),&x.eopc04_file[..]);
    out.values(&format!("{p}.fixedFormat"),std::iter::once(x.fixedFormat));
    params(out,&format!("{p}.param"),&x.param,tags);
    out.unwritten(&format!("{p}.rajStrPre"));
    out.unwritten(&format!("{p}.decjStrPre"));
    out.unwritten(&format!("{p}.rajStrPost"));
    out.unwritten(&format!("{p}.decjStrPost"));
    out.prefix(&format!("{p}.binaryModel"),&x.binaryModel[..]);
    match &x.ToAextraCovar {None=>out.unwritten(&format!("{p}.ToAextraCovar")),Some(None)=>out.symbol(&format!("{p}.ToAextraCovar"),"null"),Some(Some(v))=>out.symbol(&format!("{p}.ToAextraCovar"),&tags.matrix(v))};
    out.values(&format!("{p}.dmoffsDMnum"),std::iter::once(x.dmoffsDMnum));
    out.values(&format!("{p}.dmoffsCMnum"),std::iter::once(x.dmoffsCMnum));
    out.unwritten(&format!("{p}.dmoffsDM_mjd"));
    out.unwritten(&format!("{p}.dmoffsDM"));
    out.unwritten(&format!("{p}.dmoffsDM_error"));
    out.unwritten(&format!("{p}.dmoffsDM_weight"));
    out.unwritten(&format!("{p}.dmoffsCM_mjd"));
    out.unwritten(&format!("{p}.dmoffsCM"));
    out.unwritten(&format!("{p}.dmoffsCM_error"));
    out.unwritten(&format!("{p}.dmoffsCM_weight"));
    out.unwritten(&format!("{p}.gwsrc_ra"));
    out.unwritten(&format!("{p}.gwsrc_dec"));
    out.unwritten(&format!("{p}.gwsrc_aplus_r"));
    out.unwritten(&format!("{p}.gwsrc_aplus_i"));
    out.unwritten(&format!("{p}.gwsrc_across_r"));
    out.unwritten(&format!("{p}.gwsrc_across_i"));
    out.unwritten(&format!("{p}.gwsrc_aplus_r_e"));
    out.unwritten(&format!("{p}.gwsrc_aplus_i_e"));
    out.unwritten(&format!("{p}.gwsrc_across_r_e"));
    out.unwritten(&format!("{p}.gwsrc_across_i_e"));
    out.unwritten(&format!("{p}.gwsrc_epoch"));
    out.unwritten(&format!("{p}.gwsrc_psrdist"));
    out.unwritten(&format!("{p}.cgw_h0"));
    out.unwritten(&format!("{p}.cgw_cosinc"));
    out.unwritten(&format!("{p}.cgw_angpol"));
    out.values(&format!("{p}.cgw_mc"),std::iter::once(x.cgw_mc));
    out.unwritten(&format!("{p}.gwm_raj"));
    out.unwritten(&format!("{p}.gwm_decj"));
    out.unwritten(&format!("{p}.gwm_epoch"));
    out.unwritten(&format!("{p}.gwm_phi"));
    out.unwritten(&format!("{p}.gwm_dphase"));
    out.unwritten(&format!("{p}.gwcs_raj"));
    out.unwritten(&format!("{p}.gwcs_decj"));
    out.unwritten(&format!("{p}.gwcs_epoch"));
    out.unwritten(&format!("{p}.gwcs_width"));
    out.unwritten(&format!("{p}.gwcs_geom_p"));
    out.unwritten(&format!("{p}.gwcs_geom_c"));
    out.unwritten(&format!("{p}.gwb_epoch"));
    out.unwritten(&format!("{p}.gwb_width"));
    out.unwritten(&format!("{p}.gwb_raj"));
    out.unwritten(&format!("{p}.gwb_decj"));
    out.unwritten(&format!("{p}.gwb_geom_c"));
    out.unwritten(&format!("{p}.gwb_geom_p"));
    out.unwritten(&format!("{p}.gwecc_ra"));
    out.unwritten(&format!("{p}.gwecc_dec"));
    out.unwritten(&format!("{p}.gwecc_m1"));
    out.unwritten(&format!("{p}.gwecc_m2"));
    out.unwritten(&format!("{p}.gwecc_e"));
    out.unwritten(&format!("{p}.gwecc_inc"));
    out.unwritten(&format!("{p}.gwecc_theta_nodes"));
    out.unwritten(&format!("{p}.gwecc_nodes_orientation"));
    out.unwritten(&format!("{p}.gwecc_theta_0"));
    out.unwritten(&format!("{p}.gwecc_orbital_period"));
    out.unwritten(&format!("{p}.gwecc_distance"));
    out.unwritten(&format!("{p}.gwecc_redshift"));
    out.unwritten(&format!("{p}.gwecc_epoch"));
    out.unwritten(&format!("{p}.gwecc_psrdist"));
    out.unwritten(&format!("{p}.gwecc_pulsarTermOn"));
    out.unwritten(&format!("{p}.posPulsar"));
    out.unwritten(&format!("{p}.velPulsar"));
    out.unwritten(&format!("{p}.accPulsar"));
    out.unwritten(&format!("{p}.phaseJump"));
    out.unwritten(&format!("{p}.phaseJumpDir"));
    out.unwritten(&format!("{p}.phaseJumpID"));
    out.values(&format!("{p}.nPhaseJump"),std::iter::once(x.nPhaseJump));
    out.values(&format!("{p}.dmOffset"),std::iter::once(x.dmOffset));
    out.values(&format!("{p}.ne_sw"),std::iter::once(x.ne_sw));
    out.values(&format!("{p}.nCompanion"),std::iter::once(x.nCompanion));
    out.values(&format!("{p}.eclCoord"),std::iter::once(x.eclCoord));
    out.values(&format!("{p}.nJumps"),std::iter::once(x.nJumps));
    out.prefix(&format!("{p}.fjumpID"),&x.fjumpID[..]);
    out.values(&format!("{p}.jumpVal"),x.jumpVal.iter().copied());
    out.values(&format!("{p}.jumpSAT"),x.jumpSAT.iter().copied());
    out.unwritten(&format!("{p}.fitJump"));
    out.values(&format!("{p}.jumpValErr"),x.jumpValErr.iter().copied());
    out.unwritten(&format!("{p}.jumpStr"));
    out.values(&format!("{p}.jumpScaled"),x.jumpScaled.iter().copied());
    out.values(&format!("{p}.nfdJumps"),std::iter::once(x.nfdJumps));
    out.values(&format!("{p}.fdjump_pint_format"),std::iter::once(x.fdjump_pint_format));
    out.values(&format!("{p}.fdjumpVal"),x.fdjumpVal.iter().copied());
    out.values(&format!("{p}.fdjumpIdx"),x.fdjumpIdx.iter().copied());
    out.unwritten(&format!("{p}.fitfdJump"));
    out.values(&format!("{p}.fdjumpValErr"),x.fdjumpValErr.iter().copied());
    out.unwritten(&format!("{p}.fdjumpStr"));
    out.values(&format!("{p}.fdjump_log"),std::iter::once(x.fdjump_log));
    out.prefix(&format!("{p}.filterStr"),&x.filterStr[..]);
    out.prefix(&format!("{p}.passStr"),&x.passStr[..]);
    out.unwritten(&format!("{p}.tOffset"));
    out.unwritten(&format!("{p}.tOffset_f1"));
    out.unwritten(&format!("{p}.tOffset_f2"));
    out.unwritten(&format!("{p}.tOffset_t1"));
    out.unwritten(&format!("{p}.tOffset_t2"));
    out.unwritten(&format!("{p}.tOffsetSite"));
    out.unwritten(&format!("{p}.tOffsetFlags"));
    out.values(&format!("{p}.nToffset"),std::iter::once(x.nToffset));
    out.values(&format!("{p}.ndmx"),std::iter::once(x.ndmx));
    out.values(&format!("{p}.nSx"),std::iter::once(x.nSx));
    out.unwritten(&format!("{p}.nchromx"));
    out.unwritten(&format!("{p}.fitChisq"));
    out.unwritten(&format!("{p}.fitNfree"));
    out.unwritten(&format!("{p}.globalNfit"));
    out.unwritten(&format!("{p}.globalNoConstrain"));
    out.unwritten(&format!("{p}.nFit"));
    out.unwritten(&format!("{p}.nParam"));
    out.unwritten(&format!("{p}.nGlobal"));
    out.values(&format!("{p}.fitMode"),std::iter::once(x.fitMode));
    out.values(&format!("{p}.robust"),std::iter::once(x.robust));
    out.values(&format!("{p}.rescaleErrChisq"),std::iter::once(x.rescaleErrChisq));
    out.unwritten(&format!("{p}.offset"));
    out.unwritten(&format!("{p}.offset_e"));
    match &x.covar {None=>out.unwritten(&format!("{p}.covar")),Some(None)=>out.symbol(&format!("{p}.covar"),"null"),Some(Some(v))=>out.symbol(&format!("{p}.covar"),&tags.matrix(v))};
    out.values(&format!("{p}.calcShapiro"),std::iter::once(x.calcShapiro));
    out.values(&format!("{p}.planetShapiro"),std::iter::once(x.planetShapiro));
    out.values(&format!("{p}.jboFormat"),std::iter::once(x.jboFormat));
    match &x.obsn {None=>out.unwritten(&format!("{p}.obsn")),Some(None)=>out.symbol(&format!("{p}.obsn"),"null"),Some(Some(v))=>out.symbol(&format!("{p}.obsn"),&tags.observation(v))};
    out.values(&format!("{p}.nobs"),std::iter::once(x.nobs));
    out.values(&format!("{p}.units"),std::iter::once(x.units));
    out.values(&format!("{p}.setUnits"),std::iter::once(x.setUnits));
    out.unwritten(&format!("{p}.tempo1"));
    out.values(&format!("{p}.dilateFreq"),std::iter::once(x.dilateFreq));
    out.values(&format!("{p}.timeEphemeris"),std::iter::once(x.timeEphemeris));
    out.values(&format!("{p}.t2cMethod"),std::iter::once(x.t2cMethod));
    out.values(&format!("{p}.correctTroposphere"),std::iter::once(x.correctTroposphere));
    out.values(&format!("{p}.noWarnings"),std::iter::once(x.noWarnings));
    out.values(&format!("{p}.sorted"),std::iter::once(x.sorted));
    out.unwritten(&format!("{p}.clock"));
    out.prefix(&format!("{p}.clockFromOverride"),&x.clockFromOverride[..]);
    out.unwritten(&format!("{p}.JPL_EPHEMERIS"));
    out.unwritten(&format!("{p}.ephemeris"));
    out.values(&format!("{p}.useCalceph"),std::iter::once(x.useCalceph));
    out.unwritten(&format!("{p}.storePrec"));
    out.values(&format!("{p}.nStorePrecision"),std::iter::once(x.nStorePrecision));
    out.values(&format!("{p}.bootStrap"),std::iter::once(x.bootStrap));
    out.prefix(&format!("{p}.tzrsite"),&x.tzrsite[..]);
    out.unwritten(&format!("{p}.rmsPre"));
    out.unwritten(&format!("{p}.rmsPost"));
    out.unwritten(&format!("{p}.rmstn"));
    out.prefix(&format!("{p}.deleteFileName"),&x.deleteFileName[..]);
    out.values(&format!("{p}.nits"),std::iter::once(x.nits));
    out.values(&format!("{p}.ipm"),std::iter::once(x.ipm));
    out.values(&format!("{p}.swm"),std::iter::once(x.swm));
    out.unwritten(&format!("{p}.wave_sine"));
    out.unwritten(&format!("{p}.wave_sine_err"));
    out.unwritten(&format!("{p}.wave_cos"));
    out.unwritten(&format!("{p}.wave_cos_err"));
    out.unwritten(&format!("{p}.wave_sine_dm"));
    out.unwritten(&format!("{p}.wave_sine_dm_err"));
    out.unwritten(&format!("{p}.wave_cos_dm"));
    out.unwritten(&format!("{p}.wave_cos_dm_err"));
    out.values(&format!("{p}.nWhite"),std::iter::once(x.nWhite));
    out.unwritten(&format!("{p}.nWhite_dm"));
    out.unwritten(&format!("{p}.waveScale"));
    out.unwritten(&format!("{p}.quad_aplus_r"));
    out.unwritten(&format!("{p}.quad_aplus_r_e"));
    out.unwritten(&format!("{p}.quad_aplus_i"));
    out.unwritten(&format!("{p}.quad_aplus_i_e"));
    out.unwritten(&format!("{p}.quad_across_r"));
    out.unwritten(&format!("{p}.quad_across_r_e"));
    out.unwritten(&format!("{p}.quad_across_i"));
    out.unwritten(&format!("{p}.quad_across_i_e"));
    out.unwritten(&format!("{p}.quadEpoch"));
    out.unwritten(&format!("{p}.quadRA"));
    out.unwritten(&format!("{p}.quadDEC"));
    out.values(&format!("{p}.nQuad"),std::iter::once(x.nQuad));
    out.unwritten(&format!("{p}.ifuncT"));
    out.unwritten(&format!("{p}.ifuncV"));
    out.unwritten(&format!("{p}.ifuncE"));
    out.unwritten(&format!("{p}.ifunc_weights"));
    out.values(&format!("{p}.ifuncN"),std::iter::once(x.ifuncN));
    out.unwritten(&format!("{p}.orbifuncT"));
    out.unwritten(&format!("{p}.orbifuncV"));
    out.unwritten(&format!("{p}.orbifuncE"));
    out.values(&format!("{p}.orbifuncN"),std::iter::once(x.orbifuncN));
    out.unwritten(&format!("{p}.clk_offsT"));
    out.unwritten(&format!("{p}.clk_offsV"));
    out.unwritten(&format!("{p}.clk_offsE"));
    out.values(&format!("{p}.clkOffsN"),std::iter::once(x.clkOffsN));
    out.unwritten(&format!("{p}.quad_ifuncT_p"));
    out.unwritten(&format!("{p}.quad_ifuncV_p"));
    out.unwritten(&format!("{p}.quad_ifuncE_p"));
    out.values(&format!("{p}.quad_ifuncN_p"),std::iter::once(x.quad_ifuncN_p));
    out.unwritten(&format!("{p}.quad_ifuncT_c"));
    out.unwritten(&format!("{p}.quad_ifuncV_c"));
    out.unwritten(&format!("{p}.quad_ifuncE_c"));
    out.values(&format!("{p}.quad_ifuncN_c"),std::iter::once(x.quad_ifuncN_c));
    out.unwritten(&format!("{p}.quad_ifunc_p_RA"));
    out.unwritten(&format!("{p}.quad_ifunc_p_DEC"));
    out.unwritten(&format!("{p}.quad_ifunc_c_RA"));
    out.unwritten(&format!("{p}.quad_ifunc_c_DEC"));
    out.values(&format!("{p}.quad_ifunc_geom_p"),std::iter::once(x.quad_ifunc_geom_p));
    out.values(&format!("{p}.quad_ifunc_geom_c"),std::iter::once(x.quad_ifunc_geom_c));
    out.values(&format!("{p}.nTelDX"),std::iter::once(x.nTelDX));
    out.unwritten(&format!("{p}.setTelVelX"));
    out.unwritten(&format!("{p}.telDX_t"));
    out.unwritten(&format!("{p}.telDX_v"));
    out.unwritten(&format!("{p}.telDX_e"));
    out.unwritten(&format!("{p}.telDX_vel"));
    out.unwritten(&format!("{p}.telDX_vel_e"));
    out.values(&format!("{p}.nTelDY"),std::iter::once(x.nTelDY));
    out.unwritten(&format!("{p}.setTelVelY"));
    out.unwritten(&format!("{p}.telDY_t"));
    out.unwritten(&format!("{p}.telDY_v"));
    out.unwritten(&format!("{p}.telDY_e"));
    out.unwritten(&format!("{p}.telDY_vel"));
    out.unwritten(&format!("{p}.telDY_vel_e"));
    out.values(&format!("{p}.nTelDZ"),std::iter::once(x.nTelDZ));
    out.unwritten(&format!("{p}.setTelVelZ"));
    out.unwritten(&format!("{p}.telDZ_v"));
    out.unwritten(&format!("{p}.telDZ_t"));
    out.unwritten(&format!("{p}.telDZ_e"));
    out.unwritten(&format!("{p}.telDZ_vel"));
    out.unwritten(&format!("{p}.telDZ_vel_e"));
    out.values(&format!("{p}.nT2efac"),std::iter::once(x.nT2efac));
    out.values(&format!("{p}.nT2equad"),std::iter::once(x.nT2equad));
    out.unwritten(&format!("{p}.T2efacFlagID"));
    out.unwritten(&format!("{p}.T2efacFlagVal"));
    out.unwritten(&format!("{p}.T2efacVal"));
    out.unwritten(&format!("{p}.T2equadFlagID"));
    out.unwritten(&format!("{p}.T2equadFlagVal"));
    out.unwritten(&format!("{p}.T2equadVal"));
    out.values(&format!("{p}.T2globalEfac"),std::iter::once(x.T2globalEfac));
    out.values(&format!("{p}.nTNEF"),std::iter::once(x.nTNEF));
    out.values(&format!("{p}.nTNEQ"),std::iter::once(x.nTNEQ));
    out.values(&format!("{p}.nTNSQ"),std::iter::once(x.nTNSQ));
    out.values(&format!("{p}.nTNECORR"),std::iter::once(x.nTNECORR));
    out.values(&format!("{p}.nTNSECORR"),std::iter::once(x.nTNSECORR));
    out.unwritten(&format!("{p}.TNEFFlagID"));
    out.unwritten(&format!("{p}.TNEFFlagVal"));
    out.unwritten(&format!("{p}.TNEFVal"));
    out.values(&format!("{p}.TNGlobalEF"),std::iter::once(x.TNGlobalEF));
    out.unwritten(&format!("{p}.TNEQFlagID"));
    out.unwritten(&format!("{p}.TNEQFlagVal"));
    out.unwritten(&format!("{p}.TNEQVal"));
    out.values(&format!("{p}.TNGlobalEQ"),std::iter::once(x.TNGlobalEQ));
    out.unwritten(&format!("{p}.addTNGlobalEQ"));
    out.unwritten(&format!("{p}.TNSQFlagID"));
    out.unwritten(&format!("{p}.TNSQFlagVal"));
    out.unwritten(&format!("{p}.TNSQVal"));
    out.unwritten(&format!("{p}.TNECORRFlagID"));
    out.unwritten(&format!("{p}.TNECORRFlagVal"));
    out.unwritten(&format!("{p}.TNECORRVal"));
    out.unwritten(&format!("{p}.TNSECORRFlagID"));
    out.unwritten(&format!("{p}.TNSECORRFlagVal"));
    out.unwritten(&format!("{p}.TNSECORRVal"));
    out.values(&format!("{p}.TNRedAmp"),std::iter::once(x.TNRedAmp));
    out.values(&format!("{p}.TNRedGam"),std::iter::once(x.TNRedGam));
    out.values(&format!("{p}.TNRedAmp2"),std::iter::once(x.TNRedAmp2));
    out.values(&format!("{p}.TNRedGam2"),std::iter::once(x.TNRedGam2));
    out.values(&format!("{p}.TNRedC"),std::iter::once(x.TNRedC));
    out.unwritten(&format!("{p}.TNRedCoeffs"));
    out.values(&format!("{p}.TNRedFLow"),std::iter::once(x.TNRedFLow));
    out.values(&format!("{p}.TNRedCorner"),std::iter::once(x.TNRedCorner));
    out.values(&format!("{p}.TNDMAmp"),std::iter::once(x.TNDMAmp));
    out.values(&format!("{p}.TNDMGam"),std::iter::once(x.TNDMGam));
    out.values(&format!("{p}.TNDMC"),std::iter::once(x.TNDMC));
    out.unwritten(&format!("{p}.TNDMCoeffs"));
    out.values(&format!("{p}.SWGPAmp"),std::iter::once(x.SWGPAmp));
    out.values(&format!("{p}.SWGPGam"),std::iter::once(x.SWGPGam));
    out.values(&format!("{p}.SWGPC"),std::iter::once(x.SWGPC));
    out.unwritten(&format!("{p}.SWGPCoeffs"));
    out.values(&format!("{p}.TNChromAmp"),std::iter::once(x.TNChromAmp));
    out.values(&format!("{p}.TNChromGam"),std::iter::once(x.TNChromGam));
    out.values(&format!("{p}.TNChromIdx"),std::iter::once(x.TNChromIdx));
    out.values(&format!("{p}.TNChromC"),std::iter::once(x.TNChromC));
    out.unwritten(&format!("{p}.TNChromCoeffs"));
    out.values(&format!("{p}.TNsubtractDM"),std::iter::once(x.TNsubtractDM));
    out.values(&format!("{p}.TNsubtractRed"),std::iter::once(x.TNsubtractRed));
    out.values(&format!("{p}.TNsubtractChrom"),std::iter::once(x.TNsubtractChrom));
    out.values(&format!("{p}.TNsubtractSWGP"),std::iter::once(x.TNsubtractSWGP));
    out.values(&format!("{p}.AverageResiduals"),std::iter::once(x.AverageResiduals));
    out.unwritten(&format!("{p}.AverageDMResiduals"));
    out.unwritten(&format!("{p}.AverageFlag"));
    out.unwritten(&format!("{p}.AverageEpochWidth"));
    out.values(&format!("{p}.detL"),std::iter::once(x.detL));
    out.unwritten(&format!("{p}.outputTMatrix"));
    out.values(&format!("{p}.useTNOrth"),std::iter::once(x.useTNOrth));
    out.values(&format!("{p}.TNBandDMAmp"),std::iter::once(x.TNBandDMAmp));
    out.values(&format!("{p}.TNBandDMGam"),std::iter::once(x.TNBandDMGam));
    out.values(&format!("{p}.TNBandDMC"),std::iter::once(x.TNBandDMC));
    out.values(&format!("{p}.nTNBandNoise"),std::iter::once(x.nTNBandNoise));
    out.unwritten(&format!("{p}.TNBandNoiseLF"));
    out.unwritten(&format!("{p}.TNBandNoiseHF"));
    out.unwritten(&format!("{p}.TNBandNoiseAmp"));
    out.unwritten(&format!("{p}.TNBandNoiseGam"));
    out.unwritten(&format!("{p}.TNBandNoiseC"));
    out.values(&format!("{p}.nTNGroupNoise"),std::iter::once(x.nTNGroupNoise));
    out.unwritten(&format!("{p}.TNGroupNoiseFlagID"));
    out.unwritten(&format!("{p}.TNGroupNoiseFlagVal"));
    out.unwritten(&format!("{p}.TNGroupNoiseAmp"));
    out.unwritten(&format!("{p}.TNGroupNoiseGam"));
    out.unwritten(&format!("{p}.TNGroupNoiseC"));
    out.values(&format!("{p}.nDMEvents"),std::iter::once(x.nDMEvents));
    out.unwritten(&format!("{p}.nDMEventC"));
    out.unwritten(&format!("{p}.TNDMEvStart"));
    out.unwritten(&format!("{p}.TNDMEvLength"));
    out.unwritten(&format!("{p}.TNDMEvAmp"));
    out.unwritten(&format!("{p}.TNDMEvGam"));
    out.unwritten(&format!("{p}.TNDMEvOff"));
    out.unwritten(&format!("{p}.TNDMEvLin"));
    out.unwritten(&format!("{p}.TNDMEvQuad"));
    out.values(&format!("{p}.nTNShapeletEvents"),std::iter::once(x.nTNShapeletEvents));
    out.unwritten(&format!("{p}.TNShapeletEvN"));
    out.unwritten(&format!("{p}.TNShapeletEvPos"));
    out.unwritten(&format!("{p}.TNShapeletEvWidth"));
    out.unwritten(&format!("{p}.TNShapeletEvFScale"));
    out.unwritten(&format!("{p}.TNShapeletEvCoef"));
    out.unwritten(&format!("{p}.TNShapeletEvCoefErr"));
    out.unwritten(&format!("{p}.TNShapeletEvFitFlag"));
    out.prefix(&format!("{p}.whiteNoiseModelFile"),&x.whiteNoiseModelFile[..]);
    out.unwritten(&format!("{p}.rasim"));
    out.unwritten(&format!("{p}.decsim"));
    out.values(&format!("{p}.simflag"),std::iter::once(x.simflag));
    out.prefix(&format!("{p}.fitFunc"),&x.fitFunc[..]);
    out.values(&format!("{p}.nconstraints"),std::iter::once(x.nconstraints));
    out.values(&format!("{p}.constraint_efactor"),std::iter::once(x.constraint_efactor));
    out.unwritten(&format!("{p}.constraints"));
    out.values(&format!("{p}.auto_constraints"),std::iter::once(x.auto_constraints));
    out.pointers(&format!("{p}.constraint_special"),x.constraint_special.iter().map(|v|v.as_ref().map(|v|v.as_ref().map_or_else(||"null".into(),|v|tags.payload(v)))));
    out.unwritten(&format!("{p}.fitinfo"));
    out.unwritten(&format!("{p}.brace"));
    out.unwritten(&format!("{p}.tzrobs"));
    out.values(&format!("{p}.refphs"),std::iter::once(x.refphs));
    out.unwritten(&format!("{p}.posPulsarEquatorial"));
    out.values(&format!("{p}.dm_series_type"),std::iter::once(x.dm_series_type.map(|n|n as i32)));
    out.values(&format!("{p}.TN_QpPeriod"),std::iter::once(x.TN_QpPeriod));
    out.values(&format!("{p}.TN_QpSig"),std::iter::once(x.TN_QpSig));
    out.values(&format!("{p}.TN_QpLam"),std::iter::once(x.TN_QpLam));
    out.values(&format!("{p}.TN_QpRatio"),std::iter::once(x.TN_QpRatio));
    out.values(&format!("{p}.TNRed_log_freqs"),std::iter::once(x.TNRed_log_freqs));
    out.values(&format!("{p}.TNRed_log_factor"),std::iter::once(x.TNRed_log_factor));
    out.unwritten(&format!("{p}.ne_sw_ifuncT"));
    out.unwritten(&format!("{p}.ne_sw_ifuncV"));
    out.unwritten(&format!("{p}.ne_sw_ifuncE"));
    out.unwritten(&format!("{p}.ne_sw_ifuncN"));
    out.values(&format!("{p}.TNDM_log_freqs"),std::iter::once(x.TNDM_log_freqs));
    out.values(&format!("{p}.TNDM_log_factor"),std::iter::once(x.TNDM_log_factor));
    out.values(&format!("{p}.TNChrom_log_freqs"),std::iter::once(x.TNChrom_log_freqs));
    out.values(&format!("{p}.TNChrom_log_factor"),std::iter::once(x.TNChrom_log_factor));
    out.values(&format!("{p}.TNsubtractPoly"),std::iter::once(x.TNsubtractPoly));
}
