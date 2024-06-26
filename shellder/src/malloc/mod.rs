mod free_list_impl;
mod usize_ext;

use crate::errno::Errno;
use core::{alloc::Layout, cell::OnceCell, ptr::NonNull};
use free_list_impl::Allocator;
use spin::Mutex;

static ALLOCATOR: Mutex<OnceCell<Allocator>> = Mutex::new(OnceCell::new());

pub fn get_num_allocations() -> usize {
    let allocator = ALLOCATOR.lock();
    let Some(allocator) = allocator.get() else {
        return 0;
    };
    allocator.allocations()
}

pub fn malloc(size: usize) -> Result<NonNull<u8>, Errno> {
    let mut allocator = ALLOCATOR.lock();
    allocator.get_or_init(|| Allocator::new().unwrap());
    allocator
        .get_mut()
        .expect("Bug: allocator not initialized")
        .alloc_unaligned(size)
}

pub fn aligned_alloc(layout: Layout) -> Result<NonNull<u8>, Errno> {
    let mut allocator = ALLOCATOR.lock();
    allocator.get_or_init(|| Allocator::new().unwrap());
    allocator
        .get_mut()
        .expect("Bug: allocator not initialized")
        .alloc(layout)
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
pub unsafe fn realloc(old_region: NonNull<u8>, size: usize) -> Result<NonNull<u8>, Errno> {
    let mut allocator = ALLOCATOR.lock();
    let allocator = allocator.get_mut().expect("Bug: allocator not initialized");

    unsafe {
        let old_size = allocator.size_of(old_region)?;
        let new_region = allocator.alloc_unaligned(size)?;

        crate::string::memcpy(
            new_region.as_ptr(),
            old_region.as_ptr(),
            core::cmp::min(size, old_size),
        );

        allocator.free(old_region)?;
        Ok(new_region)
    }
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
