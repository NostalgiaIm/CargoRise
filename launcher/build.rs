use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // 将 CargoRise.ico 编译成 Windows 资源，并自动链接进 cargorise.exe。
    // 这样资源管理器、任务栏和快捷方式都会显示 CargoRise 图标。
    if env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return;
    }

    let manifest_dir = PathBuf::from(
        env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is required"),
    );
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is required"));
    let rc_file = manifest_dir.join("CargoRise.rc");
    let icon_file = manifest_dir.join("..").join("assets").join("CargoRise.ico");
    let resource_file = out_dir.join("CargoRise.res");

    println!("cargo:rerun-if-changed={}", rc_file.display());
    println!("cargo:rerun-if-changed={}", icon_file.display());

    let status = Command::new("windres")
        .current_dir(&manifest_dir)
        .args([
            "--use-temp-file",
            "--input-format=rc",
            "--output-format=coff",
            "--codepage=65001",
            "--preprocessor=cpp",
            "--input",
            rc_file.to_str().expect("resource path must be valid UTF-8"),
            "--output",
            resource_file
                .to_str()
                .expect("resource output path must be valid UTF-8"),
        ])
        .status()
        .expect("CargoRise requires windres.exe to build the Windows launcher");

    if !status.success() {
        panic!("windres failed with status {status}");
    }

    println!("cargo:rustc-link-arg-bin=cargorise={}", resource_file.display());
}
