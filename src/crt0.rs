use core::arch::asm;

#[naked]
#[no_mangle]
extern "C" fn _start() {
    unsafe {
        asm!(
            "
        call main
        call _exit
            ",
            options(noreturn)
        );
    }
}
