extern {
    fn byte_copy(to : *mut u8, n : u32, from : *mut u8);
    fn stralloc_copyb(
        arg1 : *mut stralloc, arg2 : *const u8, arg3 : u32
    ) -> i32;
    fn stralloc_readyplus(arg1 : *mut stralloc, arg2 : u32) -> i32;
}

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

#[no_mangle]
pub unsafe extern fn stralloc_catb(
    mut sa : *mut stralloc, mut s : *const u8, mut n : u32
) -> i32 {
    if (*sa).s.is_null() {
        stralloc_copyb(sa,s,n)
    } else if stralloc_readyplus(sa,n.wrapping_add(1u32)) == 0 {
        0i32
    } else {
        byte_copy((*sa).s.offset((*sa).len as (isize)),n,s as (*mut u8));
        (*sa).len = (*sa).len.wrapping_add(n);
        *(*sa).s.offset((*sa).len as (isize)) = b'Z';
        1i32
    }
}
