use autovcpkg_build;
use std::env;

fn main() {
    let cfg = autovcpkg_build::prepare();

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

    //autovcpkg_build::finish(&cfg);
}
