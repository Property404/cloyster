use crate::errno::set_errno;
use core::{
    ffi::{c_char, c_int, c_void, CStr},
    ptr::{self, NonNull},
};
use shellder::types::*;

#[no_mangle]
unsafe extern "C" fn write(fd: c_int, buf: *const c_void, count: usize) -> c_int {
    assert!(!buf.is_null());
    match unsafe { shellder::unistd::write(fd, buf, count) } {
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
    match unsafe { shellder::unistd::read(fd, buf, count) } {
        Err(errno) => {
            set_errno(errno);
            -1
        }
        Ok(val) => val,
    }
}

#[no_mangle]
extern "C" fn _exit(status: c_int) -> ! {
    shellder::unistd::exit(status);
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
    match unsafe { shellder::unistd::mmap(addr, length, prot, flags, fd, off_t) } {
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
    match unsafe { shellder::unistd::munmap(addr, length) } {
        Err(errno) => {
            set_errno(errno);
            -1
        }
        Ok(val) => val,
    }
}

#[no_mangle]
unsafe extern "C" fn sbrk(size: isize) -> *mut c_void {
    match unsafe { shellder::unistd::sbrk(size) } {
        Err(errno) => {
            set_errno(errno);
            ptr::null_mut()
        }
        Ok(val) => val.wrapping_byte_offset(-size) as *mut c_void,
    }
}

#[no_mangle]
unsafe extern "C" fn open(pathname: *const c_char, flags: OpenFlags, mode: ModeFlags) -> c_int {
    assert!(!pathname.is_null());
    match unsafe { shellder::unistd::open(CStr::from_ptr(pathname), flags, mode) } {
        Err(errno) => {
            set_errno(errno);
            -1
        }
        Ok(val) => val,
    }
}

#[no_mangle]
extern "C" fn close(fd: c_int) -> c_int {
    match shellder::unistd::close(fd) {
        Err(errno) => {
            set_errno(errno);
            -1
        }
        Ok(_val) => 0,
    }
}

#[no_mangle]
extern "C" fn lseek(fd: c_int, offset: off_t, whence: c_int) -> c_int {
    match shellder::unistd::lseek(fd, offset, whence) {
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
    match shellder::unistd::time() {
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
    match shellder::unistd::clock_gettime(id, tp) {
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
    match shellder::unistd::nanosleep(req, rem) {
        Err(errno) => {
            set_errno(errno);
            -1
        }
        Ok(val) => val,
    }
}
