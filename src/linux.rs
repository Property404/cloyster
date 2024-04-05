use core::{
    cell::RefCell,
    ffi::{c_int, c_void},
};
use syscalls::Sysno;

#[thread_local]
#[no_mangle]
static errno: RefCell<c_int> = RefCell::new(0);

#[no_mangle]
// This is unsafe because it returns the inner address so borrow checking is broken
unsafe extern "C" fn __errno_location() -> *mut c_int {
    core::ptr::addr_of_mut!(*errno.borrow_mut())
}

fn set_errno(val: c_int) {
    *errno.borrow_mut() = val;
}

#[no_mangle]
pub extern "C" fn write(fd: c_int, buf: *const c_void, count: usize) -> c_int {
    assert!(!buf.is_null());

    match unsafe { syscalls::syscall3(Sysno::write, fd.try_into().unwrap(), buf as usize, count) } {
        Ok(val) => val.try_into().unwrap(),
        Err(_err) => {
            set_errno(1);
            -1
        }
    }
}
