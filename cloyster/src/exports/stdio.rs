use crate::errno;
use core::{
    ffi::{CStr, VaListImpl, c_char, c_int, c_long, c_void},
    ptr::{self, NonNull},
};
use shellder::stdio::File;

const EOF: c_int = -1;

#[unsafe(no_mangle)]
#[must_use]
unsafe extern "C" fn puts(s: *const c_char) -> c_int {
    assert!(!s.is_null());
    let s = unsafe { CStr::from_ptr(s) };

    match shellder::stdio::puts(s) {
        Ok(val) => val,
        Err(_err) => EOF,
    }
}

#[unsafe(no_mangle)]
#[must_use]
unsafe extern "C" fn fputs(s: *const c_char, stream: Option<NonNull<File>>) -> c_int {
    assert!(!s.is_null());

    let stream = stream.expect("Unexpected null arg to `fputs()`");
    let s = unsafe { CStr::from_ptr(s) };

    match unsafe { shellder::stdio::fputs(s, stream) } {
        Ok(val) => val,
        Err(_err) => EOF,
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn clearerr(stream: Option<NonNull<File>>) {
    let stream = stream.expect("Unexpected null arg to `clearerr()`");
    unsafe {
        shellder::stdio::clearerr(stream);
    }
}

#[unsafe(no_mangle)]
#[must_use]
unsafe extern "C" fn ferror(stream: Option<NonNull<File>>) -> c_int {
    let stream = stream.expect("Unexpected null arg to `ferror()`");
    unsafe { shellder::stdio::ferror(stream).as_positive() }
}

#[unsafe(no_mangle)]
#[must_use]
unsafe extern "C" fn feof(stream: Option<NonNull<File>>) -> c_int {
    let stream = stream.expect("Unexpected null arg to `feof()`");
    unsafe { shellder::stdio::feof(stream).into() }
}

#[unsafe(no_mangle)]
#[must_use]
extern "C" fn putchar(c: c_int) -> c_int {
    match shellder::stdio::putchar(c) {
        Ok(val) => val,
        Err(_err) => EOF,
    }
}

// This is an alias for `fputc`
#[unsafe(no_mangle)]
#[must_use]
unsafe extern "C" fn putc(c: c_int, stream: Option<NonNull<File>>) -> c_int {
    unsafe { fputc(c, stream) }
}

#[unsafe(no_mangle)]
#[must_use]
unsafe extern "C" fn fputc(c: c_int, stream: Option<NonNull<File>>) -> c_int {
    unsafe {
        match shellder::stdio::fputc(c, stream.expect("Unexpected null arg to `putc()`")) {
            Ok(val) => val,
            Err(_err) => EOF,
        }
    }
}

#[unsafe(no_mangle)]
#[must_use]
unsafe extern "C" fn getc(stream: Option<NonNull<File>>) -> c_int {
    unsafe {
        match shellder::stdio::getc(stream.expect("Unexpected null arg to `getc()`")) {
            Ok(val) => val,
            Err(_err) => EOF,
        }
    }
}

#[unsafe(no_mangle)]
#[must_use]
extern "C" fn getchar() -> c_int {
    match shellder::stdio::getchar() {
        Ok(val) => val,
        Err(_err) => EOF,
    }
}

#[unsafe(no_mangle)]
#[must_use]
unsafe extern "C" fn printf(fmt: *const c_char, args: ...) -> c_int {
    unsafe { vprintf(fmt, args) }
}

#[unsafe(no_mangle)]
#[must_use]
unsafe extern "C" fn vprintf(fmt: *const c_char, args: VaListImpl) -> c_int {
    assert!(!fmt.is_null());
    unsafe {
        let fmt = CStr::from_ptr(fmt);
        shellder::stdio::printf(fmt, args)
    }
    .unwrap_or_else(|err| err.as_negative())
}

#[unsafe(no_mangle)]
#[must_use]
unsafe extern "C" fn fprintf(stream: *mut File, fmt: *const c_char, args: ...) -> c_int {
    unsafe { vfprintf(stream, fmt, args) }
}

#[unsafe(no_mangle)]
#[must_use]
unsafe extern "C" fn sprintf(s: Option<NonNull<c_char>>, fmt: *const c_char, args: ...) -> c_int {
    assert!(!fmt.is_null());
    let s = s.expect("Unexpected null arg to `sprintf()`");
    unsafe {
        let fmt = CStr::from_ptr(fmt);
        shellder::stdio::snprintf(s, usize::MAX, fmt, args)
    }
    .unwrap_or_else(|err| err.as_negative())
}

#[unsafe(no_mangle)]
#[must_use]
unsafe extern "C" fn snprintf(
    s: Option<NonNull<c_char>>,
    size: usize,
    fmt: *const c_char,
    args: ...
) -> c_int {
    assert!(!fmt.is_null());
    let s = s.expect("Unexpected null arg to `snprintf()`");
    unsafe {
        let fmt = CStr::from_ptr(fmt);
        shellder::stdio::snprintf(s, size, fmt, args)
    }
    .unwrap_or_else(|err| err.as_negative())
}

#[unsafe(no_mangle)]
#[must_use]
unsafe extern "C" fn vfprintf(stream: *mut File, fmt: *const c_char, args: VaListImpl) -> c_int {
    assert!(!fmt.is_null());
    let stream = NonNull::new(stream).expect("Unexpected null arg to `fprintf()`");
    unsafe {
        let fmt = CStr::from_ptr(fmt);
        shellder::stdio::fprintf(stream, fmt, args)
    }
    .unwrap_or_else(|err| err.as_negative())
}

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
#[must_use]
unsafe extern "C" fn fwrite(
    ptr: *const c_void,
    size: usize,
    nmemb: usize,
    file: Option<NonNull<File>>,
) -> usize {
    let file = file.expect("Unexpected null arg to `fread()`");
    assert!(!ptr.is_null());

    unsafe {
        match shellder::stdio::fwrite(ptr as *const u8, size, nmemb, file) {
            Ok(val) => val,
            Err(err) => {
                (*file.as_ptr()).error = err.as_positive();
                0
            }
        }
    }
}

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
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
