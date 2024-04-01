use std::sync::atomic::{AtomicBool, Ordering};

static HOT_RELOAD_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn set_hot_reload_enabled(enabled: bool) {
    HOT_RELOAD_ENABLED.store(enabled, Ordering::SeqCst);
}

pub(crate) fn is_hot_reload_enabled() -> bool {
    HOT_RELOAD_ENABLED.load(Ordering::SeqCst)
}

#[macro_export]
macro_rules! register {
    () => {
        #[cfg(target_os = "linux")]
        #[no_mangle]
        pub unsafe extern "C" fn __cxa_thread_atexit_impl(
            func: *mut std::ffi::c_void,
            obj: *mut std::ffi::c_void,
            dso_symbol: *mut std::ffi::c_void,
        ) {
            compromise::linux::thread_atexit(func, obj, dso_symbol);
        }
    };
}

#[cfg(target_os = "linux")]
pub mod linux;
