use core::ffi::c_int;

#[no_mangle]
extern "C" fn exit(status: c_int) -> ! {
    shellder::stdlib::exit(status)
}

/// Causes abnormal process termination
#[no_mangle]
extern "C" fn abort() -> ! {
    shellder::stdlib::abort()
}
