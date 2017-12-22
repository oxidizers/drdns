use buffer::{self, Buffer};
use byte;
use case;
use cdb::Cdb;
use libc;
use stralloc::StrAlloc;
use strerr::{StrErr, STRERR_SYS};
use tai::Tai;
use uint16;
use uint32;

extern "C" {
    fn close(arg1: i32) -> i32;
    fn dns_domain_equal(arg1: *const u8, arg2: *const u8) -> i32;
    fn dns_domain_fromdot(arg1: *mut *mut u8, arg2: *const u8, arg3: u32) -> i32;
    fn dns_domain_length(arg1: *const u8) -> u32;
    fn dns_domain_suffix(arg1: *const u8, arg2: *const u8) -> i32;
    fn dns_packet_copy(arg1: *const u8, arg2: u32, arg3: u32, arg4: *mut u8, arg5: u32) -> u32;
    fn dns_packet_getname(arg1: *const u8, arg2: u32, arg3: u32, arg4: *mut *mut u8) -> u32;
    fn dns_random_init(arg1: *const u8);
    fn droproot(arg1: *const u8);
    fn env_get(arg1: *const u8) -> *mut u8;
    fn ip4_scan(arg1: *const u8, arg2: *mut u8) -> u32;
    fn open_read(arg1: *const u8) -> i32;
    fn qlog(
        arg1: *const u8,
        arg2: u16,
        arg3: *const u8,
        arg4: *const u8,
        arg5: *const u8,
        arg6: *const u8,
    );
    fn respond(arg1: *mut u8, arg2: *mut u8, arg3: *mut u8) -> i32;
    static mut response: *mut u8;
    fn response_id(arg1: *const u8);
    static mut response_len: u32;
    fn response_query(arg1: *const u8, arg2: *const u8, arg3: *const u8) -> i32;
    fn scan_ulong(arg1: *const u8, arg2: *mut usize) -> u32;
    fn timeoutread(t: i32, fd: i32, buf: *mut u8, len: i32) -> i32;
    fn timeoutwrite(t: i32, fd: i32, buf: *mut u8, len: i32) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn nomem() {
    StrErr::die(
        111i32,
        (*b"axfrdns: fatal: \0").as_ptr(),
        (*b"out of memory\0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const StrErr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn die_truncated() {
    StrErr::die(
        111i32,
        (*b"axfrdns: fatal: \0").as_ptr(),
        (*b"truncated request\0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const StrErr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn die_netwrite() {
    StrErr::die(
        111i32,
        (*b"axfrdns: fatal: \0").as_ptr(),
        (*b"unable to write to network: \0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn die_netread() {
    StrErr::die(
        111i32,
        (*b"axfrdns: fatal: \0").as_ptr(),
        (*b"unable to read from network: \0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn die_outside() {
    StrErr::die(
        111i32,
        (*b"axfrdns: fatal: \0").as_ptr(),
        (*b"unable to locate information in data.cdb\0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const StrErr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn die_cdbread() {
    StrErr::die(
        111i32,
        (*b"axfrdns: fatal: \0").as_ptr(),
        (*b"unable to read data.cdb: \0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn die_cdbformat() {
    StrErr::die(
        111i32,
        (*b"axfrdns: fatal: \0").as_ptr(),
        (*b"unable to read data.cdb: \0").as_ptr(),
        (*b"format error\0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const StrErr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn safewrite(mut fd: i32, mut buf: *mut u8, mut len: u32) -> i32 {
    let mut w: i32;
    w = timeoutwrite(60i32, fd, buf, len as (i32));
    if w <= 0i32 {
        die_netwrite();
    }
    w
}

#[no_mangle]
pub static mut netwritespace: [u8; 1024] = [0u8; 1024];

#[no_mangle]
pub static mut netwrite: Buffer = Buffer {
    x: netwritespace.as_mut_ptr(),
    p: 0u32,
    n: ::std::mem::size_of::<[u8; 1024]>() as (u32),
    fd: 1i32,
    op: Some(safewrite as buffer::Op),
};

#[no_mangle]
pub unsafe extern "C" fn print(mut buf: *mut u8, mut len: u32) {
    let mut tcpheader: [u8; 2];
    uint16::pack_big(tcpheader.as_mut_ptr(), len as (u16));
    Buffer::put(
        &mut netwrite as (*mut Buffer),
        tcpheader.as_mut_ptr() as (*const u8),
        2u32,
    );
    Buffer::put(&mut netwrite as (*mut Buffer), buf as (*const u8), len);
    Buffer::flush(&mut netwrite as (*mut Buffer));
}

#[no_mangle]
pub static mut axfr: *mut u8 = 0 as (*mut u8);

static mut axfrok: *mut u8 = 0 as (*mut u8);

#[no_mangle]
pub unsafe extern "C" fn axfrcheck(mut q: *mut u8) {
    let mut _currentBlock;
    let mut i: i32;
    let mut j: i32;
    if axfr.is_null() {
    } else {
        i = {
            j = 0i32;
            j
        };
        'loop2: loop {
            if *axfr.offset(i as (isize)) == 0 ||
                *axfr.offset(i as (isize)) as (i32) == b'/' as (i32)
            {
                if i > j {
                    if dns_domain_fromdot(
                        &mut axfrok as (*mut *mut u8),
                        axfr.offset(j as (isize)) as (*const u8),
                        (i - j) as (u32),
                    ) == 0
                    {
                        nomem();
                    }
                    if dns_domain_equal(q as (*const u8), axfrok as (*const u8)) != 0 {
                        _currentBlock = 11;
                        break;
                    }
                }
                j = i + 1i32;
            }
            if *axfr.offset(i as (isize)) == 0 {
                _currentBlock = 10;
                break;
            }
            i = i + 1;
        }
        (if _currentBlock == 10 {
             StrErr::die(
                111i32,
                (*b"axfrdns: fatal: \0").as_ptr(),
                (*b"disallowed zone transfer request\0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const StrErr),
            );
         })
    }
}

static mut zone: *mut u8 = 0 as (*mut u8);

#[no_mangle]
pub static mut zonelen: u32 = 0u32;

#[no_mangle]
pub static mut typeclass: [u8; 4] = [0u8; 4];

#[no_mangle]
pub static mut fdcdb: i32 = 0i32;

#[no_mangle]
pub static mut bcdb: Buffer = Buffer {
    x: 0 as (*mut u8),
    p: 0u32,
    n: 0u32,
    fd: 0i32,
    op: None,
};

#[no_mangle]
pub static mut bcdbspace: [u8; 1024] = [0u8; 1024];

#[no_mangle]
pub unsafe extern "C" fn get(mut buf: *mut u8, mut len: u32) {
    let mut r: i32;
    'loop1: loop {
        if !(len > 0u32) {
            break;
        }
        r = Buffer::get(&mut bcdb as (*mut Buffer), buf, len);
        if r < 0i32 {
            die_cdbread();
        }
        if r == 0 {
            die_cdbformat();
        }
        buf = buf.offset(r as (isize));
        len = len.wrapping_sub(r as (u32));
    }
}

#[no_mangle]
pub static mut ip: [u8; 4] = [0u8; 4];

#[no_mangle]
pub static mut port: usize = 0usize;

#[no_mangle]
pub static mut clientloc: [u8; 2] = [0u8; 2];

#[no_mangle]
pub static mut now: Tai = Tai { x: 0usize };

#[no_mangle]
pub static mut data: [u8; 32767] = [0u8; 32767];

#[no_mangle]
pub static mut dlen: u32 = 0u32;

#[no_mangle]
pub static mut dpos: u32 = 0u32;

#[no_mangle]
pub unsafe extern "C" fn copy(mut buf: *mut u8, mut len: u32) {
    dpos = dns_packet_copy(data.as_mut_ptr() as (*const u8), dlen, dpos, buf, len);
    if dpos == 0 {
        die_cdbread();
    }
}

#[no_mangle]
pub unsafe extern "C" fn doname(mut sa: *mut StrAlloc) {
    static mut d: *mut u8 = 0 as (*mut u8);
    dpos = dns_packet_getname(
        data.as_mut_ptr() as (*const u8),
        dlen,
        dpos,
        &mut d as (*mut *mut u8),
    );
    if dpos == 0 {
        die_cdbread();
    }
    if StrAlloc::catb(sa, d as (*const u8), dns_domain_length(d as (*const u8))) == 0 {
        nomem();
    }
}

#[no_mangle]
pub unsafe extern "C" fn build(
    mut sa: *mut StrAlloc,
    mut q: *mut u8,
    mut flagsoa: i32,
    mut id: *mut u8,
) -> i32 {
    let mut rdatapos: u32;
    let mut misc: [u8; 20];
    let mut type_: [u8; 2];
    let mut recordloc: [u8; 2];
    let mut ttl: [u8; 4];
    let mut ttd: [u8; 8];
    let mut cutoff: Tai;
    dpos = 0u32;
    copy(type_.as_mut_ptr(), 2u32);
    if flagsoa != 0 {
        if byte::diff(
            type_.as_mut_ptr(),
            2u32,
            (*b"\0\x06\0").as_ptr() as (*mut u8),
        ) != 0
        {
            return 0i32;
        }
    }
    if flagsoa == 0 {
        if byte::diff(
            type_.as_mut_ptr(),
            2u32,
            (*b"\0\x06\0").as_ptr() as (*mut u8),
        ) == 0
        {
            return 0i32;
        }
    }
    if StrAlloc::copyb(sa, id as (*const u8), 2u32) == 0 {
        nomem();
    }
    if StrAlloc::catb(sa, (*b"\x84\0\0\0\0\x01\0\0\0\0\0").as_ptr(), 10u32) == 0 {
        nomem();
    }
    copy(misc.as_mut_ptr(), 1u32);
    if misc[0usize] as (i32) == b'=' as (i32) + 1i32 ||
        misc[0usize] as (i32) == b'*' as (i32) + 1i32
    {
        let _rhs = 1;
        let _lhs = &mut misc[0usize];
        *_lhs = (*_lhs as (i32) - _rhs) as (u8);
        copy(recordloc.as_mut_ptr(), 2u32);
        if byte::diff(recordloc.as_mut_ptr(), 2u32, clientloc.as_mut_ptr()) != 0 {
            return 0i32;
        }
    }
    if misc[0usize] as (i32) == b'*' as (i32) {
        if flagsoa != 0 {
            return 0i32;
        } else if StrAlloc::catb(sa, (*b"\x01*\0").as_ptr(), 2u32) == 0 {
            nomem();
        }
    }
    if StrAlloc::catb(sa, q as (*const u8), dns_domain_length(q as (*const u8))) == 0 {
        nomem();
    }
    if StrAlloc::catb(sa, type_.as_mut_ptr() as (*const u8), 2u32) == 0 {
        nomem();
    }
    copy(ttl.as_mut_ptr(), 4u32);
    copy(ttd.as_mut_ptr(), 8u32);
    if byte::diff(
        ttd.as_mut_ptr(),
        8u32,
        (*b"\0\0\0\0\0\0\0\0\0").as_ptr() as (*mut u8),
    ) != 0
    {
        Tai::unpack(ttd.as_mut_ptr() as (*const u8), &mut cutoff as (*mut Tai));
        if byte::diff(
            ttl.as_mut_ptr(),
            4u32,
            (*b"\0\0\0\0\0").as_ptr() as (*mut u8),
        ) == 0
        {
            if (*(&mut cutoff as (*mut Tai))).x < (*(&mut now as (*mut Tai))).x {
                return 0i32;
            } else {
                uint32::pack_big(ttl.as_mut_ptr(), 2u32);
            }
        } else if !((*(&mut cutoff as (*mut Tai))).x < (*(&mut now as (*mut Tai))).x) {
            return 0i32;
        }
    }
    if StrAlloc::catb(sa, (*b"\0\x01\0").as_ptr(), 2u32) == 0 {
        nomem();
    }
    if StrAlloc::catb(sa, ttl.as_mut_ptr() as (*const u8), 4u32) == 0 {
        nomem();
    }
    if StrAlloc::catb(sa, (*b"\0\0\0").as_ptr(), 2u32) == 0 {
        nomem();
    }
    rdatapos = (*sa).len;
    if byte::diff(
        type_.as_mut_ptr(),
        2u32,
        (*b"\0\x06\0").as_ptr() as (*mut u8),
    ) == 0
    {
        doname(sa);
        doname(sa);
        copy(misc.as_mut_ptr(), 20u32);
        if StrAlloc::catb(sa, misc.as_mut_ptr() as (*const u8), 20u32) == 0 {
            nomem();
        }
    } else if byte::diff(
        type_.as_mut_ptr(),
        2u32,
        (*b"\0\x02\0").as_ptr() as (*mut u8),
    ) == 0 ||
               byte::diff(
            type_.as_mut_ptr(),
            2u32,
            (*b"\0\x0C\0").as_ptr() as (*mut u8),
        ) == 0 ||
               byte::diff(
            type_.as_mut_ptr(),
            2u32,
            (*b"\0\x05\0").as_ptr() as (*mut u8),
        ) == 0
    {
        doname(sa);
    } else if byte::diff(
        type_.as_mut_ptr(),
        2u32,
        (*b"\0\x0F\0").as_ptr() as (*mut u8),
    ) == 0
    {
        copy(misc.as_mut_ptr(), 2u32);
        if StrAlloc::catb(sa, misc.as_mut_ptr() as (*const u8), 2u32) == 0 {
            nomem();
        }
        doname(sa);
    } else if StrAlloc::catb(
        sa,
        data.as_mut_ptr().offset(dpos as (isize)) as (*const u8),
        dlen.wrapping_sub(dpos),
    ) == 0
    {
        nomem();
    }
    if (*sa).len > 65535u32 {
        die_cdbformat();
    }
    uint16::pack_big(
        (*sa).s.offset(rdatapos as (isize)).offset(-2isize),
        (*sa).len.wrapping_sub(rdatapos) as (u16),
    );
    1i32
}

static mut c: Cdb = Cdb {
    map: 0 as (*mut u8),
    fd: 0i32,
    size: 0u32,
    loopvar: 0u32,
    khash: 0u32,
    kpos: 0u32,
    hpos: 0u32,
    hslots: 0u32,
    dpos: 0u32,
    dlen: 0u32,
};

static mut q: *mut u8 = 0 as (*mut u8);

static mut soa: StrAlloc = StrAlloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

static mut message: StrAlloc = StrAlloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

#[no_mangle]
pub unsafe extern "C" fn doaxfr(mut id: *mut u8) {
    let mut key: [u8; 512];
    let mut klen: u32;
    let mut num: [u8; 4];
    let mut eod: u32;
    let mut pos: u32;
    let mut r: i32;
    axfrcheck(zone);
    Tai::now(&mut now as (*mut Tai));
    Cdb::init(&mut c as (*mut Cdb), fdcdb);
    byte::zero(clientloc.as_mut_ptr(), 2u32);
    key[0usize] = 0u8;
    key[1usize] = b'%';
    byte::copy(key.as_mut_ptr().offset(2isize), 4u32, ip.as_mut_ptr());
    r = Cdb::find(&mut c as (*mut Cdb), key.as_mut_ptr() as (*const u8), 6u32);
    if r == 0 {
        r = Cdb::find(&mut c as (*mut Cdb), key.as_mut_ptr() as (*const u8), 5u32);
    }
    if r == 0 {
        r = Cdb::find(&mut c as (*mut Cdb), key.as_mut_ptr() as (*const u8), 4u32);
    }
    if r == 0 {
        r = Cdb::find(&mut c as (*mut Cdb), key.as_mut_ptr() as (*const u8), 3u32);
    }
    if r == 0 {
        r = Cdb::find(&mut c as (*mut Cdb), key.as_mut_ptr() as (*const u8), 2u32);
    }
    if r == -1i32 {
        die_cdbread();
    }
    if r != 0 && ((*(&mut c as (*mut Cdb))).dlen == 2u32) {
        if Cdb::read(
            &mut c as (*mut Cdb),
            clientloc.as_mut_ptr(),
            2u32,
            (*(&mut c as (*mut Cdb))).dpos,
        ) == -1i32
        {
            die_cdbread();
        }
    }
    Cdb::findstart(&mut c as (*mut Cdb));
    'loop14: loop {
        r = Cdb::findnext(&mut c as (*mut Cdb), zone as (*const u8), zonelen);
        if r == -1i32 {
            die_cdbread();
        }
        if r == 0 {
            die_outside();
        }
        dlen = (*(&mut c as (*mut Cdb))).dlen;
        if dlen as (usize) > ::std::mem::size_of::<[u8; 32767]>() {
            die_cdbformat();
        }
        if Cdb::read(
            &mut c as (*mut Cdb),
            data.as_mut_ptr(),
            dlen,
            (*(&mut c as (*mut Cdb))).dpos,
        ) == -1i32
        {
            die_cdbformat();
        }
        if build(&mut soa as (*mut StrAlloc), zone, 1i32, id) != 0 {
            break;
        }
    }
    Cdb::free(&mut c as (*mut Cdb));
    print(soa.s, soa.len);
    libc::lseek(fdcdb, 0, 0);
    Buffer::init(
        &mut bcdb as (*mut Buffer),
        buffer::unixread as buffer::Op,
        fdcdb,
        bcdbspace.as_mut_ptr(),
        ::std::mem::size_of::<[u8; 1024]>() as (u32),
    );
    pos = 0u32;
    get(num.as_mut_ptr(), 4u32);
    pos = pos.wrapping_add(4u32);
    uint32::unpack(num.as_mut_ptr() as (*const u8), &mut eod as (*mut u32));
    'loop24: loop {
        if !(pos < 2048u32) {
            break;
        }
        get(num.as_mut_ptr(), 4u32);
        pos = pos.wrapping_add(4u32);
    }
    'loop25: loop {
        if !(pos < eod) {
            break;
        }
        if eod.wrapping_sub(pos) < 8u32 {
            die_cdbformat();
        }
        get(num.as_mut_ptr(), 4u32);
        pos = pos.wrapping_add(4u32);
        uint32::unpack(num.as_mut_ptr() as (*const u8), &mut klen as (*mut u32));
        get(num.as_mut_ptr(), 4u32);
        pos = pos.wrapping_add(4u32);
        uint32::unpack(num.as_mut_ptr() as (*const u8), &mut dlen as (*mut u32));
        if eod.wrapping_sub(pos) < klen {
            die_cdbformat();
        }
        pos = pos.wrapping_add(klen);
        if eod.wrapping_sub(pos) < dlen {
            die_cdbformat();
        }
        pos = pos.wrapping_add(dlen);
        if klen as (usize) > ::std::mem::size_of::<[u8; 512]>() {
            die_cdbformat();
        }
        get(key.as_mut_ptr(), klen);
        if dlen as (usize) > ::std::mem::size_of::<[u8; 32767]>() {
            die_cdbformat();
        }
        get(data.as_mut_ptr(), dlen);
        if klen > 1u32 && (key[0usize] as (i32) == 0i32) {
            continue;
        }
        if klen < 1u32 {
            die_cdbformat();
        }
        if dns_packet_getname(
            key.as_mut_ptr() as (*const u8),
            klen,
            0u32,
            &mut q as (*mut *mut u8),
        ) != klen
        {
            die_cdbformat();
        }
        if dns_domain_suffix(q as (*const u8), zone as (*const u8)) == 0 {
            continue;
        }
        if build(&mut message as (*mut StrAlloc), q, 0i32, id) == 0 {
            continue;
        }
        print(message.s, message.len);
    }
    print(soa.s, soa.len);
}

#[no_mangle]
pub unsafe extern "C" fn netread(mut buf: *mut u8, mut len: u32) {
    let mut r: i32;
    'loop1: loop {
        if !(len > 0u32) {
            break;
        }
        r = timeoutread(60i32, 0i32, buf, len as (i32));
        if r == 0i32 {
            libc::_exit(0i32);
        }
        if r < 0i32 {
            die_netread();
        }
        buf = buf.offset(r as (isize));
        len = len.wrapping_sub(r as (u32));
    }
}

#[no_mangle]
pub static mut tcpheader: [u8; 2] = [0u8; 2];

#[no_mangle]
pub static mut buf: [u8; 512] = [0u8; 512];

#[no_mangle]
pub static mut len: u16 = 0u16;

static mut seed: [u8; 128] = [0u8; 128];

fn main() {
    let ret = unsafe { _c_main() };
    ::std::process::exit(ret);
}

#[no_mangle]
pub unsafe extern "C" fn _c_main() -> i32 {
    let mut pos: u32;
    let mut header: [u8; 12];
    let mut qtype: [u8; 2];
    let mut qclass: [u8; 2];
    let mut x: *const u8;
    droproot((*b"axfrdns: fatal: \0").as_ptr());
    dns_random_init(seed.as_mut_ptr() as (*const u8));
    axfr = env_get((*b"AXFR\0").as_ptr());
    x = env_get((*b"TCPREMOTEIP\0").as_ptr()) as (*const u8);
    if !(!x.is_null() && (ip4_scan(x, ip.as_mut_ptr()) != 0)) {
        byte::zero(ip.as_mut_ptr(), 4u32);
    }
    x = env_get((*b"TCPREMOTEPORT\0").as_ptr()) as (*const u8);
    if x.is_null() {
        x = (*b"0\0").as_ptr();
    }
    scan_ulong(x, &mut port as (*mut usize));
    'loop5: loop {
        netread(tcpheader.as_mut_ptr(), 2u32);
        uint16::unpack_big(
            tcpheader.as_mut_ptr() as (*const u8),
            &mut len as (*mut u16),
        );
        if len as (i32) > 512i32 {
            StrErr::die(
                111i32,
                (*b"axfrdns: fatal: \0").as_ptr(),
                (*b"excessively large request\0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const StrErr),
            );
        }
        netread(buf.as_mut_ptr(), len as (u32));
        pos = dns_packet_copy(
            buf.as_mut_ptr() as (*const u8),
            len as (u32),
            0u32,
            header.as_mut_ptr(),
            12u32,
        );
        if pos == 0 {
            die_truncated();
        }
        if header[2usize] as (i32) & 254i32 != 0 {
            StrErr::die(
                111i32,
                (*b"axfrdns: fatal: \0").as_ptr(),
                (*b"bogus query\0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const StrErr),
            );
        }
        if header[4usize] != 0 || header[5usize] as (i32) != 1i32 {
            StrErr::die(
                111i32,
                (*b"axfrdns: fatal: \0").as_ptr(),
                (*b"bogus query\0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const StrErr),
            );
        }
        pos = dns_packet_getname(
            buf.as_mut_ptr() as (*const u8),
            len as (u32),
            pos,
            &mut zone as (*mut *mut u8),
        );
        if pos == 0 {
            die_truncated();
        }
        zonelen = dns_domain_length(zone as (*const u8));
        pos = dns_packet_copy(
            buf.as_mut_ptr() as (*const u8),
            len as (u32),
            pos,
            qtype.as_mut_ptr(),
            2u32,
        );
        if pos == 0 {
            die_truncated();
        }
        pos = dns_packet_copy(
            buf.as_mut_ptr() as (*const u8),
            len as (u32),
            pos,
            qclass.as_mut_ptr(),
            2u32,
        );
        if pos == 0 {
            die_truncated();
        }
        if byte::diff(
            qclass.as_mut_ptr(),
            2u32,
            (*b"\0\x01\0").as_ptr() as (*mut u8),
        ) != 0 &&
            (byte::diff(
                qclass.as_mut_ptr(),
                2u32,
                (*b"\0\xFF\0").as_ptr() as (*mut u8),
            ) != 0)
        {
            StrErr::die(
                111i32,
                (*b"axfrdns: fatal: \0").as_ptr(),
                (*b"bogus query: bad class\0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const StrErr),
            );
        }
        qlog(
            ip.as_mut_ptr() as (*const u8),
            port as (u16),
            header.as_mut_ptr() as (*const u8),
            zone as (*const u8),
            qtype.as_mut_ptr() as (*const u8),
            (*b" \0").as_ptr(),
        );
        if byte::diff(
            qtype.as_mut_ptr(),
            2u32,
            (*b"\0\xFC\0").as_ptr() as (*mut u8),
        ) == 0
        {
            case::lowerb(zone, zonelen);
            fdcdb = open_read((*b"data.cdb\0").as_ptr());
            if fdcdb == -1i32 {
                die_cdbread();
            }
            doaxfr(header.as_mut_ptr());
            close(fdcdb);
        } else {
            if response_query(
                zone as (*const u8),
                qtype.as_mut_ptr() as (*const u8),
                qclass.as_mut_ptr() as (*const u8),
            ) == 0
            {
                nomem();
            }
            let _rhs = 4i32;
            let _lhs = &mut *response.offset(2isize);
            *_lhs = (*_lhs as (i32) | _rhs) as (u8);
            case::lowerb(zone, zonelen);
            response_id(header.as_mut_ptr() as (*const u8));
            let _rhs = !128i32;
            let _lhs = &mut *response.offset(3isize);
            *_lhs = (*_lhs as (i32) & _rhs) as (u8);
            if header[2usize] as (i32) & 1i32 == 0 {
                let _rhs = !1i32;
                let _lhs = &mut *response.offset(2isize);
                *_lhs = (*_lhs as (i32) & _rhs) as (u8);
            }
            if respond(zone, qtype.as_mut_ptr(), ip.as_mut_ptr()) == 0 {
                die_outside();
            }
            print(response, response_len);
        }
    }
}
