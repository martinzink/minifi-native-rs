use std::env;
use std::path::PathBuf;

fn main() {
    #[cfg(windows)]
    {
        use std::process::Command;

        let tool = cc::Build::new()
            .try_get_compiler()
            .expect("Failed to find the MSVC toolchain. Is Visual Studio or the C++ Build Tools workload installed?");

        let lib_exe_path = tool
            .path()
            .parent()
            .expect("Compiler path is expected to be in a 'bin' directory.")
            .join("lib.exe");

        if !lib_exe_path.exists() {
            panic!(
                "Could not find lib.exe at the expected path: {}",
                lib_exe_path.display()
            );
        }

        let def_file = PathBuf::from("../minifi-cpp/minifi-api/minifi-c-api.def");

        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let lib_out_path = out_dir.join("minifi-c-api.lib");

        let status = Command::new(&lib_exe_path)
            .arg(format!("/def:{}", def_file.display()))
            .arg(format!("/out:{}", lib_out_path.display()))
            .arg(format!(
                "/machine:{}",
                env::var("CARGO_CFG_TARGET_ARCH").unwrap().to_uppercase()
            ))
            .status()
            .expect("Failed to execute lib.exe.");

        if !status.success() {
            panic!("lib.exe failed to create the import library from the .def file.");
        }

        println!("cargo:rustc-link-lib=dylib=minifi-c-api");
        println!("cargo:rustc-link-search=native={}", out_dir.display());
    }

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
