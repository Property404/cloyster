use crate::{
    errno::{self, Errno},
    malloc::{free, malloc},
};
use core::{
    ffi::{c_char, c_int, c_long, c_void, CStr},
    fmt, mem, ptr,
};
mod printf;
use printf::{printf_impl, Cout};

const EOF: c_int = -1;

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct Descriptor(c_int);

impl Descriptor {
    pub(crate) fn stdout() -> Self {
        Self(1)
    }

    pub(crate) fn stderr() -> Self {
        Self(2)
    }
}

/// File type used in stdlib file functions (fopen(), fclose(), fprintf(), etc)
#[repr(C)]
#[derive(Debug)]
pub struct File {
    fd: Descriptor,
    error: c_int,
}

impl printf::Cout for Descriptor {
    fn put_cstr(&mut self, s: &[u8]) -> Result<(), Errno> {
        let len = s.len();
        if unsafe { crate::unistd::write(self.0, ptr::from_ref(s) as *const c_void, len) } >= 0 {
            Ok(())
        } else {
            Err(Errno::CloysterSyscallFailed)
        }
    }
}

impl fmt::Write for Descriptor {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.put_cstr(s.as_bytes()).map_err(|_| fmt::Error)
    }
}

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
        }
    }

    val
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
