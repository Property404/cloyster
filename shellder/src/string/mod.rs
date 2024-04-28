use core::{
    cmp::{self, Ordering},
    ffi::{c_char, CStr},
};

mod ctype;
mod mem;
pub use ctype::*;
pub use mem::*;

/// Calculate the length of a null-terminated string
///
/// # C Signature
///
/// `size_t strlen(const char *s);`
///
/// # Returns
///
/// The length of the string
#[must_use]
pub fn strlen(s: &CStr) -> usize {
    s.to_bytes().len()
}

#[must_use]
fn strncmp_inner(s1: &CStr, s2: &CStr, n: Option<usize>, ignore_case: bool) -> Ordering {
    let s1 = s1.to_bytes();
    let s2 = s2.to_bytes();
    for (count, (c1, c2)) in s1.iter().zip(s2.iter()).enumerate() {
        if let Some(n) = n {
            if count >= n {
                return Ordering::Equal;
            }
        }

        let ordering = if ignore_case {
            c1.to_ascii_lowercase().cmp(&c2.to_ascii_lowercase())
        } else {
            c1.cmp(c2)
        };

        if ordering != Ordering::Equal {
            return ordering;
        }
    }

    let s1 = cmp::min(n.unwrap_or(s1.len()), s1.len());
    let s2 = cmp::min(n.unwrap_or(s2.len()), s2.len());
    s1.cmp(&s2)
}

/// Compares the two strings `s1` and `s2` character-by-character. Returns a negative number if s1
/// < s2, a positive number if s1 > s2, otherwise 0
#[must_use]
pub fn strcmp(s1: &CStr, s2: &CStr) -> Ordering {
    strncmp_inner(s1, s2, None, false)
}

/// Like [strcmp()] but compares at most `n` bytes
#[must_use]
pub fn strncmp(s1: &CStr, s2: &CStr, n: usize) -> Ordering {
    strncmp_inner(s1, s2, Some(n), false)
}

/// Like [strcmp] but ignores case
#[must_use]
pub fn strcasecmp(s1: &CStr, s2: &CStr) -> Ordering {
    strncmp_inner(s1, s2, None, true)
}

/// Like [strncmp] but ignores case
#[must_use]
pub fn strncasecmp(s1: &CStr, s2: &CStr, n: usize) -> Ordering {
    strncmp_inner(s1, s2, Some(n), true)
}

/// Locate a substring `needle` in `haystack`
#[must_use]
pub fn strstr<'a>(haystack: &'a CStr, needle: &CStr) -> Option<&'a CStr> {
    let needle_len = strlen(needle);
    let mut haystack = haystack.as_ptr();

    // Safety: haystack is guaranteed to be nul-terminated
    while unsafe { *haystack != 0 } {
        // Safety: substr is guaranteed to be a nul-terminated string
        let substr = unsafe { CStr::from_ptr(haystack) };
        if strncmp(substr, needle, needle_len) == Ordering::Equal {
            return Some(substr);
        }

        haystack = haystack.wrapping_byte_add(1);
    }

    None
}

/// Locate a character `needle` in `haystack`
#[must_use]
pub fn strchr(haystack: &CStr, needle: c_char) -> Option<&CStr> {
    for (index, byte) in haystack.to_bytes().iter().enumerate() {
        if *byte as c_char == needle {
            return unsafe { Some(CStr::from_ptr(haystack.as_ptr().wrapping_byte_add(index))) };
        }
    }

    None
}

/// Locate a character `needle` in `haystack,` searching backwards
#[must_use]
pub fn strrchr(haystack: &CStr, needle: c_char) -> Option<&CStr> {
    for (index, byte) in haystack.to_bytes().iter().enumerate().rev() {
        if *byte as c_char == needle {
            return unsafe { Some(CStr::from_ptr(haystack.as_ptr().wrapping_byte_add(index))) };
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_length() {
        assert_eq!(strlen(c"dagan"), 5);
        assert_eq!(strlen(c""), 0);
    }

    #[test]
    fn string_compare() {
        assert_eq!(strcmp(c"a", c"b"), Ordering::Less);
        assert_eq!(strcmp(c"b", c"a"), Ordering::Greater);
        assert_eq!(strcmp(c"b", c"b"), Ordering::Equal);
        assert_eq!(strcmp(c"actually", c"b"), Ordering::Less);
        assert_eq!(strcmp(c"dagana", c"dagan"), Ordering::Greater);
        assert_eq!(strncmp(c"a", c"b", 0), Ordering::Equal);
        assert_eq!(strncmp(c"a", c"b", 1), Ordering::Less);
        assert_eq!(strncmp(c"ab", c"ac", 1), Ordering::Equal);
        assert_eq!(strncmp(c"dagana", c"daganb", 5), Ordering::Equal);
        assert_eq!(strncmp(c"dagana", c"dagan", 5), Ordering::Equal);
    }

    #[test]
    fn strstr_test() {
        assert_eq!(strstr(c"dagan", c"agan"), Some(c"agan"));
        assert_eq!(strstr(c"dagan", c"organ"), None);
    }

    #[test]
    fn strchr_test() {
        assert_eq!(strchr(c"dagan", b'a' as c_char), Some(c"agan"));
        assert_eq!(strrchr(c"dagan", b'a' as c_char), Some(c"an"));
        assert_eq!(strchr(c"dagan", b'x' as c_char), None);
        assert_eq!(strrchr(c"dagan", b'x' as c_char), None);
    }
}
