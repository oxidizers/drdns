use buffer::{self, Buffer};
use byte;
use libc;
use stralloc::StrAlloc;
use strerr::{StrErr, STRERR_SYS};
use uint16;
use uint32;

extern "C" {
    fn __swbuf(arg1: i32, arg2: *mut __sFILE) -> i32;
    fn case_lowerb(arg1: *mut u8, arg2: u32);
    fn cdb_make_add(
        arg1: *mut cdb_make,
        arg2: *const u8,
        arg3: u32,
        arg4: *const u8,
        arg5: u32,
    ) -> i32;
    fn cdb_make_finish(arg1: *mut cdb_make) -> i32;
    fn cdb_make_start(arg1: *mut cdb_make, arg2: i32) -> i32;
    fn close(arg1: i32) -> i32;
    fn dns_domain_fromdot(arg1: *mut *mut u8, arg2: *const u8, arg3: u32) -> i32;
    fn dns_domain_length(arg1: *const u8) -> u32;
    fn dns_name4_domain(arg1: *mut u8, arg2: *const u8);
    fn fmt_ulong(arg1: *mut u8, arg2: usize) -> u32;
    fn fstat(arg1: i32, arg2: *mut stat) -> i32;
    fn fsync(arg1: i32) -> i32;
    fn getln(arg1: *mut Buffer, arg2: *mut StrAlloc, arg3: *mut i32, arg4: i32) -> i32;
    fn ip4_scan(arg1: *const u8, arg2: *mut u8) -> u32;
    fn open_read(arg1: *const u8) -> i32;
    fn open_trunc(arg1: *const u8) -> i32;
    fn rename(__old: *const u8, __new: *const u8) -> i32;
    fn scan_ulong(arg1: *const u8, arg2: *mut usize) -> u32;
    fn umask(arg1: u16) -> u16;
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

#[no_mangle]
pub unsafe extern "C" fn die_datatmp() {
    StrErr::die(
        111i32,
        (*b"tinydns-data: fatal: \0").as_ptr(),
        (*b"unable to create data.tmp: \0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn nomem() {
    StrErr::die(
        111i32,
        (*b"tinydns-data: fatal: \0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn ttdparse(mut sa: *mut StrAlloc, mut ttd: *mut u8) {
    let mut i: u32;
    let mut ch: u8;
    byte::zero(ttd, 8u32);
    i = 0u32;
    'loop1: loop {
        if !(i < 16u32 && (i < (*sa).len)) {
            break;
        }
        ch = *(*sa).s.offset(i as (isize));
        if ch as (i32) >= b'0' as (i32) && (ch as (i32) <= b'9' as (i32)) {
            ch = (ch as (i32) - b'0' as (i32)) as (u8);
        } else if ch as (i32) >= b'a' as (i32) && (ch as (i32) <= b'f' as (i32)) {
            ch = (ch as (i32) - (b'a' as (i32) - 10i32)) as (u8);
        } else {
            ch = 0u8;
        }
        if i & 1u32 == 0 {
            ch = (ch as (i32) << 4i32) as (u8);
        }
        let _rhs = ch;
        let _lhs = &mut *ttd.offset((i >> 1i32) as (isize));
        *_lhs = (*_lhs as (i32) | _rhs as (i32)) as (u8);
        i = i.wrapping_add(1u32);
    }
}

#[no_mangle]
pub unsafe extern "C" fn locparse(mut sa: *mut StrAlloc, mut loc: *mut u8) {
    *loc.offset(0isize) = if (*sa).len > 0u32 {
        *(*sa).s.offset(0isize) as (i32)
    } else {
        0i32
    } as (u8);
    *loc.offset(1isize) = if (*sa).len > 1u32 {
        *(*sa).s.offset(1isize) as (i32)
    } else {
        0i32
    } as (u8);
}

#[no_mangle]
pub unsafe extern "C" fn ipprefix_cat(mut out: *mut StrAlloc, mut s: *mut u8) {
    let mut u: usize;
    let mut ch: u8;
    let mut j: u32;
    'loop1: loop {
        if *s as (i32) == b'.' as (i32) {
            s = s.offset(1isize);
        } else {
            j = scan_ulong(s as (*const u8), &mut u as (*mut usize));
            if j == 0 {
                break;
            }
            s = s.offset(j as (isize));
            ch = u as (u8);
            if !(StrAlloc::catb(out, &mut ch as (*mut u8) as (*const u8), 1u32) == 0) {
                continue;
            }
            nomem();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn txtparse(mut sa: *mut StrAlloc) {
    let mut ch: u8;
    let mut i: u32;
    let mut j: u32;
    j = 0u32;
    i = 0u32;
    'loop1: loop {
        if !(i < (*sa).len) {
            break;
        }
        ch = *(*sa).s.offset({
            let _old = i;
            i = i.wrapping_add(1u32);
            _old
        } as (isize));
        if ch as (i32) == b'\\' as (i32) {
            if i >= (*sa).len {
                break;
            }
            ch = *(*sa).s.offset({
                let _old = i;
                i = i.wrapping_add(1u32);
                _old
            } as (isize));
            if ch as (i32) >= b'0' as (i32) && (ch as (i32) <= b'7' as (i32)) {
                ch = (ch as (i32) - b'0' as (i32)) as (u8);
                if i < (*sa).len && (*(*sa).s.offset(i as (isize)) as (i32) >= b'0' as (i32)) &&
                    (*(*sa).s.offset(i as (isize)) as (i32) <= b'7' as (i32))
                {
                    ch = (ch as (i32) << 3i32) as (u8);
                    ch = (ch as (i32) +
                              (*(*sa).s.offset({
                            let _old = i;
                            i = i.wrapping_add(1u32);
                            _old
                        } as (isize)) as (i32) - b'0' as (i32))) as (u8);
                    if i < (*sa).len && (*(*sa).s.offset(i as (isize)) as (i32) >= b'0' as (i32)) &&
                        (*(*sa).s.offset(i as (isize)) as (i32) <= b'7' as (i32))
                    {
                        ch = (ch as (i32) << 3i32) as (u8);
                        ch = (ch as (i32) +
                                  (*(*sa).s.offset({
                                let _old = i;
                                i = i.wrapping_add(1u32);
                                _old
                            } as (isize)) as (i32) -
                                       b'0' as (i32))) as (u8);
                    }
                }
            }
        }
        *(*sa).s.offset({
            let _old = j;
            j = j.wrapping_add(1u32);
            _old
        } as (isize)) = ch;
    }
    (*sa).len = j;
}

#[no_mangle]
pub static mut defaultsoa: [u8; 20] = [0u8; 20];

#[derive(Copy)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: isize,
    pub tv_nsec: isize,
}

impl Clone for timespec {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct stat {
    pub st_dev: i32,
    pub st_mode: u16,
    pub st_nlink: u16,
    pub st_ino: usize,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: i32,
    pub st_atimespec: timespec,
    pub st_mtimespec: timespec,
    pub st_ctimespec: timespec,
    pub st_birthtimespec: timespec,
    pub st_size: isize,
    pub st_blocks: isize,
    pub st_blksize: i32,
    pub st_flags: u32,
    pub st_gen: u32,
    pub st_lspare: i32,
    pub st_qspare: [isize; 2],
}

impl Clone for stat {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn defaultsoa_init(mut fd: i32) {
    let mut st: stat;
    if fstat(fd, &mut st as (*mut stat)) == -1i32 {
        StrErr::die(
            111i32,
            (*b"tinydns-data: fatal: \0").as_ptr(),
            (*b"unable to stat data: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
        );
    }
    uint32::pack_big(defaultsoa.as_mut_ptr(), st.st_mtimespec.tv_sec as (u32));
    if byte::diff(
        defaultsoa.as_mut_ptr(),
        4u32,
        (*b"\0\0\0\0\0").as_ptr() as (*mut u8),
    ) == 0
    {
        defaultsoa[3usize] = 1u8;
    }
    byte::copy(
        defaultsoa.as_mut_ptr().offset(4isize),
        16u32,
        (*b"\0\0@\0\0\0\x08\0\0\x10\0\0\0\0\n\0\0").as_ptr() as (*mut u8),
    );
}

#[no_mangle]
pub static mut fdcdb: i32 = 0i32;

#[derive(Copy)]
#[repr(C)]
pub struct cdb_hp {
    pub h: u32,
    pub p: u32,
}

impl Clone for cdb_hp {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct cdb_hplist {
    pub hp: [cdb_hp; 1000],
    pub next: *mut cdb_hplist,
    pub num: i32,
}

impl Clone for cdb_hplist {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct cdb_make {
    pub bspace: [u8; 8192],
    pub final_: [u8; 2048],
    pub count: [u32; 256],
    pub start: [u32; 256],
    pub head: *mut cdb_hplist,
    pub split: *mut cdb_hp,
    pub hash: *mut cdb_hp,
    pub numentries: u32,
    pub b: Buffer,
    pub pos: u32,
    pub fd: i32,
}

impl Clone for cdb_make {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub static mut cdb: cdb_make = cdb_make {
    bspace: [0u8; 8192],
    final_: [0u8; 2048],
    count: [0u32; 256],
    start: [0u32; 256],
    head: 0 as (*mut cdb_hplist),
    split: 0 as (*mut cdb_hp),
    hash: 0 as (*mut cdb_hp),
    numentries: 0u32,
    b: Buffer {
        x: 0 as (*mut u8),
        p: 0u32,
        n: 0u32,
        fd: 0i32,
        op: None,
    },
    pos: 0u32,
    fd: 0i32,
};

static mut key: StrAlloc = StrAlloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

static mut result: StrAlloc = StrAlloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

#[no_mangle]
pub unsafe extern "C" fn rr_add(mut buf: *const u8, mut len: u32) {
    if StrAlloc::catb(&mut result as (*mut StrAlloc), buf, len) == 0 {
        nomem();
    }
}

#[no_mangle]
pub unsafe extern "C" fn rr_addname(mut d: *const u8) {
    rr_add(d, dns_domain_length(d));
}

#[no_mangle]
pub unsafe extern "C" fn rr_start(
    mut type_: *const u8,
    mut ttl: usize,
    mut ttd: *const u8,
    mut loc: *const u8,
) {
    let mut buf: [u8; 4];
    if StrAlloc::copyb(&mut result as (*mut StrAlloc), type_, 2u32) == 0 {
        nomem();
    }
    if byte::diff(loc as (*mut u8), 2u32, (*b"\0\0\0").as_ptr() as (*mut u8)) == 0 {
        rr_add((*b"=\0").as_ptr(), 1u32);
    } else {
        rr_add((*b">\0").as_ptr(), 1u32);
        rr_add(loc, 2u32);
    }
    uint32::pack_big(buf.as_mut_ptr(), ttl as (u32));
    rr_add(buf.as_mut_ptr() as (*const u8), 4u32);
    rr_add(ttd, 8u32);
}

#[no_mangle]
pub unsafe extern "C" fn rr_finish(mut owner: *const u8) {
    if byte::diff(
        owner as (*mut u8),
        2u32,
        (*b"\x01*\0").as_ptr() as (*mut u8),
    ) == 0
    {
        owner = owner.offset(2isize);
        let _rhs = 19i32;
        let _lhs = &mut *result.s.offset(2isize);
        *_lhs = (*_lhs as (i32) - _rhs) as (u8);
    }
    if StrAlloc::copyb(&mut key as (*mut StrAlloc), owner, dns_domain_length(owner)) == 0 {
        nomem();
    }
    case_lowerb(key.s, key.len);
    if cdb_make_add(
        &mut cdb as (*mut cdb_make),
        key.s as (*const u8),
        key.len,
        result.s as (*const u8),
        result.len,
    ) == -1i32
    {
        die_datatmp();
    }
}

#[no_mangle]
pub static mut b: Buffer = Buffer {
    x: 0 as (*mut u8),
    p: 0u32,
    n: 0u32,
    fd: 0i32,
    op: None,
};

#[no_mangle]
pub static mut bspace: [u8; 1024] = [0u8; 1024];

static mut line: StrAlloc = StrAlloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

#[no_mangle]
pub static mut match_: i32 = 1i32;

#[no_mangle]
pub static mut linenum: usize = 0usize;

static mut f: [StrAlloc; 15] = [StrAlloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
}; 15];

static mut d1: *mut u8 = 0 as (*mut u8);

static mut d2: *mut u8 = 0 as (*mut u8);

#[no_mangle]
pub static mut dptr: [u8; 31] = [0u8; 31];

#[no_mangle]
pub static mut strnum: [u8; 40] = [0u8; 40];

#[no_mangle]
pub unsafe extern "C" fn syntaxerror(mut why: *const u8) {
    strnum[fmt_ulong(strnum.as_mut_ptr(), linenum) as (usize)] = 0u8;
    StrErr::die(
        111i32,
        (*b"tinydns-data: fatal: \0").as_ptr(),
        (*b"unable to parse data line \0").as_ptr(),
        strnum.as_mut_ptr() as (*const u8),
        why,
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const StrErr),
    );
}

fn main() {
    let ret = unsafe { _c_main() };
    ::std::process::exit(ret);
}

#[no_mangle]
pub unsafe extern "C" fn _c_main() -> i32 {
    let mut fddata: i32;
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    let mut ch: u8;
    let mut ttl: usize;
    let mut ttd: [u8; 8];
    let mut loc: [u8; 2];
    let mut u: usize;
    let mut ip: [u8; 4];
    let mut type_: [u8; 2];
    let mut soa: [u8; 20];
    let mut buf: [u8; 4];
    umask(0o22u16);
    fddata = open_read((*b"data\0").as_ptr());
    if fddata == -1i32 {
        StrErr::die(
            111i32,
            (*b"tinydns-data: fatal: \0").as_ptr(),
            (*b"unable to open data: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
        );
    }
    defaultsoa_init(fddata);
    Buffer::init(
        &mut b as (*mut Buffer),
        buffer::unixread as buffer::Op,
        fddata,
        bspace.as_mut_ptr(),
        ::std::mem::size_of::<[u8; 1024]>() as (u32),
    );
    fdcdb = open_trunc((*b"data.tmp\0").as_ptr());
    if fdcdb == -1i32 {
        die_datatmp();
    }
    if cdb_make_start(&mut cdb as (*mut cdb_make), fdcdb) == -1i32 {
        die_datatmp();
    }
    'loop6: loop {
        if match_ == 0 {
            break;
        }
        linenum = linenum.wrapping_add(1usize);
        if getln(
            &mut b as (*mut Buffer),
            &mut line as (*mut StrAlloc),
            &mut match_ as (*mut i32),
            b'\n' as (i32),
        ) == -1i32
        {
            StrErr::die(
                111i32,
                (*b"tinydns-data: fatal: \0").as_ptr(),
                (*b"unable to read line: \0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
            );
        }
        'loop18: loop {
            if line.len == 0 {
                break;
            }
            ch = *line.s.offset(line.len.wrapping_sub(1u32) as (isize));
            if ch as (i32) != b' ' as (i32) && (ch as (i32) != b'\t' as (i32)) &&
                (ch as (i32) != b'\n' as (i32))
            {
                break;
            }
            line.len = line.len.wrapping_sub(1u32);
        }
        if line.len == 0 {
            continue;
        }
        if *line.s.offset(0isize) as (i32) == b'#' as (i32) {
            continue;
        }
        if *line.s.offset(0isize) as (i32) == b'-' as (i32) {
            continue;
        }
        j = 1i32;
        i = 0i32;
        'loop25: loop {
            if !(i < 15i32) {
                break;
            }
            if j as (u32) >= line.len {
                if StrAlloc::copys(&mut f[i as (usize)] as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
                    nomem();
                }
            } else {
                k = byte::chr(
                    line.s.offset(j as (isize)),
                    line.len.wrapping_sub(j as (u32)),
                    b':' as (i32),
                ) as (i32);
                if StrAlloc::copyb(
                    &mut f[i as (usize)] as (*mut StrAlloc),
                    line.s.offset(j as (isize)) as (*const u8),
                    k as (u32),
                ) == 0
                {
                    nomem();
                }
                j = j + (k + 1i32);
            }
            i = i + 1;
        }
        let switch1 = *line.s.offset(0isize);
        if switch1 as (i32) == b':' as (i32) {
            if dns_domain_fromdot(
                &mut d1 as (*mut *mut u8),
                f[0usize].s as (*const u8),
                f[0usize].len,
            ) == 0
            {
                nomem();
            }
            if StrAlloc::append(&mut f[3usize] as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if scan_ulong(f[3usize].s as (*const u8), &mut ttl as (*mut usize)) == 0 {
                ttl = 86400usize;
            }
            ttdparse(&mut f[4usize] as (*mut StrAlloc), ttd.as_mut_ptr());
            locparse(&mut f[5usize] as (*mut StrAlloc), loc.as_mut_ptr());
            if StrAlloc::append(&mut f[1usize] as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            scan_ulong(f[1usize].s as (*const u8), &mut u as (*mut usize));
            uint16::pack_big(type_.as_mut_ptr(), u as (u16));
            if byte::diff(
                type_.as_mut_ptr(),
                2u32,
                (*b"\0\xFC\0").as_ptr() as (*mut u8),
            ) == 0
            {
                syntaxerror((*b": type AXFR prohibited\0").as_ptr());
            }
            if byte::diff(type_.as_mut_ptr(), 2u32, (*b"\0\0\0").as_ptr() as (*mut u8)) == 0 {
                syntaxerror((*b": type 0 prohibited\0").as_ptr());
            }
            if byte::diff(
                type_.as_mut_ptr(),
                2u32,
                (*b"\0\x06\0").as_ptr() as (*mut u8),
            ) == 0
            {
                syntaxerror((*b": type SOA prohibited\0").as_ptr());
            }
            if byte::diff(
                type_.as_mut_ptr(),
                2u32,
                (*b"\0\x02\0").as_ptr() as (*mut u8),
            ) == 0
            {
                syntaxerror((*b": type NS prohibited\0").as_ptr());
            }
            if byte::diff(
                type_.as_mut_ptr(),
                2u32,
                (*b"\0\x05\0").as_ptr() as (*mut u8),
            ) == 0
            {
                syntaxerror((*b": type CNAME prohibited\0").as_ptr());
            }
            if byte::diff(
                type_.as_mut_ptr(),
                2u32,
                (*b"\0\x0C\0").as_ptr() as (*mut u8),
            ) == 0
            {
                syntaxerror((*b": type PTR prohibited\0").as_ptr());
            }
            if byte::diff(
                type_.as_mut_ptr(),
                2u32,
                (*b"\0\x0F\0").as_ptr() as (*mut u8),
            ) == 0
            {
                syntaxerror((*b": type MX prohibited\0").as_ptr());
            }
            txtparse(&mut f[2usize] as (*mut StrAlloc));
            rr_start(
                type_.as_mut_ptr() as (*const u8),
                ttl,
                ttd.as_mut_ptr() as (*const u8),
                loc.as_mut_ptr() as (*const u8),
            );
            rr_add(f[2usize].s as (*const u8), f[2usize].len);
            rr_finish(d1 as (*const u8));
        } else if switch1 as (i32) == b'\'' as (i32) {
            if dns_domain_fromdot(
                &mut d1 as (*mut *mut u8),
                f[0usize].s as (*const u8),
                f[0usize].len,
            ) == 0
            {
                nomem();
            }
            if StrAlloc::append(&mut f[2usize] as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if scan_ulong(f[2usize].s as (*const u8), &mut ttl as (*mut usize)) == 0 {
                ttl = 86400usize;
            }
            ttdparse(&mut f[3usize] as (*mut StrAlloc), ttd.as_mut_ptr());
            locparse(&mut f[4usize] as (*mut StrAlloc), loc.as_mut_ptr());
            rr_start(
                (*b"\0\x10\0").as_ptr(),
                ttl,
                ttd.as_mut_ptr() as (*const u8),
                loc.as_mut_ptr() as (*const u8),
            );
            txtparse(&mut f[1usize] as (*mut StrAlloc));
            i = 0i32;
            'loop143: loop {
                if !(i as (u32) < f[1usize].len) {
                    break;
                }
                k = f[1usize].len.wrapping_sub(i as (u32)) as (i32);
                if k > 127i32 {
                    k = 127i32;
                }
                ch = k as (u8);
                rr_add(&mut ch as (*mut u8) as (*const u8), 1u32);
                rr_add(f[1usize].s.offset(i as (isize)) as (*const u8), k as (u32));
                i = i + k;
            }
            rr_finish(d1 as (*const u8));
        } else if switch1 as (i32) == b'C' as (i32) || switch1 as (i32) == b'^' as (i32) {
            if dns_domain_fromdot(
                &mut d1 as (*mut *mut u8),
                f[0usize].s as (*const u8),
                f[0usize].len,
            ) == 0
            {
                nomem();
            }
            if dns_domain_fromdot(
                &mut d2 as (*mut *mut u8),
                f[1usize].s as (*const u8),
                f[1usize].len,
            ) == 0
            {
                nomem();
            }
            if StrAlloc::append(&mut f[2usize] as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if scan_ulong(f[2usize].s as (*const u8), &mut ttl as (*mut usize)) == 0 {
                ttl = 86400usize;
            }
            ttdparse(&mut f[3usize] as (*mut StrAlloc), ttd.as_mut_ptr());
            locparse(&mut f[4usize] as (*mut StrAlloc), loc.as_mut_ptr());
            if *line.s.offset(0isize) as (i32) == b'C' as (i32) {
                rr_start(
                    (*b"\0\x05\0").as_ptr(),
                    ttl,
                    ttd.as_mut_ptr() as (*const u8),
                    loc.as_mut_ptr() as (*const u8),
                );
            } else {
                rr_start(
                    (*b"\0\x0C\0").as_ptr(),
                    ttl,
                    ttd.as_mut_ptr() as (*const u8),
                    loc.as_mut_ptr() as (*const u8),
                );
            }
            rr_addname(d2 as (*const u8));
            rr_finish(d1 as (*const u8));
        } else if switch1 as (i32) == b'@' as (i32) {
            if dns_domain_fromdot(
                &mut d1 as (*mut *mut u8),
                f[0usize].s as (*const u8),
                f[0usize].len,
            ) == 0
            {
                nomem();
            }
            if StrAlloc::append(&mut f[4usize] as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if scan_ulong(f[4usize].s as (*const u8), &mut ttl as (*mut usize)) == 0 {
                ttl = 86400usize;
            }
            ttdparse(&mut f[5usize] as (*mut StrAlloc), ttd.as_mut_ptr());
            locparse(&mut f[6usize] as (*mut StrAlloc), loc.as_mut_ptr());
            if StrAlloc::append(&mut f[1usize] as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if byte::chr(f[2usize].s, f[2usize].len, b'.' as (i32)) >= f[2usize].len {
                if StrAlloc::cats(&mut f[2usize] as (*mut StrAlloc), (*b".mx.\0").as_ptr()) == 0 {
                    nomem();
                }
                if StrAlloc::catb(
                    &mut f[2usize] as (*mut StrAlloc),
                    f[0usize].s as (*const u8),
                    f[0usize].len,
                ) == 0
                {
                    nomem();
                }
            }
            if dns_domain_fromdot(
                &mut d2 as (*mut *mut u8),
                f[2usize].s as (*const u8),
                f[2usize].len,
            ) == 0
            {
                nomem();
            }
            if StrAlloc::append(&mut f[3usize] as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if scan_ulong(f[3usize].s as (*const u8), &mut u as (*mut usize)) == 0 {
                u = 0usize;
            }
            rr_start(
                (*b"\0\x0F\0").as_ptr(),
                ttl,
                ttd.as_mut_ptr() as (*const u8),
                loc.as_mut_ptr() as (*const u8),
            );
            uint16::pack_big(buf.as_mut_ptr(), u as (u16));
            rr_add(buf.as_mut_ptr() as (*const u8), 2u32);
            rr_addname(d2 as (*const u8));
            rr_finish(d1 as (*const u8));
            if ip4_scan(f[1usize].s as (*const u8), ip.as_mut_ptr()) == 0 {
                continue;
            }
            rr_start(
                (*b"\0\x01\0").as_ptr(),
                ttl,
                ttd.as_mut_ptr() as (*const u8),
                loc.as_mut_ptr() as (*const u8),
            );
            rr_add(ip.as_mut_ptr() as (*const u8), 4u32);
            rr_finish(d2 as (*const u8));
        } else if switch1 as (i32) == b'=' as (i32) || switch1 as (i32) == b'+' as (i32) {
            if dns_domain_fromdot(
                &mut d1 as (*mut *mut u8),
                f[0usize].s as (*const u8),
                f[0usize].len,
            ) == 0
            {
                nomem();
            }
            if StrAlloc::append(&mut f[2usize] as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if scan_ulong(f[2usize].s as (*const u8), &mut ttl as (*mut usize)) == 0 {
                ttl = 86400usize;
            }
            ttdparse(&mut f[3usize] as (*mut StrAlloc), ttd.as_mut_ptr());
            locparse(&mut f[4usize] as (*mut StrAlloc), loc.as_mut_ptr());
            if StrAlloc::append(&mut f[1usize] as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if ip4_scan(f[1usize].s as (*const u8), ip.as_mut_ptr()) == 0 {
                continue;
            }
            rr_start(
                (*b"\0\x01\0").as_ptr(),
                ttl,
                ttd.as_mut_ptr() as (*const u8),
                loc.as_mut_ptr() as (*const u8),
            );
            rr_add(ip.as_mut_ptr() as (*const u8), 4u32);
            rr_finish(d1 as (*const u8));
            if !(*line.s.offset(0isize) as (i32) == b'=' as (i32)) {
                continue;
            }
            dns_name4_domain(dptr.as_mut_ptr(), ip.as_mut_ptr() as (*const u8));
            rr_start(
                (*b"\0\x0C\0").as_ptr(),
                ttl,
                ttd.as_mut_ptr() as (*const u8),
                loc.as_mut_ptr() as (*const u8),
            );
            rr_addname(d1 as (*const u8));
            rr_finish(dptr.as_mut_ptr() as (*const u8));
        } else if switch1 as (i32) == b'&' as (i32) || switch1 as (i32) == b'.' as (i32) {
            if dns_domain_fromdot(
                &mut d1 as (*mut *mut u8),
                f[0usize].s as (*const u8),
                f[0usize].len,
            ) == 0
            {
                nomem();
            }
            if StrAlloc::append(&mut f[3usize] as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if scan_ulong(f[3usize].s as (*const u8), &mut ttl as (*mut usize)) == 0 {
                ttl = 259200usize;
            }
            ttdparse(&mut f[4usize] as (*mut StrAlloc), ttd.as_mut_ptr());
            locparse(&mut f[5usize] as (*mut StrAlloc), loc.as_mut_ptr());
            if StrAlloc::append(&mut f[1usize] as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if byte::chr(f[2usize].s, f[2usize].len, b'.' as (i32)) >= f[2usize].len {
                if StrAlloc::cats(&mut f[2usize] as (*mut StrAlloc), (*b".ns.\0").as_ptr()) == 0 {
                    nomem();
                }
                if StrAlloc::catb(
                    &mut f[2usize] as (*mut StrAlloc),
                    f[0usize].s as (*const u8),
                    f[0usize].len,
                ) == 0
                {
                    nomem();
                }
            }
            if dns_domain_fromdot(
                &mut d2 as (*mut *mut u8),
                f[2usize].s as (*const u8),
                f[2usize].len,
            ) == 0
            {
                nomem();
            }
            if *line.s.offset(0isize) as (i32) == b'.' as (i32) {
                rr_start(
                    (*b"\0\x06\0").as_ptr(),
                    if ttl != 0 { 2560i32 } else { 0i32 } as (usize),
                    ttd.as_mut_ptr() as (*const u8),
                    loc.as_mut_ptr() as (*const u8),
                );
                rr_addname(d2 as (*const u8));
                rr_add((*b"\nhostmaster\0").as_ptr(), 11u32);
                rr_addname(d1 as (*const u8));
                rr_add(defaultsoa.as_mut_ptr() as (*const u8), 20u32);
                rr_finish(d1 as (*const u8));
            }
            rr_start(
                (*b"\0\x02\0").as_ptr(),
                ttl,
                ttd.as_mut_ptr() as (*const u8),
                loc.as_mut_ptr() as (*const u8),
            );
            rr_addname(d2 as (*const u8));
            rr_finish(d1 as (*const u8));
            if ip4_scan(f[1usize].s as (*const u8), ip.as_mut_ptr()) == 0 {
                continue;
            }
            rr_start(
                (*b"\0\x01\0").as_ptr(),
                ttl,
                ttd.as_mut_ptr() as (*const u8),
                loc.as_mut_ptr() as (*const u8),
            );
            rr_add(ip.as_mut_ptr() as (*const u8), 4u32);
            rr_finish(d2 as (*const u8));
        } else if switch1 as (i32) == b'Z' as (i32) {
            if dns_domain_fromdot(
                &mut d1 as (*mut *mut u8),
                f[0usize].s as (*const u8),
                f[0usize].len,
            ) == 0
            {
                nomem();
            }
            if StrAlloc::append(&mut f[3usize] as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if scan_ulong(f[3usize].s as (*const u8), &mut u as (*mut usize)) == 0 {
                uint32::unpack_big(
                    defaultsoa.as_mut_ptr() as (*const u8),
                    &mut u as (*mut usize) as (*mut u32),
                );
            }
            uint32::pack_big(soa.as_mut_ptr(), u as (u32));
            if StrAlloc::append(&mut f[4usize] as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if scan_ulong(f[4usize].s as (*const u8), &mut u as (*mut usize)) == 0 {
                uint32::unpack_big(
                    defaultsoa.as_mut_ptr().offset(4isize) as (*const u8),
                    &mut u as (*mut usize) as (*mut u32),
                );
            }
            uint32::pack_big(soa.as_mut_ptr().offset(4isize), u as (u32));
            if StrAlloc::append(&mut f[5usize] as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if scan_ulong(f[5usize].s as (*const u8), &mut u as (*mut usize)) == 0 {
                uint32::unpack_big(
                    defaultsoa.as_mut_ptr().offset(8isize) as (*const u8),
                    &mut u as (*mut usize) as (*mut u32),
                );
            }
            uint32::pack_big(soa.as_mut_ptr().offset(8isize), u as (u32));
            if StrAlloc::append(&mut f[6usize] as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if scan_ulong(f[6usize].s as (*const u8), &mut u as (*mut usize)) == 0 {
                uint32::unpack_big(
                    defaultsoa.as_mut_ptr().offset(12isize) as (*const u8),
                    &mut u as (*mut usize) as (*mut u32),
                );
            }
            uint32::pack_big(soa.as_mut_ptr().offset(12isize), u as (u32));
            if StrAlloc::append(&mut f[7usize] as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if scan_ulong(f[7usize].s as (*const u8), &mut u as (*mut usize)) == 0 {
                uint32::unpack_big(
                    defaultsoa.as_mut_ptr().offset(16isize) as (*const u8),
                    &mut u as (*mut usize) as (*mut u32),
                );
            }
            uint32::pack_big(soa.as_mut_ptr().offset(16isize), u as (u32));
            if StrAlloc::append(&mut f[8usize] as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if scan_ulong(f[8usize].s as (*const u8), &mut ttl as (*mut usize)) == 0 {
                ttl = 2560usize;
            }
            ttdparse(&mut f[9usize] as (*mut StrAlloc), ttd.as_mut_ptr());
            locparse(&mut f[10usize] as (*mut StrAlloc), loc.as_mut_ptr());
            rr_start(
                (*b"\0\x06\0").as_ptr(),
                ttl,
                ttd.as_mut_ptr() as (*const u8),
                loc.as_mut_ptr() as (*const u8),
            );
            if dns_domain_fromdot(
                &mut d2 as (*mut *mut u8),
                f[1usize].s as (*const u8),
                f[1usize].len,
            ) == 0
            {
                nomem();
            }
            rr_addname(d2 as (*const u8));
            if dns_domain_fromdot(
                &mut d2 as (*mut *mut u8),
                f[2usize].s as (*const u8),
                f[2usize].len,
            ) == 0
            {
                nomem();
            }
            rr_addname(d2 as (*const u8));
            rr_add(soa.as_mut_ptr() as (*const u8), 20u32);
            rr_finish(d1 as (*const u8));
        } else if switch1 as (i32) == b'%' as (i32) {
            locparse(&mut f[0usize] as (*mut StrAlloc), loc.as_mut_ptr());
            if StrAlloc::copyb(&mut key as (*mut StrAlloc), (*b"\0%\0").as_ptr(), 2u32) == 0 {
                nomem();
            }
            if StrAlloc::append(&mut f[1usize] as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            ipprefix_cat(&mut key as (*mut StrAlloc), f[1usize].s);
            if !(cdb_make_add(
                &mut cdb as (*mut cdb_make),
                key.s as (*const u8),
                key.len,
                loc.as_mut_ptr() as (*const u8),
                2u32,
            ) == -1i32)
            {
                continue;
            }
            die_datatmp();
        } else {
            syntaxerror((*b": unrecognized leading character\0").as_ptr());
        }
    }
    if cdb_make_finish(&mut cdb as (*mut cdb_make)) == -1i32 {
        die_datatmp();
    }
    if fsync(fdcdb) == -1i32 {
        die_datatmp();
    }
    if close(fdcdb) == -1i32 {
        die_datatmp();
    }
    if rename((*b"data.tmp\0").as_ptr(), (*b"data.cdb\0").as_ptr()) == -1i32 {
        StrErr::die(
            111i32,
            (*b"tinydns-data: fatal: \0").as_ptr(),
            (*b"unable to move data.tmp to data.cdb: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
        );
    }
    libc::_exit(0i32);
}
