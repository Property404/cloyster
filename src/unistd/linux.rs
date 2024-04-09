use crate::{
    errno::Errno,
    types::{off_t, time_t},
};
use core::{
    ffi::{c_char, c_int, c_void},
    ptr,
};
use syscalls::Sysno;

pub(crate) unsafe fn sys_write(
    fd: c_int,
    buf: *const c_void,
    count: usize,
) -> Result<c_int, Errno> {
    unsafe { syscalls::syscall3(Sysno::write, fd.try_into()?, buf as usize, count) }
        .map_err(|_| Errno::CloysterUnknown)
        .and_then(|val| c_int::try_from(val).map_err(Errno::from))
}

pub(crate) unsafe fn sys_read(fd: c_int, buf: *mut c_void, count: usize) -> Result<c_int, Errno> {
    unsafe { syscalls::syscall3(Sysno::read, fd.try_into()?, buf as usize, count) }
        .map_err(|_| Errno::CloysterUnknown)
        .and_then(|val| c_int::try_from(val).map_err(Errno::from))
}

pub(crate) unsafe fn sys_mmap(
    addr: *const c_void,
    length: usize,
    prot: c_int,
    flags: c_int,
    fd: c_int,
    off_t: u64,
) -> Result<*mut c_void, Errno> {
    unsafe {
        syscalls::syscall6(
            Sysno::mmap,
            addr as usize,
            length,
            prot.try_into()?,
            flags.try_into()?,
            fd.try_into()?,
            off_t.try_into()?,
        )
    }
    .map_err(|_| Errno::CloysterUnknown)
    .map(|val| val as *mut c_void)
}

pub(crate) unsafe fn sys_munmap(addr: *const c_void, length: usize) -> Result<c_int, Errno> {
    unsafe { syscalls::syscall2(Sysno::mmap, addr as usize, length) }
        .map_err(|_| Errno::CloysterUnknown)
        .and_then(|val| c_int::try_from(val).map_err(Errno::from))
}

pub(crate) unsafe fn sys_brk(addr: *const c_void) -> Result<*mut c_void, Errno> {
    unsafe { syscalls::syscall1(Sysno::brk, addr as usize) }
        .map_err(|_| Errno::CloysterUnknown)
        .map(|val| val as *mut c_void)
}

pub(crate) unsafe fn sys_sbrk(offset: isize) -> Result<*mut c_void, Errno> {
    let addr = sys_brk(ptr::null())?;
    sys_brk(addr.wrapping_byte_offset(offset))
}

pub(crate) unsafe fn sys_open(
    pathname: *const c_char,
    flags: c_int,
    mode_t: c_int,
) -> Result<c_int, Errno> {
    unsafe {
        syscalls::syscall3(
            Sysno::open,
            pathname as usize,
            flags.try_into()?,
            mode_t.try_into()?,
        )
    }
    .map_err(|_| Errno::CloysterUnknown)
    .and_then(|val| c_int::try_from(val).map_err(Errno::from))
}

pub(crate) fn sys_lseek(fd: c_int, offset: off_t, whence: c_int) -> Result<c_int, Errno> {
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

pub(crate) fn sys_close(fd: c_int) -> Result<c_int, Errno> {
    unsafe { syscalls::syscall1(Sysno::close, fd.try_into()?) }
        .map_err(|_| Errno::CloysterUnknown)
        .and_then(|val| c_int::try_from(val).map_err(Errno::from))
}

pub(crate) fn sys_time() -> Result<time_t, Errno> {
    unsafe { syscalls::syscall1(Sysno::time, 0) }
        .map_err(|_| Errno::CloysterUnknown)
        .and_then(|val| time_t::try_from(val).map_err(Errno::from))
}

pub(crate) unsafe fn sys_exit(status: c_int) -> Result<c_int, Errno> {
    unsafe { syscalls::syscall1(Sysno::exit, status.try_into()?) }
        .map_err(|_| Errno::CloysterUnknown)
        .and_then(|val| c_int::try_from(val).map_err(Errno::from))
}
