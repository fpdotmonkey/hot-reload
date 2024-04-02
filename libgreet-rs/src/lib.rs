use std::{ffi::CStr, os::raw::c_char};

use common::FrameContext;

/// # Safety
/// name must be a valid pointer to a null-terminated string
#[no_mangle]
pub unsafe extern "C" fn greet(name: *const c_char) {
    let cstr = CStr::from_ptr(name);
    println!("Who let the dogs out? {}!", cstr.to_str().unwrap());
}

#[no_mangle]
pub extern "C" fn draw(context: &mut FrameContext) {
    let pixels = context.pixels();
    for (i, pixel) in pixels.into_iter().enumerate() {
        pixel.b = pixel.b.wrapping_add((i * 2 % 255).try_into().unwrap());
        pixel.g = pixel.g.wrapping_sub((i * 2 % 128).try_into().unwrap());
        pixel.r = pixel.r.wrapping_add((i * 2 % 64).try_into().unwrap());
        pixel.z = 128;
    }
}
