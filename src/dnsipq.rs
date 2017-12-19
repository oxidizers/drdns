use buffer::Buffer;
use libc;

extern "C" {
    static mut buffer_1: *mut Buffer;
    fn dns_ip4_qualify(arg1: *mut stralloc, arg2: *mut stralloc, arg3: *const stralloc) -> i32;
    fn dns_random_init(arg1: *const u8);
    fn ip4_fmt(arg1: *mut u8, arg2: *const u8) -> u32;
    fn stralloc_copys(arg1: *mut stralloc, arg2: *const u8) -> i32;
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
}

static mut seed: [u8; 128] = [0u8; 128];

#[derive(Copy)]
#[repr(C)]
pub struct stralloc {
    pub s: *mut u8,
    pub len: u32,
    pub a: u32,
}

impl Clone for stralloc {
    fn clone(&self) -> Self {
        *self
    }
}

static mut in_: stralloc = stralloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

static mut fqdn: stralloc = stralloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

static mut out: stralloc = stralloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

#[no_mangle]
pub static mut str: [u8; 20] = [0u8; 20];

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
pub unsafe extern "C" fn _c_main(mut argc: i32, mut argv: *mut *mut u8) -> i32 {
    let mut i: i32;
    dns_random_init(seed.as_mut_ptr() as (*const u8));
    if !(*argv).is_null() {
        argv = argv.offset(1isize);
    }
    'loop2: loop {
        if (*argv).is_null() {
            break;
        }
        if stralloc_copys(&mut in_ as (*mut stralloc), *argv as (*const u8)) == 0 {
            strerr_die(
                111i32,
                (*b"dnsipq: fatal: \0").as_ptr(),
                (*b"out of memory\0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const strerr),
            );
        }
        if dns_ip4_qualify(
            &mut out as (*mut stralloc),
            &mut fqdn as (*mut stralloc),
            &mut in_ as (*mut stralloc) as (*const stralloc),
        ) == -1i32
        {
            strerr_die(
                111i32,
                (*b"dnsipq: fatal: \0").as_ptr(),
                (*b"unable to find IP address for \0").as_ptr(),
                *argv as (*const u8),
                (*b": \0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                &mut strerr_sys as (*mut strerr) as (*const strerr),
            );
        }
        Buffer::put(buffer_1, fqdn.s as (*const u8), fqdn.len);
        Buffer::puts(buffer_1, (*b" \0").as_ptr());
        i = 0i32;
        'loop9: loop {
            if !((i + 4i32) as (u32) <= out.len) {
                break;
            }
            Buffer::put(
                buffer_1,
                str.as_mut_ptr() as (*const u8),
                ip4_fmt(str.as_mut_ptr(), out.s.offset(i as (isize)) as (*const u8)),
            );
            Buffer::puts(buffer_1, (*b" \0").as_ptr());
            i = i + 4i32;
        }
        Buffer::puts(buffer_1, (*b"\n\0").as_ptr());
        argv = argv.offset(1isize);
    }
    Buffer::flush(buffer_1);
    libc::_exit(0i32);
}
