use crate::malloc;
use core::ffi::c_int;

/// Causes normal process termination with status `status`
pub fn exit(status: c_int) -> ! {
    if malloc::get_num_allocations() != 0 {
        panic!("Memory leak detected on exit!");
    }
    crate::unistd::exit(status)
}

/// Causes abnormal process termination
pub fn abort() -> ! {
    crate::unistd::exit(1)
}
