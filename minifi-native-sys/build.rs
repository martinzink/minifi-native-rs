use std::env;
use std::path::PathBuf;

fn main() {
    let header_path = "../minifi-cpp/minifi-api/include/minifi-c/minifi-c.h";

    println!("cargo:rerun-if-changed={}", header_path);

    let bindings = bindgen::Builder::default()
        .header(header_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
