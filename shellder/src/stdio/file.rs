use super::printf::Cout;
use crate::errno::Errno;
use core::{
    ffi::{c_char, c_int, c_void},
    fmt, ptr,
};

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Descriptor(pub c_int);

impl Descriptor {
    pub const fn stdin() -> Self {
        Self(0)
    }

    pub const fn stdout() -> Self {
        Self(1)
    }

    pub const fn stderr() -> Self {
        Self(1)
    }
}

impl Cout for Descriptor {
    fn put_char(&mut self, s: c_char) -> Result<(), Errno> {
        crate::stdio::putchar(s as c_int)?;
        Ok(())
    }

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
    pub fd: Descriptor,
    pub error: c_int,
    pub eof: bool,
    pub offset: u64,
}

impl File {
    pub(crate) const fn from_desc(fd: Descriptor) -> Self {
        Self {
            fd,
            eof: false,
            error: 0,
            offset: 0,
        }
    }

    pub const fn stdin() -> Self {
        Self::from_desc(Descriptor::stdin())
    }

    pub const fn stdout() -> Self {
        Self::from_desc(Descriptor::stdout())
    }

    pub const fn stderr() -> Self {
        Self::from_desc(Descriptor::stderr())
    }
}
