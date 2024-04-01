use std::{ffi::CStr, os::raw::c_char};

/// # Safety
/// name must be a valid pointer to a null-terminated string
#[no_mangle]
pub unsafe extern "C" fn greet(name: *const c_char) {
    let cstr = CStr::from_ptr(name);
    println!("What's loading, {}!", cstr.to_str().unwrap());
}
