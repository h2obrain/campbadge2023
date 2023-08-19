use std::env;
use std::path::Path;

// Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir).join("../../sled_hijack/lib").display()
    );

    embuild::build::CfgArgs::output_propagated("ESP_IDF")?;
    embuild::build::LinkArgs::output_propagated("ESP_IDF")?;

    let bindings = bindgen::Builder::default()
        .header("../../sled_hijack/include/sled.h")
        .clang_arg("-I../../sled_hijack/include/")
        .clang_arg("-DBUILD_SLED_LIB")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("slede.rs"))
        .expect("Couldn't write bindings!");
    Ok(())
}
