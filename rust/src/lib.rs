extern crate libc;

use std::str;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn tr_str_is_blank(buf: *const libc::c_char) -> bool {
    let slice = unsafe { str::from_utf8(CStr::from_ptr(buf).to_bytes()).unwrap() };
    slice.chars().all(|c| c.is_whitespace())
}

#[cfg(test)]
mod tests {
    use super::*;
    use libc;

    fn is_blank(string: &'static str) -> bool {
        let ptr = string.as_ptr() as *const libc::c_char;
        tr_str_is_blank(ptr)
    }

    #[test]
    fn empty_string_is_blank() {
        assert!(is_blank(""));
    }

    #[test]
    fn whitespace_string_is_blank() {
        assert!(is_blank("  "));
    }

    #[test]
    fn filled_string_is_not_blank() {
        assert!(!is_blank("yo"));
    }
}
