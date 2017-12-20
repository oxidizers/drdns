use alloc;
use buffer::Buffer;
use byte;
use errno::errno;
use libc;
use stralloc::StrAlloc;
use tai::Tai;
use taia::TaiA;

extern "C" {
    static mut buffer_1: *mut Buffer;
    fn dns_name4_domain(arg1: *mut u8, arg2: *const u8);
    fn dns_name_packet(arg1: *mut StrAlloc, arg2: *const u8, arg3: u32) -> i32;
    fn dns_resolvconfip(arg1: *mut u8) -> i32;
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
    fn ip4_scan(arg1: *const u8, arg2: *mut u8) -> u32;
    fn scan_ulong(arg1: *const u8, arg2: *mut usize) -> u32;
    fn sgetoptmine(arg1: i32, arg2: *mut *mut u8, arg3: *const u8) -> i32;
    fn StrAlloc::append(arg1: *mut StrAlloc, arg2: *const u8) -> i32;
    fn StrAlloc::catb(arg1: *mut StrAlloc, arg2: *const u8, arg3: u32) -> i32;
    fn StrAlloc::cats(arg1: *mut StrAlloc, arg2: *const u8) -> i32;
    fn StrAlloc::copyb(arg1: *mut StrAlloc, arg2: *const u8, arg3: u32) -> i32;
    fn StrAlloc::copys(arg1: *mut StrAlloc, arg2: *const u8) -> i32;
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
    static mut subgetoptarg: *mut u8;
    static mut subgetoptdone: i32;
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
        (*b"dnsfilter: fatal: \0").as_ptr(),
        (*b"out of memory\0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const strerr),
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

#[derive(Copy)]
#[repr(C)]
pub struct line {
    pub left: StrAlloc,
    pub middle: StrAlloc,
    pub right: StrAlloc,
    pub dt: dns_transmit,
    pub flagactive: i32,
    pub io: *mut pollfd,
}

impl Clone for line {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub static mut x: *mut line = 0 as (*mut line);

#[no_mangle]
pub static mut tmp: line = line {
    left: StrAlloc {
        s: 0 as (*mut u8),
        len: 0u32,
        a: 0u32,
    },
    middle: StrAlloc {
        s: 0 as (*mut u8),
        len: 0u32,
        a: 0u32,
    },
    right: StrAlloc {
        s: 0 as (*mut u8),
        len: 0u32,
        a: 0u32,
    },
    dt: dns_transmit {
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
    },
    flagactive: 0i32,
    io: 0 as (*mut pollfd),
};

#[no_mangle]
pub static mut xmax: u32 = 1000u32;

#[no_mangle]
pub static mut xnum: u32 = 0u32;

#[no_mangle]
pub static mut numactive: u32 = 0u32;

#[no_mangle]
pub static mut maxactive: u32 = 10u32;

static mut partial: StrAlloc = StrAlloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

#[no_mangle]
pub static mut inbuf: [u8; 1024] = [0u8; 1024];

#[no_mangle]
pub static mut inbuflen: i32 = 0i32;

#[no_mangle]
pub static mut inio: *mut pollfd = 0 as (*mut pollfd);

#[no_mangle]
pub static mut flag0: i32 = 1i32;

#[no_mangle]
pub static mut io: *mut pollfd = 0 as (*mut pollfd);

#[no_mangle]
pub static mut iolen: i32 = 0i32;

#[no_mangle]
pub static mut servers: [u8; 64] = [0u8; 64];

#[no_mangle]
pub static mut ip: [u8; 4] = [0u8; 4];

#[no_mangle]
pub static mut name: [u8; 31] = [0u8; 31];

#[no_mangle]
pub unsafe extern "C" fn errout(mut i: i32) {
    let mut j: i32;
    if StrAlloc::copys(
        &mut (*x.offset(i as (isize))).middle as (*mut StrAlloc),
        (*b":\0").as_ptr(),
    ) == 0
    {
        nomem();
    }
    if StrAlloc::cats(
        &mut (*x.offset(i as (isize))).middle as (*mut StrAlloc),
        libc::strerror(errno().0),
    ) == 0
    {
        nomem();
    }
    j = 0i32;
    'loop5: loop {
        if !(j as (u32) < (*x.offset(i as (isize))).middle.len) {
            break;
        }
        if *(*x.offset(i as (isize))).middle.s.offset(j as (isize)) as (i32) == b' ' as (i32) {
            *(*x.offset(i as (isize))).middle.s.offset(j as (isize)) = b'-';
        }
        j = j + 1;
    }
}

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
    let mut stamp: TaiA;
    let mut deadline: TaiA;
    let mut opt: i32;
    let mut u: usize;
    let mut i: i32;
    let mut j: i32;
    let mut r: i32;
    'loop1: loop {
        if !({
                 opt = sgetoptmine(argc, argv, (*b"c:l:\0").as_ptr());
                 opt
             } != subgetoptdone)
        {
            break;
        }
        if opt == b'l' as (i32) {
            scan_ulong(subgetoptarg as (*const u8), &mut u as (*mut usize));
            if u < 1usize {
                u = 1usize;
            }
            if u > 1000000usize {
                u = 1000000usize;
            }
            xmax = u as (u32);
        } else if opt == b'c' as (i32) {
            scan_ulong(subgetoptarg as (*const u8), &mut u as (*mut usize));
            if u < 1usize {
                u = 1usize;
            }
            if u > 1000usize {
                u = 1000usize;
            }
            maxactive = u as (u32);
        } else {
            strerr_die(
                111i32,
                (*b"dnsfilter: usage: dnsfilter [ -c concurrency ] [ -l lines ]\0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const strerr),
            );
        }
    }
    x = alloc::alloc((xmax as (usize)).wrapping_mul(
        ::std::mem::size_of::<line>(),
    ) as (u32)) as (*mut line);
    if x.is_null() {
        nomem();
    }
    byte::zero(
        x as (*mut u8),
        (xmax as (usize)).wrapping_mul(::std::mem::size_of::<line>()) as (u32),
    );
    io = alloc::alloc((xmax.wrapping_add(1u32) as (usize)).wrapping_mul(
        ::std::mem::size_of::<pollfd>(),
    ) as (u32)) as (*mut pollfd);
    if io.is_null() {
        nomem();
    }
    if StrAlloc::copys(&mut partial as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
        nomem();
    }
    'loop8: loop {
        if !(flag0 != 0 || inbuflen != 0 || partial.len != 0 || xnum != 0) {
            break;
        }
        TaiA::now(&mut stamp as (*mut TaiA));
        TaiA::uint(&mut deadline as (*mut TaiA), 120u32);
        TaiA::add(
            &mut deadline as (*mut TaiA),
            &mut deadline as (*mut TaiA) as (*const TaiA),
            &mut stamp as (*mut TaiA) as (*const TaiA),
        );
        iolen = 0i32;
        if flag0 != 0 {
            if inbuflen as (usize) < ::std::mem::size_of::<[u8; 1024]>() {
                inio = io.offset({
                    let _old = iolen;
                    iolen = iolen + 1;
                    _old
                } as (isize));
                (*inio).fd = 0i32;
                (*inio).events = 0x1i16;
            }
        }
        i = 0i32;
        'loop14: loop {
            if !(i as (u32) < xnum) {
                break;
            }
            if (*x.offset(i as (isize))).flagactive != 0 {
                (*x.offset(i as (isize))).io = io.offset({
                    let _old = iolen;
                    iolen = iolen + 1;
                    _old
                } as (isize));
                dns_transmit_io(
                    &mut (*x.offset(i as (isize))).dt as (*mut dns_transmit),
                    (*x.offset(i as (isize))).io,
                    &mut deadline as (*mut TaiA),
                );
            }
            i = i + 1;
        }
        iopause(
            io,
            iolen as (u32),
            &mut deadline as (*mut TaiA),
            &mut stamp as (*mut TaiA),
        );
        if flag0 != 0 {
            if inbuflen as (usize) < ::std::mem::size_of::<[u8; 1024]>() {
                if (*inio).revents != 0 {
                    r = libc::read(
                        0i32,
                        inbuf.as_mut_ptr().offset(inbuflen as (isize)) as
                            (*mut libc::c_void),
                        ::std::mem::size_of::<[u8; 1024]>().wrapping_sub(inbuflen as (usize)),
                    ) as (i32);
                    if r <= 0i32 {
                        flag0 = 0i32;
                    } else {
                        inbuflen = inbuflen + r;
                    }
                }
            }
        }
        i = 0i32;
        'loop22: loop {
            if !(i as (u32) < xnum) {
                break;
            }
            if (*x.offset(i as (isize))).flagactive != 0 {
                r = dns_transmit_get(
                    &mut (*x.offset(i as (isize))).dt as (*mut dns_transmit),
                    (*x.offset(i as (isize))).io as (*const pollfd),
                    &mut stamp as (*mut TaiA) as (*const TaiA),
                );
                if r == -1i32 {
                    errout(i);
                    (*x.offset(i as (isize))).flagactive = 0i32;
                    numactive = numactive.wrapping_sub(1u32);
                } else if r == 1i32 {
                    if dns_name_packet(
                        &mut (*x.offset(i as (isize))).middle as (*mut StrAlloc),
                        (*x.offset(i as (isize))).dt.packet as (*const u8),
                        (*x.offset(i as (isize))).dt.packetlen,
                    ) == -1i32
                    {
                        errout(i);
                    }
                    if (*x.offset(i as (isize))).middle.len != 0 {
                        if StrAlloc::cats(
                            &mut (*x.offset(i as (isize))).left as (*mut StrAlloc),
                            (*b"=\0").as_ptr(),
                        ) == 0
                        {
                            nomem();
                        }
                    }
                    (*x.offset(i as (isize))).flagactive = 0i32;
                    numactive = numactive.wrapping_sub(1u32);
                }
            }
            i = i + 1;
        }
        'loop23: loop {
            if xnum != 0 && ((*x.offset(0isize)).flagactive == 0) {
                Buffer::put(
                    buffer_1,
                    (*x.offset(0isize)).left.s as (*const u8),
                    (*x.offset(0isize)).left.len,
                );
                Buffer::put(
                    buffer_1,
                    (*x.offset(0isize)).middle.s as (*const u8),
                    (*x.offset(0isize)).middle.len,
                );
                Buffer::put(
                    buffer_1,
                    (*x.offset(0isize)).right.s as (*const u8),
                    (*x.offset(0isize)).right.len,
                );
                Buffer::flush(buffer_1);
                xnum = xnum.wrapping_sub(1u32);
                tmp = *x.offset(0isize);
                i = 0i32;
                'loop56: loop {
                    if !(i as (u32) < xnum) {
                        break;
                    }
                    *x.offset(i as (isize)) = *x.offset((i + 1i32) as (isize));
                    i = i + 1;
                }
                *x.offset(xnum as (isize)) = tmp;
            } else {
                if !(xnum < xmax && (numactive < maxactive)) {
                    break;
                }
                i = byte::chr(inbuf.as_mut_ptr(), inbuflen as (u32), b'\n' as (i32)) as (i32);
                if inbuflen != 0 && (i == inbuflen) {
                    if StrAlloc::catb(
                        &mut partial as (*mut StrAlloc),
                        inbuf.as_mut_ptr() as (*const u8),
                        inbuflen as (u32),
                    ) == 0
                    {
                        nomem();
                    }
                    inbuflen = 0i32;
                } else {
                    if !(i < inbuflen || flag0 == 0 && (partial.len != 0)) {
                        break;
                    }
                    if i < inbuflen {
                        i = i + 1;
                    }
                    if StrAlloc::catb(
                        &mut partial as (*mut StrAlloc),
                        inbuf.as_mut_ptr() as (*const u8),
                        i as (u32),
                    ) == 0
                    {
                        nomem();
                    }
                    inbuflen = inbuflen - i;
                    j = 0i32;
                    'loop32: loop {
                        if !(j < inbuflen) {
                            break;
                        }
                        inbuf[j as (usize)] = inbuf[(j + i) as (usize)];
                        j = j + 1;
                    }
                    if partial.len != 0 {
                        i = byte::chr(partial.s, partial.len, b'\n' as (i32)) as (i32);
                        i = byte::chr(partial.s, i as (u32), b'\t' as (i32)) as (i32);
                        i = byte::chr(partial.s, i as (u32), b' ' as (i32)) as (i32);
                        if StrAlloc::copyb(
                            &mut (*x.offset(xnum as (isize))).left as (*mut StrAlloc),
                            partial.s as (*const u8),
                            i as (u32),
                        ) == 0
                        {
                            nomem();
                        }
                        if StrAlloc::copys(
                            &mut (*x.offset(xnum as (isize))).middle as (*mut StrAlloc),
                            (*b"\0").as_ptr(),
                        ) == 0
                        {
                            nomem();
                        }
                        if StrAlloc::copyb(
                            &mut (*x.offset(xnum as (isize))).right as (*mut StrAlloc),
                            partial.s.offset(i as (isize)) as (*const u8),
                            partial.len.wrapping_sub(i as (u32)),
                        ) == 0
                        {
                            nomem();
                        }
                        (*x.offset(xnum as (isize))).flagactive = 0i32;
                        partial.len = i as (u32);
                        if StrAlloc::append(&mut partial as (*mut StrAlloc), (*b"\0").as_ptr()) ==
                            0
                        {
                            nomem();
                        }
                        if ip4_scan(partial.s as (*const u8), ip.as_mut_ptr()) != 0 {
                            dns_name4_domain(name.as_mut_ptr(), ip.as_mut_ptr() as (*const u8));
                            if dns_resolvconfip(servers.as_mut_ptr()) == -1i32 {
                                strerr_die(
                                    111i32,
                                    (*b"dnsfilter: fatal: \0").as_ptr(),
                                    (*b"unable to read /etc/resolv.conf: \0").as_ptr(),
                                    0i32 as (*const u8),
                                    0i32 as (*const u8),
                                    0i32 as (*const u8),
                                    0i32 as (*const u8),
                                    &mut strerr_sys as (*mut strerr) as (*const strerr),
                                );
                            }
                            if dns_transmit_start(
                                &mut (*x.offset(xnum as (isize))).dt as (*mut dns_transmit),
                                servers.as_mut_ptr() as (*const u8),
                                1i32,
                                name.as_mut_ptr() as (*const u8),
                                (*b"\0\x0C\0").as_ptr(),
                                (*b"\0\0\0\0\0").as_ptr(),
                            ) == -1i32
                            {
                                errout(xnum as (i32));
                            } else {
                                (*x.offset(xnum as (isize))).flagactive = 1i32;
                                numactive = numactive.wrapping_add(1u32);
                            }
                        }
                        xnum = xnum.wrapping_add(1u32);
                    }
                    partial.len = 0u32;
                }
            }
        }
    }
    libc::_exit(0i32);
}
