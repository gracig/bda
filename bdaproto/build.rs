use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let iface_files = &["bda_model.proto", "bda_api.proto"];
    let dirs = &["./proto", "./proto/googleapis"];

    let descriptor_path = PathBuf::from("proto_descriptor.bin");

    tonic_build::configure()
        .file_descriptor_set_path(&descriptor_path)
        .build_server(true)
        .out_dir("src/")
        .compile_well_known_types(true)
        .extern_path(".google.protobuf", "::pbjson_types")
        //.type_attribute(".", "#[derive(PartialOrd)]")
        //.type_attribute(".google.protobuf", "#[derive(PartialOrd)]")
        .compile(iface_files, dirs)?;
    for file in iface_files {
        println!("cargo:rerun-if-changed={}", file);
    }
    let descriptor_set = std::fs::read(&descriptor_path)?;
    pbjson_build::Builder::new()
        .out_dir("src/")
        .register_descriptors(&descriptor_set)?
        .build(&[".bda"])?;
    Ok(())
}
