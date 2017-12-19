//! `buffer.rs`: Buffered I/O
//!
//! This should probably be replaced eventually with e.g. the bytes crate

use byte;
use errno::{errno, Errno};
use libc;

type Op = unsafe extern "C" fn(i32, *const u8, u32) -> i32;

extern "C" {
    fn read(arg1: i32, arg2: *mut ::std::os::raw::c_void, arg3: usize) -> isize;
    fn write(__fd: i32, __buf: *const ::std::os::raw::c_void, __nbyte: usize) -> isize;
}

#[derive(Copy)]
#[repr(C)]
pub struct buffer {
    pub x: *mut u8,
    pub p: u32,
    pub n: u32,
    pub fd: i32,
    pub op: Op,
}

impl Clone for buffer {
    fn clone(&self) -> Self {
        *self
    }
}

impl buffer {
    #[no_mangle]
    pub unsafe extern "C" fn buffer_init(
        mut s: *mut buffer,
        mut op: Op,
        mut fd: i32,
        mut buf: *mut u8,
        mut len: u32,
    ) {
        (*s).x = buf;
        (*s).fd = fd;
        (*s).op = op;
        (*s).p = 0u32;
        (*s).n = len;
    }

    #[no_mangle]
    pub unsafe extern "C" fn buffer_copy(mut bout: *mut buffer, mut bin: *mut buffer) -> i32 {
        let mut _currentBlock;
        let mut n: i32;
        let mut x: *mut u8;
        'loop1: loop {
            n = Self::buffer_feed(bin);
            if n < 0i32 {
                _currentBlock = 7;
                break;
            }
            if n == 0 {
                _currentBlock = 6;
                break;
            }
            x = (*bin).x.offset((*bin).n as (isize));
            if Self::buffer_put(bout, x as (*const u8), n as (u32)) == -1i32 {
                _currentBlock = 5;
                break;
            }
            (*bin).p = (*bin).p.wrapping_sub(n as (u32));
            (*bin).n = (*bin).n.wrapping_add(n as (u32));
        }
        if _currentBlock == 5 {
            -3i32
        } else if _currentBlock == 6 {
            0i32
        } else {
            -2i32
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn buffer_feed(mut s: *mut buffer) -> i32 {
        let mut r: i32;
        if (*s).p != 0 {
            (*s).p as (i32)
        } else {
            r = oneread(
                (*s).op,
                (*s).fd,
                (*s).x,
                (*s).n,
            );
            (if r <= 0i32 {
                r
            } else {
                (*s).p = r as (u32);
                (*s).n = (*s).n.wrapping_sub(r as (u32));
                if (*s).n > 0u32 {
                    byte::copyr((*s).x.offset((*s).n as (isize)), r as (u32), (*s).x);
                }
                r
            })
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn buffer_bget(mut s: *mut buffer, mut buf: *mut u8, mut len: u32) -> i32 {
        let mut r: i32;
        if (*s).p > 0u32 {
            getthis(s, buf, len)
        } else if (*s).n <= len {
            oneread(
                (*s).op,
                (*s).fd,
                buf,
                (*s).n,
            )
        } else {
            r = Self::buffer_feed(s);
            (if r <= 0i32 { r } else { getthis(s, buf, len) })
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn buffer_get(mut s: *mut buffer, mut buf: *mut u8, mut len: u32) -> i32 {
        let mut r: i32;
        if (*s).p > 0u32 {
            getthis(s, buf, len)
        } else if (*s).n <= len {
            oneread(
                (*s).op,
                (*s).fd,
                buf,
                len,
            )
        } else {
            r = Self::buffer_feed(s);
            (if r <= 0i32 { r } else { getthis(s, buf, len) })
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn buffer_peek(mut s: *mut buffer) -> *mut u8 {
        (*s).x.offset((*s).n as (isize))
    }

    #[no_mangle]
    pub unsafe extern "C" fn buffer_seek(mut s: *mut buffer, mut len: u32) {
        (*s).n = (*s).n.wrapping_add(len);
        (*s).p = (*s).p.wrapping_sub(len);
    }

    #[no_mangle]
    pub unsafe extern "C" fn buffer_flush(mut s: *mut buffer) -> i32 {
        let mut p: i32;
        p = (*s).p as (i32);
        if p == 0 {
            0i32
        } else {
            (*s).p = 0u32;
            allwrite(
                (*s).op,
                (*s).fd,
                (*s).x as (*const u8),
                p as (u32),
            )
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn buffer_putalign(
        mut s: *mut buffer,
        mut buf: *const u8,
        mut len: u32,
    ) -> i32 {
        let mut _currentBlock;
        let mut n: u32;
        'loop1: loop {
            if !(len >
                    {
                        n = (*s).n.wrapping_sub((*s).p);
                        n
                    })
            {
                _currentBlock = 2;
                break;
            }
            byte::copy((*s).x.offset((*s).p as (isize)), n, buf as (*mut u8));
            (*s).p = (*s).p.wrapping_add(n);
            buf = buf.offset(n as (isize));
            len = len.wrapping_sub(n);
            if Self::buffer_flush(s) == -1i32 {
                _currentBlock = 4;
                break;
            }
        }
        if _currentBlock == 2 {
            byte::copy((*s).x.offset((*s).p as (isize)), len, buf as (*mut u8));
            (*s).p = (*s).p.wrapping_add(len);
            0i32
        } else {
            -1i32
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn buffer_put(mut s: *mut buffer, mut buf: *const u8, mut len: u32) -> i32 {
        let mut _currentBlock;
        let mut n: u32;
        n = (*s).n;
        if len > n.wrapping_sub((*s).p) {
            if Self::buffer_flush(s) == -1i32 {
                return -1i32;
            } else {
                if n < 8192u32 {
                    n = 8192u32;
                }
                'loop4: loop {
                    if !(len > (*s).n) {
                        _currentBlock = 5;
                        break;
                    }
                    if n > len {
                        n = len;
                    }
                    if allwrite(
                        (*s).op,
                        (*s).fd,
                        buf,
                        n,
                    ) == -1i32
                    {
                        _currentBlock = 10;
                        break;
                    }
                    buf = buf.offset(n as (isize));
                    len = len.wrapping_sub(n);
                }
                if _currentBlock == 5 {
                } else {
                    return -1i32;
                }
            }
        }
        byte::copy((*s).x.offset((*s).p as (isize)), len, buf as (*mut u8));
        (*s).p = (*s).p.wrapping_add(len);
        0i32
    }

    #[no_mangle]
    pub unsafe extern "C" fn buffer_putflush(
        mut s: *mut buffer,
        mut buf: *const u8,
        mut len: u32,
    ) -> i32 {
        if Self::buffer_flush(s) == -1i32 {
            -1i32
        } else {
            allwrite(
                (*s).op,
                (*s).fd,
                buf,
                len,
            )
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn buffer_putsalign(mut s: *mut buffer, mut buf: *const u8) -> i32 {
        Self::buffer_putalign(s, buf, libc::strlen(buf as *const i8) as u32)
    }

    #[no_mangle]
    pub unsafe extern "C" fn buffer_puts(mut s: *mut buffer, mut buf: *const u8) -> i32 {
        Self::buffer_put(s, buf, libc::strlen(buf as *const i8) as u32)
    }

    #[no_mangle]
    pub unsafe extern "C" fn buffer_putsflush(mut s: *mut buffer, mut buf: *const u8) -> i32 {
        Self::buffer_putflush(s, buf, libc::strlen(buf as *const i8) as u32)
    }

    #[no_mangle]
    pub unsafe extern "C" fn buffer_unixread(mut fd: i32, mut buf: *mut u8, mut len: u32) -> i32 {
        read(fd, buf as (*mut ::std::os::raw::c_void), len as (usize)) as (i32)
    }


    #[no_mangle]
    pub unsafe extern "C" fn buffer_unixwrite(mut fd: i32, mut buf: *const u8, mut len: u32) -> i32 {
        write(fd, buf as (*const ::std::os::raw::c_void), len as (usize)) as (i32)
    }
}

unsafe extern "C" fn allwrite(
    mut op: Op,
    mut fd: i32,
    mut buf: *const u8,
    mut len: u32,
) -> i32 {
    let mut _currentBlock;
    let mut w: i32;
    'loop1: loop {
        if len == 0 {
            _currentBlock = 2;
            break;
        }
        w = op(fd, buf, len);
        if w == -1i32 {
            if !(errno() == Errno(libc::EINTR)) {
                _currentBlock = 7;
                break;
            }
        } else {
            w == 0i32;
            buf = buf.offset(w as (isize));
            len = len.wrapping_sub(w as (u32));
        }
    }
    if _currentBlock == 2 { 0i32 } else { -1i32 }
}

unsafe extern "C" fn getthis(mut s: *mut buffer, mut buf: *mut u8, mut len: u32) -> i32 {
    if len > (*s).p {
        len = (*s).p;
    }
    (*s).p = (*s).p.wrapping_sub(len);
    byte::copy(buf, len, (*s).x.offset((*s).n as (isize)));
    (*s).n = (*s).n.wrapping_add(len);
    len as (i32)
}

unsafe extern "C" fn oneread(
    mut op: Op,
    mut fd: i32,
    mut buf: *mut u8,
    mut len: u32,
) -> i32 {
    let mut r: i32;
    'loop1: loop {
        r = op(fd, buf, len);
        if !(r == -1i32) {
            break;
        }
        if !(errno() == Errno(libc::EINTR)) {
            break;
        }
    }
    r
}
