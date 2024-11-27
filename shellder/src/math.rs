use core::ffi::{c_double, c_float};

/// Return absolute value of double `x`
#[must_use]
pub fn fabs(x: c_double) -> c_double {
    if x < 0.0 { -x } else { x }
}

/// Return absolute value of float `x`
#[must_use]
pub fn fabsf(x: c_float) -> c_float {
    if x < 0.0 { -x } else { x }
}
