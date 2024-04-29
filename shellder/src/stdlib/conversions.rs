use crate::errno::Errno;
use core::{
    ffi::{c_double, c_int, c_long, c_longlong, CStr},
    str::FromStr,
};

/// Convert string to integer
pub fn atoi(nptr: &CStr) -> Result<c_int, Errno> {
    let nptr = nptr.to_str().map_err(|_| Errno::CloysterUnicodeError)?;
    Ok(FromStr::from_str(nptr)?)
}

/// Convert string to long
pub fn atol(nptr: &CStr) -> Result<c_long, Errno> {
    let nptr = nptr.to_str().map_err(|_| Errno::CloysterUnicodeError)?;
    Ok(FromStr::from_str(nptr)?)
}

/// Convert string to long long
pub fn atoll(nptr: &CStr) -> Result<c_longlong, Errno> {
    let nptr = nptr.to_str().map_err(|_| Errno::CloysterUnicodeError)?;
    Ok(FromStr::from_str(nptr)?)
}

/// Convert string to double
pub fn atof(nptr: &CStr) -> Result<c_double, Errno> {
    let nptr = nptr.to_str().map_err(|_| Errno::CloysterUnicodeError)?;
    Ok(FromStr::from_str(nptr)?)
}
