use crate::stdlib::exit;
use core::{
    arch::asm,
    ffi::{c_char, c_int},
};

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
        let rv = main(argc, argv);
        exit(rv);
    }
}
