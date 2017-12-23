use buffer::{Buffer, STDOUT_BUFFER};
use byte;
use dns;
use libc;
use stralloc::StrAlloc;
use strerr::{StrErr, STRERR_SYS};
use uint16;
use ulong;

#[no_mangle]
pub unsafe extern "C" fn nomem() {
    StrErr::die(
        111i32,
        (*b"dnsmx: fatal: \0").as_ptr(),
        (*b"out of memory\0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const StrErr),
    );
}

static mut seed: [u8; 128] = [0u8; 128];

static mut fqdn: StrAlloc = StrAlloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

static mut q: *mut u8 = 0 as (*mut u8);

static mut out: StrAlloc = StrAlloc {
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
    dns::random::init(seed.as_mut_ptr() as (*const u8));
    if !(*argv).is_null() {
        argv = argv.offset(1isize);
    }
    'loop2: loop {
        if (*argv).is_null() {
            break;
        }
        if StrAlloc::copys(&mut fqdn as (*mut StrAlloc), *argv as (*const u8)) == 0 {
            nomem();
        }
        if dns::mx::mx(
            &mut out as (*mut StrAlloc),
            &mut fqdn as (*mut StrAlloc) as (*const StrAlloc),
        ) == -1i32
        {
            StrErr::die(
                111i32,
                (*b"dnsmx: fatal: \0").as_ptr(),
                (*b"unable to find MX records for \0").as_ptr(),
                *argv as (*const u8),
                (*b": \0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
            );
        }
        if out.len == 0 {
            if dns::domain::fromdot(
                &mut q as (*mut *mut u8),
                *argv as (*const u8),
                libc::strlen(*argv as *const i8) as u32,
            ) == 0
            {
                nomem();
            }
            if StrAlloc::copys(&mut out as (*mut StrAlloc), (*b"0 \0").as_ptr()) == 0 {
                nomem();
            }
            if dns::domain::todot_cat(&mut out as (*mut StrAlloc), q as (*const u8)) == 0 {
                nomem();
            }
            if StrAlloc::cats(&mut out as (*mut StrAlloc), (*b"\n\0").as_ptr()) == 0 {
                nomem();
            }
            Buffer::put(STDOUT_BUFFER.as_mut_ptr(), out.s as (*const u8), out.len);
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
                    STDOUT_BUFFER.as_mut_ptr(),
                    strnum.as_mut_ptr() as (*const u8),
                    ulong::fmt(strnum.as_mut_ptr(), pref as (usize)),
                );
                Buffer::puts(STDOUT_BUFFER.as_mut_ptr(), (*b" \0").as_ptr());
                Buffer::put(
                    STDOUT_BUFFER.as_mut_ptr(),
                    out.s.offset(i as (isize)).offset(2isize) as (*const u8),
                    j as (u32),
                );
                Buffer::puts(STDOUT_BUFFER.as_mut_ptr(), (*b"\n\0").as_ptr());
                i = i + (j + 3i32);
            }
        }
        argv = argv.offset(1isize);
    }
    Buffer::flush(STDOUT_BUFFER.as_mut_ptr());
    libc::_exit(0i32);
}
