use core::ffi::c_int;

pub fn toupper(c: c_int) -> c_int {
    u8::try_from(c)
        .map(|c| c.to_ascii_uppercase() as c_int)
        .unwrap_or(c)
}

pub fn tolower(c: c_int) -> c_int {
    u8::try_from(c)
        .map(|c| c.to_ascii_lowercase() as c_int)
        .unwrap_or(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upper_lower() {
        assert_eq!(tolower(b'a' as c_int), b'a' as c_int);
        assert_eq!(tolower(b'A' as c_int), b'a' as c_int);
        assert_eq!(tolower(b'0' as c_int), b'0' as c_int);
        assert_eq!(tolower(0x7F), 0x7F);
        assert_eq!(tolower(0xFF), 0xFF);

        assert_eq!(toupper(b'a' as c_int), b'A' as c_int);
        assert_eq!(toupper(b'A' as c_int), b'A' as c_int);
        assert_eq!(toupper(b'0' as c_int), b'0' as c_int);
        assert_eq!(toupper(0x7F), 0x7F);
        assert_eq!(toupper(0xFF), 0xFF);
    }
}
