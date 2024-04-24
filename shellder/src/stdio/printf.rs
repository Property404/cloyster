use crate::errno::Errno;
use core::{
    ffi::{c_char, c_int, CStr, VaListImpl},
    fmt::{self, Write},
};

// VaList/VaListImpls can't be constructed, so we need a trait in order to create mock objects
pub(crate) trait VaListLike {
    // Annoyingly, VaArgSafe is a sealed trait, so we need a method for each type
    unsafe fn next_int(&mut self) -> c_int;
    unsafe fn next_char(&mut self) -> c_char;
    unsafe fn next_ptr(&mut self) -> usize;
}

impl<'a> VaListLike for VaListImpl<'a> {
    unsafe fn next_int(&mut self) -> c_int {
        unsafe { self.arg::<c_int>() }
    }

    unsafe fn next_char(&mut self) -> c_char {
        unsafe { self.arg::<c_char>() }
    }

    unsafe fn next_ptr(&mut self) -> usize {
        unsafe { self.arg::<usize>() }
    }
}

impl<V: VaListLike> VaListLike for &mut V {
    unsafe fn next_char(&mut self) -> c_char {
        unsafe { (*self).next_char() }
    }

    unsafe fn next_int(&mut self) -> c_int {
        unsafe { (*self).next_int() }
    }

    unsafe fn next_ptr(&mut self) -> usize {
        unsafe { (*self).next_ptr() }
    }
}

pub(crate) trait Cout {
    fn put_cstr(&mut self, cstr: &[u8]) -> Result<(), Errno> {
        for c in cstr {
            self.put_char(*c as c_char)?;
        }
        Ok(())
    }
    fn put_char(&mut self, cstr: c_char) -> Result<(), Errno>;
}

impl<T: Cout> Cout for &mut T {
    fn put_char(&mut self, c: c_char) -> Result<(), Errno> {
        (*self).put_char(c)
    }

    fn put_cstr(&mut self, cstr: &[u8]) -> Result<(), Errno> {
        (*self).put_cstr(cstr)
    }
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
    fn put_char(&mut self, c: c_char) -> Result<(), Errno> {
        self.inner.put_char(c)?;
        self.count += 1;
        Ok(())
    }

    fn put_cstr(&mut self, cstr: &[u8]) -> Result<(), Errno> {
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
    mut args: impl VaListLike,
) -> Result<usize, Errno> {
    assert!(!fmt.is_empty() && fmt[0] == b'%');
    let fmt = &fmt[1..];
    let mut changed = 1;

    if fmt[0] == b'd' || fmt[0] == b'i' {
        // Safe IFF previous safety guarantees hold up
        write!(cout, "{}", unsafe { args.next_int() })?;
        changed += 1;
    } else if fmt[0] == b'x' {
        // Safe IFF previous safety guarantees hold up
        write!(cout, "{:x}", unsafe { args.next_int() })?;
        changed += 1;
    } else if fmt[0] == b'X' {
        // Safe IFF previous safety guarantees hold up
        write!(cout, "{:X}", unsafe { args.next_int() })?;
        changed += 1;
    } else if fmt[0] == b'b' || fmt[0] == b'B' {
        // Safe IFF previous safety guarantees hold up
        write!(cout, "{:b}", unsafe { args.next_int() })?;
        changed += 1;
    } else if fmt[0] == b'p' {
        // Safe IFF previous safety guarantees hold up
        let val = unsafe { args.next_ptr() };
        if val == 0 {
            cout.put_cstr(b"(nil)")?;
        } else {
            write!(cout, "0x{:x}", val)?;
        }
        changed += 1;
    } else if fmt[0] == b's' {
        // Safe IFF previous safety guarantees hold up
        cout.put_cstr(unsafe { CStr::from_ptr(args.next_ptr() as *const c_char) }.to_bytes())?;
        changed += 1;
    } else if fmt[0] == b'c' {
        cout.put_char(unsafe { args.next_char() })?;
        changed += 1;
    } else if fmt[0] == b'%' {
        cout.put_cstr(b"%")?;
        changed += 1;
    } else {
        todo!()
    }

    Ok(changed)
}

pub(crate) unsafe fn printf_impl(
    cout: impl Cout,
    fmt: &CStr,
    mut args: impl VaListLike,
) -> Result<c_int, Errno> {
    let fmt = fmt.to_bytes();
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

    cout.put_cstr(&fmt[last..len])?;

    Ok(cout.count.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::{fmt::Debug, ptr};
    use std::collections::VecDeque;

    struct MockVaList(VecDeque<usize>);

    impl MockVaList {
        fn new() -> Self {
            Self(VecDeque::new())
        }

        fn with<T>(mut self, val: T) -> Self
        where
            T: TryInto<usize> + Debug,
            T::Error: Debug,
        {
            self.0.push_back(val.try_into().expect("Convert error"));
            self
        }

        fn with_str(mut self, val: &'static CStr) -> Self {
            self.0.push_back(ptr::from_ref(val) as *const () as usize);
            self
        }
    }

    impl VaListLike for MockVaList {
        unsafe fn next_int(&mut self) -> c_int {
            self.0.pop_front().unwrap().try_into().unwrap()
        }

        unsafe fn next_char(&mut self) -> c_char {
            self.0.pop_front().unwrap().try_into().unwrap()
        }

        unsafe fn next_ptr(&mut self) -> usize {
            self.0.pop_front().unwrap()
        }
    }

    impl Cout for String {
        fn put_char(&mut self, c: c_char) -> Result<(), Errno> {
            self.push(char::from_u32(c as u32).unwrap());
            Ok(())
        }
    }

    fn check(res: &str, fmt: &CStr, va: impl Into<MockVaList>) {
        let mut string = String::new();
        let length = unsafe { printf_impl(&mut string, fmt, va.into()).unwrap() };
        assert_eq!(string, res);
        assert_eq!(length, res.len().try_into().unwrap());
    }

    #[test]
    fn basic() {
        check("", c"", MockVaList::new());
        check("45", c"45", MockVaList::new());
        check("45", c"%d", MockVaList::new().with(45));
        check("[45]", c"[%d]", MockVaList::new().with(45));
        check("[A]", c"[%c]", MockVaList::new().with('A' as usize));
        check("[3141]", c"[%d%d]", MockVaList::new().with(31).with(41));
        check(
            "[hello]",
            c"[%s]",
            MockVaList::new().with_str(c"hello").with(41),
        );
        check(
            "[hello41]",
            c"[%s%d]",
            MockVaList::new().with_str(c"hello").with(41),
        );
    }
}
