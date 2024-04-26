use core::{
    cmp::Ordering,
    ffi::{c_char, c_int, CStr},
    ptr::{self, NonNull},
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
unsafe extern "C" fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int {
    let s1 = unsafe { CStr::from_ptr(s1) };
    let s2 = unsafe { CStr::from_ptr(s2) };
    match shellder::string::strcmp(s1, s2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

#[no_mangle]
unsafe extern "C" fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int {
    let s1 = unsafe { CStr::from_ptr(s1) };
    let s2 = unsafe { CStr::from_ptr(s2) };
    match shellder::string::strncmp(s1, s2, n) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

#[no_mangle]
unsafe extern "C" fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char {
    let haystack = unsafe { CStr::from_ptr(haystack) };
    let needle = unsafe { CStr::from_ptr(needle) };
    shellder::string::strstr(haystack, needle)
        .map(|v| v.as_ptr() as *mut c_char)
        .unwrap_or(ptr::null_mut())
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
