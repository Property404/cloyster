use crate::errno::set_errno;
use core::ffi::{c_int, c_void};
use syscalls::Sysno;

#[no_mangle]
pub extern "C" fn write(fd: c_int, buf: *const c_void, count: usize) -> c_int {
    assert!(!buf.is_null());

    match unsafe { syscalls::syscall3(Sysno::write, fd.try_into().unwrap(), buf as usize, count) } {
        Ok(val) => val.try_into().unwrap(),
        Err(_err) => {
            set_errno(1);
            -1
        }
    }
}
