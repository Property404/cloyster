use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::NonNull,
};

struct CloysterAllocator;

unsafe impl GlobalAlloc for CloysterAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        shellder::malloc::aligned_alloc(layout).unwrap().as_ptr()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { shellder::malloc::free(NonNull::new(ptr).unwrap()).unwrap() }
    }
}

#[global_allocator]
static GLOBAL: CloysterAllocator = CloysterAllocator;
