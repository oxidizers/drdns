//! `readclose.rs`: Read a fle into the buffer and then close it
//!
//! This is used exclusively by the `openreadclose` module, and can be
//! replaced with `std::io`

use errno::{errno, Errno};
use libc;
use stralloc::StrAlloc;

pub unsafe extern "C" fn readclose(fd: i32, sa: *mut StrAlloc, bufsize: u32) -> i32 {
    if StrAlloc::copys(sa, (*b"\0").as_ptr()) == 0 {
        libc::close(fd);
        return -1;
    }

    let current_block;
    let mut r: i32 = 0;
    'loop1: loop {
        if StrAlloc::readyplus(sa, bufsize) == 0 {
            current_block = 7;
            break;
        }
        r = libc::read(
            fd,
            (*sa).s.offset((*sa).len as (isize)) as (*mut libc::c_void),
            bufsize as (usize),
        ) as (i32);
        if r == -1i32 {
            if errno() == Errno(libc::EINTR) {
                continue;
            }
        }
        if r <= 0i32 {
            current_block = 6;
            break;
        }
        (*sa).len = (*sa).len.wrapping_add(r as (u32));
    }
    if current_block == 6 {
        libc::close(fd);
        r
    } else {
        libc::close(fd);
        -1
    }
}
