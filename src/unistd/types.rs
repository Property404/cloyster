use bitflags::bitflags;
use core::ffi::c_int;

/// Type used for describing files
#[allow(non_camel_case_types)]
pub type off_t = isize;

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

bitflags! {
    /// Mode flags for the `open()` and related syscalls
    ///
    ///
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ModeFlags: c_int {
        /// Others have execute permissions
        const OTHERS_EXECUTE = 0x1;
        /// Others have execute permissions
        const OTHERS_WRITE = 0x2;
        /// Others have execute permissions
        const OTHERS_READ = 0x4;
        /// Group has execute permissions
        const GROUP_EXECUTE = 0x8;
        /// Group has execute permissions
        const GROUP_WRITE = 0x10;
        /// Group has execute permissions
        const GROUP_READ = 0x20;
        /// Group has execute permissions
        const OWNER_EXECUTE = 0x40;
        /// Group has execute permissions
        const OWNER_WRITE = 0x80;
        /// Group has execute permissions
        const OWNER_READ = 0x100;
    }
}

impl Default for ModeFlags {
    fn default() -> Self {
        Self::OWNER_WRITE
            | Self::OWNER_READ
            | Self::GROUP_WRITE
            | Self::GROUP_READ
            | Self::OTHERS_READ
    }
}

bitflags! {
    /// Flags for the `open()` syscall
    ///
    ///
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct OpenFlags: c_int {
        /// Read Only
        const O_RDONLY = 0x0;
        /// Write only
        const O_WRONLY = 0x1;
        /// Read or write
        const O_RDWR = 0x2;
    }
}
