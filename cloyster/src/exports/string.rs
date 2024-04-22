use core::{
    ffi::{c_char, c_int},
    ptr::NonNull,
};

#[no_mangle]
unsafe extern "C" fn strlen(mut s: *const c_char) -> usize {
    assert!(!s.is_null());
    let mut count = 0;
    while unsafe { *s } != (b'\0' as c_char) {
        s = s.wrapping_byte_add(1);
        count += 1;
    }
    count
}

#[no_mangle]
unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe { shellder::string::memcpy(dst, src, n) }
}

#[no_mangle]
unsafe extern "C" fn memset(dst: *mut u8, c: c_int, n: usize) -> *mut u8 {
    let dst = NonNull::new(dst).expect("Unexpected null arg to `memset()`");
    unsafe { shellder::string::memset(dst, c, n).as_ptr() }
}

#[no_mangle]
unsafe extern "C" fn memcmp(src1: *const u8, src2: *const u8, n: usize) -> c_int {
    unsafe { shellder::string::memcmp(src1, src2, n) }
}

#[no_mangle]
#[deprecated = "Use memcmp instead"]
unsafe extern "C" fn bcmp(src1: *const u8, src2: *const u8, n: usize) -> c_int {
    unsafe { shellder::string::memcmp(src1, src2, n) }
}

#[no_mangle]
extern "C" fn toupper(c: c_int) -> c_int {
    shellder::string::toupper(c)
}

#[no_mangle]
extern "C" fn tolower(c: c_int) -> c_int {
    shellder::string::tolower(c)
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
