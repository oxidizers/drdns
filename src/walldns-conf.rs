extern {
    fn _exit(arg1 : i32);
    static mut auto_home : *const u8;
    fn finish();
    fn getpwnam(arg1 : *const u8) -> *mut passwd;
    fn init(arg1 : *const u8, arg2 : *const u8);
    fn makedir(arg1 : *const u8);
    fn makelog(arg1 : *const u8, arg2 : i32, arg3 : i32);
    fn outs(arg1 : *const u8);
    fn perm(arg1 : i32);
    fn start(arg1 : *const u8);
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
}

static UUID_NULL
    : [u8; 16]
    = [   0u8,
          0u8,
          0u8,
          0u8,
          0u8,
          0u8,
          0u8,
          0u8,
          0u8,
          0u8,
          0u8,
          0u8,
          0u8,
          0u8,
          0u8,
          0u8
      ];

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
pub unsafe extern fn usage() {
    strerr_die(
        100i32,
        (*b"walldns-conf: usage: walldns-conf acct logacct /walldns myip\0").as_ptr(
        ),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const strerr)
    );
}

#[no_mangle]
pub static mut dir : *mut u8 = 0 as (*mut u8);

#[no_mangle]
pub static mut user : *mut u8 = 0 as (*mut u8);

#[no_mangle]
pub static mut loguser : *mut u8 = 0 as (*mut u8);

#[derive(Copy)]
#[repr(C)]
pub struct passwd {
    pub pw_name : *mut u8,
    pub pw_passwd : *mut u8,
    pub pw_uid : u32,
    pub pw_gid : u32,
    pub pw_change : isize,
    pub pw_class : *mut u8,
    pub pw_gecos : *mut u8,
    pub pw_dir : *mut u8,
    pub pw_shell : *mut u8,
    pub pw_expire : isize,
}

impl Clone for passwd {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut pw : *mut passwd = 0 as (*mut passwd);

#[no_mangle]
pub static mut myip : *mut u8 = 0 as (*mut u8);

fn main() {
    use ::std::os::unix::ffi::OsStringExt;
    let mut argv_storage
        = ::std::env::args_os().map(
              |str| {
                        let mut vec = str.into_vec();
                        vec.push(b'\0');
                        vec
                    }
          ).collect::<Vec<_>>(
          );
    let mut argv
        = argv_storage.iter_mut().map(|vec| vec.as_mut_ptr()).chain(
              Some(::std::ptr::null_mut())
          ).collect::<Vec<_>>(
          );
    let ret
        = unsafe {
              _c_main(argv_storage.len() as (i32),argv.as_mut_ptr())
          };
    ::std::process::exit(ret);
}

#[no_mangle]
pub unsafe extern fn _c_main(
    mut argc : i32, mut argv : *mut *mut u8
) -> i32 {
    user = *argv.offset(1isize);
    if user.is_null() {
        usage();
    }
    loguser = *argv.offset(2isize);
    if loguser.is_null() {
        usage();
    }
    dir = *argv.offset(3isize);
    if dir.is_null() {
        usage();
    }
    if *dir.offset(0isize) as (i32) != b'/' as (i32) {
        usage();
    }
    myip = *argv.offset(4isize);
    if myip.is_null() {
        usage();
    }
    pw = getpwnam(loguser as (*const u8));
    if pw.is_null() {
        strerr_die(
            111i32,
            (*b"walldns-conf: fatal: \0").as_ptr(),
            (*b"unknown account \0").as_ptr(),
            loguser as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const strerr)
        );
    }
    init(dir as (*const u8),(*b"walldns-conf: fatal: \0").as_ptr());
    makelog(
        loguser as (*const u8),
        (*pw).pw_uid as (i32),
        (*pw).pw_gid as (i32)
    );
    makedir((*b"env\0").as_ptr());
    perm(0o2755i32);
    start((*b"env/ROOT\0").as_ptr());
    outs(dir as (*const u8));
    outs((*b"/root\n\0").as_ptr());
    finish();
    perm(0o644i32);
    start((*b"env/IP\0").as_ptr());
    outs(myip as (*const u8));
    outs((*b"\n\0").as_ptr());
    finish();
    perm(0o644i32);
    start((*b"run\0").as_ptr());
    outs((*b"#!/bin/sh\nexec 2>&1\nexec envuidgid \0").as_ptr());
    outs(user as (*const u8));
    outs((*b" envdir ./env softlimit -d250000 \0").as_ptr());
    outs(auto_home);
    outs((*b"/bin/walldns\n\0").as_ptr());
    finish();
    perm(0o755i32);
    makedir((*b"root\0").as_ptr());
    perm(0o2755i32);
    _exit(0i32);
    0
}
