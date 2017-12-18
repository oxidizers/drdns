extern "C" {
    fn __swbuf(arg1: i32, arg2: *mut __sFILE) -> i32;
    fn _exit(arg1: i32);
    fn buffer_flush(arg1: *mut buffer) -> i32;
    fn buffer_init(
        arg1: *mut buffer,
        arg2: unsafe extern "C" fn() -> i32,
        arg3: i32,
        arg4: *mut u8,
        arg5: u32,
    );
    fn buffer_putalign(arg1: *mut buffer, arg2: *const u8, arg3: u32) -> i32;
    fn buffer_unixread(arg1: i32, arg2: *mut u8, arg3: u32) -> i32;
    fn buffer_unixwrite(arg1: i32, arg2: *const u8, arg3: u32) -> i32;
    fn byte_chr(s: *mut u8, n: u32, c: i32) -> u32;
    fn byte_diff(s: *mut u8, n: u32, t: *mut u8) -> i32;
    fn close(arg1: i32) -> i32;
    fn dns_domain_equal(arg1: *const u8, arg2: *const u8) -> i32;
    fn dns_domain_fromdot(arg1: *mut *mut u8, arg2: *const u8, arg3: u32) -> i32;
    fn dns_domain_todot_cat(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn fchmod(arg1: i32, arg2: u16) -> i32;
    fn fmt_ulong(arg1: *mut u8, arg2: usize) -> u32;
    fn fstat(arg1: i32, arg2: *mut stat) -> i32;
    fn fsync(arg1: i32) -> i32;
    fn getln(arg1: *mut buffer, arg2: *mut stralloc, arg3: *mut i32, arg4: i32) -> i32;
    fn ip4_fmt(arg1: *mut u8, arg2: *const u8) -> u32;
    fn ip4_scan(arg1: *const u8, arg2: *mut u8) -> u32;
    fn open_read(arg1: *const u8) -> i32;
    fn open_trunc(arg1: *const u8) -> i32;
    fn rename(__old: *const u8, __new: *const u8) -> i32;
    fn scan_ulong(arg1: *const u8, arg2: *mut usize) -> u32;
    fn str_diff(arg1: *const u8, arg2: *const u8) -> i32;
    fn str_len(arg1: *const u8) -> u32;
    fn stralloc_append(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn stralloc_catb(arg1: *mut stralloc, arg2: *const u8, arg3: u32) -> i32;
    fn stralloc_cats(arg1: *mut stralloc, arg2: *const u8) -> i32;
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

#[no_mangle]
pub static mut fn_: *mut u8 = 0 as (*mut u8);

#[no_mangle]
pub static mut fnnew: *mut u8 = 0 as (*mut u8);

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
        (*b"tinydns-edit: usage: tinydns-edit data data.new add [ns|childns|host|alias|mx] domain a.b.c.d\0").as_ptr(
        ),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const strerr)
    );
}

#[no_mangle]
pub unsafe extern "C" fn nomem() {
    strerr_die(
        111i32,
        (*b"tinydns-edit: fatal: \0").as_ptr(),
        (*b"out of memory\0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const strerr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn die_read() {
    strerr_die(
        100i32,
        (*b"tinydns-edit: fatal: \0").as_ptr(),
        (*b"tinydns-edit: fatal: unable to read \0").as_ptr(),
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
        100i32,
        (*b"tinydns-edit: fatal: \0").as_ptr(),
        (*b"tinydns-edit: fatal: unable to write \0").as_ptr(),
        fnnew as (*const u8),
        (*b": \0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        &mut strerr_sys as (*mut strerr) as (*const strerr),
    );
}

#[no_mangle]
pub static mut mode: u8 = 0u8;

static mut target: *mut u8 = 0 as (*mut u8);

#[no_mangle]
pub static mut targetip: [u8; 4] = [0u8; 4];

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
pub static mut fdnew: i32 = 0i32;

#[no_mangle]
pub static mut bnew: buffer = buffer {
    x: 0 as (*mut u8),
    p: 0u32,
    n: 0u32,
    fd: 0i32,
    op: 0 as (unsafe extern "C" fn() -> i32),
};

#[no_mangle]
pub static mut bnewspace: [u8; 1024] = [0u8; 1024];

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

static mut line: stralloc = stralloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

#[no_mangle]
pub static mut match_: i32 = 1i32;

static mut f: [stralloc; 10] = [stralloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
}; 10];

static mut d1: *mut u8 = 0 as (*mut u8);

static mut d2: *mut u8 = 0 as (*mut u8);

#[no_mangle]
pub static mut ip: [u8; 4] = [0u8; 4];

#[no_mangle]
pub static mut ipstr: [u8; 20] = [0u8; 20];

#[no_mangle]
pub static mut strnum: [u8; 40] = [0u8; 40];

static mut names: [*mut u8; 26] = [0 as (*mut u8); 26];

static mut used: [i32; 26] = [0i32; 26];

#[no_mangle]
pub unsafe extern "C" fn put(mut buf: *const u8, mut len: u32) {
    if buffer_putalign(&mut bnew as (*mut buffer), buf, len) == -1i32 {
        die_write();
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
pub unsafe extern "C" fn _c_main(mut argc: i32, mut argv: *mut *mut u8) -> i32 {
    let mut ttl: usize;
    let mut st: stat;
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    let mut ch: u8;
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
    fn_ = *argv;
    if (*{
            argv = argv.offset(1isize);
            argv
        }).is_null()
    {
        die_usage();
    }
    fnnew = *argv;
    if (*{
            argv = argv.offset(1isize);
            argv
        }).is_null()
    {
        die_usage();
    }
    if str_diff(*argv as (*const u8), (*b"add\0").as_ptr()) != 0 {
        die_usage();
    }
    if (*{
            argv = argv.offset(1isize);
            argv
        }).is_null()
    {
        die_usage();
    }
    if str_diff(*argv as (*const u8), (*b"ns\0").as_ptr()) == 0 {
        mode = b'.';
    } else if str_diff(*argv as (*const u8), (*b"childns\0").as_ptr()) == 0 {
        mode = b'&';
    } else if str_diff(*argv as (*const u8), (*b"host\0").as_ptr()) == 0 {
        mode = b'=';
    } else if str_diff(*argv as (*const u8), (*b"alias\0").as_ptr()) == 0 {
        mode = b'+';
    } else if str_diff(*argv as (*const u8), (*b"mx\0").as_ptr()) == 0 {
        mode = b'@';
    } else {
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
        &mut target as (*mut *mut u8),
        *argv as (*const u8),
        str_len(*argv as (*const u8)),
    ) == 0
    {
        nomem();
    }
    if (*{
            argv = argv.offset(1isize);
            argv
        }).is_null()
    {
        die_usage();
    }
    if ip4_scan(*argv as (*const u8), targetip.as_mut_ptr()) == 0 {
        die_usage();
    }
    umask(0o77u16);
    fd = open_read(fn_ as (*const u8));
    if fd == -1i32 {
        die_read();
    }
    if fstat(fd, &mut st as (*mut stat)) == -1i32 {
        die_read();
    }
    buffer_init(
        &mut b as (*mut buffer),
        buffer_unixread as (unsafe extern "C" fn() -> i32),
        fd,
        bspace.as_mut_ptr(),
        ::std::mem::size_of::<[u8; 1024]>() as (u32),
    );
    fdnew = open_trunc(fnnew as (*const u8));
    if fdnew == -1i32 {
        die_write();
    }
    if fchmod(fdnew, (st.st_mode as (i32) & 0o644i32) as (u16)) == -1i32 {
        die_write();
    }
    buffer_init(
        &mut bnew as (*mut buffer),
        buffer_unixwrite as (unsafe extern "C" fn() -> i32),
        fdnew,
        bnewspace.as_mut_ptr(),
        ::std::mem::size_of::<[u8; 1024]>() as (u32),
    );
    if mode as (i32) == b'@' as (i32) {
        ttl = 86400usize;
        i = 0i32;
        'loop56: loop {
            if !(i < 26i32) {
                break;
            }
            ch = (b'a' as (i32) + i) as (u8);
            if stralloc_copyb(
                &mut f[0usize] as (*mut stralloc),
                &mut ch as (*mut u8) as (*const u8),
                1u32,
            ) == 0
            {
                nomem();
            }
            if stralloc_cats(&mut f[0usize] as (*mut stralloc), (*b".mx.\0").as_ptr()) == 0 {
                nomem();
            }
            if dns_domain_todot_cat(&mut f[0usize] as (*mut stralloc), target as (*const u8)) == 0 {
                nomem();
            }
            if dns_domain_fromdot(
                &mut names[i as (usize)] as (*mut *mut u8),
                f[0usize].s as (*const u8),
                f[0usize].len,
            ) == 0
            {
                nomem();
            }
            i = i + 1;
        }
    } else if mode as (i32) == b'=' as (i32) || mode as (i32) == b'+' as (i32) {
        ttl = 86400usize;
    } else if mode as (i32) == b'&' as (i32) || mode as (i32) == b'.' as (i32) {
        ttl = 259200usize;
        i = 0i32;
        'loop44: loop {
            if !(i < 26i32) {
                break;
            }
            ch = (b'a' as (i32) + i) as (u8);
            if stralloc_copyb(
                &mut f[0usize] as (*mut stralloc),
                &mut ch as (*mut u8) as (*const u8),
                1u32,
            ) == 0
            {
                nomem();
            }
            if stralloc_cats(&mut f[0usize] as (*mut stralloc), (*b".ns.\0").as_ptr()) == 0 {
                nomem();
            }
            if dns_domain_todot_cat(&mut f[0usize] as (*mut stralloc), target as (*const u8)) == 0 {
                nomem();
            }
            if dns_domain_fromdot(
                &mut names[i as (usize)] as (*mut *mut u8),
                f[0usize].s as (*const u8),
                f[0usize].len,
            ) == 0
            {
                nomem();
            }
            i = i + 1;
        }
    }
    'loop57: loop {
        if match_ == 0 {
            break;
        }
        if getln(
            &mut b as (*mut buffer),
            &mut line as (*mut stralloc),
            &mut match_ as (*mut i32),
            b'\n' as (i32),
        ) == -1i32
        {
            die_read();
        }
        put(line.s as (*const u8), line.len);
        if line.len != 0 && (match_ == 0) {
            put((*b"\n\0").as_ptr(), 1u32);
        }
        'loop99: loop {
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
        j = 1i32;
        i = 0i32;
        'loop105: loop {
            if !(i < 10i32) {
                break;
            }
            if j as (u32) >= line.len {
                if stralloc_copys(&mut f[i as (usize)] as (*mut stralloc), (*b"\0").as_ptr()) == 0 {
                    nomem();
                }
            } else {
                k = byte_chr(
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
        if mode as (i32) == b'@' as (i32) {
            if !(*line.s.offset(0isize) as (i32) == b'@' as (i32)) {
                continue;
            }
            if dns_domain_fromdot(
                &mut d1 as (*mut *mut u8),
                f[0usize].s as (*const u8),
                f[0usize].len,
            ) == 0
            {
                nomem();
            }
            if dns_domain_equal(d1 as (*const u8), target as (*const u8)) == 0 {
                continue;
            }
            if byte_chr(f[2usize].s, f[2usize].len, b'.' as (i32)) >= f[2usize].len {
                if stralloc_cats(&mut f[2usize] as (*mut stralloc), (*b".mx.\0").as_ptr()) == 0 {
                    nomem();
                }
                if stralloc_catb(
                    &mut f[2usize] as (*mut stralloc),
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
            if stralloc_append(&mut f[4usize] as (*mut stralloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if scan_ulong(f[4usize].s as (*const u8), &mut ttl as (*mut usize)) == 0 {
                ttl = 86400usize;
            }
            i = 0i32;
            'loop155: loop {
                if !(i < 26i32) {
                    continue 'loop57;
                }
                if dns_domain_equal(d2 as (*const u8), names[i as (usize)] as (*const u8)) != 0 {
                    break;
                }
                i = i + 1;
            }
            used[i as (usize)] = 1i32;
        } else if mode as (i32) == b'=' as (i32) {
            if !(*line.s.offset(0isize) as (i32) == b'=' as (i32)) {
                continue;
            }
            if dns_domain_fromdot(
                &mut d1 as (*mut *mut u8),
                f[0usize].s as (*const u8),
                f[0usize].len,
            ) == 0
            {
                nomem();
            }
            if dns_domain_equal(d1 as (*const u8), target as (*const u8)) != 0 {
                strerr_die(
                    100i32,
                    (*b"tinydns-edit: fatal: \0").as_ptr(),
                    (*b"host name already used\0").as_ptr(),
                    0i32 as (*const u8),
                    0i32 as (*const u8),
                    0i32 as (*const u8),
                    0i32 as (*const u8),
                    0i32 as (*const strerr),
                );
            }
            if stralloc_append(&mut f[1usize] as (*mut stralloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if ip4_scan(f[1usize].s as (*const u8), ip.as_mut_ptr()) == 0 {
                continue;
            }
            if !(byte_diff(ip.as_mut_ptr(), 4u32, targetip.as_mut_ptr()) == 0) {
                continue;
            }
            strerr_die(
                100i32,
                (*b"tinydns-edit: fatal: \0").as_ptr(),
                (*b"IP address already used\0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const strerr),
            );
        } else {
            if !(mode as (i32) == b'&' as (i32) || mode as (i32) == b'.' as (i32)) {
                continue;
            }
            if !(*line.s.offset(0isize) as (i32) == mode as (i32)) {
                continue;
            }
            if dns_domain_fromdot(
                &mut d1 as (*mut *mut u8),
                f[0usize].s as (*const u8),
                f[0usize].len,
            ) == 0
            {
                nomem();
            }
            if dns_domain_equal(d1 as (*const u8), target as (*const u8)) == 0 {
                continue;
            }
            if byte_chr(f[2usize].s, f[2usize].len, b'.' as (i32)) >= f[2usize].len {
                if stralloc_cats(&mut f[2usize] as (*mut stralloc), (*b".ns.\0").as_ptr()) == 0 {
                    nomem();
                }
                if stralloc_catb(
                    &mut f[2usize] as (*mut stralloc),
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
            if stralloc_append(&mut f[3usize] as (*mut stralloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            if scan_ulong(f[3usize].s as (*const u8), &mut ttl as (*mut usize)) == 0 {
                ttl = 259200usize;
            }
            i = 0i32;
            'loop125: loop {
                if !(i < 26i32) {
                    continue 'loop57;
                }
                if dns_domain_equal(d2 as (*const u8), names[i as (usize)] as (*const u8)) != 0 {
                    break;
                }
                i = i + 1;
            }
            used[i as (usize)] = 1i32;
        }
    }
    if stralloc_copyb(
        &mut f[0usize] as (*mut stralloc),
        &mut mode as (*mut u8) as (*const u8),
        1u32,
    ) == 0
    {
        nomem();
    }
    if dns_domain_todot_cat(&mut f[0usize] as (*mut stralloc), target as (*const u8)) == 0 {
        nomem();
    }
    if stralloc_cats(&mut f[0usize] as (*mut stralloc), (*b":\0").as_ptr()) == 0 {
        nomem();
    }
    if stralloc_catb(
        &mut f[0usize] as (*mut stralloc),
        ipstr.as_mut_ptr() as (*const u8),
        ip4_fmt(ipstr.as_mut_ptr(), targetip.as_mut_ptr() as (*const u8)),
    ) == 0
    {
        nomem();
    }
    if mode as (i32) == b'@' as (i32) || mode as (i32) == b'&' as (i32) ||
        mode as (i32) == b'.' as (i32)
    {
        i = 0i32;
        'loop68: loop {
            if !(i < 26i32) {
                break;
            }
            if used[i as (usize)] == 0 {
                break;
            }
            i = i + 1;
        }
        if i >= 26i32 {
            strerr_die(
                100i32,
                (*b"tinydns-edit: fatal: \0").as_ptr(),
                (*b"too many records for that domain\0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const strerr),
            );
        }
        ch = (b'a' as (i32) + i) as (u8);
        if stralloc_cats(&mut f[0usize] as (*mut stralloc), (*b":\0").as_ptr()) == 0 {
            nomem();
        }
        if stralloc_catb(
            &mut f[0usize] as (*mut stralloc),
            &mut ch as (*mut u8) as (*const u8),
            1u32,
        ) == 0
        {
            nomem();
        }
        if mode as (i32) == b'@' as (i32) {
            if stralloc_cats(&mut f[0usize] as (*mut stralloc), (*b":\0").as_ptr()) == 0 {
                nomem();
            }
        }
    }
    if stralloc_cats(&mut f[0usize] as (*mut stralloc), (*b":\0").as_ptr()) == 0 {
        nomem();
    }
    if stralloc_catb(
        &mut f[0usize] as (*mut stralloc),
        strnum.as_mut_ptr() as (*const u8),
        fmt_ulong(strnum.as_mut_ptr(), ttl),
    ) == 0
    {
        nomem();
    }
    if stralloc_cats(&mut f[0usize] as (*mut stralloc), (*b"\n\0").as_ptr()) == 0 {
        nomem();
    }
    put(f[0usize].s as (*const u8), f[0usize].len);
    if buffer_flush(&mut bnew as (*mut buffer)) == -1i32 {
        die_write();
    }
    if fsync(fdnew) == -1i32 {
        die_write();
    }
    if close(fdnew) == -1i32 {
        die_write();
    }
    if rename(fnnew as (*const u8), fn_ as (*const u8)) == -1i32 {
        strerr_die(
            111i32,
            (*b"tinydns-edit: fatal: \0").as_ptr(),
            (*b"unable to move \0").as_ptr(),
            fnnew as (*const u8),
            (*b" to \0").as_ptr(),
            fn_ as (*const u8),
            (*b": \0").as_ptr(),
            &mut strerr_sys as (*mut strerr) as (*const strerr),
        );
    }
    _exit(0i32);
    0
}
