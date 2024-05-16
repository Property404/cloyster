//! The 'mem*' functions

use core::{ffi::c_int, ptr::NonNull};

/// Copy `src` into `dst` for `n` bytes
///
/// # Returns
/// Pointer to `dst`
///
/// # Safety
///
/// * `dst` and `src` must be pointer to memory regions with at least `n` valid bytes
/// * `dst` and `src` must not overlap
pub unsafe fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
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

/// Copy `src` into `dst` for `n` bytes, where `src` and `dst` CAN overlap
///
/// # Returns
///
/// Pointer to `dst`
///
/// # Safety
///
/// * `dst` and `src` must be pointers to memory regions with at least `n` valid bytes
pub unsafe fn memmove(dst: *mut u8, src: *const u8, mut n: usize) -> *mut u8 {
    assert!(!src.is_null());
    assert!(!dst.is_null());
    if n == 0 {
        return dst;
    }

    // Copied from OpenBSD's implementation
    // https://github.com/openbsd/src/blob/b613adb4fae800cd980c9bd94360bd20956a6c25/sys/lib/libkern/memmove.c#L41
    // Copyright (c) 1993
    // The Regents of the University of California. All rights reserved.
    // (under BSD license)
    let mut f = src;
    let mut t = dst;
    if (f as usize) < (t as usize) {
        f = f.wrapping_byte_add(n);
        t = t.wrapping_byte_add(n);
        while n > 0 {
            n -= 1;
            t = t.wrapping_byte_offset(-1);
            f = f.wrapping_byte_offset(-1);
            unsafe {
                *t = *f;
            }
        }
    } else {
        while n > 0 {
            n -= 1;
            unsafe {
                *t = *f;
            }
            t = t.wrapping_byte_add(1);
            f = f.wrapping_byte_add(1);
        }
    }
    dst
}

/// Copies the value of `c` (converted to an unsigned char) into each of the first `n` characters
/// of the object pointed to by `dst`.
///
/// # Safety
/// `dst` must be a pointer to a region of memory that is valid for `n` bytes
///
/// # Returns
/// The value of `dst`
pub unsafe fn memset(dst: NonNull<u8>, c: c_int, n: usize) -> NonNull<u8> {
    unsafe {
        for i in 0..n {
            *(dst.as_ptr().add(i)) = c as u8;
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
pub unsafe fn memcmp(src1: *const u8, src2: *const u8, n: usize) -> c_int {
    assert!(!src1.is_null());
    assert!(!src2.is_null());
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

#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr;

    #[test]
    fn copy() {
        let a = b"dagan";
        let mut b = [0; 5];
        unsafe {
            memcpy(
                ptr::from_mut(&mut b) as *mut u8,
                ptr::from_ref(a) as *const u8,
                5,
            );
        }
        for (index, byte) in b.iter().enumerate() {
            assert_eq!(*byte, a[index]);
        }
    }

    #[test]
    fn r#move() {
        let a = b"dagan";
        let mut b = [0; 5];
        unsafe {
            memmove(
                ptr::from_mut(&mut b) as *mut u8,
                ptr::from_ref(a) as *const u8,
                5,
            );
        }
        for (index, byte) in b.iter().enumerate() {
            assert_eq!(*byte, a[index]);
        }
    }
}
