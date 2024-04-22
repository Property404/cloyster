use core::{cell::RefCell, ffi::c_int, ptr};
use shellder::Errno;
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
