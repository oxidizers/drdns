extern "C" {
    fn time(arg1: *mut isize) -> isize;
}

#[derive(Copy)]
#[repr(C)]
pub struct tai {
    pub x: usize,
}

impl Clone for tai {
    fn clone(&self) -> Self {
        *self
    }
}

#[no_mangle]
pub unsafe extern "C" fn tai_now(mut t: *mut tai) {
    (*t).x =
        4611686018427387914u64.wrapping_add(time(0i32 as (*mut isize)) as (usize) as (u64)) as
            (usize);
}
