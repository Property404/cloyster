use crate::errno::Errno;
use core::{
    ffi::{c_char, c_int, c_void},
    fmt, ptr,
};
mod printf;
use printf::{printf_impl, Cout};

const EOF: c_int = -1;

pub(crate) struct Descriptor(c_int);

impl Descriptor {
    pub(crate) fn stdout() -> Self {
        Self(1)
    }

    pub(crate) fn stderr() -> Self {
        Self(2)
    }
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
