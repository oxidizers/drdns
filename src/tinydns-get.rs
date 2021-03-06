use byte;
use buffer::{Buffer, STDOUT_BUFFER};
use case;
use dns;
use ip4;
use libc;
use stralloc::StrAlloc;
use uint16;

extern "C" {
    fn parsetype(arg1: *mut u8, arg2: *mut u8) -> i32;
    fn printpacket_cat(arg1: *mut StrAlloc, arg2: *mut u8, arg3: u32) -> u32;
    fn respond(arg1: *mut u8, arg2: *mut u8, arg3: *mut u8) -> i32;
    static mut response: *mut u8;
    static mut response_len: u32;
    fn response_query(arg1: *const u8, arg2: *const u8, arg3: *const u8) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn usage() {
    StrErr::die(
        100i32,
        (*b"tinydns-get: usage: tinydns-get type name [ip]\0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const StrErr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn oops() {
    StrErr::die(
        111i32,
        (*b"tinydns-get: fatal: \0").as_ptr(),
        (*b"unable to parse: \0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
    );
}

static mut ip: [u8; 4] = [0u8; 4];

static mut type_: [u8; 2] = [0u8; 2];

static mut q: *mut u8 = 0 as (*mut u8);

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
    let mut _currentBlock;
    let mut u16: u16;
    if (*argv).is_null() {
        usage();
    }
    if (*{
            argv = argv.offset(1isize);
            argv
        }).is_null()
    {
        usage();
    }
    if parsetype(*argv, type_.as_mut_ptr()) == 0 {
        usage();
    }
    if (*{
            argv = argv.offset(1isize);
            argv
        }).is_null()
    {
        usage();
    }
    if dns::domain::fromdot(
        &mut q as (*mut *mut u8),
        *argv as (*const u8),
        libc::strlen(*argv as *const i8) as u32,
    ) == 0
    {
        oops();
    }
    if !(*{
             argv = argv.offset(1isize);
             argv
         }).is_null()
    {
        if ip4::scan(*argv as (*const u8), ip.as_mut_ptr()) == 0 {
            usage();
        }
    }
    if StrAlloc::copys(&mut out as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
        oops();
    }
    uint16::unpack_big(type_.as_mut_ptr() as (*const u8), &mut u16 as (*mut u16));
    if StrAlloc::catulong0(&mut out as (*mut StrAlloc), u16 as (usize), 0u32) == 0 {
        oops();
    }
    if StrAlloc::cats(&mut out as (*mut StrAlloc), (*b" \0").as_ptr()) == 0 {
        oops();
    }
    if dns::domain::todot_cat(&mut out as (*mut StrAlloc), q as (*const u8)) == 0 {
        oops();
    }
    if StrAlloc::cats(&mut out as (*mut StrAlloc), (*b":\n\0").as_ptr()) == 0 {
        oops();
    }
    if response_query(
        q as (*const u8),
        type_.as_mut_ptr() as (*const u8),
        (*b"\0\x01\0").as_ptr(),
    ) == 0
    {
        oops();
    }
    let _rhs = !128i32;
    let _lhs = &mut *response.offset(3isize);
    *_lhs = (*_lhs as (i32) & _rhs) as (u8);
    let _rhs = !1i32;
    let _lhs = &mut *response.offset(2isize);
    *_lhs = (*_lhs as (i32) & _rhs) as (u8);
    let _rhs = 4i32;
    let _lhs = &mut *response.offset(2isize);
    *_lhs = (*_lhs as (i32) | _rhs) as (u8);
    case::lowerb(q, dns::domain::length(q as (*const u8)));
    if byte::diff(
        type_.as_mut_ptr(),
        2u32,
        (*b"\0\xFC\0").as_ptr() as (*mut u8),
    ) == 0
    {
        let _rhs = !15i32;
        let _lhs = &mut *response.offset(3isize);
        *_lhs = (*_lhs as (i32) & _rhs) as (u8);
        let _rhs = 4i32;
        let _lhs = &mut *response.offset(3isize);
        *_lhs = (*_lhs as (i32) | _rhs) as (u8);
        _currentBlock = 28;
    } else if respond(q, type_.as_mut_ptr(), ip.as_mut_ptr()) == 0 {
        _currentBlock = 30;
    } else {
        _currentBlock = 28;
    }
    if _currentBlock == 28 {
        if printpacket_cat(&mut out as (*mut StrAlloc), response, response_len) == 0 {
            oops();
        }
    }
    Buffer::putflush(STDOUT_BUFFER.as_mut_ptr(), out.s as (*const u8), out.len);
    libc::_exit(0i32);
}
