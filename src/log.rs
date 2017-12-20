use buffer::Buffer;
use buffer_2::BUFFER_2;
use byte;
use errno::{self, Errno};
use libc;
use uint16;
use uint32;

extern "C" {
    static mut cache_motion: usize;
    static mut numqueries: usize;
    static mut tactive: i32;
    static mut uactive: i32;
}

static mut u64: usize = 0usize;

unsafe extern "C" fn string(mut s: *const u8) {
    Buffer::puts(BUFFER_2.as_mut_ptr(), s);
}

unsafe extern "C" fn line() {
    string((*b"\n\0").as_ptr());
    Buffer::flush(BUFFER_2.as_mut_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn log_startup() {
    string((*b"starting\0").as_ptr());
    line();
}

unsafe extern "C" fn u64_print() {
    let mut buf: [u8; 20];
    let mut pos: u32;
    pos = ::std::mem::size_of::<[u8; 20]>() as (u32);
    'loop1: loop {
        if pos == 0 {
            break;
        }
        buf[{
                pos = pos.wrapping_sub(1u32);
                pos
            } as (usize)] = (b'0' as (usize)).wrapping_add(u64.wrapping_rem(10usize)) as (u8);
        u64 = u64.wrapping_div(10usize);
        if u64 == 0 {
            break;
        }
    }
    Buffer::put(
        BUFFER_2.as_mut_ptr(),
        buf.as_mut_ptr().offset(pos as (isize)) as (*const u8),
        ::std::mem::size_of::<[u8; 20]>().wrapping_sub(pos as (usize)) as (u32),
    );
}

unsafe extern "C" fn space() {
    string((*b" \0").as_ptr());
}

unsafe extern "C" fn hex(mut c: u8) {
    Buffer::put(
        BUFFER_2.as_mut_ptr(),
        (*b"0123456789abcdef\0").as_ptr().offset(
            (c as (i32) >> 4i32) as
                (isize),
        ),
        1u32,
    );
    Buffer::put(
        BUFFER_2.as_mut_ptr(),
        (*b"0123456789abcdef\0").as_ptr().offset(
            (c as (i32) & 15i32) as
                (isize),
        ),
        1u32,
    );
}

unsafe extern "C" fn ip(mut i: *const u8) {
    hex(*i.offset(0isize));
    hex(*i.offset(1isize));
    hex(*i.offset(2isize));
    hex(*i.offset(3isize));
}

unsafe extern "C" fn logid(mut id: *const u8) {
    hex(*id.offset(0isize));
    hex(*id.offset(1isize));
}

unsafe extern "C" fn logtype(mut type_: *const u8) {
    let mut u: u16;
    uint16::unpack_big(type_, &mut u as (*mut u16));
    u64 = u as (usize);
    u64_print();
}

unsafe extern "C" fn name(mut q: *const u8) {
    let mut ch: u8;
    let mut state: i32;
    if *q == 0 {
        string((*b".\0").as_ptr());
    } else {
        'loop1: loop {
            if {
                state = *{
                    let _old = q;
                    q = q.offset(1isize);
                    _old
                } as (i32);
                state
            } == 0
            {
                break;
            }
            'loop3: loop {
                if state == 0 {
                    break;
                }
                ch = *{
                    let _old = q;
                    q = q.offset(1isize);
                    _old
                };
                state = state - 1;
                if ch as (i32) <= 32i32 || ch as (i32) > 126i32 {
                    ch = b'?';
                }
                if ch as (i32) >= b'A' as (i32) && (ch as (i32) <= b'Z' as (i32)) {
                    ch = (ch as (i32) + 32i32) as (u8);
                }
                Buffer::put(BUFFER_2.as_mut_ptr(), &mut ch as (*mut u8) as (*const u8), 1u32);
            }
            string((*b".\0").as_ptr());
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn log_query(
    mut qnum: *mut usize,
    mut client: *const u8,
    mut port: u32,
    mut id: *const u8,
    mut q: *const u8,
    mut qtype: *const u8,
) {
    string((*b"query \0").as_ptr());
    u64 = *qnum;
    u64_print();
    space();
    ip(client);
    string((*b":\0").as_ptr());
    hex((port >> 8i32) as (u8));
    hex((port & 255u32) as (u8));
    string((*b":\0").as_ptr());
    logid(id);
    space();
    logtype(qtype);
    space();
    name(q);
    line();
}

#[no_mangle]
pub unsafe extern "C" fn log_querydone(mut qnum: *mut usize, mut len: u32) {
    string((*b"sent \0").as_ptr());
    u64 = *qnum;
    u64_print();
    space();
    u64 = len as (usize);
    u64_print();
    line();
}

#[no_mangle]
pub unsafe extern "C" fn log_querydrop(mut qnum: *mut usize) {
    let mut x = libc::strerror(errno().0);
    string((*b"drop \0").as_ptr());
    u64 = *qnum;
    u64_print();
    space();
    string(x);
    line();
}

#[no_mangle]
pub unsafe extern "C" fn log_tcpopen(mut client: *const u8, mut port: u32) {
    string((*b"tcpopen \0").as_ptr());
    ip(client);
    string((*b":\0").as_ptr());
    hex((port >> 8i32) as (u8));
    hex((port & 255u32) as (u8));
    line();
}

#[no_mangle]
pub unsafe extern "C" fn log_tcpclose(mut client: *const u8, mut port: u32) {
    let mut x = libc::strerror(errno().0);
    string((*b"tcpclose \0").as_ptr());
    ip(client);
    string((*b":\0").as_ptr());
    hex((port >> 8i32) as (u8));
    hex((port & 255u32) as (u8));
    space();
    string(x);
    line();
}

#[no_mangle]
pub unsafe extern "C" fn log_tx(
    mut q: *const u8,
    mut qtype: *const u8,
    mut control: *const u8,
    mut servers: *const u8,
    mut gluelessness: u32,
) {
    let mut i: i32;
    string((*b"tx \0").as_ptr());
    u64 = gluelessness as (usize);
    u64_print();
    space();
    logtype(qtype);
    space();
    name(q);
    space();
    name(control);
    i = 0i32;
    'loop1: loop {
        if !(i < 64i32) {
            break;
        }
        if byte::diff(
            servers.offset(i as (isize)) as (*mut u8),
            4u32,
            (*b"\0\0\0\0\0").as_ptr() as (*mut u8),
        ) != 0
        {
            space();
            ip(servers.offset(i as (isize)));
        }
        i = i + 4i32;
    }
    line();
}

#[no_mangle]
pub unsafe extern "C" fn log_cachedanswer(mut q: *const u8, mut type_: *const u8) {
    string((*b"cached \0").as_ptr());
    logtype(type_);
    space();
    name(q);
    line();
}

#[no_mangle]
pub unsafe extern "C" fn log_cachedcname(mut dn: *const u8, mut dn2: *const u8) {
    string((*b"cached cname \0").as_ptr());
    name(dn);
    space();
    name(dn2);
    line();
}

#[no_mangle]
pub unsafe extern "C" fn log_cachedns(mut control: *const u8, mut ns: *const u8) {
    string((*b"cached ns \0").as_ptr());
    name(control);
    space();
    name(ns);
    line();
}

#[no_mangle]
pub unsafe extern "C" fn log_cachednxdomain(mut dn: *const u8) {
    string((*b"cached nxdomain \0").as_ptr());
    name(dn);
    line();
}

#[no_mangle]
pub unsafe extern "C" fn log_nxdomain(mut server: *const u8, mut q: *const u8, mut ttl: u32) {
    string((*b"nxdomain \0").as_ptr());
    ip(server);
    space();
    u64 = ttl as (usize);
    u64_print();
    space();
    name(q);
    line();
}

#[no_mangle]
pub unsafe extern "C" fn log_nodata(
    mut server: *const u8,
    mut q: *const u8,
    mut qtype: *const u8,
    mut ttl: u32,
) {
    string((*b"nodata \0").as_ptr());
    ip(server);
    space();
    u64 = ttl as (usize);
    u64_print();
    space();
    logtype(qtype);
    space();
    name(q);
    line();
}

#[no_mangle]
pub unsafe extern "C" fn log_lame(
    mut server: *const u8,
    mut control: *const u8,
    mut referral: *const u8,
) {
    string((*b"lame \0").as_ptr());
    ip(server);
    space();
    name(control);
    space();
    name(referral);
    line();
}

#[no_mangle]
pub unsafe extern "C" fn log_servfail(mut dn: *const u8) {
    let mut x = libc::strerror(errno().0);
    string((*b"servfail \0").as_ptr());
    name(dn);
    space();
    string(x);
    line();
}

#[no_mangle]
pub unsafe extern "C" fn log_rr(
    mut server: *const u8,
    mut q: *const u8,
    mut type_: *const u8,
    mut buf: *const u8,
    mut len: u32,
    mut ttl: u32,
) {
    let mut _currentBlock;
    let mut i: i32;
    string((*b"rr \0").as_ptr());
    ip(server);
    space();
    u64 = ttl as (usize);
    u64_print();
    space();
    logtype(type_);
    space();
    name(q);
    space();
    i = 0i32;
    'loop1: loop {
        if !(i as (u32) < len) {
            _currentBlock = 5;
            break;
        }
        hex(*buf.offset(i as (isize)));
        if i > 30i32 {
            _currentBlock = 4;
            break;
        }
        i = i + 1;
    }
    if _currentBlock == 4 {
        string((*b"...\0").as_ptr());
    }
    line();
}

#[no_mangle]
pub unsafe extern "C" fn log_rrns(
    mut server: *const u8,
    mut q: *const u8,
    mut data: *const u8,
    mut ttl: u32,
) {
    string((*b"rr \0").as_ptr());
    ip(server);
    space();
    u64 = ttl as (usize);
    u64_print();
    string((*b" ns \0").as_ptr());
    name(q);
    space();
    name(data);
    line();
}

#[no_mangle]
pub unsafe extern "C" fn log_rrcname(
    mut server: *const u8,
    mut q: *const u8,
    mut data: *const u8,
    mut ttl: u32,
) {
    string((*b"rr \0").as_ptr());
    ip(server);
    space();
    u64 = ttl as (usize);
    u64_print();
    string((*b" cname \0").as_ptr());
    name(q);
    space();
    name(data);
    line();
}

#[no_mangle]
pub unsafe extern "C" fn log_rrptr(
    mut server: *const u8,
    mut q: *const u8,
    mut data: *const u8,
    mut ttl: u32,
) {
    string((*b"rr \0").as_ptr());
    ip(server);
    space();
    u64 = ttl as (usize);
    u64_print();
    string((*b" ptr \0").as_ptr());
    name(q);
    space();
    name(data);
    line();
}

#[no_mangle]
pub unsafe extern "C" fn log_rrmx(
    mut server: *const u8,
    mut q: *const u8,
    mut mx: *const u8,
    mut pref: *const u8,
    mut ttl: u32,
) {
    let mut u: u16;
    string((*b"rr \0").as_ptr());
    ip(server);
    space();
    u64 = ttl as (usize);
    u64_print();
    string((*b" mx \0").as_ptr());
    name(q);
    space();
    uint16::unpack_big(pref, &mut u as (*mut u16));
    u64 = u as (usize);
    u64_print();
    space();
    name(mx);
    line();
}

#[no_mangle]
pub unsafe extern "C" fn log_rrsoa(
    mut server: *const u8,
    mut q: *const u8,
    mut n1: *const u8,
    mut n2: *const u8,
    mut misc: *const u8,
    mut ttl: u32,
) {
    let mut u: u32;
    let mut i: i32;
    string((*b"rr \0").as_ptr());
    ip(server);
    space();
    u64 = ttl as (usize);
    u64_print();
    string((*b" soa \0").as_ptr());
    name(q);
    space();
    name(n1);
    space();
    name(n2);
    i = 0i32;
    'loop1: loop {
        if !(i < 20i32) {
            break;
        }
        uint32::unpack_big(misc.offset(i as (isize)), &mut u as (*mut u32));
        space();
        u64 = u as (usize);
        u64_print();
        i = i + 4i32;
    }
    line();
}

#[no_mangle]
pub unsafe extern "C" fn log_stats() {
    string((*b"stats \0").as_ptr());
    u64 = numqueries;
    u64_print();
    space();
    u64 = cache_motion;
    u64_print();
    space();
    u64 = uactive as (usize);
    u64_print();
    space();
    u64 = tactive as (usize);
    u64_print();
    line();
}
