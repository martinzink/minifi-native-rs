use glob::glob;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

struct BehaveRunner {
    venv_path: PathBuf,
    minifi_behave_path: PathBuf,
}

impl BehaveRunner {
    fn new(out_dir: &Path) -> Self {
        let behave_path = std::env::var("MINIFI_BEHAVE_PATH")
            .map(|s| PathBuf::from(s))
            .expect("MINIFI_BEHAVE_PATH is required");
        let venv_dir = out_dir.join(".venv");
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

        Self {
            venv_path: venv_dir,
            minifi_behave_path: behave_path,
        }
    }

    fn get_venv_behave(&self) -> PathBuf {
        if cfg!(windows) {
            self.venv_path.join("Scripts").join("behave.exe")
        } else {
            self.venv_path.join("bin").join("behavex")
        }
    }

    fn get_venv_python(&self) -> PathBuf {
        if cfg!(windows) {
            self.venv_path.join("Scripts").join("python.exe")
        } else {
            self.venv_path.join("bin").join("python")
        }
    }

    fn install_minifi_behave(&self) -> ExitStatus {
        Command::new(&self.get_venv_python())
            .arg("-m")
            .arg("pip")
            .arg("install")
            .arg(self.minifi_behave_path.to_string_lossy().as_ref())
            .status()
            .expect("Failed to install dependencies")
    }

    fn find_features(root: &Path) -> Vec<PathBuf> {
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
        feature_files
    }

    fn run_tests(&self, root: &Path) -> ExitStatus {
        let mut cmd = Command::new(&self.get_venv_behave());

        cmd.args(&Self::find_features(root));

        cmd.arg("--show-progress-bar");
        cmd.arg("--parallel-processes");
        cmd.arg("2");

        cmd.current_dir(root)
            .status()
            .expect("Failed to run behave")
    }
}

#[allow(dead_code)]
fn build_linux_so(root_path: &Path) {
    let mut build_linux = Command::new("./linux_build.sh");
    let status = build_linux
        .current_dir(root_path)
        .status()
        .expect("Failed to run behave");
    assert!(status.success());
}

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let root = Path::new(&manifest_dir);

    #[cfg(target_os = "macos")]
    build_linux_so(root);

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));

    let behave_runner = BehaveRunner::new(&out_dir);
    let install_status = behave_runner.install_minifi_behave();

    assert!(install_status.success(), "Pip install failed");

    let status = behave_runner.run_tests(&root);

    assert!(status.success(), "Behave tests failed");
}
