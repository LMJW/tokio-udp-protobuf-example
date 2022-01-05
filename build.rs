use std::io::Result;
fn main() -> Result<()> {
    let mut config = prost_build::Config::default();

    config.out_dir("src/protogen");

    config.compile_protos(&["proto/echo.proto"], &["proto"])?;

    Ok(())
}