use cbindgen::Language;

fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = format!("{}/../../php/", crate_dir);
    let header_path = format!("{}/chat_xdk.h", out_dir);

    std::fs::create_dir_all(&out_dir).expect("Failed to create PHP include directory");

    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_language(Language::C)
        .with_no_includes()
        .with_header("#define FFI_LIB \"./libchat_xdk_php.so\"")
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file(&header_path);
}

