use autovcpkg;
fn main() {
    autovcpkg::configure(&["curl", "zlib"]);
}
