fn main() {
    let behave_path =
        std::env::var("DEP_MINIFI_BEHAVE_PATH").expect("DEP_MINIFI_BEHAVE_PATH is required");
    println!("cargo:rustc-env=MINIFI_BEHAVE_PATH={}", behave_path);
}
