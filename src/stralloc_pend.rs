extern "C" {
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
pub unsafe extern "C" fn stralloc_append(mut x: *mut stralloc, mut i: *const u8) -> i32 {
    if stralloc_readyplus(x, 1u32) == 0 {
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
