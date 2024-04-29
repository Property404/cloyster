use core::{ffi::c_int, fmt, num};
use enumn::N;

#[derive(Copy, Clone, Debug, N)]
#[repr(i16)]
pub enum Errno {
    /// Operation not permitted
    EPERM = 1,
    /// Invalid arguments
    EINVAL = 22,

    /// Unknown error
    CloysterUnknown = 0x1000,
    /// Fmt error
    CloysterFmtError,
    /// Unicode conversion error
    CloysterUnicodeError,
    /// Number parsing error
    CloysterParseError,
    /// Failed integer conversion
    CloysterConversionError,
    /// Integer Overflow
    CloysterOverflow,
    /// Generic syscall failure. More information can usually be found in `errno`
    CloysterSyscallFailed,
    /// Allocation failure
    CloysterAlloc,
}

impl Errno {
    /// Return as positive value, to set errno
    pub fn as_positive(self) -> c_int {
        self as c_int
    }

    /// Return as negative value, to return from functions
    pub fn as_negative(self) -> c_int {
        -(self as c_int)
    }
}

impl From<fmt::Error> for Errno {
    fn from(_err: fmt::Error) -> Self {
        Self::CloysterFmtError
    }
}

impl From<num::TryFromIntError> for Errno {
    fn from(_err: num::TryFromIntError) -> Self {
        Self::CloysterConversionError
    }
}

impl From<num::ParseIntError> for Errno {
    fn from(_err: num::ParseIntError) -> Self {
        Self::CloysterParseError
    }
}

impl From<num::ParseFloatError> for Errno {
    fn from(_err: num::ParseFloatError) -> Self {
        Self::CloysterParseError
    }
}

#[cfg(target_os = "linux")]
impl From<syscalls::Errno> for Errno {
    fn from(err: syscalls::Errno) -> Errno {
        let Ok(err) = err.into_raw().try_into() else {
            return Self::CloysterConversionError;
        };
        Self::n(err).unwrap_or(Errno::CloysterUnknown)
    }
}
