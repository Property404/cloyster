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

pub(crate) unsafe fn sys_exit(status: c_int) -> Result<c_int, Errno> {
    unsafe { syscalls::syscall1(Sysno::exit, status.try_into()?) }
        .map_err(|_| Errno::CloysterUnknown)
        .and_then(|val| c_int::try_from(val).map_err(Errno::from))
}
