use crate::{
    errno::{self, Errno},
    malloc::{free, malloc},
};
use core::{
    ffi::{c_char, c_int, c_long, c_void, CStr},
    mem, ptr,
};
mod file;
pub(crate) use file::{init, Descriptor, File};
mod printf;
use printf::printf_impl;

const EOF: c_int = -1;

/// Output string with terminating newline
///
/// # C Signature
///
/// `int puts(const char *s);`
///
/// # Returns
///
/// A non-negative number on success, or EOF on error
///
/// # Safety
///
/// `s` must be a pointer to a null-terminated string
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn puts(s: *const c_char) -> c_int {
    assert!(!s.is_null());

    let length = crate::string::strlen(s);

    if crate::unistd::write(1, s as *const c_void, length) < 0 {
        return EOF;
    }

    if putchar(b'\n'.into()) < 0 {
        return EOF;
    }

    // XXX: We can return anything we want here as long as it's not a negative number
    0
}

/// Output single extended-ASCII character. Note that while this function accepts an integer, it
/// will panic with any input greater than 0xFF.
///
/// # C Signature
///
/// `int putchar(int c);`
///
/// # Returns
///
/// The character written as an unsigned char cast to int, or EOF on error
#[no_mangle]
#[must_use]
pub extern "C" fn putchar(c: c_int) -> c_int {
    let c: c_char = c
        .try_into()
        .expect("Argument to `putchar` must be a valid C character");

    if unsafe { crate::unistd::write(1, ptr::from_ref(&c) as *const c_void, 1) } < 0 {
        return EOF;
    }

    c.into()
}

/// Output single extended-ASCII character. Note that while this function accepts an integer, it
/// will panic with any input greater than 0xFF.
///
/// # C Signature
///
/// `int putc(int c, FILE *stream);`
///
/// # Returns
///
/// The character written as an unsigned char cast to int, or EOF on error
///
/// # Safety
///
/// `stream` must be a valid stream opened with [fopen]
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn putc(c: c_int, stream: *mut File) -> c_int {
    let c: c_char = c
        .try_into()
        .expect("Argument to `putc` must be a valid C character");

    let fd = unsafe { (*stream).fd };

    if unsafe { crate::unistd::write(fd.0, ptr::from_ref(&c) as *const c_void, 1) } < 0 {
        return EOF;
    }

    c.into()
}

/// Get one C character from file stream
///
/// # Safety
///
/// Same as [read]
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn getc(stream: *mut File) -> c_int {
    let fd = unsafe { (*stream).fd };

    let mut c: c_char = 0;

    if unsafe { crate::unistd::read(fd.0, ptr::from_mut(&mut c) as *mut c_void, 1) } < 0 {
        return EOF;
    }

    c.into()
}

/// Print arguments according to `fmt`. See the man page
///
/// # Returns
///
/// The number of characters outputed, or a negative number on error
///
/// # Safety
///
/// `fmt` must be a pointer to a null-terminated string.
/// The number of args must match the number of args in `fmt`
/// Each arg must be valid to its respective formatter
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn printf(fmt: *const c_char, _args: ...) -> c_int {
    printf_impl(Descriptor::stdout(), fmt, _args).unwrap_or_else(|err| err.as_negative())
}

/// Like [printf] but writes to a file
///
/// # Safety
///
/// See [printf]
///
/// Additionally, `stream` must not overlap with `fmt` or any argument
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn fprintf(stream: *mut File, fmt: *const c_char, _args: ...) -> c_int {
    printf_impl(unsafe { (*stream).fd }, fmt, _args).unwrap_or_else(|err| err.as_negative())
}

/// Open a file
///
/// # Safety
///
/// `pathname` and `mode` must be valid, non-overlapping pointers to null-terminated strings
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut File {
    use crate::unistd::types::{ModeFlags, OpenFlags};
    assert!(!pathname.is_null());
    assert!(!mode.is_null());

    let mut open_flags = OpenFlags::empty();
    for mode in unsafe { CStr::from_ptr(mode) }.to_bytes() {
        open_flags |= match *mode {
            b'r' => OpenFlags::O_RDONLY,
            // This flag is ignored on POSIX
            // TODO: Don't ignore this for Windows
            b'b' => continue,
            b => {
                panic!("Invalid flag to fopen(): {b}");
            }
        }
    }

    let fd = unsafe { crate::unistd::open(pathname, open_flags, ModeFlags::default()) };

    match malloc(mem::size_of::<File>()) {
        Ok(ptr) => {
            let ptr = ptr as *mut File;
            unsafe {
                *ptr = File {
                    fd: Descriptor(fd),
                    error: 0,
                    offset: 0,
                };
            }
            ptr
        }
        Err(errno) => {
            errno::set_errno(errno);
            ptr::null_mut()
        }
    }
}

/// Read a file into `ptr`
///
/// # Safety
///
/// * `file` must have been previously allocated with [fopen]
/// * `ptr` must be a valid writable region of memory at least [size*nmemb] bytes long
/// *
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn fread(
    ptr: *mut c_void,
    size: usize,
    nmemb: usize,
    file: *mut File,
) -> usize {
    assert!(!file.is_null());
    assert!(!ptr.is_null());

    let fd = unsafe { (*file).fd };

    let Some(count) = size.checked_mul(nmemb) else {
        unsafe {
            (*file).error = Errno::CloysterOverflow.as_positive();
        }
        return 0;
    };

    let val = crate::unistd::read(fd.0, ptr, count);

    if let Ok(val) = val.try_into() {
        unsafe {
            match u64::try_from(val) {
                Ok(offset) => {
                    (*file).offset += offset;
                }
                Err(err) => {
                    (*file).error = Errno::from(err).as_positive();
                }
            }
        };
        val
    } else {
        unsafe {
            (*file).error = -val;
            0
        }
    }
}

/// Reposition a stream
///
/// # Safety
///
/// * `stream` must have been previously allocated with [fopen]
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn fseek(stream: *mut File, offset: c_long, whence: c_int) -> c_int {
    assert!(!stream.is_null());

    let fd = unsafe { (*stream).fd };

    let Ok(offset) = offset.try_into() else {
        unsafe {
            (*stream).error = Errno::CloysterOverflow.as_positive();
        }
        return -1;
    };
    let val = crate::unistd::lseek(fd.0, offset, whence);

    if val < 0 {
        unsafe {
            (*stream).error = -val;
            return -1;
        }
    };

    (*stream).offset += u64::try_from(val).expect("BUG: we already checked for negative");

    0
}

/// Get current offset of file
///
/// # Safety
///
/// `file` must have been previously allocated with [fopen]
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn ftell(file: *mut File) -> c_long {
    assert!(!file.is_null());
    unsafe {
        (*file).offset.try_into().unwrap_or_else(|err| {
            let err = Errno::from(err);
            errno::set_errno(err);
            -1
        })
    }
}

/// Close a file
///
/// # Safety
///
/// `file` must have been previously allocated with [fopen]
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn fclose(file: *mut File) -> c_int {
    assert!(!file.is_null());
    match free(file as *mut c_void) {
        Err(err) => err.as_negative(),
        Ok(()) => 0,
    }
}
