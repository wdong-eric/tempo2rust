//! The ten packed-polynomial helpers from t2fit_nestlike.h; no noise models.
#![allow(non_snake_case)]
use crate::pulsar_lifecycle::{LifecycleBoundary, fail};
pub const T2_TNSUBPOLY_FLAG_POSTFIT: u64 = 1;
pub const T2_TNSUBPOLY_FLAG_NESTLIKE: u64 = 2;
pub fn t2ClampSubPolyOrder(order: i32) -> i32 {
    order.clamp(0, 3)
}
pub fn t2DecodeTNSubPolyOrdStored(packed: u64, shift: i32) -> Result<i32, LifecycleBoundary> {
    if !(0..64).contains(&shift) {
        return Err(fail("t2DecodeTNSubPolyOrdStored", "shift", "invalid_shift"));
    }
    Ok(t2ClampSubPolyOrder(((packed >> shift) & 0xff) as i32))
}
pub fn t2EncodeTNsubtractPoly(flags: u64, red: i32, dm: i32, chrom: i32) -> u64 {
    (flags & 0xff)
        | ((t2ClampSubPolyOrder(red) as u64 & 0xff) << 8)
        | ((t2ClampSubPolyOrder(dm) as u64 & 0xff) << 16)
        | ((t2ClampSubPolyOrder(chrom) as u64 & 0xff) << 24)
}
pub fn t2DecodeTNsubtractPolyFlags(packed: u64) -> u64 {
    packed & 0xff
}
pub fn t2DecodeTNRedSubPolyOrd(packed: u64) -> i32 {
    t2DecodeTNSubPolyOrdStored(packed, 8).unwrap()
}
pub fn t2DecodeTNDMSubPolyOrd(packed: u64) -> i32 {
    t2DecodeTNSubPolyOrdStored(packed, 16).unwrap()
}
pub fn t2DecodeTNChromSubPolyOrd(packed: u64) -> i32 {
    t2DecodeTNSubPolyOrdStored(packed, 24).unwrap()
}
pub fn t2SetTNRedSubPolyOrd(packed: u64, order: i32) -> u64 {
    (packed & !(0xff << 8)) | ((t2ClampSubPolyOrder(order) as u64 & 0xff) << 8)
}
pub fn t2SetTNDMSubPolyOrd(packed: u64, order: i32) -> u64 {
    (packed & !(0xff << 16)) | ((t2ClampSubPolyOrder(order) as u64 & 0xff) << 16)
}
pub fn t2SetTNChromSubPolyOrd(packed: u64, order: i32) -> u64 {
    (packed & !(0xff << 24)) | ((t2ClampSubPolyOrder(order) as u64 & 0xff) << 24)
}
