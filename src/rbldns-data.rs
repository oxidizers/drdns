extern {
    fn __swbuf(arg1 : i32, arg2 : *mut __sFILE) -> i32;
    fn _exit(arg1 : i32);
    fn buffer_init(
        arg1 : *mut buffer,
        arg2 : unsafe extern fn() -> i32,
        arg3 : i32,
        arg4 : *mut u8,
        arg5 : u32
    );
    fn buffer_unixread(arg1 : i32, arg2 : *mut u8, arg3 : u32) -> i32;
    fn byte_chr(s : *mut u8, n : u32, c : i32) -> u32;
    fn cdb_make_add(
        arg1 : *mut cdb_make,
        arg2 : *const u8,
        arg3 : u32,
        arg4 : *const u8,
        arg5 : u32
    ) -> i32;
    fn cdb_make_finish(arg1 : *mut cdb_make) -> i32;
    fn cdb_make_start(arg1 : *mut cdb_make, arg2 : i32) -> i32;
    fn close(arg1 : i32) -> i32;
    fn fmt_ulong(arg1 : *mut u8, arg2 : usize) -> u32;
    fn fsync(arg1 : i32) -> i32;
    fn getln(
        arg1 : *mut buffer,
        arg2 : *mut stralloc,
        arg3 : *mut i32,
        arg4 : i32
    ) -> i32;
    fn ip4_scan(arg1 : *const u8, arg2 : *mut u8) -> u32;
    fn open_read(arg1 : *const u8) -> i32;
    fn open_trunc(arg1 : *const u8) -> i32;
    fn rename(__old : *const u8, __new : *const u8) -> i32;
    fn scan_ulong(arg1 : *const u8, arg2 : *mut usize) -> u32;
    fn stralloc_append(arg1 : *mut stralloc, arg2 : *const u8) -> i32;
    fn stralloc_catb(
        arg1 : *mut stralloc, arg2 : *const u8, arg3 : u32
    ) -> i32;
    fn stralloc_copyb(
        arg1 : *mut stralloc, arg2 : *const u8, arg3 : u32
    ) -> i32;
    fn stralloc_copys(arg1 : *mut stralloc, arg2 : *const u8) -> i32;
    fn strerr_die(
        arg1 : i32,
        arg2 : *const u8,
        arg3 : *const u8,
        arg4 : *const u8,
        arg5 : *const u8,
        arg6 : *const u8,
        arg7 : *const u8,
        arg8 : *const strerr
    );
    static mut strerr_sys : strerr;
    fn umask(arg1 : u16) -> u16;
}

enum __sFILEX {
}

#[derive(Copy)]
#[repr(C)]
pub struct __sbuf {
    pub _base : *mut u8,
    pub _size : i32,
}

impl Clone for __sbuf {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct __sFILE {
    pub _p : *mut u8,
    pub _r : i32,
    pub _w : i32,
    pub _flags : i16,
    pub _file : i16,
    pub _bf : __sbuf,
    pub _lbfsize : i32,
    pub _cookie : *mut ::std::os::raw::c_void,
    pub _close : unsafe extern fn(*mut ::std::os::raw::c_void) -> i32,
    pub _read : unsafe extern fn(*mut ::std::os::raw::c_void, *mut u8, i32) -> i32,
    pub _seek : unsafe extern fn(*mut ::std::os::raw::c_void, isize, i32) -> isize,
    pub _write : unsafe extern fn(*mut ::std::os::raw::c_void, *const u8, i32) -> i32,
    pub _ub : __sbuf,
    pub _extra : *mut __sFILEX,
    pub _ur : i32,
    pub _ubuf : [u8; 3],
    pub _nbuf : [u8; 1],
    pub _lb : __sbuf,
    pub _blksize : i32,
    pub _offset : isize,
}

impl Clone for __sFILE {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn __sputc(
    mut _c : i32, mut _p : *mut __sFILE
) -> i32 {
    if {
           (*_p)._w = (*_p)._w - 1;
           (*_p)._w
       } >= 0i32 || (*_p)._w >= (*_p)._lbfsize && (_c as (u8) as (i32) != b'\n' as (i32)) {
        ({
             let _rhs = _c;
             let _lhs
                 = &mut *{
                             let _old = (*_p)._p;
                             (*_p)._p = (*_p)._p.offset(1isize);
                             _old
                         };
             *_lhs = _rhs as (u8);
             *_lhs
         }) as (i32)
    } else {
        __swbuf(_c,_p)
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct strerr {
    pub who : *mut strerr,
    pub x : *const u8,
    pub y : *const u8,
    pub z : *const u8,
}

impl Clone for strerr {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn nomem() {
    strerr_die(
        111i32,
        (*b"rbldns-data: fatal: \0").as_ptr(),
        (*b"out of memory\0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const strerr)
    );
}

#[no_mangle]
pub static mut fd : i32 = 0i32;

#[derive(Copy)]
#[repr(C)]
pub struct buffer {
    pub x : *mut u8,
    pub p : u32,
    pub n : u32,
    pub fd : i32,
    pub op : unsafe extern fn() -> i32,
}

impl Clone for buffer {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut b
    : buffer
    = buffer {
          x: 0 as (*mut u8),
          p: 0u32,
          n: 0u32,
          fd: 0i32,
          op: 0 as (unsafe extern fn() -> i32)
      };

#[no_mangle]
pub static mut bspace : [u8; 1024] = [0u8; 1024];

#[no_mangle]
pub static mut fdcdb : i32 = 0i32;

#[derive(Copy)]
#[repr(C)]
pub struct cdb_hp {
    pub h : u32,
    pub p : u32,
}

impl Clone for cdb_hp {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct cdb_hplist {
    pub hp : [cdb_hp; 1000],
    pub next : *mut cdb_hplist,
    pub num : i32,
}

impl Clone for cdb_hplist {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct cdb_make {
    pub bspace : [u8; 8192],
    pub final_ : [u8; 2048],
    pub count : [u32; 256],
    pub start : [u32; 256],
    pub head : *mut cdb_hplist,
    pub split : *mut cdb_hp,
    pub hash : *mut cdb_hp,
    pub numentries : u32,
    pub b : buffer,
    pub pos : u32,
    pub fd : i32,
}

impl Clone for cdb_make {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut cdb
    : cdb_make
    = cdb_make {
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
                 op: 0 as (unsafe extern fn() -> i32)
             },
          pos: 0u32,
          fd: 0i32
      };

#[derive(Copy)]
#[repr(C)]
pub struct stralloc {
    pub s : *mut u8,
    pub len : u32,
    pub a : u32,
}

impl Clone for stralloc {
    fn clone(&self) -> Self { *self }
}

static mut tmp
    : stralloc
    = stralloc { s: 0 as (*mut u8), len: 0u32, a: 0u32 };

static mut line
    : stralloc
    = stralloc { s: 0 as (*mut u8), len: 0u32, a: 0u32 };

#[no_mangle]
pub static mut match_ : i32 = 1i32;

#[no_mangle]
pub static mut linenum : usize = 0usize;

#[no_mangle]
pub static mut strnum : [u8; 40] = [0u8; 40];

#[no_mangle]
pub unsafe extern fn syntaxerror(mut why : *const u8) {
    strnum[fmt_ulong(strnum.as_mut_ptr(),linenum) as (usize)] = 0u8;
    strerr_die(
        111i32,
        (*b"rbldns-data: fatal: \0").as_ptr(),
        (*b"unable to parse data line \0").as_ptr(),
        strnum.as_mut_ptr() as (*const u8),
        why,
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const strerr)
    );
}

#[no_mangle]
pub unsafe extern fn die_datatmp() {
    strerr_die(
        111i32,
        (*b"rbldns-data: fatal: \0").as_ptr(),
        (*b"unable to create data.tmp: \0").as_ptr(),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        0i32 as (*const u8),
        &mut strerr_sys as (*mut strerr) as (*const strerr)
    );
}

fn main() {
    let ret = unsafe { _c_main() };
    ::std::process::exit(ret);
}

#[no_mangle]
pub unsafe extern fn _c_main() -> i32 {
    let mut ip : [u8; 4];
    let mut u : usize;
    let mut j : u32;
    let mut k : u32;
    let mut ch : u8;
    umask(0o22u16);
    fd = open_read((*b"data\0").as_ptr());
    if fd == -1i32 {
        strerr_die(
            111i32,
            (*b"rbldns-data: fatal: \0").as_ptr(),
            (*b"unable to open data: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    buffer_init(
        &mut b as (*mut buffer),
        buffer_unixread as (unsafe extern fn() -> i32),
        fd,
        bspace.as_mut_ptr(),
        ::std::mem::size_of::<[u8; 1024]>() as (u32)
    );
    fdcdb = open_trunc((*b"data.tmp\0").as_ptr());
    if fdcdb == -1i32 {
        die_datatmp();
    }
    if cdb_make_start(&mut cdb as (*mut cdb_make),fdcdb) == -1i32 {
        die_datatmp();
    }
    'loop6: loop {
        if match_ == 0 {
            break;
        }
        linenum = linenum.wrapping_add(1usize);
        if getln(
               &mut b as (*mut buffer),
               &mut line as (*mut stralloc),
               &mut match_ as (*mut i32),
               b'\n' as (i32)
           ) == -1i32 {
            strerr_die(
                111i32,
                (*b"rbldns-data: fatal: \0").as_ptr(),
                (*b"unable to read line: \0").as_ptr(),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                0i32 as (*const u8),
                &mut strerr_sys as (*mut strerr) as (*const strerr)
            );
        }
        'loop18: loop {
            if line.len == 0 {
                break;
            }
            ch = *line.s.offset(line.len.wrapping_sub(1u32) as (isize));
            if ch as (i32) != b' ' as (i32) && (ch as (i32) != b'\t' as (i32)) && (ch as (i32) != b'\n' as (i32)) {
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
        if switch1 as (i32) == b'9' as (i32) || switch1 as (i32) == b'8' as (i32) || switch1 as (i32) == b'7' as (i32) || switch1 as (i32) == b'6' as (i32) || switch1 as (i32) == b'5' as (i32) || switch1 as (i32) == b'4' as (i32) || switch1 as (i32) == b'3' as (i32) || switch1 as (i32) == b'2' as (i32) || switch1 as (i32) == b'1' as (i32) || switch1 as (i32) == b'0' as (i32) {
            if stralloc_append(
                   &mut line as (*mut stralloc),
                   (*b"\0").as_ptr()
               ) == 0 {
                nomem();
            }
            j = 0u32;
            if stralloc_copys(
                   &mut tmp as (*mut stralloc),
                   (*b"\0").as_ptr()
               ) == 0 {
                nomem();
            }
            'loop41: loop {
                k = scan_ulong(
                        line.s.offset(j as (isize)) as (*const u8),
                        &mut u as (*mut usize)
                    );
                if k == 0 {
                    break;
                }
                ch = u as (u8);
                if stralloc_catb(
                       &mut tmp as (*mut stralloc),
                       &mut ch as (*mut u8) as (*const u8),
                       1u32
                   ) == 0 {
                    nomem();
                }
                j = j.wrapping_add(k);
                if *line.s.offset(j as (isize)) as (i32) != b'.' as (i32) {
                    break;
                }
                j = j.wrapping_add(1u32);
            }
            if stralloc_catb(
                   &mut tmp as (*mut stralloc),
                   (*b"\0\0\0\0\0").as_ptr(),
                   4u32
               ) == 0 {
                nomem();
            }
            tmp.len = 4u32;
            if *line.s.offset(j as (isize)) as (i32) == b'/' as (i32) {
                scan_ulong(
                    line.s.offset(j as (isize)).offset(1isize) as (*const u8),
                    &mut u as (*mut usize)
                );
            } else {
                u = 32usize;
            }
            if u > 32usize {
                u = 32usize;
            }
            ch = u as (u8);
            if stralloc_catb(
                   &mut tmp as (*mut stralloc),
                   &mut ch as (*mut u8) as (*const u8),
                   1u32
               ) == 0 {
                nomem();
            }
            if !(cdb_make_add(
                     &mut cdb as (*mut cdb_make),
                     tmp.s as (*const u8),
                     tmp.len,
                     (*b"\0").as_ptr(),
                     0u32
                 ) == -1i32) {
                continue;
            }
            die_datatmp();
        } else if switch1 as (i32) == b':' as (i32) {
            j = byte_chr(
                    line.s.offset(1isize),
                    line.len.wrapping_sub(1u32),
                    b':' as (i32)
                );
            if j >= line.len.wrapping_sub(1u32) {
                syntaxerror((*b": missing colon\0").as_ptr());
            }
            if ip4_scan(
                   line.s.offset(1isize) as (*const u8),
                   ip.as_mut_ptr()
               ) != j {
                syntaxerror((*b": malformed IP address\0").as_ptr());
            }
            if stralloc_copyb(
                   &mut tmp as (*mut stralloc),
                   ip.as_mut_ptr() as (*const u8),
                   4u32
               ) == 0 {
                nomem();
            }
            if stralloc_catb(
                   &mut tmp as (*mut stralloc),
                   line.s.offset(j as (isize)).offset(2isize) as (*const u8),
                   line.len.wrapping_sub(j).wrapping_sub(2u32)
               ) == 0 {
                nomem();
            }
            if !(cdb_make_add(
                     &mut cdb as (*mut cdb_make),
                     (*b"\0").as_ptr(),
                     0u32,
                     tmp.s as (*const u8),
                     tmp.len
                 ) == -1i32) {
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
    if rename(
           (*b"data.tmp\0").as_ptr(),
           (*b"data.cdb\0").as_ptr()
       ) == -1i32 {
        strerr_die(
            111i32,
            (*b"rbldns-data: fatal: \0").as_ptr(),
            (*b"unable to move data.tmp to data.cdb: \0").as_ptr(),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            0i32 as (*const u8),
            &mut strerr_sys as (*mut strerr) as (*const strerr)
        );
    }
    _exit(0i32);
    0
}
