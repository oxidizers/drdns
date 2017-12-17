extern "C" {
    fn buffer_copy(arg1: *mut buffer, arg2: *mut buffer) -> i32;
    fn buffer_flush(arg1: *mut buffer) -> i32;
    fn buffer_init(
        arg1: *mut buffer,
        arg2: unsafe extern "C" fn() -> i32,
        arg3: i32,
        arg4: *mut u8,
        arg5: u32,
    );
    fn buffer_put(arg1: *mut buffer, arg2: *const u8, arg3: u32) -> i32;
    fn buffer_puts(arg1: *mut buffer, arg2: *const u8) -> i32;
    fn buffer_unixwrite(arg1: i32, arg2: *const u8, arg3: u32) -> i32;
    fn chdir(arg1: *const u8) -> i32;
    fn chmod(arg1: *const u8, arg2: u16) -> i32;
    fn chown(arg1: *const u8, arg2: u32, arg3: u32) -> i32;
    fn close(arg1: i32) -> i32;
    fn fsync(arg1: i32) -> i32;
    fn mkdir(arg1: *const u8, arg2: u16) -> i32;
    fn open_trunc(arg1: *const u8) -> i32;
    fn strerr_die(
        arg1: i32,
        arg2: *const u8,
        arg3: *const u8,
        arg4: *const u8,
        arg5: *const u8,
        arg6: *const u8,
        arg7: *const u8,
        arg8: *const strerr,
    );
    static mut strerr_sys: strerr;
    fn umask(arg1: u16) -> u16;
}

static mut fatal: *const u8 = 0 as (*const u8);

static mut dir: *const u8 = 0 as (*const u8);

static mut fn_: *const u8 = 0 as (*const u8);

static mut fd: i32 = 0i32;

static mut buf: [u8; 1024] = [0u8; 1024];

#[derive(Copy)]
#[repr(C)]
pub struct buffer {
    pub x: *mut u8,
    pub p: u32,
    pub n: u32,
    pub fd: i32,
    pub op: unsafe extern "C" fn() -> i32,
}

impl Clone for buffer {
    fn clone(&self) -> Self {
        *self
    }
}

static mut ss: buffer = buffer {
    x: 0 as (*mut u8),
    p: 0u32,
    n: 0u32,
    fd: 0i32,
    op: 0 as (unsafe extern "C" fn() -> i32),
};

#[derive(Copy)]
#[repr(C)]
pub struct strerr {
    pub who: *mut strerr,
    pub x: *const u8,
    pub y: *const u8,
    pub z: *const u8,
}

impl Clone for strerr {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn init(mut d: *const u8, mut f: *const u8) {
    dir = d;
    fatal = f;
    umask(0o22u16);
    if mkdir(dir, 0o700u16) == -1i32 {
        strerr_die(
            111i32,
            fatal,
            (*b"unable to create \0").as_ptr(),
            dir,
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr),
        );
    }
    if chmod(dir, 0o3755u16) == -1i32 {
        strerr_die(
            111i32,
            fatal,
            (*b"unable to set mode of \0").as_ptr(),
            dir,
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr),
        );
    }
    if chdir(dir) == -1i32 {
        strerr_die(
            111i32,
            fatal,
            (*b"unable to switch to \0").as_ptr(),
            dir,
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn fail() {
    strerr_die(
        111i32,
        fatal,
        (*b"unable to create \0").as_ptr(),
        dir,
        (*b"/\0").as_ptr(),
        fn_,
        (*b": \0").as_ptr(),
        &mut strerr_sys as (*mut strerr) as (*const strerr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn makedir(mut s: *const u8) {
    fn_ = s;
    if mkdir(fn_, 0o700u16) == -1i32 {
        fail();
    }
}

#[no_mangle]
pub unsafe extern "C" fn start(mut s: *const u8) {
    fn_ = s;
    fd = open_trunc(fn_);
    if fd == -1i32 {
        fail();
    }
    buffer_init(
        &mut ss as (*mut buffer),
        buffer_unixwrite as (unsafe extern "C" fn() -> i32),
        fd,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[u8; 1024]>() as (u32),
    );
}

#[no_mangle]
pub unsafe extern "C" fn outs(mut s: *const u8) {
    if buffer_puts(&mut ss as (*mut buffer), s) == -1i32 {
        fail();
    }
}

#[no_mangle]
pub unsafe extern "C" fn out(mut s: *const u8, mut len: u32) {
    if buffer_put(&mut ss as (*mut buffer), s, len) == -1i32 {
        fail();
    }
}

#[no_mangle]
pub unsafe extern "C" fn copyfrom(mut b: *mut buffer) {
    if buffer_copy(&mut ss as (*mut buffer), b) < 0i32 {
        fail();
    }
}

#[no_mangle]
pub unsafe extern "C" fn finish() {
    if buffer_flush(&mut ss as (*mut buffer)) == -1i32 {
        fail();
    }
    if fsync(fd) == -1i32 {
        fail();
    }
    close(fd);
}

#[no_mangle]
pub unsafe extern "C" fn perm(mut mode: i32) {
    if chmod(fn_, mode as (u16)) == -1i32 {
        fail();
    }
}

#[no_mangle]
pub unsafe extern "C" fn owner(mut uid: i32, mut gid: i32) {
    if chown(fn_, uid as (u32), gid as (u32)) == -1i32 {
        fail();
    }
}

#[no_mangle]
pub unsafe extern "C" fn makelog(mut user: *const u8, mut uid: i32, mut gid: i32) {
    makedir((*b"log\0").as_ptr());
    perm(0o2755i32);
    makedir((*b"log/main\0").as_ptr());
    owner(uid, gid);
    perm(0o2755i32);
    start((*b"log/status\0").as_ptr());
    finish();
    owner(uid, gid);
    perm(0o644i32);
    start((*b"log/run\0").as_ptr());
    outs((*b"#!/bin/sh\nexec\0").as_ptr());
    outs((*b" setuidgid \0").as_ptr());
    outs(user);
    outs((*b" multilog t ./main\n\0").as_ptr());
    finish();
    perm(0o755i32);
}
