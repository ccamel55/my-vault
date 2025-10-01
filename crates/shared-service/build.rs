use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const WORKSPACE_DIR: &str = env!("WORKSPACE_DIR");

    let include_path = PathBuf::from(WORKSPACE_DIR).join("protobufs");
    let descriptor_path =
        PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("shared-service-descriptors.bin");

    // We have to manually include the protobuf files we want to use.
    // todo: search through directory and add all protobuf files automatically
    let files = vec![
        include_path.join("client.proto"),
        include_path.join("user.proto"),
    ];

    // Create rust code
    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .build_transport(false)
        .emit_rerun_if_changed(true)
        .file_descriptor_set_path(descriptor_path)
        .compile_protos(&files, &[include_path])?;

    Ok(())
}
