#![cfg_attr(not(test), no_std)]
#![feature(c_variadic)]

mod errno;
pub mod exit;
pub mod malloc;
pub mod math;
pub mod stdio;
pub mod string;
pub mod types;
pub mod unistd;

pub use errno::Errno;
pub use exit::exit;
