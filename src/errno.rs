use core::{cell::RefCell, ffi::c_int, fmt};

// The C standard only requires 3 error codes: EDOM, ERANGE, and EILSEQ
// The POSIX standard

#[thread_local]
#[no_mangle]
static errno: RefCell<c_int> = RefCell::new(0);

#[no_mangle]
// This is unsafe because it returns the inner address so borrow checking is broken
unsafe extern "C" fn __errno_location() -> *mut c_int {
    core::ptr::addr_of_mut!(*errno.borrow_mut())
}

pub(crate) fn set_errno(val: c_int) {
    *errno.borrow_mut() = val;
}

#[derive(Copy, Clone, Debug)]
#[repr(i16)]
pub enum Errno {
    /// Operation not permitted
    EPERM = 1,
    /// Invalid arguments
    EINVAL = 22,

    /// Fmt error
    CloysterFmtError = 0x1000,
    /// Generic syscall failure. More information can usually be found in `errno`
    CloysterSyscallFailed,
}

impl Errno {
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
