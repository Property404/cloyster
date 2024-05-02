use crate::{
    errno::Errno,
    malloc::{free, malloc},
};
use core::{
    ffi::{c_char, c_int, c_long, c_void, CStr, VaListImpl},
    mem,
    ptr::{self, NonNull},
};
mod file;
pub use file::{Descriptor, File};
mod printf;
use printf::printf_impl;

/// Output string with terminating newline
///
/// # C Signature
///
/// `int puts(const char *s);`
///
/// # Returns
///
/// A non-negative number on success, or EOF on error
pub fn puts(s: &CStr) -> Result<c_int, Errno> {
    let length = crate::string::strlen(s);

    let res = unsafe { crate::unistd::write(1, s.as_ptr() as *const c_void, length)? };

    putchar(b'\n'.into())?;

    Ok(res)
}

/// Output string to stream with terminating newline
///
/// # C Signature
///
/// `int fputs(const char *s);`
///
/// # Returns
///
/// A non-negative number on success, or EOF on error
///
/// # Safety
///
/// `stream` must be a pointer to a File
pub unsafe fn fputs(s: &CStr, stream: NonNull<File>) -> Result<c_int, Errno> {
    let res = unsafe { fwrite(s.as_ptr() as *const u8, crate::string::strlen(s), 1, stream)? };
    unsafe {
        fputc(b'\n' as c_int, stream)?;
    }
    Ok(c_int::try_from(res)? + 1)
}

/// Clear error indicator and eof indicator
///
///
/// # Safety
///
/// `stream` must be a valid pointer to a File
pub unsafe fn clearerr(stream: NonNull<File>) {
    let stream = stream.as_ptr();
    unsafe {
        (*stream).error = 0;
        (*stream).eof = false;
    }
}

/// Return true if eof indicator is set
///
///
/// # Safety
///
/// `stream` must be a valid pointer to a File
pub unsafe fn feof(stream: NonNull<File>) -> bool {
    let stream = stream.as_ptr();
    unsafe { (*stream).eof }
}

/// Return error indicator
///
///
/// # Safety
///
/// `stream` must be a valid pointer to a File
pub unsafe fn ferror(stream: NonNull<File>) -> Errno {
    let stream = stream.as_ptr();
    unsafe { Errno::from_int((*stream).error) }
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
pub fn putchar(c: c_int) -> Result<c_int, Errno> {
    let c: c_char = c
        .try_into()
        .expect("Argument to `putchar` must be a valid C character");

    unsafe {
        crate::unistd::write(1, ptr::from_ref(&c) as *const c_void, 1)?;
    }

    Ok(c.into())
}

/// Output single extended-ASCII character. Note that while this function accepts an integer, it
/// will panic with any input greater than 0xFF.
///
/// # C Signature
///
/// `int fputc(int c, FILE *stream);`
///
/// # Returns
///
/// The character written as an unsigned char cast to int, or EOF on error
///
/// # Safety
///
/// `stream` must be a valid stream opened with [fopen]
pub unsafe fn fputc(c: c_int, stream: NonNull<File>) -> Result<c_int, Errno> {
    let c: c_char = c
        .try_into()
        .expect("Argument to `fputc` must be a valid C character");

    let stream = stream.as_ptr();
    let fd = unsafe { (*stream).fd };

    unsafe {
        crate::unistd::write(fd.0, ptr::from_ref(&c) as *const c_void, 1)?;
    }

    Ok(c.into())
}

/// Get one C character from stdin
pub fn getchar() -> Result<c_int, Errno> {
    let mut c: c_char = 0;

    unsafe {
        crate::unistd::read(0, ptr::from_mut(&mut c) as *mut c_void, 1)?;
    }

    Ok(c.into())
}

/// Get one C character from file stream
///
/// # Safety
///
/// Same as [fread]
pub unsafe fn getc(stream: NonNull<File>) -> Result<c_int, Errno> {
    let stream = stream.as_ptr();
    let fd = unsafe { (*stream).fd };

    let mut c: c_char = 0;

    unsafe {
        crate::unistd::read(fd.0, ptr::from_mut(&mut c) as *mut c_void, 1)?;
    }

    Ok(c.into())
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
pub unsafe fn printf(fmt: &CStr, args: VaListImpl) -> Result<c_int, Errno> {
    unsafe { printf_impl(Descriptor::stdout(), fmt, args) }
}

/// Like [printf()] but writes to a file
///
/// # Safety
///
/// See [printf()]
///
/// Additionally, `stream` must not overlap with `fmt` or any argument
pub unsafe fn fprintf(stream: NonNull<File>, fmt: &CStr, args: VaListImpl) -> Result<c_int, Errno> {
    unsafe { printf_impl((*stream.as_ptr()).fd, fmt, args) }
}

/// Open a file
pub fn fopen(pathname: &CStr, mode: &CStr) -> Result<NonNull<File>, Errno> {
    use crate::unistd::types::{ModeFlags, OpenFlags};
    let mut open_flags = OpenFlags::empty();
    for mode in mode.to_bytes() {
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

    let fd = unsafe { crate::unistd::open(pathname, open_flags, ModeFlags::default())? };

    let file_ptr = malloc(mem::size_of::<File>())?.cast();

    unsafe {
        *(file_ptr.as_ptr()) = File::from_desc(Descriptor(fd));
    }

    Ok(file_ptr)
}

/// Read a file into `ptr`
///
/// # Safety
///
/// * `file` must have been previously allocated with [fopen]
/// * `ptr` must be a valid writable region of memory at least `size*nmemb` bytes long
/// *
pub unsafe fn fread(
    ptr: NonNull<u8>,
    size: usize,
    nmemb: usize,
    file: NonNull<File>,
) -> Result<usize, Errno> {
    let file = file.as_ptr();

    let fd = unsafe { (*file).fd };

    let count = size.checked_mul(nmemb).ok_or(Errno::CloysterOverflow)?;

    let val = unsafe { crate::unistd::read(fd.0, ptr.as_ptr() as *mut c_void, count)? };
    let val = u64::try_from(val)?;

    unsafe {
        (*file).offset += val;
    }

    Ok(val.try_into()?)
}

/// Write to file from `ptr`
///
/// # Safety
///
/// * `file` must have been previously allocated with [fopen]
/// * `ptr` must be a valid redable region of memory at least `size*nmemb` bytes long
/// *
pub unsafe fn fwrite(
    ptr: *const u8,
    size: usize,
    nmemb: usize,
    file: NonNull<File>,
) -> Result<usize, Errno> {
    let file = file.as_ptr();

    let fd = unsafe { (*file).fd };

    let count = size.checked_mul(nmemb).ok_or(Errno::CloysterOverflow)?;

    let val = unsafe { crate::unistd::write(fd.0, ptr as *mut c_void, count)? };
    let val = u64::try_from(val)?;

    unsafe {
        (*file).offset += val;
    }

    Ok(val.try_into()?)
}

/// Reposition a stream
///
/// # Safety
///
/// * `stream` must have been previously allocated with [fopen]
pub unsafe fn fseek(stream: NonNull<File>, offset: c_long, whence: c_int) -> Result<(), Errno> {
    let stream = stream.as_ptr();
    let fd = unsafe { (*stream).fd };

    let val = crate::unistd::lseek(fd.0, offset.try_into()?, whence)?;

    unsafe {
        (*stream).offset += u64::try_from(val)?;
    }

    Ok(())
}

/// Get current offset of file
///
/// # Safety
///
/// `file` must have been previously allocated with [fopen]
pub unsafe fn ftell(file: NonNull<File>) -> Result<c_long, Errno> {
    unsafe { Ok((*file.as_ptr()).offset.try_into()?) }
}

/// Close a file
///
/// # Safety
///
/// `file` must have been previously allocated with [fopen]
pub unsafe fn fclose(file: NonNull<File>) -> Result<(), Errno> {
    unsafe {
        crate::unistd::close((*file.as_ptr()).fd.0)?;
        free(file.cast())?;
    }

    Ok(())
}
