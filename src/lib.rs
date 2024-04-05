#![no_std]
#![feature(thread_local)]
#![feature(lang_items)]
#![feature(c_variadic)]
#![feature(naked_functions)]
#![allow(unused_imports, internal_features)]

use core::{
    ffi::{c_int, c_void},
    fmt::{self, Write},
    panic::PanicInfo,
};

mod assert;
mod crt0;
pub mod linux;
pub mod stdio;
pub mod string;

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

#[lang = "eh_personality"]
fn rust_eh_personality() {}
