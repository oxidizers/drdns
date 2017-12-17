#[derive(Copy)]
#[repr(C)]
pub struct tai {
    pub x : usize,
}

impl Clone for tai {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct taia {
    pub sec : tai,
    pub nano : usize,
    pub atto : usize,
}

impl Clone for taia {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn taia_uint(mut t : *mut taia, mut s : u32) {
    (*t).sec.x = s as (usize);
    (*t).nano = 0usize;
    (*t).atto = 0usize;
}
