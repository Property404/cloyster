use core::{cell::RefCell, ffi::c_int, fmt, num, ptr};
use spin::Mutex;

// The C standard only requires 3 error codes: EDOM, ERANGE, and EILSEQ
// The POSIX standard

static ERRNO: Mutex<RefCell<c_int>> = Mutex::new(RefCell::new(0));

#[no_mangle]
// This is unsafe because it leaks the inner reference to errno,
// which is not updated atomically
// Really, we need thread_locals
unsafe extern "C" fn __errno_location() -> *const c_int {
    let errno = ERRNO.lock();
    let ptr = ptr::from_ref(&*errno.borrow());
    ptr
}

pub(crate) fn set_errno(val: Errno) {
    let errno = ERRNO.lock();
    *errno.borrow_mut() = val.as_positive();
}

#[derive(Copy, Clone, Debug)]
#[repr(i16)]
pub enum Errno {
    /// Operation not permitted
    EPERM = 1,
    /// Invalid arguments
    EINVAL = 22,

    /// Unknown error
    CloysterUnknown = 0x1000,
    /// Fmt error
    CloysterFmtError,
    /// Failed integer conversion
    CloysterConversionError,
    /// Generic syscall failure. More information can usually be found in `errno`
    CloysterSyscallFailed,
    /// Allocation failure
    CloysterAlloc,
}

impl Errno {
    // Return as positive value, to set errno
    fn as_positive(self) -> c_int {
        self as c_int
    }

    /// Return as negative value, to return from functions
    pub fn as_negative(self) -> c_int {
        -(self as c_int)
    }
}

impl From<fmt::Error> for Errno {
    fn from(_err: fmt::Error) -> Self {
        Self::CloysterFmtError
    }
}

impl From<num::TryFromIntError> for Errno {
    fn from(_err: num::TryFromIntError) -> Self {
        Self::CloysterConversionError
    }
}
