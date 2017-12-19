use libc;

extern "C" {
    fn stralloc_copyb(arg1: *mut stralloc, arg2: *const u8, arg3: u32) -> i32;
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
pub unsafe extern "C" fn stralloc_copys(mut sa: *mut stralloc, mut s: *const u8) -> i32 {
    stralloc_copyb(sa, s, libc::strlen(s))
}
