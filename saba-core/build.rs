use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .join("protos");

    let protos = ["init.proto", "register.proto", "connection.proto"]
        .into_iter()
        .map(|file| proto_root.join(file))
        .collect::<Vec<_>>();

    tonic_build::configure().compile(
        &protos.iter().map(|path| path.as_path()).collect::<Vec<_>>(),
        &[proto_root],
    )?;

    Ok(())
}
