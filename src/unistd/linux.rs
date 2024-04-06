use crate::errno::Errno;
use core::ffi::{c_int, c_void};
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

pub(crate) unsafe fn sys_exit(status: c_int) -> Result<c_int, Errno> {
    unsafe { syscalls::syscall1(Sysno::exit, status.try_into()?) }
        .map_err(|_| Errno::CloysterUnknown)
        .and_then(|val| c_int::try_from(val).map_err(Errno::from))
}
