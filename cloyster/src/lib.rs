#![cfg_attr(not(test), no_std)]
#![feature(thread_local)]
#![feature(lang_items)]
#![feature(c_variadic)]
#![feature(naked_functions)]
#![allow(internal_features)]

#[cfg(not(test))]
use core::{fmt::Write, panic::PanicInfo};

mod assert;
#[cfg(not(test))]
mod crt0;
pub mod errno;
mod logging;
pub mod malloc;
pub mod stdio;
pub mod stdlib;
pub mod string;
pub mod types;
pub mod unistd;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let err = writeln!(stdio::Descriptor::stderr(), "CLOYSTER: {info}").is_err();
    if err {
        let _ = writeln!(stdio::Descriptor::stderr(), "(write error while panicking)");
    }
    stdlib::abort();
}

#[cfg(not(test))]
#[lang = "eh_personality"]
fn rust_eh_personality() {}
