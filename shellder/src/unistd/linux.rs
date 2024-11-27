use crate::{errno::Errno, types::*};
use core::{
    ffi::{CStr, c_int, c_void},
    ptr::{self, NonNull},
};
use syscalls::Sysno;

/// Writes up to `count` bytes from `buf` referred to by the file descriptor `fd`
///
/// # Safety
/// `buf` must point to a valid readable region of memory that is valid for at least `count` bytes
pub unsafe fn write(fd: c_int, buf: *const c_void, count: usize) -> Result<c_int, Errno> {
    assert!(!buf.is_null());
    Ok(
        unsafe { syscalls::syscall3(Sysno::write, fd.try_into()?, buf as usize, count)? }
            .try_into()?,
    )
}

/// Read up to `count` bytes to `buf` referred to by the file descriptor `fd`
///
/// # Safety
/// `buf` must point to a valid writable region of memory that is valid for at least `count` bytes
pub unsafe fn read(fd: c_int, buf: *mut c_void, count: usize) -> Result<c_int, Errno> {
    assert!(!buf.is_null());
    Ok(
        unsafe { syscalls::syscall3(Sysno::read, fd.try_into()?, buf as usize, count)? }
            .try_into()?,
    )
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
    let val = unsafe {
        syscalls::syscall6(
            Sysno::mmap,
            addr as usize,
            length,
            prot.bits().try_into()?,
            flags.bits().try_into()?,
            fd.try_into()?,
            off_t.try_into()?,
        )
    }?;

    NonNull::new(val as *mut c_void).ok_or(Errno::CloysterUnknown)
}

/// Wrapper for `munmap` syscall
///
/// # Safety
///
/// See man page
pub unsafe fn munmap(addr: NonNull<c_void>, length: usize) -> Result<c_int, Errno> {
    let addr = addr.as_ptr();
    assert!(length > 0);
    assert!(((addr as usize) & 0xFFF) == 0);
    assert!((length & 0xFFF) == 0);
    unsafe {
        syscalls::syscall2(Sysno::munmap, addr as usize, length)?;
    }
    Ok(0)
}

/// Wrapper for `brk` syscall
///
/// # Safety
///
/// No
pub unsafe fn brk(addr: *const u8) -> Result<*mut u8, Errno> {
    Ok(unsafe { syscalls::syscall1(Sysno::brk, addr as usize)? } as *mut u8)
}

/// Wrapper for `brk` syscall
///
/// # Safety
///
/// No
pub unsafe fn sbrk(offset: isize) -> Result<*mut u8, Errno> {
    let addr = unsafe { brk(ptr::null())? };
    unsafe {
        brk(addr.wrapping_byte_offset(offset))?;
    }
    Ok(addr)
}

/// Wrapper for `open` syscall
///
/// # Safety
///
/// `pathname` must point to a valid null-terminated string
/// `mode` MUST be specified correctly if `O_CREAT` or `O_TEMPFILE` is specified in `flags`
pub unsafe fn open(pathname: &CStr, flags: OpenFlags, mode_t: ModeFlags) -> Result<c_int, Errno> {
    Ok(unsafe {
        syscalls::syscall4(
            Sysno::openat,
            AT_FDCWD as usize,
            pathname.as_ptr() as usize,
            flags.bits().try_into()?,
            mode_t.bits().try_into()?,
        )?
    }
    .try_into()?)
}

/// Repositions the file offset of the file descriptor to the direction of `whence`
pub fn lseek(fd: c_int, offset: off_t, whence: c_int) -> Result<c_int, Errno> {
    Ok(unsafe {
        syscalls::syscall3(
            Sysno::lseek,
            fd.try_into()?,
            offset.try_into()?,
            whence.try_into()?,
        )?
    }
    .try_into()?)
}

/// Close file descriptor
pub fn close(fd: c_int) -> Result<c_int, Errno> {
    Ok(unsafe { syscalls::syscall1(Sysno::close, fd.try_into()?)? }.try_into()?)
}

/// Get time
pub fn time() -> Result<time_t, Errno> {
    let tv = TimeVal::default();
    let tv_ptr = ptr::from_ref(&tv) as usize;
    unsafe { syscalls::syscall2(Sysno::gettimeofday, tv_ptr, 0)? };
    Ok(tv.seconds)
}

/// Get time
pub fn clock_gettime(id: clockid_t, tp: NonNull<TimeSpec>) -> Result<c_int, Errno> {
    Ok(
        unsafe { syscalls::syscall2(Sysno::clock_gettime, id as usize, tp.as_ptr() as usize)? }
            .try_into()?,
    )
}

/// Sleep for a period of time
///
/// # Safety
///
/// `req` must be non-NULL and point to a valid readable `timespec`
/// `rem` must be either NULL or point to a valid writable `timespec`
pub fn nanosleep(req: *const TimeSpec, rem: Option<NonNull<TimeSpec>>) -> Result<c_int, Errno> {
    let rem = rem.map(|v| v.as_ptr() as usize).unwrap_or(0);
    Ok(unsafe { syscalls::syscall2(Sysno::nanosleep, req as usize, rem)? }.try_into()?)
}

/// # Safety
///
/// See man page
#[cfg(target_arch = "x86_64")]
pub unsafe fn arch_prctl(code: ArchPrctlCode, addr: *const u8) -> Result<usize, Errno> {
    Ok(unsafe { syscalls::syscall2(Sysno::arch_prctl, code as usize, addr as usize)? })
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
