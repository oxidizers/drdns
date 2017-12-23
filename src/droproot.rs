use libc;
use prot;
use strerr::{StrErr, STRERR_SYS};
use ulong;

pub unsafe fn droproot(fatal: *const u8) {
    let mut id: usize = 0;
    let mut x = libc::getenv((*b"ROOT\0").as_ptr() as *const libc::c_char) as *mut u8;
    if x.is_null() {
        StrErr::die(
            111i32,
            fatal,
            (*b"$ROOT not set\0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const StrErr),
        );
    }
    if libc::chdir(x as (*const i8)) == -1i32 {
        StrErr::die(
            111i32,
            fatal,
            (*b"unable to chdir to \0").as_ptr(),
            x as (*const u8),
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
        );
    }
    if libc::chroot((*b".\0").as_ptr() as *const i8) == -1i32 {
        StrErr::die(
            111i32,
            fatal,
            (*b"unable to chroot to \0").as_ptr(),
            x as (*const u8),
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
        );
    }
    x = libc::getenv((*b"GID\0").as_ptr() as *const libc::c_char) as *mut u8;
    if x.is_null() {
        StrErr::die(
            111i32,
            fatal,
            (*b"$GID not set\0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const StrErr),
        );
    }
    ulong::scan(x as (*const u8), &mut id as (*mut usize));
    if prot::gid(id as (i32)) == -1i32 {
        StrErr::die(
            111i32,
            fatal,
            (*b"unable to setgid: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
        );
    }
    x = libc::getenv((*b"UID\0").as_ptr() as *const libc::c_char) as *mut u8;
    if x.is_null() {
        StrErr::die(
            111i32,
            fatal,
            (*b"$UID not set\0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const StrErr),
        );
    }
    ulong::scan(x as (*const u8), &mut id as (*mut usize));
    if prot::uid(id as (i32)) == -1i32 {
        StrErr::die(
            111i32,
            fatal,
            (*b"unable to setuid: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
        );
    }
}
