use core::{cell::RefCell, ffi::c_int};

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
