use std::path::PathBuf;

fn main() {
    const LIB_NAME: &str = "mina-decoder";

    let lib_file = if cfg!(feature = "static") {
        format!("lib{LIB_NAME}.a")
    } else {
        format!("lib{LIB_NAME}.so")
    };
    println!("using {lib_file}...");

    let src_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("lib");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let from = src_dir.join(&lib_file);
    let to = PathBuf::from(&out_dir).join(&lib_file);
    if let Err(e) = std::fs::copy(&from, &to) {
        panic!("failed to copy library from {from:?} to {to:?}: {e}");
    }
    println!("cargo:rustc-link-search={out_dir}");
    println!("cargo:rustc-link-lib={LIB_NAME}");
}
