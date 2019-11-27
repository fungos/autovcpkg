use std::ffi::CStr;
use std::os::raw::c_char;

mod ffi {
    use std::os::raw::c_char;

    extern "C" {
        pub fn zlibVersion() -> *const c_char;
    }
}

pub fn version() {
    unsafe {
        let ptr: *const c_char = ffi::zlibVersion();
        let c_str = CStr::from_ptr(ptr);
        println!("zlib version: {}", c_str.to_string_lossy().to_string());
    }
}
