use libc;
use strerr::{StrErr, STRERR_SYS};
use ulong;

extern "C" {
    fn chdir(arg1: *const u8) -> i32;
    fn chroot(arg1: *const u8) -> i32;
    fn prot_gid(arg1: i32) -> i32;
    fn prot_uid(arg1: i32) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn droproot(mut fatal: *const u8) {
    let mut x: *mut u8;
    let mut id: usize;
    x = libc::getenv((*b"ROOT\0").as_ptr() as *const libc::c_char);
    if x.is_null() {
        StrErr::die(
            111i32,
            fatal,
            (*b"$ROOT not set\0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const strerr),
        );
    }
    if chdir(x as (*const u8)) == -1i32 {
        StrErr::die(
            111i32,
            fatal,
            (*b"unable to chdir to \0").as_ptr(),
            x as (*const u8),
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut STRERR_SYS as (*mut strerr) as (*const strerr),
        );
    }
    if chroot((*b".\0").as_ptr()) == -1i32 {
        StrErr::die(
            111i32,
            fatal,
            (*b"unable to chroot to \0").as_ptr(),
            x as (*const u8),
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut STRERR_SYS as (*mut strerr) as (*const strerr),
        );
    }
    x = libc::getenv((*b"GID\0").as_ptr() as *const libc::c_char);
    if x.is_null() {
        StrErr::die(
            111i32,
            fatal,
            (*b"$GID not set\0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const strerr),
        );
    }
    ulong::scan(x as (*const u8), &mut id as (*mut usize));
    if prot_gid(id as (i32)) == -1i32 {
        StrErr::die(
            111i32,
            fatal,
            (*b"unable to setgid: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut STRERR_SYS as (*mut strerr) as (*const strerr),
        );
    }
    x = libc::getenv((*b"UID\0").as_ptr() as *const libc::c_char);
    if x.is_null() {
        StrErr::die(
            111i32,
            fatal,
            (*b"$UID not set\0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const strerr),
        );
    }
    ulong::scan(x as (*const u8), &mut id as (*mut usize));
    if prot_uid(id as (i32)) == -1i32 {
        StrErr::die(
            111i32,
            fatal,
            (*b"unable to setuid: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut STRERR_SYS as (*mut strerr) as (*const strerr),
        );
    }
}
