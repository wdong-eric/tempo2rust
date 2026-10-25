//! readJBO_bat.C byte decoding and helper semantics at fda6ad4.
use crate::clock_text::{NumberFormat as F, c_bytes, format_number as num};
use crate::{
    PRECISION_BITS, TimingError,
    bats::{ConsumedInput, consume},
    observation::{SourceObservation, write_string},
    pulsar::SourcePulsar,
};
use rug::Float;
use std::{os::unix::ffi::OsStrExt, path::Path};
pub fn swap4(raw: &mut [u8; 4]) {
    raw.reverse();
}
pub fn swap8(raw: &mut [u8; 8]) {
    raw.reverse();
}
pub fn swap8_double(value: &mut f64) {
    *value = f64::from_bits(value.to_bits().swap_bytes());
}
pub fn makechars(raw: &[u8; 200]) -> [[u8; 9]; 25] {
    std::array::from_fn(|i| {
        let mut out = [0; 9];
        out[..8].copy_from_slice(&raw[i * 8..i * 8 + 8]);
        out
    })
}
pub fn date2mjd(date: i32) -> Result<f64, TimingError> {
    let year = date / 10000 + 1900;
    let month = (date / 100) % 100;
    let day = date % 100;
    if !(0..=13).contains(&month) {
        return Err(crate::clock_diagnostics::boundary("JBO_month_index"));
    }
    let mut days = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut n = (year as f64 * 365.25).trunc();
    if year % 4 == 0 {
        days[2] = 29;
        n -= 1.0;
    }
    for i in 1..month {
        n += days[i as usize] as f64;
    }
    Ok(n + day as f64 - 678956.0)
}
pub fn read_jbo_bat(
    path: &Path,
    psr: &mut SourcePulsar,
    rows: &mut Vec<SourceObservation>,
    inputs: &mut Vec<ConsumedInput>,
) -> Result<(), TimingError> {
    crate::clkcorr::with_process(|session| {
        let diagnostics = &mut session.diagnostics;
        diagnostics.version("readJBO_bat.C", "readJBO_bat()");
        diagnostics.stdout(
            &[
                b"Attempting to read ".as_slice(),
                path.as_os_str().as_bytes(),
                b"\n",
            ]
            .concat(),
        );
        let s = consume(path, "JBO BAT", inputs)?;
        if s.bytes.is_empty() || s.bytes.len() % 212 != 0 {
            return Err(s.error("JBO record", "incomplete external record"));
        }
        let mut sites: Vec<Vec<u8>> = vec![];
        let mut count = 0;
        let mut raw = [0; 200];
        let mut kind = 0i32;
        for index in 0..=s.bytes.len() / 212 {
            if let Some(record) = s.bytes.chunks_exact(212).nth(index) {
                kind = i32::from_be_bytes(record[4..8].try_into().unwrap());
                raw.copy_from_slice(&record[8..208]);
            } else {
                // Failed fread retains the last (possibly swapped) buffer and type.
                kind = kind.swap_bytes();
            }
            diagnostics.log(
                b'm',
                "readJBO_bat.C",
                74,
                format!("Got record type = {kind} ({})", index + 1).as_bytes(),
            );
            let chars = makechars(&raw);
            match kind {
                0 => diagnostics.log(
                    b'm',
                    "readJBO_bat.C",
                    84,
                    &[
                        b"Source name: ".as_slice(),
                        c_bytes(&chars[0]),
                        c_bytes(&chars[1]),
                        c_bytes(&chars[2]),
                    ]
                    .concat(),
                ),
                3 => diagnostics.log(
                    b'm',
                    "readJBO_bat.C",
                    80,
                    &[
                        b"Read BAT file created by ".as_slice(),
                        c_bytes(&chars[1]),
                        b", ver ",
                        c_bytes(&chars[2]),
                        b", user ",
                        c_bytes(&chars[3]),
                    ]
                    .concat(),
                ),
                1 => {
                    for i in 3..6 {
                        raw[i * 8..i * 8 + 8].reverse();
                    }
                    let coordinates: [Vec<u8>; 3] = std::array::from_fn(|i| {
                        num(
                            f64::from_ne_bytes(raw[(i + 3) * 8..(i + 4) * 8].try_into().unwrap()),
                            F::Six,
                        )
                    });
                    diagnostics.log(
                        b'm',
                        "readJBO_bat.C",
                        91,
                        &[
                            b"Observatory name: ".as_slice(),
                            c_bytes(&chars[0]),
                            c_bytes(&chars[1]),
                            c_bytes(&chars[2]),
                            b" ",
                            &coordinates[0],
                            b" ",
                            &coordinates[1],
                            b" ",
                            &coordinates[2],
                        ]
                        .concat(),
                    );
                    let mut site = raw[..24].to_vec();
                    site.truncate(
                        site.iter()
                            .position(|&b| b == 0 || b == b' ')
                            .unwrap_or(site.len()),
                    );
                    if sites.len() == 256 {
                        return Err(s.error("JBO sites", "capacity"));
                    }
                    sites.push(site);
                }
                2 => {
                    // readJBO_bat.C::swap8 cast a double's storage to int*, violating
                    // aliasing and yielding stale O2 word diagnostics (N report).
                    // Decode/reverse owned bytes instead; only those diagnostic words
                    // change. AC's corrected-native replay replaces the int accesses
                    // with byte swaps and uses declared double storage.
                    let mut d = [0.0; 25];
                    for i in 0..25 {
                        if i == 13 {
                            diagnostics.log(
                                b'm',
                                "readJBO_bat.C",
                                105,
                                &[format!("{i} {} ", i + 1).as_bytes(), c_bytes(&chars[i])]
                                    .concat(),
                            );
                        } else {
                            raw[i * 8..i * 8 + 8].reverse();
                            d[i] = f64::from_ne_bytes(raw[i * 8..i * 8 + 8].try_into().unwrap());
                            diagnostics.log(
                                b'm',
                                "readJBO_bat.C",
                                108,
                                &[format!("{i} {} ", i + 1).as_bytes(), &num(d[i], F::Six)]
                                    .concat(),
                            );
                        }
                    }
                    let index = crate::ifteph::native_i32(d[4], "JBO_site_int")? - 1;
                    let site = sites
                        .get(index as usize)
                        .ok_or_else(|| s.error("JBO site", "unwritten or invalid site index"))?;
                    diagnostics.log(
                        b'm',
                        "readJBO_bat.C",
                        112,
                        &[b"site = ".as_slice(), site].concat(),
                    );
                    let date = crate::ifteph::native_i32(d[0], "JBO_date_int")?;
                    let diff = (date2mjd(crate::ifteph::native_i32(d[23], "JBO_date_int")?)?
                        * 86400.0
                        + d[24])
                        - (date2mjd(date)? * 86400.0 + d[1]);
                    diagnostics.log(
                        b'm',
                        "readJBO_bat.C",
                        117,
                        &[b"sat-bat = ".as_slice(), &num(diff, F::GeneralSix)].concat(),
                    );
                    let bat = Float::with_val(PRECISION_BITS, date2mjd(date)? + d[1] / 86400.0);
                    if rows.len() <= count {
                        rows.resize_with(count + 1, SourceObservation::default);
                    }
                    let corr = d[15] + d[16] + d[17] + d[21];
                    diagnostics.log(
                        b'm',
                        "readJBO_bat.C",
                        122,
                        &[b"corr =  = ".as_slice(), &num(corr, F::Six)].concat(),
                    );
                    let o = &mut rows[count];
                    o.sat = Some(bat.clone());
                    o.bat = Some(bat);
                    diagnostics.log(
                        b'm',
                        "readJBO_bat.C",
                        125,
                        &[
                            b"SAT = ".as_slice(),
                            &num(o.sat.as_ref().unwrap().to_f64(), F::Twelve),
                        ]
                        .concat(),
                    );
                    diagnostics.log(
                        b'm',
                        "readJBO_bat.C",
                        126,
                        &[
                            b"BAT = ".as_slice(),
                            &num(o.bat.as_ref().unwrap().to_f64(), F::Twelve),
                        ]
                        .concat(),
                    );
                    o.toaErr = Some(d[2]);
                    o.origErr = Some(d[2]);
                    o.efac = Some(1.0);
                    o.equad = Some(0.0);
                    o.freq = Some(d[3]);
                    o.deleted = Some(0);
                    o.clockCorr = Some(0);
                    o.delayCorr = Some(0);
                    write_string(&mut o.telID, site, "telID")?;
                    write_string(&mut o.fname, b"UNKNOWN", "fname")?;
                    count += 1;
                }
                _ => (),
            }
        }
        psr.nobs = Some(count as i32);
        Ok(())
    })
}
