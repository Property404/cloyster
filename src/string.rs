use core::ffi::{c_char, c_int, c_void};

/// Calculate the length of a null-terminated string
///
/// # C Signature
///
/// `size_t strlen(const char *s);`
///
/// # Returns
///
/// The length of the string
///
/// # Safety
///
/// `s` must be a pointer to a null-terminated string
#[no_mangle]
pub unsafe extern "C" fn strlen(mut s: *const c_char) -> usize {
    assert!(!s.is_null());

    let mut count = 0;
    while unsafe { *s != (b'\0' as c_char) } {
        s = unsafe { s.add(1) };
        count += 1;
    }

    count
}

/// Copy `src` into `dst` for `n` bytes
///
/// # Returns
/// Pointer to `dst`
///
/// # Safety
///
/// * `dst` and `src` must be pointer to memory regions with at least `n` valid bits
/// * `dst` and `src` must not overlap
#[no_mangle]
unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    for i in 0..n {
        unsafe {
            *(dst.add(i)) = *(src.add(i));
        }
    }
    dst
}

// TODO: fix overlapping
#[no_mangle]
extern "C" fn memmove(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        for i in 0..n {
            *(dst.add(i)) = *(src.add(i));
        }
    }
    dst
}

#[no_mangle]
extern "C" fn memset(dst: *mut u8, c: c_int, n: usize) {
    unsafe {
        for i in 0..n {
            *(dst.add(i)) = u8::try_from(c).unwrap();
        }
    }
}

#[no_mangle]
extern "C" fn memcmp(src1: *const u8, src2: *const u8, n: usize) -> c_int {
    unsafe {
        for i in 0..n {
            let res = c_int::from(*(src1.add(i))) - c_int::from(*(src2.add(i)));
            if res != 0 {
                return res;
            }
        }
    }
    0
}

#[no_mangle]
extern "C" fn bcmp(src1: *const u8, src2: *const u8, n: usize) -> c_int {
    memcmp(src1, src2, n)
}
