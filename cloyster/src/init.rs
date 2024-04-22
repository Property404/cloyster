use crate::tls;
use core::ffi::{c_char, c_int};
use shellder::exit;

extern "C" {
    fn main(argc: c_int, argv: *const *const c_char) -> c_int;
}

#[no_mangle]
unsafe extern "C" fn __cloyster_start(argc: c_int, argv: *const *const c_char) {
    crate::logging::Logger::init();
    crate::globals::init();

    unsafe {
        let fs = tls::thread_local_init().unwrap();
        let rv = main(argc, argv);
        tls::thread_local_uninit(fs).unwrap();
        exit(rv);
    }
}
