use core::ffi::{c_char, c_int};

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
pub unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    // XXX: Running unit tests, I get instances of memcpy with arguments 0x1, 0x1, 0x00,
    // which is crazy. Need to figure out why this happens. This prevents panicking
    if n == 0 {
        return dst;
    }

    assert_ne!(src, dst);
    assert!(!src.is_null());
    assert!(!dst.is_null());
    {
        // Make sure not overlapping
        let src = src as usize;
        let dst = dst as usize;
        if src > dst {
            assert!(dst + n <= src);
        } else {
            assert!(src + n <= dst);
        }
    }

    for i in 0..n {
        unsafe {
            *(dst.add(i)) = *(src.add(i));
        }
    }
    dst
}

// TODO: fix overlapping
//
// Currently this cals memcpy which will just panic if the buffers overlap
/*
#[no_mangle]
pub unsafe extern "C" fn memmove(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    assert!(!src.is_null());
    assert!(!dst.is_null());
    unimplemented!()
    //unsafe { memcpy(dst, src, n) }
}
*/

/// Copies the value of `c` (converted to an unsigned char) into each of the first `n` characters
/// of the object pointed to by `dst`.
///
/// # Safety
/// `dst` must be a pointer to a region of memory that is valid for `n` bytes
///
/// # Bugs
/// Currently panics on unexpected `c` input. This is not in accordance with the C standard
///
/// # Returns
/// The value of `dst`
#[no_mangle]
pub unsafe extern "C" fn memset(dst: *mut u8, c: c_int, n: usize) -> *mut u8 {
    unsafe {
        for i in 0..n {
            *(dst.add(i)) = u8::try_from(c).unwrap();
        }
    }
    dst
}

/// Compares the first `n` characters of the object pointed to by `src` to the first `n`
/// characters of the object pointed to by `src2`
///
/// # Returns
///
/// 0 if `src1` and `src2` are equal, otherwise returns the difference in value between the first
///   mismatched bytes, returning a negative value if `src1` contains the lesser value.
///
/// # Safety
///
/// `src1` and `src2` must point to regions of memory that are valid for `n` bytes
#[no_mangle]
pub unsafe extern "C" fn memcmp(src1: *const u8, src2: *const u8, n: usize) -> c_int {
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

/// Identical to [memcmp], use that instead
///
/// # Safety
///
/// See [memcmp]
#[no_mangle]
#[deprecated = "Use memcmp instead"]
pub unsafe extern "C" fn bcmp(src1: *const u8, src2: *const u8, n: usize) -> c_int {
    memcmp(src1, src2, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_length() {
        unsafe {
            assert_eq!(strlen(c"dagan".as_ptr()), 5);
            assert_eq!(strlen(c"".as_ptr()), 0);
        }
    }
}
