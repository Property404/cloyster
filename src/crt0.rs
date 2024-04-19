use crate::{stdlib::exit, types::*};
use core::{
    arch::asm,
    ffi::{c_char, c_int, c_void},
    ptr,
};

const STATIC_TLS_SIZE: usize = 0x2000;

extern "C" {
    fn main(argc: c_int, argv: *const *const c_char) -> c_int;
}

#[naked]
#[no_mangle]
extern "C" fn _start() {
    unsafe {
        asm!(
            "
        # Pass argc/argv to main
        pop	rdi
        mov rsi, rsp
        push rdi
        call _cloyster_start
            ",
            options(noreturn)
        );
    }
}

#[no_mangle]
unsafe extern "C" fn _cloyster_start(argc: c_int, argv: *const *const c_char) {
    crate::stdio::init();

    unsafe {
        let fs = thread_local_init();
        let rv = main(argc, argv);
        crate::unistd::munmap(fs, STATIC_TLS_SIZE);
        exit(rv);
    }
}

// Set thread local pointer
// This is NOT set up for multiple threads yet
fn thread_local_init() -> *mut c_void {
    let map_addr = unsafe {
        crate::unistd::mmap(
            ptr::null(),
            STATIC_TLS_SIZE,
            MmapProtFlags::PROT_READ | MmapProtFlags::PROT_WRITE,
            MmapFlags::MAP_ANONYMOUS | MmapFlags::MAP_PRIVATE,
            0,
            0,
        )
    };
    assert!(!map_addr.is_null());
    let addr = map_addr.wrapping_add(STATIC_TLS_SIZE - 8);
    crate::unistd::linux::sys_arch_prctl(
        crate::types::ArchPrctlCode::ARCH_SET_FS,
        addr as *const u8,
    )
    .expect("Fork");
    map_addr
}
