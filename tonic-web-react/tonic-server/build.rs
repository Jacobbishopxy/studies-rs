//! file: build.rs
//! author: Jacob Xie
//! date: 2023/08/31 22:03:14 Thursday
//! brief:

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/page.proto")?;
    tonic_build::compile_protos("../proto/helloworld.proto")?;

    Ok(())
}
