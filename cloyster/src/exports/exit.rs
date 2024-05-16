use alloc::vec::Vec;
use core::{cell::RefCell, ffi::c_int, mem};
use spin::Mutex;

static AT_EXIT_FNS: Mutex<RefCell<Vec<extern "C" fn()>>> = Mutex::new(RefCell::new(Vec::new()));

#[no_mangle]
extern "C" fn atexit(function: extern "C" fn()) -> c_int {
    let vec = AT_EXIT_FNS.lock();
    let mut vec = vec.borrow_mut();
    vec.push(function);
    0
}

#[no_mangle]
pub(crate) extern "C" fn exit(status: c_int) -> ! {
    let vec = AT_EXIT_FNS.lock();
    let mut vec = vec.borrow_mut();
    let mut funcs = Vec::new();
    mem::swap(&mut funcs, &mut *vec);

    for func in funcs.into_iter() {
        func();
    }

    shellder::stdlib::exit_without_cleanup(status);
}

/// Causes abnormal process termination
#[no_mangle]
extern "C" fn abort() -> ! {
    shellder::stdlib::abort()
}
