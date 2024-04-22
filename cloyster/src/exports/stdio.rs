use crate::errno;
use core::{
    ffi::{c_char, c_int, c_long, c_void, CStr},
    ptr::{self, NonNull},
};
use shellder::stdio::File;

const EOF: c_int = -1;

#[no_mangle]
#[must_use]
unsafe extern "C" fn puts(s: *const c_char) -> c_int {
    assert!(!s.is_null());
    let s = unsafe { CStr::from_ptr(s) };

    match shellder::stdio::puts(s) {
        Ok(val) => val,
        Err(_err) => EOF,
    }
}

#[no_mangle]
#[must_use]
extern "C" fn putchar(c: c_int) -> c_int {
    match shellder::stdio::putchar(c) {
        Ok(val) => val,
        Err(_err) => EOF,
    }
}

#[no_mangle]
#[must_use]
unsafe extern "C" fn putc(c: c_int, stream: Option<NonNull<File>>) -> c_int {
    unsafe {
        match shellder::stdio::putc(c, stream.expect("Unexpected null arg to `putc()`")) {
            Ok(val) => val,
            Err(_err) => EOF,
        }
    }
}

#[no_mangle]
#[must_use]
unsafe extern "C" fn getc(stream: Option<NonNull<File>>) -> c_int {
    unsafe {
        match shellder::stdio::getc(stream.expect("Unexpected null arg to `getc()`")) {
            Ok(val) => val,
            Err(_err) => EOF,
        }
    }
}

#[no_mangle]
#[must_use]
unsafe extern "C" fn printf(fmt: *const c_char, args: ...) -> c_int {
    // This doesn't use the Shellder implementation because we'd recurse indefinitely
    assert!(!fmt.is_null());
    unsafe {
        let fmt = CStr::from_ptr(fmt);
        shellder::stdio::printf(fmt, args)
    }
    .unwrap_or_else(|err| err.as_negative())
}

#[no_mangle]
#[must_use]
unsafe extern "C" fn fprintf(stream: *mut File, fmt: *const c_char, args: ...) -> c_int {
    assert!(!fmt.is_null());
    let stream = NonNull::new(stream).expect("Unexpected null arg to `fprintf()`");
    unsafe {
        let fmt = CStr::from_ptr(fmt);
        shellder::stdio::fprintf(stream, fmt, args)
    }
    .unwrap_or_else(|err| err.as_negative())
}

#[no_mangle]
#[must_use]
unsafe extern "C" fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut File {
    assert!(!pathname.is_null());
    assert!(!mode.is_null());
    let (pathname, mode) = unsafe { (CStr::from_ptr(pathname), CStr::from_ptr(mode)) };

    match shellder::stdio::fopen(pathname, mode) {
        Ok(val) => val.as_ptr(),
        Err(err) => {
            errno::set_errno(err);
            ptr::null_mut()
        }
    }
}

#[no_mangle]
#[must_use]
unsafe extern "C" fn fread(
    ptr: Option<NonNull<c_void>>,
    size: usize,
    nmemb: usize,
    file: Option<NonNull<File>>,
) -> usize {
    let ptr = ptr.expect("Unexpected null arg to `fread()`");
    let file = file.expect("Unexpected null arg to `fread()`");

    unsafe {
        match shellder::stdio::fread(ptr.cast(), size, nmemb, file) {
            Ok(val) => val,
            Err(err) => {
                (*file.as_ptr()).error = err.as_positive();
                0
            }
        }
    }
}

#[no_mangle]
#[must_use]
unsafe extern "C" fn fseek(stream: Option<NonNull<File>>, offset: c_long, whence: c_int) -> c_int {
    let stream = stream.expect("Unexpected null arg to `fread()`");

    unsafe {
        match shellder::stdio::fseek(stream, offset, whence) {
            Ok(()) => 0,
            Err(err) => {
                errno::set_errno(err);
                EOF
            }
        }
    }
}

#[no_mangle]
#[must_use]
unsafe extern "C" fn ftell(file: Option<NonNull<File>>) -> c_long {
    let file = file.expect("Unexpected null arg to `fread()`");

    unsafe {
        match shellder::stdio::ftell(file) {
            Ok(val) => val,
            Err(err) => {
                errno::set_errno(err);
                -1
            }
        }
    }
}

#[no_mangle]
#[must_use]
unsafe extern "C" fn fclose(file: Option<NonNull<File>>) -> c_int {
    let file = file.expect("Unexpected null arg to `fread()`");

    unsafe {
        match shellder::stdio::fclose(file) {
            Ok(()) => 0,
            Err(err) => {
                errno::set_errno(err);
                EOF
            }
        }
    }
}
