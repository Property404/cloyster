use core::arch::asm;

#[naked]
#[no_mangle]
extern "C" fn _start() {
    unsafe {
        asm!(
            "
        # Pass argc/argv to main
        pop	rdi
        call main
        call _exit
            ",
            options(noreturn)
        );
    }
}
