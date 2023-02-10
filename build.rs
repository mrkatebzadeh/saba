fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("protos/init.proto")?;
    tonic_build::compile_protos("protos/register.proto")?;
    Ok(())
}
