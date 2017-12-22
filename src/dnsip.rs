use buffer::{Buffer, STDOUT_BUFFER};
use ip4;
use libc;
use stralloc::StrAlloc;
use strerr::{StrErr, STRERR_SYS};

extern "C" {
    fn dns_ip4(arg1: *mut StrAlloc, arg2: *const StrAlloc) -> i32;
    fn dns_random_init(arg1: *const u8);
}

static mut seed: [u8; 128] = [0u8; 128];

static mut fqdn: StrAlloc = StrAlloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

static mut out: StrAlloc = StrAlloc {
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
        if StrAlloc::copys(&mut fqdn as (*mut StrAlloc), *argv as (*const u8)) == 0 {
            StrErr::die(
                111i32,
                (*b"dnsip: fatal: \0").as_ptr(),
                (*b"out of memory\0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const StrErr),
            );
        }
        if dns_ip4(
            &mut out as (*mut StrAlloc),
            &mut fqdn as (*mut StrAlloc) as (*const StrAlloc),
        ) == -1i32
        {
            StrErr::die(
                111i32,
                (*b"dnsip: fatal: \0").as_ptr(),
                (*b"unable to find IP address for \0").as_ptr(),
                *argv as (*const u8),
                (*b": \0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
            );
        }
        i = 0i32;
        'loop9: loop {
            if !((i + 4i32) as (u32) <= out.len) {
                break;
            }
            Buffer::put(
                STDOUT_BUFFER.as_mut_ptr(),
                str.as_mut_ptr() as (*const u8),
                ip4::fmt(str.as_mut_ptr(), out.s.offset(i as (isize)) as (*const u8)),
            );
            Buffer::puts(STDOUT_BUFFER.as_mut_ptr(), (*b" \0").as_ptr());
            i = i + 4i32;
        }
        Buffer::puts(STDOUT_BUFFER.as_mut_ptr(), (*b"\n\0").as_ptr());
        argv = argv.offset(1isize);
    }
    Buffer::flush(STDOUT_BUFFER.as_mut_ptr());
    libc::_exit(0i32);
}
