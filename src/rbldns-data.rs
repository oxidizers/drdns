use buffer::{self, Buffer};
use byte;
use cdb::CdbMake;
use ip4;
use libc;
use stralloc::StrAlloc;
use strerr::{StrErr, STRERR_SYS};
use ulong;

extern "C" {
    fn __swbuf(arg1: i32, arg2: *mut __sFILE) -> i32;
    fn close(arg1: i32) -> i32;
    fn fsync(arg1: i32) -> i32;
    fn getln(arg1: *mut Buffer, arg2: *mut StrAlloc, arg3: *mut i32, arg4: i32) -> i32;
    fn open_read(arg1: *const u8) -> i32;
    fn open_trunc(arg1: *const u8) -> i32;
    fn rename(__old: *const u8, __new: *const u8) -> i32;
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
pub unsafe extern "C" fn nomem() {
    StrErr::die(
        111i32,
        (*b"rbldns-data: fatal: \0").as_ptr(),
        (*b"out of memory\0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const StrErr),
    );
}

#[no_mangle]
pub static mut fd: i32 = 0i32;

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

#[no_mangle]
pub static mut fdcdb: i32 = 0i32;

#[no_mangle]
pub static mut cdb: CdbMake = CdbMake {
    bspace: [0u8; 8192],
    final_: [0u8; 2048],
    count: [0u32; 256],
    start: [0u32; 256],
    head: 0 as (*mut CdbHpList),
    split: 0 as (*mut CdbHp),
    hash: 0 as (*mut CdbHp),
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

static mut tmp: StrAlloc = StrAlloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

static mut line: StrAlloc = StrAlloc {
    s: 0 as (*mut u8),
    len: 0u32,
    a: 0u32,
};

#[no_mangle]
pub static mut match_: i32 = 1i32;

#[no_mangle]
pub static mut linenum: usize = 0usize;

#[no_mangle]
pub static mut strnum: [u8; 40] = [0u8; 40];

#[no_mangle]
pub unsafe extern "C" fn syntaxerror(mut why: *const u8) {
    strnum[ulong::fmt(strnum.as_mut_ptr(), linenum) as (usize)] = 0u8;
    StrErr::die(
        111i32,
        (*b"rbldns-data: fatal: \0").as_ptr(),
        (*b"unable to parse data line \0").as_ptr(),
        strnum.as_mut_ptr() as (*const u8),
        why,
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const StrErr),
    );
}

#[no_mangle]
pub unsafe extern "C" fn die_datatmp() {
    StrErr::die(
        111i32,
        (*b"rbldns-data: fatal: \0").as_ptr(),
        (*b"unable to create data.tmp: \0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
    );
}

fn main() {
    let ret = unsafe { _c_main() };
    ::std::process::exit(ret);
}

#[no_mangle]
pub unsafe extern "C" fn _c_main() -> i32 {
    let mut ip: [u8; 4];
    let mut u: usize;
    let mut j: u32;
    let mut k: u32;
    let mut ch: u8;
    umask(0o22u16);
    fd = open_read((*b"data\0").as_ptr());
    if fd == -1i32 {
        StrErr::die(
            111i32,
            (*b"rbldns-data: fatal: \0").as_ptr(),
            (*b"unable to open data: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut STRERR_SYS as (*mut StrErr) as (*const StrErr),
        );
    }
    Buffer::init(
        &mut b as (*mut Buffer),
        buffer::unixread as buffer::Op,
        fd,
        bspace.as_mut_ptr(),
        ::std::mem::size_of::<[u8; 1024]>() as (u32),
    );
    fdcdb = open_trunc((*b"data.tmp\0").as_ptr());
    if fdcdb == -1i32 {
        die_datatmp();
    }
    if CdbMake::start(&mut cdb as (*mut CdbMake), fdcdb) == -1i32 {
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
                (*b"rbldns-data: fatal: \0").as_ptr(),
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
        let switch1 = *line.s.offset(0isize);
        if switch1 as (i32) == b'#' as (i32) {
            continue;
        }
        if switch1 as (i32) == b'9' as (i32) || switch1 as (i32) == b'8' as (i32) ||
            switch1 as (i32) == b'7' as (i32) || switch1 as (i32) == b'6' as (i32) ||
            switch1 as (i32) == b'5' as (i32) ||
            switch1 as (i32) == b'4' as (i32) ||
            switch1 as (i32) == b'3' as (i32) ||
            switch1 as (i32) == b'2' as (i32) ||
            switch1 as (i32) == b'1' as (i32) ||
            switch1 as (i32) == b'0' as (i32)
        {
            if StrAlloc::append(&mut line as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            j = 0u32;
            if StrAlloc::copys(&mut tmp as (*mut StrAlloc), (*b"\0").as_ptr()) == 0 {
                nomem();
            }
            'loop41: loop {
                k = ulong::scan(
                    line.s.offset(j as (isize)) as (*const u8),
                    &mut u as (*mut usize),
                );
                if k == 0 {
                    break;
                }
                ch = u as (u8);
                if StrAlloc::catb(
                    &mut tmp as (*mut StrAlloc),
                    &mut ch as (*mut u8) as (*const u8),
                    1u32,
                ) == 0
                {
                    nomem();
                }
                j = j.wrapping_add(k);
                if *line.s.offset(j as (isize)) as (i32) != b'.' as (i32) {
                    break;
                }
                j = j.wrapping_add(1u32);
            }
            if StrAlloc::catb(&mut tmp as (*mut StrAlloc), (*b"\0\0\0\0\0").as_ptr(), 4u32) == 0 {
                nomem();
            }
            tmp.len = 4u32;
            if *line.s.offset(j as (isize)) as (i32) == b'/' as (i32) {
                ulong::scan(
                    line.s.offset(j as (isize)).offset(1isize) as (*const u8),
                    &mut u as (*mut usize),
                );
            } else {
                u = 32usize;
            }
            if u > 32usize {
                u = 32usize;
            }
            ch = u as (u8);
            if StrAlloc::catb(
                &mut tmp as (*mut StrAlloc),
                &mut ch as (*mut u8) as (*const u8),
                1u32,
            ) == 0
            {
                nomem();
            }
            if !(CdbMake::add(
                &mut cdb as (*mut CdbMake),
                tmp.s as (*const u8),
                tmp.len,
                (*b"\0").as_ptr(),
                0u32,
            ) == -1i32)
            {
                continue;
            }
            die_datatmp();
        } else if switch1 as (i32) == b':' as (i32) {
            j = byte::chr(
                line.s.offset(1isize),
                line.len.wrapping_sub(1u32),
                b':' as (i32),
            );
            if j >= line.len.wrapping_sub(1u32) {
                syntaxerror((*b": missing colon\0").as_ptr());
            }
            if ip4::scan(line.s.offset(1isize) as (*const u8), ip.as_mut_ptr()) != j {
                syntaxerror((*b": malformed IP address\0").as_ptr());
            }
            if StrAlloc::copyb(
                &mut tmp as (*mut StrAlloc),
                ip.as_mut_ptr() as (*const u8),
                4u32,
            ) == 0
            {
                nomem();
            }
            if StrAlloc::catb(
                &mut tmp as (*mut StrAlloc),
                line.s.offset(j as (isize)).offset(2isize) as (*const u8),
                line.len.wrapping_sub(j).wrapping_sub(2u32),
            ) == 0
            {
                nomem();
            }
            if !(CdbMake::add(
                &mut cdb as (*mut CdbMake),
                (*b"\0").as_ptr(),
                0u32,
                tmp.s as (*const u8),
                tmp.len,
            ) == -1i32)
            {
                continue;
            }
            die_datatmp();
        } else {
            syntaxerror((*b": unrecognized leading character\0").as_ptr());
        }
    }
    if CdbMake::finish(&mut cdb as (*mut CdbMake)) == -1i32 {
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
            (*b"rbldns-data: fatal: \0").as_ptr(),
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
