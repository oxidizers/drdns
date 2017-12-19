use buffer::{self, Buffer};
use errno::{errno, Errno};
use libc;

extern "C" {
    static mut auto_home: *const u8;
    fn chdir(arg1: *const u8) -> i32;
    fn copyfrom(arg1: *mut Buffer);
    fn finish();
    fn getgid() -> u32;
    fn getpid() -> i32;
    fn getppid() -> i32;
    fn getpwnam(arg1: *const u8) -> *mut passwd;
    fn getuid() -> u32;
    fn init(arg1: *const u8, arg2: *const u8);
    fn makedir(arg1: *const u8);
    fn open_read(arg1: *const u8) -> i32;
    fn out(arg1: *const u8, arg2: u32);
    fn outs(arg1: *const u8);
    fn owner(arg1: i32, arg2: i32);
    fn perm(arg1: i32);
    fn start(arg1: *const u8);
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
    fn taia_now(arg1: *mut TaiA);
    fn taia_pack(arg1: *mut u8, arg2: *const TaiA);
}

static UUID_NULL: [u8; 16] = [
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
    0u8,
    0u8,
];

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
pub unsafe extern "C" fn usage() {
    strerr_die(
        100i32,
        (*b"dnscache-conf: usage: dnscache-conf acct logacct /dnscache [ myip ]\0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const strerr),
    );
}

#[no_mangle]
pub static mut fdrootservers: i32 = 0i32;

#[no_mangle]
pub static mut rootserversbuf: [u8; 64] = [0u8; 64];

#[no_mangle]
pub static mut ssrootservers: Buffer = Buffer {
    x: 0 as (*mut u8),
    p: 0u32,
    n: 0u32,
    fd: 0i32,
    op: None,
};

#[no_mangle]
pub static mut dir: *mut u8 = 0 as (*mut u8);

#[no_mangle]
pub static mut user: *mut u8 = 0 as (*mut u8);

#[no_mangle]
pub static mut loguser: *mut u8 = 0 as (*mut u8);

#[derive(Copy)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut u8,
    pub pw_passwd: *mut u8,
    pub pw_uid: u32,
    pub pw_gid: u32,
    pub pw_change: isize,
    pub pw_class: *mut u8,
    pub pw_gecos: *mut u8,
    pub pw_dir: *mut u8,
    pub pw_shell: *mut u8,
    pub pw_expire: isize,
}

impl Clone for passwd {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub static mut pw: *mut passwd = 0 as (*mut passwd);

#[no_mangle]
pub static mut myip: *const u8 = 0 as (*const u8);

#[no_mangle]
pub static mut seed: [u32; 32] = [0u32; 32];

#[no_mangle]
pub static mut seedpos: i32 = 0i32;

#[no_mangle]
pub unsafe extern "C" fn seed_adduint32(mut u: u32) {
    let mut i: i32;
    let _rhs = u;
    let _lhs = &mut seed[seedpos as (usize)];
    *_lhs = (*_lhs).wrapping_add(_rhs);
    if {
        seedpos = seedpos + 1;
        seedpos
    } == 32i32
    {
        i = 0i32;
        'loop2: loop {
            if !(i < 32i32) {
                break;
            }
            u = (u ^ seed[i as (usize)]).wrapping_add(0x9e3779b9u32) ^ u << 7i32 ^ u >> 25i32;
            seed[i as (usize)] = u;
            i = i + 1;
        }
        seedpos = 0i32;
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct tai {
    pub x: usize,
}

impl Clone for tai {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct taia {
    pub sec: Tai,
    pub nano: usize,
    pub atto: usize,
}

impl Clone for taia {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn seed_addtime() {
    let mut t: TaiA;
    let mut tpack: [u8; 16];
    let mut i: i32;
    taia_now(&mut t as (*mut TaiA));
    taia_pack(tpack.as_mut_ptr(), &mut t as (*mut TaiA) as (*const TaiA));
    i = 0i32;
    'loop1: loop {
        if !(i < 16i32) {
            break;
        }
        seed_adduint32(tpack[i as (usize)] as (u32));
        i = i + 1;
    }
}

fn main() {
    use std::os::unix::ffi::OsStringExt;
    let mut argv_storage = ::std::env::args_os()
        .map(|str| {
            let mut vec = str.into_vec();
            vec.push(b'\0');
            vec
        })
        .collect::<Vec<_>>();
    let mut argv = argv_storage
        .iter_mut()
        .map(|vec| vec.as_mut_ptr())
        .chain(Some(::std::ptr::null_mut()))
        .collect::<Vec<_>>();
    let ret = unsafe { _c_main(argv_storage.len() as (i32), argv.as_mut_ptr()) };
    ::std::process::exit(ret);
}

#[no_mangle]
pub unsafe extern "C" fn _c_main(mut argc: i32, mut argv: *mut *mut u8) -> i32 {
    seed_addtime();
    seed_adduint32(getpid() as (u32));
    seed_adduint32(getppid() as (u32));
    seed_adduint32(getuid());
    seed_adduint32(getgid());
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
    myip = *argv.offset(4isize) as (*const u8);
    if myip.is_null() {
        myip = (*b"127.0.0.1\0").as_ptr();
    }
    pw = getpwnam(loguser as (*const u8));
    seed_addtime();
    if pw.is_null() {
        strerr_die(
            111i32,
            (*b"dnscache-conf: fatal: \0").as_ptr(),
            (*b"unknown account \0").as_ptr(),
            loguser as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const strerr),
        );
    }
    if chdir(auto_home) == -1i32 {
        strerr_die(
            111i32,
            (*b"dnscache-conf: fatal: \0").as_ptr(),
            (*b"unable to switch to \0").as_ptr(),
            auto_home,
            (*b": \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr),
        );
    }
    fdrootservers = open_read((*b"/etc/dnsroots.local\0").as_ptr());
    if fdrootservers == -1i32 {
        if errno() != Errno(libc::ENOENT) {
            strerr_die(
                111i32,
                (*b"dnscache-conf: fatal: \0").as_ptr(),
                (*b"unable to open /etc/dnsroots.local: \0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                &mut strerr_sys as (*mut strerr) as (*const strerr),
            );
        }
        fdrootservers = open_read((*b"/etc/dnsroots.global\0").as_ptr());
        if fdrootservers == -1i32 {
            strerr_die(
                111i32,
                (*b"dnscache-conf: fatal: \0").as_ptr(),
                (*b"unable to open /etc/dnsroots.global: \0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                &mut strerr_sys as (*mut strerr) as (*const strerr),
            );
        }
    }
    init(dir as (*const u8), (*b"dnscache-conf: fatal: \0").as_ptr());
    seed_addtime();
    makedir((*b"log\0").as_ptr());
    seed_addtime();
    perm(0o2755i32);
    seed_addtime();
    makedir((*b"log/main\0").as_ptr());
    seed_addtime();
    owner((*pw).pw_uid as (i32), (*pw).pw_gid as (i32));
    seed_addtime();
    perm(0o2755i32);
    seed_addtime();
    start((*b"log/status\0").as_ptr());
    finish();
    seed_addtime();
    owner((*pw).pw_uid as (i32), (*pw).pw_gid as (i32));
    seed_addtime();
    perm(0o644i32);
    seed_addtime();
    makedir((*b"env\0").as_ptr());
    seed_addtime();
    perm(0o2755i32);
    seed_addtime();
    start((*b"env/ROOT\0").as_ptr());
    outs(dir as (*const u8));
    outs((*b"/root\n\0").as_ptr());
    finish();
    seed_addtime();
    perm(0o644i32);
    seed_addtime();
    start((*b"env/IP\0").as_ptr());
    outs(myip);
    outs((*b"\n\0").as_ptr());
    finish();
    seed_addtime();
    perm(0o644i32);
    seed_addtime();
    start((*b"env/IPSEND\0").as_ptr());
    outs((*b"0.0.0.0\n\0").as_ptr());
    finish();
    seed_addtime();
    perm(0o644i32);
    seed_addtime();
    start((*b"env/CACHESIZE\0").as_ptr());
    outs((*b"1000000\n\0").as_ptr());
    finish();
    seed_addtime();
    perm(0o644i32);
    seed_addtime();
    start((*b"env/DATALIMIT\0").as_ptr());
    outs((*b"3000000\n\0").as_ptr());
    finish();
    seed_addtime();
    perm(0o644i32);
    seed_addtime();
    start((*b"run\0").as_ptr());
    outs(
        (*b"#!/bin/sh\nexec 2>&1\nexec <seed\nexec envdir ./env sh -c \'\n  exec envuidgid \0")
            .as_ptr(),
    );
    outs(user as (*const u8));
    outs((*b" softlimit -o250 -d \"$DATALIMIT\" \0").as_ptr());
    outs(auto_home);
    outs((*b"/bin/dnscache\n\'\n\0").as_ptr());
    finish();
    seed_addtime();
    perm(0o755i32);
    seed_addtime();
    start((*b"log/run\0").as_ptr());
    outs((*b"#!/bin/sh\nexec setuidgid \0").as_ptr());
    outs(loguser as (*const u8));
    outs((*b" multilog t ./main\n\0").as_ptr());
    finish();
    seed_addtime();
    perm(0o755i32);
    seed_addtime();
    makedir((*b"root\0").as_ptr());
    seed_addtime();
    perm(0o2755i32);
    seed_addtime();
    makedir((*b"root/ip\0").as_ptr());
    seed_addtime();
    perm(0o2755i32);
    seed_addtime();
    start((*b"root/ip/127.0.0.1\0").as_ptr());
    finish();
    seed_addtime();
    perm(0o600i32);
    seed_addtime();
    makedir((*b"root/servers\0").as_ptr());
    seed_addtime();
    perm(0o2755i32);
    seed_addtime();
    start((*b"root/servers/@\0").as_ptr());
    Buffer::init(
        &mut ssrootservers as (*mut Buffer),
        buffer::unixread as buffer::Op,
        fdrootservers,
        rootserversbuf.as_mut_ptr(),
        ::std::mem::size_of::<[u8; 64]>() as (u32),
    );
    copyfrom(&mut ssrootservers as (*mut Buffer));
    finish();
    seed_addtime();
    perm(0o644i32);
    seed_addtime();
    start((*b"seed\0").as_ptr());
    out(seed.as_mut_ptr() as (*mut u8) as (*const u8), 128u32);
    finish();
    perm(0o600i32);
    libc::_exit(0i32);
}
