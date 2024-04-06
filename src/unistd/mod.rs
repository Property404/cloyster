use crate::errno::set_errno;
use core::ffi::{c_int, c_void};

mod linux;
use linux as os;

/// Writes up to `count` bytes from `buf` referred to by the file descriptor `fd`
///
/// # Returns
/// On success, the number of bytes written. On error, `write()` returns -1 and sets `errno`
/// appropriately
///
/// # Safety
/// `buf` must point to a valid readable region of memory that is valid for at least `count` bytes
#[no_mangle]
pub unsafe extern "C" fn write(fd: c_int, buf: *const c_void, count: usize) -> c_int {
    assert!(!buf.is_null());
    match unsafe { os::sys_write(fd, buf, count) } {
        Err(errno) => {
            set_errno(errno);
            -1
        }
        Ok(val) => val,
    }
}
