use crate::{stdlib::exit, types::*};
use core::{
    arch::asm,
    ffi::{c_char, c_int, c_void},
    ptr::{self, NonNull},
};

const STATIC_TLS_SIZE: usize = 0x2000;

extern "C" {
    fn main(argc: c_int, argv: *const *const c_char) -> c_int;
    static __tdata_start: c_void;
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
        crate::unistd::munmap(fs.as_ptr(), STATIC_TLS_SIZE).unwrap();
        exit(rv);
    }
}

#[link_section = ".gnu.linkonce.td.tdata_end"]
static mut TDATA_END: () = ();

// Set thread local pointer
// This is NOT set up for multiple threads yet
fn thread_local_init() -> NonNull<c_void> {
    let map_addr = unsafe {
        crate::unistd::mmap(
            ptr::null(),
            STATIC_TLS_SIZE,
            MmapProtFlags::PROT_READ | MmapProtFlags::PROT_WRITE,
            MmapFlags::MAP_ANONYMOUS | MmapFlags::MAP_PRIVATE,
            0,
            0,
        )
        .unwrap()
    };
    let addr = map_addr.as_ptr().wrapping_add(STATIC_TLS_SIZE);
    unsafe {
        crate::unistd::arch_prctl(crate::types::ArchPrctlCode::ARCH_SET_FS, addr as *const u8)
            .expect("Fork");
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

    map_addr
}
