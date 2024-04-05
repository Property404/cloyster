use core::{arch::asm, ffi::c_int};

#[naked]
#[no_mangle]
extern "C" fn _start() {
    unsafe {
        asm!(
            "
        call main
        call exit
            ",
            options(noreturn)
        );
    }
}

#[no_mangle]
extern "C" fn exit(_: c_int) {
    loop {
        core::hint::spin_loop();
    }
}
