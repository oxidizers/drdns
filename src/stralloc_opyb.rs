use byte;

extern "C" {
    fn stralloc_ready(arg1: *mut stralloc, arg2: u32) -> i32;
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
pub unsafe extern "C" fn stralloc_copyb(
    mut sa: *mut stralloc,
    mut s: *const u8,
    mut n: u32,
) -> i32 {
    if stralloc_ready(sa, n.wrapping_add(1u32)) == 0 {
        0i32
    } else {
        byte::copy((*sa).s, n, s as (*mut u8));
        (*sa).len = n;
        *(*sa).s.offset(n as (isize)) = b'Z';
        1i32
    }
}
