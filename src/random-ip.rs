use buffer::{Buffer, STDOUT_BUFFER};
use libc;
use ulong;

extern "C" {
    fn dns_random(arg1: u32) -> u32;
    fn dns_random_init(arg1: *const u8);
}

#[no_mangle]
pub static mut ip: [u8; 4] = [0u8; 4];

#[no_mangle]
pub static mut ipfixed: i32 = 0i32;

#[no_mangle]
pub static mut loops: usize = 10000usize;

#[no_mangle]
pub static mut tab: [u8; 256] = [0u8; 256];

#[no_mangle]
pub static mut strnum: [u8; 40] = [0u8; 40];

#[no_mangle]
pub static mut seed: [u8; 128] = [0u8; 128];

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
    let mut u: usize;
    let mut i: i32;
    let mut j: i32;
    let mut c: u8;
    dns_random_init(seed.as_mut_ptr() as (*const u8));
    i = 0i32;
    'loop1: loop {
        if !(i < 256i32) {
            break;
        }
        tab[i as (usize)] = i as (u8);
        i = i + 1;
    }
    j = 256i32;
    'loop3: loop {
        if !(j > 0i32) {
            break;
        }
        i = dns_random(j as (u32)) as (i32);
        c = tab[(j - 1i32) as (usize)];
        tab[(j - 1i32) as (usize)] = tab[i as (usize)];
        tab[i as (usize)] = c;
        j = j - 1;
    }
    if !(*argv).is_null() {
        argv = argv.offset(1isize);
    }
    if !(*argv).is_null() {
        ulong::scan(
            *{
                let _old = argv;
                argv = argv.offset(1isize);
                _old
            } as (*const u8),
            &mut loops as (*mut usize),
        );
    }
    if !(*argv).is_null() {
        ulong::scan(
            *{
                let _old = argv;
                argv = argv.offset(1isize);
                _old
            } as (*const u8),
            &mut u as (*mut usize),
        );
        ip[0usize] = u as (u8);
        ipfixed = 1i32;
    }
    if !(*argv).is_null() {
        ulong::scan(
            *{
                let _old = argv;
                argv = argv.offset(1isize);
                _old
            } as (*const u8),
            &mut u as (*mut usize),
        );
        ip[1usize] = u as (u8);
        ipfixed = 2i32;
    }
    if !(*argv).is_null() {
        ulong::scan(
            *{
                let _old = argv;
                argv = argv.offset(1isize);
                _old
            } as (*const u8),
            &mut u as (*mut usize),
        );
        ip[2usize] = u as (u8);
        ipfixed = 3i32;
    }
    if !(*argv).is_null() {
        ulong::scan(
            *{
                let _old = argv;
                argv = argv.offset(1isize);
                _old
            } as (*const u8),
            &mut u as (*mut usize),
        );
        ip[3usize] = u as (u8);
        ipfixed = 4i32;
    }
    if ipfixed >= 1i32 {
        if loops > 16777216usize {
            loops = 16777216usize;
        }
    }
    if ipfixed >= 2i32 {
        if loops > 65536usize {
            loops = 65536usize;
        }
    }
    if ipfixed >= 3i32 {
        if loops > 256usize {
            loops = 256usize;
        }
    }
    if ipfixed >= 4i32 {
        if loops > 1usize {
            loops = 1usize;
        }
    }
    'loop28: loop {
        if loops == 0 {
            break;
        }
        loops = loops.wrapping_sub(1usize);
        u = loops;
        i = ipfixed;
        'loop31: loop {
            if !(i < 4i32) {
                break;
            }
            ip[i as (usize)] = (u & 255usize) as (u8);
            u = u >> 8i32;
            i = i + 1;
        }
        if ipfixed == 3i32 {
            c = ip[3usize];
            ip[3usize] = tab[c as (usize)];
        } else if ipfixed < 3i32 {
            c = 0u8;
            j = 0i32;
            'loop35: loop {
                if !(j < 100i32) {
                    break;
                }
                i = ipfixed;
                'loop37: loop {
                    if !(i < 4i32) {
                        break;
                    }
                    c = (c as (i32) ^ ip[i as (usize)] as (i32)) as (u8);
                    c = tab[c as (usize)];
                    ip[i as (usize)] = c;
                    i = i + 1;
                }
                j = j + 1;
            }
        }
        u = ip[0usize] as (usize);
        Buffer::put(
            STDOUT_BUFFER.as_mut_ptr(),
            strnum.as_mut_ptr() as (*const u8),
            ulong::fmt(strnum.as_mut_ptr(), u),
        );
        Buffer::puts(STDOUT_BUFFER.as_mut_ptr(), (*b".\0").as_ptr());
        u = ip[1usize] as (usize);
        Buffer::put(
            STDOUT_BUFFER.as_mut_ptr(),
            strnum.as_mut_ptr() as (*const u8),
            ulong::fmt(strnum.as_mut_ptr(), u),
        );
        Buffer::puts(STDOUT_BUFFER.as_mut_ptr(), (*b".\0").as_ptr());
        u = ip[2usize] as (usize);
        Buffer::put(
            STDOUT_BUFFER.as_mut_ptr(),
            strnum.as_mut_ptr() as (*const u8),
            ulong::fmt(strnum.as_mut_ptr(), u),
        );
        Buffer::puts(STDOUT_BUFFER.as_mut_ptr(), (*b".\0").as_ptr());
        u = ip[3usize] as (usize);
        Buffer::put(
            STDOUT_BUFFER.as_mut_ptr(),
            strnum.as_mut_ptr() as (*const u8),
            ulong::fmt(strnum.as_mut_ptr(), u),
        );
        Buffer::puts(STDOUT_BUFFER.as_mut_ptr(), (*b"\n\0").as_ptr());
    }
    Buffer::flush(STDOUT_BUFFER.as_mut_ptr());
    libc::_exit(0i32);
}
