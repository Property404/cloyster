use core::{
    ffi::{c_int, CStr},
    ptr::NonNull,
};

/// Calculate the length of a null-terminated string
///
/// # C Signature
///
/// `size_t strlen(const char *s);`
///
/// # Returns
///
/// The length of the string
pub fn strlen(s: &CStr) -> usize {
    s.to_bytes().len()
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

// TODO: fix overlapping
//
// Currently this cals memcpy which will just panic if the buffers overlap
/*
pub unsafe fn memmove(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
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

pub fn toupper(c: c_int) -> c_int {
    u8::try_from(c)
        .map(|c| c.to_ascii_uppercase() as c_int)
        .unwrap_or(c)
}

pub fn tolower(c: c_int) -> c_int {
    u8::try_from(c)
        .map(|c| c.to_ascii_lowercase() as c_int)
        .unwrap_or(c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr;

    #[test]
    fn upper_lower() {
        assert_eq!(tolower(b'a' as c_int), b'a' as c_int);
        assert_eq!(tolower(b'A' as c_int), b'a' as c_int);
        assert_eq!(tolower(b'0' as c_int), b'0' as c_int);
        assert_eq!(tolower(0x7F), 0x7F);
        assert_eq!(tolower(0xFF), 0xFF);

        assert_eq!(toupper(b'a' as c_int), b'A' as c_int);
        assert_eq!(toupper(b'A' as c_int), b'A' as c_int);
        assert_eq!(toupper(b'0' as c_int), b'0' as c_int);
        assert_eq!(toupper(0x7F), 0x7F);
        assert_eq!(toupper(0xFF), 0xFF);
    }

    #[test]
    fn string_length() {
        assert_eq!(strlen(c"dagan"), 5);
        assert_eq!(strlen(c""), 0);
    }

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
}
