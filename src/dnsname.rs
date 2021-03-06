use buffer::{Buffer, STDOUT_BUFFER};
use dns;
use ip4;
use libc;
use stralloc::StrAlloc;
use strerr::{StrErr, STRERR_SYS};

static mut seed: [u8; 128] = [0u8; 128];

#[no_mangle]
pub static mut ip: [u8; 4] = [0u8; 4];

static mut out: StrAlloc = StrAlloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

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
    dns::random::init(seed.as_mut_ptr() as (*const u8));
    if !(*argv).is_null() {
        argv = argv.offset(1isize);
    }
    'loop2: loop {
        if (*argv).is_null() {
            break;
        }
        if ip4::scan(*argv as (*const u8), ip.as_mut_ptr()) == 0 {
            StrErr::die(
                111i32,
                (*b"dnsname: fatal: \0").as_ptr(),
                (*b"unable to parse IP address \0").as_ptr(),
                *argv as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const StrErr),
            );
        }
        if dns::name::name4(&mut out as (*mut StrAlloc), ip.as_mut_ptr() as (*const u8)) == -1i32 {
            StrErr::die(
                111i32,
                (*b"dnsname: fatal: \0").as_ptr(),
                (*b"unable to find host name for \0").as_ptr(),
                *argv as (*const u8),
                (*b": \0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
            );
        }
        Buffer::put(STDOUT_BUFFER.as_mut_ptr(), out.s as (*const u8), out.len);
        Buffer::puts(STDOUT_BUFFER.as_mut_ptr(), (*b"\n\0").as_ptr());
        argv = argv.offset(1isize);
    }
    Buffer::flush(STDOUT_BUFFER.as_mut_ptr());
    libc::_exit(0i32);
}
