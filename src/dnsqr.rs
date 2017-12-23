use buffer::{Buffer, STDOUT_BUFFER};
use dns::{self, DnsTransmit};
use errno::errno;
use libc;
use stralloc::StrAlloc;
use strerr::{StrErr, STRERR_SYS};
use tai::Tai;
use taia::TaiA;
use uint16;

extern "C" {
    fn parsetype(arg1: *mut u8, arg2: *mut u8) -> i32;
    fn printpacket_cat(arg1: *mut StrAlloc, arg2: *mut u8, arg3: u32) -> u32;
}

#[no_mangle]
pub unsafe extern "C" fn usage() {
    StrErr::die(
        100i32,
        (*b"dnsqr: usage: dnsqr type name\0").as_ptr(),
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
        (*b"dnsqr: fatal: \0").as_ptr(),
        (*b"unable to parse: \0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
    );
}

#[no_mangle]
pub static mut type_: [u8; 2] = [0u8; 2];

static mut q: *mut u8 = 0 as (*mut u8);

static mut out: StrAlloc = StrAlloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

static mut seed: [u8; 128] = [0u8; 128];

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
    let mut u16: u16;
    dns::random::init(seed.as_mut_ptr() as (*const u8));
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
        usage();
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
    if dns::resolve::resolve(q as (*const u8), type_.as_mut_ptr() as (*const u8)) == -1i32 {
        if StrAlloc::cats(&mut out as (*mut StrAlloc), libc::strerror(errno().0)) == 0 {
            oops();
        }
        if StrAlloc::cats(&mut out as (*mut StrAlloc), (*b"\n\0").as_ptr()) == 0 {
            oops();
        }
    } else {
        if resolve::TX.packetlen < 4u32 {
            oops();
        }
        let _rhs = !1i32;
        let _lhs = &mut *resolve::TX.packet.offset(2isize);
        *_lhs = (*_lhs as (i32) & _rhs) as (u8);
        let _rhs = !128i32;
        let _lhs = &mut *resolve::TX.packet.offset(3isize);
        *_lhs = (*_lhs as (i32) & _rhs) as (u8);
        if printpacket_cat(
            &mut out as (*mut StrAlloc),
            resolve::TX.packet,
            resolve::TX.packetlen,
        ) == 0
        {
            oops();
        }
    }
    Buffer::putflush(STDOUT_BUFFER.as_mut_ptr(), out.s as (*const u8), out.len);
    libc::_exit(0i32);
}
