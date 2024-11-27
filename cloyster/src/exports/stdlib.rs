use core::ffi::{CStr, c_char, c_double, c_int, c_long, c_longlong};

#[must_use]
#[unsafe(no_mangle)]
unsafe extern "C" fn abs(x: c_int) -> c_int {
    x.abs()
}

#[must_use]
#[unsafe(no_mangle)]
unsafe extern "C" fn atoi(nptr: *const c_char) -> c_int {
    let nptr = unsafe { CStr::from_ptr(nptr) };
    shellder::stdlib::atoi(nptr).unwrap_or(0)
}

#[must_use]
#[unsafe(no_mangle)]
unsafe extern "C" fn atol(nptr: *const c_char) -> c_long {
    let nptr = unsafe { CStr::from_ptr(nptr) };
    shellder::stdlib::atol(nptr).unwrap_or(0)
}

#[must_use]
#[unsafe(no_mangle)]
unsafe extern "C" fn atoll(nptr: *const c_char) -> c_longlong {
    let nptr = unsafe { CStr::from_ptr(nptr) };
    shellder::stdlib::atoll(nptr).unwrap_or(0)
}

#[must_use]
#[unsafe(no_mangle)]
unsafe extern "C" fn atof(nptr: *const c_char) -> c_double {
    let nptr = unsafe { CStr::from_ptr(nptr) };
    shellder::stdlib::atof(nptr).unwrap_or(0.0)
}
