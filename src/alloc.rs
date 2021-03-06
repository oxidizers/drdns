//! `alloc.rs`: Legacy allocator functionality
//!
//! This should eventually be replaced with the Rust global allocator, and in
//! a perfect world safe Rust that uses `Box` and `Heap`.

use byte;
use errno::{self, Errno};
use libc;

pub unsafe fn alloc(n: u32) -> *mut u8 {
    let x: *mut u8;
    x = libc::malloc(n as (usize)) as (*mut u8);
    if x.is_null() {
        errno::set_errno(Errno(libc::ENOMEM));
    }
    x
}

pub unsafe fn alloc_re(x: *mut *mut u8, m: u32, n: u32) -> i32 {
    let y: *mut u8;
    y = alloc(n);
    if y.is_null() {
        0i32
    } else {
        byte::copy(y, m, *x);
        free(*x);
        *x = y;
        1i32
    }
}

pub unsafe fn free(x: *mut u8) {
    libc::free(x as (*mut libc::c_void));
}
