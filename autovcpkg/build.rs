use std::env;
use std::path::PathBuf;

pub struct Config {
    pub root: String,
    pub packages: Vec<String>,
}

pub fn build_root() -> PathBuf {
    let mut out = PathBuf::from(env::var("OUT_DIR").unwrap());
    out.push(".."); // build
    out.push(".."); // out
    out.push(".."); // <crate>
    out
}

pub fn vcpkg_root() -> String {
    #[cfg(target_os = "windows")]
    env::set_var("VCPKGRS_DYNAMIC", "1");

    // if we have VCPKG_ROOT, pass it as AUTO_VCPKG_ROOT too
    if let Ok(path) = env::var("VCPKG_ROOT") {
        env::set_var("AUTO_VCPKG_ROOT", path.clone());
        return path;
    }

    let mut out = build_root();
    out.push(".."); // debug|release
    out.push("vcpkg");
    let path = env::var("AUTO_VCPKG_ROOT").unwrap_or_else(|_| format!("{}", out.display()));

    // if we didn't have VCPKG_ROOT, we need now to pass to vcpkg-rs
    // would be better be able to give this via vcpkg::Configure
    env::set_var("VCPKG_ROOT", path.clone());
    path
}

pub fn prepare() -> Config {
    let vcpkg_packages = env::var("AUTO_VCPKG_PACKAGES").unwrap_or_else(
        |_| "".into(),
    );
    let mut feature_packages = Vec::new();
    for (key, _) in env::vars() {
        if key.starts_with("CARGO_FEATURE_") {
            let pkg = key
                .to_string()
                .trim_start_matches("CARGO_FEATURE_")
                .to_string()
                .to_lowercase()
                .replace("_", "-");
            feature_packages.push(pkg);
        }
    }
    let mut packages = vcpkg_packages
        .split(";")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    packages.append(&mut feature_packages);

    Config {
        root: vcpkg_root(),
        packages,
    }
}

fn main() {
    let cfg = prepare();

    let tag = env::var("AUTO_VCPKG_GIT_TAG").unwrap_or_else(|_|
        env::var("CARGO_PKG_VERSION_PATCH").unwrap().to_owned() + "." +
        &env::var("CARGO_PKG_VERSION_PRE").unwrap()
    );

    let pkg_list = cfg.packages.join(";");
    cmake::Config::new("shim-sys")
        .define("AUTO_VCPKG_ROOT", &cfg.root)
        .define("AUTO_VCPKG_GIT_TAG", &tag)
        .define("AUTO_VCPKG_PACKAGES", pkg_list)
        .build_target("shim")
        .build();
}
