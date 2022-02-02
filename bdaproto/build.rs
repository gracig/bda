fn main() -> Result<(), Box<dyn std::error::Error>> {
    let iface_files = &["bda_model.proto", "bda_api.proto"];
    let dirs = &["./proto", "./proto/googleapis"];
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/")
        .compile(iface_files, dirs)?;
    for file in iface_files {
        println!("cargo:rerun-if-changed={}", file);
    }
    Ok(())
}
