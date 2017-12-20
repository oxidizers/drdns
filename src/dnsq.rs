use byte;
use buffer::Buffer;
use buffer_1::BUFFER_1;
use errno::errno;
use libc;
use stralloc::StrAlloc;
use strerr::{StrErr, STRERR_SYS};
use tai::Tai;
use taia::TaiA;
use uint16;

extern "C" {
    fn dns_domain_fromdot(arg1: *mut *mut u8, arg2: *const u8, arg3: u32) -> i32;
    fn dns_domain_todot_cat(arg1: *mut StrAlloc, arg2: *const u8) -> i32;
    fn dns_ip4_qualify(arg1: *mut StrAlloc, arg2: *mut StrAlloc, arg3: *const StrAlloc) -> i32;
    fn dns_random_init(arg1: *const u8);
    fn dns_transmit_get(arg1: *mut dns_transmit, arg2: *const pollfd, arg3: *const TaiA) -> i32;
    fn dns_transmit_io(arg1: *mut dns_transmit, arg2: *mut pollfd, arg3: *mut TaiA);
    fn dns_transmit_start(
        arg1: *mut dns_transmit,
        arg2: *const u8,
        arg3: i32,
        arg4: *const u8,
        arg5: *const u8,
        arg6: *const u8,
    ) -> i32;
    fn iopause(arg1: *mut pollfd, arg2: u32, arg3: *mut TaiA, arg4: *mut TaiA);
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

#[derive(Copy)]
#[repr(C)]
pub struct dns_transmit {
    pub query: *mut u8,
    pub querylen: u32,
    pub packet: *mut u8,
    pub packetlen: u32,
    pub s1: i32,
    pub tcpstate: i32,
    pub udploop: u32,
    pub curserver: u32,
    pub deadline: TaiA,
    pub pos: u32,
    pub servers: *const u8,
    pub localip: [u8; 4],
    pub qtype: [u8; 2],
}

impl Clone for dns_transmit {
    fn clone(&self) -> Self {
        *self
    }
}

static mut tx: dns_transmit = dns_transmit {
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
    if dns_transmit_start(
        &mut tx as (*mut dns_transmit),
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
            dns_transmit_io(
                &mut tx as (*mut dns_transmit),
                x.as_mut_ptr(),
                &mut deadline as (*mut TaiA),
            );
            iopause(
                x.as_mut_ptr(),
                1u32,
                &mut deadline as (*mut TaiA),
                &mut stamp as (*mut TaiA),
            );
            r = dns_transmit_get(
                &mut tx as (*mut dns_transmit),
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
    dns_random_init(seed.as_mut_ptr() as (*const u8));
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
    if dns_domain_fromdot(
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
    if dns_ip4_qualify(
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
    if dns_domain_todot_cat(&mut out as (*mut StrAlloc), q as (*const u8)) == 0 {
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
    Buffer::putflush(BUFFER_1.as_mut_ptr(), out.s as (*const u8), out.len);
    libc::_exit(0i32);
}
