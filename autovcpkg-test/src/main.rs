use std::ffi::CStr;
use std::os::raw::c_char;

mod ffi {
    use std::os::raw::c_char;

    extern "C" {
        pub fn zlibVersion() -> *const c_char;
        pub fn curl_version() -> *const c_char;
    }
}

fn zlib_version() {
    unsafe {
        let ptr: *const c_char = ffi::zlibVersion();
        let c_str = CStr::from_ptr(ptr);
        println!("zlib version: {}", c_str.to_string_lossy().to_string());
    }
}

fn curl_version() {
    unsafe {
        let ptr: *const c_char = ffi::curl_version();
        let c_str = CStr::from_ptr(ptr);
        println!("curl version: {}", c_str.to_string_lossy().to_string());
    }
}

fn main() {
    zlib_version();
    curl_version();
}
