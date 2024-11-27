use core::ffi::{c_double, c_float};

#[must_use]
#[unsafe(no_mangle)]
extern "C" fn fabs(x: c_double) -> c_double {
    shellder::math::fabs(x)
}

#[must_use]
#[unsafe(no_mangle)]
extern "C" fn fabsf(x: c_float) -> c_float {
    shellder::math::fabsf(x)
}
