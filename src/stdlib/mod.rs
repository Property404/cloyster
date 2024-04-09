use core::ffi::c_int;

/// Causes normal process termination with status `status`
#[no_mangle]
pub extern "C" fn exit(status: c_int) -> ! {
    crate::unistd::_exit(status)
}

/// Causes abnormal process termination
#[no_mangle]
pub extern "C" fn abort() -> ! {
    crate::unistd::_exit(1)
}