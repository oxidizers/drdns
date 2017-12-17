extern {
    fn stralloc_copyb(
        arg1 : *mut stralloc, arg2 : *const u8, arg3 : u32
    ) -> i32;
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
pub unsafe extern fn stralloc_copy(
    mut sato : *mut stralloc, mut safrom : *const stralloc
) -> i32 {
    stralloc_copyb(sato,(*safrom).s as (*const u8),(*safrom).len)
}
