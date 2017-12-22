//! `openreadclose.rs`: Open a file, read it, and then close it
//!
//! This should probably be replaced by `std::io`

use errno::{errno, Errno};
use libc;
use open;
use readclose::readclose;
use stralloc::StrAlloc;

pub unsafe fn openreadclose(
    filename: *const u8,
    sa: *mut StrAlloc,
    bufsize: u32,
) -> i32 {
    let fd = open::read(filename);

    if fd == -1 {
        if errno() == Errno(libc::ENOENT) {
            0
        } else {
            -1
        }
    } else if readclose(fd, sa, bufsize) == -1 {
        -1
    } else {
        1
    }
}
