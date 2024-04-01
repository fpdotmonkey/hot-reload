use std::ffi::c_void;

type ThreadAtExitFn = unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void);

static SYSTEM_THREAD_ATEXIT: once_cell::sync::Lazy<Option<ThreadAtExitFn>> =
    once_cell::sync::Lazy::new(|| unsafe {
        std::mem::transmute(libc::dlsym(
            libc::RTLD_NEXT,
            #[allow(clippy::transmute_ptr_to_ref)]
            c"__cxa_thread_atexit_impl".as_ptr(),
        ))
    });

pub unsafe fn thread_atexit(func: *mut c_void, obj: *mut c_void, dso_symbol: *mut c_void) {
    if crate::is_hot_reload_enabled() {
        // avoid registering TLS destructors to avoid double-frees and
        // general crashiness
    } else if let Some(system_thread_at_exit) = *SYSTEM_THREAD_ATEXIT {
        // forward TLS destructor registration to glibc
        system_thread_at_exit(func, obj, dso_symbol)
    } else {
        // hot reloading is disabled, but __cxa__thread_atexit_impl
        // isn't defined in libc, so we have to just leak memory
    }
}
