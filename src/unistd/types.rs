use bitflags::bitflags;
use core::ffi::c_int;

bitflags! {
    /// Flags for [mmap]
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct MmapFlags: c_int {
        /// Updates to this mapping are visible to other processes. The underlying file in
        /// file-backed mappings will be affected by updates to this region.
        /// Standard: POSIX
        const MAP_SHARED = 0x1;
        /// Updates to this mapping are NOT visible to other processes, and the underlying file in
        /// file-backed mappings will NOT be affected by updates to this region.
        /// Standard: POSIX
        const MAP_PRIVATE = 0x2;
        /// For a mapping not backed by any file
        ///
        /// Standard: None (defacto POSIX)
        #[cfg(target_os="linux")]
        const MAP_ANONYMOUS = 0x20;
        #[cfg(target_os="openbsd")]
        const MAP_ANONYMOUS = 0x1000;
    }
}

bitflags! {
    /// Protection flags for [mmap]
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct MmapProtFlags: c_int {
        /// No permissions
        const PROT_NONE = 0x0;
        /// Pages can be read
        const PROT_READ = 0x1;
        /// Pages can be written
        const PROT_WRITE = 0x2;
        /// Pages can be executed
        const PROT_EXEC = 0x4;
    }
}
