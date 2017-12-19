use alloc;
use byte;
use libc;

extern "C" {
    fn __swbuf(arg1: i32, arg2: *mut __sFILE) -> i32;
    fn buffer_init(
        arg1: *mut buffer,
        arg2: unsafe extern "C" fn() -> i32,
        arg3: i32,
        arg4: *mut u8,
        arg5: u32,
    );
    fn buffer_unixread(arg1: i32, arg2: *mut u8, arg3: u32) -> i32;
    fn case_diffb(arg1: *const u8, arg2: u32, arg3: *const u8) -> i32;
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
    fn fmt_ulong(arg1: *mut u8, arg2: usize) -> u32;
    fn fsync(arg1: i32) -> i32;
    fn getln(arg1: *mut buffer, arg2: *mut stralloc, arg3: *mut i32, arg4: i32) -> i32;
    fn ip4_scan(arg1: *const u8, arg2: *mut u8) -> u32;
    fn open_read(arg1: *const u8) -> i32;
    fn open_trunc(arg1: *const u8) -> i32;
    fn rename(__old: *const u8, __new: *const u8) -> i32;
    fn scan_ulong(arg1: *const u8, arg2: *mut usize) -> u32;
    fn stralloc_append(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn stralloc_catb(arg1: *mut stralloc, arg2: *const u8, arg3: u32) -> i32;
    fn stralloc_copyb(arg1: *mut stralloc, arg2: *const u8, arg3: u32) -> i32;
    fn stralloc_copys(arg1: *mut stralloc, arg2: *const u8) -> i32;
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
        (*b"pickdns-data: fatal: \0").as_ptr(),
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
pub unsafe extern "C" fn ipprefix_cat(mut out: *mut stralloc, mut s: *mut u8) {
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
            if !(stralloc_catb(out, &mut ch as (*mut u8) as (*const u8), 1u32) == 0) {
                continue;
            }
            nomem();
        }
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct address {
    pub name: *mut u8,
    pub namelen: u32,
    pub ip: [u8; 4],
    pub location: [u8; 2],
}

impl Clone for address {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn address_diff(mut p: *mut address, mut q: *mut address) -> i32 {
    let mut r: i32;
    r = byte::diff((*p).location.as_mut_ptr(), 2u32, (*q).location.as_mut_ptr());
    if r < 0i32 {
        -1i32
    } else if r > 0i32 {
        1i32
    } else if (*p).namelen < (*q).namelen {
        -1i32
    } else if (*p).namelen > (*q).namelen {
        1i32
    } else {
        case_diffb(
            (*p).name as (*const u8),
            (*p).namelen,
            (*q).name as (*const u8),
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn address_sort(mut z: *mut address, mut n: u32) {
    let mut i: u32;
    let mut j: u32;
    let mut p: u32;
    let mut q: u32;
    let mut t: address;
    i = {
        j = n;
        j
    };
    z = z.offset(-1isize);
    'loop1: loop {
        if !(j > 1u32) {
            break;
        }
        if i > 1u32 {
            i = i.wrapping_sub(1u32);
            t = *z.offset(i as (isize));
        } else {
            t = *z.offset(j as (isize));
            *z.offset(j as (isize)) = *z.offset(i as (isize));
            j = j.wrapping_sub(1u32);
        }
        q = i;
        'loop7: loop {
            if !({
                     p = q.wrapping_mul(2u32);
                     p
                 } < j)
            {
                break;
            }
            if address_diff(
                &mut *z.offset(p.wrapping_add(1u32) as (isize)) as (*mut address),
                &mut *z.offset(p as (isize)) as (*mut address),
            ) >= 0i32
            {
                p = p.wrapping_add(1u32);
            }
            *z.offset(q as (isize)) = *z.offset(p as (isize));
            q = p;
        }
        if p == j {
            *z.offset(q as (isize)) = *z.offset(p as (isize));
            q = p;
        }
        'loop10: loop {
            if !(q > i &&
                     (address_diff(
                    &mut t as (*mut address),
                    &mut *z.offset({
                        p = q.wrapping_div(2u32);
                        p
                    } as (isize)) as (*mut address),
                ) > 0i32))
            {
                break;
            }
            *z.offset(q as (isize)) = *z.offset(p as (isize));
            q = p;
        }
        *z.offset(q as (isize)) = t;
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct address_alloc {
    pub s: *mut address,
    pub len: u32,
    pub a: u32,
}

impl Clone for address_alloc {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn address_alloc_readyplus(mut x: *mut address_alloc, mut n: u32) -> i32 {
    let mut i: u32;
    if !(*x).s.is_null() {
        i = (*x).a;
        n = n.wrapping_add((*x).len);
        (if n > i {
             (*x).a = 30u32.wrapping_add(n).wrapping_add(n >> 3i32);
             (if alloc::alloc_re(
                &mut (*x).s as (*mut *mut address) as (*mut *mut u8),
                (i as (usize)).wrapping_mul(::std::mem::size_of::<address>()) as (u32),
                ((*x).a as (usize)).wrapping_mul(::std::mem::size_of::<address>()) as (u32),
            ) != 0
            {
                  1i32
              } else {
                  (*x).a = i;
                  0i32
              })
         } else {
             1i32
         })
    } else {
        (*x).len = 0u32;
        !{
            (*x).s = alloc::alloc(({
                 (*x).a = n;
                 (*x).a
             } as (usize))
                .wrapping_mul(::std::mem::size_of::<address>()) as
                (u32)) as (*mut address);
            (*x).s
        }.is_null() as (i32)
    }
}

#[no_mangle]
pub unsafe extern "C" fn address_alloc_append(
    mut x: *mut address_alloc,
    mut i: *const address,
) -> i32 {
    if address_alloc_readyplus(x, 1u32) == 0 {
        0i32
    } else {
        *(*x).s.offset({
            let _old = (*x).len;
            (*x).len = (*x).len.wrapping_add(1u32);
            _old
        } as (isize)) = *i;
        1i32
    }
}

static mut x: address_alloc = address_alloc {
    s: 0 as (*mut address),
    len: 0u32,
    a: 0u32,
};

#[no_mangle]
pub static mut fd: i32 = 0i32;

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
    pub b: buffer,
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
    b: buffer {
        x: 0 as (*mut u8),
        p: 0u32,
        n: 0u32,
        fd: 0i32,
        op: 0 as (unsafe extern "C" fn() -> i32),
    },
    pos: 0u32,
    fd: 0i32,
};

static mut key: stralloc = stralloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

static mut result: stralloc = stralloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

static mut line: stralloc = stralloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

#[no_mangle]
pub static mut match_: i32 = 1i32;

#[no_mangle]
pub static mut linenum: usize = 0usize;

static mut f: [stralloc; 3] = [stralloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
}; 3];

#[no_mangle]
pub static mut strnum: [u8; 40] = [0u8; 40];

#[no_mangle]
pub unsafe extern "C" fn syntaxerror(mut why: *const u8) {
    strnum[fmt_ulong(strnum.as_mut_ptr(), linenum) as (usize)] = 0u8;
    strerr_die(
        111i32,
        (*b"pickdns-data: fatal: \0").as_ptr(),
        (*b"unable to parse data line \0").as_ptr(),
        strnum.as_mut_ptr() as (*const u8),
        why,
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const strerr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn die_datatmp() {
    strerr_die(
        111i32,
        (*b"pickdns-data: fatal: \0").as_ptr(),
        (*b"unable to create data.tmp: \0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        &mut strerr_sys as (*mut strerr) as (*const strerr),
    );
}

fn main() {
    let ret = unsafe { _c_main() };
    ::std::process::exit(ret);
}

#[no_mangle]
pub unsafe extern "C" fn _c_main() -> i32 {
    let mut t: address;
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    let mut ch: u8;
    umask(0o22u16);
    if address_alloc_readyplus(&mut x as (*mut address_alloc), 0u32) == 0 {
        nomem();
    }
    fd = open_read((*b"data\0").as_ptr());
    if fd == -1i32 {
        strerr_die(
            111i32,
            (*b"pickdns-data: fatal: \0").as_ptr(),
            (*b"unable to open data: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr),
        );
    }
    buffer_init(
        &mut b as (*mut buffer),
        buffer_unixread as (unsafe extern "C" fn() -> i32),
        fd,
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
    'loop8: loop {
        if match_ == 0 {
            break;
        }
        linenum = linenum.wrapping_add(1usize);
        if getln(
            &mut b as (*mut buffer),
            &mut line as (*mut stralloc),
            &mut match_ as (*mut i32),
            b'\n' as (i32),
        ) == -1i32
        {
            strerr_die(
                111i32,
                (*b"pickdns-data: fatal: \0").as_ptr(),
                (*b"unable to read line: \0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                &mut strerr_sys as (*mut strerr) as (*const strerr),
            );
        }
        'loop39: loop {
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
        j = 1i32;
        i = 0i32;
        'loop44: loop {
            if !(i < 3i32) {
                break;
            }
            if j as (u32) >= line.len {
                if stralloc_copys(&mut f[i as (usize)] as (*mut stralloc), (*b"\0").as_ptr()) == 0 {
                    nomem();
                }
            } else {
                k = byte::chr(
                    line.s.offset(j as (isize)),
                    line.len.wrapping_sub(j as (u32)),
                    b':' as (i32),
                ) as (i32);
                if stralloc_copyb(
                    &mut f[i as (usize)] as (*mut stralloc),
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
        if switch1 as (i32) == b'-' as (i32) || switch1 as (i32) == b'#' as (i32) {
            continue;
        }
        if switch1 as (i32) == b'%' as (i32) {
            if stralloc_append(&mut f[0usize] as (*mut stralloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if stralloc_append(&mut f[0usize] as (*mut stralloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if stralloc_copyb(
                &mut result as (*mut stralloc),
                f[0usize].s as (*const u8),
                2u32,
            ) == 0
            {
                nomem();
            }
            if stralloc_append(&mut f[1usize] as (*mut stralloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if stralloc_copys(&mut key as (*mut stralloc), (*b"%\0").as_ptr()) == 0 {
                nomem();
            }
            ipprefix_cat(&mut key as (*mut stralloc), f[1usize].s);
            if !(cdb_make_add(
                &mut cdb as (*mut cdb_make),
                key.s as (*const u8),
                key.len,
                result.s as (*const u8),
                result.len,
            ) == -1i32)
            {
                continue;
            }
            die_datatmp();
        } else if switch1 as (i32) == b'+' as (i32) {
            byte::zero(
                &mut t as (*mut address) as (*mut u8),
                ::std::mem::size_of::<address>() as (u32),
            );
            if dns_domain_fromdot(
                &mut t.name as (*mut *mut u8),
                f[0usize].s as (*const u8),
                f[0usize].len,
            ) == 0
            {
                nomem();
            }
            t.namelen = dns_domain_length(t.name as (*const u8));
            case_lowerb(t.name, t.namelen);
            if stralloc_append(&mut f[1usize] as (*mut stralloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if ip4_scan(f[1usize].s as (*const u8), t.ip.as_mut_ptr()) == 0 {
                syntaxerror((*b": malformed IP address\0").as_ptr());
            }
            if stralloc_append(&mut f[2usize] as (*mut stralloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if stralloc_append(&mut f[2usize] as (*mut stralloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            byte::copy(t.location.as_mut_ptr(), 2u32, f[2usize].s);
            if !(address_alloc_append(
                &mut x as (*mut address_alloc),
                &mut t as (*mut address) as (*const address),
            ) == 0)
            {
                continue;
            }
            nomem();
        } else {
            syntaxerror((*b": unrecognized leading character\0").as_ptr());
        }
    }
    close(fd);
    address_sort(x.s, x.len);
    i = 0i32;
    'loop10: loop {
        if !(i as (u32) < x.len) {
            break;
        }
        j = i + 1i32;
        'loop21: loop {
            if !(j as (u32) < x.len) {
                break;
            }
            if address_diff(x.s.offset(i as (isize)), x.s.offset(j as (isize))) != 0 {
                break;
            }
            j = j + 1;
        }
        if stralloc_copys(&mut key as (*mut stralloc), (*b"+\0").as_ptr()) == 0 {
            nomem();
        }
        if stralloc_catb(
            &mut key as (*mut stralloc),
            (*x.s.offset(i as (isize))).location.as_mut_ptr() as (*const u8),
            2u32,
        ) == 0
        {
            nomem();
        }
        if stralloc_catb(
            &mut key as (*mut stralloc),
            (*x.s.offset(i as (isize))).name as (*const u8),
            (*x.s.offset(i as (isize))).namelen,
        ) == 0
        {
            nomem();
        }
        if stralloc_copys(&mut result as (*mut stralloc), (*b"\0").as_ptr()) == 0 {
            nomem();
        }
        'loop32: loop {
            if !(i < j) {
                break;
            }
            if !(stralloc_catb(
                &mut result as (*mut stralloc),
                (*x.s.offset({
                    let _old = i;
                    i = i + 1;
                    _old
                } as (isize)))
                    .ip
                    .as_mut_ptr() as (*const u8),
                4u32,
            ) == 0)
            {
                continue;
            }
            nomem();
        }
        if !(cdb_make_add(
            &mut cdb as (*mut cdb_make),
            key.s as (*const u8),
            key.len,
            result.s as (*const u8),
            result.len,
        ) == -1i32)
        {
            continue;
        }
        die_datatmp();
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
        strerr_die(
            111i32,
            (*b"pickdns-data: fatal: \0").as_ptr(),
            (*b"unable to move data.tmp to data.cdb: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr),
        );
    }
    libc::_exit(0i32);
}
