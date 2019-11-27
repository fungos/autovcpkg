use autovcpkg_build;

fn main() {
    autovcpkg_build::configure(&["gtk"]);
    #[cfg(target_os = "windows")]
    autovcpkg_build::lib_fixup(&[("gtk-3.0.lib", "gtk-3.lib"), ("gdk-3.0.lib", "gdk-3.lib")]);
    println!("cargo:rerun-if-changed=build.rs");
}
