use autovcpkg_build;
fn main() {
    autovcpkg_build::configure(&["curl", "zlib"]);
}
