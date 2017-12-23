use byte;
use buffer::{Buffer, STDOUT_BUFFER};
use dns::{self, DnsTransmit};
use errno::errno;
use iopause::iopause;
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
        (*b"dnsq: usage: dnsq type name server\0").as_ptr(),
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
        (*b"dnsq: fatal: \0").as_ptr(),
        (*b"unable to parse: \0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
    );
}

static mut tx: DnsTransmit = DnsTransmit {
    query: 0 as (*mut u8),
    querylen: 0u32,
    packet: 0 as (*mut u8),
    packetlen: 0u32,
    s1: 0i32,
    tcpstate: 0i32,
    udploop: 0u32,
    curserver: 0u32,
    deadline: TaiA {
        sec: Tai { x: 0usize },
        nano: 0usize,
        atto: 0usize,
    },
    pos: 0u32,
    servers: 0 as (*const u8),
    localip: [0u8; 4],
    qtype: [0u8; 2],
};

#[derive(Copy)]
#[repr(C)]
pub struct pollfd {
    pub fd: i32,
    pub events: i16,
    pub revents: i16,
}

impl Clone for pollfd {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn resolve(mut q: *mut u8, mut qtype: *mut u8, mut servers: *mut u8) -> i32 {
    let mut _currentBlock;
    let mut stamp: TaiA;
    let mut deadline: TaiA;
    let mut x: [pollfd; 1];
    let mut r: i32;
    if DnsTramsit::start(
        &mut tx as (*mut DnsTransmit),
        servers as (*const u8),
        0i32,
        q as (*const u8),
        qtype as (*const u8),
        (*b"\0\0\0\0\0").as_ptr(),
    ) == -1i32
    {
        -1i32
    } else {
        'loop1: loop {
            TaiA::now(&mut stamp as (*mut TaiA));
            TaiA::uint(&mut deadline as (*mut TaiA), 120u32);
            TaiA::add(
                &mut deadline as (*mut TaiA),
                &mut deadline as (*mut TaiA) as (*const TaiA),
                &mut stamp as (*mut TaiA) as (*const TaiA),
            );
            DnsTramsit::io(
                &mut tx as (*mut DnsTransmit),
                x.as_mut_ptr(),
                &mut deadline as (*mut TaiA),
            );
            iopause(
                x.as_mut_ptr(),
                1u32,
                &mut deadline as (*mut TaiA),
                &mut stamp as (*mut TaiA),
            );
            r = DnsTramsit::get(
                &mut tx as (*mut DnsTransmit),
                x.as_mut_ptr() as (*const pollfd),
                &mut stamp as (*mut TaiA) as (*const TaiA),
            );
            if r == -1i32 {
                _currentBlock = 4;
                break;
            }
            if r == 1i32 {
                _currentBlock = 3;
                break;
            }
        }
        (if _currentBlock == 3 { 0i32 } else { -1i32 })
    }
}

#[no_mangle]
pub static mut servers: [u8; 64] = [0u8; 64];

static mut ip: StrAlloc = StrAlloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

static mut fqdn: StrAlloc = StrAlloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

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
    if (*{
            argv = argv.offset(1isize);
            argv
        }).is_null()
    {
        usage();
    }
    if StrAlloc::copys(&mut out as (*mut StrAlloc), *argv as (*const u8)) == 0 {
        oops();
    }
    if dns::ip4::qualify(
        &mut ip as (*mut StrAlloc),
        &mut fqdn as (*mut StrAlloc),
        &mut out as (*mut StrAlloc) as (*const StrAlloc),
    ) == -1i32
    {
        oops();
    }
    if ip.len >= 64u32 {
        ip.len = 64u32;
    }
    byte::zero(servers.as_mut_ptr(), 64u32);
    byte::copy(servers.as_mut_ptr(), ip.len, ip.s);
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
    if resolve(q, type_.as_mut_ptr(), servers.as_mut_ptr()) == -1i32 {
        if StrAlloc::cats(&mut out as (*mut StrAlloc), libc::strerror(errno().0)) == 0 {
            oops();
        }
        if StrAlloc::cats(&mut out as (*mut StrAlloc), (*b"\n\0").as_ptr()) == 0 {
            oops();
        }
    } else if printpacket_cat(&mut out as (*mut StrAlloc), tx.packet, tx.packetlen) == 0 {
        oops();
    }
    Buffer::putflush(STDOUT_BUFFER.as_mut_ptr(), out.s as (*const u8), out.len);
    libc::_exit(0i32);
}
