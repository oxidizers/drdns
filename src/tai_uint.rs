#[derive(Copy)]
#[repr(C)]
pub struct tai {
    pub x : usize,
}

impl Clone for tai {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn tai_uint(mut t : *mut tai, mut u : u32) {
    (*t).x = u as (usize);
}
