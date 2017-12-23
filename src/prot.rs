//! `prot.rs`: UID/GID protection
//!
//! Replace this with Rust standard library functionality

use libc;

pub unsafe fn gid(mut g: i32) -> i32 {
    if libc::setgroups(1, &mut g as (*mut i32) as (*const u32)) == -1i32 {
        -1i32
    } else {
        libc::setgid(g as (u32))
    }
}

pub unsafe fn uid(u: i32) -> i32 {
    libc::setuid(u as (u32))
}
