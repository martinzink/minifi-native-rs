use glob::glob;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn get_venv_python(venv_path: &Path) -> PathBuf {
    if cfg!(windows) {
        venv_path.join("Scripts").join("python.exe")
    } else {
        venv_path.join("bin").join("python")
    }
}

fn get_venv_behave(venv_path: &Path) -> PathBuf {
    if cfg!(windows) {
        venv_path.join("Scripts").join("behave.exe")
    } else {
        venv_path.join("bin").join("behavex")
    }
}

fn run_build_in_docker(root_path: &Path) {
    let mut rocky_build = Command::new("./rockybuild.sh");
    let status = rocky_build
        .current_dir(root_path)
        .status()
        .expect("Failed to run behave");
    assert!(status.success());
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let root = Path::new(&manifest_dir);

    #[cfg(target_os = "macos")]
    run_build_in_docker(root);

    let venv_dir = root.join(".venv");

    if !venv_dir.exists() {
        println!("Creating virtual environment at {:?}", venv_dir);
        let status = Command::new("python3")
            .arg("-m")
            .arg("venv")
            .arg(&venv_dir)
            .status()
            .expect("Failed to create venv");

        assert!(status.success(), "Failed to initialize venv");
    }
    println!("Installing MiNiFi behave framework...");
    let python_bin = get_venv_python(&venv_dir);

    let install_status = Command::new(&python_bin)
        .arg("-m")
        .arg("pip")
        .arg("install")
        .arg("-e")
        .arg(root.join("..").join("minifi-cpp").join("behave_framework"))
        .status()
        .expect("Failed to install dependencies");

    assert!(install_status.success(), "Pip install failed");

    let pattern = format!("{}/../**/*.feature", root.display());

    let mut feature_files = Vec::new();
    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if !path.to_string_lossy().contains(".venv")
                    && !path.to_string_lossy().contains("minifi-cpp")
                {
                    println!("Found feature file: {:?}", path);
                    feature_files.push(path);
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }

    if feature_files.is_empty() {
        panic!("No .feature files found in project subdirectories!");
    }

    let mut cmd = Command::new(&get_venv_behave(&venv_dir));

    cmd.args(&feature_files);

    cmd.arg("--show-progress-bar");
    cmd.arg("--parallel-processes");
    cmd.arg("2");

    let status = cmd
        .current_dir(root)
        .status()
        .expect("Failed to run behave");

    assert!(status.success(), "Behave tests failed");
}
