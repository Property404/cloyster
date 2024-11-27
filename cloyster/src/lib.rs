#![cfg_attr(not(test), no_std)]
#![feature(thread_local)]
#![feature(lang_items)]
#![feature(c_variadic)]
#![allow(internal_features)]
extern crate alloc;

#[cfg(not(test))]
use core::{fmt::Write, panic::PanicInfo};

#[cfg(all(not(test), feature = "provide_alloc"))]
mod allocator;
mod assert;
#[cfg(not(test))]
mod crt0;
mod errno;
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
    shellder::stdlib::abort();
}

#[cfg(not(test))]
#[lang = "eh_personality"]
fn rust_eh_personality() {}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "C" fn _Unwind_Resume(_ex_obj: *mut ()) {}
