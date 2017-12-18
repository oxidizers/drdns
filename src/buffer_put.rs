extern "C" {
    fn byte_copy(to: *mut u8, n: u32, from: *mut u8);
    static mut errno: i32;
    static mut error_intr: i32;
    fn str_len(arg1: *const u8) -> u32;
}

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

unsafe extern "C" fn allwrite(
    mut op: unsafe extern "C" fn(i32, *const u8, u32) -> i32,
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
            if !(errno == error_intr) {
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

#[no_mangle]
pub unsafe extern "C" fn buffer_flush(mut s: *mut buffer) -> i32 {
    let mut p: i32;
    p = (*s).p as (i32);
    if p == 0 {
        0i32
    } else {
        (*s).p = 0u32;
        allwrite(
            (*s).op as (unsafe extern "C" fn(i32, *const u8, u32) -> i32),
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
        byte_copy((*s).x.offset((*s).p as (isize)), n, buf as (*mut u8));
        (*s).p = (*s).p.wrapping_add(n);
        buf = buf.offset(n as (isize));
        len = len.wrapping_sub(n);
        if buffer_flush(s) == -1i32 {
            _currentBlock = 4;
            break;
        }
    }
    if _currentBlock == 2 {
        byte_copy((*s).x.offset((*s).p as (isize)), len, buf as (*mut u8));
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
        if buffer_flush(s) == -1i32 {
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
                    (*s).op as (unsafe extern "C" fn(i32, *const u8, u32) -> i32),
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
    byte_copy((*s).x.offset((*s).p as (isize)), len, buf as (*mut u8));
    (*s).p = (*s).p.wrapping_add(len);
    0i32
}

#[no_mangle]
pub unsafe extern "C" fn buffer_putflush(
    mut s: *mut buffer,
    mut buf: *const u8,
    mut len: u32,
) -> i32 {
    if buffer_flush(s) == -1i32 {
        -1i32
    } else {
        allwrite(
            (*s).op as (unsafe extern "C" fn(i32, *const u8, u32) -> i32),
            (*s).fd,
            buf,
            len,
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn buffer_putsalign(mut s: *mut buffer, mut buf: *const u8) -> i32 {
    buffer_putalign(s, buf, str_len(buf))
}

#[no_mangle]
pub unsafe extern "C" fn buffer_puts(mut s: *mut buffer, mut buf: *const u8) -> i32 {
    buffer_put(s, buf, str_len(buf))
}

#[no_mangle]
pub unsafe extern "C" fn buffer_putsflush(mut s: *mut buffer, mut buf: *const u8) -> i32 {
    buffer_putflush(s, buf, str_len(buf))
}
