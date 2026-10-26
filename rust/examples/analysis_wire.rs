//! Bidirectional native prefix exchange via the public safe codec.
mod analysis_support;
use analysis_support::*;
use std::{
    fs::File,
    io::{Seek, SeekFrom},
};
use tempo2_rust::gw_file::{self, Profile};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = args()?;
    let input = required(&a, "--input")?;
    let output = required(&a, "--output")?;
    let profile = Profile::LittleEndianI32Binary128;
    for kind in ["tensor", "general"] {
        let mut source = File::open(format!("{input}.{kind}.bin"))?;
        let mut dest = File::create(format!("{output}.{kind}.bin"))?;
        for id in [2, 9] {
            source.seek(SeekFrom::Start(0))?;
            let prefixes = if kind == "tensor" {
                let mut rows = vec![];
                let n = gw_file::read_tensor(&mut source, id, &mut rows, profile)?;
                gw_file::write_tensor(&mut dest, id, &rows[..n], profile)?;
                rows.iter()
                    .map(|r| {
                        r.prefix()
                            .map(|v| u128::from_le_bytes(gw_file::encode_binary128(v)))
                            .to_vec()
                    })
                    .collect::<Vec<_>>()
            } else {
                let mut rows = vec![];
                let n = gw_file::read_general(&mut source, id, &mut rows, profile)?;
                gw_file::write_general(&mut dest, id, &rows[..n], profile)?;
                rows.iter()
                    .map(|r| {
                        r.prefix()
                            .map(|v| u128::from_le_bytes(gw_file::encode_binary128(v)))
                            .to_vec()
                    })
                    .collect::<Vec<_>>()
            };
            emit(&format!("{kind}.{id}.count"), "exact", prefixes.len());
            emit(
                &format!("{kind}.{id}.cursor"),
                "exact",
                source.stream_position()?,
            );
            for (i, row) in prefixes.iter().enumerate() {
                for (j, v) in row.iter().enumerate() {
                    emit(&format!("{kind}.{id}.{i}.{j}"), "bits", v);
                }
            }
        }
    }
    Ok(())
}
