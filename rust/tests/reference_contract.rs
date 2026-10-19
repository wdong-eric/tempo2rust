#[path = "support/projection.rs"]
mod projection;
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};

#[test]
fn accepted_reference_projection_and_provenance() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let reference = root.join("tests/reference");
    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(reference.join("provenance.json")).unwrap()).unwrap();
    assert_eq!(
        manifest["reference_revision"],
        "7795d53eeca32ef3dbb61c2b6cdb1c3e7c5d3572"
    );
    for (file, record) in manifest["files"].as_object().unwrap() {
        assert_eq!(
            format!(
                "{:x}",
                Sha256::digest(fs::read(reference.join(file)).unwrap())
            ),
            record["sha256"].as_str().unwrap(),
            "{file}"
        );
    }
    let output = root.join("target/reference-comparison");
    projection::produce(&reference, &output).unwrap();
    for fixture in ["barycentric", "barycentric_4ns", "long_span"] {
        let result = Command::new("python3")
            .arg(reference.join("compare.py"))
            .arg(reference.join(format!("outputs/{fixture}.json")))
            .arg(output.join(format!("{fixture}.json")))
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{fixture}: {} {}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
    }
}
