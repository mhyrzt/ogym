use std::env;
use std::path::PathBuf;

fn main() {
    let home = env::var("HOME").expect("Could not find HOME");
    let mujoco_path = PathBuf::from(&home).join(".local/mujoco/lib");

    println!("cargo:rustc-link-search=native={}", mujoco_path.display());

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_os == "linux" || target_os == "macos" {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", mujoco_path.display());
    } else if target_os == "windows" {
        println!("cargo:warning=On Windows, you must add the mujoco DLL folder to your PATH environment variable manually.");
    }

    println!("cargo:rerun-if-env-changed=HOME");
}
