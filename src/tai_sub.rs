#[derive(Copy)]
#[repr(C)]
pub struct tai {
    pub x : usize,
}

impl Clone for tai {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn tai_sub(
    mut t : *mut tai, mut u : *const tai, mut v : *const tai
) {
    (*t).x = (*u).x.wrapping_sub((*v).x);
}
