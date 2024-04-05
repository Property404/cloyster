use core::{
    ffi::{c_char, c_int, c_void, CStr},
    fmt::{self, Write},
    panic::PanicInfo,
    ptr,
};

const EOF: c_int = -1;

pub struct Stdout;
impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        let len = s.len();
        // TODO: switch to unistd
        if crate::linux::write(1, s.as_ptr() as *const c_void, len) >= 0 {
            Ok(())
        } else {
            Err(fmt::Error)
        }
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
unsafe extern "C" fn puts(s: *const c_char) -> c_int {
    assert!(!s.is_null());

    let length = crate::string::strlen(s);

    // TODO: switch to unistd
    if crate::linux::write(1, s as *const c_void, length) < 0 {
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
extern "C" fn putchar(c: c_int) -> c_int {
    let c: c_char = c
        .try_into()
        .expect("Argument to `putchar` must be a valid C character");

    // TODO: switch to unistd
    if crate::linux::write(1, ptr::from_ref(&c) as *const c_void, 1) < 0 {
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
    assert!(!fmt.is_null());
    let fmt = unsafe { CStr::from_ptr(fmt) }.to_bytes();

    let mut char_count = 0;
    let mut idx = 0;
    while idx < fmt.len() {
        if fmt[idx] == b'%' {
            if fmt[idx + 1] == b'd' {
            } else if fmt[idx + 1] == b'%' {
                if putchar(b'%'.into()) < 0 {
                    return -1;
                }
                idx += 1;
                char_count += 1;
            } else {
                unimplemented!();
            }
        } else {
            if putchar(fmt[idx].into()) < 0 {
                return -1;
            }
            char_count += 1;
        }
        idx += 1;
    }
    char_count
    /*
    sum += args.arg::<usize>();
    */
}
