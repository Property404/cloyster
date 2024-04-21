use crate::{errno::set_errno, types::*};
use core::{
    ffi::{c_char, c_int, c_void},
    ptr::{self, NonNull},
};

#[no_mangle]
unsafe extern "C" fn write(fd: c_int, buf: *const c_void, count: usize) -> c_int {
    assert!(!buf.is_null());
    match unsafe { crate::unistd::write(fd, buf, count) } {
        Err(errno) => {
            set_errno(errno);
            -1
        }
        Ok(val) => val,
    }
}

#[no_mangle]
unsafe extern "C" fn read(fd: c_int, buf: *mut c_void, count: usize) -> c_int {
    assert!(!buf.is_null());
    match unsafe { crate::unistd::read(fd, buf, count) } {
        Err(errno) => {
            set_errno(errno);
            -1
        }
        Ok(val) => val,
    }
}

#[no_mangle]
extern "C" fn _exit(status: c_int) -> ! {
    crate::unistd::exit(status);
}

#[no_mangle]
unsafe extern "C" fn mmap(
    addr: *const c_void,
    length: usize,
    prot: MmapProtFlags,
    flags: MmapFlags,
    fd: c_int,
    off_t: u64,
) -> *mut c_void {
    match unsafe { crate::unistd::mmap(addr, length, prot, flags, fd, off_t) } {
        Err(errno) => {
            set_errno(errno);
            ptr::null_mut()
        }
        Ok(val) => val.as_ptr(),
    }
}

#[no_mangle]
unsafe extern "C" fn munmap(addr: Option<NonNull<c_void>>, length: usize) -> c_int {
    let addr = addr.expect("Cannot unmap null address");
    match unsafe { crate::unistd::munmap(addr, length) } {
        Err(errno) => {
            set_errno(errno);
            -1
        }
        Ok(val) => val,
    }
}

#[no_mangle]
unsafe extern "C" fn sbrk(size: isize) -> *mut c_void {
    match unsafe { crate::unistd::sbrk(size) } {
        Err(errno) => {
            set_errno(errno);
            ptr::null_mut()
        }
        Ok(val) => val.wrapping_byte_offset(-size),
    }
}

#[no_mangle]
unsafe extern "C" fn open(pathname: *const c_char, flags: OpenFlags, mode: ModeFlags) -> c_int {
    match unsafe { crate::unistd::open(pathname, flags, mode) } {
        Err(errno) => {
            set_errno(errno);
            -1
        }
        Ok(val) => val,
    }
}

#[no_mangle]
extern "C" fn close(fd: c_int) -> c_int {
    match crate::unistd::close(fd) {
        Err(errno) => {
            set_errno(errno);
            -1
        }
        Ok(_val) => 0,
    }
}

#[no_mangle]
extern "C" fn lseek(fd: c_int, offset: off_t, whence: c_int) -> c_int {
    match crate::unistd::lseek(fd, offset, whence) {
        Err(errno) => {
            set_errno(errno);
            -1
        }
        Ok(val) => val,
    }
}

#[no_mangle]
extern "C" fn time(time: *mut time_t) -> time_t {
    if !time.is_null() {
        unimplemented!("Cloyster only currently accepts the argument to time() to be NULL");
    }
    match crate::unistd::time() {
        Err(errno) => {
            set_errno(errno);
            -1
        }
        Ok(val) => val,
    }
}

#[no_mangle]
extern "C" fn clock_gettime(id: clockid_t, tp: Option<NonNull<TimeSpec>>) -> c_int {
    let tp = tp.expect("Unexpected null arg to `clock_gettime()`");
    match crate::unistd::clock_gettime(id, tp) {
        Err(errno) => {
            set_errno(errno);
            -1
        }
        Ok(val) => val,
    }
}

#[no_mangle]
extern "C" fn nanosleep(req: *const TimeSpec, rem: Option<NonNull<TimeSpec>>) -> c_int {
    assert!(!req.is_null());
    match crate::unistd::nanosleep(req, rem) {
        Err(errno) => {
            set_errno(errno);
            -1
        }
        Ok(val) => val,
    }
}
