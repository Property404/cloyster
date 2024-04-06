use core::{
    ffi::{c_char, c_int, CStr, VaListImpl},
    fmt::{self, Write},
};

pub(crate) trait Cout {
    fn put_cstr(&mut self, cstr: &[u8]) -> Result<(), ()>;
}

struct CountingCout<T: Cout> {
    inner: T,
    count: usize,
}

impl<T: Cout> fmt::Write for CountingCout<T> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        if self.put_cstr(s.as_bytes()).is_err() {
            Err(fmt::Error)
        } else {
            Ok(())
        }
    }
}

impl<T: Cout> Cout for CountingCout<T> {
    fn put_cstr(&mut self, cstr: &[u8]) -> Result<(), ()> {
        self.inner.put_cstr(cstr)?;
        self.count += cstr.len();
        Ok(())
    }
}

impl<T: Cout> From<T> for CountingCout<T> {
    fn from(c: T) -> Self {
        Self { inner: c, count: 0 }
    }
}

unsafe fn parse_placeholder<T: Cout>(
    cout: &mut CountingCout<T>,
    fmt: &[u8],
    args: &mut VaListImpl,
) -> Result<usize, ()> {
    assert!(!fmt.is_empty() && fmt[0] == b'%');
    let fmt = &fmt[1..];
    let mut changed = 2;

    if fmt[0] == b'd' {
        // Safe IFF previous safety guarantees hold up
        write!(cout, "{}", unsafe { args.arg::<c_int>() }).map_err(|_| ())?;
    } else if fmt[0] == b's' {
        // Safe IFF previous safety guarantees hold up
        write!(cout, "{}", unsafe {
            CStr::from_ptr(args.arg::<*const c_char>())
                .to_str()
                .map_err(|_| ())?
        })
        .map_err(|_| ())?;
    } else {
        todo!()
    }

    Ok(changed)
}

pub(crate) unsafe fn printf_impl(
    cout: impl Cout,
    fmt: *const c_char,
    mut args: VaListImpl,
) -> Result<c_int, ()> {
    assert!(!fmt.is_null());
    let fmt = unsafe { CStr::from_ptr(fmt) }.to_bytes();
    let len = fmt.len();

    let mut cout = CountingCout::from(cout);

    let mut last = 0;
    let mut idx = 0;
    while idx < len {
        if fmt[idx] == b'%' {
            // Print the part before the % in one go
            assert!(last <= idx);
            cout.put_cstr(&fmt[last..idx])?;

            // Safe IFF previous safety guarantees hold up
            let changed = unsafe { parse_placeholder(&mut cout, &fmt[idx..], &mut args)? };

            // Parsing the placeholder moves us over some amount of chars
            assert!(changed > 0);
            idx += changed;
            last = idx;
            assert!(idx <= len);
        } else {
            idx += 1;
        }
    }

    cout.put_cstr(&fmt[last..])?;

    Ok(cout.count.try_into().unwrap())
}
