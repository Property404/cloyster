use core::{
    alloc::Layout,
    ffi::c_void,
    ptr::{self, NonNull},
};

#[unsafe(no_mangle)]
extern "C" fn malloc(size: usize) -> *mut c_void {
    shellder::malloc::malloc(size)
        .map(|v| v.cast().as_ptr())
        .unwrap_or(ptr::null_mut())
}

#[unsafe(no_mangle)]
extern "C" fn aligned_alloc(alignment: usize, size: usize) -> *mut c_void {
    let Ok(layout) = Layout::from_size_align(size, alignment) else {
        return ptr::null_mut();
    };
    shellder::malloc::aligned_alloc(layout)
        .map(|v| v.cast().as_ptr())
        .unwrap_or(ptr::null_mut())
}

#[unsafe(no_mangle)]
extern "C" fn calloc(nmemb: usize, size: usize) -> *mut c_void {
    shellder::malloc::calloc(nmemb, size)
        .map(|v| v.cast().as_ptr())
        .unwrap_or(ptr::null_mut())
}

#[unsafe(no_mangle)]
unsafe extern "C" fn realloc(ptr: Option<NonNull<c_void>>, size: usize) -> *mut c_void {
    unsafe {
        if let Some(ptr) = ptr {
            shellder::malloc::realloc(ptr.cast(), size)
                .map(|v| v.cast().as_ptr())
                .unwrap_or(ptr::null_mut())
        } else {
            malloc(size)
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn free(ptr: Option<NonNull<c_void>>) {
    if let Some(ptr) = ptr {
        unsafe { shellder::malloc::free(ptr.cast()).expect("Failed to free") }
    }
}
