use buffer::Buffer;
use byte;
use libc;
use uint16;

extern "C" {
    static mut buffer_1: *mut Buffer;
    fn dns_domain_fromdot(arg1: *mut *mut u8, arg2: *const u8, arg3: u32) -> i32;
    fn dns_domain_todot_cat(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn dns_mx(arg1: *mut stralloc, arg2: *const stralloc) -> i32;
    fn dns_random_init(arg1: *const u8);
    fn fmt_ulong(arg1: *mut u8, arg2: usize) -> u32;
    fn stralloc_cats(arg1: *mut stralloc, arg2: *const u8) -> i32;
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
pub unsafe extern "C" fn nomem() {
    strerr_die(
        111i32,
        (*b"dnsmx: fatal: \0").as_ptr(),
        (*b"out of memory\0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const strerr),
    );
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

static mut fqdn: stralloc = stralloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

static mut q: *mut u8 = 0 as (*mut u8);

static mut out: stralloc = stralloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

#[no_mangle]
pub static mut strnum: [u8; 40] = [0u8; 40];

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
    let mut i: i32;
    let mut j: i32;
    let mut pref: u16;
    dns_random_init(seed.as_mut_ptr() as (*const u8));
    if !(*argv).is_null() {
        argv = argv.offset(1isize);
    }
    'loop2: loop {
        if (*argv).is_null() {
            break;
        }
        if stralloc_copys(&mut fqdn as (*mut stralloc), *argv as (*const u8)) == 0 {
            nomem();
        }
        if dns_mx(
            &mut out as (*mut stralloc),
            &mut fqdn as (*mut stralloc) as (*const stralloc),
        ) == -1i32
        {
            strerr_die(
                111i32,
                (*b"dnsmx: fatal: \0").as_ptr(),
                (*b"unable to find MX records for \0").as_ptr(),
                *argv as (*const u8),
                (*b": \0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                &mut strerr_sys as (*mut strerr) as (*const strerr),
            );
        }
        if out.len == 0 {
            if dns_domain_fromdot(
                &mut q as (*mut *mut u8),
                *argv as (*const u8),
                libc::strlen(*argv as *const i8) as u32,
            ) == 0
            {
                nomem();
            }
            if stralloc_copys(&mut out as (*mut stralloc), (*b"0 \0").as_ptr()) == 0 {
                nomem();
            }
            if dns_domain_todot_cat(&mut out as (*mut stralloc), q as (*const u8)) == 0 {
                nomem();
            }
            if stralloc_cats(&mut out as (*mut stralloc), (*b"\n\0").as_ptr()) == 0 {
                nomem();
            }
            Buffer::put(buffer_1, out.s as (*const u8), out.len);
        } else {
            i = 0i32;
            'loop10: loop {
                if !((i + 2i32) as (u32) < out.len) {
                    break;
                }
                j = byte::chr(
                    out.s.offset(i as (isize)).offset(2isize),
                    out.len.wrapping_sub(i as (u32)).wrapping_sub(2u32),
                    0i32,
                ) as (i32);
                uint16::unpack_big(
                    out.s.offset(i as (isize)) as (*const u8),
                    &mut pref as (*mut u16),
                );
                Buffer::put(
                    buffer_1,
                    strnum.as_mut_ptr() as (*const u8),
                    fmt_ulong(strnum.as_mut_ptr(), pref as (usize)),
                );
                Buffer::puts(buffer_1, (*b" \0").as_ptr());
                Buffer::put(
                    buffer_1,
                    out.s.offset(i as (isize)).offset(2isize) as (*const u8),
                    j as (u32),
                );
                Buffer::puts(buffer_1, (*b"\n\0").as_ptr());
                i = i + (j + 3i32);
            }
        }
        argv = argv.offset(1isize);
    }
    Buffer::flush(buffer_1);
    libc::_exit(0i32);
}
