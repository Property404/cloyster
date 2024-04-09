mod alloc_impl;
mod usize_ext;

use crate::errno::Errno;
use alloc_impl::{Allocator, DefaultMemoryExtender};
use core::{cell::OnceCell, ffi::c_void};
use spin::Mutex;

static ALLOCATOR: Mutex<OnceCell<Allocator<DefaultMemoryExtender>>> = Mutex::new(OnceCell::new());

pub fn malloc(size: usize) -> Result<*mut c_void, Errno> {
    let mut allocator = ALLOCATOR.lock();
    allocator.get_or_init(|| Allocator::new(DefaultMemoryExtender).unwrap());
    allocator
        .get_mut()
        .expect("Bug: allocator not initialized")
        .alloc(size)
}

pub fn calloc(nmemb: usize, size: usize) -> Result<*mut c_void, Errno> {
    let size = nmemb.checked_mul(size).ok_or(Errno::CloysterOverflow)?;
    let ptr = malloc(size)?;
    unsafe {
        crate::string::memset(ptr as *mut u8, 0x00, size);
    }
    Ok(ptr)
}

/// # Safety
/// See [free]()
pub unsafe fn realloc(ptr: *mut c_void, size: usize) -> Result<*mut c_void, Errno> {
    unsafe {
        free(ptr)?;
    }
    malloc(size)
}

/// Free a previously allocation section of memory
///
/// # Safety
/// `ptr` must be a pointer to not-already-freed region of memory that was previously allocated
/// with Cloyster's implementation of malloc (or related memory allocation functions)
pub unsafe fn free(ptr: *mut c_void) -> Result<(), Errno> {
    let mut allocator = ALLOCATOR.lock();
    let allocator = allocator.get_mut().expect("Bug: allocator not initialized");
    unsafe { allocator.free(ptr) }
}

// Don't use this impl of malloc/free in test because if there's something wrong with it, it'll
// manifest in weird ways
#[cfg(not(test))]
mod c_exports {
    use super::*;
    use core::ptr;

    #[no_mangle]
    extern "C" fn malloc(size: usize) -> *mut c_void {
        super::malloc(size).unwrap_or(ptr::null_mut())
    }

    #[no_mangle]
    extern "C" fn calloc(nmemb: usize, size: usize) -> *mut c_void {
        super::calloc(nmemb, size).unwrap_or(ptr::null_mut())
    }

    #[no_mangle]
    unsafe extern "C" fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
        unsafe { super::realloc(ptr, size).unwrap_or(ptr::null_mut()) }
    }

    #[no_mangle]
    extern "C" fn free(ptr: *mut c_void) {
        unsafe { super::free(ptr).expect("Failed to free") }
    }
}
