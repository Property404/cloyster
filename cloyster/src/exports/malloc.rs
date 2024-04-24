use core::{
    ffi::c_void,
    ptr::{self, NonNull},
};

#[no_mangle]
extern "C" fn malloc(size: usize) -> *mut c_void {
    shellder::malloc::malloc(size)
        .map(|v| v.cast().as_ptr())
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
extern "C" fn calloc(nmemb: usize, size: usize) -> *mut c_void {
    shellder::malloc::calloc(nmemb, size)
        .map(|v| v.cast().as_ptr())
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
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

#[no_mangle]
extern "C" fn free(ptr: Option<NonNull<c_void>>) {
    if let Some(ptr) = ptr {
        unsafe { shellder::malloc::free(ptr.cast()).expect("Failed to free") }
    }
}
