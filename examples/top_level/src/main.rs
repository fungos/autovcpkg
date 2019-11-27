extern crate curl_vcpkg;
extern crate zlib_vcpkg;

fn main() {
    zlib_vcpkg::version();
    curl_vcpkg::version();
}
