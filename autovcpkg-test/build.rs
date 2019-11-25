use autovcpkg_build;

fn main() {
    autovcpkg_build::configure(&["sdl2", "libcurl", "zlib"]);
    #[cfg(target_os = "windows")]
    autovcpkg_build::install(&["libcurl.dll", "libcurl.pdb"])
}
