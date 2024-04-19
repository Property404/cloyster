use crate::{errno::Errno, types::*};
use core::{
    ffi::{c_char, c_int, c_void},
    ptr::{self, NonNull},
};
use syscalls::Sysno;

/// Writes up to `count` bytes from `buf` referred to by the file descriptor `fd`
///
/// # Safety
/// `buf` must point to a valid readable region of memory that is valid for at least `count` bytes
pub unsafe fn write(fd: c_int, buf: *const c_void, count: usize) -> Result<c_int, Errno> {
    assert!(!buf.is_null());
    unsafe { syscalls::syscall3(Sysno::write, fd.try_into()?, buf as usize, count) }
        .map_err(|_| Errno::CloysterUnknown)
        .and_then(|val| c_int::try_from(val).map_err(Errno::from))
}

/// Read up to `count` bytes to `buf` referred to by the file descriptor `fd`
///
/// # Safety
/// `buf` must point to a valid writable region of memory that is valid for at least `count` bytes
pub unsafe fn read(fd: c_int, buf: *mut c_void, count: usize) -> Result<c_int, Errno> {
    assert!(!buf.is_null());
    unsafe { syscalls::syscall3(Sysno::read, fd.try_into()?, buf as usize, count) }
        .map_err(|_| Errno::CloysterUnknown)
        .and_then(|val| c_int::try_from(val).map_err(Errno::from))
}

/// Wrapper for `mmap` syscall
///
/// # Safety
///
/// See man page
pub unsafe fn mmap(
    addr: *const c_void,
    length: usize,
    prot: MmapProtFlags,
    flags: MmapFlags,
    fd: c_int,
    off_t: u64,
) -> Result<NonNull<c_void>, Errno> {
    unsafe {
        syscalls::syscall6(
            Sysno::mmap,
            addr as usize,
            length,
            prot.bits().try_into()?,
            flags.bits().try_into()?,
            fd.try_into()?,
            off_t.try_into()?,
        )
    }
    .map_err(|_| Errno::CloysterUnknown)
    .and_then(|val| NonNull::new(val as *mut c_void).ok_or(Errno::CloysterUnknown))
}

/// Wrapper for `munmap` syscall
///
/// # Safety
///
/// See man page
pub unsafe fn munmap(addr: *const c_void, length: usize) -> Result<c_int, Errno> {
    assert!(!addr.is_null());
    unsafe { syscalls::syscall2(Sysno::mmap, addr as usize, length) }
        .map_err(|_| Errno::CloysterUnknown)
        .map(|_| 0)
}

/// Wrapper for `brk` syscall
///
/// # Safety
///
/// No
pub unsafe fn brk(addr: *const c_void) -> Result<*mut c_void, Errno> {
    unsafe { syscalls::syscall1(Sysno::brk, addr as usize) }
        .map_err(|_| Errno::CloysterUnknown)
        .map(|val| val as *mut c_void)
}

/// Wrapper for `brk` syscall
///
/// # Safety
///
/// No
pub unsafe fn sbrk(offset: isize) -> Result<*mut c_void, Errno> {
    let addr = unsafe { brk(ptr::null())? };
    unsafe { brk(addr.wrapping_byte_offset(offset)) }
}

/// Wrapper for `open` syscall
///
/// # Safety
///
/// `pathname` must point to a valid null-terminated string
/// `mode` MUST be specified correctly if `O_CREAT` or `O_TEMPFILE` is specified in `flags`
pub unsafe fn open(
    pathname: *const c_char,
    flags: OpenFlags,
    mode_t: ModeFlags,
) -> Result<c_int, Errno> {
    assert!(!pathname.is_null());
    unsafe {
        syscalls::syscall3(
            Sysno::open,
            pathname as usize,
            flags.bits().try_into()?,
            mode_t.bits().try_into()?,
        )
    }
    .map_err(|_| Errno::CloysterUnknown)
    .and_then(|val| c_int::try_from(val).map_err(Errno::from))
}

/// Repositions the file offset of the file descriptor to the direction of `whence`
pub fn lseek(fd: c_int, offset: off_t, whence: c_int) -> Result<c_int, Errno> {
    unsafe {
        syscalls::syscall3(
            Sysno::lseek,
            fd.try_into()?,
            offset.try_into()?,
            whence.try_into()?,
        )
    }
    .map_err(|_| Errno::CloysterUnknown)
    .and_then(|val| c_int::try_from(val).map_err(Errno::from))
}

/// Close file descriptor
pub fn close(fd: c_int) -> Result<c_int, Errno> {
    unsafe { syscalls::syscall1(Sysno::close, fd.try_into()?) }
        .map_err(|_| Errno::CloysterUnknown)
        .and_then(|val| c_int::try_from(val).map_err(Errno::from))
}

/// Get time
///
/// # Safety
///
/// `tloc` must be NULL or a valid time_t
pub fn time() -> Result<time_t, Errno> {
    unsafe { syscalls::syscall1(Sysno::time, 0) }
        .map_err(|_| Errno::CloysterUnknown)
        .and_then(|val| time_t::try_from(val).map_err(Errno::from))
}

/// # Safety
///
/// See man page
pub unsafe fn arch_prctl(code: ArchPrctlCode, addr: *const u8) -> Result<usize, Errno> {
    unsafe { syscalls::syscall2(Sysno::arch_prctl, code as usize, addr as usize) }
        .map_err(|_| Errno::CloysterUnknown)
}

/// Exits the current process with status `status`
pub fn exit(status: c_int) -> ! {
    unsafe {
        syscalls::syscall1(
            Sysno::exit,
            status.try_into().expect("c_int should fit into usize"),
        )
        .expect("Failed to exit");
    }
    unreachable!("Did not exit for some reason");
}
