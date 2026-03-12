use glob::glob;
use std::env;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

enum MinifiBehaveLocation {
    Directory(PathBuf),
    Wheel(PathBuf),
}

impl Display for MinifiBehaveLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            MinifiBehaveLocation::Directory(dir) => dir.to_str().unwrap(),
            MinifiBehaveLocation::Wheel(wheel) => wheel.to_str().unwrap(),
        };
        write!(f, "{}", str)
    }
}

impl MinifiBehaveLocation {
    fn from_local_repo(repo_root: &Path) -> Option<Self> {
        let behave_location = repo_root.to_path_buf().join("behave_framework");
        if !behave_location.exists() {
            return None;
        }
        Some(MinifiBehaveLocation::Directory(behave_location))
    }

    fn from_local_sdk(sdk_root: &Path) -> Option<Self> {
        std::fs::read_dir(&sdk_root)
            .ok()?
            .filter_map(|e| e.ok())
            .find(|e| e.path().extension().unwrap_or_default() == "whl")
            .map(|e| MinifiBehaveLocation::Wheel(e.path()))
    }
}

struct BehaveRunner {
    minifi_behave_location: MinifiBehaveLocation,
    venv_path: PathBuf,
}

impl BehaveRunner {
    fn new(minifi_behave_location: MinifiBehaveLocation, out_dir: &Path) -> Self {
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
            minifi_behave_location,
            venv_path: venv_dir,
        }
    }

    fn setup() -> Option<Self> {
        let sdk_path = env::var("MINIFI_SDK_PATH").expect(
            "MINIFI_SDK_PATH must be set. Please define it in your environment or .cargo/config.toml"
        );
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        if sdk_path.starts_with("https://") {
            let zip_path = out_dir.join("downloaded-sdk.zip");
            let extract_dir = out_dir.join("extracted-sdk");
            if !extract_dir.exists() {
                Self::download_file(&sdk_path, &zip_path);
                Self::extract_zip(&zip_path, &extract_dir);
            }
            MinifiBehaveLocation::from_local_sdk(&extract_dir).map(|loc| Self::new(loc, &out_dir))
        } else {
            let local_path = PathBuf::from(&sdk_path);
            if local_path.is_file() && local_path.extension().unwrap_or_default() == "zip" {
                let extract_dir = out_dir.join("extracted-local-sdk");
                if !extract_dir.exists() {
                    Self::extract_zip(&local_path, &extract_dir);
                }
                MinifiBehaveLocation::from_local_sdk(&extract_dir)
                    .map(|loc| Self::new(loc, &out_dir))
            } else {
                MinifiBehaveLocation::from_local_sdk(&local_path)
                    .or_else(|| MinifiBehaveLocation::from_local_repo(&local_path))
                    .map(|loc| Self::new(loc, &out_dir))
            }
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
            .arg(self.minifi_behave_location.to_string())
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

    fn download_file(url: &str, dest: &Path) {
        println!("Downloading SDK from {}...", url);
        let status = Command::new("curl")
            .args(["-fSL", "-o", dest.to_str().unwrap(), url])
            .status()
            .expect("Failed to execute curl to download SDK");
        assert!(status.success(), "Failed to download SDK from {}", url);
    }

    fn extract_zip(archive: &Path, dest_dir: &Path) {
        println!("Extracting SDK archive...");
        std::fs::create_dir_all(dest_dir).unwrap();
        let status = Command::new("tar")
            .args([
                "-xf",
                archive.to_str().unwrap(),
                "-C",
                dest_dir.to_str().unwrap(),
            ])
            .status()
            .expect("Failed to execute tar to extract SDK zip");
        assert!(status.success(), "Failed to extract SDK zip file");
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
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let root = Path::new(&manifest_dir);

    #[cfg(target_os = "macos")]
    build_linux_so(root);

    let behave_runner = BehaveRunner::setup().expect("Failed to setup behave runner");
    let install_status = behave_runner.install_minifi_behave();

    assert!(install_status.success(), "Pip install failed");

    let status = behave_runner.run_tests(&root);

    assert!(status.success(), "Behave tests failed");
}
