pub use crate::unistd::types::*;

/// Time type
#[allow(non_camel_case_types)]
pub type time_t = i64;

/// Time in seconds and nanoseconds
/// Corresponds to the C `timespec` struct
#[derive(Debug, Default, Copy, Clone)]
#[repr(C)]
pub struct TimeSpec {
    pub seconds: time_t,
    // (XXX)
    // Until C23, the standard mandated this to be c_long, although some implementations like glibc
    // did not ALWAYS conform (it was sometimes long long)
    //
    // Since we're focusing on C23, we'll go with "implementation defined" and try for
    // compatibility with glibc, which seems to go with 64-bit signed int
    pub nanoseconds: i64,
}
