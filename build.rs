use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["src/example.proto"], &["src/"])?;
    println!("cargo:rerun-if-changed=src/example.proto");
    Ok(())
}
