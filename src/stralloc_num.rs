extern "C" {
    fn stralloc_append(arg1: *mut stralloc, arg2: *const u8) -> i32;
    fn stralloc_readyplus(arg1: *mut stralloc, arg2: u32) -> i32;
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
pub unsafe extern "C" fn stralloc_catulong0(
    mut sa: *mut stralloc,
    mut u: usize,
    mut n: u32,
) -> i32 {
    let mut len: u32;
    let mut q: usize;
    let mut s: *mut u8;
    len = 1u32;
    q = u;
    'loop1: loop {
        if !(q > 9usize) {
            break;
        }
        len = len.wrapping_add(1u32);
        q = q.wrapping_div(10usize);
    }
    if len < n {
        len = n;
    }
    if stralloc_readyplus(sa, len) == 0 {
        0i32
    } else {
        s = (*sa).s.offset((*sa).len as (isize));
        (*sa).len = (*sa).len.wrapping_add(len);
        'loop6: loop {
            if len == 0 {
                break;
            }
            *s.offset({
                len = len.wrapping_sub(1u32);
                len
            } as (isize)) = (b'0' as (usize)).wrapping_add(u.wrapping_rem(10usize)) as (u8);
            u = u.wrapping_div(10usize);
        }
        1i32
    }
}

#[no_mangle]
pub unsafe extern "C" fn stralloc_catlong0(mut sa: *mut stralloc, mut l: isize, mut n: u32) -> i32 {
    if l < 0isize {
        if stralloc_append(sa, (*b"-\0").as_ptr()) == 0 {
            return 0i32;
        } else {
            l = -l;
        }
    }
    stralloc_catulong0(sa, l as (usize), n)
}
