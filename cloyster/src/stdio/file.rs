use super::printf::Cout;
use crate::errno::Errno;
use core::{
    ffi::{c_int, c_void},
    fmt, ptr,
};

static mut STDIN: File = File::from_std_stream(0);
static mut STDOUT: File = File::from_std_stream(1);
static mut STDERR: File = File::from_std_stream(2);

// TODO: these should definitely be thread local pointers
#[no_mangle]
static mut stdin: usize = 0;
#[no_mangle]
static mut stdout: usize = 0;
#[no_mangle]
static mut stderr: usize = 0;

pub(crate) fn init() {
    unsafe {
        stdin = ptr::addr_of_mut!(STDIN) as usize;
        stdout = ptr::addr_of_mut!(STDOUT) as usize;
        stderr = ptr::addr_of_mut!(STDERR) as usize;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct Descriptor(pub(crate) c_int);

impl Descriptor {
    pub(crate) fn stdout() -> Self {
        Self(1)
    }

    pub(crate) fn stderr() -> Self {
        Self(2)
    }
}

impl Cout for Descriptor {
    fn put_cstr(&mut self, s: &[u8]) -> Result<(), Errno> {
        let len = s.len();
        unsafe {
            crate::unistd::write(self.0, ptr::from_ref(s) as *const c_void, len)?;
        }
        Ok(())
    }
}

impl fmt::Write for Descriptor {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.put_cstr(s.as_bytes()).map_err(|_| fmt::Error)
    }
}

/// File type used in stdlib file functions (fopen(), fclose(), fprintf(), etc)
#[repr(C)]
#[derive(Debug)]
pub struct File {
    pub(crate) fd: Descriptor,
    pub(crate) error: c_int,
    pub(crate) offset: u64,
}

impl File {
    const fn from_std_stream(fd: c_int) -> Self {
        Self {
            fd: Descriptor(fd),
            error: 0,
            offset: 0,
        }
    }
}
