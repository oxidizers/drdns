extern {
    fn _exit(arg1 : i32);
    fn chdir(arg1 : *const u8) -> i32;
    static mut errno : i32;
    static mut error_noent : i32;
    fn hier();
    fn stat(arg1 : *const u8, arg2 : *mut stat) -> i32;
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
    fn strerr_warn(
        arg1 : *const u8,
        arg2 : *const u8,
        arg3 : *const u8,
        arg4 : *const u8,
        arg5 : *const u8,
        arg6 : *const u8,
        arg7 : *const strerr
    );
}

#[derive(Copy)]
#[repr(C)]
pub struct timespec {
    pub tv_sec : isize,
    pub tv_nsec : isize,
}

impl Clone for timespec {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct stat {
    pub st_dev : i32,
    pub st_mode : u16,
    pub st_nlink : u16,
    pub st_ino : usize,
    pub st_uid : u32,
    pub st_gid : u32,
    pub st_rdev : i32,
    pub st_atimespec : timespec,
    pub st_mtimespec : timespec,
    pub st_ctimespec : timespec,
    pub st_birthtimespec : timespec,
    pub st_size : isize,
    pub st_blocks : isize,
    pub st_blksize : i32,
    pub st_flags : u32,
    pub st_gen : u32,
    pub st_lspare : i32,
    pub st_qspare : [isize; 2],
}

impl Clone for stat {
    fn clone(&self) -> Self { *self }
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
pub unsafe extern fn perm(
    mut prefix1 : *mut u8,
    mut prefix2 : *mut u8,
    mut prefix3 : *mut u8,
    mut file : *mut u8,
    mut type_ : i32,
    mut uid : i32,
    mut gid : i32,
    mut mode : i32
) {
    let mut st : stat;
    if stat(file as (*const u8),&mut st as (*mut stat)) == -1i32 {
        if errno == error_noent {
            strerr_warn(
                (*b"instcheck: warning: \0").as_ptr(),
                prefix1 as (*const u8),
                prefix2 as (*const u8),
                prefix3 as (*const u8),
                file as (*const u8),
                (*b" does not exist\0").as_ptr(),
                0i32 as (*const strerr)
            );
        } else {
            strerr_warn(
                (*b"instcheck: warning: \0").as_ptr(),
                (*b"unable to stat .../\0").as_ptr(),
                file as (*const u8),
                (*b": \0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                &mut strerr_sys as (*mut strerr) as (*const strerr)
            );
        }
    } else {
        if uid != -1i32 && (st.st_uid != uid as (u32)) {
            strerr_warn(
                (*b"instcheck: warning: \0").as_ptr(),
                prefix1 as (*const u8),
                prefix2 as (*const u8),
                prefix3 as (*const u8),
                file as (*const u8),
                (*b" has wrong owner\0").as_ptr(),
                0i32 as (*const strerr)
            );
        }
        if gid != -1i32 && (st.st_gid != gid as (u32)) {
            strerr_warn(
                (*b"instcheck: warning: \0").as_ptr(),
                prefix1 as (*const u8),
                prefix2 as (*const u8),
                prefix3 as (*const u8),
                file as (*const u8),
                (*b" has wrong group\0").as_ptr(),
                0i32 as (*const strerr)
            );
        }
        if st.st_mode as (i32) & 0o7777i32 != mode {
            strerr_warn(
                (*b"instcheck: warning: \0").as_ptr(),
                prefix1 as (*const u8),
                prefix2 as (*const u8),
                prefix3 as (*const u8),
                file as (*const u8),
                (*b" has wrong permissions\0").as_ptr(),
                0i32 as (*const strerr)
            );
        }
        if st.st_mode as (i32) & 0o170000i32 != type_ {
            strerr_warn(
                (*b"instcheck: warning: \0").as_ptr(),
                prefix1 as (*const u8),
                prefix2 as (*const u8),
                prefix3 as (*const u8),
                file as (*const u8),
                (*b" has wrong type\0").as_ptr(),
                0i32 as (*const strerr)
            );
        }
    }
}

#[no_mangle]
pub unsafe extern fn h(
    mut home : *mut u8, mut uid : i32, mut gid : i32, mut mode : i32
) {
    perm(
        (*b"\0").as_ptr() as (*mut u8),
        (*b"\0").as_ptr() as (*mut u8),
        (*b"\0").as_ptr() as (*mut u8),
        home,
        0o40000i32,
        uid,
        gid,
        mode
    );
}

#[no_mangle]
pub unsafe extern fn d(
    mut home : *mut u8,
    mut subdir : *mut u8,
    mut uid : i32,
    mut gid : i32,
    mut mode : i32
) {
    if chdir(home as (*const u8)) == -1i32 {
        strerr_die(
            111i32,
            (*b"instcheck: fatal: \0").as_ptr(),
            (*b"unable to switch to \0").as_ptr(),
            home as (*const u8),
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    perm(
        (*b"\0").as_ptr() as (*mut u8),
        home,
        (*b"/\0").as_ptr() as (*mut u8),
        subdir,
        0o40000i32,
        uid,
        gid,
        mode
    );
}

#[no_mangle]
pub unsafe extern fn p(
    mut home : *mut u8,
    mut fifo : *mut u8,
    mut uid : i32,
    mut gid : i32,
    mut mode : i32
) {
    if chdir(home as (*const u8)) == -1i32 {
        strerr_die(
            111i32,
            (*b"instcheck: fatal: \0").as_ptr(),
            (*b"unable to switch to \0").as_ptr(),
            home as (*const u8),
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    perm(
        (*b"\0").as_ptr() as (*mut u8),
        home,
        (*b"/\0").as_ptr() as (*mut u8),
        fifo,
        0o10000i32,
        uid,
        gid,
        mode
    );
}

#[no_mangle]
pub unsafe extern fn c(
    mut home : *mut u8,
    mut subdir : *mut u8,
    mut file : *mut u8,
    mut uid : i32,
    mut gid : i32,
    mut mode : i32
) {
    if chdir(home as (*const u8)) == -1i32 {
        strerr_die(
            111i32,
            (*b"instcheck: fatal: \0").as_ptr(),
            (*b"unable to switch to \0").as_ptr(),
            home as (*const u8),
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    if chdir(subdir as (*const u8)) == -1i32 {
        strerr_die(
            111i32,
            (*b"instcheck: fatal: \0").as_ptr(),
            (*b"unable to switch to \0").as_ptr(),
            home as (*const u8),
            (*b"/\0").as_ptr(),
            subdir as (*const u8),
            (*b": \0").as_ptr(),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    perm(
        (*b".../\0").as_ptr() as (*mut u8),
        subdir,
        (*b"/\0").as_ptr() as (*mut u8),
        file,
        0o100000i32,
        uid,
        gid,
        mode
    );
}

#[no_mangle]
pub unsafe extern fn z(
    mut home : *mut u8,
    mut file : *mut u8,
    mut len : i32,
    mut uid : i32,
    mut gid : i32,
    mut mode : i32
) {
    if chdir(home as (*const u8)) == -1i32 {
        strerr_die(
            111i32,
            (*b"instcheck: fatal: \0").as_ptr(),
            (*b"unable to switch to \0").as_ptr(),
            home as (*const u8),
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    perm(
        (*b"\0").as_ptr() as (*mut u8),
        home,
        (*b"/\0").as_ptr() as (*mut u8),
        file,
        0o100000i32,
        uid,
        gid,
        mode
    );
}

fn main() {
    let ret = unsafe { _c_main() };
    ::std::process::exit(ret);
}

#[no_mangle]
pub unsafe extern fn _c_main() -> i32 {
    hier();
    _exit(0i32);
    0
}
