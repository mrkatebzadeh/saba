use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .join("protos");

    let protos: Vec<_> = ["init.proto", "register.proto", "connection.proto"]
        .into_iter()
        .map(|file| proto_root.join(file))
        .collect();

    let proto_refs: Vec<_> = protos.iter().map(|path| path.as_path()).collect();

    let include_dirs = [proto_root.as_path()];
    tonic_prost_build::configure().compile_protos(&proto_refs, &include_dirs)?;

    Ok(())
}
