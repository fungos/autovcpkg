use autovcpkg;

fn main() {
    autovcpkg::configure(&["gtk"]);
    #[cfg(target_os = "windows")]
    autovcpkg::lib_fixup(&[("gtk-3.0.lib", "gtk-3.lib"), ("gdk-3.0.lib", "gdk-3.lib")]);
}
