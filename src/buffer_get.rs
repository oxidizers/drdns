extern "C" {
    fn byte_copy(to: *mut u8, n: u32, from: *mut u8);
    fn byte_copyr(to: *mut u8, n: u32, from: *mut u8);
    static mut errno: i32;
    static mut error_intr: i32;
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

unsafe extern "C" fn oneread(
    mut op: unsafe extern "C" fn(i32, *mut u8, u32) -> i32,
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
        if !(errno == error_intr) {
            break;
        }
    }
    r
}

#[no_mangle]
pub unsafe extern "C" fn buffer_feed(mut s: *mut buffer) -> i32 {
    let mut r: i32;
    if (*s).p != 0 {
        (*s).p as (i32)
    } else {
        r = oneread(
            (*s).op as (unsafe extern "C" fn(i32, *mut u8, u32) -> i32),
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
                 byte_copyr((*s).x.offset((*s).n as (isize)), r as (u32), (*s).x);
             }
             r
         })
    }
}

unsafe extern "C" fn getthis(mut s: *mut buffer, mut buf: *mut u8, mut len: u32) -> i32 {
    if len > (*s).p {
        len = (*s).p;
    }
    (*s).p = (*s).p.wrapping_sub(len);
    byte_copy(buf, len, (*s).x.offset((*s).n as (isize)));
    (*s).n = (*s).n.wrapping_add(len);
    len as (i32)
}

#[no_mangle]
pub unsafe extern "C" fn buffer_bget(mut s: *mut buffer, mut buf: *mut u8, mut len: u32) -> i32 {
    let mut r: i32;
    if (*s).p > 0u32 {
        getthis(s, buf, len)
    } else if (*s).n <= len {
        oneread(
            (*s).op as (unsafe extern "C" fn(i32, *mut u8, u32) -> i32),
            (*s).fd,
            buf,
            (*s).n,
        )
    } else {
        r = buffer_feed(s);
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
            (*s).op as (unsafe extern "C" fn(i32, *mut u8, u32) -> i32),
            (*s).fd,
            buf,
            len,
        )
    } else {
        r = buffer_feed(s);
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
