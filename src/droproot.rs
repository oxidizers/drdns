extern {
    fn chdir(arg1 : *const u8) -> i32;
    fn chroot(arg1 : *const u8) -> i32;
    fn env_get(arg1 : *const u8) -> *mut u8;
    fn prot_gid(arg1 : i32) -> i32;
    fn prot_uid(arg1 : i32) -> i32;
    fn scan_ulong(arg1 : *const u8, arg2 : *mut usize) -> u32;
    fn strerr_die(
        arg1 : i32,
        arg2 : *const u8,
        arg3 : *const u8,
        arg4 : *const u8,
        arg5 : *const u8,
        arg6 : *const u8,
        arg7 : *const u8,
        arg8 : *const strerr
    );
    static mut strerr_sys : strerr;
}

#[derive(Copy)]
#[repr(C)]
pub struct strerr {
    pub who : *mut strerr,
    pub x : *const u8,
    pub y : *const u8,
    pub z : *const u8,
}

impl Clone for strerr {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn droproot(mut fatal : *const u8) {
    let mut x : *mut u8;
    let mut id : usize;
    x = env_get((*b"ROOT\0").as_ptr());
    if x.is_null() {
        strerr_die(
            111i32,
            fatal,
            (*b"$ROOT not set\0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const strerr)
        );
    }
    if chdir(x as (*const u8)) == -1i32 {
        strerr_die(
            111i32,
            fatal,
            (*b"unable to chdir to \0").as_ptr(),
            x as (*const u8),
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    if chroot((*b".\0").as_ptr()) == -1i32 {
        strerr_die(
            111i32,
            fatal,
            (*b"unable to chroot to \0").as_ptr(),
            x as (*const u8),
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    x = env_get((*b"GID\0").as_ptr());
    if x.is_null() {
        strerr_die(
            111i32,
            fatal,
            (*b"$GID not set\0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const strerr)
        );
    }
    scan_ulong(x as (*const u8),&mut id as (*mut usize));
    if prot_gid(id as (i32)) == -1i32 {
        strerr_die(
            111i32,
            fatal,
            (*b"unable to setgid: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    x = env_get((*b"UID\0").as_ptr());
    if x.is_null() {
        strerr_die(
            111i32,
            fatal,
            (*b"$UID not set\0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const strerr)
        );
    }
    scan_ulong(x as (*const u8),&mut id as (*mut usize));
    if prot_uid(id as (i32)) == -1i32 {
        strerr_die(
            111i32,
            fatal,
            (*b"unable to setuid: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
}
