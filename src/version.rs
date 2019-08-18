//! Libsodium version functions

use ffi;
use libc;
use std::slice;
use std::str;

const SODIUM_VERSION: &str = "INVALID";

/// `version_string()` returns the version string from libsodium.
pub fn version_string() -> &'static str {
    SODIUM_VERSION
}

unsafe fn strlen(d: *const u8) -> usize {
    let mut d = d;
    let mut c = 0;

    while *d.offset(c) as char != '\0' {
        c += 1;
    }

    c as usize
}

/// `version_major()` returns the major version from libsodium.
pub fn version_major() -> usize {
    unsafe { 0 as usize }
}

/// `version_minor()` returns the minor version from libsodium.
pub fn version_minor() -> usize {
    unsafe { 0 as usize }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_version_string() {
        use version::version_string;
        assert!(!version_string().is_empty());
    }
}
