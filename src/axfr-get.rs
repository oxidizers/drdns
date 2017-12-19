use byte;
use errno::{self, Errno};
use libc;

extern "C" {
    fn __swbuf(arg1: i32, arg2: *mut __sFILE) -> i32;
    fn buffer_flush(arg1: *mut buffer) -> i32;
    fn buffer_get(arg1: *mut buffer, arg2: *mut u8, arg3: u32) -> i32;
    fn buffer_init(
        arg1: *mut buffer,
        arg2: unsafe extern "C" fn() -> i32,
        arg3: i32,
        arg4: *mut u8,
        arg5: u32,
    );
    fn buffer_put(arg1: *mut buffer, arg2: *const u8, arg3: u32) -> i32;
    fn buffer_unixread(arg1: i32, arg2: *mut u8, arg3: u32) -> i32;
    fn buffer_unixwrite(arg1: i32, arg2: *const u8, arg3: u32) -> i32;
    fn close(arg1: i32) -> i32;
    fn dns_domain_equal(arg1: *const u8, arg2: *const u8) -> i32;
    fn dns_domain_fromdot(arg1: *mut *mut u8, arg2: *const u8, arg3: u32) -> i32;
    fn dns_domain_length(arg1: *const u8) -> u32;
    fn dns_domain_suffix(arg1: *const u8, arg2: *const u8) -> i32;
    fn dns_domain_todot_cat(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn dns_packet_copy(arg1: *const u8, arg2: u32, arg3: u32, arg4: *mut u8, arg5: u32) -> u32;
    fn dns_packet_getname(arg1: *const u8, arg2: u32, arg3: u32, arg4: *mut *mut u8) -> u32;
    fn dns_packet_skipname(arg1: *const u8, arg2: u32, arg3: u32) -> u32;
    fn fsync(arg1: i32) -> i32;
    fn getln(arg1: *mut buffer, arg2: *mut stralloc, arg3: *mut i32, arg4: i32) -> i32;
    fn ip4_fmt(arg1: *mut u8, arg2: *const u8) -> u32;
    fn open_read(arg1: *const u8) -> i32;
    fn open_trunc(arg1: *const u8) -> i32;
    fn rename(__old: *const u8, __new: *const u8) -> i32;
    fn scan_ulong(arg1: *const u8, arg2: *mut usize) -> u32;
    fn str_len(arg1: *const u8) -> u32;
    fn stralloc_append(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn stralloc_catb(arg1: *mut stralloc, arg2: *const u8, arg3: u32) -> i32;
    fn stralloc_cats(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn stralloc_catulong0(arg1: *mut stralloc, arg2: usize, arg3: u32) -> i32;
    fn stralloc_copyb(arg1: *mut stralloc, arg2: *const u8, arg3: u32) -> i32;
    fn stralloc_copys(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn stralloc_ready(arg1: *mut stralloc, arg2: u32) -> i32;
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
    fn timeoutread(t: i32, fd: i32, buf: *mut u8, len: i32) -> i32;
    fn timeoutwrite(t: i32, fd: i32, buf: *mut u8, len: i32) -> i32;
    fn uint16_pack_big(arg1: *mut u8, arg2: u16);
    fn uint16_unpack_big(arg1: *const u8, arg2: *mut u16);
    fn uint32_unpack_big(arg1: *const u8, arg2: *mut u32);
}

enum __sFILEX {
}

#[derive(Copy)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut u8,
    pub _size: i32,
}

impl Clone for __sbuf {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut u8,
    pub _r: i32,
    pub _w: i32,
    pub _flags: i16,
    pub _file: i16,
    pub _bf: __sbuf,
    pub _lbfsize: i32,
    pub _cookie: *mut ::std::os::raw::c_void,
    pub _close: unsafe extern "C" fn(*mut ::std::os::raw::c_void) -> i32,
    pub _read: unsafe extern "C" fn(*mut ::std::os::raw::c_void, *mut u8, i32) -> i32,
    pub _seek: unsafe extern "C" fn(*mut ::std::os::raw::c_void, isize, i32) -> isize,
    pub _write: unsafe extern "C" fn(*mut ::std::os::raw::c_void, *const u8, i32) -> i32,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: i32,
    pub _ubuf: [u8; 3],
    pub _nbuf: [u8; 1],
    pub _lb: __sbuf,
    pub _blksize: i32,
    pub _offset: isize,
}

impl Clone for __sFILE {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn __sputc(mut _c: i32, mut _p: *mut __sFILE) -> i32 {
    if {
        (*_p)._w = (*_p)._w - 1;
        (*_p)._w
    } >= 0i32 || (*_p)._w >= (*_p)._lbfsize && (_c as (u8) as (i32) != b'\n' as (i32))
    {
        ({
             let _rhs = _c;
             let _lhs = &mut *{
                 let _old = (*_p)._p;
                 (*_p)._p = (*_p)._p.offset(1isize);
                 _old
             };
             *_lhs = _rhs as (u8);
             *_lhs
         }) as (i32)
    } else {
        __swbuf(_c, _p)
    }
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
pub unsafe extern "C" fn die_usage() {
    strerr_die(
        100i32,
        (*b"axfr-get: usage: axfr-get zone fn fn.tmp\0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const strerr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn die_generate() {
    strerr_die(
        111i32,
        (*b"axfr-get: fatal: \0").as_ptr(),
        (*b"unable to generate AXFR query: \0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        &mut strerr_sys as (*mut strerr) as (*const strerr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn die_parse() {
    strerr_die(
        111i32,
        (*b"axfr-get: fatal: \0").as_ptr(),
        (*b"unable to parse AXFR results: \0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        &mut strerr_sys as (*mut strerr) as (*const strerr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn x_copy(
    mut buf: *mut u8,
    mut len: u32,
    mut pos: u32,
    mut out: *mut u8,
    mut outlen: u32,
) -> u32 {
    pos = dns_packet_copy(buf as (*const u8), len, pos, out, outlen);
    if pos == 0 {
        die_parse();
    }
    pos
}

#[no_mangle]
pub unsafe extern "C" fn x_getname(
    mut buf: *mut u8,
    mut len: u32,
    mut pos: u32,
    mut out: *mut *mut u8,
) -> u32 {
    pos = dns_packet_getname(buf as (*const u8), len, pos, out);
    if pos == 0 {
        die_parse();
    }
    pos
}

#[no_mangle]
pub unsafe extern "C" fn x_skipname(mut buf: *mut u8, mut len: u32, mut pos: u32) -> u32 {
    pos = dns_packet_skipname(buf as (*const u8), len, pos);
    if pos == 0 {
        die_parse();
    }
    pos
}

static mut zone: *mut u8 = 0 as (*mut u8);

#[no_mangle]
pub static mut zonelen: u32 = 0u32;

#[no_mangle]
pub static mut fn_: *mut u8 = 0 as (*mut u8);

#[no_mangle]
pub static mut fntmp: *mut u8 = 0 as (*mut u8);

#[no_mangle]
pub unsafe extern "C" fn die_netread() {
    strerr_die(
        111i32,
        (*b"axfr-get: fatal: \0").as_ptr(),
        (*b"unable to read from network: \0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        &mut strerr_sys as (*mut strerr) as (*const strerr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn die_netwrite() {
    strerr_die(
        111i32,
        (*b"axfr-get: fatal: \0").as_ptr(),
        (*b"unable to write to network: \0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        &mut strerr_sys as (*mut strerr) as (*const strerr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn die_read() {
    strerr_die(
        111i32,
        (*b"axfr-get: fatal: \0").as_ptr(),
        (*b"unable to read \0").as_ptr(),
        fn_ as (*const u8),
        (*b": \0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        &mut strerr_sys as (*mut strerr) as (*const strerr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn die_write() {
    strerr_die(
        111i32,
        (*b"axfr-get: fatal: \0").as_ptr(),
        (*b"unable to write \0").as_ptr(),
        fntmp as (*const u8),
        (*b": \0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        &mut strerr_sys as (*mut strerr) as (*const strerr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn saferead(mut fd: i32, mut buf: *mut u8, mut len: u32) -> i32 {
    let mut r: i32;
    r = timeoutread(60i32, fd, buf, len as (i32));
    if r == 0i32 {
        errno::set_errno(Errno(libc::EPROTO));
        die_parse();
    }
    if r <= 0i32 {
        die_netread();
    }
    r
}

#[no_mangle]
pub unsafe extern "C" fn safewrite(mut fd: i32, mut buf: *mut u8, mut len: u32) -> i32 {
    let mut r: i32;
    r = timeoutwrite(60i32, fd, buf, len as (i32));
    if r <= 0i32 {
        die_netwrite();
    }
    r
}

#[no_mangle]
pub static mut netreadspace: [u8; 1024] = [0u8; 1024];

#[derive(Copy)]
#[repr(C)]
pub struct buffer {
    pub x: *mut u8,
    pub p: u32,
    pub n: u32,
    pub fd: i32,
    pub op: unsafe extern "C" fn() -> i32,
}

impl Clone for buffer {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub static mut netread: buffer = buffer {
    x: netreadspace.as_mut_ptr(),
    p: 0u32,
    n: ::std::mem::size_of::<[u8; 1024]>() as (u32),
    fd: 6i32,
    op: saferead as (unsafe extern "C" fn() -> i32),
};

#[no_mangle]
pub static mut netwritespace: [u8; 1024] = [0u8; 1024];

#[no_mangle]
pub static mut netwrite: buffer = buffer {
    x: netwritespace.as_mut_ptr(),
    p: 0u32,
    n: ::std::mem::size_of::<[u8; 1024]>() as (u32),
    fd: 7i32,
    op: safewrite as (unsafe extern "C" fn() -> i32),
};

#[no_mangle]
pub unsafe extern "C" fn netget(mut buf: *mut u8, mut len: u32) {
    let mut r: i32;
    'loop1: loop {
        if !(len > 0u32) {
            break;
        }
        r = buffer_get(&mut netread as (*mut buffer), buf, len);
        buf = buf.offset(r as (isize));
        len = len.wrapping_sub(r as (u32));
    }
}

#[no_mangle]
pub static mut fd: i32 = 0i32;

#[no_mangle]
pub static mut b: buffer = buffer {
    x: 0 as (*mut u8),
    p: 0u32,
    n: 0u32,
    fd: 0i32,
    op: 0 as (unsafe extern "C" fn() -> i32),
};

#[no_mangle]
pub static mut bspace: [u8; 1024] = [0u8; 1024];

#[no_mangle]
pub unsafe extern "C" fn put(mut buf: *mut u8, mut len: u32) {
    if buffer_put(&mut b as (*mut buffer), buf as (*const u8), len) == -1i32 {
        die_write();
    }
}

#[no_mangle]
pub unsafe extern "C" fn printable(mut ch: u8) -> i32 {
    if ch as (i32) == b'.' as (i32) {
        1i32
    } else if ch as (i32) >= b'a' as (i32) && (ch as (i32) <= b'z' as (i32)) {
        1i32
    } else if ch as (i32) >= b'0' as (i32) && (ch as (i32) <= b'9' as (i32)) {
        1i32
    } else if ch as (i32) >= b'A' as (i32) && (ch as (i32) <= b'Z' as (i32)) {
        1i32
    } else if ch as (i32) == b'-' as (i32) {
        1i32
    } else {
        0i32
    }
}

static mut d1: *mut u8 = 0 as (*mut u8);

static mut d2: *mut u8 = 0 as (*mut u8);

static mut d3: *mut u8 = 0 as (*mut u8);

#[derive(Copy)]
#[repr(C)]
pub struct stralloc {
    pub s: *mut u8,
    pub len: u32,
    pub a: u32,
}

impl Clone for stralloc {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub static mut line: stralloc = stralloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

#[no_mangle]
pub static mut match_: i32 = 0i32;

#[no_mangle]
pub static mut numsoa: i32 = 0i32;

#[no_mangle]
pub unsafe extern "C" fn doit(mut buf: *mut u8, mut len: u32, mut pos: u32) -> u32 {
    let mut _currentBlock;
    let mut data: [u8; 20];
    let mut ttl: u32;
    let mut dlen: u16;
    let mut typenum: u16;
    let mut u32: u32;
    let mut i: i32;
    pos = x_getname(buf, len, pos, &mut d1 as (*mut *mut u8));
    pos = x_copy(buf, len, pos, data.as_mut_ptr(), 10u32);
    uint16_unpack_big(data.as_mut_ptr() as (*const u8), &mut typenum as (*mut u16));
    uint32_unpack_big(
        data.as_mut_ptr().offset(4isize) as (*const u8),
        &mut ttl as (*mut u32),
    );
    uint16_unpack_big(
        data.as_mut_ptr().offset(8isize) as (*const u8),
        &mut dlen as (*mut u16),
    );
    if len.wrapping_sub(pos) < dlen as (u32) {
        errno::set_errno(Errno(libc::EPROTO));
        0u32
    } else {
        len = pos.wrapping_add(dlen as (u32));
        (if dns_domain_suffix(d1 as (*const u8), zone as (*const u8)) == 0 {
             len
         } else if byte::diff(
            data.as_mut_ptr().offset(2isize),
            2u32,
            (*b"\0\x01\0").as_ptr() as (*mut u8),
        ) != 0
        {
             len
         } else {
             if byte::diff(
                data.as_mut_ptr(),
                2u32,
                (*b"\0\x06\0").as_ptr() as (*mut u8),
            ) == 0
            {
                 if {
                     numsoa = numsoa + 1;
                     numsoa
                 } >= 2i32
                {
                     return len;
                 } else {
                     pos = x_getname(buf, len, pos, &mut d2 as (*mut *mut u8));
                     pos = x_getname(buf, len, pos, &mut d3 as (*mut *mut u8));
                     x_copy(buf, len, pos, data.as_mut_ptr(), 20u32);
                     uint32_unpack_big(data.as_mut_ptr() as (*const u8), &mut u32 as (*mut u32));
                     if stralloc_copys(&mut line as (*mut stralloc), (*b"#\0").as_ptr()) == 0 {
                         return 0u32;
                     } else if stralloc_catulong0(
                        &mut line as (*mut stralloc),
                        u32 as (usize),
                        0u32,
                    ) == 0
                    {
                         return 0u32;
                     } else if stralloc_cats(
                        &mut line as (*mut stralloc),
                        (*b" auto axfr-get\n\0").as_ptr(),
                    ) == 0
                    {
                         return 0u32;
                     } else if stralloc_cats(&mut line as (*mut stralloc), (*b"Z\0").as_ptr()) ==
                                0
                    {
                         return 0u32;
                     } else if dns_domain_todot_cat(
                        &mut line as (*mut stralloc),
                        d1 as (*const u8),
                    ) == 0
                    {
                         return 0u32;
                     } else if stralloc_cats(&mut line as (*mut stralloc), (*b":\0").as_ptr()) ==
                                0
                    {
                         return 0u32;
                     } else if dns_domain_todot_cat(
                        &mut line as (*mut stralloc),
                        d2 as (*const u8),
                    ) == 0
                    {
                         return 0u32;
                     } else if stralloc_cats(
                        &mut line as (*mut stralloc),
                        (*b".:\0").as_ptr(),
                    ) == 0
                    {
                         return 0u32;
                     } else if dns_domain_todot_cat(
                        &mut line as (*mut stralloc),
                        d3 as (*const u8),
                    ) == 0
                    {
                         return 0u32;
                     } else if stralloc_cats(&mut line as (*mut stralloc), (*b".\0").as_ptr()) ==
                                0
                    {
                         return 0u32;
                     } else {
                         i = 0i32;
                         'loop97: loop {
                             if !(i < 5i32) {
                                 _currentBlock = 98;
                                 break;
                             }
                             uint32_unpack_big(
                                data.as_mut_ptr().offset((4i32 * i) as (isize)) as (*const u8),
                                &mut u32 as (*mut u32),
                            );
                             if stralloc_cats(
                                &mut line as (*mut stralloc),
                                (*b":\0").as_ptr(),
                            ) == 0
                            {
                                 _currentBlock = 109;
                                 break;
                             }
                             if stralloc_catulong0(
                                &mut line as (*mut stralloc),
                                u32 as (usize),
                                0u32,
                            ) == 0
                            {
                                 _currentBlock = 108;
                                 break;
                             }
                             i = i + 1;
                         }
                         if _currentBlock == 98 {
                         } else if _currentBlock == 108 {
                             return 0u32;
                         } else {
                             return 0u32;
                         }
                     }
                 }
             } else if byte::diff(
                data.as_mut_ptr(),
                2u32,
                (*b"\0\x02\0").as_ptr() as (*mut u8),
            ) == 0
            {
                 if stralloc_copys(&mut line as (*mut stralloc), (*b"&\0").as_ptr()) == 0 {
                     return 0u32;
                 } else if byte::diff(d1, 2u32, (*b"\x01*\0").as_ptr() as (*mut u8)) == 0 {
                     errno::set_errno(Errno(libc::EPROTO));
                     return 0u32;
                 } else if dns_domain_todot_cat(
                    &mut line as (*mut stralloc),
                    d1 as (*const u8),
                ) == 0
                {
                     return 0u32;
                 } else if stralloc_cats(&mut line as (*mut stralloc), (*b"::\0").as_ptr()) == 0 {
                     return 0u32;
                 } else {
                     x_getname(buf, len, pos, &mut d1 as (*mut *mut u8));
                     if dns_domain_todot_cat(&mut line as (*mut stralloc), d1 as (*const u8)) == 0 {
                         return 0u32;
                     } else if stralloc_cats(&mut line as (*mut stralloc), (*b".\0").as_ptr()) ==
                                0
                    {
                         return 0u32;
                     }
                 }
             } else if byte::diff(
                data.as_mut_ptr(),
                2u32,
                (*b"\0\x05\0").as_ptr() as (*mut u8),
            ) == 0
            {
                 if stralloc_copys(&mut line as (*mut stralloc), (*b"C\0").as_ptr()) == 0 {
                     return 0u32;
                 } else if dns_domain_todot_cat(
                    &mut line as (*mut stralloc),
                    d1 as (*const u8),
                ) == 0
                {
                     return 0u32;
                 } else if stralloc_cats(&mut line as (*mut stralloc), (*b":\0").as_ptr()) == 0 {
                     return 0u32;
                 } else {
                     x_getname(buf, len, pos, &mut d1 as (*mut *mut u8));
                     if dns_domain_todot_cat(&mut line as (*mut stralloc), d1 as (*const u8)) == 0 {
                         return 0u32;
                     } else if stralloc_cats(&mut line as (*mut stralloc), (*b".\0").as_ptr()) ==
                                0
                    {
                         return 0u32;
                     }
                 }
             } else if byte::diff(
                data.as_mut_ptr(),
                2u32,
                (*b"\0\x0C\0").as_ptr() as (*mut u8),
            ) == 0
            {
                 if stralloc_copys(&mut line as (*mut stralloc), (*b"^\0").as_ptr()) == 0 {
                     return 0u32;
                 } else if dns_domain_todot_cat(
                    &mut line as (*mut stralloc),
                    d1 as (*const u8),
                ) == 0
                {
                     return 0u32;
                 } else if stralloc_cats(&mut line as (*mut stralloc), (*b":\0").as_ptr()) == 0 {
                     return 0u32;
                 } else {
                     x_getname(buf, len, pos, &mut d1 as (*mut *mut u8));
                     if dns_domain_todot_cat(&mut line as (*mut stralloc), d1 as (*const u8)) == 0 {
                         return 0u32;
                     } else if stralloc_cats(&mut line as (*mut stralloc), (*b".\0").as_ptr()) ==
                                0
                    {
                         return 0u32;
                     }
                 }
             } else if byte::diff(
                data.as_mut_ptr(),
                2u32,
                (*b"\0\x0F\0").as_ptr() as (*mut u8),
            ) == 0
            {
                 let mut dist: u16;
                 if stralloc_copys(&mut line as (*mut stralloc), (*b"@\0").as_ptr()) == 0 {
                     return 0u32;
                 } else if dns_domain_todot_cat(
                    &mut line as (*mut stralloc),
                    d1 as (*const u8),
                ) == 0
                {
                     return 0u32;
                 } else if stralloc_cats(&mut line as (*mut stralloc), (*b"::\0").as_ptr()) == 0 {
                     return 0u32;
                 } else {
                     pos = x_copy(buf, len, pos, data.as_mut_ptr(), 2u32);
                     uint16_unpack_big(data.as_mut_ptr() as (*const u8), &mut dist as (*mut u16));
                     x_getname(buf, len, pos, &mut d1 as (*mut *mut u8));
                     if dns_domain_todot_cat(&mut line as (*mut stralloc), d1 as (*const u8)) == 0 {
                         return 0u32;
                     } else if stralloc_cats(
                        &mut line as (*mut stralloc),
                        (*b".:\0").as_ptr(),
                    ) == 0
                    {
                         return 0u32;
                     } else if stralloc_catulong0(
                        &mut line as (*mut stralloc),
                        dist as (usize),
                        0u32,
                    ) == 0
                    {
                         return 0u32;
                     }
                 }
             } else if byte::diff(
                data.as_mut_ptr(),
                2u32,
                (*b"\0\x01\0").as_ptr() as (*mut u8),
            ) == 0 && (dlen as (i32) == 4i32)
            {
                 let mut ipstr: [u8; 20];
                 if stralloc_copys(&mut line as (*mut stralloc), (*b"+\0").as_ptr()) == 0 {
                     return 0u32;
                 } else if dns_domain_todot_cat(
                    &mut line as (*mut stralloc),
                    d1 as (*const u8),
                ) == 0
                {
                     return 0u32;
                 } else if stralloc_cats(&mut line as (*mut stralloc), (*b":\0").as_ptr()) == 0 {
                     return 0u32;
                 } else {
                     x_copy(buf, len, pos, data.as_mut_ptr(), 4u32);
                     if stralloc_catb(
                        &mut line as (*mut stralloc),
                        ipstr.as_mut_ptr() as (*const u8),
                        ip4_fmt(ipstr.as_mut_ptr(), data.as_mut_ptr() as (*const u8)),
                    ) == 0
                    {
                         return 0u32;
                     }
                 }
             } else {
                 let mut ch: u8;
                 let mut ch2: u8;
                 if stralloc_copys(&mut line as (*mut stralloc), (*b":\0").as_ptr()) == 0 {
                     return 0u32;
                 } else if dns_domain_todot_cat(
                    &mut line as (*mut stralloc),
                    d1 as (*const u8),
                ) == 0
                {
                     return 0u32;
                 } else if stralloc_cats(&mut line as (*mut stralloc), (*b":\0").as_ptr()) == 0 {
                     return 0u32;
                 } else if stralloc_catulong0(
                    &mut line as (*mut stralloc),
                    typenum as (usize),
                    0u32,
                ) == 0
                {
                     return 0u32;
                 } else if stralloc_cats(&mut line as (*mut stralloc), (*b":\0").as_ptr()) == 0 {
                     return 0u32;
                 } else {
                     i = 0i32;
                     'loop15: loop {
                         if !(i < dlen as (i32)) {
                             _currentBlock = 98;
                             break;
                         }
                         pos = x_copy(buf, len, pos, data.as_mut_ptr(), 1u32);
                         ch = data[0usize];
                         if printable(ch) != 0 {
                             if stralloc_catb(
                                &mut line as (*mut stralloc),
                                &mut ch as (*mut u8) as (*const u8),
                                1u32,
                            ) == 0
                            {
                                 _currentBlock = 27;
                                 break;
                             }
                         } else {
                             if stralloc_cats(
                                &mut line as (*mut stralloc),
                                (*b"\\\0").as_ptr(),
                            ) == 0
                            {
                                 _currentBlock = 24;
                                 break;
                             }
                             ch2 = (b'0' as (i32) + (ch as (i32) >> 6i32 & 7i32)) as (u8);
                             if stralloc_catb(
                                &mut line as (*mut stralloc),
                                &mut ch2 as (*mut u8) as (*const u8),
                                1u32,
                            ) == 0
                            {
                                 _currentBlock = 23;
                                 break;
                             }
                             ch2 = (b'0' as (i32) + (ch as (i32) >> 3i32 & 7i32)) as (u8);
                             if stralloc_catb(
                                &mut line as (*mut stralloc),
                                &mut ch2 as (*mut u8) as (*const u8),
                                1u32,
                            ) == 0
                            {
                                 _currentBlock = 22;
                                 break;
                             }
                             ch2 = (b'0' as (i32) + (ch as (i32) & 7i32)) as (u8);
                             if stralloc_catb(
                                &mut line as (*mut stralloc),
                                &mut ch2 as (*mut u8) as (*const u8),
                                1u32,
                            ) == 0
                            {
                                 _currentBlock = 21;
                                 break;
                             }
                         }
                         i = i + 1;
                     }
                     if _currentBlock == 98 {
                     } else if _currentBlock == 21 {
                         return 0u32;
                     } else if _currentBlock == 22 {
                         return 0u32;
                     } else if _currentBlock == 23 {
                         return 0u32;
                     } else if _currentBlock == 24 {
                         return 0u32;
                     } else {
                         return 0u32;
                     }
                 }
             }
             (if stralloc_cats(&mut line as (*mut stralloc), (*b":\0").as_ptr()) == 0 {
                  0u32
              } else if stralloc_catulong0(&mut line as (*mut stralloc), ttl as (usize), 0u32) ==
                         0
            {
                  0u32
              } else if stralloc_cats(&mut line as (*mut stralloc), (*b"\n\0").as_ptr()) == 0 {
                  0u32
              } else {
                  put(line.s, line.len);
                  len
              })
         })
    }
}

#[no_mangle]
pub static mut packet: stralloc = stralloc {
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
    let mut out: [u8; 20];
    let mut u: usize;
    let mut dlen: u16;
    let mut pos: u32;
    let mut oldserial: u32 = 0u32;
    let mut newserial: u32 = 0u32;
    let mut numqueries: u16;
    let mut numanswers: u16;
    if (*argv).is_null() {
        die_usage();
    }
    if (*{
            argv = argv.offset(1isize);
            argv
        }).is_null()
    {
        die_usage();
    }
    if dns_domain_fromdot(
        &mut zone as (*mut *mut u8),
        *argv as (*const u8),
        str_len(*argv as (*const u8)),
    ) == 0
    {
        die_generate();
    }
    zonelen = dns_domain_length(zone as (*const u8));
    if (*{
            argv = argv.offset(1isize);
            argv
        }).is_null()
    {
        die_usage();
    }
    fn_ = *argv;
    if (*{
            argv = argv.offset(1isize);
            argv
        }).is_null()
    {
        die_usage();
    }
    fntmp = *argv;
    fd = open_read(fn_ as (*const u8));
    if fd == -1i32 {
        if errno::errno() != Errno(libc::ENOENT) {
            die_read();
        }
    } else {
        buffer_init(
            &mut b as (*mut buffer),
            buffer_unixread as (unsafe extern "C" fn() -> i32),
            fd,
            bspace.as_mut_ptr(),
            ::std::mem::size_of::<[u8; 1024]>() as (u32),
        );
        if getln(
            &mut b as (*mut buffer),
            &mut line as (*mut stralloc),
            &mut match_ as (*mut i32),
            b'\n' as (i32),
        ) == -1i32
        {
            die_read();
        }
        if stralloc_append(&mut line as (*mut stralloc), (*b"\0").as_ptr()) == 0 {
            die_read();
        }
        if *line.s.offset(0isize) as (i32) == b'#' as (i32) {
            scan_ulong(line.s.offset(1isize) as (*const u8), &mut u as (*mut usize));
            oldserial = u as (u32);
        }
        close(fd);
    }
    if stralloc_copyb(
        &mut packet as (*mut stralloc),
        (*b"\0\0\0\0\0\x01\0\0\0\0\0\0\0").as_ptr(),
        12u32,
    ) == 0
    {
        die_generate();
    }
    if stralloc_catb(&mut packet as (*mut stralloc), zone as (*const u8), zonelen) == 0 {
        die_generate();
    }
    if stralloc_catb(
        &mut packet as (*mut stralloc),
        (*b"\0\x06\0\x01\0").as_ptr(),
        4u32,
    ) == 0
    {
        die_generate();
    }
    uint16_pack_big(out.as_mut_ptr(), packet.len as (u16));
    buffer_put(
        &mut netwrite as (*mut buffer),
        out.as_mut_ptr() as (*const u8),
        2u32,
    );
    buffer_put(
        &mut netwrite as (*mut buffer),
        packet.s as (*const u8),
        packet.len,
    );
    buffer_flush(&mut netwrite as (*mut buffer));
    netget(out.as_mut_ptr(), 2u32);
    uint16_unpack_big(out.as_mut_ptr() as (*const u8), &mut dlen as (*mut u16));
    if stralloc_ready(&mut packet as (*mut stralloc), dlen as (u32)) == 0 {
        die_parse();
    }
    netget(packet.s, dlen as (u32));
    packet.len = dlen as (u32);
    pos = x_copy(packet.s, packet.len, 0u32, out.as_mut_ptr(), 12u32);
    uint16_unpack_big(
        out.as_mut_ptr().offset(4isize) as (*const u8),
        &mut numqueries as (*mut u16),
    );
    uint16_unpack_big(
        out.as_mut_ptr().offset(6isize) as (*const u8),
        &mut numanswers as (*mut u16),
    );
    'loop29: loop {
        if numqueries == 0 {
            break;
        }
        numqueries = (numqueries as (i32) - 1) as (u16);
        pos = x_skipname(packet.s, packet.len, pos);
        pos = pos.wrapping_add(4u32);
    }
    if numanswers == 0 {
        errno::set_errno(Errno(libc::EPROTO));
        die_parse();
    }
    pos = x_getname(packet.s, packet.len, pos, &mut d1 as (*mut *mut u8));
    if dns_domain_equal(zone as (*const u8), d1 as (*const u8)) == 0 {
        errno::set_errno(Errno(libc::EPROTO));
        die_parse();
    }
    pos = x_copy(packet.s, packet.len, pos, out.as_mut_ptr(), 10u32);
    if byte::diff(
        out.as_mut_ptr(),
        4u32,
        (*b"\0\x06\0\x01\0").as_ptr() as (*mut u8),
    ) != 0
    {
        errno::set_errno(Errno(libc::EPROTO));
        die_parse();
    }
    pos = x_skipname(packet.s, packet.len, pos);
    pos = x_skipname(packet.s, packet.len, pos);
    pos = x_copy(packet.s, packet.len, pos, out.as_mut_ptr(), 4u32);
    uint32_unpack_big(
        out.as_mut_ptr() as (*const u8),
        &mut newserial as (*mut u32),
    );
    if oldserial != 0 && (newserial != 0) {
        if oldserial == newserial {
            libc::_exit(0i32);
        }
    }
    fd = open_trunc(fntmp as (*const u8));
    if fd == -1i32 {
        die_write();
    }
    buffer_init(
        &mut b as (*mut buffer),
        buffer_unixwrite as (unsafe extern "C" fn() -> i32),
        fd,
        bspace.as_mut_ptr(),
        ::std::mem::size_of::<[u8; 1024]>() as (u32),
    );
    if stralloc_copyb(
        &mut packet as (*mut stralloc),
        (*b"\0\0\0\0\0\x01\0\0\0\0\0\0\0").as_ptr(),
        12u32,
    ) == 0
    {
        die_generate();
    }
    if stralloc_catb(&mut packet as (*mut stralloc), zone as (*const u8), zonelen) == 0 {
        die_generate();
    }
    if stralloc_catb(
        &mut packet as (*mut stralloc),
        (*b"\0\xFC\0\x01\0").as_ptr(),
        4u32,
    ) == 0
    {
        die_generate();
    }
    uint16_pack_big(out.as_mut_ptr(), packet.len as (u16));
    buffer_put(
        &mut netwrite as (*mut buffer),
        out.as_mut_ptr() as (*const u8),
        2u32,
    );
    buffer_put(
        &mut netwrite as (*mut buffer),
        packet.s as (*const u8),
        packet.len,
    );
    buffer_flush(&mut netwrite as (*mut buffer));
    numsoa = 0i32;
    'loop48: loop {
        if !(numsoa < 2i32) {
            break;
        }
        netget(out.as_mut_ptr(), 2u32);
        uint16_unpack_big(out.as_mut_ptr() as (*const u8), &mut dlen as (*mut u16));
        if stralloc_ready(&mut packet as (*mut stralloc), dlen as (u32)) == 0 {
            die_parse();
        }
        netget(packet.s, dlen as (u32));
        packet.len = dlen as (u32);
        pos = x_copy(packet.s, packet.len, 0u32, out.as_mut_ptr(), 12u32);
        uint16_unpack_big(
            out.as_mut_ptr().offset(4isize) as (*const u8),
            &mut numqueries as (*mut u16),
        );
        'loop61: loop {
            if numqueries == 0 {
                break;
            }
            numqueries = (numqueries as (i32) - 1) as (u16);
            pos = x_skipname(packet.s, packet.len, pos);
            pos = pos.wrapping_add(4u32);
        }
        'loop62: loop {
            if !(pos < packet.len) {
                break;
            }
            pos = doit(packet.s, packet.len, pos);
            if !(pos == 0) {
                continue;
            }
            die_parse();
        }
    }
    if buffer_flush(&mut b as (*mut buffer)) == -1i32 {
        die_write();
    }
    if fsync(fd) == -1i32 {
        die_write();
    }
    if close(fd) == -1i32 {
        die_write();
    }
    if rename(fntmp as (*const u8), fn_ as (*const u8)) == -1i32 {
        strerr_die(
            111i32,
            (*b"axfr-get: fatal: \0").as_ptr(),
            (*b"unable to move \0").as_ptr(),
            fntmp as (*const u8),
            (*b" to \0").as_ptr(),
            fn_ as (*const u8),
            (*b": \0").as_ptr(),
            &mut strerr_sys as (*mut strerr) as (*const strerr),
        );
    }
    libc::_exit(0i32);
}
