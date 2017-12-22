//! `open.rs`: Helpers for opening files
//!
//! These should get replaced with `std::io`

use libc;

pub unsafe fn read(filename: *const u8) -> i32 {
    libc::open(filename, libc::O_RDONLY | libc::O_NDELAY)
}

pub unsafe fn trunc(filename: *const u8) -> i32 {
    libc::open(filename, libc::O_WRONLY | libc::O_NDELAY | libc::O_TRUNC | libc::O_CREAT, 0o644)
}
