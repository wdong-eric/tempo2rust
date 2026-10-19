#[path = "../tests/support/projection.rs"]
mod projection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    projection::produce(
        &root.join("tests/reference"),
        &root.join("target/reference-comparison"),
    )
}
