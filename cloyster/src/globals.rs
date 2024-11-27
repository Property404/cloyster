//! Cloyster C library
//!
//! This crate mostly contains C exports
//! For inner functionality, use Shellder
use core::ptr;
use shellder::stdio::File;

static mut STDIN: File = File::stdin();
static mut STDOUT: File = File::stdout();
static mut STDERR: File = File::stderr();

// TODO: these should definitely be thread local pointers
#[unsafe(no_mangle)]
static mut stdin: usize = 0;
#[unsafe(no_mangle)]
static mut stdout: usize = 0;
#[unsafe(no_mangle)]
static mut stderr: usize = 0;

pub(crate) fn init() {
    unsafe {
        stdin = ptr::addr_of_mut!(STDIN) as usize;
        stdout = ptr::addr_of_mut!(STDOUT) as usize;
        stderr = ptr::addr_of_mut!(STDERR) as usize;
    }
}
