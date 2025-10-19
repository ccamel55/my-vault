use std::collections::HashSet;
use std::fmt::Display;
use std::path::{Path, PathBuf};

// Get files recursively
fn get_files_recursive(input_path: &Path, file_extensions: &HashSet<&str>) -> Vec<PathBuf> {
    let mut input_paths = Vec::new();
    let mut found_files = Vec::new();

    input_paths.push(input_path.to_path_buf());

    // Perform non-recursive tree traversal
    while let Some(input_path) = input_paths.pop() {
        for file in std::fs::read_dir(&input_path).expect("could not read input path") {
            let file = file.expect("could not get file").path();
            if file.is_dir() {
                input_paths.push(file.to_path_buf())
            } else if let Some(extension) = file.extension() {
                let extension = extension.to_str().unwrap();
                if file_extensions.contains(extension) {
                    found_files.push(file)
                }
            }
        }
    }

    found_files
}

fn emit_rerun_if_changed<T: Display>(path: T) {
    println!("cargo:rerun-if-changed={path}")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let include_path = PathBuf::from(env!("WORKSPACE_DIR")).join("protobufs");
    let descriptor_path =
        PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("shared-service-descriptors.bin");

    // We have to manually include the protobuf files we want to use.
    // todo: search through directory and add all protobuf files automatically
    let files = get_files_recursive(
        &include_path,
        &HashSet::from_iter(vec!["proto", "protobuf"]),
    );

    // Rerun build.rs if our protobufs folder is updated.
    emit_rerun_if_changed(include_path.display());

    for file in &files {
        emit_rerun_if_changed(file.display());
    }

    // Create rust code
    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .build_transport(false)
        .file_descriptor_set_path(descriptor_path)
        .compile_protos(&files, &[include_path])?;

    Ok(())
}
