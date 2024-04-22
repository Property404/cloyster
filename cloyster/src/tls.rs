//! TLS(Thread Local Storage setup
use core::{
    ffi::c_void,
    ptr::{self, NonNull},
};
use shellder::types::*;
use shellder::Errno;

extern "C" {
    static __tdata_start: c_void;
}

const STATIC_TLS_SIZE: usize = 0x2000;

#[link_section = ".gnu.linkonce.td.tdata_end"]
static mut TDATA_END: () = ();

unsafe fn set_thread_pointer(addr: *const c_void) -> Result<(), Errno> {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        shellder::unistd::arch_prctl(
            shellder::types::ArchPrctlCode::ARCH_SET_FS,
            addr as *const u8,
        )?;
        Ok(())
    }
    #[cfg(target_arch = "riscv64")]
    unsafe {
        core::arch::asm!("mv tp, {}", in(reg) addr);
        Ok(())
    }
}

// Set thread local pointer
// This is NOT set up for multiple threads yet
pub(crate) unsafe fn thread_local_init() -> Result<NonNull<c_void>, Errno> {
    let map_addr = unsafe {
        shellder::unistd::mmap(
            ptr::null(),
            STATIC_TLS_SIZE,
            MmapProtFlags::PROT_READ | MmapProtFlags::PROT_WRITE,
            MmapFlags::MAP_ANONYMOUS | MmapFlags::MAP_PRIVATE,
            0,
            0,
        )?
    };
    let addr = map_addr.as_ptr().wrapping_add(STATIC_TLS_SIZE);

    unsafe {
        set_thread_pointer(addr)?;
    }

    unsafe {
        let tdata_start = ptr::from_ref(&__tdata_start) as *const u8;
        let tdata_len = ptr::addr_of_mut!(TDATA_END) as usize - tdata_start as usize;
        ptr::copy_nonoverlapping(
            tdata_start,
            addr.wrapping_sub(tdata_len) as *mut u8,
            tdata_len,
        );
    }

    Ok(map_addr)
}

pub(crate) unsafe fn thread_local_uninit(ptr: NonNull<c_void>) -> Result<(), Errno> {
    unsafe {
        shellder::unistd::munmap(ptr, STATIC_TLS_SIZE)?;
    }
    Ok(())
}
