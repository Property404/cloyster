use core::ffi::{c_char, c_uint, CStr};
// Abort the program after false assertion
//a
// C signature:
// `void __assert_fail(const char * assertion, const char * file, unsigned int line, const char *
// function);`
//
// SAFETY: all pointers must be valid null-terminated strings
#[no_mangle]
unsafe extern "C" fn __assert_fail(
    assertion: *const c_char,
    file: *const c_char,
    line: c_uint,
    function: *const c_char,
) {
    assert!(!assertion.is_null());
    assert!(!file.is_null());
    assert!(!function.is_null());
    let assertion = unsafe { CStr::from_ptr(assertion) }.to_str().unwrap();
    let file = unsafe { CStr::from_ptr(file) }.to_str().unwrap();
    let function = unsafe { CStr::from_ptr(function) }.to_str().unwrap();
    panic!("{file}:{line}: {function}: Assertion `{assertion}` failed");
}
