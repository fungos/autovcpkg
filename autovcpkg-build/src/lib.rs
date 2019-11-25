use std::path::PathBuf;
use std::env;
use std::fs;

pub struct Config {
    pub root: String,
    pub packages: Vec<String>,
}

pub fn vcpkg_triplet() -> String {
    let arch = match &*env::var("CARGO_CFG_TARGET_ARCH").unwrap() {
        "x86_64" => "x64",
        _ => "x86", // FIXME
    }.to_owned();
    let family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
    arch + "-" + &family
}

pub fn build_root() -> PathBuf {
    let mut out = PathBuf::from(env::var("OUT_DIR").unwrap());
    out.push(".."); // build
    out.push(".."); // out
    out.push(".."); // <crate>
    out
}

pub fn vcpkg_root() -> String {
    let mut out = build_root();
    out.push(".."); // debug|release
    out.push("vcpkg");
    env::var("AUTO_VCPKG_ROOT").unwrap_or_else(|_|
        format!("{}", out.display())
    )
}

pub fn prepare() -> Config {
    let vcpkg_packages = env::var("AUTO_VCPKG_PACKAGES").unwrap_or_else(|_|
        "".into() // crc32c for testing
    );
    let mut feature_packages = Vec::new();
    for (key, _) in env::vars() {
        if key.starts_with("CARGO_FEATURE_") {
            let pkg = key.to_string().trim_start_matches("CARGO_FEATURE_").to_string().to_lowercase().replace("_", "-");
            feature_packages.push(pkg);
        }
    }
    let mut packages = vcpkg_packages.split(";").map(|s| s.to_string()).collect::<Vec<String>>();
    packages.append(&mut feature_packages);

    Config {
        root: vcpkg_root(),
        packages
    }
}

// HACK: temporary hack to validate theory
// FIXME: make cmake find_package and automatically extract a list of all
// libs needed
pub fn package_libs(package: &str) -> Vec<&str> {
    match package {
        "curl" => vec!["libcurl"],
        "gtk" => vec!["gtk-3.0"],
        _ => vec![package],
    }
}

pub fn finish(cfg: &Config) {
    let mut root = PathBuf::from(&cfg.root);
    root.push("installed");
    root.push(vcpkg_triplet());
    let mut lib = root.clone();
    lib.push("lib");

    println!("cargo:rustc-env=AUTO_VCPKG_GIT_TAG=2019.10");
    println!("cargo:rustc-link-search=native={}", lib.display());
    // FIXME: manual-link libs
    for pkg in &cfg.packages {
        if pkg != "" {
            let libs = package_libs(pkg);
            for lib in libs {
                println!("cargo:rustc-link-lib=static={}", lib);
            }
        }
    }
    // DEBUG to view env vars that we have access to
    /*for (key, val) in env::vars() {
        println!("# {} = {}", key, val);
    }*/
}

pub fn configure(packages: &[&str]) {
    let mut cfg = prepare();
    for &pkg in packages.iter() {
        cfg.packages.push(pkg.to_owned());
    }
    finish(&cfg)
}

pub fn install(files: &[&str]) {
    let mut bin = PathBuf::from(vcpkg_root());
    bin.push("installed");
    bin.push(vcpkg_triplet());
    bin.push("bin");

    let target = build_root();
    for &file in files.iter() {
        let mut src = bin.clone();
        src.push(file);
        let mut dst = target.clone();
        dst.push(file);
        fs::copy(src, dst).unwrap();
    }
}

pub fn lib_fixup(files: &[(&str, &str)]) {
    let mut lib = PathBuf::from(vcpkg_root());
    lib.push("installed");
    lib.push(vcpkg_triplet());
    lib.push("lib");

    for (s, d) in files.iter() {
        let mut src = lib.clone();
        src.push(s);
        let mut dst = lib.clone();
        dst.push(d);
        fs::copy(src, dst).unwrap();
    }
}