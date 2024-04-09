use crate::errno::set_errno;
use core::{
    ffi::{c_char, c_int, c_void},
    ptr,
};

pub mod types;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
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

/// Exits the current process with status `status`
#[no_mangle]
pub extern "C" fn _exit(status: c_int) -> ! {
    // SAFETY: This exits the process, but doesn't clean anything up. Not sure if that's enough to
    // justify marking this function unsafe
    let rval = unsafe { os::sys_exit(status) };
    unreachable!("Failed to exit process {rval:?}");
}

/// Wrapper for `mmap` syscall
///
/// # Safety
///
/// See man page
#[no_mangle]
pub unsafe extern "C" fn mmap(
    addr: *const c_void,
    length: usize,
    prot: types::MmapProtFlags,
    flags: types::MmapFlags,
    fd: c_int,
    off_t: u64,
) -> *mut c_void {
    match unsafe { os::sys_mmap(addr, length, prot.bits(), flags.bits(), fd, off_t) } {
        Err(errno) => {
            set_errno(errno);
            ptr::null_mut()
        }
        Ok(val) => val,
    }
}

/// Wrapper for `munmap` syscall
///
/// # Safety
///
/// See man page
#[no_mangle]
pub unsafe extern "C" fn munmap(addr: *const c_void, length: usize) -> c_int {
    match unsafe { os::sys_munmap(addr, length) } {
        Err(errno) => {
            set_errno(errno);
            -1
        }
        Ok(val) => val,
    }
}

/// Wrapper for `brk` syscall
///
/// # Safety
///
/// No
#[no_mangle]
pub unsafe extern "C" fn sbrk(size: isize) -> *mut c_void {
    match unsafe { os::sys_sbrk(size) } {
        Err(errno) => {
            set_errno(errno);
            ptr::null_mut()
        }
        Ok(val) => val.wrapping_byte_offset(-size),
    }
}

/// Wrapper for `open` syscall
///
/// # Safety
///
/// `pathname` must point to a valid null-terminated string
/// `mode` MUST be specified correctly if `O_CREAT` or `O_TEMPFILE` is specified in `flags`
#[no_mangle]
pub unsafe extern "C" fn open(
    pathname: *const c_char,
    flags: types::OpenFlags,
    mode: types::ModeFlags,
) -> c_int {
    match unsafe { os::sys_open(pathname, flags.bits(), mode.bits()) } {
        Err(errno) => {
            set_errno(errno);
            -1
        }
        Ok(val) => val,
    }
}

/// Close file descriptor
#[no_mangle]
pub extern "C" fn close(fd: c_int) -> c_int {
    match os::sys_close(fd) {
        Err(errno) => {
            set_errno(errno);
            -1
        }
        Ok(_val) => 0,
    }
}
