mod alloc_impl;
mod usize_ext;

use crate::errno::Errno;
use alloc_impl::{Allocator, DefaultMemoryExtender};
use core::{cell::OnceCell, ptr::NonNull};
use spin::Mutex;

static ALLOCATOR: Mutex<OnceCell<Allocator<DefaultMemoryExtender>>> = Mutex::new(OnceCell::new());

pub fn get_num_allocations() -> usize {
    let allocator = ALLOCATOR.lock();
    let Some(allocator) = allocator.get() else {
        return 0;
    };
    allocator.allocations()
}

pub fn malloc(size: usize) -> Result<NonNull<u8>, Errno> {
    let mut allocator = ALLOCATOR.lock();
    allocator.get_or_init(|| Allocator::new(DefaultMemoryExtender).unwrap());
    allocator
        .get_mut()
        .expect("Bug: allocator not initialized")
        .alloc(size)
}

pub fn calloc(nmemb: usize, size: usize) -> Result<NonNull<u8>, Errno> {
    let size = nmemb.checked_mul(size).ok_or(Errno::CloysterOverflow)?;
    let ptr = malloc(size)?;
    unsafe {
        crate::string::memset(ptr, 0x00, size);
    }
    Ok(ptr)
}

/// # Safety
/// See [free]()
pub unsafe fn realloc(ptr: NonNull<u8>, size: usize) -> Result<NonNull<u8>, Errno> {
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
pub unsafe fn free(ptr: NonNull<u8>) -> Result<(), Errno> {
    let mut allocator = ALLOCATOR.lock();
    let allocator = allocator.get_mut().expect("Bug: allocator not initialized");
    unsafe { allocator.free(ptr) }
}
