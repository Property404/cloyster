use core::ffi::c_int;

#[no_mangle]
extern "C" fn exit(status: c_int) -> ! {
    shellder::exit::exit(status)
}

/// Causes abnormal process termination
#[no_mangle]
extern "C" fn abort() -> ! {
    shellder::exit::abort()
}
