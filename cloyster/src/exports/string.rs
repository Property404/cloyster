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
unsafe extern "C" fn memmove(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe { shellder::string::memmove(dst, src, n) }
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

fn ord_to_int(ordering: Ordering) -> c_int {
    match ordering {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

#[no_mangle]
unsafe extern "C" fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int {
    let s1 = unsafe { CStr::from_ptr(s1) };
    let s2 = unsafe { CStr::from_ptr(s2) };
    ord_to_int(shellder::string::strcmp(s1, s2))
}

#[no_mangle]
unsafe extern "C" fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int {
    let s1 = unsafe { CStr::from_ptr(s1) };
    let s2 = unsafe { CStr::from_ptr(s2) };
    ord_to_int(shellder::string::strncmp(s1, s2, n))
}

#[no_mangle]
unsafe extern "C" fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int {
    let s1 = unsafe { CStr::from_ptr(s1) };
    let s2 = unsafe { CStr::from_ptr(s2) };
    ord_to_int(shellder::string::strcasecmp(s1, s2))
}

#[no_mangle]
unsafe extern "C" fn strncasecmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int {
    let s1 = unsafe { CStr::from_ptr(s1) };
    let s2 = unsafe { CStr::from_ptr(s2) };
    ord_to_int(shellder::string::strncasecmp(s1, s2, n))
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
unsafe extern "C" fn strchr(s: *const c_char, c: c_int) -> *mut c_char {
    let Ok(c) = c.try_into() else {
        return ptr::null_mut();
    };
    let s = unsafe { CStr::from_ptr(s) };
    shellder::string::strchr(s, c)
        .map(|v| v.as_ptr() as *mut c_char)
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
unsafe extern "C" fn strrchr(s: *const c_char, c: c_int) -> *mut c_char {
    let Ok(c) = c.try_into() else {
        return ptr::null_mut();
    };
    let s = unsafe { CStr::from_ptr(s) };
    shellder::string::strrchr(s, c)
        .map(|v| v.as_ptr() as *mut c_char)
        .unwrap_or(ptr::null_mut())
}

#[no_mangle]
unsafe extern "C" fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char {
    assert!(!src.is_null());

    let dst = NonNull::new(dst).expect("Unexpected NULL argument to `strcpy`");
    let src = unsafe { CStr::from_ptr(src) };

    unsafe { shellder::string::strcpy(dst, src).as_ptr() }
}

#[no_mangle]
unsafe extern "C" fn strncpy(dst: *mut c_char, src: *const c_char, n: usize) -> *mut c_char {
    assert!(!src.is_null());

    let dst = NonNull::new(dst).expect("Unexpected NULL argument to `strcpy`");
    let src = unsafe { CStr::from_ptr(src) };

    unsafe { shellder::string::strncpy(dst, src, n).as_ptr() }
}

#[no_mangle]
unsafe extern "C" fn stpcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char {
    assert!(!src.is_null());

    let dst = NonNull::new(dst).expect("Unexpected NULL argument to `strcpy`");
    let src = unsafe { CStr::from_ptr(src) };

    unsafe { shellder::string::stpcpy(dst, src).as_ptr() }
}

#[no_mangle]
unsafe extern "C" fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char {
    assert!(!src.is_null());

    let dst = NonNull::new(dst).expect("Unexpected NULL argument to `strcpy`");
    let src = unsafe { CStr::from_ptr(src) };

    unsafe { shellder::string::strcat(dst, src).as_ptr() }
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
