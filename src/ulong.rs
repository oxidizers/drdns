//! `ulong.rs`: Functions that act on ulongs
//!
//! Not sure exactly what these do, but we can probably get rid of them

pub unsafe fn fmt(mut s: *mut u8, mut u: usize) -> u32 {
    let mut len: u32;
    let mut q: usize;
    len = 1u32;
    q = u;
    'loop1: loop {
        if !(q > 9usize) {
            break;
        }
        len = len.wrapping_add(1u32);
        q = q.wrapping_div(10usize);
    }
    if !s.is_null() {
        s = s.offset(len as (isize));
        'loop4: loop {
            *{
                s = s.offset(-1isize);
                s
            } = (b'0' as (usize)).wrapping_add(u.wrapping_rem(10usize)) as (u8);
            u = u.wrapping_div(10usize);
            if u == 0 {
                break;
            }
        }
    }
    len
}

pub unsafe fn scan(s: *const u8, u: *mut usize) -> u32 {
    let mut pos: u32 = 0u32;
    let mut result: usize = 0usize;
    let mut c: usize;
    'loop1: loop {
        if !({
                 c = (*s.offset(pos as (isize)) as (i32) - b'0' as (i32)) as (u8) as (usize);
                 c
             } < 10usize)
        {
            break;
        }
        result = result.wrapping_mul(10usize).wrapping_add(c);
        pos = pos.wrapping_add(1u32);
    }
    *u = result;
    pos
}
