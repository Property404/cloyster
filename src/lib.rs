#![cfg_attr(not(test), no_std)]
#![feature(thread_local)]
#![feature(lang_items)]
#![feature(c_variadic)]
#![feature(naked_functions)]
#![allow(internal_features)]

use core::{fmt::Write, panic::PanicInfo};

mod assert;
#[cfg(not(test))]
mod crt0;
pub mod errno;
pub mod linux;
mod printf;
pub mod stdio;
pub mod string;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let err = writeln!(stdio::Stdout, "CLOYSTER: {info}").is_err();
    if err {
        let _ = writeln!(stdio::Stdout, "(write error while panicking)");
    }
    loop {
        core::hint::spin_loop();
    }
}

#[cfg(not(test))]
#[lang = "eh_personality"]
fn rust_eh_personality() {}
