//! GWsim.C background file prefixes, safe value codec for the measured profile.
//! Rug object memory is never serialized. Geometry/metadata are not file fields.
use crate::{
    PRECISION_BITS,
    gw_sim::{GeneralSource, TensorSource},
};
use rug::{
    Float,
    float::{Round, Special},
};
use std::io::{self, Read, Seek, Write};
#[derive(Clone, Copy, Debug)]
pub enum Profile {
    LittleEndianI32Binary128,
}

pub fn encode_binary128(value: &Float) -> [u8; 16] {
    let sign = if value.is_sign_negative() {
        1u128 << 127
    } else {
        0
    };
    if value.is_nan() {
        return (sign | (0x7fffu128 << 112) | (1u128 << 111)).to_le_bytes();
    }
    if value.is_infinite() {
        return (sign | (0x7fffu128 << 112)).to_le_bytes();
    }
    let (mut v, round) = Float::with_val_round(113, value, Round::Nearest);
    v.subnormalize_round(-16381, round, Round::Nearest);
    if v.is_zero() {
        return sign.to_le_bytes();
    }
    let exp = v.get_exp().expect("finite nonzero exponent");
    if exp > 16384 {
        return (sign | (0x7fffu128 << 112)).to_le_bytes();
    }
    v.abs_mut();
    let exponent = exp - 1;
    let bits = if exponent < -16382 {
        v <<= 16494;
        v.to_integer().unwrap().to_u128().unwrap()
    } else {
        v <<= 112 - exponent;
        let mantissa = v.to_integer().unwrap().to_u128().unwrap();
        (((exponent + 16383) as u128) << 112) | (mantissa & ((1u128 << 112) - 1))
    };
    (sign | bits).to_le_bytes()
}
pub fn decode_binary128(bytes: [u8; 16]) -> Float {
    let bits = u128::from_le_bytes(bytes);
    let exp = ((bits >> 112) & 0x7fff) as i32;
    let frac = bits & ((1u128 << 112) - 1);
    let mut value = if exp == 0x7fff {
        Float::with_val(
            PRECISION_BITS,
            if frac == 0 {
                Special::Infinity
            } else {
                Special::Nan
            },
        )
    } else {
        let mut v = Float::with_val(
            PRECISION_BITS,
            if exp == 0 {
                frac
            } else {
                frac | (1u128 << 112)
            },
        );
        v <<= if exp == 0 { -16494 } else { exp - 16383 - 112 };
        v
    };
    if bits >> 127 != 0 {
        value = -value;
    }
    value
}
#[derive(Debug)]
pub struct ReadError {
    pub completed: usize,
    pub message: String,
}
impl std::fmt::Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({} complete records)", self.message, self.completed)
    }
}
impl std::error::Error for ReadError {}
fn read_prefixes<R: Read + Seek, const N: usize>(
    stream: &mut R,
    id: i32,
    mut store: impl FnMut(usize, [Float; N]),
) -> Result<usize, ReadError> {
    let mut completed = 0;
    let result = (|| -> io::Result<usize> {
        // 0053, GWsim.C:306–326/1400–1417: unchecked fread/fseek and
        // loop-count-as-realization could return stale ids/counts and partial
        // scientific fields. Check every read, search actual ids, and publish
        // only complete prefixes. Earlier complete rows and cursor survive errors.
        loop {
            let mut bytes = [0; 4];
            stream.read_exact(&mut bytes)?;
            let actual = i32::from_le_bytes(bytes);
            stream.read_exact(&mut bytes)?;
            let count = i32::from_le_bytes(bytes);
            if count < 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "negative background count",
                ));
            }
            if actual == id {
                for i in 0..count as usize {
                    let mut values = std::array::from_fn(|_| Float::new(PRECISION_BITS));
                    for v in &mut values {
                        let mut bytes = [0; 16];
                        stream.read_exact(&mut bytes)?;
                        *v = decode_binary128(bytes);
                    }
                    store(i, values);
                    completed += 1;
                }
                return Ok(count as usize);
            }
            let bytes = count as u64 * N as u64 * 16;
            let copied = io::copy(&mut stream.take(bytes), &mut io::sink())?;
            if copied != bytes {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "truncated skipped realization",
                ));
            }
        }
    })();
    result.map_err(|e| ReadError {
        completed,
        message: format!("GW background realization {id}: {e}"),
    })
}
fn write_prefixes<'a, W: Write, const N: usize>(
    stream: &mut W,
    id: i32,
    values: impl ExactSizeIterator<Item = [&'a Float; N]>,
) -> io::Result<()> {
    let count = i32::try_from(values.len()).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "background count exceeds source int",
        )
    })?;
    stream.write_all(&id.to_le_bytes())?;
    stream.write_all(&count.to_le_bytes())?;
    for v in values {
        for x in v {
            stream.write_all(&encode_binary128(x))?;
        }
    }
    stream.flush()
}
pub fn write_tensor<W: Write>(
    stream: &mut W,
    id: i32,
    rows: &[TensorSource],
    _profile: Profile,
) -> io::Result<()> {
    write_prefixes(stream, id, rows.iter().map(TensorSource::prefix))
}
pub fn write_general<W: Write>(
    stream: &mut W,
    id: i32,
    rows: &[GeneralSource],
    _profile: Profile,
) -> io::Result<()> {
    write_prefixes(stream, id, rows.iter().map(GeneralSource::prefix))
}
pub fn read_tensor<R: Read + Seek>(
    stream: &mut R,
    id: i32,
    rows: &mut Vec<TensorSource>,
    _profile: Profile,
) -> Result<usize, ReadError> {
    read_prefixes(stream, id, |i, v| {
        let mut row = TensorSource::from_prefix(v);
        if i < rows.len() {
            row.geometry = rows[i].geometry.take();
            row.binary_metadata = rows[i].binary_metadata.take();
            rows[i] = row;
        } else {
            rows.push(row);
        }
    })
}
pub fn read_general<R: Read + Seek>(
    stream: &mut R,
    id: i32,
    rows: &mut Vec<GeneralSource>,
    _profile: Profile,
) -> Result<usize, ReadError> {
    read_prefixes(stream, id, |i, v| {
        let mut row = GeneralSource::from_prefix(v);
        if i < rows.len() {
            row.geometry = rows[i].geometry.take();
            row.binary_metadata = rows[i].binary_metadata.take();
            rows[i] = row;
        } else {
            rows.push(row);
        }
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn binary128_classes_subnormal_and_rounding() {
        for bits in [
            0,
            1u128 << 127,
            0x3fff_u128 << 112,
            1,
            1u128 << 112,
            0x7fff_u128 << 112,
            (0x7fff_u128 << 112) | (1u128 << 111),
            (1u128 << 127) | (0x7fff_u128 << 112) | (1u128 << 111),
            (1u128 << 127) | (0x7fff_u128 << 112),
        ] {
            assert_eq!(
                u128::from_le_bytes(encode_binary128(&decode_binary128(bits.to_le_bytes()))),
                bits
            );
        }
        let mut x = Float::with_val(PRECISION_BITS, 1);
        x >>= 16495;
        assert_eq!(u128::from_le_bytes(encode_binary128(&x)), 0);
        x *= 1.5;
        assert_eq!(u128::from_le_bytes(encode_binary128(&x)), 1);
    }
    #[test]
    fn cursor_prefix_and_recovery() {
        let s =
            TensorSource::from_prefix(std::array::from_fn(|i| Float::with_val(PRECISION_BITS, i)));
        let mut bytes = vec![];
        write_tensor(
            &mut bytes,
            3,
            std::slice::from_ref(&s),
            Profile::LittleEndianI32Binary128,
        )
        .unwrap();
        write_tensor(
            &mut bytes,
            9,
            &[s.clone(), s],
            Profile::LittleEndianI32Binary128,
        )
        .unwrap();
        let mut f = io::Cursor::new(bytes.clone());
        let mut rows = vec![];
        assert_eq!(
            read_tensor(&mut f, 3, &mut rows, Profile::LittleEndianI32Binary128).unwrap(),
            1
        );
        assert_eq!(f.position(), 152);
        assert_eq!(
            read_tensor(&mut f, 9, &mut rows, Profile::LittleEndianI32Binary128).unwrap(),
            2
        );
        let complete = bytes.clone();
        bytes.pop();
        let mut f = io::Cursor::new(bytes);
        let e = read_tensor(&mut f, 9, &mut rows, Profile::LittleEndianI32Binary128).unwrap_err();
        assert_eq!(e.completed, 1);
        assert_eq!(rows.len(), 2);
        let mut good = io::Cursor::new(complete);
        assert!(read_tensor(&mut good, 42, &mut rows, Profile::LittleEndianI32Binary128).is_err());
        good.set_position(0);
        rows[0].setup();
        rows[0].binary_metadata = Some(std::array::from_fn(|_| Float::with_val(PRECISION_BITS, 7)));
        assert_eq!(
            read_tensor(&mut good, 3, &mut rows, Profile::LittleEndianI32Binary128).unwrap(),
            1
        );
        assert!(rows[0].geometry.is_some());
        assert_eq!(rows[0].binary_metadata.as_ref().unwrap()[0], 7);
        // Prefix serialization borrows values: Rug's Clone discards NaN signs.
        let nan = decode_binary128(
            ((1u128 << 127) | (0x7fff_u128 << 112) | (1u128 << 111)).to_le_bytes(),
        );
        let g = GeneralSource::from_prefix(std::array::from_fn(|_| {
            -Float::with_val(PRECISION_BITS, Special::Nan)
        }));
        assert!(nan.is_sign_negative());
        let mut wire = Vec::new();
        write_general(&mut wire, 9, &[g], Profile::LittleEndianI32Binary128).unwrap();
        let mut input = io::Cursor::new(wire.clone());
        let mut general = Vec::new();
        assert!(
            read_general(
                &mut input,
                42,
                &mut general,
                Profile::LittleEndianI32Binary128
            )
            .is_err()
        );
        input.set_position(0);
        assert_eq!(
            read_general(
                &mut input,
                9,
                &mut general,
                Profile::LittleEndianI32Binary128
            )
            .unwrap(),
            1
        );
        let mut again = Vec::new();
        write_general(&mut again, 9, &general, Profile::LittleEndianI32Binary128).unwrap();
        assert_eq!(wire, again);
        wire.pop();
        assert_eq!(
            read_general(
                &mut io::Cursor::new(wire),
                9,
                &mut general,
                Profile::LittleEndianI32Binary128
            )
            .unwrap_err()
            .completed,
            0
        );
    }
}
