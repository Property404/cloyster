#![cfg_attr(not(test), no_std)]
#![feature(thread_local)]
#![feature(lang_items)]
#![feature(c_variadic)]
#![allow(internal_features)]

#[cfg(not(test))]
use core::{fmt::Write, panic::PanicInfo};

mod assert;
#[cfg(not(test))]
mod crt0;
mod errno;
#[cfg(not(test))]
mod exports;
mod globals;
mod init;
mod logging;
mod tls;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let err = writeln!(shellder::stdio::Descriptor::stderr(), "CLOYSTER: {info}").is_err();
    if err {
        let _ = writeln!(
            shellder::stdio::Descriptor::stderr(),
            "(write error while panicking)"
        );
    }
    shellder::exit::abort();
}

#[cfg(not(test))]
#[lang = "eh_personality"]
fn rust_eh_personality() {}
