use buffer::{self, Buffer};
use open;
use strerr::{StrErr, STRERR_SYS};

extern "C" {
    fn chdir(arg1: *const u8) -> i32;
    fn chmod(arg1: *const u8, arg2: u16) -> i32;
    fn chown(arg1: *const u8, arg2: u32, arg3: u32) -> i32;
    fn close(arg1: i32) -> i32;
    fn fsync(arg1: i32) -> i32;
    fn mkdir(arg1: *const u8, arg2: u16) -> i32;
    fn umask(arg1: u16) -> u16;
}

static mut fatal: *const u8 = 0 as (*const u8);

static mut dir: *const u8 = 0 as (*const u8);

static mut filename: *const u8 = 0 as (*const u8);

static mut fd: i32 = 0i32;

static mut buf: [u8; 1024] = [0u8; 1024];

static mut ss: Buffer = Buffer {
    x: 0 as (*mut u8),
    p: 0u32,
    n: 0u32,
    fd: 0i32,
    op: None,
};

#[no_mangle]
pub unsafe extern "C" fn init(mut d: *const u8, mut f: *const u8) {
    dir = d;
    fatal = f;
    umask(0o22u16);
    if mkdir(dir, 0o700u16) == -1i32 {
        StrErr::die(
            111i32,
            fatal,
            (*b"unable to create \0").as_ptr(),
            dir,
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
        );
    }
    if chmod(dir, 0o3755u16) == -1i32 {
        StrErr::die(
            111i32,
            fatal,
            (*b"unable to set mode of \0").as_ptr(),
            dir,
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
        );
    }
    if chdir(dir) == -1i32 {
        StrErr::die(
            111i32,
            fatal,
            (*b"unable to switch to \0").as_ptr(),
            dir,
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn fail() {
    StrErr::die(
        111i32,
        fatal,
        (*b"unable to create \0").as_ptr(),
        dir,
        (*b"/\0").as_ptr(),
        filename,
        (*b": \0").as_ptr(),
        &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn makedir(mut s: *const u8) {
    filename = s;
    if mkdir(filename, 0o700u16) == -1i32 {
        fail();
    }
}

#[no_mangle]
pub unsafe extern "C" fn start(mut s: *const u8) {
    filename = s;
    fd = open::trunc(filename);
    if fd == -1i32 {
        fail();
    }
    Buffer::init(
        &mut ss as (*mut Buffer),
        buffer::unixwrite as buffer::Op,
        fd,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[u8; 1024]>() as (u32),
    );
}

#[no_mangle]
pub unsafe extern "C" fn outs(mut s: *const u8) {
    if Buffer::puts(&mut ss as (*mut Buffer), s) == -1i32 {
        fail();
    }
}

#[no_mangle]
pub unsafe extern "C" fn out(mut s: *const u8, mut len: u32) {
    if Buffer::put(&mut ss as (*mut Buffer), s, len) == -1i32 {
        fail();
    }
}

#[no_mangle]
pub unsafe extern "C" fn copyfrom(mut b: *mut Buffer) {
    if Buffer::copy(&mut ss as (*mut Buffer), b) < 0i32 {
        fail();
    }
}

#[no_mangle]
pub unsafe extern "C" fn finish() {
    if Buffer::flush(&mut ss as (*mut Buffer)) == -1i32 {
        fail();
    }
    if fsync(fd) == -1i32 {
        fail();
    }
    close(fd);
}

#[no_mangle]
pub unsafe extern "C" fn perm(mut mode: i32) {
    if chmod(filename, mode as (u16)) == -1i32 {
        fail();
    }
}

#[no_mangle]
pub unsafe extern "C" fn owner(mut uid: i32, mut gid: i32) {
    if chown(filename, uid as (u32), gid as (u32)) == -1i32 {
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
