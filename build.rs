use std::env;
use std::fs;
use std::path::Path;

fn split(path: &Path) -> (String, String) {
    let name = path.file_stem().unwrap().to_str().unwrap().to_string();
    let dir = path.parent().unwrap().to_str().unwrap().to_string();

    (
        dir,
        if name.starts_with("lib") {
            name.replacen("lib", "", 1)
        } else {
            name
        },
    )
}

fn get_lib_name(key: &str, long: bool) -> String {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    let ext = if cfg!(windows) { "lib" } else { "a" };
    let flag = if cfg!(windows) { "" } else { "lib" };
    let name = format!("{}-{}-{}", key, os, arch);
    if long {
        format!("{}{}.{}", flag, name, ext)
    } else {
        name
    }
}

fn main() -> std::io::Result<()> {
    let name = "YUV_LIBRARY_PATH";
    println!("cargo:cargo:rerun-if-env-changed={}", name);
    if let Ok(path) = env::var(name) {
        println!("cargo:rerun-if-changed={}", path);
    }

    let bindir = Path::new("./binaries");
    let absolute = fs::canonicalize(bindir)?;

    let (yuv_lib_path, yuv_lib_name) = env::var("YUV_LIBRARY_PATH")
        .map(|path| split(Path::new(&path)))
        .unwrap_or_else(|_| split(&absolute.join(get_lib_name("yuv", true))));

    println!("cargo:rustc-link-lib={}", yuv_lib_name);
    println!("cargo:rustc-link-search=all={}", yuv_lib_path);

    Ok(())
}
