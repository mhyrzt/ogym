use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_MUJOCO");
    println!("cargo:rerun-if-env-changed=MUJOCO_DIR");
    println!("cargo:rerun-if-env-changed=HOME");

    if env::var_os("CARGO_FEATURE_MUJOCO").is_none() {
        return;
    }

    let mujoco_dir = env::var_os("MUJOCO_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            let home = env::var_os("HOME").expect("Could not find HOME or MUJOCO_DIR");
            PathBuf::from(home).join(".local/mujoco")
        });
    let mujoco_path = mujoco_dir.join("lib");

    println!("cargo:rustc-link-search=native={}", mujoco_path.display());

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_os == "linux" || target_os == "macos" {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", mujoco_path.display());
    } else if target_os == "windows" {
        println!("cargo:warning=On Windows, you must add the mujoco DLL folder to your PATH environment variable manually.");
    }
}
