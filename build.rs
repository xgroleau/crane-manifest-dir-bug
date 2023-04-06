use std::path::PathBuf;

/// Build script for the protobuf definition
fn main() {
    let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("wearable");
    let protos = [base_path.join("example.proto").into_os_string()];

    prost_build::Config::new()
        .compile_protos(&protos, &[base_path.clone().into_os_string()])
        .expect("Failed to compile {:?}");

    println!("cargo:rerun-if-changed={}", base_path.to_str().unwrap());
}
